# MCP setup

Yekaterina is a stdio MCP server. Configure your MCP-capable host to launch the official `yekaterina.exe` directly.

A generic configuration shape used by many MCP hosts looks like:

```json
{
  "mcpServers": {
    "yekaterina": {
      "command": "C:\\Tools\\Yekaterina\\v1.0.0\\yekaterina.exe",
      "args": []
    }
  }
}
```

Exact configuration keys differ by MCP host. Consult the documentation for your host.

## Exposed tools

```text
yk.find
yk.spec
yk.compute
```

Typical flow:

```text
1. yk.find   -> locate a relevant operation
2. yk.spec   -> inspect its compact argument/return contract
3. yk.compute -> execute it
```

Batch and pipeline execution allow many computations to be grouped under the same compact MCP surface.
