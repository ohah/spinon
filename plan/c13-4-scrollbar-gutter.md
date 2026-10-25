# C13.4 첫 조각 · classic scrollbar gutter의 안정 배치

**상태:** 구현 전 계획 · Android/iOS Simulator의 실제 scrollbar mode와 사용자 표시 차이를 확인하기 전에는 구현하지 않는다.
**공식 상위 항목:** [C13 overflow·scroll·clipping](../spec/STATUS.md#css-구현-체크리스트)
**선행:** C13.1 typed overflow/clip, C13.2 immutable scroll owner·scrollport·offset snapshot, C13.3 내부 scroll input 경계
**계약 초안:** [내부 계약 0083](../spec/internal/0083-c13-4-scrollbar-gutter.md)
**Chrome 관찰:** [2026-10-26 비교와 실행 범위](../spec/internal/evidence/c13-4-scrollbar-precomparison-2026-10-26/README.md)
**계획 실패 관점 검토:** [20개 검토](../spec/internal/evidence/c13-4-scrollbar-plan-review-20-2026-10-26.md)

## 목표와 중단 조건

C13.4 전체의 scrollbar paint, thumb 이동, `scrollbar-width`, `scrollbar-color`, root viewport 전파를 한 번에 구현하지 않는다. 첫 후보는 기존 C13.2 Block/LTR element scroll owner에서 명시된 두 축 `overflow: scroll`과 `scrollbar-gutter: auto|stable|stable both-edges`의 inline-axis layout reservation이다. 이 조각은 실제 target platform에서 classic scrollbar gutter와 그에 따른 content/frame 변화가 관찰되는 경우에만 implementation 대상으로 승격할 수 있다.

CSS Overflow Level 3의 고정 2025-10-07 Working Draft §4.2에 따르면 classic scrollbar는 gutter에서 공간을 쓰지만 overlay scrollbar는 gutter를 만들지 않는다. `stable`은 classic scrollbar의 존재 여부와 관계없이 gutter를 예약하고, `both-edges`는 반대편에도 같은 gutter를 만든다. 이 속성은 scrollbar 자체의 가시성을 바꾸지 않는다. 따라서 Android 또는 iOS Simulator가 overlay mode라면 `stable`/`both-edges`가 layout을 바꾸지 않는 것이 기대 결과다. 이 경우 custom scrollbar, 강제 classic mode, 임의의 gutter 폭을 추가하지 않고 해당 platform slice를 중단한다.

계획된 첫 profile의 성공 관문은 모두 충족되어야 한다.

1. Android API 37 ARM64 AVD와 지정 iOS Simulator에서 각 OS/browser의 scrollbar mode를 실행 정보와 실제 화면으로 식별한다. 현재 C13.2 app 화면에서 보이지 않는 bar를 CSS 지원으로 간주하지 않는다.
2. Agent Browser로 고정 Chrome oracle을 수집한다. 저장소 계획의 기준은 Chrome for Testing 155.0.8059.40이다. 현재 system Chrome에는 155.0.8059.85와 156.0.8078.48이 있으나 정확한 pinned patch는 확인되지 않았다. 기존 Chrome 156 관찰은 version-scoped 보조 자료로만 남기고 pinned-155 결과로 승격하지 않는다. UA가 정의하는 gutter 폭은 상수가 아니므로 플랫폼별 관찰이나 명세가 정하지 않은 픽셀 수를 기대값으로 만들지 않는다.
3. 플랫폼 mode가 classic이고 stable gutter가 자식의 content/frame 위치에 실제 차이를 만들면, C13.2의 owner·scroll range·clip·scroll revision을 보존하는 단 하나의 Block/LTR element profile을 구현한다. gutter는 해당 scrollport의 client geometry에만 반영하고 scrollbar paint/drag affordance는 범위에서 제외한다.
4. 플랫폼 mode가 overlay이면 `stable`과 `stable both-edges`의 gutter 폭이 0이고 content/frame·scroll range·hit-test가 변하지 않는지 positive/negative geometry control로 확인한다. 사용자에게 보이는 bar가 없는 상태에서 desktop classic 관찰값 15px를 모바일에 강제하지 않는다.
5. `scrollbar-width`, `scrollbar-color`, root viewport/body propagation, RTL/vertical writing mode, nested classic/overlay mixing, `overflow:auto`의 cyclic overflow selection, scrollbar paint/interaction은 후속 단계로 남긴다.

## 현재 코드 경계

- Cascade snapshot은 C13.1/2 overflow 축을 보존하지만 `scrollbar-gutter`와 presentation mode를 아직 전달하지 않는다. 계산 값은 `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs`에서 수집되고 `crates/spinon-style-to-layout/src/projection/style_values.rs`가 `LayoutStyle`로 projection하는 구조다.
- `crates/spinon-layout/src/style.rs`의 `LayoutStyle`에는 gutter token이 없다. `crates/spinon-layout/src/taffy_style.rs`는 C13.4 전까지 Taffy scrollbar width를 0으로 둔다. Taffy `Overflow::Scroll`만 `scrollbar_width`를 사용하며 Spinon `auto`는 currently `Overflow::Hidden`으로 보낸다. 이 매핑을 넓히면 automatic minimum size와 Block/Flex/Grid sizing 영향이 있어 별도 회귀 gate가 필요하다.
- `crates/spinon-style-to-render/src/stacking/scroll/geometry.rs`가 border edge와 scrollable extent로 client/scrollport geometry를 만들고 `crates/spinon-render/src/runtime/scroll.rs`가 그 geometry에서 offsets/range를 확정한다. Gutter를 도입하면 layout client box와 runtime scrollport를 일치시킨 다음 paint clip/hit-test snapshot에 같은 revision으로 전달해야 한다.
- 위 Rust 경로에 값이 없는 동안 CSS 선언이 cascade에서 읽히거나 Taffy field가 존재한다는 사실만으로 runtime 동작을 주장하지 않는다.

현재 C13.2 불변조건의 no-device baseline으로 `mise exec -- cargo test --locked -p spinon-style-to-render c13_2_scroll_geometry:: -- --nocapture`을 실행해 12/12 통과했다. 이 실행은 `d94f6458`의 Rust 소스 기준이며 새로운 gutter 동작을 검증한 결과가 아니다. C13.4 구현이 진행되면 이 회귀 행렬을 다시 실행하고 추가 positive/negative controls를 별도로 둔다.

## 고정 Chrome 156 사전 관찰

현재 Agent Browser가 실행한 Chrome은 HeadlessChrome 156.0.0.0, macOS arm64, DPR 1이다. `--hide-scrollbars false`를 명시해 실제 scrollbar UI를 표시했다. 220×130 CSS px border-box와 2px border, 380×280 content로 수평·수직 overflow를 동시에 만든 고정 fixture에서 `auto` scrollbar는 15×15 CSS px, `thin`은 11×11 CSS px의 client-area 감소를 관찰했다. `none`은 해당 실행에서 scrollbar가 표시되지 않고 client area가 216×126 CSS px였다. `stable both-edges`는 inline 방향으로 두 번째 15px gutter를 더해 client width가 186 CSS px였다. 전체 observation·input·screenshot hash는 evidence README에 연결한다.

이 결과는 pinned Chrome 155와 Android/iOS 결과가 아니다. Chromium build가 다르고 headless launch option에서 native scrollbars 표시를 강제했으므로 target mobile platform에 그대로 적용하지 않는다. `scrollbar-width`의 geometry는 이 실행에서 참고값으로만 보존하며 첫 구현 profile에 포함하지 않는다.

## Geometry 계약과 원자성

- `ScrollbarPresentationMode`는 환경에서 명시적으로 제공되는 `Classic { used_width_css_px }` 또는 `Overlay`이어야 한다. mode/used width가 없는 입력을 Classic으로 추측하지 않는다.
- Overlay mode의 gutter insets는 항상 0이다. Classic mode에서는 기존 C13.2 scrollport를 기준으로 inline edge에 UA used scrollbar width를 적용한다. LTR만 첫 profile에 허용하며 UA/OS가 시작·끝 edge를 정하지 않으면 fail-closed한다.
- `scrollbar-gutter: auto | stable | stable both-edges`의 computed token은 cascade/style revision과 함께 전달한다. `stable both-edges`는 classic gutter가 생길 때만 반대 edge에 같은 폭을 적용한다.
- gutter는 element border-box나 outer frame을 바꾸지 않는다. 변경된 client/scrollport box로 descendant layout, scrollable extent/range, clip, paint와 hit-test를 하나의 새 immutable snapshot에서 함께 계산한다.
- 이전 offset은 새 range 안에서 유지하고 범위를 벗어나면 C13.2 규칙대로 clamp한다. style/gutter 변경은 style/layout revision을 올리며 range 또는 owner mapping이 달라지면 scroll revision도 올린다. 이전 paint/hit snapshot은 stale로 거부한다.
- 측정된 native scrollbar UI를 별도로 그리지 않는다. stable gutter의 padding-background paint만 고정 Chrome/Simulator surface oracle로 확인할 수 있을 때 범위에 포함한다.

## Fixture와 검증 행렬

구현에 진입한다면 기존 C13.2 opt-in host profile과 독립 scene ID를 사용한다. fixture test feature, C header, JNI/ObjC++ adapter와 payload는 opt-in 경계 밖에 나타나지 않아야 한다.

| Control | 관찰할 값 | 예상 관계 |
| --- | --- | --- |
| classic + `overflow:scroll` + `auto` | border-box, scrollport/client width, child frame, scroll range | classic scrollbar used width만큼 inline content가 좁아진다. |
| classic + `overflow:scroll` + `stable` | 위 값과 initial/grown content | overflow 유무에 관계없이 같은 gutter가 예약된다. |
| classic + `stable both-edges` | 좌·우 gutter와 child frame | inline 양쪽에 같은 폭을 예약하며 child frame의 시작점도 이동한다. |
| overlay + `stable` / `stable both-edges` | 위 값, clip/hit identity, scroll range | gutter 0, 기존 geometry·scroll state와 exact equality. |
| `overflow:clip` 및 visible box | owner set, gutter, hit-test | scroll owner가 아니며 gutter가 없다. |
| nested owner / style revision | ancestor clip, offset, range, stale key | child만 변경되며 sibling·ancestor와 stale query는 C13.2 계약을 지킨다. |
| geometry/mode 없는 negative controls | error code, published snapshot | partial gutter나 이전 snapshot과 새 scroll state가 섞이지 않고 fail-closed한다. |

Chrome geometry의 허용치는 plan fixture에서 CSS px 단위로 고정하고 frame/owner/clip identity는 exact 비교한다. 앱 surface screenshot·pixel 표본은 Android API 37 ARM64 AVD PixelCopy, iOS 지정 Simulator Metal drawable로 따로 기록한다. Simulator UI가 overlay scrollbar를 감춰 surface 차이를 보여주지 못하면 “미관찰/미검증”으로 남기고 구현을 중단한다. 실제 기기, hardware GPU, iOS compositor 이후 출력, 일반 author CSS/WPT는 포함하지 않는다.

## 의존성과 범위 밖

C12.5 sticky는 C13.2 scrollport/offset/clip 자료를 소비하는 후속 기능이다. 이 계획은 sticky layout을 변경하지 않는다. C13.5 scroll snap과 C13.6 logical axis도 제외한다. C13.2의 ink-only `box-shadow` overflow는 계속 fail-closed한다. C13.4 scrollbar profile 결과만으로 C13 전체를 완료 처리하지 않는다.
