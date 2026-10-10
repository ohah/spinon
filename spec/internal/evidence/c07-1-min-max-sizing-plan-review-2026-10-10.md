# C07.1 · 계획 실패 경로 검토

계획과 고정 Chromium 비교 모델을 구현 전에 서로 다른 실패 관점으로 공격했다. 각 행은 별도 입력·불변 조건 또는 계약 경계를 검토한다. 이 문서는 구현 코드 검토를 대신하지 않는다.

| # | 공격 관점 | 계획을 깨뜨리는 경우 | 계획의 대응 또는 수정 |
| ---: | --- | --- | --- |
| 1 | Oracle drift | Chrome 자동 업데이트 후 기준이 바뀌어 오차가 숨겨진다. | 실행 파일 버전·revision·SHA-256을 고정하고 불일치하면 캡처 실패. |
| 2 | Fixture provenance | HTML만 바뀌고 inventory/reference가 옛 입력을 가리킨다. | HTML·inventory·capture tool hash를 reference에 묶고 변경 후 새 비교를 요구. |
| 3 | DPR leakage | physical pixel 반올림이 CSS px 결과를 바꾼다. | DPR 1·2의 computed observation과 모든 rect 필드 동일성을 요구. |
| 4 | Computed/used 혼동 | `getComputedStyle()` 문자열을 최종 used geometry로 오해한다. | property typed observation과 `getBoundingClientRect()`를 별도 산출값으로 비교. |
| 5 | Min/max precedence | `min > max`에서 max가 우선되어 Chrome과 달라진다. | width 80, min 80, max 50 케이스로 min 우선 결과를 고정. |
| 6 | Content-box sizing | max 100에 padding을 포함해 100 전체 frame으로 잘못 제한한다. | content-box 및 border-box 같은 padding 값을 짝지어 비교. |
| 7 | Padding floor | border-box 지정값보다 padding 합계가 커 음수 content size가 생긴다. | 10px min과 24px padding 입력으로 frame 최소 24px을 비교. |
| 8 | Wrong percentage axis | `max-height:50%`를 부모 width 기준으로 푼다. | definite width/height가 다른 부모를 두고 높이 60px을 고정. |
| 9 | Indefinite basis | 부모 auto 크기에서 percentage가 임의 0 또는 viewport로 환산된다. | C06의 definite/indefinite 계약을 재사용하고 무근거 fallback을 허용하지 않음. |
| 10 | Flex shrink ordering | shrink 후 min constraint가 적용되지 않는다. | 두 100px basis에 150px 부모와 min 80px를 넣어 각 80px을 비교. |
| 11 | Flex growth clamp | grow 단계에서 max가 무시되어 항목이 남는 공간까지 커진다. | max 60px 두 항목과 space-between 최종 x좌표를 함께 검사. |
| 12 | Auto min intrinsic | 빈 박스 결과를 텍스트의 min-content까지 지원한다고 과장한다. | `auto` 비교는 빈 요소에 한정하고 text shaping/intrinsic content를 제외. |
| 13 | `none` mapping | max `none`을 0으로 보내 모든 항목이 사라진다. | max `none`은 무제한 의미로 Taffy auto constraint에 매핑하고 직접 테스트. |
| 14 | Intrinsic keyword | `min-content` 등 지원되지 않는 키워드를 0/auto로 조용히 성공 처리한다. | Stylo typed enum의 미지원 variant는 속성 포함 오류로 거부. |
| 15 | Invalid declaration | 음수 선언을 computed value로 채택해 앞선 유효 선언을 덮는다. | CSS cascade에서 무효 선언 제거와 layout DTO의 잘못된 값 거부를 각각 시험. |
| 16 | CSS math overflow | calc 평가가 NaN/무한대가 되어 일부 노드만 새 geometry로 표시된다. | 기존 Taffy calc resolver의 finite censor와 failure-atomic layout을 유지. |
| 17 | Cascade source mismatch | `var()`의 이긴 선언과 typed computed AST가 서로 다른 rule에서 온다. | Stylo winner source 및 custom/registered property profile 각각을 비교. |
| 18 | Profile allowlist gap | cascade는 값을 내지만 runtime adapter 허용 목록이 누락 또는 과잉 허용한다. | runtime layout과 실제 파생 profile 등록·인라인 검사·스타일 투영을 교차 검사. |
| 19 | Unsupported baseline inflation | Chrome fixture의 미지원 노드까지 분모에 넣어 결과가 좋아 보인다. | 인벤토리에서 지원/관찰만 구분하고 pass 분모에 관찰 전용 노드를 넣지 않음. |
| 20 | Simulator overclaim | 시뮬레이터 표시 성공을 실제 GPU·실기기 성능 증거라고 보고한다. | 시뮬레이터는 통합 경로만 증명하며 hardware GPU·실기기·성능을 명시적으로 제외. |

## 검토 결과 반영

처음 만든 percentage height 사례는 `800px` 부모 아래에 있어 `50%` 제한이 실제로 걸리지 않았다. 이 계획 검토 전에 입력을 `120px` definite-height 부모로 옮겨 기대 결과를 `60px`로 고쳤고, 고정 Chrome reference를 다시 생성했다. 현재 계획 범위에는 미결정 선택지가 남아 있지 않다. 구현 검토는 별도 20개 관점으로 수행한다.
