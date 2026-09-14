# Yekaterina v1.4 Free

**Deterministic compute for AI agents. Small tool surface, broad verified capability.**

Yekaterina is a local compute engine for LLM agents over MCP. It keeps the model-facing interface intentionally small while exposing a large deterministic operation catalog behind three tools.

> **v1.4 is the durable free baseline.** After v1.4.0, the public v1.4 line becomes maintenance-oriented. New commercial Core development moves to a private v1.5+ line.

## Release status

This branch is the **v1.4.0 Free release-candidate line**.

- Built-in/control operations: **1,429**
- MCP tools: **3** — `yk.compute`, `yk.find`, `yk.spec`
- Golden regression corpus: **527/527** retained as a release gate
- Frozen v1.2 Full Capability Audit: **1,410/1,410** retained as a release gate
- Native transformer operations retained from v1.3: **15**
- MCP initialize identity: `1.0.0` by compatibility policy
- Rust edition/toolchain: 2024 / pinned 1.98.0

Final package metadata remains promotion-gated until the exact v1.4.0 candidate is accepted and versioned for release.

## What v1.4 adds

### Core Math

Four focused operations are added over the v1.3 baseline:

- `alg.linear_root(a, b)` — solve `a*x + b = 0`
- `linalg.solve(matrix, rhs)` — solve bounded square linear systems using partial pivoting
- `alg.proportion(a, b, c)` — solve `a/b = c/x`
- `num.round_sigfig(value, sigfigs)` — round finite values to 1–15 significant figures

v1.4 deliberately avoids an opcode-count race. Before another operation is added, the existing catalog is audited for overlap.

### Better agent discovery

`yk.find` gains additive semantic bridges so common natural-language requests can reach operations that already exist.

Examples:

```text
solve linear equation        -> alg.linear_root
system of equations          -> linalg.solve
solve proportion             -> alg.proportion
round to significant figures -> num.round_sigfig
quadratic equation           -> alg.quadratic_roots
calculate the average        -> stat.mean
standard deviation           -> stat.std
percentage change            -> pct.change
compound annual growth rate  -> fin.cagr
matrix multiplication        -> mat.mul
numerical integration        -> num.integrate
```

Exact canonical names and established aliases remain stronger than semantic hints.

## MCP surface

Yekaterina exposes exactly three model-facing tools:

- `yk.find` — discover operations lazily
- `yk.spec` — inspect the selected operation contract
- `yk.compute` — execute single calls, batches, pipelines and supported user operations

The design rule is unchanged:

> **Capability may grow without casually growing the model-visible tool schema.**

## Compatibility contract

The free v1.4 line freezes the public behavior expected from maintenance releases:

- exactly three MCP tools;
- stable canonical operation IDs for the final v1.4.0 manifest;
- no silent semantic rewrites of existing operations;
- additive discovery hints are allowed when they only improve reachability;
- no new feature expansion by default after v1.4.0;
- correctness, security, compatibility and packaging fixes remain appropriate for v1.4.x.

See [`docs/V14_FREE_COMPATIBILITY_CONTRACT.md`](docs/V14_FREE_COMPATIBILITY_CONTRACT.md).

## Architecture

Historical release layers remain auditable instead of being rewritten in place:

```text
src/registry.rs / src/engine.rs
  frozen v1.2 baseline — 1,410 operations

src/registry_v13.rs / src/engine_v13.rs
  frozen v1.3 additive layer — 1,425 aggregate operations

src/registry_v14.rs / src/engine_v14.rs
  v1.4 aggregate layer — 1,429 operations

src/math_v14.rs
  v1.4-only math implementation
```

The live `registry` and `engine` exports route through the v1.4 layer while the historical implementations stay separately verifiable.

## Verification gate

The exact v1.4.0 release candidate must pass all of the following before promotion:

```text
static_audit_v14                         PASS
aggregate operation manifest             1,429 unique operations
cargo test --locked --all-targets         PASS
cargo clippy --locked --all-targets       PASS
cargo build --locked --release            PASS
MCP showcase demo                         PASS
v1.3 transformer runtime verifier         PASS
v1.4 math + discovery verifier            PASS
v1.2 independent operation verifier       PASS
statistics reference verifier             PASS
multiplicity reference verifier           PASS
Golden regression                         527/527
frozen v1.2 Full Capability Audit          1410/1410 --strict
benchmark invariants                       PASS
```

A packaged binary must also pass the stable smoke test before publication.

## Free-line boundary

v1.4.0 is intended to be the final public feature-development baseline.

```text
v1.4.0 Free
  public source-development boundary
  maintenance-oriented v1.4.x line

v1.5+
  private commercial Core development
```

The free line remains useful on its own. The commercial line is not intended to retroactively remove functionality from v1.4.

## Development and build

```bash
cargo test --locked --all-targets
cargo clippy --locked --all-targets
cargo build --locked --release
```

Run the MCP demo against the release binary:

```bash
python tools/demo.py ./target/release/yekaterina
```

On Windows, use the `.exe` path instead.

## Safety

Yekaterina compute operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access. Formula evaluation and workloads are bounded by internal guards.

Verification evidence applies to the stated test suites and contracts. It is not a guarantee that every result is appropriate for safety-critical, financial, engineering, medical, or other high-impact decisions without independent review.

## Documentation

- [`CURRENT_STATE.md`](CURRENT_STATE.md) — release-candidate state
- [`docs/V14_CORE_GAP_AUDIT.md`](docs/V14_CORE_GAP_AUDIT.md) — capability-overlap audit
- [`docs/V14_FREE_COMPATIBILITY_CONTRACT.md`](docs/V14_FREE_COMPATIBILITY_CONTRACT.md) — free-line compatibility contract
- [`CHANGELOG.md`](CHANGELOG.md) — release history

## License boundary

The public v1.4 development source remains under the license shipped with this public source line. Official binary distribution may carry a separate binary license in the stable distribution branch.

Previously granted open-source rights to already-published source snapshots are not revoked by the later commercial transition. Proprietary v1.5+ implementation source is developed outside the public source-development line.
