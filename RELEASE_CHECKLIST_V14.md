# Yekaterina v1.4.0 Free — Final Release Checklist

This checklist closes the public v1.4 feature line and promotes the exact accepted source to the durable Free baseline.

## R1 — Scope freeze

- [x] Core gap audit exists.
- [x] Operation-count growth is no longer a release goal.
- [x] Public v1.4 compatibility contract exists.
- [x] Three-tool MCP surface remains the target.
- [x] v1.4 Free release notes drafted.
- [x] Release-facing README drafted.

## R2 — Release identity

The version-promotion change must update the package and lockfile together.

- [ ] `Cargo.toml` package version = `1.4.0`.
- [ ] root `yekaterina` entry in `Cargo.lock` = `1.4.0`.
- [ ] `scripts/static_audit_v14.py` expects the final `1.4.0` package/lock identity instead of the development `1.3.0` identity.
- [ ] current-state text no longer describes package metadata as pending.
- [ ] release notes no longer contain the release-candidate disclaimer.

Do not merge a partial version bump that makes `cargo --locked` require an uncommitted lockfile change.

## R3 — Exact-source verification

Run on the exact version-promoted candidate:

```text
python scripts/static_audit_v14.py
python scripts/operation_manifest_v14.py
python scripts/validate_golden_manifest.py
python scripts/validate_full_audit.py
cargo test --locked --all-targets
cargo clippy --locked --all-targets
cargo build --locked --release
python tools/demo.py <release-binary>
python scripts/verify_v13_transformer_runtime.py <release-binary>
python scripts/verify_v14_math_agent_runtime.py <release-binary>
python scripts/verify_v12_operations.py
python scripts/verify_statistics.py
python scripts/verify_multiplicity.py
python golden/run_golden.py --exe <release-binary> --out <out-dir>
python scripts/check_golden_result.py <golden-result>
python full_audit/run_full_audit.py --exe <release-binary> --out <out-dir> --strict
python bench/run_bench.py --exe <release-binary> --out <out-dir> --runs 2 --cold-reps 3 --label v1.4.0
```

Expected release invariants:

- [ ] aggregate manifest = **1,429 unique operations**.
- [ ] MCP tools = exactly **3**.
- [ ] Golden = **527/527**.
- [ ] frozen v1.2 Full Capability Audit = **1410/1410 strict**.
- [ ] v1.3 transformer runtime verifier = PASS.
- [ ] v1.4 math + discovery runtime verifier = PASS.
- [ ] benchmark invariants = PASS.

## R4 — Windows distribution

- [ ] Build exact accepted source as Windows x64 release binary.
- [ ] Package official v1.4.0 ZIP.
- [ ] Generate matching SHA-256 file.
- [ ] Confirm archive contains the intended license/privacy/security/support material.
- [ ] Extract into a clean path.
- [ ] Run the packaged-binary stable smoke test.
- [ ] Run the MCP showcase demo against the extracted packaged binary.

## R5 — Promotion

- [ ] Record exact accepted source commit SHA.
- [ ] Create the v1.4.0 release/promotion branch from that exact source.
- [ ] Promote public distribution documentation to v1.4.0.
- [ ] Publish the Windows ZIP and checksum without overwriting an existing release.
- [ ] Verify the published asset names, sizes and checksums remotely.
- [ ] Record the exact v1.4.0 tag/commit as the Free-line boundary.

## R6 — Post-release boundary

After v1.4.0 publication:

- [ ] v1.4.x accepts maintenance-class changes only by default.
- [ ] Public v1.4 source does not become the development home for later product features.
- [ ] Next-line feature development starts from the recorded v1.4 boundary in its designated development location.

## Exit criterion

v1.4.0 Free is complete only when a clean user can obtain the official package, verify its checksum, connect it to an MCP client, discover representative operations, execute them successfully, and the full release evidence is tied to the exact published source and binary.
