# C11.4 `grid-auto-rows:auto` stretch 구현 PR diff 실패 관점 검토 30개

대상: 구현 candidate의 staged source, fixture, contract/status/plan 문서와 simulator evidence diff. 구현/runtime 20개 검토와는 별도로 PR의 전체 변경 경계를 점검했다. 관점마다 실패 경로를 달리해 실제 변경 내용을 대조했다.

| # | PR diff에서 살핀 실패 경로 | 판정 |
|---:|---|---|
| 1 | 기존 placement mode를 수정해 예전 intrinsic row 경로가 달라짐 | PASS — 새 `RootHtmlAutoRowStretchV1` variant를 추가하고 legacy auto-row rejection 회귀를 둔다. |
| 2 | 새 mode 판별 helper가 다른 mode를 intrinsic sizing으로 오인 | PASS — 두 Root HTML intrinsic mode만 포함한다. |
| 3 | auto row admission이 모든 `RootHtmlIntrinsicTrackSizingV1` 입력에 확장 | PASS — normalizer의 `GridAutoRows::Auto` arm은 새 mode에만 열려 있다. |
| 4 | 직접 explicit track list 확장이 implicit auto row를 숨김 | PASS — 새 mode에서는 `expanded_rows`를 만들지 않아 Taffy가 implicit track을 생성한다. |
| 5 | fixed/track-list implicit row 처리의 reservation·overflow 검사가 변함 | PASS — 기존 확장 branch와 `RootHtmlImplicitRowsV1` extent 검증 순서를 보존한다. |
| 6 | grid line normalization이 새 mode를 건너뛰거나 placement 오류를 놓침 | PASS — 새 mode를 기존 limited Root HTML row placement validation에 연결한다. |
| 7 | intrinsic measurement callback에서 text contribution이 사라짐 | PASS — calc tree는 두 intrinsic mode의 child를 sizing 대상에 포함한다. |
| 8 | normal justification 변환이 새 profile에서 달라짐 | PASS — 기존 intrinsic route helper에서만 special `normal` 변환을 유지한다. |
| 9 | Taffy style adapter가 auto rows를 다른 CSS track으로 변환 | PASS — auto row를 보존하고 sizing 엔진이 implicit `auto`를 직접 처리한다. |
| 10 | `GridAutoColumns` 변경이 새 mode에 불필요하게 열림 | PASS — projection candidate는 `auto` 외 값을 막고 implicit columns 없는 2-item 입력만 허용한다. |
| 11 | auto flow가 column/dense로 바뀌며 row 결과가 달라짐 | PASS — computed grid candidate는 row sparse만 받고 기존 row-flow validator도 적용한다. |
| 12 | root computed track geometry가 계획의 고정 기준과 달라짐 | PASS — candidate는 정확히 40px column과 10px row 한 개를 요구한다. |
| 13 | repeat/area metadata가 track count를 확장 | PASS — repeat group, auto-repeat 및 template-area metadata를 candidate에서 배제한다. |
| 14 | `align-content` 별칭/기본값 처리로 stretch 조건이 넓어짐 | PASS — computed `normal`을 요구하고 `start` 부정 실행을 확인했다. |
| 15 | `align-items`와 `align-self`의 역할이 섞임 | PASS — Grid 기본값 `normal`과 두 번째 item의 authored/computed `auto`를 분리해 검사한다. |
| 16 | `justify-content`/`justify-items` 조건을 생략해 다른 분배를 받음 | PASS — element candidate가 두 값도 `normal`로 제한한다. |
| 17 | fixed item intrinsic size가 바뀌어 explicit row base가 달라짐 | PASS — 첫 item의 고정 40×10px와 auto min/max, static position을 검사한다. |
| 18 | source order에서 text가 첫 item이 되거나 extra item이 두 번째가 됨 | PASS — source order 및 정확히 2 visible direct items를 검사한다. |
| 19 | Grid owner 밖 element/추가 owner가 성공 route에 들어옴 | PASS — computed Grid candidate 수와 runtime-visible Grid owner 수를 모두 제한한다. |
| 20 | nested/anonymous text가 기존 text grammar보다 넓게 처리됨 | PASS — 단일 direct Text child와 기존 ASCII token/whitespace gate를 재사용한다. |
| 21 | text style 변경으로 두 줄 minimum contribution이 변함 | PASS — 10px monospace/line-height 및 normal wrapping/LTR profile 검사를 유지한다. |
| 22 | aspect-ratio/position/size constraints가 row stretch 입력을 왜곡 | PASS — root와 items에서 ratio 미설정, static, fixed/auto min/max 값을 확인한다. |
| 23 | hidden text를 visible item count에 포함하거나 제외 경계가 잘못됨 | PASS — 기존 hidden-node skip을 유지하고 visible direct items를 별도 센다. |
| 24 | style capacity 계산이 implicit row 1개보다 작게 projection을 막음 | PASS — strict candidate에 대해 row-capacity 경로가 기존 track ceiling까지 계산한다. |
| 25 | Style-to-layout mode override 뒤 다른 route가 이를 다시 덮음 | PASS — mode 선택, intrinsic sizing node set 및 이후 projection 순서를 읽어 추적했다. |
| 26 | 실패한 입력이 이전 revision frame을 새 layout으로 게시 | PASS — rejection assertion은 `completed=None`을 확인하고 같은 session 정상 재요청도 성공한다. |
| 27 | trace가 최종 frame만 기록하고 auto-track stretch 단계를 숨김 | PASS — opt-in trace에서 row bases `[10,20]`→`[10,30]`, used rows와 1/1 explicit/implicit count를 검사한다. |
| 28 | oracle 기대값이 브라우저 fixture가 아닌 다른 source bytes를 설명 | PASS — Chrome/expected JSON, fixture HTML 및 양 simulator app의 fixture SHA가 동일하다. |
| 29 | 임시 build script가 저장소의 원본 bootstrap 파일에 남아 있음 | PASS — source·HEAD·backup hashes가 같고 복구 log가 보존되어 있다. |
| 30 | test fixture가 release/runtime artifact에 들어가거나 screenshot을 pixel proof로 과장 | PASS — 기본 release `.rlib`에서 fixture marker가 빠지고 test executable positive control에는 있다. Simulator screenshots는 visual smoke로만 설명한다. |

검토 결론: 변경 diff에서 수정이 필요한 결함을 찾지 못했다. 제품 artifact scan은 Rust release `.rlib` 경계 확인이며 최종 Android/iOS release bundle이나 일반 CSS/WPT conformance를 뜻하지 않는다. Raw Xcode build logs에 원래 출력된 trailing spaces와 마지막 빈 줄은 로그 증거를 변형하지 않도록 보존했다. 이 whitespace는 Rust source diff에서 나온 것이 아니다.
