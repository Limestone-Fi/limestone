# Limestone Core Workspace
This is the workspace for Limestone's original v0 core smart contracts. These are the contracts that have been used preceding the "Diagenesis" update (found in the ``cancun`` workspace). These contracts have been tested and are still the main core contracts utilized on chains that lack support for Cancun. That said, when the opportunity to deploy the Cancun-compatible version is available, it is advised to use the codebase from the ``cancun`` instead.

## Differences Between ``core`` & ``cancun``
- Lack of specific gas optimizations including TSTORE, SSTORE2, etc.
- No Multi Modal Worker.
- No delegated debt.
- No Position Coordinator.

## Building
To build locally: ``forge build``
To build from the root monorepo: ``[npx] nx build cancun``
