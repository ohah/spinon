# 0054 · C12.1 정적·상대 위치와 물리 inset

**문서 ID:** `0054` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 구현·검증 완료 · PR 검토 전 · **공개 API:** 아님

`0054`는 문서 ID다. 출시 전 숫자 버전은 구현·검증·문서 갱신으로 올리지 않는다.

## 범위

현재 runtime Flex 및 Block formatting 경로에서 Stylo의 computed `position`과 물리 `top`·`right`·`bottom`·`left` 값을 같은 cascade revision으로 보존하고 레이아웃 입력으로 전달한다. `inset` shorthand, `auto`, 지원 길이·percentage·`calc()`·`var()`, stylesheet cascade와 inline override를 포함한다.

이 단계에서 layout adapter가 받는 `position` 값은 `static`과 `relative`다. `absolute`, `fixed`, `sticky`는 레이아웃 성공 경로에서 명시적으로 거부한다. 물리 inset만 지원한다. 현재 기준은 `horizontal-tb`·LTR이며 RTL 상대 offset, 논리 inset, 다른 writing mode는 이 계약에 포함하지 않는다.

## 계산 결과와 불변 조건

- `static` box는 inset을 사용 위치에 적용하지 않는다. inset이 CSS에서 계산된 값으로 snapshot에 남더라도 flow frame과 visual frame은 같다. `static`은 자손의 positioned containing block owner가 아니다.
- `relative` box의 inset은 normal-flow 배치 뒤 visual 위치에 한 번 적용한다. 해당 box의 크기, normal-flow 위치, 형제 위치 및 부모의 normal-flow 크기는 바뀌지 않는다.
- 상대 위치 box의 이동은 그 box의 descendants가 그리는 visual 좌표에 전파된다. normal-flow 좌표에는 전파되지 않는다. renderer가 소비하는 `frames`는 최종 visual frame 하나이며 별도 inset 보정을 추가하지 않는다.
- 내부 layout 결과는 `flow_frames`와 `frames`를 모두 제공한다. `flow_frames`에는 어떠한 relative inset도 반영하지 않고, `frames`에는 자신과 조상의 relative offset을 각각 한 번 적용한다. `display:none` 및 그 자손은 두 결과에서 모두 `(0, 0, 0, 0)`이다.
- positioned containing-block owner는 box가 있는 노드의 가장 가까운 `position:relative` ancestor다. 없으면 viewport, 자신이 `display:none`이거나 숨겨진 ancestor 아래 있으면 owner 없음으로 표시한다. 이 owner metadata는 후속 C12.2의 absolute placement 입력을 위한 것이며 absolute box 계산을 의미하지 않는다.
- horizontal axis에서 LTR `left`와 `right`가 모두 non-auto이면 `left`가 사용된다. vertical axis에서 `top`과 `bottom`이 모두 non-auto이면 `top`이 사용된다. 반대쪽은 computed style에 유지하되 visual offset 계산에는 쓰지 않는다.
- inset percentage의 수평 basis는 containing block content width, 수직 basis는 containing block content height다. 아직 definite하지 않은 basis를 임의로 0으로 바꾸지 않고 해당 inset을 `node/property` 진단과 함께 거부한다.
- 음수 inset과 음수 `calc()` 결과를 보존한다. NaN·무한대·잘못된 calc handle·stale revision은 부분 frame을 발행하지 않는 전체 계산 오류다.

## 데이터 경계

1. Stylo가 winning longhand의 computed `position` 및 네 inset side를 계산한다. shorthand expansion, cascade layer, specificity, `!important`, custom property와 registered property 승자를 layout adapter가 다시 파싱하지 않는다.
2. `ComputedElementStyle`은 typed position, typed `auto | length | percentage` inset, 보존된 CSS math AST를 같은 style revision에 담는다. Stylo가 표현하지 못한 유효 계산값은 문자열로 추측하지 않고 unsupported diagnostic으로 닫는다.
3. `spinon-style-to-layout`은 position subset과 네 inset을 `LayoutPositioning`으로 투영한다. inset calc는 side별 basis를 유지한다. position이 `static`이면 computed inset이 무엇이든 used inset은 `auto`다.
4. `LayoutInput`은 일반 `LayoutStyle`과 별도의 positioning map을 소유한다. 기존 Flex·Block sizing field를 position 값으로 재해석하지 않는다. Taffy 0.14의 `Position::Relative` 및 inset을 활용하되 CSS `static`은 inset `auto`로 표현한다.
5. 상대 inset이 하나라도 활성화되면 Taffy 계산을 visual pass와 inset-neutral flow pass로 분리한다. neutral pass는 `position:relative` 자격과 기존 formatting context를 유지하고 inset만 `auto`로 바꾼다. 두 pass의 크기·형제 흐름이 같지 않거나 한 pass만 성공하면 전체 요청을 실패시킨다.
6. layout root 자체가 `relative`이면 Taffy root에 부모가 없어 inset이 적용되지 않으므로, 그 경우에만 viewport 크기의 합성 containing-block box를 둔다. 이 box는 HostDocument·NodeId·출력 frame에 노출되지 않으며 root inset percentage와 자손 visual offset에만 참여한다. Block formatting은 기존 viewport wrapper를 사용한다.
7. runtime layout state와 scene admission은 기존 document·render tree·style·environment revision tuple을 그대로 확인한다. position/inset 변경을 별도 cache key 없이 재사용하지 않는다.

