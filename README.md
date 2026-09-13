# Yekaterina

**Pure computation. Minimal tokens. Verified evolution.**

Yekaterina is a compute engine for LLM agents over MCP, designed to keep the model-facing interface small while the internal deterministic computation layer grows.

This repository is the **stable distribution, documentation, and active development home** for Yekaterina.

- Stable distribution: **v1.3.0** on `main`
- Active source development: [`dev/v1.4`](https://github.com/stickleetoto/Yekaterina/tree/dev/v1.4)
- Stable MCP surface: **3 tools**
- Stable operation set: **1,425 operations**
- MCP schema footprint: **412 tokens / 1,725 bytes**

> Official stable binaries distributed from `main` are governed by `LICENSE.txt`. Development source on `dev/v1.4` is distributed under the Apache License 2.0 in that branch's `LICENSE` file. Branches intentionally have different distribution terms.

## Yekaterina v1.3.0

v1.3.0 promotes the verified transformer-native expansion to stable while preserving the compact MCP interface and the complete v1.2 baseline.

| Metric | v1.3.0 |
|---|---:|
| Registered built-in/control operations | **1,425** |
| Native transformer operations | **15** |
| Exposed MCP tools | **3** |
| MCP schema footprint | **412 tokens / 1,725 bytes** |
| Golden correctness corpus | **527/527** retained |
| Frozen v1.2 Full Capability Audit | **1,410/1,410** retained |
| Transformer runtime verification | **15-op surface verified** |
| Default workers | **1** |
| Crate version | **1.3.0** |
| MCP advertised version | **1.0.0** (deliberately frozen) |

The v1.3 release was originally promoted from the former `stickleetoto/Yekaterina-Dev` repository at commit `6955d707c63efccdcf721e8369462cea9ee8d965`. That provenance remains part of the release record; active development has since been consolidated into this repository.

The v1.3 release preserves the frozen 1,410-operation v1.2 registry and engine as historical audit baselines, then exposes a live aggregate registry of 1,425 operations through v1.3 dispatch shims. The original v1.2 Full Capability Audit remains frozen evidence; the 15 new transformer-native operations are additionally exercised by the v1.3 real-process runtime verifier.

## What's new in v1.3

The stable operation set grows from **1,410 to 1,425** without increasing the three-tool MCP surface.

The v1.3 line adds 15 native `xfmr.*` operations covering:

- material, interpolation, thermal and basic geometry calculations;
- winding field, leakage and AC-loss calculations;
- deterministic candidate evaluation and ranking with explicit constraint evidence.

The implementation deliberately keeps the historical v1.2 registry and engine frozen. The v1.3 aggregate registry and dispatch layers add the transformer-native surface while delegating every legacy operation to the proven baseline.

Caller-supplied limits and objective bands are policy data. Registry promotion does not imply IEC/IEEE/DOE certification or factory-design approval.

## MCP surface

Yekaterina exposes exactly three MCP tools:

- `yk.find` — discover operations lazily
- `yk.spec` — inspect a selected operation
- `yk.compute` — execute single calls, batches, pipelines, and supported user operations

The design rule remains:

> Internal capability may grow without casually expanding the LLM-facing schema.

Operation discovery stays out of `tools/list`, so the registry can grow without forcing every operation into the model-visible tool schema.

## Verification evidence

The source promoted to v1.3.0 completed the finalization CI on the exact promoted development commit:

```text
static_audit_v13                         PASS
aggregate operation manifest             1425 total / 15 xfmr
cargo test --locked --all-targets         PASS
cargo clippy --locked --all-targets       PASS
cargo build --locked --release            PASS
MCP showcase demo                         PASS
transformer runtime verifier              PASS
v1.2 independent operation verifier       PASS
statistics reference verifier             PASS
multiplicity reference verifier           PASS
MCP Golden                                527/527
frozen v1.2 Full Capability Audit          1410/1410 --strict
benchmark invariants                       PASS
```

The Windows stable package is built from the exact promoted source commit with the committed lockfile, run through the real MCP demo and stable smoke test, packaged with release/license/privacy and dependency-inventory material, and accompanied by a SHA-256 checksum.

See [v1.3.0 release notes](releases/v1.3.0/RELEASE_NOTES.md) for the promotion record.

## Stable distribution vs development

Yekaterina now uses one public repository with branch-separated roles:

```text
stickleetoto/Yekaterina
    ├─ main
    │   └─ stable binaries, documentation, release evidence and issue tracking
    └─ dev/v1.4
        └─ Rust source development, optimization, verification and release preparation
```

Development changes are promoted from the development branch only after regression, compatibility and capability gates are checked against the frozen baselines. The former `stickleetoto/Yekaterina-Dev` repository is retained as migration provenance while the consolidated layout is validated.

## Download

Official stable binaries are distributed through **GitHub Releases**.

Download `Yekaterina_v1.3.0_windows-x64.zip` together with its `.sha256.txt` file, verify the checksum, then point your stdio-capable MCP client at `yekaterina.exe`.

See [Installation](docs/INSTALLATION.md) and [MCP Setup](docs/MCP_SETUP.md).

## Compatibility

The MCP request-schema surface remains compatible with the frozen v1.0.0 line. The server's MCP `initialize` response still advertises `1.0.0` deliberately; crate/release versioning can evolve without silently changing what existing MCP clients observe.

Every v1.2 canonical operation remains present in its original order. The default worker count remains 1 and parallel batch execution remains opt-in.

## License

The official Yekaterina Core binary distributed from `main` is provided under the [Yekaterina Freeware License v1.0](LICENSE.txt).

Development source on [`dev/v1.4`](https://github.com/stickleetoto/Yekaterina/tree/dev/v1.4) is distributed under the Apache License 2.0 provided by that branch's `LICENSE` file. Do not assume the stable binary distribution and development source have identical distribution terms.

Third-party components bundled in official binary releases retain their own licenses. Official release archives include the applicable component inventory and collected third-party license material.

## Security

Yekaterina compute operations do not expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access as compute operations. Formula evaluation and workload execution use bounded internal guards.

For vulnerability reporting, see [SECURITY.md](SECURITY.md).
