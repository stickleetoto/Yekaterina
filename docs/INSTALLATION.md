# Installation — Windows x64

1. Download the official `Yekaterina_v1.3.0_windows-x64.zip` release asset from GitHub Releases.
2. Download the accompanying `.sha256.txt` checksum file.
3. Verify the archive checksum before use.
4. Extract to a stable local directory, for example:

```text
C:\Tools\Yekaterina\v1.3.0\
```

5. Configure your MCP host to launch `yekaterina.exe` over stdio.

Yekaterina does not require the public GitHub repository to remain present after installation.

## Verification

Official release archives include `SMOKE_TEST_WINDOWS.bat` and `tools/smoke_test.py`. With Python 3 available, run the smoke test against the packaged executable:

```powershell
.\SMOKE_TEST_WINDOWS.bat .\yekaterina.exe
```

Expected minimum result:

```text
MCP tools: yk.compute, yk.find, yk.spec
math.add smoke: 42
PASS
```

For the full v1.3.0 promotion evidence, see [Verification](VERIFICATION.md) and the [v1.3.0 release notes](../releases/v1.3.0/RELEASE_NOTES.md).