## Android fixture 시작 순서

C12.1 Android fixture는 Surface의 폭·높이가 모두 양수로 확정된 `surfaceChanged` 뒤에 runtime queue에서 한 번 시작한다. surface generation이 정해지기 전에 initial fixture를 평가하면 첫 scene이 superseded될 수 있다. initial script는 DOM root를 추가하므로 이를 무조건 재평가해 복구하지 않는다. host 생성 후 resize·환경 변화는 기존 environment revision 갱신 경로를 따른다. 이 순서는 C12.1 전용 fixture harness 동작이며 CSS API 의미를 추가하지 않는다.

## 오류와 미지원 경계

`absolute`·`fixed`·`sticky`, 지원하지 않는 writing mode/direction, 미확정 percentage basis, unsupported computed inset, 비유한 결과, positioning map의 누락·중복·문서 외 NodeId는 명시 오류다. 일반 flow frame이나 `auto`로 조용히 바꾸지 않는다.

`display:none`은 box, frame, paint 및 positioned owner를 만들지 않는다. 다만 이 단계는 `display:contents`, table, inline fragmentation, float, scrollable overflow, hit-test offset, clipping, focus/accessibility geometry, border paint, stacking context와 `z-index`를 구현하지 않는다. 그 항목들은 각각 기존 C09/C13/C17/S05/C22/C12 계획의 경계를 따른다.

## 비교 기준 및 완료 판정

기준은 고정 Chromium 154 fixture `tests/fixtures/css/references/c12-1-position-static-relative-v1.json`이다. 12 case·47 node, DPR 1·2, initial 및 두 mutation state에서 computed property, positioned owner, `flowRect`, visual `rect`를 node별로 비교한다. 모든 rect field 최대 절대 오차는 `0.5 CSS px` 이하여야 하고, owner mismatch는 실패다. WPT revision은 고정되어 있지만 WPT 전체는 실행 근거로 주장하지 않는다.

Rust style/layout/reference 검사를 통과했고 같은 JavaScript fixture를 Android 실기기와 iOS Simulator의 V8→Stylo→Taffy→WGPU 경로에서 실행했다. 두 플랫폼은 301×100 CSS px surface에서 47개 fixture 요소의 세 상태 geometry를 Chrome과 비교해 최대 오차 0 CSS px를 기록했다. root viewport의 폭·높이는 surface 크기가 달라 비교에서 제외했다. Android에서는 초기 Surface 경합을 수정한 뒤 앱 종료 후 다시 실행 3회 모두 세 상태를 기록했다. 모바일 화면은 scene presentation과 frame 로그의 보조 근거다. 구현 후 별도 실패 관점 검토에서 cascade·layout·frame·revision·runtime·platform 경계를 다시 확인하고 발견한 결함을 수정했다. 상세 근거는 [실행 기록](./evidence/c12-1-static-relative/README.md)과 [구현 실패 경로 검토](./evidence/c12-1-static-relative-implementation-review-2026-10-11.md)에 둔다.

이 상태는 현재 구현 브랜치의 검증 완료를 뜻한다. PR 검토·merge 전이며 C12 상위 범위를 완료 처리하지 않는다.

## 근거

- [C12 위치 지정 구현 계획](../../plan/c12-positioning.md)
- [Chromium 사전 비교 모델](./evidence/c12-1-static-relative-precomparison-2026-10-11.md)
- [C12.1 구현 실패 경로 검토](./evidence/c12-1-static-relative-implementation-review-2026-10-11.md)
- [Android 실기기·iOS Simulator 실행 근거와 캡처](./evidence/c12-1-static-relative/README.md)
- 고정 reference: `../../tests/fixtures/css/references/c12-1-position-static-relative-v1.json`
- [CSS Positioned Layout Module Level 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)
