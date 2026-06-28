# NOTICE

This binary links the OpenSCAD parser:

- [`openscad-rs`](https://github.com/ierror/openscad-rs) by ierror, licensed **GPL-3.0-or-later**.

Because the parser is GPL, this whole binary is GPL-3.0-or-later (see `LICENSE`). It is confined to
this helper, which `host-reference` runs at arm's length over the command line (`call/0033`,
`call/0034`), so the permissive `host-reference-openscad` plugin and the rest of the `host-reference`
workspace are an aggregation with this binary rather than a derivative of it, and the GPL does not
reach across the boundary.

The `.scad` format this parses is OpenSCAD's; see <https://openscad.org>.
