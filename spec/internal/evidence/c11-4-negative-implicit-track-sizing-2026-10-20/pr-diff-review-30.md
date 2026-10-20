# C11.4 negative implicit track sizing PR diff review

> This review covers the earlier 106-scene negative-track diff. The current auto-repeat implementation PR diff is reviewed in [implementation-pr-diff-review-30.md](./implementation-pr-diff-review-30.md).

**기준:** `70877fff99306033ae38b2d7e347d34d7b76c429`의 main 기준 diff. 병합 전 PR diff를 서로 겹치지 않는 30개 실패 경로로 검토한다.

| # | 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | 공식 상태·완료 수가 계획/구현 후보만으로 앞서 바뀌는가 | `spec/STATUS.md`를 수정하지 않았고 C11.4 및 C11 완료 수는 그대로다. 내부 계약 문서에도 이 slice가 전체 C11.4 완료를 뜻하지 않는다고 썼다. |
| 2 | Chrome 관찰을 규범 기대값으로 사용하거나 구현에서 기대값을 유도하는가 | 33-case Chrome observation과 W3C 기반 expectation은 별도 자료다. validator가 둘을 구현 결과와 독립 비교하고 결과는 33/33 통과했다. |
| 3 | signed coordinates와 Taffy의 non-negative line 좌표가 중간에 혼합되는가 | 정규화·점유·auto cursor는 signed 좌표로 유지되고 최종 shift를 적용해 Taffy line으로 만든다(`track_sizing.rs:82-445`, `lines.rs:18-34`). |
| 4 | negative CSS line이 explicit grid가 아니라 이미 확장한 grid를 기준으로 해석되는가 | `axis.rs:115-155`는 explicit count만 받아 negative boundary를 계산한다. leading extent는 line 해석 뒤에 결정된다. |
| 5 | numeric line 0 또는 i32 극단값이 wrap/truncate되는가 | 0은 명시 오류다. checked i64 산술, span 변환과 최종 i32 검증이 있고 zero/extreme controls가 있다(`axis.rs:37-155`, `lines.rs:18-34`). |
| 6 | reversed 또는 같은 definite line이 CSS 규칙과 다르게 영역을 만든다 | reversed pair는 정렬하고 같은 boundary는 end auto로 처리한다. 33-case oracle와 Rust reversed-line test에서 검사한다(`axis.rs:27-46`). |
| 7 | line/span 혼합이나 양쪽 span에서 범위 오류가 난다 | span은 positive span 검증 뒤 checked arithmetic으로 boundary에 반영한다. browser cases에는 explicit 경계를 가로지르는 span과 negative definite line 조합이 포함된다(`axis.rs:49-112`). |
| 8 | `grid-template:none`의 단일 anchor가 positive/negative coordinate에 일관되지 않다 | TrackSizing에서 빈 explicit list를 유지하고 signed extent는 `0` boundary에서 양쪽을 계산한다. style/layout tests가 leading 및 trailing frame을 확인한다(`incremental/grid.rs:372-395`, `grid_values.rs:261-270`). |
| 9 | `display:none` child가 implicit extent를 늘리거나 paint node로 남는다 | visible children만 item list에 넣기 전에 세고 배치한다(`track_sizing.rs:47-84`). hidden scene의 root size와 box count가 기대값과 일치한다. |
| 10 | rows/columns 중 한 축의 shift가 다른 축 origin을 잘못 덮는다 | row와 column extent, origin, leading/trailing count와 shift를 각각 계산한다(`track_sizing.rs:82-112,400-445`). 양 축이 서로 다른 pattern인 scene이 Rust와 두 simulator에서 통과했다. |
| 11 | row-flow auto placement가 leading row에서 시작하지 않는다 | row primary origin을 extent start로 두며 fixed/auto axis를 row-flow로 매핑한다(`track_sizing.rs:39-44,263-349`). leading-row fixture가 `y=0`부터 배치된다. |
| 12 | column-flow가 row-flow 대칭 구현을 놓친다 | column primary cursor와 row secondary extent를 별도 매핑한다. column sparse/dense, negative columns/rows scenes가 앱 matrix에 포함된다(`track_sizing.rs:39-44,263-349`). |
| 13 | row-locked/column-locked item cursor가 leading signed coordinate에서 잘못 index된다 | locked cursor 크기와 좌표 index가 같은 signed origin을 사용한다(`track_sizing.rs:162-183`). locked placement의 leading rows/columns가 검사된다. |
| 14 | sparse cursor가 앞선 occupied cell을 재방문하거나 dense cursor가 hole을 놓친다 | sparse는 이전 좌표부터 단조 진행하고 dense는 primary/secondary origin을 되돌린다(`track_sizing.rs:263-349`). paired hole tests가 서로 다른 expected 위치를 명시한다. |
| 15 | auto item이 definite item을 덮거나 item source order가 바뀐다 | 두 축 definite placement를 먼저 occupancy에 기록하고 `placed` set에 넣는다(`track_sizing.rs:125-160`). 같은 source-order fixture를 Chrome와 앱에 대조한다. |
| 16 | 인접 track 경계가 overlap으로 오인되거나 interval list가 정렬되지 않는다 | query는 half-open 조건을 사용하고 삽입 때 겹치거나 인접한 interval만 병합한다(`occupancy.rs:28-48,86-117`). 행별 interval 검색 전제와 대응한다. |
| 17 | line/span 산술 overflow 또는 10,000 초과가 allocate 이후 부분 결과로 노출된다 | checked extent count 및 `track_end`가 cap을 지키고 expanded vector를 reserve하기 전에 최종 수를 검증한다(`extent.rs:29-44,83-105`, `tracks.rs:20-29`). 10,000 cap의 범위는 단위 산술 시험으로 한정해 문서에 표시했다. |
| 18 | allocation failpoint가 잘못된 reserve site를 흉내 내거나 다음 scene에 실패 상태가 누수된다 | six exact reserve sites가 구체 오류를 반환하고 같은 worker의 즉시 재계산이 성공한다(`track_sizing_tests.rs:392-505`). Taffy 내부 전체 OOM 복구로 일반화하지 않았다. |
| 19 | leading `grid-auto-*` cycle이 반대로 순환하거나 0px을 빠뜨린다 | leading pattern은 경계에 인접한 track부터 reverse-index하고 trailing은 authored order로 시작한다(`tracks.rs:51-79`). asymmetric list, zero-size track과 mixed positive/negative tests가 있다. |
| 20 | 새 normalizer가 다른 profile의 positive behavior를 바꾸거나 기존 profile의 empty grid를 허용한다 | TrackSizing 외 profile은 기존 route를 유지하고 empty-list positive control은 오류를 기대한다(`dense_column.rs:32-60`, `grid.rs:384-395,652-665`). |
| 21 | CSS cascade/projection이 `none`, typed track list 또는 line metadata를 잃는다 | `none`은 C11.4 profile에 한정하고 line-name count 검사도 empty/empty pair만 허용한다. end-to-end projection tests가 negative line과 auto list를 확인한다(`incremental/grid.rs:372-395`, `grid_values.rs:261-270`). |
| 22 | intrinsic auto track text가 실제 paragraph sizing 대신 box 크기를 쓰거나 text 최소 크기 실패를 숨긴다 | Ahem을 로드한 negative min/max-content cases가 Chrome observation과 root/item frame을 대조한다. `min-height:0` text measurement 차이는 명시 regression test로 제한한다(`calc_tree/text.rs:57-69`, negative runtime text cases). |
| 23 | TrackSizing auto-repeat container가 새 signed normalizer로 잘못 흘러간다 | auto-repeat route가 먼저 분기해 기존 normalizer를 쓴다. typed `grid-auto-*` list 결합은 오류이며 negative line 교차는 미검증으로 문서화했다(`dense_column.rs:32-60`). |
| 24 | auto-fit empty track 때문에 explicit prefix 쪽 8px boundary gutter가 사라진다 | Chrome 155 별도 boundary/empty-pattern probe와 Rust regression test가 추가됐고 vendored Taffy가 leading prefix gutter를 유지한다(`explicit_grid.rs:333-403`, `grid_track_sizing_tests.rs:537-598`). |
| 25 | 고정 scene payload 또는 FFI symbol이 제품 기본 artifact에 들어간다 | C11.4 fixture는 opt-in `c04-runtime-gpu-fixture-test` route에 연결된다. 기본/positive-control static library marker 검사가 evidence에 기록되어 있다. |
| 26 | Android scene 이름, frame 기대, native sample 대상 사이에 ID mismatch가 있다 | Android route의 33 negative IDs와 expected frame table이 추가됐고 전체 106 AVD scene에서 draw/present가 성공했다. 두 fixed-root overflow scene은 PixelCopy의 다섯 sample 모두 exact다. |
| 27 | iOS route 또는 Metal drawable sample이 Android 정의와 다르다 | Swift scene IDs/frame expectations와 overflow sample table이 대응하며 iPhone 17 Pro iOS 26.2 Simulator 106/106을 기록했다. sample은 drawable readback이다. |
| 28 | runtime trace가 잘리거나 앱이 화면 제출 전에 실패한 scene을 통과로 기록한다 | `result.json`은 scene마다 trace chunk와 draw/present를 보존한다. Android는 544 chunks, superseded retry 1회를 새 generation으로 재실행했고 iOS는 1,033 chunks, retry 0회다. |
| 29 | root edge 표본이나 screenshot을 border stroke/compositor 픽셀 증거로 과장한다 | root edge 좌표는 root background sample이다. CSS border stroke 미구현, AVD software rendering, iOS compositor 전 drawable이라는 제한을 plan/evidence에 적었다. |
| 30 | 전체 테스트 상태와 PR 화면 링크가 불완전하거나 baseline 실패를 감춘다 | workspace Rust는 1,169 passed/0 failed/4 ignored, Clippy/rustfmt와 fixture boundary 6/6 통과다. CSS Reference는 559/563이며 unrelated baseline source-contract 4 failures를 별도 기록한다. PR 본문 Android/iOS raw screenshot URL은 Agent Browser에서 로드 확인 후 유지한다. |

## 발견 및 수정

- 계획 문구의 auto-repeat negative line 처리와 border stroke pixel 조건이 실제 구현 경로보다 강하게 적혀 있어 수정했다.
- 10,000-track 전체 layout success라고 오해할 수 있는 문구를 checked extent cap 시험과 Taffy trace budget 설명으로 분리했다.
- 30개 관점에서는 추가 blocking 결함을 찾지 못했다. 이 표는 PR diff와 코드·고정 oracle·런타임 근거를 함께 본 결과이며, 상위 C11.4/전체 CSS Grid 적합성을 판정하지 않는다.
