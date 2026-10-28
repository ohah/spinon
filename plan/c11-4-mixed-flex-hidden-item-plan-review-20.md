# C11.4 `display:none` mixed-track 계획 · 실패 관점 검토 20개

**대상:** [집중 계획](./c11-4-mixed-flex-hidden-item.md) 및 [사전 비교·독립 기대값](../spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/README.md)

계획을 구현하기 전에 표준 의미, 수치 판정, 입력 경계와 플랫폼 근거를 서로 다른 관점으로 대조했다. 이 검토는 구현/runtime 검토나 PR diff 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | `display:none`이 box tree와 Grid sizing 참여에서 실제로 제외되는 규칙이 모호함 | **통과.** CSS Display Level 3 box-generation 규칙을 고정 링크로 인용하고 hidden span subtree의 box 부재를 명시했다. |
| 2 | `visibility:hidden`을 `display:none`과 같은 제외 처리로 오인 | **통과.** visibility는 box/layout 참여를 유지하고 runtime에서 unsupported control로 거부하도록 분리했다. |
| 3 | 150px hidden text 값이 arbitrary font estimate에 의존 | **통과.** Ahem 10px, 15 `X`, Chrome font readiness와 독립 폭을 고정하고 150px visible control로 관찰했다. |
| 4 | 보이는 형제 `a`의 100px base도 hidden 처리로 함께 빠짐 | **통과.** `a`는 별도 직접 text item이며 두 hidden scene에서 첫 base를 100px로 기대한다. |
| 5 | visible control이 실제로 intrinsic base를 변화시키지 않는 무효 대조군 | **통과.** Chrome 156에서 visible control tracks `[150,40,90]`, hidden tracks `[100,40,140]`을 관찰했다. |
| 6 | Freeze 비교 방향 또는 strict boundary가 반대로 구현됨 | **통과.** 초기 fraction 80px보다 100px/150px base가 커 첫 track이 freeze되는 독립 계산이다. |
| 7 | Fixed neighbor나 두 gutter를 빼지 않아 flexible space가 틀림 | **통과.** 300−40−10−10=240px, 초기 1fr:2fr fraction은 80px로 기록했다. |
| 8 | Freeze 후 재분배가 남은 track factor 2를 무시 | **통과.** 100px hidden case의 잔여 140px, visible 150px control의 잔여 90px을 각각 명시했다. |
| 9 | Grid track과 gutter 합이 definite axis를 넘거나 부족 | **통과.** 두 hidden 및 browser control scene 모두 track 합+20px gutter가 300px이다. |
| 10 | row-axis text contribution이 orthogonal writing mode 때문에 다른 축을 측정 | **통과.** `vertical-rl`을 고정해 Ahem inline contribution을 row sizing 축에 두고 Chrome row 결과를 보존했다. |
| 11 | 이미지가 40px fixed neighbor sizing을 오염 | **통과.** natural size 24×16px의 direct image를 가운데 40px track에 두고 start alignment를 유지한다. |
| 12 | hidden `<span>`의 placement가 첫 track 밖이라 track base 비교가 무효 | **통과.** 두 axis 모두 hidden span과 `a`를 첫 explicit grid cell에 놓는다고 고정했다. |
| 13 | 잘못된 DOM child count/order가 어댑터에서 generic path로 fallthrough | **통과.** 정확한 네-child shape와 오류 시 fail-closed/partial-output 부재를 요구한다. |
| 14 | nested hidden subtree나 추가 text가 숨김 예외를 넓힘 | **통과.** span의 direct text node 하나만 허용하고 모든 추가 자식/공백 텍스트 변조를 거부한다. |
| 15 | CSS parser의 `display:none` computed value를 adapter가 검사하지 않음 | **통과.** computed display, profile, style, placement 및 revision 검사를 계약에 포함했다. |
| 16 | hidden text가 text measurement 또는 intrinsic trace에 등록됨 | **통과.** hidden node는 paragraph/intrinsic input에 없어야 하고 contribution trace에 나타나지 않아야 한다. |
| 17 | zero frame bookkeeping을 실제 CSS layout box 생성으로 표현 | **통과.** zero frame은 internal output 표현만이며 CSS box 부재 및 draw item 부재와 구분한다. |
| 18 | stale snapshot이나 축/track 변조가 새 known scene으로 부분 승인 | **통과.** stale revision, wrong axis, changed tracks/gaps/sizes/profile과 source mutations를 negative control로 둔다. |
| 19 | 계획의 두 runtime scene을 기존 여덟 장면 또는 전체 C11.4로 확대 | **통과.** 10-scene 회귀와 C11.4 미완료·72/213 유지, 전체 CSS/WPT 비주장 범위를 썼다. |
| 20 | Chrome 화면만으로 Android/iOS layout/draw 또는 pixel pass를 주장 | **통과.** Chrome 자료는 사전 browser oracle control로 한정하고 앱 검증은 구현 후 양 simulator 근거로 별도 요구한다. |

**판정:** 이 계획의 범위에서 구현을 막는 미해결 논리 결함은 발견하지 못했다. 150px visible control은 browser oracle에만 두고, runtime은 두 exact hidden scene만 받아야 한다. 계획 PR의 실제 변경 연결성은 별도 30관점 문서에서 검토한다.
