//! Compiled host pipelines kept on disk, and the title pipelines worth
//! starting early (decision 0007, "Pipeline cache").
//!
//! 1. Every host pipeline goes into the host's pipeline library, named by
//!    a hash of all that its host description holds (the translated
//!    shaders' code, which changes when a translation does, our root
//!    signature, the graphics state). The library is saved under the cache
//!    folder; the host refuses a library of another adapter or driver, and
//!    an entry whose description differs, so neither is ever used. A kind
//!    of pipeline the library refuses to load right after storing it (as
//!    D3D12's does mesh pipelines) is kept no more: each is compiled when
//!    made.
//! 2. The title pipelines a run drew or dispatched with are listed by a key
//!    known when the title creates them (their shader containers and
//!    graphics state); the next run hands those to the workers at
//!    creation, during loading.
//!
//! The host's library is thread-safe and synchronises itself, except that
//! loads of one pipeline must not overlap (CreatePipelineLibrary, "Thread
//! Safety"): loads and stores take a lock for their name only, so workers
//! load in parallel and a save does not stop them. One lock over every
//! call made loads (which take up to seconds each) run one at a time, and
//! workers waited seconds for it.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock, RwLock};
use std::time::{Duration, Instant};

use windows::Win32::Graphics::Direct3D12::*;
use windows::core::{HSTRING, Interface, PCWSTR};

static DIR: OnceLock<PathBuf> = OnceLock::new();

/// Keeps the pipeline library and the early list in `dir` from now on.
pub fn use_pipeline_cache(dir: PathBuf) {
    let _ = DIR.set(dir);
}

/// Host pipelines created, loaded from the library, and title pipelines
/// started at creation, since the start.
pub(crate) static MADE: AtomicU64 = AtomicU64::new(0);
pub(crate) static LOADED: AtomicU64 = AtomicU64::new(0);
pub(crate) static EARLY: AtomicU64 = AtomicU64::new(0);

struct Library {
    library: ID3D12PipelineLibrary1,
    /// The blob the library was made from: it must outlive the library.
    _blob: Vec<u8>,
    /// Pipelines stored since the last save.
    added: AtomicU64,
}

// SAFETY: the library is free-threaded (calls that must not overlap take a
// lock for their name); the blob is only read.
unsafe impl Send for Library {}
// SAFETY: as above.
unsafe impl Sync for Library {}

const LIBRARY_FILE: &str = "host-pipelines.bin";
const EARLY_FILE: &str = "early-pipelines.txt";
/// Title pipelines listed for an early start, at most.
const EARLY_MAX: usize = 8192;

fn library() -> Option<&'static Library> {
    static LIBRARY: OnceLock<Option<Library>> = OnceLock::new();
    LIBRARY
        .get_or_init(|| {
            let device = crate::host::host()?.device.cast::<ID3D12Device1>().ok()?;
            let blob = DIR
                .get()
                .and_then(|d| std::fs::read(d.join(LIBRARY_FILE)).ok())
                .unwrap_or_default();
            let library = open(&device, blob);
            if library.is_none() {
                tracing::warn!("no host pipeline library: pipelines are not kept");
            }
            library
        })
        .as_ref()
}

/// The library in `blob`, or an empty one if the host rejects the blob (an
/// adapter or driver change, damage).
fn open(device: &ID3D12Device1, blob: Vec<u8>) -> Option<Library> {
    let make = |bytes: &[u8]| -> windows::core::Result<ID3D12PipelineLibrary1> {
        // SAFETY: the bytes outlive the library (kept beside it).
        unsafe {
            device
                .CreatePipelineLibrary::<ID3D12PipelineLibrary>(bytes)?
                .cast()
        }
    };
    if !blob.is_empty() {
        match make(&blob) {
            Ok(library) => {
                tracing::info!(bytes = blob.len(), "host pipeline library loaded");
                return Some(Library {
                    library,
                    _blob: blob,
                    added: AtomicU64::new(0),
                });
            }
            Err(e) => {
                tracing::info!(error = %e, "host pipeline library rejected; starting an empty one")
            }
        }
    } else {
        tracing::info!("no host pipeline library yet; starting an empty one");
    }
    let library = make(&[]).ok()?;
    Some(Library {
        library,
        _blob: Vec::new(),
        added: AtomicU64::new(0),
    })
}

