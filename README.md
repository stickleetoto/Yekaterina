# Yekaterina

**Pure computation. Minimal tokens.**

Yekaterina is a proprietary, free-to-use compute engine for LLM agents over MCP. The public repository is the official distribution, documentation, release-evidence, and issue-tracking home for Yekaterina. **The engine source code is not published here.**

## Yekaterina v1.0.0

The V1 core is frozen as the first stable baseline.

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

The design goal is that internal capability can grow without expanding the LLM-facing MCP tool surface.

## Download

Official binaries are distributed through **GitHub Releases**. The initial supported release target is Windows x64.

Download the latest release asset, verify its SHA-256 checksum, then point your stdio-capable MCP client at `yekaterina.exe`.

See [Installation](docs/INSTALLATION.md) and [MCP Setup](docs/MCP_SETUP.md).

## Verification evidence

V1 passed:

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

## Core and DLC

**Yekaterina Core v1 is frozen.** New scientific, mathematical, engineering, financial, and other domain capability is intended to ship as separately versioned Yekaterina DLC / extension packages rather than continuously expanding the V1 core.

See [DLC model](docs/DLC.md).

## License

Yekaterina Core is **freeware, not open source**. You may use the official binary under the [Yekaterina Freeware License v1.0](LICENSE.txt). Source code rights are not granted.

Third-party components bundled in official binary releases retain their own licenses. Official release archives should include a component inventory and applicable third-party license texts.

## Security

V1 computation operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access as compute operations. Formula evaluation and workload execution use bounded internal guards.

For vulnerability reporting, see [SECURITY.md](SECURITY.md).
