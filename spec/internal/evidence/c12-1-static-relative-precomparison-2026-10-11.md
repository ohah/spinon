# C12.1 정적·상대 위치 사전 비교 결과

이 자료는 CSS Position 3의 `static`·`relative` 의미를 구현하기 전에 고정 Chromium 관찰값을 확보한 기록이다. Spinon 구현이 통과했다는 뜻이 아니며 C12.1 상태는 미구현으로 유지한다.

## 실행 재현

| 입력 | 고정값 |
| --- | --- |
| Chromium | Chrome 154.0.8037.98, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS arm64 |
| 실행 파일 SHA-256 | `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954` |
| CSS Position 기준 | [2025-10-07 W3C Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/); 기준 URL을 inventory와 reference에 보존 |
| 뷰포트 | 360 × 800 CSS px, DPR 1·2 |
| 환경 | `en-US`, UTC, light, forced colors 없음, coarse pointer, hover 없음, LTR, `horizontal-tb` |
| 입력 | 12 case, 47개 식별 노드; 동일 JavaScript fixture를 나중에 Spinon 런타임에도 제공 |
| 관찰 상태 | 초기 상태, target만 relative, ancestor와 중첩 ancestor도 relative |
| 허용 geometry 오차 | 각 frame field별 최대 0.5 CSS px; 누락 node·잘못된 owner는 오차와 별도로 실패 |
| WPT | revision `9ec154ff43db468923997c08bb08f905ceab62a5`; 일부 의미만 대응, WPT 실행은 하지 않음 |

실제 원본은 [고정 Chromium reference](../../../tests/fixtures/css/references/c12-1-position-static-relative-v1.json)이고 입력은 [fixture inventory](../../../tests/fixtures/css/c12/position-static-relative-inventory.json), [HTML 진입점](../../../tests/fixtures/css/c12/position-static-relative.html), [공용 JavaScript fixture](../../../tests/fixtures/css/c12/runtime-position-static-relative.js)이다. Chromium 실행 도구의 입력 hash도 reference 안에 기록한다. CSS Position 3에서 `static`의 inset 비적용, `relative`의 흐름 배치 후 시각 offset, non-static positioned box의 후손 containing block 자격을 근거로 삼았다. 해당 문서는 Working Draft이므로 구현도 이 고정 문서 버전에 대한 호환 범위로 비교한다.

재수집은 `mise exec -- bun run css:reference:c12-1-static-relative`, 검증은 `mise exec -- node --test tools/css-reference/c12-1-static-relative.test.mjs`로 한다. 기준 파일이 이미 존재하면 도구가 덮어쓰기를 거부한다.

## 기준에서 확인한 대표 동작

| Case | Chromium 관찰 |
| --- | --- |
| 정적 위치 | `left:12px; top:6px` computed value는 유지하지만 target의 frame은 `x=0, y=0`으로 흐름 frame과 같다. |
| 상대 위치 | target 흐름 frame은 `(0,40)`, 최종 frame은 `(12,37)`이다. 뒤 형제의 `y=52`는 바뀌지 않는다. |
| 반대 방향 inset | LTR에서 `right:8px`만 지정하면 x offset은 `-8px`; `bottom:4px`는 y offset `-4px`이다. |
| 양쪽 지정 | `left:5px; right:40px`의 수평 초과 제약은 LTR 기준 left가 사용되어 x offset `5px`이다. |
| 백분율 | content 크기 200×40, padding 10px인 containing block에서 `left:10%; top:25%` computed inset은 20px·10px이고 target frame은 `(30,180)`이다. |
| calc·var | `calc(10% + var(--move-x))`는 24px, `calc(-2px - var(--rise, 3px))`는 -5px로 계산된다. |
| shorthand·cascade | inset 1–4값 확장, longhand 덮어쓰기, cascade layer, `!important`의 최종 물리 inset을 별도 단언한다. |
| positioned owner | 중첩 relative box 안 자식은 가장 가까운 relative ancestor를 owner로 갖는다. static 자손도 이 owner 관계를 상속해 보고한다. |
| 숨김 subtree | `display:none` positioned parent와 자손은 layout box가 없고, 뒤 sibling은 흐름에 남는다. |
| style 변경 | target을 relative로 바꿔도 흐름 좌표와 뒤 sibling은 유지된다. ancestor를 relative로 바꾼 뒤에는 자손 owner와 최종 좌표가 갱신된다. |

## 범위와 한계

- reference는 Chrome에서 고정한 독립 oracle이다. owner 값은 DOM ancestry와 computed `position`을 따라 계산한 비교값이며 Spinon owner 구현의 결과가 아니다.
- flow frame은 fixture 요소들의 위치를 일시적으로 `static`으로 강제하고 inset을 `auto`로 바꾼 합성 관찰값이다. 해당 동작은 기준 계산법이며 브라우저가 제공하는 별도 CSSOM API로 표현되는 값은 아니다.
- DPR 간 computed style과 CSS px geometry의 동일성을 확인한다. 픽셀 rasterization·GPU paint·hit testing은 이 자료가 다루지 않는다.
- fixture는 Block 및 Flex의 고정 definite-size case에 한정한다. inline fragmentation, Grid, RTL, vertical writing mode, indefinite percentage basis, absolute/fixed/sticky, transform·contain 등 ancestor 효과는 지원 판정에서 제외한다.
- 고정 WPT commit의 `position-static-001`, `position-relative-001`, `position-relative-002`는 의미 일부만 fixture에 대응시켰다. `position-relative-006`·`007`의 indefinite/auto-sized percentage 입력은 제외했다. 저장소는 WPT suite를 실행하지 않았다.
- 이 PR 단계에서 Android 실기기와 iOS Simulator 제품 런타임 경로를 실행하지 않았다. 런타임 fixture 연결, CSS parser/cascade, layout output, renderer, 모바일 evidence가 아직 없다.

## 검증 및 구현 상태

- pinned Chromium capture 실행 완료: DPR 1·2, 두 환경에서 각각 세 상태를 수집했다.
- C12.1 reference 단위검사 통과: 고정 버전/hash, node identity·parent/child·owner, 각 의미 case, mutation, DPR invariant를 검사한다.
- fixture·capture source의 SHA-256을 기준 JSON과 비교한다.
- C12.1 제품 구현과 API contract는 미착수다. 다음 단계에서 내부 contract와 오류 경계, computed style snapshot, Taffy projection, final frame·flow frame 전달을 정한 뒤 구현해야 한다.
