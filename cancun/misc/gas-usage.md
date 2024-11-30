No files changed, compilation skipped

Ran 3 tests for test/LendingPool.t.sol:LendingPoolTest
[PASS] testDeposit() (gas: 287301)
[PASS] testDepositAndWithdraw() (gas: 376940)
[PASS] testWork() (gas: 518828)
Suite result: ok. 3 passed; 0 failed; 0 skipped; finished in 1.20ms (1.09ms CPU time)
| src/LendingPool.sol:LendingPool contract |                 |        |        |        |         |
|------------------------------------------|-----------------|--------|--------|--------|---------|
| Deployment Cost                          | Deployment Size |        |        |        |         |
| 2960775                                  | 13474           |        |        |        |         |
| Function Name                            | min             | avg    | median | max    | # calls |
| addLendingPool                           | 128677          | 128677 | 128677 | 128677 | 3       |
| deposit                                  | 168852          | 168852 | 168852 | 168852 | 3       |
| doHardWork                               | 141486          | 141486 | 141486 | 141486 | 1       |
| initialize                               | 90643           | 90643  | 90643  | 90643  | 3       |
| withdraw                                 | 93992           | 93992  | 93992  | 93992  | 1       |


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




Ran 1 test suite in 2.54ms (1.20ms CPU time): 3 tests passed, 0 failed, 0 skipped (3 total tests)
