# Capability overview

Yekaterina v1.2.0 contains **1,410** registered built-in/control operations while exposing only three MCP tools.

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
- batch execution, pipelines, Formula UDOs and Composite UDOs,
- persistent user-operation snapshots and package-oriented extension support.

The MCP surface remains exactly `yk.find`, `yk.spec`, and `yk.compute`. Runtime discovery is intentionally lazy so the internal registry can grow without enumerating every operation in the model-facing tool schema.

The public stable repository intentionally distributes binaries, documentation, release evidence, and packaging material rather than the engine source tree. Active source development and verification live in `stickleetoto/Yekaterina-Dev`.
