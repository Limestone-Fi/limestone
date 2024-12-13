# Limestone Cancun Workspace
This is the workspace for Limestone's latest and upcoming smart contract updates. Due to EVM differences between the Cancun upgrade and previous versions of the EVM, this workspace is separate from the core workspace as to maintain backwards compatibility with other earlier EVM versions that some of our deployments still use.

## Protocol Updates
- Lending Pool facet optimizations, including the introduction of new features such as transient storage for temporarily storing the worker execution scope instead of relying on SSTORE and SLOAD.
- SSTORE2 for storing large data structures related to the lending pool. Includes a migration from the original ``Market`` struct to the new SSTORE2 version.
- Delegated health management to enable contracts capable of more complex borrowing strategies.
- Smart contract whitelist to enable new protocol primitives and the ability to tap into the lending pool assets.
- Position Coordinator contract which enables users to borrow on both sides of a liquidity pool. Manages liquidations on its own through the new delegated health management feature.
- Multi Modal Worker, capable of supporting lending positions made by the Position Coordinator.
- Protocol QoL changes made to avoid future technical debt.

## Building
To build locally: ``forge build``
To build from the root monorepo: ``[npx] nx build cancun``
