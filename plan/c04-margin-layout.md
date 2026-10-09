# C04.6 · CSS margin → Taffy Flex 입력

- **문서 유형:** 구현 계획 · 현재 구현 상태는 [상태 대장](../spec/STATUS.md#css-구현-체크리스트)에서 관리
- **기준일:** 2026-10-09
- **상위 항목:** [C04 stylesheet·selector·cascade](../spec/STATUS.md#css-구현-체크리스트)
- **선행 구현:** C04.2 `FlexLayoutV1` Stylo cascade → Taffy adapter

## 목표

CSS cascade에서 계산된 네 방향의 margin을 Taffy Flex 입력으로 전달한다. 기존 C04.2·C04.3·C04.4 profile은 그대로 두고, 새 `FlexMarginV1` profile과 `compute_flex_margin_style_layout` entrypoint를 추가한다. 이 구현은 CSS 값을 실제 프레임 위치에 반영하는 레이아웃 기능이다.

## 입력과 동작 계약

- C04.2에서 지원하는 `display`, `box-sizing`, `width`, `height`, `flex-direction`, `flex-grow`, `flex-shrink`, `flex-basis`, `direction`, `row-gap`, `column-gap`에 `margin-top`, `margin-right`, `margin-bottom`, `margin-left` computed value를 추가한다.
- Author shorthand `margin`, `margin-inline`, `margin-block`, 네 physical longhand, 네 logical margin longhand는 허용한다. logical margin은 Stylo가 computed `margin-top/right/bottom/left`로 변환한 뒤 투영한다. `writing-mode` 변경은 허용하지 않으며 기본 `horizontal-tb`만 대상으로 한다.
- Taffy에 전달할 margin computed value는 유한한 CSS px만 허용한다. 음수 px는 유효한 margin으로 전달한다. `%`, `auto`, `calc()` 등 px가 아닌 계산값은 node·property·원본 값을 포함해 실패시키며 기본값으로 대체하지 않는다.
- 선택된 layout root의 네 margin이 모두 0이어야 한다. viewport에 직접 매핑한 root margin은 부모 containing block이 없어 현재 root 계약으로 정확히 해석할 수 없으므로 nonzero 값은 거부한다.
- top/right/bottom/left는 물리 방향으로 유지한다. RTL에서는 Stylo computed 값이 결정한 물리 좌우 값을 그대로 Taffy에 전달한다.
- subtree에 inline `style`, 텍스트 노드, CSS 진단 또는 현재 profile 바깥 author 선언이 있으면 전체 요청을 실패한다. 이전 profile을 통과시켜 일부 결과를 반환하지 않는다.
- root·viewport·revision 및 Taffy 출력의 기존 계약은 유지한다. 런타임 스케줄링, 자동 무효화, 공개 DOM/CSSOM API, text layout, GPU·Android·iOS 앱 연결은 범위 밖이다.

## 비교 기준

- Chromium oracle은 저장소의 고정 capture helper와 Chrome `154.0.8037.98` / revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`을 사용한다.
- 기본 10 case와 logical shorthand 3 case를 별도 input/reference 묶음으로 저장한다. 기존 C04 fixture를 수정하거나 이미 생성된 reference를 덮어쓰지 않는다.
- 고정 viewport `301×80 CSS px`, scale `1`; root 1개와 flex item 3개를 둔다. case는 네 방향 longhand, shorthand 1/2/3/4-value expansion, inline/author cascade 우선순위, 음수 margin, RTL computed physical edges를 포함한다.
- 성공 case는 computed margin 문자열과 root·item의 `x`, `y`, `width`, `height`를 각각 비교한다. CSS 계산 문자열은 정확히 같아야 하고 각 좌표의 최대 절대 오차는 `0.5 CSS px` 이하여야 한다.
- `%`, `auto`, `calc()`, 지원 밖 property, inline style, parse diagnostic, stale snapshot은 실패해야 하며 실패 결과에 layout output이 없어야 한다.

## 구현 단계

1. 고정 Chromium reference와 입력 hash를 만든다. 기존 산출물은 덮어쓰지 않는다.
2. `spinon-layout::LayoutStyle`에 음수도 허용하는 margin edge 값을 추가하고 Taffy 변환 및 유효성 검사를 연결한다. 기존 default margin은 0이다.
3. `spinon-style`에 새 `FlexMarginV1` computed-style profile과 정확한 author property allowlist를 추가한다.
4. `spinon-style-to-layout`에 전용 entrypoint와 CSS px parser를 추가한다. margin은 음수 CSS px를 허용하고 비-px 값은 전체 실패 처리한다.
5. 새 내부 계약·fixture·실행 근거를 추가하고 C04의 C04.6 하위 항목만 완료 처리한다. 공개 API와 C04 전체는 미완료로 둔다.
6. C04.2·C04.3·C04.4 회귀, 신규 Chromium geometry, 실패 경계, revision 경계를 실행한 다음 구현 코드에 대한 별도의 20개 실패 관점을 기록한다.

## 계획 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 반영한 방지 기준 |
|---:|---|---|
| 1 | 새 margin 필드가 기존 호출자의 기본 배치를 바꿈 | default는 0이고 기존 reference 전체를 회귀 실행한다. |
| 2 | 음수 margin을 부정값이라는 이유로 거부 | margin 전용 유효성 검사는 유한성만 요구하고 negative fixture를 둔다. |
| 3 | 음수 값이 CSS px parser에서 잘못 잘림 | 부호·소수·0을 포함하는 parser case를 고정한다. |
| 4 | `%`를 viewport 기준으로 잘못 계산 | `%`는 computed 문자열 단계에서 명시 거부한다. |
| 5 | `auto`를 0으로 바꿔 분배 결과를 감춤 | `auto`를 미지원 computed 값으로 반환하고 output을 만들지 않는다. |
| 6 | `calc()`를 단순 px처럼 부분 파싱 | suffix parser는 숫자와 `px` 이외 입력을 전부 거부한다. |
| 7 | `margin` shorthand를 불완전한 한 방향 값으로 처리 | 1~4개 값 확장을 Chromium computed output과 확인한다. |
| 8 | logical margin shorthand/longhand가 Stylo allowlist를 우회하거나 RTL에서 물리 좌우로 잘못 매핑 | [logical shorthand fixture](../tests/fixtures/css/c04/margin-logical-shorthand.v1.json)에서 inline/block shorthand를 LTR·RTL Chromium computed values 및 frames와 비교한다. |
| 9 | vertical writing mode의 logical edge를 horizontal로 투영 | writing-mode 선언은 allowlist에서 거부한다. |
| 10 | Taffy가 자식 margin을 무시하거나 parent padding과 혼합 | 비대칭 네 방향 fixture에서 모든 frame을 개별 비교한다. |
| 11 | 인접 Block margin collapse를 Flex margin으로 잘못 주장 | 이번 geometry를 Flex container로 제한하고 collapse는 범위 밖으로 고정한다. |
| 12 | margin이 gap과 중복되어 Flex 간격이 과대 계산 | margin과 gap을 별도 case 및 computed input으로 검증한다. |
| 13 | 선택 root의 margin을 부모 없이 Taffy가 조용히 무시 | root의 nonzero margin을 명시 오류로 거부한다. |
| 14 | `display:none` 노드의 margin이 sibling 위치에 영향을 줌 | none child를 둔 대조 case에서 후속 item frame을 검사한다. |
| 15 | author shorthand allowlist가 무관한 property를 함께 허용 | 새 profile은 기존 Flex 집합과 명시한 margin property만 허용한다. |
| 16 | stylesheet 오류가 일부 스타일만 반영한 layout을 반환 | diagnostic 발견 시 현재 adapter와 동일하게 전체 실패한다. |
| 17 | inline style이 author stylesheet처럼 조용히 수용됨 | 기존 inline-style 거부 경계를 유지한다. |
| 18 | 다른 document revision의 computed style이 사용됨 | generation/document/render-tree mismatch 실패를 별도 확인한다. |
| 19 | duplicate/missing computed element가 일부 frame을 냄 | 기존 projection 구조 검증과 실패 결과 부재를 확인한다. |
| 20 | 내부 Flex margin 구현을 웹·제품 CSS 지원 완료로 확대 해석 | 상태 대장·계약·실행 근거에서 API/런타임/GPU/플랫폼 범위를 분리한다. |

이 표는 계획 단계 검토만 나타낸다. 구현 뒤의 적대적 검토는 별도 증거 파일에 다른 실패 관점으로 기록한다.