/// A pipeline's name in the library: the hash of what its host
/// description holds.
pub(crate) fn name(kind: &str, root: &[u8], code: &[&[u8]], state: &str) -> String {
    let mut h = blake3::Hasher::new();
    for part in [kind.as_bytes(), root]
        .into_iter()
        .chain(code.iter().copied())
        .chain([state.as_bytes()])
    {
        h.update(&(part.len() as u64).to_le_bytes());
        h.update(part);
    }
    h.finalize().to_hex()[..32].to_owned()
}

/// Whether the library keeps a kind of pipeline ("compute", "graphics",
/// "mesh"), as the first of the kind stored shows when loaded back at once.
const UNKNOWN: u8 = 0;
const YES: u8 = 1;
const NO: u8 = 2;
static KEPT: [AtomicU8; 3] = [
    AtomicU8::new(UNKNOWN),
    AtomicU8::new(UNKNOWN),
    AtomicU8::new(UNKNOWN),
];

fn kind_index(kind: &str) -> usize {
    match kind {
        "compute" => 0,
        "graphics" => 1,
        _ => 2,
    }
}

/// The pipeline named `name`, of `kind`: loaded from the library by `load`
/// if it is there with the same description, else made by `create` and
/// stored. The first of a kind stored is loaded back at once: refused, the
/// library gets no more of the kind (and says so once).
pub(crate) fn pipeline(
    name: &str,
    kind: &str,
    load: impl Fn(&ID3D12PipelineLibrary1, PCWSTR) -> windows::core::Result<ID3D12PipelineState>,
    create: impl FnOnce() -> windows::core::Result<ID3D12PipelineState>,
) -> windows::core::Result<ID3D12PipelineState> {
    let wide = HSTRING::from(name);
    let kept = &KEPT[kind_index(kind)];
    let library = library().filter(|_| kept.load(Ordering::Relaxed) != NO);
    let Some(library) = library else {
        MADE.fetch_add(1, Ordering::Relaxed);
        return create();
    };
    let loaded = {
        let _name = lock_name(name);
        load(&library.library, PCWSTR(wide.as_ptr()))
    };
    if let Ok(pso) = loaded {
        LOADED.fetch_add(1, Ordering::Relaxed);
        return Ok(pso);
    }
    // Compiled outside the locks: compiles take up to seconds.
    let pso = create()?;
    MADE.fetch_add(1, Ordering::Relaxed);
    let _name = lock_name(name);
    let _store = STORES.read().unwrap_or_else(|e| e.into_inner());
    // SAFETY: a live pipeline; a name already stored (a description that
    // differs) is refused, which leaves the library as it was.
    if unsafe { library.library.StorePipeline(PCWSTR(wide.as_ptr()), &pso) }.is_ok() {
        library.added.fetch_add(1, Ordering::Relaxed);
    }
    if kept.load(Ordering::Relaxed) == UNKNOWN {
        let again = load(&library.library, PCWSTR(wide.as_ptr()));
        kept.store(if again.is_ok() { YES } else { NO }, Ordering::Relaxed);
        if let Err(e) = again {
            tracing::warn!(
                error = %e,
                "host pipeline library refuses a {kind} pipeline it just stored: {kind} pipelines are compiled every run, none kept"
            );
        }
    }
    Ok(pso)
}

/// Held shared by stores and alone by a save: the serialised size must not
/// change between asking for it and serialising. Loads do not change it.
static STORES: RwLock<()> = RwLock::new(());

/// The lock for loads and stores of pipelines named `name` (a hex hash):
/// one of a few, so that pipelines of one name never load at once.
fn lock_name(name: &str) -> MutexGuard<'static, ()> {
    static NAMES: [Mutex<()>; 64] = [const { Mutex::new(()) }; 64];
    let stripe = name
        .bytes()
        .fold(0usize, |h, b| h.wrapping_mul(31).wrapping_add(b.into()))
        % NAMES.len();
    NAMES[stripe].lock().unwrap_or_else(|e| e.into_inner())
}

/// The first wait between periodic saves; it doubles up to `SAVE_MAX`.
const SAVE_EVERY: Duration = Duration::from_secs(20);
const SAVE_MAX: Duration = Duration::from_secs(160);
/// A periodic save waits at least this many times as long as the last took
/// (the library grows past a gigabyte; serialising it holds back stores,
/// and writing it takes seconds of disk).
const COST_FACTOR: u32 = 30;

