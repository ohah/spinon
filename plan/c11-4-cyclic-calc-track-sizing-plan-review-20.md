# C11.4 cyclic `calc()` track sizing 계획 검토 20개

검토 대상: [cyclic `calc()` track sizing 계획](./c11-4-cyclic-calc-track-sizing.md), [Chrome 156 사전 관찰](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/chrome156-observation.json), 독립 CSS Grid 2025 CRD 계산.

| # | 실패 관점 | 차단 근거 또는 수정 |
| --- | --- | --- |
| 1 | intrinsic basis를 최종 percent 결과로 먼저 줄여 순환 계산 | 각 cyclic 축의 basis를 item max-content 24px/16px로 고정하고 final resolve를 뒤에 둔다. |
| 2 | `calc()`의 px 항을 percentage와 함께 잃거나 두 번 더함 | `50% + 4px`와 `50% + 2px`를 각각 16px/10px로 독립 산술 계산한다. |
| 3 | `calc()`를 무조건 fixed track으로 처리해 intrinsic sizing에서 cycle 누락 | CRD의 cyclic percentage 규칙을 계획 기준으로 쓰고 Chrome의 intrinsic root size를 대조한다. |
| 4 | 전체 `calc()`를 `auto`로 영구 치환해 definite final sizing에서 px 값을 얻지 못함 | definite control에서 24px/12px used track을 요구한다. |
| 5 | definite control을 cyclic case로 오판 | control root는 명시적으로 40×20px이고 cyclic scene과 별도 판정한다. |
| 6 | row percentage의 basis를 inline axis에서 가져옴 | row-only scene은 16px block-axis basis와 10px track을 확인한다. |
| 7 | column percentage의 basis를 block axis에서 가져옴 | column-only scene은 24px inline-axis basis와 16px track을 확인한다. |
| 8 | 한 축만 지원하고 두 축 동시 calc를 놓침 | simultaneous scene에서 columns `[16]`, rows `[10]`을 함께 검사한다. |
| 9 | auto-placement가 probe를 다음 track으로 밀어 track 측정값을 왜곡 | browser fixture의 item과 hidden probe를 같은 `1/2` cell에 고정한다. |
| 10 | hidden probe의 intrinsic size가 track sizing을 바꿈 | probe는 min-width/min-height 0이고 visibility hidden이며 크기 지정이 없어 사용 track 측정만 한다. |
| 11 | item이 track에 맞춰져 overflow 경계를 검출하지 못함 | item은 24×16px 고정이며 cyclic track보다 넓거나 높다. |
| 12 | root의 intrinsic size가 fixture viewport 폭/높이에서 우연히 결정됨 | cyclic scene은 양 축 `max-content` 또는 해당 축 `max-content`를 명시한다. |
| 13 | border/padding/gap이 basis에 섞여 독립 계산이 달라짐 | 모두 0으로 고정하고 content-box 크기만 비교한다. |
| 14 | track 크기만 맞고 root/item frame이 틀림 | Chrome 관측과 Rust/runtime acceptance에 track·root·item을 모두 둔다. |
| 15 | unsupported shape가 fixture 이름만 같으면 실행됨 | scene ID 외에 root/item 수, 태그, 스타일과 DOM 연결을 고정 gate에 둔다. |
| 16 | calc typed AST가 layout calc arena에서 다른 속성 id로 resolve됨 | cascade math, style projection, `LayoutCalcId`, column/row trace까지 연쇄 확인한다. |
| 17 | fixture 코드가 기본 제품 artifact에 섞임 | default artifact negative와 opt-in fixture-test positive control을 각각 검사한다. |
| 18 | Android/iOS의 layout 통과를 surface paint까지 확대 해석 | PixelCopy와 Metal drawable track 안/밖 표본을 별도로 검사하고 iOS compositor-after는 주장하지 않는다. |
| 19 | screenshot의 작은 item이 scene identity나 overflow를 보여주지 않음 | runtime 근거에 scene별 로그·화면을 보존하고 PR 본문에는 두 플랫폼 화면을 구분해 연결한다. |
| 20 | 한 Chrome 빌드 결과를 CSS 적합성 전체로 일반화 | Chrome 156은 단일 oracle로만 기록하고 WPT·다른 `calc()` grammar·C11.4 지원은 미완료로 명시한다. |

판정: 계획의 oracle, 축별 basis, definite control, fixture gate와 플랫폼 증거 경계를 분리했다. 구현 전 계획 관문 통과.
