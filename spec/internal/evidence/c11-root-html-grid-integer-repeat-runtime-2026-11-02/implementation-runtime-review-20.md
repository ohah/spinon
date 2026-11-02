# C11.1 Root HTML 정수 `repeat()` 구현·runtime 적대적 검토 20개

검토 대상은 `RuntimeRootHtmlTextPaintV1`에 정수 repeat expansion을 연결한 cascade 변경, typed track projection, runtime 회귀 입력, 플랫폼 실행 근거다. 계획 검토 20개와 별도로 진행했다. 각 항목은 서로 다른 실패 경로를 확인한다.

| # | 공격 관점 | 코드·실행 대조 및 판정 |
|---:|---|---|
| 1 | 새 허용 flag가 Root HTML 외 cascade profile까지 열리는가 | `incremental/element.rs`에서 `allow_integer_repeat`에 추가한 profile은 `RuntimeRootHtmlTextPaintV1` 하나다. 기존 Grid Template/Track Sizing profile만 기존대로 허용한다. Root HTML runtime positive matrix가 새 entry를 통과한다. |
| 2 | 정수 repeat 허용이 intrinsic sizing 또는 auto-repeat까지 함께 여는가 | 같은 options 생성부에서 `allow_intrinsic_tracks`와 `allow_auto_repeat` 값은 변경하지 않았다. `auto-fill` rejection 및 `min-content` repeated track rejection을 runtime scene 부재까지 확인했다. |
| 3 | adapter가 CSSOM 문자열을 재파싱해 authored 값을 임의 확장하는가 | cascade가 Stylo typed `TrackList`를 받아 기존 `expand_integer_repeats`로 계산한다. Root HTML runtime 테스트는 typed track과 render frame을 따로 대조한다. adapter 문자열 재파싱 코드는 추가되지 않았다. |
| 4 | 반복 결과가 columns/rows 또는 definite-axis basis를 혼동하는가 | Chrome fixture는 repeated px columns와 repeated px rows를 따로 포함한다. repeated percentage와 fraction columns도 200px·150px definite width에서 독립 산술 및 Chrome CSSOM/DOMRect와 일치한다. |
| 5 | 단일 track 반복에서 횟수를 무시하거나 pattern을 중복 확장하는가 | `repeat(1, 30px 50px)` control은 `[30, 50]` track과 두 item frame을 검사한다. |
| 6 | 여러 repeat group 사이 direct track이나 authored 순서가 유실되는가 | `repeat(2, 20px) 10px repeat(2, 30px)`의 typed tracks와 각 item의 누적 frame을 검사한다. |
| 7 | percentage·fraction 혼합 반복에서 고정 폭을 두 번 빼거나 factor 합이 틀리는가 | `20px repeat(2, 20% 1fr)` 입력의 expected used tracks `[20,40,50,40,50]`를 독립 산술, Chrome, runtime과 대조했다. |
| 8 | invalid zero-count repeat가 성공 입력처럼 처리되거나 앞선 cascade 선언을 지우는가 | `repeat(0, 30px)` 뒤에 선언된 invalid declaration을 CSS가 버리고 앞선 direct tracks를 유지하는 control이 전체 geometry를 통과한다. |
| 9 | 정확히 10,000개 track이 경계에서 거부되는가 | Root HTML typed cascade 자체에서 `repeat(10000,1px)`를 실행해 10,000개 값, 10,001 line groups, 전부 빈 metadata를 확인한다. |
| 10 | 10,001개 확장이 잘리거나 부분 scene으로 게시되는가 | runtime rejection matrix의 `repeat(10001,1px)`는 실패 상태, 진단 존재, completed layout 부재를 확인한다. expansion은 checked count와 기존 상한을 사용한다. |
| 11 | named-line byte quota가 track 수 제한만으로 우회되는가 | 10,000회 반복·큰 line name 입력은 1 MiB computed line metadata 한도를 넘겨 실패하며, partial layout/scene을 게시하지 않는다. |
| 12 | track vector 예약 실패가 contextual error 대신 panic이나 빈 정상 결과가 되는가 | `Tracks` failpoint에서 `GridAllocationFailure { node, property: grid-template-columns }`를 확인한 뒤 같은 입력 재계산에서 완전한 track list를 확인한다. |
| 13 | line-group vector 예약 실패 뒤 failpoint가 남거나 partial metadata가 재사용되는가 | `LineGroups` failpoint의 contextual error 뒤 동일 view cascade가 다시 성공하며 전체 5개 line group을 보존한다. |
| 14 | 각 line-name vector 예약 실패가 track-list 오류 문맥을 잃는가 | `LineNames` failpoint에서 node/property를 확인하고, guard 해제 뒤 이름 그룹 순서를 재검증한다. |
| 15 | UTF-8 line-name text 예약 실패 뒤 문자열 일부가 공유되는가 | `LineNameText` failpoint의 contextual error 및 재시도 성공을 확인한다. 단순 integer repeat path에 없는 `FixedRepeatGroups` failpoint는 첫 시험에서 실제 호출되지 않아 최종 matrix에서 제외했다. |
| 16 | named explicit line이 Root HTML layout으로 새어 들어가는가 | runtime rejection은 named-line 입력에서 실패하며 completed scene을 만들지 않는다. lower-level cascade allocation test의 이름 보존은 cascade 내부 테스트이며 app projection 허용을 주장하지 않는다. |
| 17 | Root HTML integer-repeat 허용이 named-area의 빈 explicit track을 scene으로 공개하는가 | `grid-template-areas`와 두 축 `none` 대조군에서 cascade snapshot은 authored empty tracks를 보존하지만 Root HTML projection은 거부한다. layout 실패·completed scene 부재를 확인한다. |
| 18 | semantic selector, tag, ID 또는 NodeId에 반복 결과가 우연히 종속되는가 | 같은 7-scene 입력에서 root tag·class·ID와 selector를 바꾼 control도 typed tracks/frames가 통과한다. reverse source order는 semantic item별 예상 위치로 확인한다. |
| 19 | 스타일과 layout의 다른 revision을 합치거나 rejection 뒤 stale scene을 노출하는가 | positive runtime cases에서 cascade key와 completed layout key가 같다. out-of-profile/over-budget 입력은 completed layout이 없고, 기존 stale new-environment named-area regression도 함께 30-case run에서 통과했다. |
| 20 | Chrome·simulator screenshot smoke가 pixel/readback 검증으로 과장되는가 | fixture hash가 Chrome 입력과 Android/iOS bundled `index.html`에서 같다. Android API 37 AVD는 SwiftShader, iPhone 17 Pro/iOS 26.2는 Simulator Metal이며 양쪽 모두 37 boxes draw/present를 남겼다. screenshots는 시각 smoke 증거만이며 PixelCopy, Metal readback, iOS compositor 이후 픽셀 동일성은 주장하지 않는다. |

## 재검증 결과

- `cargo test --locked -p spinon-runtime root_html_grid -- --nocapture`: 30 passed.
- `cargo test --locked -p spinon-style root_html -- --nocapture`: 13 passed.
- `cargo test --locked -p spinon-style-to-layout root_html -- --nocapture`: 15 passed.
- `cargo clippy --locked -p spinon-runtime -p spinon-style-to-layout -p spinon-style --all-targets -- -D warnings`: 통과.
- `cargo fmt --all -- --check` 및 `git diff --check`: 최종 검증에서 다시 실행한다.

이 검토는 한정된 parsed Root HTML integer-repeat slice만 다룬다. C11.1·C11, 전체 CSS Grid, 공개 WPT 적합성, Android 실기기/hardware GPU, iOS post-compositor surface는 완료 판정하지 않는다.
