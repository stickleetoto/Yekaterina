# Security Policy

## Supported version

| Version | Security fixes |
|---|---|
| 1.0.x | Yes |

## Reporting a vulnerability

Please do **not** publish exploit details in a normal public issue. Use GitHub's private vulnerability reporting / Security Advisory feature for this repository when available.

Include:

- affected Yekaterina version,
- operating system,
- reproduction steps,
- expected vs observed behavior,
- whether arbitrary code execution, filesystem access, network access, persistence, or denial of service is involved.

## Security boundary

Yekaterina v1 is designed as a local stdio MCP compute engine. Compute operations do not intentionally expose arbitrary shell execution, arbitrary network access, or arbitrary filesystem access. Resource limits exist for batches, pipelines, expression evaluation, user operations, and numerical workloads.

No software security statement is an absolute guarantee. Security claims apply to the documented release and tested threat boundary.
