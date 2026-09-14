# Yekaterina

**Pure computation. Minimal tokens. Verified evolution.**

Yekaterina is a deterministic compute engine for LLM agents over MCP. It keeps the model-facing interface small while exposing a large verified operation registry behind exactly three tools.

## Stable release

**Yekaterina v1.4.0 Free** is the final public feature-development baseline.

| Metric | v1.4.0 Free |
|---|---:|
| Built-in/control operations | **1,429** |
| MCP tools | **3** |
| MCP schema footprint | **412 tokens / 1,725 bytes** |
| Golden regression corpus | **527/527** |
| Frozen v1.2 Full Capability Audit | **1,410/1,410** |
| Default workers | **1** |
| Package version | **1.4.0** |
| MCP advertised version | **1.0.0** (compatibility-gated) |

The three MCP tools remain:

- `yk.find` — discover operations lazily
- `yk.spec` — inspect a selected operation
- `yk.compute` — execute single calls, batches, pipelines, and supported user operations

The design rule remains:

> Capability may grow without expanding the model-visible tool surface.

## What changed in v1.4

v1.4 adds four focused Core Math operations while prioritizing agent discoverability over opcode-count growth:

- `alg.linear_root(a, b)`
- `linalg.solve(matrix, rhs)`
- `alg.proportion(a, b, c)`
- `num.round_sigfig(value, sigfigs)`

`yk.find` also gains semantic bridges for common natural-language requests such as statistics, percentage calculations, finance, matrix operations, interpolation, and numerical methods. Canonical operation names and established aliases remain stronger than semantic hints.

The final v1.4 registry contains **1,429 unique operations** while the MCP surface remains exactly three tools.

## Verification

The public v1.4 source line is release-gated by:

```text
static_audit_v14                         PASS
aggregate operation manifest             1,429 unique operations
cargo test --locked --all-targets         PASS
cargo clippy --locked --all-targets       PASS
cargo build --locked --release            PASS
MCP showcase demo                         PASS
v1.3 transformer runtime verifier         PASS
v1.4 math + discovery verifier            PASS
v1.2 independent reference verifiers      PASS
Golden regression                         527/527
frozen v1.2 Full Capability Audit          1,410/1,410 --strict
benchmark invariants                       PASS
```

The Windows release pipeline builds from the exact recorded source commit, runs the MCP demo and stable-package smoke test, produces a Windows x64 ZIP, verifies its SHA-256 checksum, and publishes the immutable GitHub Release.

See [v1.4.0 release notes](releases/v1.4.0/RELEASE_NOTES.md).

## Public/free boundary

v1.4.0 closes the public feature-development line.

```text
v1.4.x
  public free baseline
  correctness / security / compatibility maintenance only

v1.5+
  private commercial development
```

The public repository remains the distribution, documentation, issue-tracking, and free-line maintenance home. Proprietary post-v1.4 implementation source is developed separately.

## Download

Official stable binaries are distributed through **GitHub Releases**.

Download:

```text
Yekaterina_v1.4.0_windows-x64.zip
Yekaterina_v1.4.0_windows-x64.zip.sha256.txt
```

Verify the checksum, extract the archive, and point a stdio-capable MCP client at `yekaterina.exe`.

See [Installation](docs/INSTALLATION.md) and [MCP Setup](docs/MCP_SETUP.md).

## Compatibility

v1.4 preserves the compact MCP contract established by earlier stable lines. Existing canonical IDs in the final v1.4 manifest remain stable across v1.4.x maintenance releases, and semantic-discovery improvements must not silently rewrite execution semantics.

The MCP `initialize` version remains deliberately compatibility-gated at `1.0.0`; product/package versioning is not automatically exposed as an MCP protocol-surface change.

## License

Official stable Yekaterina binaries are distributed under the [Yekaterina Freeware License v1.0](LICENSE.txt).

The source that formed the public v1.4 line was published under the Apache License 2.0 on its development branch. That historical source license remains applicable to source already published under it. Post-v1.4 commercial source is not part of the public development line.

Third-party components retain their respective licenses.

## Security

Yekaterina compute operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access as compute operations. Formula evaluation and workload execution use bounded internal guards.

For vulnerability reporting, see [SECURITY.md](SECURITY.md).
