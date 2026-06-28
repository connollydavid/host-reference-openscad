# host-reference-openscad-helper

The out-of-process OpenSCAD helper for
[`host-reference`](https://github.com/connollydavid/host-reference). It parses a `.scad` file and
prints the kind of each top-level statement to stdout, one per line. It is a separate program by
design: the GPL-3.0 `openscad-rs` parser stays behind an arms-length boundary (host decisions
`call/0033` and `call/0034`), so `host-reference`'s permissive `openscad` plugin runs this binary as a
separate process and reads the result back. That is an aggregation, not a linkage.

## Licence and attribution

- **This binary is GPL-3.0-or-later.** It links the GPL-3.0 `openscad-rs` parser, so the whole binary
  carries that licence. See [`LICENSE`](LICENSE).
- It is kept apart from `host-reference` for exactly this reason: `host-reference` and its `openscad`
  plugin are permissive and run this binary at arm's length, never linking it, so the GPL stays on
  this side of the boundary.

## Citations

- openscad-rs OpenSCAD parser, ierror: <https://github.com/ierror/openscad-rs> (GPL-3.0-or-later).
- OpenSCAD, the language and its reference: <https://openscad.org> (the `.scad` format this parses).

## Use

    host-reference-openscad-helper <file.scad>

It prints the kind of each top-level statement, one per line, in source order. The `host-reference`
`openscad` plugin finds this binary through the `HOST_REFERENCE_OPENSCAD_HELPER` environment variable
or the binary name on `PATH`, and tallies the kinds into the structure skeleton.

## Build and test

    cargo build --release
    cargo test          # a byte-for-byte conformance golden over a small .scad
    cargo deny check    # the dependency-licence lane; openscad-rs is the flagged GPL exception
