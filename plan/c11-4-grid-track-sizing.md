# C11.4 · CSS Grid track sizing 계획

**상위 원본:** [C11 상태와 범위](/spinon/STATUS.html) · [C11 Grid 계획](c11-css-grid.md) · [CSS 적합성 계약](/spinon/0001-conformance.html) · [Grid 내부 계약 0059](/spinon/internal/0059-c11-grid-layout.html)

**상태:** 계획·Chrome 155 사전 비교 완료. 단계 A의 direct-leaf text/image intrinsic source 입력과 단계 B의 typed track input·`RuntimeBlockFlexGridTrackSizingV1` 및 제한 stage trace가 후보에 연결됐다. trace는 Taffy 0.15.0의 열·행 pass, 일부 단계의 base/growth-limit 스냅샷, item min/max/minimum contribution query 및 automatic-minimum 분기를 보존한다. Spinon trace/input·projection buffer의 제한 초과와 fallible reservation 오류는 부분 `LayoutOutput` 없이 반환하지만, Taffy 내부 working `Vec` 일부는 infallible `reserve`/`with_capacity`를 사용해 모든 allocator failure가 `LayoutError`가 되는 것은 아니다. fixed Chrome 155 oracle 26개 case를 Rust frame matrix에서 대조했다(이미지 8, span 3, text 10, indefinite percentage 1, auto-repeat 4). Ahem span min/max-content 회귀, style cascade/projection auto-repeat 검사도 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 앱 scene 7/7의 frame을 확인했고, auto-fill·auto-fit·track overflow 세 scene의 screenshot 색 영역에서 100px grid 바깥 item overflow 약 22 CSS px를 측정했다. workspace test, all-target Clippy `-D warnings`, rustfmt가 통과했다. 현재 CSS auto-repeat은 한 축의 단일 repeat component와 고정 최소 길이 pattern 일부만 지원하는 제한 후보이며 line names, areas, 명시 track 혼합 및 다른 형태는 fail-closed다. 이 결과는 전체 contribution stage, text/image contribution 일반 행렬, cyclic percentage/calc edge, nested/anonymous item, 전체 auto-repeat placement/gutter matrix, 독립 normative stage oracle, WPT를 닫지 않으므로 C11.4와 C11 상위는 미완료다. [Chrome 26-case·trace 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-trace-2026-10-19/README.html) · [Simulator 7-scene 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/README.html) · [auto-repeat·overflow 화면 및 픽셀 범위](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/auto-repeat/README.html)

**단계 판정 메모:** 제한 auto-repeat subset은 현재 worktree에서 조기 검증 후보로 구현됐지만, 단계 B 전체 관문과 단계 C 완료 기준을 통과한 것으로 계산하지 않는다. B의 남은 contribution·normative stage 행렬이 닫히기 전까지 C11.4 상태는 미완료다.

2026-10-19 앱 후속으로 고정 direct-leaf text 한 장면 `min-content-text-normal`을 추가했다. `XX XXXXXXX`를 Ahem 10px로 측정해 앱의 Rust Grid 경로에서 min-content 70px, max-content 100px, item frame 70×20px와 Columns/Rows stage trace를 확인했다. Android API 37 AVD, iPhone 17 Pro와 iPad Pro 13-inch M5의 iOS 26.2 Simulator에서 해당 scene을 실행했다. Chrome 155 Agent Browser가 같은 앱 선언에서 70px column과 70×20px item을 확인했다. fixture renderer는 이 측정 profile의 text glyph를 그리지 않으므로 simulator screenshot은 box geometry만 보여준다. 이는 일반 text/image contribution matrix, glyph paint, WPT, 전체 C11.4를 완료하지 않는다. [실행 근거·화면·hash](../spec/internal/evidence/c11-4-grid-track-sizing-app-text-intrinsic-2026-10-19/README.md)

