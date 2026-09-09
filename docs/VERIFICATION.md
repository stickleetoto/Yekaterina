# Yekaterina v1.2.0 verification evidence

Promoted source point:

`stickleetoto/Yekaterina-Dev@9019194af02f7b9cbe72c5a232e753e236a84b4f`

Recorded acceptance result:

```text
static_audit_v12                         24 pass / 0 fail
rc_gate                                  PASS
Rust tests                               386 / 0 failures
cargo clippy                             PASS
verify_v12_operations                    164 assertions
verify_statistics                        961 reference checks
verify_multiplicity                      577 assertions
MCP Golden                               527/527
MCP tools                                3
Registered operations                    1410
Full Capability Audit                    1410/1410 --strict
MCP showcase demo                        DEMO PASS
mutation gates                           6/6 caught
```

The promoted development commit completed the end-to-end GitHub CI successfully before stable packaging.

## Interpretation

`1410/1410` Full Capability Audit means every registered opcode was discovered and executed through the live MCP boundary using a valid fixture and matched its declared return-type contract.

It is **not** a proof that every mathematical result is correct for every possible input. Mathematical correctness is additionally covered by the Golden suite, Rust tests, independent Python reference checks, edge-case hardening, and domain-specific validation performed during development.

## Compatibility evidence

v1.2.0 preserves exactly three MCP tools and the measured **412-token / 1,725-byte** model-facing schema footprint. The MCP `initialize` advertised version remains `1.0.0` deliberately for client compatibility even though the release/crate version is `1.2.0`.

## Packaging evidence

The official Windows x64 release workflow checks out the exact promoted source SHA, builds with `cargo build --locked --release`, runs the real MCP demo and stable smoke test, creates the release ZIP, verifies the ZIP SHA-256 locally, and publishes the archive plus checksum to the stable GitHub Release.
