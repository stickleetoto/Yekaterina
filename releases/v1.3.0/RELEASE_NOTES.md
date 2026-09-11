# Yekaterina v1.3.0

Yekaterina v1.3.0 promotes the verified transformer-native expansion to the stable distribution.

## Stable release snapshot

- Registered built-in/control operations: **1,425**
- New native transformer operations: **15**
- MCP tools: **3** — `yk.compute`, `yk.find`, `yk.spec`
- MCP schema footprint: **412 tokens / 1,725 bytes**
- Golden corpus: **527/527** regression gate retained
- Frozen v1.2 Full Capability Audit: **1,410/1,410** retained
- Crate version: **1.3.0**
- MCP advertised version: **1.0.0** (deliberately frozen for client compatibility)
- Default workers: **1**; parallel batch execution remains opt-in
- Source promotion point: `stickleetoto/Yekaterina-Dev@6955d707c63efccdcf721e8369462cea9ee8d965`

## What changed since v1.2.0

The stable operation set grows from **1,410 to 1,425** while preserving the same three-tool MCP surface and every canonical v1.2 operation in its original order.

v1.3 adds 15 native `xfmr.*` operations across three groups:

- material, interpolation, thermal and basic transformer geometry calculations;
- winding-field, leakage and AC-loss calculations;
- deterministic candidate evaluation and ranking with explicit constraint evidence.

The v1.3 architecture preserves `src/registry.rs` and `src/engine.rs` as the frozen v1.2 baseline. `src/registry_v13.rs` exposes the aggregate 1,425-operation catalog, and `src/engine_v13.rs` dispatches transformer-native calls while delegating legacy operations to the historical engine.

The candidate helpers operate on caller-supplied limits and objective bands. This release does not claim IEC/IEEE/DOE certification or factory-design approval.

## Verification

The exact promoted source commit completed the final v1.3 verification path successfully:

- `scripts/static_audit_v13.py`: PASS, including synchronized `Cargo.toml` / `Cargo.lock` version metadata;
- aggregate operation manifest: **1,425 total / 15 xfmr**;
- `cargo test --locked --all-targets`: PASS;
- `cargo clippy --locked --all-targets`: PASS;
- `cargo build --locked --release`: PASS;
- MCP showcase demo: PASS;
- real-process transformer runtime verifier: PASS;
- v1.2 independent operation, statistics and multiplicity verifiers: PASS;
- Golden regression corpus: **527/527**;
- frozen v1.2 Full Capability Audit: **1,410/1,410** under `--strict`;
- benchmark invariants: PASS.

The promoted commit was reverified after merge to `Yekaterina-Dev/main`; both the transformer-candidate workflow and the end-to-end CI completed successfully.

## Compatibility

Yekaterina still exposes exactly three MCP tools. The request-schema surface and MCP `initialize` identity remain deliberately compatible with the frozen v1.0.0 client-facing surface.

Every v1.2 canonical operation remains available in its original order. The default worker count remains 1 and existing error vocabulary and scheduling rules are unchanged.

## Distribution

The official Windows x64 archive is built from the exact promoted source commit using the committed lockfile, exercised through the real MCP demo and the stable distribution smoke test, packaged with license/privacy/release-note material and a locked dependency inventory, and published together with a SHA-256 checksum.

Yekaterina Core binaries remain governed by `LICENSE.txt`. The separate development-source repository has its own source license and development history.
