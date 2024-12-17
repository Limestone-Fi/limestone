# <date YYYY-MM-DD> #0
Report ``<number>`` for pertaining to fuzzing data collected on the date `<YYYY-MM-DD>`.

## Invariants
Invariants used in the suite.

**Key:**
| Symbol | Classification |
| --- | --- |
| ✅ | Invariant passed |
| ⚠️ | Invariant Failure, Suite Error |
| ❌ | Invariant Failure, Contract Error |
### Global
| ID | Description | Passed |
| --- | --- | --- |
| GLOBAL-01 | Lending pool total tokens should be greater than or equal to the amount of liquidity we have supplied | |
| GLOBAL-02 | Lending pool total tokens should be greater than or equal to the shares supply | |
| GLOBAL-03 | Warchest supply (as a proxy of total shares) should not exceed the underlying token supply | |
| GLOBAL-04 | Lending pool reserve pool should not hit the max uint112 value | |
| GLOBAL-05 | Lending pool global debt should not be less than the actual debt | |
| GLOBAL-06 | Lending pool debt shares should not be `0` despite having debt | |
| GLOBAL-07 | Lending pool debt shares are somehow greater than the total debt value | |
### LendingPool
| ID | Method | Description | Passed |
| --- | --- | --- | --- |
| LEND-01 | `deposit` | Shares received from the deposit should be the amount expected (based on share price calcs) | |
| WITHDRAW-1 | `withdraw` | Assets received from the withdrawal should be the amount expected (based on share price calcs) | |
| CREATE-LEGACY-01 | `doHardWork` | New position owner should be the expected actor (user) | |
| CREATE-LEGACY-02 | `doHardWork` | Debt shares issued to the position should be the amount expected (based on debt share calcs) | |
| CREATE-LEGACY-03 | `doHardWork` | Newly created position shouldn't exceed the max leverage set. | |
| INCREASE-LEGACY-01 | `increaseCollateral` | Position shouldn't somehow become overleveraged (edge case) | |
### PositionCoordinator
| ID | Method | Description | Passed |
| --- | --- | --- | --- |
| NEWINVEST-01 | `investInV2LikePosition` | Newly opened position should not be able to exceed max leverage (3x) | |
| ADDINVEST-01 | `investInV2LikePosition` | Position should not become overleveraged when adding/borrowing assets to the position | |
| DIVEST-01 | `divestFromV2LikePosition` | Position shouldn't be able to become overleveraged after removing assets | |
| DIVEST-02 | `divestFromV2LikePosition` | Debt repayment should not remove more debt shares than the position holds (paying off more debt than needed) | |
| REPAY-01 | `repayV2LikeLiquidityPositionDebt` | Debt repayment should not remove more debt shares than the position holds. | |
| REPAY-02 | `repayV2LikeLiquidityPositionDebt` | Lending pool total tokens shouldn't change (debt is already accounted for) | |

## Invariant Violations
### SAMPLE-1337
Classification: put classification here
#### Issue
Some sample issue.
#### Solution
Have you tried just turning it off and on again?

---

### Sample-0420
Classification: put classification here
#### Issue
Not an issue.
#### Solution
```
while true {
  somethingCool()
}
```
