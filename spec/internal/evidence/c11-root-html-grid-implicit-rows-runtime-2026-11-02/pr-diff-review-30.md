# C11.1 fixed implicit rows PR diff 실패 관점 검토 30개

대상은 `origin/main` (`f270e43ff19a5a3739e8391cafcbf16d2ffd303d`)부터 PR #1024 HEAD `0e45d69481f3dd388e0a2b6522cf39ad48755347`까지의 최종 diff다. 구현/runtime 검토 20개와 별개로 projection/layout 경계, 오류·상한, 기본 route, 문서·증거를 한 번씩 대조했다.

| # | 실패 관점 | 확인 근거와 판정 |
|---:|---|---|
| 1 | 새 placement mode가 다른 Root HTML Grid에도 적용되는가 | `grid_values.rs`가 parsed Root HTML profile에서 computed `LengthPx` auto row일 때만 새 mode를 선택한다. 기본 `auto`는 기존 mode를 유지한다. 통과. |
| 2 | fixture/C11.3.3 mode를 Root HTML 기본 route가 우회 선택하는가 | `RuntimeRootHtmlTextPaintV1` 분기만 추가됐고 scene ID나 fixture registry dispatch는 없다. 기본 runtime integration이 `new_runtime_gpu_with_root_html()`로 실행된다. 통과. |
| 3 | 기존 auto-row 경로의 projection 입력이 바뀌는가 | auto row projection은 sparse row, dense placement, 새 Root mode로만 분기한다. 기존 Root HTML의 computed `auto`는 경로 밖이다. Root HTML runtime 회귀 34/34. 통과. |
| 4 | computed auto-row list를 첫 값만 취해 허용하는가 | projection은 단일 `LengthPx`만 Root HTML implicit mode로 인정한다. 다중 list, `%`, `fr`, `minmax()` rejection controls가 layout 실패 및 scene 미발행을 확인한다. 통과. |
| 5 | NaN, 무한대, 음수 auto-row가 core에 직접 들어올 때 유효한가 | projection과 core contract 모두 finite non-negative px를 요구한다. zero 경계는 runtime에서 따로 확인한다. 통과. |
| 6 | 새 mode와 named-area template가 합쳐져 영역 profile을 넓히는가 | projection에서 implicit row + `template_areas`를 거부하고 core contract도 areas를 거부한다. dedicated projection regression이 `grid-auto-rows` 오류를 확인한다. 통과. |
| 7 | implicit-row 조합에서 direct percentage 또는 `fr` explicit track이 조용히 계산되는가 | projection은 새 mode에서 explicit columns/rows 전체를 fixed px로 제한한다. `%`와 `fr` negative controls 및 percentage projection test가 각각 거부를 확인한다. 통과. |
| 8 | repeat 또는 named line metadata가 이름을 버리고 평탄화되는가 | projection과 core는 auto-repeat 및 비어 있지 않은 line-name metadata를 거부한다. 통과. |
| 9 | implicit columns가 row 전용 normalizer에 전달되는가 | `grid-auto-columns:auto`만 허용하며 non-auto columns control은 projection에서 거부된다. 통과. |
| 10 | column 또는 dense flow가 sparse row semantics로 오인되는가 | computed auto-flow는 `row`만 허용하고 core에서도 정확히 검사한다. column/dense controls가 실패한다. 통과. |
| 11 | track axis가 비거나 invalid한 채 나눗셈·normalizer에 도달하는가 | core contract가 빈 explicit axis와 finite/non-negative가 아닌 px track을 먼저 거부하고, item preflight도 column count 0을 방어한다. 통과. |
| 12 | 행 배치 전에 definite width/height basis 검사가 우회되는가 | `validate_grid_layout`의 기존 definite-axis 검사를 새 mode도 그대로 거친다. mode 추가는 indefinite basis 예외를 만들지 않는다. 통과. |
| 13 | percentage/음수/비정상 gap이 implicit track 계산에 섞이는가 | 새 mode의 container-style validator가 두 gap 모두 finite non-negative `LengthPx`인지 검사한다. 통과. |
| 14 | 비기본 alignment가 track geometry를 바꾸는데 성공 처리되는가 | container의 `align-items`, `align-content`, `justify-content`, `justify-items`를 좁은 초기값으로 제한하고 Root HTML control이 실패를 확인한다. 통과. |
| 15 | clipping/scroll overflow가 implicit row 결과에 섞이는가 | container와 item 양쪽 overflow가 visible이어야 하며 hidden overflow negative control은 성공 scene을 만들지 않는다. 통과. |
| 16 | 숨긴 직접 자식이 occupancy, item 수, 행 상한에 포함되는가 | 공통 `hidden_subtrees` 결과를 preflight와 sparse normalizer가 사용한다. positive runtime control에서 hidden orange item은 scene에 없다. 통과. |
| 17 | nonzero `order`를 DOM source order와 혼동해 지원한다고 처리하는가 | visible item의 order를 projection/core에서 검사한다. 20개 구현 검토의 order rejection 및 source reorder positive control과 일치한다. 통과. |
| 18 | authored line/span placement가 auto-placement와 혼합되는가 | 새 mode preflight는 네 placement 값이 모두 auto인지 요구한다. numeric/named placement controls가 scene publish 전에 실패한다. 통과. |
| 19 | nested 또는 text item이 빈 block item으로 취급되는가 | core preflight는 자식과 text content를 거부하고 item-shape validator가 같은 제약을 재검사한다. text/nested runtime controls는 layout 실패다. 통과. |
| 20 | auto-placement visible child가 fixed-size empty block 조건을 벗어나는가 | item validator가 block display, 유한 non-negative fixed width/height, min/max/aspect-ratio, margin/padding/border, align-self와 position을 확인한다. 통과. |
| 21 | 직접 만든 Grid item이 container가 아닌 노드로 잘못 검증되는가 | 기존 Grid input의 node-ID ownership 검사가 먼저 실행되며 새 mode 검사는 실제 `container_node.children`만 순회한다. 통과. |
| 22 | visible item 수 계산이 `usize` 덧셈 overflow를 일으키는가 | preflight가 `checked_add(column_count - 1)`를 사용하고 `column_count == 0`을 먼저 거부한다. overflow는 명시 오류로 반환한다. 통과. |
| 23 | implicit row 요구 수가 resource cap을 넘는데 Taffy track 확장부터 하는가 | placement preflight가 최소 필요 행을 `GRID_MAX_TRACKS`와 비교한다. 10,001-row 전용 layout test는 expansion 전 `GridImplicitTrackRequired`를 확인한다. 통과. |
| 24 | 전체 row cap은 통과해도 Root HTML item capacity가 explicit cells로만 제한되는가 | projection capacity는 안전한 multiplication으로 제한 implicit profile의 columns × `GRID_MAX_TRACKS`를 사용하고, 다른 profile은 기존 explicit capacity를 유지한다. 통과. |
| 25 | explicit row 뒤의 첫 implicit row가 0부터 시작하거나 row gap을 잃는가 | 기존 sparse placement와 track expansion 경로를 재사용한다. exact frame은 첫 implicit item y=25, 반복 row y=45 및 row-gap 5px를 확인한다. 통과. |
| 26 | 0px auto row가 누락되거나 양수로 바뀌는가 | zero boundary runtime test가 root height 25 및 item frame `(0,25,40,0)`을 검사한다. 통과. |
| 27 | source 순서 대신 ID/class 또는 명시 `row` 선언 유무가 결과를 정하는가 | runtime test는 ID/class rename, 명시 기본 `row`, DOM order 변경을 서로 비교한다. rename 결과는 동일하고 source reorder는 기대한 cell만 바꾼다. 통과. |
| 28 | unsupported 입력 오류 후 앞선/부분 scene이 성공 결과로 남는가 | 17개 runtime rejection control이 `Failed`, 오류 보유, `completed=None`을 확인한다. stale document revision guard도 별도 1/1이다. 통과. |
| 29 | CSS oracle 값 또는 화면 캡처가 exact layout/pixel 근거로 과장되는가 | geometry는 Chrome 156 precomparison JSON과 Rust exact frame assertions로 비교한다. AVD/Simulator 화면은 시각 및 draw/present 근거로만 기록하고 surface readback·pixel equality는 명시적으로 제외한다. 통과. |
| 30 | 계획·명세·상태·roadmap 또는 PR 이미지가 제한 범위를 다르게 약속하는가 | 내부 계약 0059, C11 계획, `STATUS.md`, roadmap과 PR 본문은 candidate로 표현하고 상위 C11 체크 및 공식 `72/213`을 유지한다. Agent Browser에서 PR 본문의 두 raw screenshot을 각각 complete 및 natural size 1080×2400, 1206×2622로 확인했다. 통과. |

## 결과

이 diff review에서 수정이 필요한 새 결함을 찾지 못했다. review 시점 GitHub PR #1024는 open/mergeable이고 check rollup은 비어 있었다. CI 통과로 세지 않으며, C11 전체·C11.1 전체·일반 CSS admission·WPT·실기기/hardware GPU는 완료로 표시하지 않는다.
