# CURRENT_STATE.md

> Release-candidate snapshot for Yekaterina v1.4.

## Current milestone

**Yekaterina v1.4 feature work is frozen and the project is in Free release finalization.**

The published stable line is v1.3.0. The v1.4 candidate remains additive over the released 1,425-operation baseline and preserves the frozen v1.2/v1.3 implementation layers.

| Item | State |
|---|---|
| Published stable | **v1.3.0** |
| Candidate | **v1.4.0 Free** |
| Current package metadata | `1.3.0` until explicit release promotion |
| MCP initialize identity | `1.0.0` |
| v1.4 operation target | **1,429** |
| New v1.4 math operations | **4** |
| MCP tools | **3** — `yk.compute`, `yk.find`, `yk.spec` |
| Golden gate | **527/527** |
| Frozen v1.2 full audit | **1,410/1,410** |
| Rust toolchain | 2024 / pinned 1.98.0 |

## Product decision

- v1.4.0 is the durable free baseline.
- v1.4.x becomes maintenance-oriented after release.
- Later feature development moves to the next product line.
- Operation count is not a release success metric.

The compatibility promise is recorded in `docs/V14_FREE_COMPATIBILITY_CONTRACT.md`.

## Final v1.4 capability delta

Core Math adds:

- `alg.linear_root(a, b)`
- `linalg.solve(matrix, rhs)`
- `alg.proportion(a, b, c)`
- `num.round_sigfig(value, sigfigs)`

`yk.find` also gains semantic bridges for common requests such as linear equations, averages, standard deviation, percentage change, CAGR, matrix multiplication and numerical integration.

Canonical operation identity and established aliases remain stronger than semantic hints. No fourth MCP tool is introduced.

## Layering

- `src/registry.rs` / `src/engine.rs` — frozen v1.2 baseline, 1,410 operations.
- `src/registry_v13.rs` / `src/engine_v13.rs` — frozen v1.3 aggregate layer, 1,425 operations.
- `src/registry_v14.rs` / `src/engine_v14.rs` — v1.4 aggregate layer, 1,429 operations.
- `src/math_v14.rs` — v1.4-only math implementation.

## Release gate

The exact v1.4.0 candidate must pass:

- v1.4 static audit;
- 1,429-operation manifest validation;
- locked Rust tests, clippy and release build;
- MCP showcase demo;
- v1.3 transformer runtime verification;
- v1.4 math + semantic-discovery runtime verification;
- retained v1.2 operation/statistics/multiplicity verifiers;
- Golden 527/527;
- frozen v1.2 Full Capability Audit 1410/1410 strict;
- benchmark invariants;
- packaged-binary smoke test before publication.

## Remaining release work

1. Finalize release-facing documentation and release notes.
2. Promote Cargo package and lock metadata together to `1.4.0`.
3. Update the v1.4 static audit to final-release version gating.
4. Run the complete release gate on the exact promoted source.
5. Produce and smoke-test the Windows x64 package and SHA-256 checksum.
6. Promote the verified candidate and record the exact v1.4 boundary commit/tag.

No new v1.4 feature should be added merely to increase the operation count.
