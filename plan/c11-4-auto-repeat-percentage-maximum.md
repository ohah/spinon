# C11.4 · definite percentage auto-repeat maximum 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)
**상태:** 구현 전 계획. Chrome 156 Agent Browser 관찰과 CSS Grid 2025 CRD 독립 계산을 고정했다. C11.4는 미완료이며 이 변경은 공식 집계를 수정하지 않는다.

## 결손과 목표

현재 opt-in auto-repeat profile은 minmax minimum으로 px, positive percentage 또는 제한된 calc를 허용하지만 maximum은 px 또는 flex에 한정한다. cascade gate와 layout gate가 percentage maximum을 거부한다. 그러나 auto-repeat의 반복 수는 definite maximum을 먼저 사용하고, maximum과 minimum이 모두 definite이면 maximum을 minimum 이상으로 floor한 뒤 gap을 포함해 가장 큰 반복 수를 고른다. vendored Taffy 0.15.0의 auto-repeat 계산도 이 규칙을 구현하지만 Spinon의 typed projection gate가 해당 입력까지 전달하지 않는다.

definite positive content-axis subset에서 fixed percentage maximum을 projection하고 기존 Taffy sizing 경로에 연결한다. 직접 percentage와 양의 length-plus-percentage calc를 처리하고, max < min 및 zero maximum을 min으로 floor하는 경계를 함께 고정한다. 새 parser나 계산기를 추가하지 않는다.

## 규범 근거와 사전 비교

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1·§7.2.3.2·§11.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)다. auto-repeat은 fixed-size grammar만 받고, definite preferred/max size에서 각 repeated track의 definite maximum을 쓰며, min과 max가 모두 definite이면 max를 min보다 작지 않게 floor한다. 계산에서 gap도 track sizing function과 함께 고려한다. auto-fit은 동일한 repeat count를 만든 뒤 item placement 이후 비어 있는 repeated tracks와 인접 gap을 접는다.

[고정 Chrome fixture](../spec/internal/evidence/c11-4-auto-repeat-percentage-maximum-precomparison-2026-10-23/oracle.html), [Agent Browser Chrome 156 관찰 JSON](../spec/internal/evidence/c11-4-auto-repeat-percentage-maximum-precomparison-2026-10-23/chrome-observation.json), [CSS Grid에서 독립 계산한 기대값](../spec/internal/evidence/c11-4-auto-repeat-percentage-maximum-precomparison-2026-10-23/spec-derived-expectations.json), [화면](../spec/internal/evidence/c11-4-auto-repeat-percentage-maximum-precomparison-2026-10-23/chrome156-agent-browser.png)은 184px content axis와 8px gap의 28 scene을 고정한다. 20개 단일 축 scene에 더해 양축 동시 percentage/calc와 control scene을 둔다. Chrome은 oracle 관측이며 Spinon 구현 결과나 일반 브라우저 적합성 결과가 아니다.

- min 16px, max 25% = 46px이면 count는 3이다. 3×46 + 2×8 = 154px이며 네 번째는 208px으로 넘친다. min 기준으로 잘못 계산하면 8개가 가능해 결과가 달라진다.
- min 25% = 46px, max 20% = 36.8px 및 max 0%는 각각 max를 46px로 floor한다. 반복은 3개이며 고정 minmax(46px, 46px) control과 같다.
- max calc(20px + 10%)는 184px axis에서 ideal 38.4px이다. 4×38.4 + 3×8 = 177.6px이라 4개가 들어가고, 다섯 번째는 넘친다. Chrome layout은 1/64 CSS px 단위로 38.390625px을 관측하므로 허용 차이는 1/64px로 기록한다.
- 각 계산을 columns와 rows에 따로 적용한다. fill 장면은 두 item frame과 빈 repeat track을 확인하고, fit 장면은 반복 수 3과 placement 뒤 [46, 0, 0] collapse를 분리해 검사한다.

## 구현 경계

