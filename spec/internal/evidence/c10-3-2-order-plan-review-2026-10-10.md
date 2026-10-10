# C10.3.2 `order` 계획 · 실패 관점 검토

검토 대상은 [C10.3 계획](../../../plan/c10-3-flex-order-alignment.md)의 C10.3.2 절, 고정된 Stylo 0.22.0·Taffy 0.14.0 source, Chromium `154.0.8037.98` 실측, W3C CSS Flexbox·CSS Display·CSS Values 초안이다. 이 기록은 계획 검토이며 기능 구현이나 앱 검증 완료를 뜻하지 않는다.

## 계획 공격

| # | 독립 실패 관점 | 실패 질문 | 계획의 수정·통제 |
|---:|---|---|---|
| 1 | 표준 정의 분산 | Flexbox 문서의 설명만 읽고 `order`의 실제 property grammar·초깃값·상속 계약을 놓치는가? | Flexbox 2025-10-14 CRD는 flex layout/paint 동작, CSS Display 2026-06-05 CRD는 `order`의 `<integer>`, initial `0`, 비상속·적용 대상을 각각 고정한다. |
| 2 | Grid 적용 과장 | CSS `order`가 Grid item에도 적용되는데 Flex 구현을 모든 formatting context 지원으로 과장하는가? | 구현 범위를 runtime Flex item으로 제한한다. Grid 구현 전 `display:grid`는 선택된 profile에서 지원되는 것처럼 처리하지 않는다. |
| 3 | Stylo typed representation | 문자열을 임의 정수로 읽다가 Stylo가 실제로 계산한 signed 값과 다른가? | 잠긴 Stylo 0.22.0의 `CSSInteger = i32`와 `LonghandId::Order`를 조사했다. projection은 typed `i32`를 전달한다. |
| 4 | 양수 overflow | `i32::MAX`보다 큰 CSS literal이 잘리거나 wrap되어 가장 앞 순서로 오지 않는가? | 고정 Chromium 154가 초과값을 `2147483647`로 clamp함을 확인했다. 초과값의 source order를 역전해 clamp 동점과 안정 정렬을 구분한다. |
| 5 | 음수 underflow | `i32::MIN`보다 작은 값이 wrap되어 가장 뒤에 놓이지 않는가? | 고정 Chromium 154가 더 작은 literal을 `-2147483648`로 clamp함을 확인했다. MIN 주변의 source order를 반대로 두어 계산 순서를 판별한다. |
| 6 | CSSOM 경계 정밀도 | 큰 `order`의 `getComputedStyle()` 문자열을 그대로 정수 변환해 경계 검사를 잘못 통과시키는가? | Chromium은 int32 경계를 `2.14748e+09` 같은 반올림된 문자열로 반환했다. 경계 reference는 Typed OM 숫자와 node geometry를 함께 저장한다. |
| 7 | 문법상 분수 token | `1.0` 또는 `1.5`를 정수처럼 받아들여 앞선 유효 선언을 덮어쓰는가? | Chromium은 일반 분수 token을 무시했다. 기존 유효 선언과 initial `0`이 남는 두 경우를 따로 검사한다. |
| 8 | CSS integer 계산 | `calc()`가 지원되는 정수 문맥인데 소수 계산 결과 반올림 규칙을 놓치는가? | Chromium에서 `calc(1.5)`는 `2`, `calc(-1.5)`는 `-1`이다. CSS Values의 tie 방향과 Stylo 계산 경로를 fixture에 고정한다. |
| 9 | custom property cascade | `order:var(--item-order)`가 여섯 runtime Flex profile 중 일부에서만 빠지거나 cascade 순서를 잃는가? | 일반 runtime, paint, custom-property, registered-property 6개 profile과 stylesheet·inline 경로에서 computed 값 및 typed projection을 확인한다. |
| 10 | 음수 비교 | signed 값을 unsigned로 캐스팅해 `-1`이 양수 order보다 뒤에 오는가? | 음수·0·양수를 섞고 DOM source와 역전된 값 순서를 사용해 signed 오름차순을 검사한다. |
| 11 | 같은 값 tie | unstable sort 또는 NodeId 정렬로 같은 `order`의 원래 형제 순서가 바뀌는가? | stable sort를 사용하고 서로 다른 NodeId·크기·색을 가진 같은 order 형제를 둔다. |
| 12 | line collection 시점 | 이미 source 순서로 줄바꿈한 뒤 frame만 재정렬해 두 번째 줄 소속이 틀리는가? | line collection 전에 형제 순서를 적용한다. 정확히 맞는 줄 경계와 한 항목 초과 case를 비교한다. |
| 13 | flexible size 계산 | `order`는 paint만 바꾸고 flex base/grow/shrink 분배는 source order에 남아 다른 frame이 되는가? | layout 순서 변경 후 각 item의 basis·grow·shrink와 line별 frame을 Chromium과 비교한다. |
| 14 | reverse 축 혼합 | C10.3.1의 `row-reverse`/`column-reverse`를 CSS `order` 구현 대신 사용하거나 두 순서를 중복 적용하는가? | 같은 형제 집합에서 reverse만, `order`만, 두 기능 조합을 나누고 source NodeId별 frame 및 paint rank를 비교한다. |
| 15 | 비-Flex item 적용 | 일반 block 자식의 `order`를 시각적으로 재정렬하거나 Grid까지 지원한다고 오인하는가? | CSS `order`가 적용되지 않는 일반 Block 형제는 source 순서를 유지한다. Grid item은 별도 구현으로 남긴다. |
| 16 | 원본 DOM 순서 오염 | Taffy용 정렬 vector를 `HostDocument.children()`나 JS `childNodes` 결과에 다시 기록하는가? | Layout adapter 내부 복사본만 정렬한다. HostDocument source traversal, NodeId, 사용자 API 순서를 별도 불변식으로 검사한다. |
| 17 | 중첩 Flex scope | 바깥 item과 안쪽 item을 하나의 global order로 섞거나 자손을 부모 item 밖에 paint하는가? | 각 flex parent의 직접 자식에서만 정렬하고 재귀 paint traversal은 sibling item의 descendant subtree를 통째로 유지한다. |
| 18 | 숨김 요소와 anonymous item | `display:none` child를 flex item으로 배치·그리거나 미지원 text anonymous item을 조용히 건너뛰는가? | 숨김 branch는 결과 scene에서 제외하고, visible text/anonymous item은 현재 제한에 따라 명시적으로 거부한다. `visibility:hidden`의 layout 보존과도 혼동하지 않는다. |
| 19 | positioned item 경계 | absolute child를 line에 넣거나 paint 비교에서 실제 CSS `order`를 적용해 `order:0` 규칙을 놓치는가? | absolute child의 line 제외와 `order:0` paint 규칙은 문서에 기록하되, position 지원 전 C12/C10.3.5 완료를 가정하지 않는다. |
| 20 | raster·stacking·입력 경계 | paint rank가 실제 WGPU 제출 순서와 다르거나 `z-index`/stacking context 또는 아직 없는 runtime hit-test까지 구현했다고 주장하는가? | `z-index`/stacking은 C12·C22에 남긴다. 이 단계는 runtime scene 순서와 simulator WGPU 겹침 결과를 모두 확인하고, S05 pointer target 연결은 완료 범위에서 제외한다. |

