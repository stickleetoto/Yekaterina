# Benchmark record

## V1 self-regression result

Yekaterina v1.0.0 was compared against the frozen alpha.10 baseline.

| Metric | alpha.10 | v1.0.0 | Change |
|---|---:|---:|---:|
| Registered opcodes | 1,054 | 1,215 | **+15.28%** |
| MCP tools | 3 | 3 | 0% |
| Schema tokens | 412 | 412 | 0% |
| Fixed 10k wire tokens | 159,794 | 159,794 | 0% |
| Fixed 10k arithmetic accuracy | 100% | 100% | unchanged |
| Fixed 10k MCP time | 24.6739 ms | 26.8091 ms | +8.65% |
| Hard regression gate | — | PASS | **CURRENT WINS** |

The primary architectural result is that capability increased by 15.28% while the exposed tool count, schema-token footprint, and fixed-workload wire-token cost remained unchanged.

## Historical external comparison

Earlier development compared Yekaterina with Arithma v0.3.0 on the same MCP/JSON-RPC measurement methodology. The recorded baseline showed a reduction from 16,896 tool-schema tokens to 412 (97.56%) and a fixed 10,000-operation wire-token reduction from 812,100 to 159,794 (80.32%).

These are protocol/tokenization measurements for the benchmark harness, **not provider-billed LLM token claims**.

## Reproducibility note

Latency and RSS are environment-sensitive. Token/schema measurements are stronger regression invariants than wall-clock timing across different machines or background-load conditions.
