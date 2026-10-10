# 0044 · C07.2 테두리 폭 레이아웃

**문서 ID:** `0044` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** PR #103 리베이스 병합 완료 · **공개 API:** 아님

`0044`는 문서 ID다. 출시 전 내부 계약 숫자 버전 `0.1.0`과 모든 Spinon crate 숫자 버전은 구현·문서 작업으로 올리지 않는다.

## 범위와 입력

현재 제한 runtime layout profile에서 물리 방향 `border-top/right/bottom/left-width` 및 `border-width`·`border` shorthand가 만든 cascade 결과를 box layout에 적용한다. border를 화면에 그리는 계약은 아니다. `border-style`은 면별 폭이 0인지 결정하기 위한 입력이다.

Stylo의 typed computed `BorderSideWidth`와 `BorderStyle`을 입력으로 사용한다. 물리 네 면은 서로 독립적이다. Stylo에서 `calc()`·`var()`가 computed scalar로 해결된 뒤 CSS px로 변환한다. pinned Chrome `computedStyleMap()`도 해당 폭을 `CSSUnitValue`로 제공한다. border 값은 문자열 재파싱이나 CSS math AST 전달을 하지 않는다.

고정 Chrome은 fractional 폭을 DPR 1·2에서 같은 정수 CSS px로 snap한다. Stylo typed 값은 DPR 2에서 일부 소수 폭을 보존하므로 그대로 Taffy에 전달하면 Chrome과 frame 크기가 다르다. 이 계약은 50-node reference에서 확인한 양수 폭을 `max(1, floor(widthCssPx))`로 정규화한다. 정확한 0 및 `none`·`hidden`은 0이다. 이는 고정 Chrome 비교 규칙이며 다른 브라우저의 일반 규칙이라고 주장하지 않는다. 내부 computed side width 문자열과 Taffy border 입력은 같은 정규화 폭을 사용한다.

## 면별 사용 폭

| computed border style | layout에 전달하는 면별 폭 |
| --- | --- |
| `none`, `hidden` | `0 CSS px` |
| 다른 유효한 border style | pinned Chrome CSS px 규칙으로 정규화한 Stylo typed border width |

`border-color`는 이 계약의 layout 입력이 아니다. 투명 색상도 width contribution을 제거하지 않는다. `outline`은 box geometry에 포함하지 않는다. CSS `border` shorthand가 초기값으로 재설정하는 `border-image-*` 선언은 그 초기값일 때만 parser profile 검사를 통과하며, author가 비초기 border-image 값을 지정하면 거부한다.

border 폭은 nonnegative finite CSS length여야 한다. `thin`, `medium`, `thick` 및 길이 단위 환산·`calc()`·`var()`의 scalar 결과는 고정 Chromium 기준과 Stylo typed computed value를 비교한다. 퍼센트와 음수는 CSS 선언 단계에서 무효다. 앞선 유효 선언이 있으면 cascade winner로 유지된다. 내부 `LayoutBorder` DTO는 각 면을 CSS px `f32`로만 표현하므로 percentage·잘못된 CSS math 소유권을 전달할 수 없다. 음수·NaN/Infinity는 node와 면을 포함한 오류이며 partial layout을 만들지 않는다.

## 상자 계산

- `content-box`: 콘텐츠 지정 크기에 padding과 면별 사용 테두리 폭이 추가되어 외곽 frame을 만든다.
- `border-box`: 지정 크기는 padding과 면별 사용 테두리 폭을 포함한다. 내부 콘텐츠 크기가 음수가 되는 경우 외곽 frame은 padding·테두리 합보다 작아지지 않는다.
- 기존 min/max, percentage basis, flex shrink와 growth는 테두리를 포함한 같은 Taffy box model을 사용한다.
- `border-style` 또는 `border-width`가 바뀌면 style revision 변경을 통해 layout 입력에 반영한다. 색상만 바꾸어도 used width와 frame은 같지만 현재 전역 `StyleRevision` 모델에서는 계산을 다시 수행할 수 있다. C07.2는 속성별 재계산 생략 최적화를 약속하지 않는다.

## 자료 흐름과 실패

Stylo same-revision snapshot → 면별 typed width/style → CSS px 정규화 → none/hidden used-width gate → `LayoutStyle.border` (`LayoutBorder`) → Taffy `Style.border` → `LayoutFrame` 순서다. 계산식 ID는 border 경계에 존재하지 않는다. Taffy가 box model에 한 번만 반영하며 adapter는 값을 중복 가산하지 않는다. 잘못된 입력, stale style revision, Taffy panic 또는 non-finite 결과는 기존 `LayoutError` 경로로 전체 계산을 거부한다.

## 고정 검증 기준

Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`의 고정된 50-node fixture와 DPR 1·2 관찰이 기준이다. 기준 파일의 HTML, inventory, 캡처기, helper, Chrome 실행 파일 SHA-256이 일치하지 않으면 비교를 무효화한다. 각 node의 계산값과 `x/y/width/height`를 대조한다. 각 frame field 최대 절대 오차는 `0.5 CSS px` 이하여야 하며 CSS px 결과는 DPR에 독립적이어야 한다.

계획 검토와 구현 검토는 별개다. 계획 검토와 구현 후 실패 경로 검토 결과는 각각 연결한 근거에 둔다. 이 계약은 제한 runtime profile의 내부 구현 경계이며 공개 API 지원 선언이 아니다. PR #103 병합에 따라 C07.2 상태를 공식 상태 대장과 Tailscale 미리보기에 동기화한다. C07 상위는 aspect ratio 등 잔여 범위 때문에 미완료다.

## 구현·실행 근거

- [구현 전 Chrome 비교](./evidence/c07-2-border-width-layout-precomparison-2026-10-10.md)
- [Android·iOS Simulator 실행](./evidence/c07-2-border-width-layout-simulators-2026-10-10.md)
- [구현 실패 관점 검토](./evidence/c07-2-border-width-layout-implementation-review-2026-10-10.md)
