# Third-party notices for binary releases

This repository does not contain the private Yekaterina engine source or its Cargo dependency tree.

Before publishing an official binary release, use `MAKE_WINDOWS_RELEASE.bat` with the locally built V1 executable and the private Cargo project directory. The script uses the **locked** Cargo dependency graph to generate:

- `THIRD_PARTY_COMPONENTS.md`
- `third_party_licenses/`
- executable SHA-256
- final release ZIP + ZIP SHA-256

Review the generated notices before publication. Third-party license obligations are independent of the proprietary Yekaterina Core license.