1. 범위는 RuntimeBlockFlexGridTrackSizingV1 opt-in profile의 auto-repeat component 하나다. 일반 CSS authoring profile이나 새 grid profile을 열지 않는다.
2. repeat track은 minmax의 positive fixed px 또는 positive definite percentage minimum을 사용하고, maximum은 fixed px control·nonnegative percentage 또는 단일 length-plus-percentage sum calc만 허용한다. calc는 finite positive length와 finite nonnegative percentage term으로 한정한다.
3. minmax(25%, 20%)와 minmax(25%, 0%)는 CSS Grid fixed-size grammar상 유효하며 maximum을 minimum으로 floor한다. negative/non-finite value, indefinite/cyclic percentage, 미지원 calc tree는 기존 fail-closed 오류를 유지한다.
4. 반복 계산은 definite positive content-box width 또는 height를 사용한다. 해당 axis의 min-size는 0, max-size는 auto여야 한다. border, padding, viewport, 반대 axis 크기를 percentage basis로 사용하지 않는다.
5. 고정 content size는 각 축에서 184px이고 row/column gap은 각각 8px이다. 일부 8 scene은 같은 Grid에서 columns와 rows를 동시에 반복해 축별 percentage basis와 calc identity를 확인한다. prefix/suffix neighbor, 여러 repeat, line-name/area, implicit placement, intrinsic maximum은 제외한다.
6. auto-fill/auto-fit을 모두 고정한다. auto-fit은 item placement 뒤 empty tracks 및 gutter collapse를 검사한다. 해당 결과에서 border stroke나 compositor-after pixel equality를 주장하지 않는다.
7. 최대값의 definite percentage 및 calc은 기존 cascade typed value, calc node/property identity binding, fallible layout validation을 재사용한다. CSS 텍스트를 다시 파싱하지 않고 Taffy 내부 구현을 수정하지 않는다. 실제 regression이 vendor sizing algorithm에서 재현되면 그때 source 및 별도 수정 범위를 재검토한다.
8. 28 scene 각각에서 style snapshot, pre/post-collapse used track, repeat count, root/item frame, complete track trace, draw/present를 보존한다. Chrome oracle/fixed control과 Android/iOS app fixture를 대조한다. 단일 축 case 외에 columns와 rows의 percentage/calc maximum을 동시에 둔 scene으로 axis-specific calc identity도 확인한다.
9. 기본/product FFI static archive에서 fixture entry, scene ID와 payload가 빠지고 opt-in fixture archive에는 포함되는지 확인한다. APK/IPA 전체 제외는 해당 bundle을 직접 검사하지 않으면 주장하지 않는다.
10. 내부 계약 0059 숫자 버전은 0.1.0으로 유지한다. C11.4 checkbox와 공식 완료 집계도 올리지 않는다.

## 구현·검증 관문

- 계획 자체는 서로 다른 실패 관점 20개로 검토하고, 이 계획 PR의 실제 변경은 별도로 30개 관점으로 검토한다. 계획 검토 파일을 구현 review에 재사용하지 않는다.
- 스타일 test는 computed direct percentage와 length-plus-percentage maximum의 typed AST, zero/max-below-min 값, 반복 mode와 기존 unlisted/indefinite 입력 거부를 확인한다.
- layout test는 maximum 우선 반복 수, max(max,min) flooring, axis-specific positive basis, gap accounting, auto-fit collapse, fixed control 동등성 및 calc ID의 node/property 연결을 확인한다.
- opt-in app fixture는 28 scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행한다. 사용자 요청에 따라 physical device는 사용하지 않는다. root/item frame, used track, trace, draw/present와 선택된 표면 표본을 기록하고 border overflow 픽셀이나 iOS compositor 이후를 과장하지 않는다.
- 관련 Rust tests, fixture catalog/route 검사, fixture-module-boundary, cargo fmt --all -- --check, workspace all-target Clippy, Android/iOS app build 및 default/product 대 fixture FFI artifact 비교를 기록한다.
- 구현/runtime은 계획 검토와 겹치지 않는 새 실패 관점 20개, 구현 PR diff는 별도 30개로 검토한다. 결함을 고치면 영향을 받는 검토 관점을 다시 판정한다.
- PR 본문에는 Android/iOS screenshot을 가능한 범위에서 첨부한다. Agent Browser로 PR 화면과 이미지 로딩을 확인한 후 세션과 해당 browser process를 닫는다.
- 구현 병합 뒤 내부 계약/상태/공개 roadmap과 사용자 지정 Tailnet 미리보기를 별도 문서 PR로 갱신한다. route, HTTP response, served SHA-256과 Agent Browser 렌더를 확인한다. 이 계획은 C11.4 checkbox와 공식 집계를 변경하지 않는다.

## 계획 자체 적대 검토

[서로 다른 실패 관점 20개](./c11-4-auto-repeat-percentage-maximum-plan-review-20.md). 이 검토는 implementation/runtime 및 PR diff 검토와 분리한다. plan PR 변경 검토는 구현 전 별도 evidence 파일에 기록한다.
