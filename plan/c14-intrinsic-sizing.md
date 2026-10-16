# C14 · 내재 크기와 대체 요소 기초 계획

## 목표

CSS intrinsic sizing과 replaced element sizing을 작은 비교 가능한 단계로 구현한다. 상위 C14는 S02 입력 revision, U05 이미지 자원·측정, C15 실제 텍스트 측정, X16 적합성 증거가 모두 준비되기 전까지 미완료로 유지한다. 이 계획의 첫 단계는 그중 layout 계산기에 이미지 자연 크기를 전달하는 revision-stamped 측정 입력만 다룬다.

출시 전 내부 인터페이스와 crate 숫자 버전은 `0.1.0`을 유지한다. 이 계획과 첫 단계는 공개 CSS 지원 선언이 아니다.

## 단계 분해

| 단계 | 내용 | 선행 조건 | C14 판정 |
| --- | --- | --- | --- |
| C14.1 | HTML `img`의 유한한 자연 너비·높이를 layout leaf의 콘텐츠 측정값으로 전달하고 입력 revision에 묶는다. 두 CSS 축이 모두 `auto`인 경우만 성공시킨다. | S02 revision stamp 확장, C07 box sizing·min/max 기반 | 기초 입력 경로만 완료 |
| C14.2 | [전용 계획](c14-2-image-ratio-sizing.md)에 따라 intrinsic ratio, 한 축 지정, CSS 2.2 min/max 조합, replaced default object size를 처리한다. | C14.1, U05 decode metadata | 구현·검증 후보 완료, stacked PR 검토 전 |
| C14.3 | [`img` intrinsic contribution과 percentage resolution](c14-3-image-intrinsic-contributions.md)에 따라 definite Block/Flex/fixed-track Grid item context에서 `min-content`, `max-content`, `fit-content`와 제한 cyclic percentage 경계를 구현한다. | C14.1, C14.2, C09/C10/C11 사용 문맥 inventory | Chrome 155 14-case oracle, Android API 37 AVD·iOS 26.2 Simulator 14/14, workspace 873 passed·4 ignored, Clippy와 artifact 경계 검증 완료 · 구현 PR #216 merge 반영 중 |
| C14.4 | 텍스트·버튼의 intrinsic contribution을 shaping/inline 결과와 결합한다. 합성 폭 근사값은 사용하지 않는다. | C15, U04, S07 | 미완료 |
| C14.5 | 오류·네트워크·캐시·자원 교체·재계산과 X16 적합성 subset을 닫는다. | U05, S02 runtime owner, X16 | 미완료 |

## C14.1 범위

- 입력은 source `LayoutSourceRevision`, document-scoped measurement revision, `NodeId → (natural width, natural height)` map을 함께 가진 immutable measurement snapshot이다. generic `LayoutInput`은 DOM tag나 resource state를 보유하지 않으므로 측정 snapshot은 별도 계산 인자로 Taffy에 전달한다. 엔진은 source·node·leaf·axis·값을 검증하고 output revision에 측정 revision을 기록한다. `spinon-style-to-layout`은 HostDocument의 HTML `img`를 식별해 auto/auto 누락과 partial-auto 범위를 사전 검사한다. 두 길이는 density 보정을 끝낸 CSS px이며 양의 유한값이어야 한다. 향후 U05가 decoder 측정 결과를 이 경계에 연결한다. map에는 현재 CSS size와 무관하게 decoder가 가진 모든 HTML image의 자연 크기가 들어갈 수 있다.
- 대상은 연결된 HTML namespace의 `img` leaf뿐이다. 현재 단계는 CSS `width:auto;height:auto`만 측정한다. 한 축이 지정된 경우, `aspect-ratio`, natural ratio 전용 이미지, intrinsic keyword, CSS math, `<input>`, `<video>`, SVG `<image>`는 명시적으로 지원하지 않는다.
- 보이는 `img`에서 양 축이 auto인데 측정값이 없으면 0 크기로 진행하지 않고 node를 식별하는 오류를 반환한다. 양 축이 모두 definite이면 측정값은 layout 결과에 영향을 주지 않는다. 한 축만 auto이면 measurement 유무와 관계없이 ratio 계산이 필요한 현재 미지원 경로로 거부한다. `display:none` 또는 숨겨진 조상 아래의 이미지는 크기 측정을 요구하지 않는다.
- 측정값은 콘텐츠 크기다. `box-sizing`, padding, border와 min/max clamp는 기존 Taffy 계산 한 번에 적용한다. 측정 단계에서 frame 크기로 변환하거나 간격을 두 번 더하지 않는다.
- 한 측정 revision은 document generation 안에서 단조 증가하며 모든 surface가 공유한다. `checked_next()`가 소진되면 실패하고 wrap하지 않는다. 문서·스타일·환경 revision과 별도로 `LayoutInputRevision`과 `CurrentLayoutInputs`에 보존해, 자원 크기 갱신 뒤 이전 layout 결과의 admission을 막는다. 측정 source가 다른 generation에서 복사된 snapshot도 거부한다.
- 기존 스타일 projection API는 측정 snapshot 없이 실행한다. 보이는 `img` 두 축이 auto인 입력은 새 overload가 측정값을 제공해야 성공한다. 이전 호출이 조용히 0 크기로 성공하도록 두지 않는다.

## 제외 범위

텍스트 노드, 글꼴 측정과 shaping, 버튼·폼 컨트롤 UA 크기, CSS `min-content`·`max-content`·`fit-content`, intrinsic aspect ratio, 한 축이 지정된 replaced sizing, 이미지 decode·캐시·로딩·실패 UI, `object-fit`·`object-position`, SVG image, video/canvas/input sizing, async relayout scheduling, 공개 API/CSS 지원 및 전체 WPT 적합성은 포함하지 않는다. 이 단계 통과로 C14, C15, U05 또는 X16 완료를 표시하지 않는다.

