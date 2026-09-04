# Yekaterina v1.0.0 — First frozen release

Yekaterina v1.0.0 is the first frozen stable baseline of the proprietary freeware Yekaterina Core.

## Highlights

- 1,215 registered compute/control opcodes.
- Exactly 3 exposed MCP tools.
- 527/527 Golden correctness cases.
- 1,215/1,215 live `yk.spec` coverage.
- 1,215/1,215 full MCP execution fixture coverage.
- 1,215/1,215 clean replay / return-type contract coverage.
- Self-regression hard gate PASS against the frozen alpha.10 baseline.
- +15.28% capability vs alpha.10 with unchanged 412 schema tokens and unchanged 159,794 fixed-10k wire tokens.
- V1 core frozen; future domain growth moves to DLC/extensions.

## Distribution

Yekaterina Core is free to use under the Yekaterina Freeware License v1.0 but is not open source. Official binaries are distributed through GitHub Releases. Third-party dependencies retain their own licenses and notices.

## Verification scope

Full execution coverage proves live MCP reachability and declared return-type contracts. It does not claim mathematical proof over every possible input.
