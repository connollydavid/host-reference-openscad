//! The out-of-process OpenSCAD helper (call/0033). It links the GPL-3.0 `openscad-rs` parser, so this
//! binary is GPL, and it stays on this side of the arms-length boundary. The permissive
//! `host-reference-openscad` plugin runs it as a separate process and reads its stdout, so the two are
//! aggregated rather than linked. It reads one `.scad` path from argv, parses it, and prints the kind
//! of each top-level statement, one per line, in source order. The permissive plugin tallies those
//! into the structure skeleton (call/0032).

use std::error::Error;

use openscad_rs::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: host-reference-openscad-helper <file.scad>")?;
    let source = std::fs::read_to_string(&path)?;
    let ast = parse(&source).map_err(|e| format!("openscad: {e:?}"))?;
    for statement in &ast.statements {
        let debug = format!("{statement:?}");
        let kind = debug.split(['(', ' ', '{', '\n']).next().unwrap_or("Statement");
        println!("{kind}");
    }
    Ok(())
}
