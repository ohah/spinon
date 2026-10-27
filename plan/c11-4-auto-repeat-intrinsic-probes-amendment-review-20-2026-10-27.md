# C11.4 auto-repeat intrinsic 재계산 · absolute probe 범위 검토 20개

검토 대상은 [집중 계획](./c11-4-auto-repeat-intrinsic-recompute.md)에 추가한 WPT의 네 out-of-flow Grid probe 지원 조건이다. 기존 C11.4 layout profile은 positioned Grid item을 거부하므로, runtime 재현을 통과시키려면 WPT의 정확한 probe 형식만 구분해 허용해야 한다.

| # | 별도 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | probe 수가 세 개 또는 다섯 개인 입력도 통과할 수 있음 | **PASS.** 네 개의 서로 다른 열 line pair가 모두 있을 때만 허용한다. |
| 2 | probe placement 순서가 기대 line과 섞일 수 있음 | **PASS.** pair `1/2`, `2/3`, `3/4`, `4/5`를 집합으로 한 번씩 요구한다. |
| 3 | probe가 in-flow item 수에 포함되어 auto-placement 결과를 바꿀 수 있음 | **PASS.** 허용 probe는 in-flow child count와 track contribution에서 제외한다. |
| 4 | probe가 intrinsic contribution을 바꾸어 repeat count를 오염시킬 수 있음 | **PASS.** absolute child는 Taffy Grid placement/sizing item 집합 밖의 OOF 경로로 둔다. |
| 5 | probe의 row placement가 뜻밖의 implicit row를 만들 수 있음 | **PASS.** 두 row line은 모두 auto로 제한한다. |
| 6 | width 100%가 containing block이 아닌 root 폭으로 계산될 수 있음 | **PASS.** Grid line pair가 만든 area의 width를 probe width의 containing basis로 검증한다. |
| 7 | root의 relative position이 빠져 다른 containing block을 참조할 수 있음 | **PASS.** root가 relative이고 probe가 그 Grid의 직계 자식인 경우만 허용한다. |
| 8 | inset 값 때문에 static position이 달라질 수 있음 | **PASS.** 네 inset을 모두 auto로 고정한다. |
| 9 | probe height 변경이 grid contribution처럼 처리될 수 있음 | **PASS.** 높이 5px의 OOF frame만 확인하고 track sizing 입력에서 제외한다. |
| 10 | width·height minimum/maximum clamp가 probe frame을 바꿀 수 있음 | **PASS.** WPT 조건 외 dimension constraint와 aspect ratio는 거부한다. |
| 11 | probe 자체의 자식·text가 layout 경로를 확장할 수 있음 | **PASS.** probe는 leaf block으로 제한한다. |
| 12 | probe가 hidden/none 처리되어도 성공으로 셀 수 있음 | **PASS.** 네 probe 모두 visible direct child여야 한다. |
| 13 | 중복 line pair가 빠진 geometry를 대신할 수 있음 | **PASS.** 중복 pair는 허용하지 않고 네 pair의 전체 존재를 확인한다. |
| 14 | order 또는 alignment 차이가 area 좌표를 가릴 수 있음 | **PASS.** order는 0이고 다른 probe style은 고정된 기본값을 요구한다. |
| 15 | padding/border/margin이 grid area size에 더해질 수 있음 | **PASS.** probe의 padding·border·margin은 0으로 고정한다. |
| 16 | row axis 또는 두 번째 auto-repeat으로 지원 범위가 확장될 수 있음 | **PASS.** 이 probe 예외는 열 auto-fill 한 축에서만 허용한다. |
| 17 | OOF probe를 static/in-flow로 바꾸어도 회귀가 통과할 수 있음 | **PASS.** positioning kind는 absolute이며 static 변형은 OOF predicate를 만족하지 못한다. |
| 18 | Grid line resolve 단계가 repeat count 이전의 authored track 수를 쓸 수 있음 | **PASS.** probe geometry는 auto-repeat 확장 후의 line edge로 비교한다. |
| 19 | 다른 absolute Grid item이 같은 exception으로 허용될 수 있음 | **PASS.** line pair와 모든 dimension, inset, parent 조건이 맞지 않는 OOF item은 fail closed한다. |
| 20 | simulator frame이 같은-document mutation evidence로 과장될 수 있음 | **PASS.** runtime은 동일 document sequence, simulator는 각 독립 scene의 앱 경로로 구분한다. |

**판정:** probe 범위 검토 20/20 PASS. 구현 후 새 runtime/implementation 실패 관점 20개와 PR diff 관점 30개를 별도로 검토한다.