2026-10-19 후속은 이 경로를 고정 Ahem direct-leaf text 10개 scene으로 확장했다. `auto`, `min-content`, `max-content`, `fit-content()`, `fit-content(calc())`, inline edge, `min-width`, `max-width`, `break-all`, `nowrap` scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행했다. Chrome 155 Agent Browser에서 앱과 같은 computed `align-items:start`·`align-content:normal` profile을 관찰했다. 재실행 로그의 root/item `x/y/width/height`, node·paint 순서가 Chrome과 양 플랫폼에서 10/10 일치했고, 복원한 Columns/Rows JSON trace도 10/10 의미상 일치했다(원본 8/10 byte 동일, 나머지 2건은 `AutomaticMinimum` debug 공백만 차이). `min-content-text-break-all`의 grid root 높이 40 CSS px 아래 y=45에서 item 색을 포함한 Android PixelCopy와 iOS Metal 표본도 각각 3/3 exact로 확인했다. 입력, 앱 로그, 화면, overflow 표본, trace 비교와 SHA-256은 [C11.4 text intrinsic matrix evidence](../spec/internal/evidence/c11-4-text-intrinsic-matrix-2026-10-19/README.md)에 기록했다. 이 opt-in fixture는 다른 font/script, anonymous/nested item, image contribution, glyph paint, OS compositor 이후 표시, 실기기, WPT 또는 전체 C11.4 지원을 확인하지 않는다.

**고정 사전 자료:** [26 case Chrome 155 oracle와 hash 목록](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.html) · [Android·iOS Simulator smoke 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/README.html) · [inline span edge supplemental oracle](/spinon/internal/evidence/c11-4-grid-track-sizing-text-inline-edge-2026-10-19/README.html) · [계획 적대 검토 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/plan-review-20.html) · [기존 callback trace 구현 검토 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/implementation-review-20.html) · [contribution input 구현 검토 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/grid-item-intrinsic-contribution-implementation-review-20.html) · [contribution PR diff 검토 30개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/grid-item-intrinsic-contribution-pr-diff-review-30.html) · [raw intrinsic input 구현 검토 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/grid-item-intrinsic-input-review-20.html) · [기존 PR diff 적대 검토 30개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/pr-diff-review-30.html) · [Taffy 0.14.0 automatic minimum 소스 감사](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/taffy-0.14-automatic-minimum-audit.html) · [감사 반영 계획 재검토 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/taffy-audit-plan-review-20.html) · [fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html)

