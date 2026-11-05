# Parsed Root HTML Grid auto-repeat 계획 PR diff 적대 검토 30개 · 2026-11-05

**대상 PR:** [#1178](https://github.com/ohah/spinon/pull/1178) · 최종 substantive diff 기준 `1669d790a` (`origin/main` 대비 12개 파일).
**범위:** 계획, C11 계획 목록 연결, 20관점 계획 검토, 고정 HTML·독립 expected values·Agent Browser 관측/화면·비교 도구와 hashes. Product Rust/app 코드는 변경하지 않았다.

| # | 별도 실패 관점 | 확인한 변경/방어 | 판정 |
|---:|---|---|---|
| 1 | 이미 병합된 C11.4 auto-repeat 작업을 새 Root HTML 기능으로 중복 보고 | 기존 C11.4 typed track-sizing profile과 기본 `RuntimeRootHtmlTextPaintV1` rejection을 구분했다. 전용 Root HTML auto-repeat 계획은 없음을 현재 plan/code search에서 확인했다. | 통과 |
| 2 | 계획 후보를 구현 완료로 표현 | 계획, C11 master index, PR 본문 모두 plan-only 및 미구현이라고 표시했다. | 통과 |
| 3 | C11/C11.1/C11.4 상태 checkbox 또는 공개 CSS 지원을 앞당김 | 계획 PR은 `spec/STATUS.md`를 수정하지 않고 제품/API 완료도 선언하지 않는다. | 통과 |
| 4 | 공식 count가 이 날짜에 맞지 않음 | `spec/README.md`의 현재 `78/214`를 대조해 전용 계획과 C11 master index에 기록했다. | 통과 |
| 5 | 신규 계획이 C11 master index에서 발견되지 않음 | `plan/c11-css-grid.md` 상단의 2026-11-05 후보 항목이 전용 계획으로 연결된다. | 통과 |
| 6 | 기존 내부 계약이 이미 auto-repeat을 허용한다고 잘못 설명 | 0059 C11 계약의 integer `repeat()` 허용 및 Root HTML auto-repeat 거부 범위를 실제 본문과 대조했다. | 통과 |
| 7 | Taffy 변환 지원을 Root HTML runtime 지원으로 확대 해석 | 공용 converter 존재와 profile별 admission/rejection을 별개의 코드 경계로 썼다. | 통과 |
| 8 | W3C의 다른 draft/revision을 normative basis로 연결 | Agent Browser에서 지정한 2025 CRD §§7.2.3.2, 11.7, 11.7.1 heading과 내용이 일치함을 확인했다. | 통과 |
| 9 | 169/170px repeat boundary 산술 오류 | 40px+30px 최소 pattern 두 번과 gutter 세 개가 170px임을 계산과 양 축 fixture로 확인했다. | 통과 |
| 10 | `fr` 크기를 최소 base에 free-space를 더해 잘못 계산 | 잘못된 최초 expected artifact를 보존하고, §11.7.1 freeze/restart 규칙을 반영한 corrected expected values와 Chrome 156 관측을 대조했다. | 통과 |
| 11 | UA 1px repeat-count floor를 실제 used track minimum에 적용 | 0.5px minimum·2px axis positive control은 count 두 번과 실제 1px track 두 개를 구분한다. | 통과 |
| 12 | columns와 rows에서 반복 축 또는 frame origin이 뒤바뀜 | fixture에 columns/rows 별도 scene이 있고 comparator는 각 track axis 및 root-relative item frames를 검증한다. | 통과 |
| 13 | empty auto-fit에서 반복 track/gutter가 남음 | columns·rows 빈 grid가 모두 있고 CSSOM은 네 0px tracks를 반환한다. | 통과 |
| 14 | hidden child가 auto-fit occupancy/item capacity에 들어감 | hidden case에서 computed display, zero frame, visible item count를 분리 검사한다. | 통과 |
| 15 | auto-fill의 비점유 repeated track이 collapse | auto-fill 두-repeat cases가 네 computed tracks의 크기를 모두 비교한다. | 통과 |
| 16 | `auto-fit` occupied track이 남은 폭을 잘못 나눔 | 두 occupied tracks 95px/95px와 단일 occupied track 200px 기대값을 별도 둔다. | 통과 |
| 17 | repeat fragment 앞뒤 tracks, named lines 또는 areas가 제한 grammar에 유입 | 계획은 fragment 전체-list 조건을 요구하고 prefix/suffix/name/areas를 거부한다. | 통과 |
| 18 | percentage/calc/intrinsic minimum이나 indefinite axis에서 repeat count 계산 | fixed px min, fixed px gap, definite positive px repeat axis만 계획의 admission 조건이다. | 통과 |
| 19 | padding/border/margin으로 count basis와 content box가 다름 | zero-edge 조건과 axis-specific basis validation을 계획 및 negative tests에 넣었다. | 통과 |
| 20 | integer repeat, 동일 축 복수 repeat 또는 양 축 repeat 조합을 허용 | 한 auto-repeat total, 한 axis, 한 fragment만 허용하도록 계획했다. | 통과 |
| 21 | 반복 확장 후 실제 capacity 대신 authored track count를 사용 | expanded repeat count 기반 capacity와 no-implicit-track refusal을 구현 gate로 명시했다. | 통과 |
| 22 | 확장 결과가 Taffy의 minimum-one, UA floor 또는 10,000-track cap과 불일치 | 위 grammar로 제한한 count formula를 Taffy 경계·`GRID_MAX_TRACKS`와 대조하도록 계획했다. | 통과 |
| 23 | NaN/overflow/oversized item count가 capacity arithmetic을 통과 | overflow-safe 산술, finite 검사, over-cap/item-capacity negative controls 및 no partial scene 조건을 요구한다. | 통과 |
| 24 | C11.4 profile admission을 넓혀 기존 behavior를 바꿈 | 기존 track-sizing profile은 그대로 두고 Root HTML validator가 승인한 입력만 별도 허용하도록 한다. | 통과 |
| 25 | app 검증이 Rust unit test나 C11.4 opt-in route만 실행 | Android/iOS에서 `new_runtime_gpu_with_root_html` 기본 Root HTML profile을 사용하고 C11.4 route로 대체하지 않도록 계획을 수정했다. | 통과 |
| 26 | simulator에 fixture 입력을 넣을 수 없어 계획된 검증을 실행하지 못함 | 기존 Root HTML SurfaceView/Metal harness 사용을 우선하고 필요 시 전용 fixture-test feature/build define을 추가하도록 범위를 구체화했다. | 통과 |
| 27 | 새 fixture adapter가 기본 artifact에 포함됨 | 신규 harness가 필요할 때 default APK/iOS executable negative control과 fixture build positive control을 검증하도록 했다. | 통과 |
| 28 | visible overflow가 root boundary에서 clamp/clip됨 | 170px root 안의 240px item을 두고 70px overflow frame과 화면을 독립 비교했다. | 통과 |
| 29 | Chrome observation/comparison metadata 또는 hashes가 서로 다름 | 13-case comparator pass, fixture/expected/observation/comparison/script/failure/screenshot SHA-256을 재계산해 README와 대조했다. | 통과 |
| 30 | PR screenshot/link/check/merge evidence가 잘못되거나 product verification을 암시 | Agent Browser에서 PR 이미지가 naturalWidth 1280으로 로드됨과 branch-path 문서 링크 접근을 확인했다. PR check rollup은 비어 있고 제품/runtime/simulator 수행 주장은 없다. | 통과 |

## 판정

계획 PR diff의 substantive 변경 30/30 관점 통과. 계획 PR이므로 Rust tests, app build, Android AVD/iOS Simulator runtime은 실행하지 않았다. Plan review 20개는 별도 파일에 있고 이 30개는 PR diff 자체를 확인한다. 구현/runtime review 20개 및 implementation PR diff review 30개는 다음 구현 PR에서 새로 수행한다.
