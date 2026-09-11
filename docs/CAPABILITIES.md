# Capability overview

Yekaterina v1.3.0 contains **1,425** registered built-in/control operations while exposing only three MCP tools.

Major capability areas include:

- arithmetic and arbitrary-precision integer/decimal computation,
- statistics, probability distributions, inference and confidence intervals,
- multiplicity correction, post-hoc tests, effect sizes and risk measures,
- matrices, linear algebra, SVD, pseudoinverse and PCA,
- numerical methods and root finding,
- optimization,
- ODE integration,
- series and approximation,
- special mathematical functions,
- geometry, vectors, curves and predicates,
- frame-aware rigid transforms,
- financial, percentage and unit calculations,
- verification and convergence checks,
- mechanics, fluids, thermodynamics and electrical calculations,
- optics, waves, geodesy and astronomy,
- chemistry, networking, information/data calculations,
- transformer-native material/interpolation/thermal/basic geometry operations,
- transformer winding-field, leakage and AC-loss calculations,
- deterministic transformer candidate evaluation and ranking,
- batch execution, pipelines, Formula UDOs and Composite UDOs,
- persistent user-operation snapshots and package-oriented extension support.

The v1.3 live registry is an aggregate surface: the frozen v1.2 catalog of 1,410 canonical operations is retained intact and followed by 15 native `xfmr.*` operations. Legacy execution continues through the historical v1.2 engine while transformer-native calls are dispatched through the v1.3 layer.

The MCP surface remains exactly `yk.find`, `yk.spec`, and `yk.compute`. Runtime discovery is intentionally lazy so the internal registry can grow without enumerating every operation in the model-facing tool schema.

The transformer candidate helpers provide deterministic computation from caller-supplied constraints and objective bands. They do not constitute IEC/IEEE/DOE certification or factory-design approval.

The public stable repository intentionally distributes binaries, documentation, release evidence, and packaging material rather than the engine source tree. Active source development and verification live in `stickleetoto/Yekaterina-Dev`.
