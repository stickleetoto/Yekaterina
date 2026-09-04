# Capability overview

Yekaterina v1.0.0 contains **1,215** registered built-in/control opcodes while exposing only three MCP tools.

Major capability areas include:

- arithmetic, exact integer and decimal computation,
- statistics and probability,
- matrices, linear algebra, SVD, pseudoinverse and PCA,
- numerical methods and root finding,
- optimization,
- ODE integration,
- series and approximation,
- special mathematical functions,
- geometry, vectors, curves and predicates,
- frame-aware rigid transforms,
- verification and convergence checks,
- mechanics, fluids, thermodynamics and electrical calculations,
- optics, waves, geodesy and astronomy,
- chemistry, networking, information/data calculations,
- batch execution, pipelines, Formula UDOs and Composite UDOs,
- persistent user-operation snapshots and package-oriented extension support.

The public repository intentionally does not publish the private engine implementation or a full source-code registry dump. Runtime discovery is provided through `yk.find` and `yk.spec`.
