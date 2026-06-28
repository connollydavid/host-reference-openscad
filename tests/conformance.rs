//! Conformance for the OpenSCAD helper: a byte-for-byte golden over a small `.scad`. The parse is
//! deterministic. Never auto-blessed; set `HOST_REFERENCE_BLESS=1` to rewrite the golden deliberately.

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn model_statements() {
    let helper = env!("CARGO_BIN_EXE_host-reference-openscad-helper");
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/model");
    let output = Command::new(helper)
        .arg(base.join("input.scad"))
        .output()
        .expect("run helper");
    assert!(
        output.status.success(),
        "helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let got = String::from_utf8(output.stdout).expect("utf8 stdout");

    let golden = base.join("expected.golden");
    if std::env::var("HOST_REFERENCE_BLESS").is_ok() {
        fs::write(&golden, &got).expect("write golden");
        return;
    }
    let want = fs::read_to_string(&golden)
        .expect("read golden; bless it first with HOST_REFERENCE_BLESS=1");
    assert_eq!(got, want, "statement kinds drifted from the golden");
}
