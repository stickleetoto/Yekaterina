# Yekaterina — Development

Active source-development branch for Yekaterina.

This branch is the successor to the former `stickleetoto/Yekaterina-Dev` repository. The former repository is archived and retained as migration provenance / rollback history.

## Repository layout

- Stable distribution and release documentation: [`main`](https://github.com/stickleetoto/Yekaterina/tree/main) — **v1.3.0**.
- Active source development: **`dev/v1.4` — Core Math + Agent Usability**.
- Development source license: **Apache License 2.0** (`LICENSE` on this branch).
- Stable binary distribution uses its separate `LICENSE.txt` on `main`.

## Status

- Development package metadata remains **1.3.0** until an explicit v1.4 release-promotion phase.
- Live v1.4 built-in/control operation surface: **1,429 operations**.
- v1.3 baseline retained: **1,425 operations**, including **15** native `xfmr.*` operations.
- v1.4 additive math layer: **4 operations** across two focused slices.
- MCP tools remain exactly **3**: `yk.compute`, `yk.find`, `yk.spec`.
- Golden regression corpus: **527/527** retained.
- Frozen v1.2 full-capability audit surface: **1,410/1,410** retained.
- Advertised MCP initialize version remains `1.0.0` by compatibility policy.

## v1.4 direction

v1.4 is intentionally not an opcode-count race. The line has two goals:

1. **Core Math** — add small, high-leverage deterministic operations that remove common reasoning/calculation work from the LLM.
2. **Agent Usability** — make existing capabilities easier to discover from natural-language intent without expanding the MCP tool schema.

### Slice 1 — equations

- `alg.linear_root(a, b)` — solve `a*x + b = 0`;
- `linalg.solve(matrix, rhs)` — solve bounded square linear systems with pivoted Gaussian elimination.

### Slice 2 — common math

- `alg.proportion(a, b, c)` — solve `a/b = c/x` for `x` (the common rule-of-three calculation);
- `num.round_sigfig(value, sigfigs)` — round finite numbers to **1–15 significant figures**.

`yk.find` includes additive natural-language bridges such as `solve proportion`, `rule of three`, and `round to significant figures`. Exact canonical/alias ownership remains stronger than semantic ranking, preserving v1.3 behavior.

## Additive architecture

Historical layers are preserved rather than rewritten:

- `src/registry.rs` / `src/engine.rs` — frozen v1.2 baseline, 1,410 operations;
- `src/registry_v13.rs` / `src/engine_v13.rs` — frozen v1.3 aggregate/dispatch layer, 1,425 operations;
- `src/registry_v14.rs` / `src/engine_v14.rs` — active v1.4 aggregate/dispatch layer;
- `src/math_v14.rs` — focused v1.4 math implementations.

This keeps release evidence separable while allowing the live runtime to move forward.

## Verification

The v1.4 CI path requires:

- `scripts/static_audit_v14.py`;
- aggregate operation manifest: **1,429 total**;
- locked Rust tests, clippy, and release build;
- unchanged three-tool MCP demo;
- retained v1.3 transformer runtime verification;
- v1.4 real-process math + natural-language discovery verification;
- existing v1.2 operation/statistics/multiplicity verifiers;
- Golden regression corpus;
- frozen v1.2 Full Capability Audit;
- benchmark invariants.

For the authoritative development snapshot, see [`CURRENT_STATE.md`](CURRENT_STATE.md).
