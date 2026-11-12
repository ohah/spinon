# C11.4 five-fit row-span mixed minimum 실제 변경 검토 30개

검토 기준은 현재 branch의 변경 파일과 기준 HEAD `33d35857b6e3f7f5b2bc2f83781ec7ed711e99b6`이다. 확인 대상은 style/layout admission, 공유 five-/six-fit span sizing과 1/64px rounding, RuntimeSession matrix/rejection tests, 계획 및 evidence다. 검토 뒤에도 C11.4/C11 상태와 공식 `78/214`는 변경하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | Style projection이 five-fit 이름만으로 임의 track list를 허용 | PASS — 기존 typed-cap multiset·six-member exact predicate `supports_repeated_five_fit_row_span_track_list`를 재사용한다. |
| 2 | Layout admission이 Style보다 넓거나 좁은 profile을 허용 | PASS — layout은 동일한 five-fit track predicate와 기존 repeated-text row-span profile을 모두 요구한다. |
| 3 | Style과 Layout이 control 또는 span-item minimum을 다르게 해석 | PASS — 두 계층 모두 첫 empty explicit-row control을 zero로 고정하고 나머지는 `Auto|Fixed(0)`만 받는다. |
| 4 | Control item이 DOM child 순서에 따라 span item으로 간주됨 | PASS — 기존 topology gate가 control과 span topology를 구별하고, mask test 및 auto-control rejection이 양 계층 경계를 확인한다. |
| 5 | Control `min-height:auto`가 mixed mask로 조용히 허용 | PASS — RuntimeSession media-query mutation이 atomic rejection과 same-session recovery를 확인한다. |
| 6 | 다른 양수·음수·계산 minimum 값이 0/auto로 뭉개짐 | PASS — computed type admission은 `LengthPx(0)` 또는 `Auto`만 수용하며 invalid-value rejection regression이 있다. |
| 7 | Mixed profile 추가로 기존 all-zero/all-auto endpoint가 깨짐 | PASS — 기존 five-fit uniform endpoint 34,560 scenes가 네 청크 모두 통과했다. |
| 8 | 다섯 typed cap의 저자 순서가 정규화되어 바뀜 | PASS — exact multiset helper는 membership만 검사하고 입력 order는 유지한다. Oracle metadata가 cap position/type을 검사한다. |
| 9 | 중복·누락·미등록 cap, 잘못된 tail 또는 추가 track이 통과 | PASS — 기존 five-fit profile mutation tests와 metadata/sample verifier가 각 경계를 거부·검증한다. |
| 10 | Percentage cap이 definite block-size 없이 해석됨 | PASS — profile의 definite `40×220px` Grid 조건과 simulator의 유한 percentage basis 검사가 유지된다. |
| 11 | Grid owner·document revision이 다른 trace와 결합 | PASS — RuntimeSession은 같은 accepted Root HTML revision의 owner/frame을 관측하고 실패 revision은 completed snapshot을 게시하지 않는다. |
| 12 | Auto-placement나 column/dense flow로 admission 범위가 번짐 | PASS — 기존 exact row-placement/topology gate는 변경되지 않았고 sample은 fixed sparse row placement만 포함한다. |
| 13 | Unsupported gap, overflow, direction, font 또는 payload가 profile에 섞임 | PASS — 기존 repeated-text profile validation을 공통으로 요구하며 oracle manifest와 제한 문서가 grammar를 고정한다. |
| 14 | Automatic minimum을 전체 mask 단위로 계산해 item별 eligibility를 무시 | PASS — runtime test는 각 item의 실제 spanned tracks와 contribution trace로 eligibility를 확인한다. |
| 15 | Flexible track을 span하는 item의 automatic minimum을 잘못 적용 | PASS — trace replay는 해당 intrinsic batch가 flexible track을 가로지르면 fail-closed한다. |
| 16 | Source-order reverse에서 mask bit가 다른 semantic item에 붙음 | PASS — A–E/A–D semantic ID로 mask를 다시 인코딩하며 forward/reverse frame pair를 대조한다. |
| 17 | Span·contribution kind가 서로 다른 phase에 replay | PASS — 각 batch에서 같은 span 및 `ContributionPhase::contribution_kind()`만 전달하고 stage base를 대조한다. |
| 18 | Minimum/content-based minimum이 intrinsic growth limit에서 잘못 멈춤 | PASS — beyond-step distribution은 이 phase에서 성장 한도를 넘길 수 있게 분기하며 mixed sample의 auto floor를 전부 보존했다. |
| 19 | Max-content minimum이 fit-content cap을 넘어 성장 | PASS — 해당 phase는 fit-content cap을 potential로 사용하고 관련 span rounding tests 및 oracle 비교가 통과했다. |
| 20 | Intrinsic/max-content maximum phase에서 limit 또는 무한 growability가 잘못 처리 | PASS — 기존 성장 potential·infinite-growth 처리와 stage trace 대조가 유지되고 six-fit uniform 11,520-scene 회귀가 통과했다. |
| 21 | 여러 item contribution의 track 증가량을 합산해 과성장하거나 마지막 item만 사용 | PASS — 각 item을 같은 base에서 별도 분배한 뒤 track별 최대 planned increase를 적용한다. mixed mask 전 sample frames/rows가 oracle과 일치했다. |
| 22 | 중복·누락 sizing trace, 잘못된 stage 순서 또는 track 수가 부분 결과로 반환 | PASS — owner/row pass/Initialize/track cardinality 불일치는 `None`으로 거부하고 최종 pre-alignment values를 확인한다. |
| 23 | Growth-limit trace 불일치 fallback이 base-size 불일치를 무시 | PASS — growth limit 동기화 전후 모두 각 stage base size를 확인하며 실패 시 row replay는 거부된다. |
| 24 | Gutter, explicit row 종류 또는 일곱 visible row 전제가 달라짐 | PASS — gutter는 0만 허용하고 track 수/explicit intrinsic 여부를 검증한다. |
| 25 | 비유한·음수 pixel 값이 LayoutUnit 변환에서 accepted state로 들어감 | PASS — trace와 percentage limit은 유한·비음수 확인 후 1/64px 단위로 변환한다. |
| 26 | 1/64px ceil 보정이 실제 geometry 차이를 숨기거나 무관한 rounding 경로에 적용 | PASS — 오차 allowance는 f32 epsilon에 제한되고 five-fit mixed에서만 auto-tail residual 예외가 켜진다. Focused rounding cases가 경계를 확인한다. |
| 27 | Growth potential이 subpixel f32 noise에 따라 순서가 바뀜 | PASS — 비교를 LayoutUnit 단위로 양자화하고 기존 auto/later-row tie-break를 유지한다. Five-/six-fit endpoint regressions가 통과했다. |
| 28 | `align-content:stretch`가 여러 row 또는 intrinsic tail을 늘림 | PASS — 특별 보정은 singleton stretched auto tail에 한정하고 다른 row delta 및 합계를 trace와 검사한다. Max-content-tail regression이 통과했다. |
| 29 | Baseline overflow/floor 집계 배열을 auto/zero 반대로 해석 | PASS — 기록 순서는 `[min-height:0,min-height:auto]`; 라벨 수정 후 재실행 값 `[120,0]`, `[2,0]`이 예상과 일치한다. 전체 mixed sample은 floor `[1388,0]`, Grid-end `[4,0]`이다. |
| 30 | 검증 범위·앱 스크린샷이 전체 oracle이나 feature UI 검증으로 과장 | PASS — Chrome 414,720, RuntimeSession mixed sample 3,984, five-fit endpoint 34,560, six-fit endpoint 11,520, #1369 mixed 재실행 11,520/138,240을 분리 기록한다. Android/iOS 화면은 기본 route smoke이고 feature fixture가 아니다. |

검토에서 수정 차단 결함은 발견되지 않았다. #1369 six-fit mixed 전체 138,240 scene 중 이번 branch에서 다시 실행하지 않은 126,720 scene, 일반 CSS/WPT, 실제 앱 화면의 feature fixture 및 표면 pixel parity는 남은 범위다. 이 검토는 Chrome reference·Rust sample 및 bounded profile에 한정하며 C11.4 완료나 일반 CSS 지원을 뜻하지 않는다.
