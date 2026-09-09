# Changelog

## v1.2.0 — Stable operation expansion

- Promoted the verified `v1.2.0-rc1` source line from `stickleetoto/Yekaterina-Dev` at `9019194af02f7b9cbe72c5a232e753e236a84b4f`.
- Expanded the registered built-in/control operation set from **1,215 to 1,410**.
- Kept exactly **3** exposed MCP tools: `yk.compute`, `yk.find`, `yk.spec`.
- Kept the MCP schema footprint at **412 tokens / 1,725 bytes**.
- Added exact/applied arithmetic, statistical inference, multiplicity/post-hoc/effect-size, and risk-measure families.
- Included the v1.1 ordered parallel batch infrastructure while keeping the default worker count at 1.
- Fixed the v1.1 `expr.eval` worker-classification defect.
- Golden correctness: **527/527**.
- Full Capability Audit: **1,410/1,410** under `--strict`.
- Rust test executions: **386 / 0 failures**; `cargo clippy --locked --all-targets` PASS.
- Independent v1.2 verifiers: 164 operation assertions, 961 statistics reference checks, and 577 multiplicity/effect-size assertions.
- MCP showcase demo: `DEMO PASS`.
- Mutation gates: **6/6** caught.
- MCP `initialize` advertised version remains **1.0.0** deliberately for client compatibility.

## v1.0.0 — Frozen V1 baseline

- Promoted the fully verified alpha.12-hotfix9 runtime to V1.
- 1,215 registered built-in/control opcodes.
- Exactly 3 exposed MCP tools: `yk.compute`, `yk.find`, `yk.spec`.
- 527/527 MCP Golden correctness cases.
- 1,215/1,215 live `yk.spec` coverage.
- 1,215/1,215 live MCP execution fixture coverage.
- 1,215/1,215 clean replay / return-type contract coverage.
- Self-regression hard gate: PASS; verdict: CURRENT WINS.