/// The wait before the next periodic save, after `saves` of them, the last
/// of which took `cost`.
fn save_interval(saves: u32, cost: Duration) -> Duration {
    let backoff = SAVE_EVERY.saturating_mul(1 << saves.min(16)).min(SAVE_MAX);
    backoff.max(
        cost.saturating_mul(COST_FACTOR)
            .min(Duration::from_secs(600)),
    )
}

struct Schedule {
    /// When a periodic save may run next (`None`: at once).
    next: Option<Instant>,
    saves: u32,
}

/// Writes the library and the early list if pipelines were added since the
/// last save and the periodic wait has passed (or `now`, at the end of a
/// run). A periodic save while another is running is skipped: workers call
/// this after every pipeline and must not wait for a save.
pub(crate) fn save(now: bool) {
    // Held over serialisation and both writes, so overlapping saves cannot
    // replace files under each other.
    static SCHEDULE: Mutex<Schedule> = Mutex::new(Schedule {
        next: None,
        saves: 0,
    });
    let (Some(dir), Some(library)) = (DIR.get(), library()) else {
        return;
    };
    let mut schedule = match SCHEDULE.try_lock() {
        Ok(s) => s,
        Err(std::sync::TryLockError::Poisoned(e)) => e.into_inner(),
        Err(std::sync::TryLockError::WouldBlock) if now => {
            SCHEDULE.lock().unwrap_or_else(|e| e.into_inner())
        }
        Err(std::sync::TryLockError::WouldBlock) => return,
    };
    if !now && schedule.next.is_some_and(|t| Instant::now() < t) {
        return;
    }
    let started = Instant::now();
    let wait = match save_files(dir, library) {
        Some(()) => {
            schedule.saves += 1;
            save_interval(schedule.saves - 1, started.elapsed())
        }
        None => SAVE_EVERY,
    };
    schedule.next = Some(Instant::now() + wait);
}

/// Writes the library and the early list if anything was added; `Some`
/// if they were written.
fn save_files(dir: &std::path::Path, library: &Library) -> Option<()> {
    let (blob, saved_additions, used_changed) = {
        let _stores = STORES.write().unwrap_or_else(|e| e.into_inner());
        let added = library.added.load(Ordering::Relaxed);
        let used_changed = USED_CHANGED.swap(false, Ordering::Relaxed);
        if added == 0 && !used_changed {
            return None;
        }
        // SAFETY: a buffer of the size the library asks for, which no
        // store changes meanwhile.
        let bytes = unsafe {
            let mut bytes = vec![0u8; library.library.GetSerializedSize()];
            library.library.Serialize(&mut bytes).map(|()| bytes)
        };
        match bytes {
            Ok(bytes) => (bytes, added, used_changed),
            Err(e) => {
                if used_changed {
                    USED_CHANGED.store(true, Ordering::Relaxed);
                }
                tracing::warn!(error = %e, "host pipeline library not serialised");
                return None;
            }
        }
    };
    let used: Vec<String> = USED
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .flatten()
        .cloned()
        .collect();
    let written = std::fs::create_dir_all(dir)
        .and_then(|()| write_atomically(&dir.join(LIBRARY_FILE), &blob))
        .and_then(|()| write_atomically(&dir.join(EARLY_FILE), used.join("\n").as_bytes()));
    match written {
        Ok(()) => {
            // Pipelines stored while we were writing this snapshot count for
            // the next save.
            library.added.fetch_sub(saved_additions, Ordering::Relaxed);
            tracing::info!(
                bytes = blob.len(),
                early = used.len(),
                made = MADE.load(Ordering::Relaxed),
                loaded = LOADED.load(Ordering::Relaxed),
                started_early = EARLY.load(Ordering::Relaxed),
                "host pipeline library saved"
            );
            Some(())
        }
        Err(e) => {
            if used_changed {
                USED_CHANGED.store(true, Ordering::Relaxed);
            }
            tracing::warn!(error = %e, "host pipeline library not saved");
            None
        }
    }
}

