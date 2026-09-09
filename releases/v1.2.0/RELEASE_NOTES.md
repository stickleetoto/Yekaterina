# Yekaterina v1.2.0

Yekaterina v1.2.0 promotes the verified v1.2 release-candidate line to the stable distribution.

## Stable release snapshot

- Registered built-in/control operations: **1,410**
- MCP tools: **3** — `yk.compute`, `yk.find`, `yk.spec`
- MCP schema footprint: **412 tokens / 1,725 bytes**
- Golden corpus: **527/527**
- Full Capability Audit: **1,410/1,410**
- Crate version: **1.2.0**
- MCP advertised version: **1.0.0** (deliberately frozen for client compatibility)
- Default workers: **1**; parallel batch execution remains opt-in
- Source promotion point: `stickleetoto/Yekaterina-Dev@9019194af02f7b9cbe72c5a232e753e236a84b4f`

## What changed since v1.0.0

The stable operation set grows from **1,215 to 1,410** while preserving the same three-tool MCP surface.

The v1.2 line adds:

- exact and applied arithmetic families across `int`, `dec`, `geo`, `fin`, `vec`, `unit`, and `pct`;
- statistical inference, probability distributions, tests, intervals, and regression diagnostics;
- multiplicity corrections, post-hoc testing, effect sizes, and risk measures;
- the v1.1 worker-pool and ordered parallel batch infrastructure;
- a fix for the v1.1 `expr.eval` worker-classification defect.

## Verification

The promoted source commit completed the repository's end-to-end CI successfully:

- `scripts/static_audit_v12.py`: 24 pass / 0 fail;
- `scripts/rc_gate.py`: PASS;
- lexical, operation-manifest, Golden-manifest, and Full-Audit validators: PASS;
- Rust test executions: **386 / 0 failures**;
- `cargo clippy --locked --all-targets`: PASS;
- `scripts/verify_v12_operations.py`: 164 assertions;
- `scripts/verify_statistics.py`: 961 reference-value checks;
- `scripts/verify_multiplicity.py`: 577 assertions;
- Golden: **527/527**;
- Full Capability Audit: **1,410/1,410** under `--strict`;
- MCP showcase demo: `DEMO PASS`;
- mutation gates: **6/6** caught.

The Windows release package is built from the exact promoted source commit with the committed lockfile, exercised through the real MCP demo, packaged with license/privacy/release-note material and a locked dependency inventory, and published with a SHA-256 checksum.

## Compatibility

Yekaterina still exposes exactly three MCP tools. The request-schema surface in `src/model.rs` and the MCP `initialize` identity remain deliberately compatible with the frozen v1.0.0 client-facing surface.

## Distribution

The official Windows x64 archive is distributed from the `stickleetoto/Yekaterina` GitHub Release for `v1.2.0` together with its SHA-256 checksum.

Yekaterina Core binaries remain governed by `LICENSE.txt`. The separate development-source repository has its own source license and development history.