**현재 구현·PR 변경 검토:** [기본 Grid sizing 구현 실패 관점 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/current-implementation-review-20-2026-10-19.html) · [앱 stage trace 후속 구현 실패 관점 20개](/spinon/internal/evidence/c11-4-grid-track-sizing-app-stage-trace-2026-10-19/implementation-review-20.html) · [PR #345 변경 적대 검토 30개](/spinon/internal/evidence/c11-4-grid-track-sizing-app-stage-trace-2026-10-19/pr-diff-review-30.html) · [track sizing PR diff 적대 검토 30개](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/track-sizing-pr-diff-review-30.html)

## 목표와 닫을 범위

C11.4는 C11.3에서 보존한 track 목록을 기반으로 CSS Grid의 intrinsic track sizing을 실행한다. 대상은 두 축의 명시·양의 암시 track, `auto`, `min-content`, `max-content`, `minmax()`, `fit-content()`, item의 min/max-content 기여, 단일·다중 track span, definite·indefinite percentage, `auto-fill`·`auto-fit` 반복과 빈 반복 track 처리다. 기존 fixed/percentage/`fr` track과 C11 배치·named line·area 입력은 그대로 연결한다.

완료는 함수를 CSS 값으로 파싱하는 것으로 판정하지 않는다. 같은 revision의 computed style에서 track sizing 입력, item contribution, 계산 단계별 track base/growth limit, 최종 frame과 오류를 추적하고, 고정 Chrome oracle 및 독립된 표준 기대값에 맞춰야 한다. 명시적 미지원 문법은 node/property/value와 계산 단계를 식별해 거부한다. `auto`, 0, 빈 track 또는 부분 LayoutOutput으로 바꾸는 대체 처리는 허용하지 않는다.

CSS Grid Level 1 §7.2.2–§7.2.3.2와 §11.3–§11.8을 normative algorithm으로 사용한다. Taffy `0.15.0`은 계산 후보이며, Chrome·표준 차이를 자동으로 정답 취급하지 않는다. 이전 0.14.0 소스 감사는 의존성 변경 기록으로 보존하고 0.15.0 코드도 단계별로 검토한다. [CSS Grid Layout Module Level 1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)

## 현재 입력 경계

- `GridTrackSize`는 `LengthPx`, `Percentage`, `Fraction`, `Auto`, `MinContent`, `MaxContent`, `MinMax`, `FitContent`, `Calc`의 typed 값을 보존한다. 제한 `RuntimeBlockFlexGridTrackSizingV1` profile은 이를 Taffy 0.15.0 후보에 연결한다. CSS cascade/projection은 현재 한 축의 단일 auto-repeat component와 고정 최소 길이 pattern 일부를 추가로 보존한다. 그 외 auto-repeat 형태는 fail-closed이며 전체 §7.2.3.2 동작을 지원하지 않는다.
- `GridAutoRows`·`GridAutoColumns`는 `auto` 또는 단일 고정 px뿐이다. `CompactTaffyStyle`은 grid auto-track 목록을 비워 반환한다.
- C11.4 제한 profile의 opt-in 앱 fixture route에는 기존 네 장면과 auto-fill·auto-fit·track overflow 세 장면이 있다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 7 scene의 root/item frame을 확인했다. 추가 세 scene의 screenshot에서 root background와 item 색 영역을 재어 overflow가 grid 우측 바깥 약 22 CSS px까지 이어짐을 확인했으나 border stroke 자체의 색은 검사하지 않았다. 당시 앱 fixture에는 text contribution과 Taffy stage trace 출력이 연결되지 않았다. 2026-10-19 후속에서 고정 Ahem text scene 하나를 추가해 제한된 입력·trace를 앱 로그에서 확인했다. 전체 CSS combination을 검증한 것은 아니다. [실행 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/README.html) · [auto-repeat 후속](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/auto-repeat/README.html)
- C14.3 image natural-size contribution은 immutable replaced-size 입력과 Taffy measure callback으로 연결된다. 새 TrackSizing profile은 현재 제한 배치·track 입력과 조합해도 anonymous/nested image item aggregate는 다루지 않는다. Taffy 0.15.0의 automatic-minimum span 선택은 코드 검사와 Ahem regression으로 확인했지만 전체 CSS matrix는 남아 있다. 이전 Taffy 0.14.0 감사는 [소스 감사](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/taffy-0.14-automatic-minimum-audit.html)에 기록했다.
- C15 text callback은 definite width만 받아 Parley layout을 호출한다. `MinContent`·`MaxContent` available space는 `IndefiniteTextAvailableWidth` 오류가 된다. 별도 raw intrinsic source에서 동일 font registry·revision의 intrinsic width를 받지만, 모든 CSS contribution 사용 경로와 anonymous/nested text item은 아직 연결되지 않았다.
- Taffy 0.15.0은 `vendor/taffy`에서 trace용 제한 변경과 함께 관리하며, 현재 후보에서 모든 `spinon-layout` profile이 이 버전을 사용한다. 추적은 opt-in이며 stage snapshot과 item contribution query, minimum-contribution source를 기록한다. full algorithm audit log는 아니다. trace/output adapter가 소유한 한도 초과와 fallible 예약 오류는 전체 layout 실패로 반환하지만, Taffy 내부 working `Vec` 일부의 allocator failure는 이 오류 계약으로 변환되지 않는다.
- 기존 profile 회귀: Taffy 0.15.0 전환 후 C11.3.3 Chrome 154 고정 26 scene 행렬을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 26/26 통과했고, 61 frame 최대 차이 0 CSS px, 표면 표본 각각 2/2 exact였다. C11.2.3 dense/column profile은 같은 iOS Simulator에서 25 scene·107 node, 성공 19·기대 거부 6, 최대 geometry 오차 0 CSS px와 표면 99/99 exact를 확인했다. 이 선택 행렬은 모든 기존 profile 회귀를 닫지 않는다. [재현·상세 결과](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/taffy-015-existing-grid-regression.html)
- Grid에는 축별 10,000 track 상한이 있다. Spinon adapter가 소유한 확장·출력 buffer는 fallible `Vec` 예약과 계산 오류 전파 경계를 적용하지만 Taffy 내부 working buffer의 모든 할당 실패를 같은 오류 계약으로 바꾸지는 않는다. auto-repeat의 상한과 output atomicity를 보장할 범위를 독립 fixture로 더 확인한다.

## 단계와 의존 관계

모든 단계는 합쳐서 C11.4의 목표를 이룬다. 중간 PR이나 profile 통과를 C11.4 또는 C11 완료로 표시하지 않는다.

| 단계 | 구현 범위 | 진입/완료 조건 |
| --- | --- | --- |
| A · intrinsic contribution 입력 | Parley 기반 text min-content/max-content advance를 같은 shaping/font/revision 입력에서 계산한다. Ahem 기준의 `normal`, `word-break:break-all`, `white-space:nowrap`를 먼저 수치화하고, C15에서 계산할 수 없는 shaping·line-break mode는 거부한다. C14.3 image natural min/max contribution과 `min-width`/`max-width`, aspect-ratio 기반 축 전달을 재사용한다. Contribution source·axis·revision과 padding/border/box sizing의 외곽 반영을 기록한다. Track span, track minimum 함수, flex 예외가 필요한 automatic minimum은 B단계에서 계산한다. typed C13 overflow state가 없는 조합은 fail closed한다. | text/image item마다 min, max, preferred contribution 입력의 출처·축·revision이 남는다. definite-width C15 snapshot은 유지하고 intrinsic measure 과정에서 덮어쓰지 않는다. |
| B · intrinsic track algorithm | typed track 모델과 전용 `RuntimeBlockFlexGridTrackSizingV1` profile을 추가한다. fixed/percent/`fr`와 intrinsic min/max track, `auto`, `minmax(min,max)`, `fit-content(length-percentage)`를 명시·positive implicit track에 연결한다. §6.6 automatic minimum의 item별 track-span 조건과 specified/transferred/content size suggestion을 이 단계에서 계산한다. track 함수를 축 전체에서 찾지 말고 해당 item이 실제로 걸친 track 범위에서만 찾는다. §11.3–§11.8의 초기화, base size/growth limit, 단일 span 및 span별 기여 배분, max-content 단계, flex 확장, auto-track stretch를 각각 기록한다. 행과 열 양쪽에서 실행한다. | Chrome fixture 및 표준 control이 각 단계·최종 frame에서 맞고, automatic minimum이 그 item의 span 이외 track에 영향받지 않으며, unsupported item/style은 전 계산을 실패시킨다. C14/C15와 기존 C11.1–3 runtime 회귀를 유지한다. |
| C · auto-repeat | **부분 후보 구현:** 단계 B 전체가 닫히기 전에 같은 worktree에서 조기 검증 후보로 한 축의 단일 auto-repeat component와 현재 CSS adapter가 허용하는 fixed-minimum length pattern 일부를 구현했다. 이를 단계 C 진입 조건이나 완료로 판정하지 않고, B의 남은 관문을 계속 적용한다. definite inline size에서 `auto-fill` 반복 track을 유지하고 `auto-fit`의 빈 track을 collapse하는 제한 layout 경로가 있다. 그 외 repeat grammar, preferred/max/min basis 전부의 선택, repeated named lines, placement·span/gutter의 전체 조합, 10,000 상한 경계는 아직 독립 검증·완료하지 않았다. | §7.2.3.2 전체 기대값, 반복 직전/직후 placement, 빈 track·span 점유·gutter, overflow/10,000 상한을 독립 검사해야 한다. B 전체 관문과 C11.4 완료는 계속 미완료다. |
| D · 교차·회귀 종결 | hidden/empty item, `display:none`, span, fixed/min/max constraints, baseline-free text, image, mixed Block/Flex/Grid ancestor, revisions, 계산/예약 실패를 두 축으로 닫는다. 표준 및 WPT 후보 각 파일을 case별로 분류해 실행 결과를 연결한다. | 전체 matrix의 성공·기대 거부가 고정되어 있고, runtime frame과 surface 표본이 계산 출력과 일치한다. 모든 남은 차이·미지원 범위가 error/계약에 보인다. |

### 표준 알고리즘 불변식

1. 트랙의 base size, growth limit과 item contribution을 혼합된 최종 `gridTemplateColumns` 문자열 하나로만 검사하지 않는다. 각 sizing stage 뒤의 typed 숫자 배열과 사용한 contribution을 관찰한다.
2. `minmax()`의 최소·최대 함수 조합, flexible minimum/maximum 제한, fixed-size와 intrinsic 함수 우선순위는 Stylo computed form에서 검증한다. parser가 만든 원문 문자열만 보고 정상 값을 추측하지 않는다.
3. `fit-content(limit)`은 item max-content contribution, limit, track auto-minimum을 적용한다. 이 값은 Grid item의 `min-width:auto` automatic minimum과 별개다. C14.3의 24px image와 Ahem 70px min-content word는 limit보다 큰 track auto-minimum을 보존한다. `%`/`calc()` limit은 해당 axis basis가 definite일 때만 해석한다.
4. 여러 track에 걸친 item은 span 길이와 intrinsic 함수 종류별로 표준의 별도 단계에 포함한다. gap을 contribution에 중복 더하지 않는다. 주어진 전체 item 크기를 임의로 같은 크기의 track에 나누는 규칙으로 대체하지 않는다.
5. intrinsic measurement 중 cyclic percentage는 해당 algorithm stage가 정한 임시 취급과 최종 layout resolution을 분리한다. max-content `24px` grid의 50% track 최종값 12px와 24px image의 overflow는 고정 oracle이다. 최종 12px을 intrinsic contribution 입력으로 재사용하지 않는다.
6. auto-repeat은 반복 가능한 fixed-size 문법 및 단일 반복 위치를 preflight한다. definite preferred/max axis를 우선해 가능한 가장 큰 비-overflow 횟수를 선택하고, 이것이 없으면 definite minimum axis를 만족하는 최소 횟수를 선택하며, 모두 indefinite일 때 한 번 반복한다. track/gap arithmetic overflow와 0회 경계는 §7.2.3.2에 따라 판정하고 임의의 clamp를 넣지 않는다.
7. auto-fit의 empty 판정은 placement 이후 이루어진다. span이 걸친 repeated track은 occupied로 유지한다. collapse는 빈 반복 track뿐 아니라 규칙에 해당하는 인접 gutter에도 적용한다.
8. 새 profile의 어떤 항목에서 오류가 나도 이전 C11 profile의 의미, source order, revision, partial frame 부재가 유지된다. fallback이나 부분 결과를 게시하지 않는다.

## 사전 oracle 및 재현 조건

- 독립 기준: Agent Browser로 연 Google Chrome `155.0.8059.85`, macOS `26.5.1` ARM, DPR 1. browser metadata는 `navigator.userAgentData`로 받았다. viewport-dependent fixed test가 아니며 fixture는 320 CSS px page와 고정 item/track 값을 선언한다.
- 입력 fixture: `tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html`. 26 case 전부 PNG `24×16 CSS px`를 decode한 뒤 수집하고, text cases는 저장소 Ahem TTF를 load/check한 후 수집한다. 첫 image decode와 경합한 0×0 관찰은 폐기되어 golden data에 포함하지 않았다.
- 출력: `getComputedStyle(grid).gridTemplateColumns/Rows`, container/item `getBoundingClientRect()`, image natural dimensions, overflow, text font readiness. 기준 JSON과 화면·raw Agent Browser 출력은 [사전비교 README](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.html)에 보존한다.
- 판정: used track·container·item frame의 각 CSS px field는 Chrome double → JSON 값을 `0.05 CSS px` 이내로 비교한다. auto-repeat fractional track은 Chrome의 1/64 CSS px layout quantization 범위에 대해 exact-oracle/epsilon 기준을 적용한다. intentional item overflow는 계산된 grid border 바깥에서도 좌표와 색 sample로 검사하며 container 안쪽 표본만으로 통과 처리하지 않는다.
- WPT inventory는 read-only pinned checkout `c999c58338ee1d223df5ad62e48be1202ced0d37`의 다섯 후보와 SHA-256으로 고정했다. 각 파일을 Chromium에서 열거나 upstream pass로 기록하는 것과 Spinon WPT pass는 별개다. Spinon 공식 WPT runner가 연결되지 않은 동안 WPT 적합성은 미검증으로 둔다.

## 구현 전 판정 집합

26개 고정 case의 수치 oracle 및 실행 범위는 [README 표](/spinon/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.html)에 있다. 아래 case는 같은 CSS 입력을 기반으로 하되 text/image contribution, track 단계, repeat, overflow의 별도 경계를 고정한다.

| 묶음 | 필수 case ID |
| --- | --- |
| intrinsic image/track | `auto-image-definite`, `minmax-auto-image`, `minmax-min-max-image`, `fit-content-below-minimum`, `fit-content-above-intrinsic`, `fit-content-percent`, `fixed-small-track-overflow`, `min-content-row-image` |
| spanning contribution | `spanning-minmax-items`, `spanning-auto-min-flex-tracks`, `spanning-min-content-items` |
| auto-repeat | `auto-fill-definite`, `auto-fit-definite`, `auto-fit-all-empty`, `auto-fill-indefinite` |
| cyclic percentage | `indefinite-percent-track` |
| text contributions | `auto-text-max-content`, `min-content-text-normal`, `max-content-text-normal`, `fit-content-text-minimum`, `fit-content-calc-limit`, `min-content-text-edge`, `min-content-text-min-width`, `min-content-text-max-width`, `min-content-text-break-all`, `min-content-text-nowrap` |

추가 implementation controls에는 3+ span ordering, multiple spanning items의 경쟁, mixed intrinsic/flexible/fixed tracks, reversed item order, empty/hidden items, specified/transferred-size suggestion, `minmax(auto, max-content)`, zero/negative/non-finite input, non-resolvable `calc()` input, unsupported overflow state, duplicate/nested auto-repeat, auto-repeat with non-fixed sizing, one-track/10,000-track 경계, Spinon-owned allocation failure 및 같은 worker 재사용; Taffy 내부 `reserve`/`with_capacity` 실패는 현재 주입·복구 계약 밖이며 별도 제한 사항으로 기록을 둔다. `fit-content(calc(10% + 5px))`, inline padding/border, min/max width와 two-flex-track span은 각각 고정 oracle로 둔다. 나머지 calc case는 computed typed value와 basis를 검증한다. 이 control의 예상 결과는 CSS Grid normative rule에서 도출하거나 별도의 Chrome case를 구현 전 추가한다. 구현 후보 결과를 보고 기대값을 역으로 만들지 않는다.

C15 inline span edge는 Grid item box의 padding/border oracle와 분리해 [추가 Chrome 155 관찰](/spinon/internal/evidence/c11-4-grid-track-sizing-text-inline-edge-2026-10-19/README.html)에 고정했다. normal·nowrap·break-all·pre-line 각 line-boundary contribution을 검증하며 base 26개 fixture count에는 포함하지 않는다.

## WPT 후보와 역할

고정 upstream checkout은 `/Users/yoonhb/Documents/workspace/spinon/build/wpt-c124-source`, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`이다. 파일별 hash와 사용하는 assertion은 사전비교 README에 둔다.

| 후보 | 계획상 대응 범위 |
| --- | --- |
| `fit-content-track-sizing.html` | max-content growth, fit-content cap/floor, auto-track stretch 비대상, definite/percentage argument, auto row 및 implicit column |
| `grid-layout-auto-tracks.html` | auto track의 item contribution과 intrinsic sizing 상호작용 |
| `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html` | intrinsic inline-size 변경 시 반복 수 및 auto-fit 재배치 |
| `grid-auto-repeat-intrinsic-001.html` | definite/intrinsic container size에서 auto-fill/auto-fit 반복 재계산 |
| `grid-auto-fit-columns-001.html` | definite auto-fit, minmax/fixed repeat, line names, repeat 전·후의 fixed tracks |

이 목록은 고정 출처와 추후 실행 후보이며, Spinon이 이미 실행했거나 통과했다는 뜻이 아니다. 구현 시 각 upstream test의 `testharness` assertion, helper/reference 의존성, 적용 가능한 runtime profile을 먼저 확인한다. 일부 WPT를 실행한 결과를 전체 CSS Grid 적합성으로 확대하지 않는다.

## Rust·앱 검증과 증거 경계

1. `spinon-style` computed track projection과 `spinon-style-to-layout` normalization: computed value, property/node, track-list ordering, named line boundary, auto-repeat shape, invalid input을 검사한다.
2. `spinon-layout`: pure Rust 단계별 algorithm/control matrix와 기존 C11.1–3, C14.3 image, C15 text regression을 실행한다. item/frame output 하나만 비교하지 말고 각 track stage·contribution source·error·revision을 보존한다.
3. Taffy 0.15.0 후보 결과를 표준/Chrome 기대값과 대조하고, Taffy 0.14.0에서 유지하던 Spinon 회귀도 함께 실행한다. 차이는 adapter가 숨기지 않으며, API 변화로 생긴 실제 regression은 고친 뒤 근거를 남긴다. 필요하면 자체 sizing pass를 추가하고 특정 unsupported combination은 fail closed한다.
4. Runtime: Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 V8-authored HostDocument → 같은 document/style/environment revision → typed profile → Rust layout → WGPU scene/surface를 확인한다. 기존 앱 fixture 7 scene의 root/item frame은 두 플랫폼에서 각각 7/7 통과했다. 이번 후속에서는 각 scene의 layout revision, Columns/Rows stage snapshot과 contribution을 앱 응답에 기록해 양 플랫폼 7/7 JSON을 복원·검사했다. PR 검토 후 trace 로깅을 frame assertion 앞으로 옮긴 수정 빌드도 양 simulator 7-scene에서 통과했다. text/image intrinsic contribution 앱 실행과 기대 진단·surface 표본·submit 부재의 전체 matrix는 남아 있다. 이 제한 smoke는 실기기·hardware GPU·surface pixel 근거가 아니다. [7-scene stage trace 로그·캡처](/spinon/internal/evidence/c11-4-grid-track-sizing-app-stage-trace-2026-10-19/README.html) · [assertion 전 trace 보존 재검증](/spinon/internal/evidence/c11-4-grid-track-sizing-app-trace-failure-path-followup-2026-10-19/README.html) · [이전 frame·overflow 실행 근거](/spinon/internal/evidence/c11-4-grid-track-sizing-simulator-2026-10-19/README.html)
5. surface: non-clipped frame과 실제 paint는 분리한다. 12px track을 넘는 24px item, min-content line의 세로 overflow, parent/grid border 바깥이지만 viewport 안인 위치를 포함해 Android PixelCopy와 iOS Simulator Metal drawable에서 sample한다. 색/coordinate mapping을 manifest로 보존하며 sample 위치가 border 안으로만 이동하면 실패다.
6. artifact: fixture adapter/FFI를 opt-in runtime fixture feature에 둔다. 기본·제품 artifact에서 fixture FFI symbol, CSS, payload가 빠지는지와 opt-in positive control을 각각 확인한다.
7. 검토 관문: 계획은 구현 전에 별도 실패 관점 20개, 구현 뒤 새 실패 관점 20개, PR 변경은 다시 별도 적대 관점 30개로 검토한다. 수정된 관점은 재실행한다. PR에는 검토 횟수를 장식처럼 적지 말고 실제 발견·수정·남은 한계를 쓴다.

시뮬레이터는 사용자가 이전에 요청한 Android/iOS 교차 검증의 허용 실행 경로다. 실제 기기나 OS compositor 이후 결과를 시뮬레이터 자료에서 추론하지 않는다. 구현 실행 후 Agent Browser는 닫고 관련 preview server/browser process가 남지 않았는지 확인한다. Terminal Browser는 사용하지 않는다.

## API·상태 판정

- `spec/internal/0059-c11-grid-layout.md`의 C11 내부 계약을 track input, computed stages, diagnostics, profile, revision/output atomicity에 맞춰 `0.1.0`으로 확장한다. 출시 전 숫자 버전을 올리지 않는다. C14/C15 계약의 소유 동작을 바꾸면 해당 문서도 함께 고친다.
- `spec/STATUS.md`에서 C11.4는 B/C/D 모든 필수 관문이 끝난 뒤에만 해당 subtask 제한 profile의 결과와 한계를 기록한다. C11 상위, S02, X09, X16 및 공개 CSS 지원을 연쇄 완료 처리하지 않는다.
- 새 제한 profile의 exact 지원 값, unsupported syntax별 error, WPT 미실행 여부, Chrome/Android/iOS 결과, overflow 경계, product API 제외를 PR·contract·status에서 같은 용어로 유지한다.

## 실행 순서

1. 계획 자체의 서로 다른 실패 관점 20개를 통과시킨다. 계획 검토에서 발견한 누락은 이 문서와 oracle을 먼저 수정한다.
2. 계획 PR을 `Rebase and merge`로 병합하고 최신 `origin/main`에서 구현 브랜치를 만든다.
3. B단계 코드를 바꾸기 전에 A 입력 contract와 부족한 Chrome oracle을 확장한다. Automatic minimum은 track model과 item span을 알아야 하므로 A에서 독립 계산하지 않고 B의 placed-item 단계에서 해결한다. C15 measure API 구현 전 별도 Chrome text oracle 추가가 필요하면 새 fixture/hash를 보존한다.
4. Rust 계산기의 track sizing stages와 error atomicity를 구현하고, 독립 20 관점으로 새 코드를 hostile review한다.
5. C 단계 auto-repeat을 구현한다. A/B review 표를 재사용하지 않고 auto-repeat 자체의 새로운 실패 관점도 따로 검토한다.
6. Android API 37 AVD, iOS 26.2 Simulator, artifact boundary를 각각 실행한다. 실패한 SDK/app 경로는 source log의 첫 실제 오류부터 수정하고 재실행한다.
7. 내부 계약·C11 plan·conformance·status·fixture README를 일치시키고, 새 PR diff를 별도 20 관점으로 검토한다. 요구 CI와 재검증이 통과한 뒤 Rebase and merge한다.

계획 PR 병합은 코드 구현이나 C11.4 완료를 뜻하지 않는다. Auto-repeat, text intrinsic measurement 또는 simulator 범위가 남아 있으면 goal을 계속 진행한다.