/// Writes `bytes` to a temporary file beside `path`, then renames it over
/// `path`: a reader never sees a part. Serialised so concurrent worker and
/// shutdown saves do not race on the same staging file or target rename.
fn write_atomically(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::sync::atomic::AtomicU64;
    static WRITING: Mutex<()> = Mutex::new(());
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let _guard = WRITING.lock().unwrap_or_else(|e| e.into_inner());
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".tmp-{}-{n}", std::process::id()));
    let tmp = path.with_file_name(name);
    let res = std::fs::write(&tmp, bytes).and_then(|()| std::fs::rename(&tmp, path));
    if res.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    res
}

/// Title pipeline keys drawn or dispatched with (this run and the last).
static USED: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static USED_CHANGED: AtomicBool = AtomicBool::new(false);

fn used() -> std::sync::MutexGuard<'static, Option<HashSet<String>>> {
    let mut used = USED.lock().unwrap_or_else(|e| e.into_inner());
    if used.is_none() {
        let listed = DIR
            .get()
            .and_then(|d| std::fs::read_to_string(d.join(EARLY_FILE)).ok())
            .unwrap_or_default();
        let set: HashSet<String> = listed
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_owned)
            .collect();
        tracing::info!(pipelines = set.len(), "title pipelines to start early");
        *used = Some(set);
    }
    used
}

/// Whether the title pipeline with `key` was drawn with in an earlier run.
pub(crate) fn wanted_early(key: &str) -> bool {
    used().as_ref().is_some_and(|s| s.contains(key))
}

/// The title pipeline with `key` was drawn or dispatched with.
pub(crate) fn note_used(key: &str) {
    let mut used = used();
    let set = used.get_or_insert_with(HashSet::new);
    if set.len() < EARLY_MAX && set.insert(key.to_owned()) {
        USED_CHANGED.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_change_with_what_goes_into_the_pipeline() {
        let a = name("graphics", b"root", &[b"vs", b"ps"], "state");
        assert_eq!(a, name("graphics", b"root", &[b"vs", b"ps"], "state"));
        assert_ne!(a, name("graphics", b"root", &[b"vs", b"ps2"], "state"));
        assert_ne!(a, name("graphics", b"root", &[b"vs", b"ps"], "state2"));
        assert_ne!(a, name("graphics", b"root2", &[b"vs", b"ps"], "state"));
        assert_ne!(a, name("mesh", b"root", &[b"vs", b"ps"], "state"));
        // Parts do not run into each other.
        assert_ne!(a, name("graphics", b"root", &[b"vsp", b"s"], "state"));
    }

    /// Each kind of pipeline has a verdict of its own.
    #[test]
    fn kinds_are_told_apart() {
        let kinds = ["compute", "graphics", "mesh"].map(kind_index);
        assert_eq!(kinds, [0, 1, 2]);
    }

    #[test]
    fn a_rejected_blob_gives_an_empty_library() {
        let Some(device) = crate::host::host().and_then(|h| h.device.cast::<ID3D12Device1>().ok())
        else {
            return; // no host GPU here
        };
        let library =
            open(&device, b"not a pipeline library".repeat(16)).expect("an empty library");
        // SAFETY: a plain query on a live library.
        assert!(unsafe { library.library.GetSerializedSize() } > 0);
        assert!(library._blob.is_empty());
    }

    #[test]
    fn concurrent_write_atomically_never_fails_or_truncates() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("host-pipelines.bin");
        let threads: Vec<_> = (0..4)
            .map(|t| {
                let target = target.clone();
                std::thread::spawn(move || {
                    let payload = vec![b'a' + t as u8; 4096];
                    for _ in 0..30 {
                        write_atomically(&target, &payload).unwrap();
                    }
                })
            })
            .collect();
        for t in threads {
            t.join().unwrap();
        }
        let final_bytes = std::fs::read(&target).unwrap();
        assert_eq!(final_bytes.len(), 4096);
        assert!(final_bytes.iter().all(|&b| b == final_bytes[0]));
    }

    #[test]
    fn periodic_saves_back_off_and_follow_their_cost() {
        let cheap = Duration::from_millis(100);
        let waits: Vec<u64> = (0..6).map(|n| save_interval(n, cheap).as_secs()).collect();
        assert_eq!(waits, [20, 40, 80, 160, 160, 160]);
        assert_eq!(save_interval(u32::MAX, cheap), SAVE_MAX);
        // A save of 4 s waits 2 minutes even at first.
        assert_eq!(
            save_interval(0, Duration::from_secs(4)),
            Duration::from_secs(120)
        );
    }
}
