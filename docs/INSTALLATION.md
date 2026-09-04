# Installation — Windows x64

1. Download the official `Yekaterina_v1.0.0_windows-x64.zip` release asset from GitHub Releases.
2. Download or view the accompanying SHA-256 checksum.
3. Verify the archive or executable checksum before use.
4. Extract to a stable local directory, for example:

```text
C:\Tools\Yekaterina\v1.0.0\
```

5. Configure your MCP host to launch `yekaterina.exe` over stdio.

Yekaterina does not require the public GitHub repository to remain present after installation.

## Verification

Official release archives should include `SMOKE_TEST_WINDOWS.bat` and `tools/smoke_test.py`. With Python 3 available, run:

```powershell
.\SMOKE_TEST_WINDOWS.bat
```

Expected minimum result:

```text
MCP tools: yk.compute, yk.find, yk.spec
math.add smoke: 42
PASS
```