## 비교 기준과 판정

- HTML fixture: [replaced-auto-size.html](../tests/fixtures/css/c14/replaced-auto-size.html), inventory: [replaced-auto-size-inventory.json](../tests/fixtures/css/c14/replaced-auto-size-inventory.json).
- 구현 전 관찰은 agent-browser가 연 로컬 fixture의 `HTMLImageElement.naturalWidth`, `naturalHeight`, `getBoundingClientRect()` 값이다. 실행 브라우저는 Google Chrome `155.0.8059.40`이다. 이는 저장소의 기존 Chrome 154 고정 fixture와 다른 capture이며, 이 제한을 [사전 비교 근거](../spec/internal/evidence/c14-1-replaced-intrinsic-inputs-precomparison-2026-10-14.md)에 기록한다.
- oracle의 핵심 case는 natural size `24×16 CSS px`, CSS width·height `auto`, 관찰 frame `24×16 CSS px`다. 레이아웃 계산에 들어가는 측정 map은 같은 CSS px 크기를 담고, 구현 결과의 frame 네 필드는 각각 절대 오차 `0.5 CSS px` 이하이어야 한다.
- 비교 전에 예상값은 `naturalWidth=24`, `naturalHeight=16`, 양쪽 auto의 used frame `24×16`으로 고정한다. 화면 DPR을 CSS px로 잘못 전달하거나 padding을 두 번 더하면 실패다. width-only `48×32`, height-only `36×24` observation은 후속 ratio 범위의 경계 자료이며 C14.1 성공 분모에 넣지 않는다.
- Android/iOS 시뮬레이터는 runtime adapter를 실제 호출하는 fixture가 준비된 경우에만 통합 smoke에 사용한다. 앱이 이 측정 snapshot을 아직 만들 수 없다면 시뮬레이터 표시는 C14.1의 증거로 과장하지 않는다. 실기기는 요구하지 않는다. GPU queue 또는 실제 표시 시점에서의 stale 폐기는 현재 S02 계약 밖이며 이 단계의 admission gate 통과로 주장하지 않는다.

## 구현 순서와 게이트

1. 이 계획 자체를 별도 20개 실패 관점으로 검토하고 결함을 반영한다.
2. core에 document-scoped `IntrinsicSizeRevision`을 추가한다. overflow 시 wrap하거나 재사용하지 않는다.
3. `LayoutInputRevision`, 현재 입력 gate, layout input에 측정 revision을 보존한다. 오래된 measurement revision을 가진 output은 static snapshot admission에서 거부한다.
4. `TaffyLayoutEngine::compute_with_intrinsic_size_snapshot`에 generic leaf-measurement map을 전달한다. 엔진은 source/generation 불일치, unknown node, 비-leaf, partial-auto 축, 0·음수·NaN·무한 측정값을 명시적 오류로 처리한다. style-to-layout HostDocument preflight는 map key가 HTML image인지, 보이는 auto/auto image의 누락, hidden ancestor와 fixed/fixed image를 검사한다. 두 축이 definite인 image의 natural size는 무시한다. 중간 map 생성에서 같은 NodeId가 중복되면 덮어쓰지 않고 오류로 처리한다.
5. pinned Taffy `0.14.0` leaf measure callback에 자연 크기를 전달하고 `item_is_replaced` 표지를 켜 일반 Block의 auto 폭 stretch가 측정 너비를 덮지 않게 한다. box edge·min/max가 한 번만 적용되는지와 Flex auto basis에서 같은 측정값이 쓰이는지 시험한다.
6. `spinon-style-to-layout`에 revision-stamped intrinsic input overload를 추가하고 기존 runtime overload는 빈 측정 snapshot으로 위임한다.
7. Chrome fixture reference를 검증하고 Rust HostDocument→Stylo→Taffy 결과를 node별로 비교한다. 회귀·부정 입력을 포함한다.
8. 구현 뒤 계획 표와 다른 20개 구현 실패 관점으로 코드를 검토하고 결함 수정 후 영향받는 관점을 다시 실행한다.
9. 내부 인터페이스 0009, S02 현재 입력 gate 문서, C14 상태와 증거 인덱스를 갱신한다. 공개 지원 완료 표시는 하지 않는다.
10. PR에서 20개 서로 다른 적대 관점으로 검토하고 필요한 CI 및 mergeability를 다시 확인한 뒤 Rebase and merge를 사용한다.

## C14.1 완료 판정

- source generation을 포함한 natural size snapshot과 독립적인 measurement revision이 layout input/output/current-input gate를 함께 통과한다.
- 양 축 auto인 HTML `img`는 제공된 natural size로 CSS px frame을 얻고, 누락되거나 지원 밖인 경계는 명시적 오류를 낸다.
- CSS px/DPR, padding, box sizing, min/max clamp, flex auto basis, hidden subtree, generation mismatch, resource size revision 갱신과 stale snapshot 거부를 분리해서 확인한다.
- `cargo fmt`, 영향 crate의 focused tests, workspace lint/build에 대해 실행한 결과와 실패 원인을 기록한다. 실행하지 않은 시뮬레이터·기기·WPT는 미검증으로 남긴다.
- 계획 검토 20개와 새 구현 검토 20개에서 미해결 P1/P2 결함이 없다. 구현 근거와 문서가 같은 change에 들어간다.
- C14, C15, U05, X16은 모두 미완료 상태로 유지한다.
