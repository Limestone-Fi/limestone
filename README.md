# Limestone Monorepo
Monorepo containing code pertaining to Limestone's contracts, API, etc.

## Monorepo Structure
- ``cancun`` - Latest Limestone contracts, such as the PositionCoordinator and MultiModalWorker.
- ``core`` - Limestone's legacy contracts such as the old lending pool facet.
- ``simulations`` - Arbiter agent-based simulations used in testing/auditing Limestone contracts and lending environment.

## Tasks
### Overall Syntax
Use the following to run specific tasks pertaining to one of the projects:
```
nx <target> <project-name>
```

### Building (Contracts)
```
nx build <foundry-project-name>
```
### Testing (Contracts)
```
nx build <foundry-project-name>
```
