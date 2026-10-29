# C11.4 `fit-content()` column axis 계획 실패 관점 검토 20개

**검토 대상:** [열 축 계획](./c11-4-fit-content-column-axis.md), [Chrome 156 precomparison](../spec/internal/evidence/c11-4-fit-content-columns-precomparison-2026-10-30/README.md), 고정 fixture 및 expected JSON. 각 관점은 계획의 서로 다른 입력·계산·출력·플랫폼 경계를 확인한다.

| # | 실패 관점 | 판정 및 방어 경계 |
| ---: | --- | --- |
| 1 | Chrome의 used track을 표준 기대값으로 복사한다 | **통과.** §11.2 식 `max(minimum,min(limit,max-content))`과 0/100px 기여에서 expected 30/40/100px을 독립 계산하고 Chrome은 별도 관측으로 둔다. |
| 2 | `fit-content(200px)`이 60px root의 definite available space에 잘려 argument 경계를 구별하지 못한다 | **통과.** 해당 장면의 definite root width를 300px로 둔다. 초기 60px 입력은 cap 판정 불가로 확인하고 최종 precomparison에서 제외했다. |
| 3 | Main scene의 automatic minimum이 limit 아래로 내려가지 않아 30/40px cap을 검사하지 못한다 | **통과.** main scene은 `min-width:0`을 고정하고 Chrome computed value와 item frame을 확인한다. |
| 4 | `min-width:0` 때문에 max-content contribution도 0으로 사라진다 | **통과.** 100px fixed child가 max-content input이며 `MaximizeTracks` contribution을 runtime assertion으로 분리한다. |
| 5 | Floor control이 main case와 동일한 zero minimum을 사용해 automatic minimum을 검증하지 못한다 | **통과.** control만 `min-width:auto`이고 100px child, 40px limit에서 100px used column이 기대값이다. |
| 6 | `grid-auto-columns`가 아닌 explicit template column이 사용된다 | **통과.** 고정 입력은 `grid-template-columns:none`과 `grid-auto-columns:fit-content(...)`를 사용한다. |
| 7 | Implicit column 생성을 위해 item이 row 1에 놓이지 않아 다른 implicit axis를 만든다 | **통과.** explicit row 60px, `grid-auto-flow:column`, 단일 direct child로 column만 추가한다. |
| 8 | Row contribution이 row track 크기를 바꾸어 열 축과 섞인다 | **통과.** 60px explicit row와 10px child height를 고정하고 Columns/Rows trace를 각각 확인한다. |
| 9 | `justify-items` 또는 self alignment가 item 폭을 used track과 다르게 만든다 | **통과.** `justify-items:stretch`를 고정하고 item width와 used column을 모두 검사한다. |
| 10 | 30/40px item의 child overflow가 clip되어 track cap 경계가 감춰진다 | **통과.** `overflow:visible`을 고정하고 child frame과 별도 surface coordinates를 사용한다. |
| 11 | 200px control이 root/viewport 밖이라 finite max-content 한계를 측정하지 못한다 | **통과.** root width 300px이며 precomparison viewport 1280px에서 root/item/child 전체가 보인다. |
| 12 | Floor control의 root가 60px라 100px item을 viewport 밖으로 잘라낸다 | **통과.** root overflow는 visible이고 implementation pixel samples는 root 밖·viewport 안의 좌표를 지정한다. |
| 13 | Root border/padding이 available size와 sample 좌표를 바꾼다 | **통과.** root/item/child margin·padding·border는 0이고 box sizing/60·300px content width를 고정한다. |
| 14 | Chrome CSSOM used track과 DOMRect frame이 서로 다른 계층 값인데 하나로 합산된다 | **통과.** `gridTemplateColumns`, root, item, child를 separate observation fields로 저장한다. |
| 15 | Browser/source hash 또는 이미지가 다른 fixture revision에 묶인다 | **통과.** fixture SHA-256, screenshot SHA-256, Agent Browser 0.31.1, HeadlessChrome 156, viewport 및 DPR을 기록한다. |
| 16 | 고정 screenshot을 pixel equality나 app runtime proof로 과장한다 | **통과.** 계획과 README는 screenshot을 precomparison 표시자료로만 쓰며 PixelCopy/Metal은 구현 관문으로 분리한다. |
| 17 | 신규 route가 일반 CSS admission이나 product FFI에 노출된다 | **통과.** 구현은 기존 fixture-test feature와 exact scene IDs만 허용하고 default/product artifact negative control을 요구한다. |
| 18 | Android/iOS 결과가 서로 다른 source revision 또는 scene allowlist를 사용한다 | **통과.** 최종 동일 commit, build hash, 네 scene별 matcher와 draw/present revision을 두 플랫폼에서 요구한다. |
| 19 | Raw max-content growth limit을 fit-content used base와 같다고 오판한다 | **통과.** raw finite growth limit 100px과 30/40/100px used base를 별도 assertion으로 둔다. |
| 20 | 제한 scene을 C11.4 완료 또는 공식 count 증가로 잘못 표시한다 | **통과.** `STATUS`, parent plan과 plan은 C11.4 unchecked, official `72/213`, implementation/runtime not started를 유지한다. |

**판정:** 계획 실패 관점 20/20을 통과했다. 본 기록은 계획 검토이며 구현/runtime 검토나 plan PR diff 검토를 대신하지 않는다. 발견된 initial root-width 혼입은 최종 fixture를 고친 뒤 관점 2에서 재검토했다.
