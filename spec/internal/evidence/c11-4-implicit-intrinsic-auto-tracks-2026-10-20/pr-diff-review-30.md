# C11.4 implicit intrinsic auto-track PR diff 적대 검토 30개

이 표는 최종 implementation diff를 별도의 30개 실패 관점으로 검토한다. 계획 review와 [구현/runtime 20관점 검토](implementation-review-20.md)는 다시 쓰지 않았다. 30개 관점을 통과했으며, auto-repeat 조합 거부 및 implicit locked cursor 결함은 구현 review 중 발견해 최종 diff에 수정했다.

| # | diff 실패 관점 | 검토 근거와 판정 |
| --- | --- | --- |
| 1 | 지원 범위가 새 JavaScript/C API처럼 노출됨 | 변경은 typed Rust layout input·cascade snapshot 및 opt-in native fixture에 한정된다. 새 JS/C entrypoint는 없고 상위 CSS profile 표시도 바꾸지 않았다. |
| 2 | 내부 계약 버전이 무단으로 올라감 | 계약 `0059` 자체는 수정하지 않았으며 계획은 `0.1.0` 유지 경계를 명시한다. |
| 3 | 기존 exported layout type의 Rust 소스 영향이 숨겨짐 | `GridAutoTrackSize`에 `TrackList(Vec<GridTrackSize>)`가 추가된다. 이는 workspace Rust type 확장이고 JS/C API 증가는 아니며, 기존 `Auto`·`LengthPx` 생성 형태는 그대로 둔다. |
| 4 | 이전 profile의 auto-track 문법이 넓어짐 | cascade와 projection에서 TrackList를 C11.4 profile로 제한하고 layout도 `TrackSizing` 이외에서 거부한다. 관련 profile regression tests 통과. |
| 5 | 단일 고정 px 경로 회귀 | 단일 px computed value는 `LengthPx`로 보존한다. C11.2.2/C11.2.3 fixed subset 검사를 통과한다. |
| 6 | 목록 순서가 cascade에서 손실 | snapshot은 typed vector 순서를 보존한다. 양 축 목록 cascade test와 expected snapshot 비교가 통과한다. |
| 7 | rows/columns 데이터가 교차됨 | row converter는 `GridTemplateRows`, column converter는 `GridTemplateColumns` math property를 쓴다. projection math test가 두 결과 ID를 검증한다. |
| 8 | CSS converter에서 지원하지 않는 syntax가 조용히 축약 | `track_size(size, true)`가 표현할 수 없는 entry를 serialized property value로 fail-closed 처리한다. |
| 9 | CSS math ID가 축에 잘못 연결 | 기존 typed math projector와 property context를 재사용하고 calc row/column을 각각 검사한다. |
| 10 | cascade/projection allocation 오류 문맥 누락 | auto rows/columns 각각 failpoint를 두고 `GridAllocationFailure`의 NodeId/property를 검사한다. 5 cascade, 3 projection 집중 테스트 통과. |
| 11 | empty, NaN, negative 또는 cap 초과 목록이 실행까지 도달 | layout 검증에서 empty/잘못된 크기와 10,001 목록을 거부하고 CSS cascade의 oversized value는 serialized unsupported 진단으로 차단한다. |
| 12 | template track validator에 새 크기 종류가 빠짐 | `validate_tracks`는 C11.4 intrinsic type만 허용하고 min/max function 하위 값도 검증한다. 이전 profile에는 허용되지 않는다. |
| 13 | 양의 implicit endpoint가 잘못된 0-based index로 변환 | CSS line을 1-based로 유지한 뒤 line-1을 track index로 변환한다. item line 2/3 row·column frame이 Chrome 및 두 앱에서 맞는다. |
| 14 | negative implicit line이 positive extension에 섞임 | 음수 endpoint는 explicit-line resolver로만 이동하고 범위 밖이면 `InvalidGridPlacement`를 반환한다. `-3 / -2` 회귀 통과. |
| 15 | span이 explicit/implicit 경계에서 track list offset을 변경 | `[12px]` explicit 뒤 span 2개가 `[10px,20px]`을 받는 결과와 42px frame을 Rust test에서 검사한다. |
| 16 | 목록 길이와 무관하게 한 값만 반복 | 패턴 index는 implicit index를 목록 길이로 나눈 나머지로 계산한다. 3개 positive implicit track에서 `[10,20,10]`을 확인한다. |
| 17 | row flow에서 column flow용 축 count를 씀 | auto-flow 축을 따라 secondary span을 계산하고 final expansion에 각 축 property를 따로 넘긴다. row·column matrix와 runtime scene 모두 일치한다. |
| 18 | positive definite placement가 locked cursor보다 긴 축을 만듦 | definite positive start/span을 placement step 2 전에 미리 count한다. 초기 review에서 발견한 cursor OOB를 수정했고 두 축 locked-auto regressions 통과. |
| 19 | auto-placement overlap 또는 hidden child가 track을 추가 | definite occupancy를 먼저 구성하고 hidden subtree를 visible item list에서 뺀다. hidden-positive child는 확장을 만들지 않는다. |
| 20 | auto-repeat 경로가 목록을 무시해 기본 `auto`처럼 표시 | C11.4 auto-repeat 분기에서 rows/columns TrackList를 명시 거부한다. 범위 밖 composition을 성공으로 위장하지 않는다. |
| 21 | 확장 vector 예약 실패가 부분 output을 게시 | rows 또는 columns expansion 중 실패하면 local normalized candidate가 반환되지 않는다. `ExpandedRows` failpoint 뒤 동일 engine 재계산도 통과한다. |
| 22 | track cap이 positive line, span, pattern 확장에서 우회 | `track_end`, `validate_count`, `validate_tracks` 모두 10,000 제한을 확인한다. 긴 목록/invalid/empty layout controls 통과. |
| 23 | auto track intrinsic sizing이 text measure와 stretched item height를 충돌시킴 | text mismatch는 measured text가 final content보다 클 때만 발생하게 바꾸고, narrow/wide track fixture 및 workspace 단위 검증으로 확인한다. |
| 24 | text error 진단이 이전 의미로 남음 | error 출력 문구가 “content height가 text measure보다 작음”으로 맞춰져 기존 equality 오류처럼 오해되지 않는다. |
| 25 | fixture source code가 runtime product에 상시 포함 | FFI fixture entrypoint는 `c04-runtime-gpu-fixture-test`, runtime text fixture는 `c11-4-grid-track-sizing-fixture-test` gate 아래다. 실제 기본 `.a`는 C11.4 symbol·20개 scene ID가 0개이고 opt-in `.a`에는 3개 symbol·20개 ID가 모두 있음을 확인했다. [artifact 기록](ffi-artifact-boundary.md). |
| 26 | FFI fixture에 임의 scene 또는 ID collision | 새 20 scene의 정적 allowlist는 개수·중복 ID·source byte cap·Ahem·node count를 검사하고 unknown scene을 반환하지 않는다. opt-in test 1/1 통과. |
| 27 | V8→Stylo runtime에서 다른 DOM/text로 계산 | runtime adapter는 scene ID, root tag/style/size, grid track counts, direct child identity/order, text node ownership/content/style 및 Ahem font를 확인한다. |
| 28 | Android/iOS host에서 기대값 또는 scene route 불일치 | 두 host의 scene allowlist, root box count/frame, overflow samples를 추가했고 73 scene 양쪽 runtime 결과가 ready/draw-present로 끝났다. |
| 29 | oracle/surface screenshot이 잘못된 source나 stale run을 가리킴 | Chrome 입력/관찰/화면/규범 기대값 SHA 6개를 검증했다. Android logcat은 앱 PID와 최근 1,000줄만 읽으며, 최종 run ID가 각 log와 result에 일치한다. |
| 30 | 문서가 실기기·WPT·완료 수를 과장하거나 PR image를 제공하지 않음 | evidence는 Android API 37 AVD와 iOS 26.2 Simulator라고 한정한다. 5+5 surface 표본은 OS compositor 전이며 WPT·negative implicit·full auto-repeat을 미완료로 남긴다. 공식 STATUS/count 및 Tailnet aggregate는 merge/docs gate 전까지 그대로 둔다. |

## 검토 결론

마지막 diff check와 포맷·lint·집중 테스트를 통과했다. `cargo test --locked --workspace`의 기존 auto-repeat neighbor 한 건과 전체 CSS-reference의 변경 외 4개 source-shape 실패는 별도 기준 실패로 기록했고 이 diff의 green CI로 표시하지 않는다. 최신 시뮬레이터 APK/app hash 및 73/73 실행은 [구현 evidence](README.md)에 연결했다.
