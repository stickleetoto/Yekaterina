# Yekaterina v1.4.0 Free — Release Notes

Yekaterina v1.4.0 is the durable free baseline for the public product line.

The release keeps the model-facing surface at exactly three MCP tools while improving common-math coverage and making existing capabilities easier for agents to discover from natural-language intent.

## Highlights

- **1,429** built-in/control operations.
- Exactly **3 MCP tools**: `yk.compute`, `yk.find`, `yk.spec`.
- Four focused Core Math additions.
- Expanded semantic discovery for already-existing statistics, percentage, finance, matrix, interpolation, and numerical operations.
- Frozen v1.2 and v1.3 implementation layers remain separately auditable.
- v1.4 becomes maintenance-oriented after release.
- v1.5+ feature development moves to a private commercial line.

## New Core Math operations

- `alg.linear_root(a, b)` — solve `a*x + b = 0`.
- `linalg.solve(matrix, rhs)` — solve bounded square linear systems using partial pivoting.
- `alg.proportion(a, b, c)` — solve the common proportion `a/b = c/x`.
- `num.round_sigfig(value, sigfigs)` — round finite numeric input to 1–15 significant figures.

## Agent discovery improvements

`yk.find` recognizes more common intent phrases without adding a new MCP tool or duplicating operations that already exist.

Representative mappings include:

```text
solve linear equation        -> alg.linear_root
system of equations          -> linalg.solve
solve proportion             -> alg.proportion
round to significant figures -> num.round_sigfig
calculate the average        -> stat.mean
standard deviation           -> stat.std
correlation coefficient      -> stat.correlation
percentage change            -> pct.change
compound annual growth rate  -> fin.cagr
monthly loan payment         -> fin.loan_payment
linear interpolation         -> math.lerp
matrix multiplication        -> mat.mul
matrix rank                  -> mat.rank
numerical integration        -> num.integrate
bisection root               -> num.bisect
```

Canonical operation names and established aliases remain stronger than semantic hints.

## Compatibility

The v1.4 Free line keeps the following expectations stable across maintenance releases:

- exactly three MCP tools;
- stable canonical operation IDs for the final v1.4.0 manifest;
- no silent semantic rewrites of existing operations;
- no repurposing of established deterministic error classes;
- additive discovery hints may improve reachability without changing execution semantics;
- MCP initialize identity remains compatibility-gated rather than mirroring the package version automatically.

## Verification gate

The exact source recorded in `SOURCE_PROVENANCE.txt` must pass the full v1.4 CI gate before publication, including:

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
frozen v1.2 Full Capability Audit          1,410/1,410 --strict
benchmark invariants                       PASS
packaged-binary smoke test                 PASS
```

## Free-line policy

v1.4.0 is the final public feature-development baseline. The v1.4.x line is maintenance-oriented and may receive correctness, security, compatibility, packaging, and discovery-only fixes.

Post-v1.4 feature development proceeds in the private commercial core.
