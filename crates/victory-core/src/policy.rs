//! Rule #1, enforced at runtime: game-derived data never lands in the repo.
//!
//! Everything VIctory produces from the user's files (converted assets,
//! prepared images, shader caches, crash dumps) is as much "from the game"
//! as the files themselves. Every tool that writes such output must call
//! [`ensure_outside_repo`] on the destination first. The static half of the
//! rule is `cargo xtask policy`, which checks what git tracks.

use std::io;
use std::path::{Component, Path, PathBuf};

/// Marker file at the root of the source tree.
pub const ROOT_MARKER: &str = ".victory-root";

/// The source tree this binary was compiled from.
fn compiled_workspace_root() -> PathBuf {
    // crates/victory-core -> workspace root
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error(
        "refusing to use {path} for game-derived data: it is inside the VIctory source tree \
         ({root}). Keep game files, caches and outputs outside the repository \
         (see docs/asset-policy.md)"
    )]
    InsideRepo { path: PathBuf, root: PathBuf },
    #[error("cannot resolve {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

/// Returns `Ok` only if `path` is outside every VIctory source tree.
///
/// `path` does not have to exist yet. Two checks are made:
/// 1. it is not below the source tree this binary was built from;
/// 2. none of its ancestors contains the [`ROOT_MARKER`] file (catches
///    other clones/copies of the repository).
pub fn ensure_outside_repo(path: &Path) -> Result<(), PolicyError> {
    let result = ensure_outside_repo_with(path, |path| std::fs::canonicalize(path));
    #[cfg(windows)]
    if let Err(PolicyError::Io { source, .. }) = &result
        && source.kind() == io::ErrorKind::PermissionDenied
    {
        // An AppContainer can open a directory without access to its DOS volume name.
        // Compare both paths in the same namespace and still check the resolved ancestors.
        return ensure_outside_repo_with(path, canonicalize_native);
    }
    result
}

fn ensure_outside_repo_with(
    path: &Path,
    canonicalize: impl Fn(&Path) -> io::Result<PathBuf>,
) -> Result<(), PolicyError> {
    let resolved = resolve(path, &canonicalize)?;

    if let Ok(root) = canonicalize(&compiled_workspace_root())
        && root.join(ROOT_MARKER).is_file()
        && resolved.starts_with(&root)
    {
        return Err(PolicyError::InsideRepo {
            path: path.to_path_buf(),
            root,
        });
    }

    for ancestor in resolved.ancestors() {
        if ancestor.join(ROOT_MARKER).is_file() {
            return Err(PolicyError::InsideRepo {
                path: path.to_path_buf(),
                root: ancestor.to_path_buf(),
            });
        }
    }
    Ok(())
}

/// Canonicalizes the longest existing prefix of `path` and re-appends the
/// rest, so that paths which do not exist yet can still be compared.
fn resolve(
    path: &Path,
    canonicalize: impl Fn(&Path) -> io::Result<PathBuf>,
) -> Result<PathBuf, PolicyError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|source| PolicyError::Io {
                path: path.to_path_buf(),
                source,
            })?
            .join(path)
    };

    let mut existing = absolute.as_path();
    let mut rest = Vec::new();
    loop {
        match canonicalize(existing) {
            Ok(canon) => {
                let mut out = canon;
                for component in rest.iter().rev() {
                    out.push(component);
                }
                return Ok(normalize(&out));
            }
            Err(source) if cfg!(windows) && source.kind() == io::ErrorKind::PermissionDenied => {
                return Err(PolicyError::Io {
                    path: path.to_path_buf(),
                    source,
                });
            }
            Err(source) => match (existing.parent(), existing.file_name()) {
                (Some(parent), Some(name)) => {
                    rest.push(name.to_os_string());
                    existing = parent;
                }
                _ => {
                    return Err(PolicyError::Io {
                        path: path.to_path_buf(),
                        source,
                    });
                }
            },
        }
    }
}

#[cfg(windows)]
fn canonicalize_native(path: &Path) -> io::Result<PathBuf> {
    use std::fs::OpenOptions;
    use std::os::windows::{ffi::OsStringExt, fs::OpenOptionsExt, io::AsRawHandle};
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_BACKUP_SEMANTICS, GetFinalPathNameByHandleW, VOLUME_NAME_NT,
    };

    let file = OpenOptions::new()
        .access_mode(0)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    let mut name = vec![0u16; 512];
    loop {
        // SAFETY: the live file owns the handle, and the buffer is writable for its given length.
        let len = unsafe {
            GetFinalPathNameByHandleW(
                file.as_raw_handle(),
                name.as_mut_ptr(),
                name.len() as u32,
                VOLUME_NAME_NT,
            )
        } as usize;
        if len == 0 {
            return Err(io::Error::last_os_error());
        }
        if len >= name.len() {
            name.resize(len, 0);
            continue;
        }

        // Make the normalized NT path usable by ordinary Win32 file APIs.
        let mut absolute = std::ffi::OsString::from(r"\\?\GLOBALROOT");
        absolute.push(std::ffi::OsString::from_wide(&name[..len]));
        return Ok(PathBuf::from(absolute));
    }
}

/// Removes `.` and resolves `..` lexically (only used on the non-existing tail).
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_paths_inside_this_repo() {
        let inside = compiled_workspace_root().join("cache").join("textures");
        assert!(matches!(
            ensure_outside_repo(&inside),
            Err(PolicyError::InsideRepo { .. })
        ));
        let sneaky = std::env::temp_dir().join("..").join("x"); // sanity: no panic
        let _ = ensure_outside_repo(&sneaky);
    }

    #[test]
    fn rejects_dotdot_escapes_back_into_repo() {
        let root = compiled_workspace_root();
        let path = root
            .join("crates")
            .join("..")
            .join("not-yet-created")
            .join("out");
        assert!(ensure_outside_repo(&path).is_err());
    }

    #[test]
    fn rejects_other_clones_via_marker() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(ROOT_MARKER), "").unwrap();
        assert!(ensure_outside_repo(&tmp.path().join("cache")).is_err());
    }

    #[test]
    fn accepts_unrelated_directories() {
        let tmp = tempfile::tempdir().unwrap();
        ensure_outside_repo(&tmp.path().join("does").join("not").join("exist")).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn native_paths_still_check_repository_markers() {
        let temp = tempfile::tempdir().unwrap();
        let target = temp.path().join("missing").join("cache");
        ensure_outside_repo_with(&target, canonicalize_native).unwrap();
        std::fs::write(temp.path().join(ROOT_MARKER), "").unwrap();
        assert!(matches!(
            ensure_outside_repo_with(&target, canonicalize_native),
            Err(PolicyError::InsideRepo { .. })
        ));
        assert!(matches!(
            ensure_outside_repo_with(
                &compiled_workspace_root().join("missing").join("cache"),
                canonicalize_native,
            ),
            Err(PolicyError::InsideRepo { .. })
        ));
    }
}
