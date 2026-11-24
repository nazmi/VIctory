use std::path::Path;
use std::process::Command;

#[test]
fn census_fails_when_a_requested_path_is_missing() {
    let valid = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let missing = valid.with_extension("missing-census-input");
    assert!(!missing.exists());

    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .args([&valid, &missing])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "census succeeded: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(stderr.contains("Cargo.missing-census-input"), "{stderr}");
}

#[test]
fn census_accepts_a_valid_path_with_no_pe_images() {
    let valid = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .arg(valid)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("0 images scanned, 0 unreadable"));
}

/// A directory of `names`, each holding the same unparsable PE (`MZ` only):
/// synthetic, nothing from a title.
fn synthetic_images(names: &[&str]) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "victory-census-dedup-{}-{}",
        std::process::id(),
        names.concat()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for name in names {
        let mut bytes = b"MZ".to_vec();
        bytes.resize(64, 0);
        std::fs::write(dir.join(name), bytes).unwrap();
    }
    dir
}

#[test]
fn census_reads_an_image_once_across_repeated_roots() {
    // The same root named twice visits each file twice; the count and the
    // diagnostics must name it once.
    let dir = synthetic_images(&["a.bin"]);
    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .args([&dir, &dir])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains("0 images scanned, 1 unreadable"),
        "{stdout}"
    );
    assert_eq!(stdout.matches("  skipped ").count(), 1, "{stdout}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn census_counts_identical_bytes_under_two_names_twice() {
    // Deduplication is by path: two files that happen to match are two
    // inputs.
    let dir = synthetic_images(&["a.bin", "b.bin"]);
    let output = Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("census")
        .arg(&dir)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout.contains("0 images scanned, 2 unreadable"),
        "{stdout}"
    );
    assert_eq!(stdout.matches("  skipped ").count(), 2, "{stdout}");
    let _ = std::fs::remove_dir_all(&dir);
}
