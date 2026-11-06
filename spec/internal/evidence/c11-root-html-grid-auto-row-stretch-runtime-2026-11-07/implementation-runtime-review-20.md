# C11.4 auto implicit-row stretch 구현·runtime 실패 관점 검토 20개

대상은 이 후보의 style projection·layout placement/sizing·RuntimeSession 변경과 Android API 37 AVD/iOS Simulator pass02다. 계획 검토 20개는 별도 문서이며 여기서 재사용하지 않았다. 각 항목에서 지원 입력 경계, 실제 계산값 또는 게시 상태를 대조했다.

| # | 독립 실패 경로 | 확인 및 판정 |
|---:|---|---|
| 1 | `grid-auto-rows:auto`가 아닌 track list가 새 stretch route에 진입 | projection candidate와 RuntimeSession max-content rejection을 확인 — PASS. |
| 2 | explicit column/row 수가 늘어나 single-track slice가 확장 | candidate는 `[40px]`/`[10px]`만 받고 style unit test의 추가 column·잘못된 row negative가 거부됨 — PASS. |
| 3 | repeat, area 또는 auto-repeat metadata가 candidate와 섞임 | candidate의 repeat count, auto-repeat 및 template-area gate를 확인 — PASS. |
| 4 | column sparse flow 외의 placement flow가 자동 배치에 들어옴 | candidate에서 row flow를 요구하고 변형 단위 테스트가 이를 거부 — PASS. |
| 5 | 다른 Root HTML Grid owner가 함께 layout됨 | owner set, 후보 cardinality 및 전체 visible Grid 수를 검사 — PASS. |
| 6 | 다른 computed-style profile에서 새 mode가 선택됨 | projection은 `RuntimeRootHtmlTextPaintV1`에서만 candidate set을 만들고 route selector도 같은 profile을 확인 — PASS. |
| 7 | Grid의 width/height·min/max·aspect ratio가 확정 수용값 밖임 | item gate는 40×40px, auto min-size, none max-size 및 미설정 ratio를 요구 — PASS. |
| 8 | Grid padding/border 또는 nonzero gap이 used free space를 바꿈 | spacing/projection guard와 zero-gap condition을 확인 — PASS. |
| 9 | `align-content:normal`이 아니어도 auto-row stretch가 성공 처리됨 | element candidate와 runtime negative test가 `start`를 거부 — PASS. |
| 10 | item alignment 기본값이 track stretch와 혼동됨 | Grid `align-items:normal`, item `align-self:auto` 및 `justify-self:auto`를 별도 확인 — PASS. |
| 11 | first fixed item이 implicit row로 이동하거나 explicit row를 잃음 | DOM source order 및 40×10px fixed item 조건을 확인; 40×40 Grid geometry test 통과 — PASS. |
| 12 | text item이 첫 direct child가 되거나 fixed item 다음이 아닌 위치에 배치됨 | direct child index와 item kind 조합 검사; resulting text frame y=10px — PASS. |
| 13 | text item이 nested/inline markup 또는 여러 Text node로 바뀜 | 정확히 하나의 direct text child와 기존 ASCII/whitespace source gate를 재사용 — PASS. |
| 14 | text font·line height·white-space·writing mode가 20px minimum을 바꿈 | 기존 monospace 10px/10px direct-text allowlist 유지; trace에 두 줄·20px measurement 존재 — PASS. |
| 15 | fixed item 크기나 text item size가 intrinsic route 밖에서 배치됨 | fixed item 40×10px, text width/height auto와 auto min/max 조건을 각각 확인 — PASS. |
| 16 | extra item이 두 번째 implicit row를 생성해 자유 공간을 바꿈 | projection은 visible direct item 정확히 2개만 허용; runtime extra-item negative 통과 — PASS. |
| 17 | layout normalizer가 implicit auto row를 explicit template row로 확장함 | 새 mode에서는 row expansion을 하지 않고 Taffy implicit sizing에 맡김; trace explicit/implicit count `1/1` — PASS. |
| 18 | fixed 20px을 hard-code하거나 text contribution이 잘못됨 | RuntimeSession trace `Minimum=20px`, text layout 2 lines/20px, row base 20→30px을 별도 assertion — PASS. |
| 19 | 기존 intrinsic min-content route가 새 auto admission으로 넓어짐 | Legacy `RootHtmlIntrinsicTrackSizingV1` negative layout test가 auto implicit row를 계속 거부; intrinsic regression suites 통과 — PASS. |
| 20 | 실패가 partial layout을 남기거나 같은 session이 복구하지 못함 | unsupported candidate는 `completed=None`; dark media의 잘못된 alignment 실패 뒤 light media가 같은 session에서 다시 30px frame으로 복구 — PASS. |

검증 근거: layout intrinsic 14/14, style-to-layout intrinsic 20/20, RuntimeSession intrinsic rows default 19/19 및 fixture-test 19/19, 변경 세 crate all-target Clippy와 `cargo fmt --all -- --check` 통과. Android/iOS pass02의 `layout=ready`와 screenshot은 기본 경로 실행 확인이며 exact geometry 근거로 쓰지 않는다. Exact geometry와 track trace는 Rust test assertion이다. 검토에서 수정이 필요할 결함을 발견하지 않았다.
