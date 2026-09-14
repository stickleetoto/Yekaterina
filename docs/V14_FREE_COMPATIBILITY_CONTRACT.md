# Yekaterina v1.4 Free Compatibility Contract

## Status

This document defines the stable compatibility contract for the Yekaterina v1.4 free line.

## Product role

Yekaterina v1.4 is the durable **free baseline** before post-v1.4 feature development moves to the private commercial line.

The purpose of v1.4 is not to maximize operation count. It is to provide a stable, deterministic, compact compute surface that agents can reliably discover and call.

## Stable MCP surface

The v1.4 line exposes exactly three model-facing MCP tools:

- `yk.find`
- `yk.spec`
- `yk.compute`

A v1.4 maintenance release must not add a fourth MCP tool or remove one of these tools.

The MCP initialize identity remains compatibility-gated and must not be changed merely to mirror the package version.

## Operation compatibility

For operations included in the final v1.4.0 manifest:

1. canonical operation IDs are stable across v1.4.x;
2. an existing canonical ID must not silently change semantics;
3. required inputs must not be removed or reinterpreted incompatibly;
4. existing deterministic error classes must not be repurposed to mean unrelated failures;
5. aliases and semantic-discovery hints may be added when they only improve reachability of existing capability;
6. a maintenance release should not add a new operation unless required to correct a release-blocking capability or contract defect and explicitly approved.

## Discovery behavior

`yk.find` may use additive semantic hints to map common natural-language intent to an existing canonical operation.

Precedence must preserve the following principle:

1. exact canonical operation identity;
2. exact/established aliases;
3. semantic discovery hints;
4. broader lexical ranking.

Semantic discovery must not silently rename the canonical operation returned by `yk.spec` or executed by `yk.compute`.

## Execution behavior

The v1.4 free line preserves these release expectations:

- deterministic local execution for supported operations;
- bounded execution rules already enforced by the runtime;
- no arbitrary shell execution as a compute operation;
- no arbitrary network access as a compute operation;
- no arbitrary filesystem access as a compute operation;
- default worker behavior remains unchanged unless a correctness or safety defect requires a reviewed fix.

## Verification requirements

The exact v1.4.0 source is promotable only after the full release gate passes, including static audit, manifest validation, locked Rust test/clippy/build, MCP demo, v1.3/v1.4 runtime verifiers, retained v1.2 reference verifiers, Golden regression, frozen Full Capability Audit, and benchmark invariants.

The release package must then pass its packaged-binary smoke test before publication.

## Maintenance policy

After v1.4.0, the v1.4 free line is maintenance-oriented.

Appropriate v1.4.x changes include:

- correctness fixes;
- security fixes;
- compatibility fixes;
- packaging/install documentation fixes;
- additive discovery hints that do not change execution semantics.

Feature development intended to create new commercial value belongs to the post-v1.4 private line rather than being backported into the free baseline by default.

## Boundary to v1.5+

The final v1.4 release source is the public free-line boundary.

Post-v1.4 commercial development uses a private development location. The public repository may continue to contain binaries, release notes, public documentation, SDK/interface material, checksums, and support/security information, but proprietary post-v1.4 implementation source is not developed on public branches.