## 확인한 사전 비교 사실

- 고정 Chromium은 `2147483648` 및 더 큰 양수를 `2147483647`로, `-2147483649` 및 더 작은 음수를 `-2147483648`로 처리했다. `computedStyleMap().get("order").value`는 경계값을 정확히 보존했고, `getComputedStyle().order`는 경계에서 지수 표기와 정밀도 손실을 보였다.
- 일반 `1.0`, `1.5` declaration은 무시됐다. 계산값 `calc(1.5)`, `calc(-1.5)`, `calc(1 + 2)`, `calc(2 * 3)`은 computed integer `2`, `-1`, `3`, `6`이 됐다.
- source 순서가 `2147483649`, `2147483648`, `2147483647`인 세 자식은 모두 Chromium에서 같은 최대 order로 clamp되어 source order를 유지했다. 최소 경계 아래도 같은 방식으로 안정 tie가 관찰됐다.
- 잠긴 Stylo source에서 `order`는 `LonghandId::Order`이며 `CSSInteger`는 `i32`다. Taffy 0.14.0 `Style`에는 CSS `order` 필드가 없고 flex item의 계산 순서는 child iterator에서 만든다. 따라서 Taffy에 넘기는 flex parent의 child ID 복사본을 안정 정렬해야 한다.

## 계획 수정

- C10.3.1의 PR #119 병합 상태를 계획·상태 대장·계약 색인에 동기화한다.
- 초안 값 범위와 실제 구현의 정수 표현을 구분하고, Stylo/Chromium 경계 `i32` 및 반대 source-order 입력을 고정한다.
- `calc()` 정수 반올림, invalid decimal declaration, CSS custom-property 경로를 구현 범위에 포함한다.
- `getComputedStyle().order`의 경계 정밀도 손실을 기록하고 Typed OM 숫자를 기준 관찰값으로 추가한다.
- CSS Display Level 3의 `order` 정의를 기준 문서에 추가하고 Grid item 적용은 미구현 범위로 명시한다.
- 겹침 결과의 완료 조건을 paint-list 검사만으로 끝내지 않고 Android/iOS Simulator WGPU 출력까지 확인하도록 강화한다.

계획 본문을 표의 20개 질문과 대조했다. 남은 발견 사항 없이 계획 단계의 재검토를 마쳤다. 이 기록 이후 실제 코드·fixture·runtime은 별도 구현 검토 대상이다.
