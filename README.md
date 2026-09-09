# Yekaterina

**Pure computation. Minimal tokens. Verified evolution.**

Yekaterina is a compute engine for LLM agents over MCP, designed to keep the model-facing interface small while the internal deterministic computation layer grows.

This repository is the **stable distribution and documentation home** for Yekaterina.

- Stable distribution: **v1.2.0**
- Active source development: [stickleetoto/Yekaterina-Dev](https://github.com/stickleetoto/Yekaterina-Dev)
- Stable MCP surface: **3 tools**
- Stable operation set: **1,410 operations**
- MCP schema footprint: **412 tokens / 1,725 bytes**

> Official stable binaries distributed from this repository are governed by `LICENSE.txt`. The separate development-source repository has its own source license and development history.

## Yekaterina v1.2.0

v1.2.0 promotes the verified v1.2 release-candidate line to stable while preserving the compact MCP interface.

| Metric | v1.2.0 |
|---|---:|
| Registered built-in/control operations | **1,410** |
| Exposed MCP tools | **3** |
| MCP schema footprint | **412 tokens / 1,725 bytes** |
| Golden correctness corpus | **527/527** |
| Full Capability Audit | **1,410/1,410** |
| Rust test executions | **386 / 0 failures** |
| Error codes | **30** |
| Default workers | **1** |
| Crate version | **1.2.0** |
| MCP advertised version | **1.0.0** (deliberately frozen) |

The promoted source point is `stickleetoto/Yekaterina-Dev@9019194af02f7b9cbe72c5a232e753e236a84b4f`.

The Full Capability Audit establishes that every registered opcode was exercised through the live MCP interface and matched its declared return-type contract. It does **not** claim mathematical proof for every possible input.

## What's new in v1.2

The stable operation set grows from **1,215 to 1,410** without increasing the three-tool MCP surface.

The v1.2 line adds:

- exact and applied families across `int`, `dec`, `geo`, `fin`, `vec`, `unit`, and `pct`;
- statistical inference, distributions, tests, confidence intervals, and regression diagnostics;
- multiplicity corrections, post-hoc testing, effect sizes, and risk measures;
- ordered parallel batch infrastructure from the v1.1 performance line;
- a fix for the v1.1 `expr.eval` worker-classification defect.

## MCP surface

Yekaterina exposes exactly three MCP tools:

- `yk.find` — discover operations lazily
- `yk.spec` — inspect a selected operation
- `yk.compute` — execute single calls, batches, pipelines, and supported user operations

The design rule remains:

> Internal capability may grow without casually expanding the LLM-facing schema.

Operation discovery stays out of `tools/list`, so the registry can grow without forcing every operation into the model-visible tool schema.

## Verification evidence

The source promoted to v1.2.0 completed the end-to-end v1.2 CI successfully:

```text
static_audit_v12                         24 pass / 0 fail
rc_gate                                  PASS
Rust tests                               386 / 0 failures
cargo clippy                             PASS
verify_v12_operations                    164 assertions
verify_statistics                        961 reference checks
verify_multiplicity                      577 assertions
MCP Golden                               527/527
Full Capability Audit                    1410/1410 --strict
MCP showcase demo                        DEMO PASS
mutation gates                           6/6 caught
```

The Windows stable package is built from the exact promoted source commit with the committed lockfile, run through the real MCP demo, packaged with release/license/privacy and dependency-inventory material, and accompanied by a SHA-256 checksum.

See [v1.2.0 release notes](releases/v1.2.0/RELEASE_NOTES.md) for the promotion record.

## Stable distribution vs development

Yekaterina uses two public repositories with different roles:

```text
stickleetoto/Yekaterina
    └─ stable binaries, documentation, release evidence and issue tracking

stickleetoto/Yekaterina-Dev
    └─ Rust source development, optimization, verification and release preparation
```

Development changes are promoted only after regression, compatibility and capability gates are checked against the frozen baselines.

## Download

Official stable binaries are distributed through **GitHub Releases**.

Download `Yekaterina_v1.2.0_windows-x64.zip` together with its `.sha256.txt` file, verify the checksum, then point your stdio-capable MCP client at `yekaterina.exe`.

See [Installation](docs/INSTALLATION.md) and [MCP Setup](docs/MCP_SETUP.md).

## Compatibility

The MCP request-schema surface remains compatible with the frozen v1.0.0 line. The server's MCP `initialize` response still advertises `1.0.0` deliberately; crate/release versioning can evolve without silently changing what existing MCP clients observe.

Parallel batch execution is opt-in. The default worker count remains 1.

## License

The official Yekaterina Core binary distributed from this repository is provided under the [Yekaterina Freeware License v1.0](LICENSE.txt).

The development-source repository is distributed separately under its own license. The two repositories should not be assumed to have identical distribution terms.

Third-party components bundled in official binary releases retain their own licenses. Official release archives include the applicable component inventory and collected third-party license material.

## Security

Yekaterina compute operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access as compute operations. Formula evaluation and workload execution use bounded internal guards.

For vulnerability reporting, see [SECURITY.md](SECURITY.md).
