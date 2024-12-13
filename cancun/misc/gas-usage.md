Compiling 2 files with Solc 0.8.28
Solc 0.8.28 finished in 2.30s
Compiler run successful!

Ran 3 tests for test/LendingPool.t.sol:LendingPoolTest
[PASS] testDeposit() (gas: 280211)
[PASS] testDepositAndWithdraw() (gas: 362744)
[PASS] testWorkNormal() (gas: 511397)
Suite result: ok. 3 passed; 0 failed; 0 skipped; finished in 1.66s (517.02ms CPU time)
| src/LendingPool.sol:LendingPool contract |                 |        |        |        |         |
|------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                          | Deployment Size |        |        |        |         |
| 3105853                                  | 14144           |        |        |        |         |
| Function Name                            | min             | avg    | median | max    | # calls |
| addLendingPool                           | 125155          | 125155 | 125155 | 125155 | 3       |
| deposit                                  | 161762          | 161762 | 161762 | 161762 | 3       |
| doHardWork                               | 141189          | 141189 | 141189 | 141189 | 1       |
| initialize                               | 90599           | 90599  | 90599  | 90599  | 3       |
| withdraw                                 | 86886           | 86886  | 86886  | 86886  | 1       |


| test/mocks/MockToken.sol:MockToken contract |                 |       |        |       |         |
|---------------------------------------------|-----------------|-------|--------|-------|---------|
| Deployment Cost                             | Deployment Size |       |        |       |         |
| 465841                                      | 2436            |       |        |       |         |
| Function Name                               | min             | avg   | median | max   | # calls |
| approve                                     | 45957           | 45957 | 45957  | 45957 | 4       |
| balanceOf                                   | 537             | 870   | 537    | 2537  | 6       |
| mint                                        | 50769           | 63594 | 67869  | 67869 | 4       |


| test/mocks/MockWarchest.sol:MockWarchest contract |                 |      |        |      |         |
|---------------------------------------------------|-----------------|------|--------|------|---------|
| Deployment Cost                                   | Deployment Size |      |        |      |         |
| 582886                                            | 2981            |      |        |      |         |
| Function Name                                     | min             | avg  | median | max  | # calls |
| underlyingBalanceWithInvestment                   | 3050            | 4175 | 3050   | 7550 | 4       |




Ran 1 test suite in 2.45s (1.66s CPU time): 3 tests passed, 0 failed, 0 skipped (3 total tests)
