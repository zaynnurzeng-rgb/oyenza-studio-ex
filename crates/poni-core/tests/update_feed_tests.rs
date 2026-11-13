use poni_core::update::{newer, parse_file, parse_payload};
use std::fs;
use std::path::Path;

#[test]
fn update_feed_file_is_valid_and_trusted() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let feed_path = root.join("site").join("latest.json");
    if !feed_path.exists() {
        return;
    }
    let content = fs::read_to_string(&feed_path).expect("read site/latest.json");
    let signed = parse_file(&content).expect("parse site/latest.json signed envelope");
    let release = parse_payload(&signed.payload).expect("parse site/latest.json release payload");

    assert!(!release.version.is_empty(), "version must not be empty");
    assert!(!release.notes.is_empty(), "notes must not be empty");
    assert!(release.windows.is_some(), "windows download must be present");

    // The feed may lag behind the code while a release is still being built,
    // but it must never run ahead of it: that is how 1.3.1 was announced while
    // no installer existed. A feed that describes this very version is what
    // people are downloading, so it needs every platform's file.
    assert!(
        !newer(&release.version, env!("CARGO_PKG_VERSION")),
        "site/latest.json must never name a version newer than the crate"
    );
    if release.version == env!("CARGO_PKG_VERSION") {
        assert!(release.mac.is_some(), "mac download must be present");
    }
}
