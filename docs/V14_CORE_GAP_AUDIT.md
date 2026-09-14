# Yekaterina v1.4 Core Gap Audit

## Purpose

Before adding more v1.4 Core operations, compare the v1.4 roadmap against the frozen v1.2/v1.3 catalog. The goal is to avoid promoting duplicate operations when the real gap is discovery.

## Result

A large part of the v1.4 Core Math roadmap is already covered by the frozen baseline.

| Roadmap capability | Existing operation | Assessment |
|---|---|---|
| Linear equation | `alg.linear_root` (v1.4) | Genuine v1.4 addition |
| Linear system solve | `mat.solve` (legacy), `linalg.solve` (v1.4) | Functional overlap; preserve/document the v1.4 numerical/error-contract distinction before further expansion |
| Quadratic equation | `alg.quadratic_roots` | Already covered |
| Polynomial evaluation | `alg.poly_eval` | Already covered |
| Ratio / proportion | `alg.proportion` (v1.4) | Genuine v1.4 addition |
| Percentage change | `pct.change` | Already covered |
| Percentage increase/decrease | `pct.increase`, `pct.decrease` | Already covered |
| CAGR | `fin.cagr` | Already covered |
| Matrix multiplication | `mat.mul` | Already covered |
| Matrix determinant | `mat.det` | Already covered |
| Matrix inverse | `mat.inverse` | Already covered |
| Matrix transpose | `mat.transpose` | Already covered |
| Matrix rank | `mat.rank` | Already covered |
| Dot product | `vec.dot` | Already covered |
| Vector norm | `vec.norm` | Already covered |
| Vector normalization | `vec.normalize` | Already covered |
| Vector distance | `vec.distance` | Already covered |
| Mean / median / mode | `stat.mean`, `stat.median`, `stat.mode` | Already covered |
| Variance / standard deviation | `stat.variance`, `stat.std` | Already covered |
| Percentile / quantile | `stat.percentile`, `stat.quantile` | Already covered |
| Covariance / correlation | `stat.covariance`, `stat.correlation` | Already covered |
| Clamp | `math.clamp` | Already covered |
| Linear interpolation | `math.lerp` | Already covered |
| Root finding | `num.bisect`, `num.newton` | Already covered |
| Numerical integration | `num.trapezoid`, `num.simpson_uniform`, `num.integrate` | Already covered |
| Significant-figure rounding | `num.round_sigfig` (v1.4) | Genuine v1.4 addition |

## Decision for the current slice

Prioritize **agent discovery density** over operation count.

The first discovery expansion maps common natural-language requests to existing canonical operations, including statistics, percentages, finance, matrix operations, interpolation and numerical methods. This keeps the MCP surface unchanged and makes already-implemented capability easier for agents to reach.

## Guardrail for further v1.4 work

Before registering another Core operation:

1. Search the frozen v1.2 registry by canonical name, aliases and capability family.
2. Check adjacent implementations whose names differ but semantics overlap.
3. Prefer a `yk.find` semantic bridge when the capability already exists.
4. Add a new Core opcode only when a real capability or contract gap remains.
5. Require unit and real-process MCP discovery verification for new semantic bridges.

Operation count is not a v1.4 success metric. Reachable, deterministic capability is.
