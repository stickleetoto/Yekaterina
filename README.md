# Yekaterina

**Pure computation. Minimal tokens.**

Yekaterina is a compute engine for LLM agents over MCP, designed to keep the LLM-facing interface small while allowing the internal computation layer to grow.

This repository is the **stable distribution and documentation home** for Yekaterina.

- Stable distribution: **v1.0.0**
- Active source development: [stickleetoto/Yekaterina-Dev](https://github.com/stickleetoto/Yekaterina-Dev)
- Stable MCP surface: **3 tools**
- Stable v1.0 operation set: **1,215 operations**

> The official stable binary distributed from this repository is governed by `LICENSE.txt`. The separate development-source repository has its own source license and development history.

## Yekaterina v1.0.0

The V1 distribution is the first frozen stable baseline.

| Metric | v1.0.0 |
|---|---:|
| Registered compute/control opcodes | **1,215** |
| Exposed MCP tools | **3** |
| Golden correctness cases | **527/527 (100%)** |
| Live `yk.spec` coverage | **1,215/1,215** |
| Live MCP execution fixtures | **1,215/1,215** |
| Clean replay / return-type contract | **1,215/1,215** |
| Schema tokens | **412** |
| Fixed 10k workload wire tokens | **159,794** |

The 1,215/1,215 Full Capability Audit establishes that every registered opcode was exercised through the live MCP interface and matched its declared return-type contract. It does **not** mean every operation is mathematically proven correct for every possible input.

## MCP surface

Yekaterina exposes exactly three MCP tools:

- `yk.find` — discover operations lazily
- `yk.spec` — inspect a selected operation
- `yk.compute` — execute single calls, batches, pipelines, and supported user operations

The design rule is simple:

> Internal capability may grow without casually expanding the LLM-facing schema.

## Stable distribution vs development

Yekaterina uses two public repositories with different roles:

```text
stickleetoto/Yekaterina
    └─ stable binaries, documentation, release evidence and issue tracking

stickleetoto/Yekaterina-Dev
    └─ Rust source development, optimization, verification and release preparation
```

The active development line is currently **v1.2.0**, with **1,387 registered operations** while preserving the same three-tool MCP surface and 412-token schema footprint.

Development changes are promoted only after regression, compatibility and capability gates are checked against the frozen baselines.

## Download

Official stable binaries are distributed through **GitHub Releases**.

Download the appropriate release asset, verify its SHA-256 checksum, then point your stdio-capable MCP client at `yekaterina.exe`.

See [Installation](docs/INSTALLATION.md) and [MCP Setup](docs/MCP_SETUP.md).

## Verification evidence

The stable V1 line passed:

```text
Local verification                       PASS
MCP Golden                               527/527 (100%)
MCP tools                                3
Opcode enumeration                       1215/1215
Live yk.spec coverage                    1215/1215
Full Audit fixture coverage              1215/1215
Clean replay / return-type contract      1215/1215
Golden oracle                            100%
Self-regression hard gate                PASS
Self-regression verdict                  CURRENT WINS
```

Against the frozen alpha.10 baseline, capability increased from **1,054 to 1,215 (+15.28%)** while the 3-tool surface, **412 schema tokens**, and fixed 10k workload **159,794 wire tokens** remained unchanged.

See [Verification](docs/VERIFICATION.md) and [Benchmarks](docs/BENCHMARKS.md).

For current development verification, concurrency work and the v1.2 operation expansion, see [Yekaterina-Dev](https://github.com/stickleetoto/Yekaterina-Dev).

## License

The official Yekaterina Core binary distributed from this repository is provided under the [Yekaterina Freeware License v1.0](LICENSE.txt).

The development-source repository is distributed separately under its own license. The two repositories should not be assumed to have identical distribution terms.

Third-party components bundled in official binary releases retain their own licenses. Official release archives should include the applicable component inventory and third-party notices.

## Security

Yekaterina compute operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access as compute operations. Formula evaluation and workload execution use bounded internal guards.

For vulnerability reporting, see [SECURITY.md](SECURITY.md).
