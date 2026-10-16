# C14.2 · 이미지 자연 비율과 대체 크기

## 목표

C14.1의 revision-stamped image snapshot을 확장해, HTML `img`의 자연 크기와 자연 비율을 지정 크기 및 min/max 제약과 함께 CSS px 레이아웃 크기로 계산한다. U05.1의 PNG는 자연 너비·높이가 모두 있으므로 해당 비율은 두 값에서 만든다. 입력 모델은 너비·높이·비율이 각각 없을 수 있는 리소스도 표현한다.

이 계획은 공개 CSS 지원 선언이 아니다. C14.1, C14.2, U05, S02 제품 runtime owner, C15 텍스트 측정과 X16 적합성은 각각 별도 경계를 유지한다. 내부 숫자 버전은 `0.1.0`을 유지한다.

## 현재 구현 상태

2026-10-14 기준 구현 후보 [PR #156](https://github.com/ohah/spinon/pull/156)은 부모 [PR #155](https://github.com/ohah/spinon/pull/155)가 main에 병합된 뒤 최신 main 위로 재기반했다. 재기반 뒤 workspace 773 passed·4 ignored, Clippy, rustfmt, diff check를 확인하고 PR 본문에 agent-browser Chrome 155 비교 화면을 첨부했다. 별도 구현 실패 20개, PR 변경 20개, 재기반 20개 검토를 남겼다. PR #156은 2026-10-14 rebase 방식으로 main에 병합됐다(merge SHA `18d3ff13b15ce42a4824167b2febdb9727995091`). C14.2와 상위 C14는 계속 미완료다.

2026-10-16 현재 남은 검증 공백은 기존 21개 Block PNG·8개 Flex oracle을 live V8 앱 fixture에서 실행한 Android/iOS Simulator 근거다. [C14.2 live V8 Simulator 검증 계획](c14-2-live-v8-simulator-verification.md)은 이 내부 검증 경계만 다루며 제품 image loader나 runtime admission을 추가하지 않는다.

## 기준 명세

- [CSS 2.2 §10.3.2](https://www.w3.org/TR/CSS22/visudet.html#inline-replaced-width)와 [§10.4](https://www.w3.org/TR/CSS22/visudet.html#min-max-widths): replaced width와 min/max width 조합.
- [CSS 2.2 §10.6.2](https://www.w3.org/TR/CSS22/visudet.html#inline-replaced-height): replaced height와 natural ratio 규칙.
- [CSS Images 3 §4](https://www.w3.org/TR/css-images-3/#default-sizing): natural dimension/ratio와 default object sizing.
- [CSS Sizing 4 §4.1](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio): `aspect-ratio`의 auto/numeric preferred ratio와 `box-sizing`에 따른 ratio dimension.

이 구현 slice는 CSS 2.2 replaced min/max 표와 CSS Sizing 4의 ratio box dimension 설명을 별도로 적용한다. 이 명세들을 전체 CSS 적합성 또는 모든 replaced element 지원으로 해석하지 않는다.

## 포함 범위

- 성공한 리소스 metadata entry의 자연 너비, 자연 높이, 자연 비율을 독립적으로 표현한다. entry가 없다는 것은 로드·디코드된 측정 결과가 없다는 뜻이다. 세 값이 모두 없는 entry는 성공적으로 확인했지만 자연 sizing 정보가 없는 경우다. 디코드 오류와 `src` 누락은 이 경계에서 여전히 별도의 실패이며, 빈 metadata로 바꾸지 않는다.
- 자연 너비·높이가 모두 있으면 비율을 `width / height`로 계산한다. 크기 하나와 자연 비율이 있으면 반대 축을 비율로 계산한다. 반대 축을 구할 비율도 없으면 CSS Images 기본 크기 `300×150 CSS px`의 해당 축을 사용한다.
- CSS `width`와 `height`가 둘 다 auto이면 자연 너비·높이가 있으면 그것을 사용한다. 한 자연 축만 있으면 자연 비율이 있을 때 반대 축을 비율로 유도하고, 없으면 반대 축에 기본 크기를 사용한다. 자연 크기나 비율이 모두 없는 성공 metadata entry는 `300×150 CSS px`를 사용한다.
- 두 CSS 축이 auto이고 자연 비율이 있는 경우 CSS 2.2 §10.4의 10개 min/max 제약 표를 적용한다. `max-width < min-width` 및 `max-height < min-height`는 각각 max를 min으로 올려 min이 우선한다.
- 한 CSS 축이 definite이고 다른 축이 auto이면 자연 비율을 먼저 사용하고, 비율이 없으면 해당 자연 축, 마지막으로 기본 객체 크기를 사용한다. definite 축의 제약과 auto 축의 제약은 CSS 2.2의 해당 축 규칙대로 적용한다. 따라서 auto 축에만 제약이 걸린 경우 used ratio가 깨질 수 있다.
- 현재 bridge가 보존하는 수치 `aspect-ratio`는 자연 비율보다 우선한다. `aspect-ratio:auto`는 자연 비율 사용과 같다. 기존 cascade가 거부하는 `aspect-ratio:auto <ratio>`는 이 단계에서도 거부한다. 수치 `aspect-ratio`와 min/max의 조합은 기존 C07.3 adapter 계약상 이 단계 밖이다.
- `box-sizing`, padding, border를 반영해 자연 비율은 콘텐츠 상자에 적용하고 edge는 한 번만 더한다. 수치 `aspect-ratio`는 `box-sizing`이 지정한 box dimensions를 사용한다. 단위는 DPR 변환 전 CSS px다.
- 기존 layout source와 `IntrinsicSizeRevision` 검증 및 output stamp를 유지한다. 보이는 auto 축에 metadata entry가 없으면 `MissingIntrinsicSize`; hidden subtree는 metadata를 요구하지 않는다.
- runtime block 및 현재 지원하는 flex auto-basis leaf 경계를 유지한다. CSS Grid intrinsic contribution, flex grow/shrink 뒤 반대축 재측정, shrink-to-fit, percentage와 CSS math 기반 width/height/min/max는 명시적 오류 또는 후속 단계로 남긴다.

## 제외 범위

ratio만 있는 이미지의 두 CSS 축 auto intrinsic contribution은 C14.3에서 다룬다. 이 크기는 layout context의 max-content/initial-containing-block 규칙과 연동되므로 `300×150` 기본 객체 크기로 가장하지 않는다. `<input>`, `<video>`, `<canvas>`, SVG `image`, SVG decode, 이미지 실패 아이콘, 비동기 loader/cache/replacement, `object-fit`/`object-position`, GPU 표시, 앱 runtime admission, 텍스트와 WPT 전체 적합성도 포함하지 않는다.

현재 `spinon-image` PNG snapshot adapter는 성공 PNG의 두 자연 축을 채운다. 자연 metadata가 없는 성공 리소스 입력의 기본 크기는 `spinon-layout` snapshot 단위 fixture에서 확인하며, 새 이미지 포맷이나 decode 경로가 완료됐다고 주장하지 않는다.

## 크기 계산 계약

1. `LayoutIntrinsicSizeSnapshot`의 entry 존재와 metadata 값은 별개다. 세 필드가 모두 `None`이어도 성공 metadata entry로 유효하다. 엔진에서 entry 누락은 데이터가 없는 상태이므로 visible auto축 계산을 실패시킨다.
2. 존재하는 자연 너비·높이·비율은 양수·유한이어야 한다. 두 자연 축과 명시 자연 비율이 함께 오면 비율이 `width / height`와 일치해야 한다. 나눗셈/곱셈 overflow와 0 또는 무한 비율은 실패한다.
3. C14.2의 CSS width/height/min/max/padding 입력은 `auto` 또는 절대 CSS px length만 받는다. 백분율, `calc()`, sizing keyword는 이 계산 경로에서 사용하지 않는다. 기존 style projection에서 property/node를 식별해 거부한다.
4. border-box 값을 콘텐츠-box 크기로 바꾸기 전에 각 물리 축의 padding·border 합을 구한다. border-box의 CSS width/height/min/max에서 edge를 뺀 값은 0에서 바닥을 둔다. 최종 콘텐츠 크기를 다시 원래 `box-sizing`의 Taffy property 값으로 바꾼다.
5. 둘 다 auto이고 ratio가 있는 경우, 제약 전 크기 `w,h`와 `min-width`, `max-width`, `min-height`, `max-height`의 모든 10개 위반 조합을 CSS 2.2 표에서 고른다. 구현은 표의 `max(min,max)` 정규화를 먼저 수행하고 ratio 비교도 유한값만 사용한다. ratio가 없으면 각 축을 독립적으로 clamp한다.
6. 둘 다 definite이면 natural metadata는 결과 크기를 바꾸지 않는다. 한 축만 auto이면 effective ratio가 있으면 반대 축을 구한 뒤 min/max를 적용한다. auto 축의 제약으로 over-constrained가 생기는 CSS 2.2 동작을 임의로 ratio 보정하지 않는다.
7. Taffy 0.14.0의 일반 leaf ratio 보정은 max constraint 뒤에도 ratio를 재적용해 CSS 2.2의 auto축 제약 동작과 다를 수 있다. 그래서 계산된 replaced size는 Taffy의 ratio 재보정에 맡기지 않고, 하나의 검증된 content-box 결과를 Taffy leaf 경계에 전달한다. block, flex auto-basis, flow-only와 visual pass가 같은 결과를 쓰는지 각각 확인한다.

## 구현 전 비교 기준

- Fixture: [일반 흐름 replaced sizing](../tests/fixtures/css/c14/replaced-ratio-sizing-precomparison.html) / [Flex replaced sizing](../tests/fixtures/css/c14/replaced-ratio-sizing-flex-precomparison.html); 각각의 case inventory는 fixture 옆에 둔다.
- 관찰 결과와 전체 user-agent/viewport/device-scale 조건: [Chrome 155 사전 비교](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/README.md), [일반 흐름 관찰](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-observation.json), [Flex 관찰](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155-flex-observation.json), [일반 흐름 화면](../spec/internal/evidence/c14-2-image-ratio-precomparison-2026-10-14/chrome155.png).
- 브라우저는 agent-browser가 연 로컬 fixture의 Chrome Headless `155.0.8059.40`, viewport `1280×633 CSS px`, DPR `1`이다. PNG 입력은 저장소의 고정 `24×16` 파일이며 source bytes SHA-256은 사전 비교 README에 있다. terminal-browser를 쓰지 않았다.
- C14.2 기준 case는 `24×16` 자연 크기, width `48`이면 `48×32`, height `24`이면 `36×24`, min-width `48`이면 `48×32`, max-height `10`이면 `15×10`, 양축 max `18×10`이면 `15×10`, min-width `48`/max-height `24` 충돌은 `48×24`, border-box `48px`과 padding/border는 `48×34`다. Browser의 `max-width:20px` 결과만 `19.984375px`로 관찰됐으며 비교 tolerance는 `0.05 CSS px`; 나머지 고정 정수 경계도 `0.05 CSS px`로 비교한다.
- 본 비교는 DOM 관찰 oracle다. U05 PNG decode→snapshot은 별도 Rust fixture로 재검증한다. 성공하지만 자연 크기가 없는 metadata fallback은 layout 단위 테스트에서 `300×150` 및 clamp 불변식과 비교한다.

## 구현 순서

1. 이 계획을 별도 실패 관점 20개로 검토하고, [검토 기록](../spec/internal/evidence/c14-2-image-ratio-plan-review-2026-10-14.md)의 발견과 후속 gate를 반영한다.
2. 사전 fixture/observations/inventory의 hash와 browser identity를 고정한다.
3. intrinsic metadata 입력 모델을 optional natural dimensions/ratio로 확장하고 유효성·revision/source 검증을 보존한다. 기존 PNG adapter는 `24×16`을 전달하고 ratio를 파생한다.
4. C14.2 supported style profile에서 image sizing을 preflight한다. fixed/fixed 무시, auto축 metadata 누락, percent/math, unsupported auto ratio, CSS aspect ratio와 min/max 조합을 적절한 식별 오류로 닫는다.
5. 별도 pure replaced-sizing solver에 기본크기, 한 축 transfer, CSS 2.2 10-row min/max, box-sizing edge 변환을 구현한다. 누락·숨김·잘못된 metadata는 기존 source/revision gate를 우회하지 않는다.
6. Taffy replaced leaf 경계가 solver 결과를 동일한 CSS box sizing으로 적용하도록 연결한다. Flex의 자식 측정도 지정된 한 축, natural ratio, min/max 결과를 전달해야 하며 Taffy ratio 전파가 used size를 다시 덮지 않아야 한다. `item_is_replaced`, flex auto-basis, positioned flow pass 및 display-none 동작을 유지한다.
7. layout 및 style-to-layout positive/negative tests를 추가한다. 브라우저 fixture의 PNG cases와 synthetic optional-metadata cases를 비교한다. stale source/revision, duplicate/unknown/non-image/non-leaf input, NaN·infinity·zero·ratio overflow, min>max를 negative control로 둔다.
8. 계획에서 사용하지 않은 새로운 20개 관점으로 구현·실패 경로를 검토하고, 결함 수정 뒤 영향 검토를 반복한다.
9. 계약 0009, C14 parent plan, internal index, STATUS를 갱신한다. C14.1/C14.2 상위, U05, S02 제품 runtime owner, C15, X16 완료는 주장하지 않는다.
10. Rust workspace 검증과 가능한 Android API 37 AVD Rust test executable 검증을 수행한다. AVD 실행은 앱/V8/runtime integration이 아니라 OS/ABI에서 Rust 경계가 실행된 근거로만 보고한다. 실기기는 요구하지 않는다.
11. 부모 PR #155 병합 뒤 PR #156을 최신 main에 재기반하고, 본문에 비교 화면·근거·미검증 경계를 연결한다. 별도 PR 변경 및 재기반 적대 검토를 기록하고 GitHub의 현재 필수 gate와 mergeability가 허용하는 경우 리베이스 병합한다.

## 완료 판정

- Chrome PNG oracle 및 별도 CSS 2.2 알고리즘을 기준으로 natural size, ratio transfer, min/max의 10가지 조합, min>max, edge/box sizing을 검증한다.
- successful no-natural metadata entry는 기본 크기 경로를 타고, missing/failed decode는 해당 경로와 구분되어 실패한다.
- 기존 C14.1 behavior와 source/revision stamp, hidden image, fixed/fixed ignore, flex auto-basis 회귀가 없다.
- browser fixture, Rust tests, simulator test (실행 시) 결과와 한계를 해당 증거 파일에 남긴다. CI status, 앱 runtime integration, WPT, 실기기, GPU 출력은 별도 상태다.
- 계획 검토 20개와 구현 검토 20개에서 해결되지 않은 P1/P2가 없다. C14 및 상위 기능은 계속 미완료로 둔다.
