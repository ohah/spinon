# C11.4 PR #375 후속 roadmap 문서 diff 검토 30관점

범위는 PR #375 병합 뒤 상태·계약·계획·roadmap source에 추가한 5개 문서 변경이다. 코드를 다시 검증하는 대신 수치, 의미, 경로, 플랫폼 경계와 후속 동기화 절차를 각각 따로 확인했다. PR #375 구현 diff의 30관점 검토는 [별도 기록](./pr-diff-review-30.md)에 있다.

| # | 실패 가능성 | 확인 결과 |
| ---: | --- | --- |
| 1 | 문서가 merge SHA를 PR commit SHA와 혼동 | GitHub PR #375의 `mergeCommit` `e01f0dfc386e5d77ad0cdf8fb3e3b11df40696f8`과 일치한다. |
| 2 | 병합 사실을 main 기반이 아닌 feature HEAD로만 기록 | PR 상태가 `MERGED`; `origin/main`은 merge commit을 포함한다. |
| 3 | 부분 구현을 C11.4 완료로 승격 | STATUS의 C11.4 `[ ]`를 유지하고 부분 slice라고 적었다. |
| 4 | C11 상위 체크가 함께 바뀜 | STATUS C11 상위도 `[ ]` 그대로다. |
| 5 | public roadmap이 공식 완료 수를 수동 증가 | 변경은 note 문단뿐이며 checkbox/count script는 수정하지 않았다. |
| 6 | roadmap 설명이 전체 implicit track 지원을 뜻함 | `RuntimeBlockFlexGridTrackSizingV1`과 positive implicit rows/columns로 범위를 한정했다. |
| 7 | `grid-auto-*` authored order나 cyclic semantics가 누락 | 내부 계약은 축별 `auto_tracks[(i - explicit_track_count) % len]` 규칙을 정의한다. |
| 8 | 명시적 track prefix가 목록 index에 포함되는지 모호 | internal contract는 normalized axis index와 `i >= explicit_track_count`를 적는다. |
| 9 | negative implicit line이 지원되는 것처럼 읽힘 | 상태·계약·roadmap에서 negative implicit은 미완료/거부로 남겼다. |
| 10 | auto-repeat과 track list가 같이 성공한다고 오해 | 해당 조합은 fail-closed이며 별도 미완료로 적었다. |
| 11 | nested/anonymous/out-of-flow item을 보장한다고 오해 | internal contract와 계획이 이 조합을 범위 밖으로 둔다. |
| 12 | Chrome oracle case를 WPT pass로 표시 | Chrome 155 20/20은 reference 비교로만 적고 WPT는 미완료다. |
| 13 | Android AVD를 실기기 검증으로 부풀림 | API 37 ARM64 AVD라고 적었으며 물리기기 주장은 없다. |
| 14 | iOS Simulator를 iOS 실기기와 혼동 | iPhone 17 Pro iOS 26.2 Simulator라고 명시했다. |
| 15 | Metal drawable 표본을 compositor 결과로 오해 | compositor 제출 전 표본이라고 제한했다. |
| 16 | 5/5 pixel 표본을 전체 장면 픽셀 동등성으로 확대 | border overflow의 고정 표본 결과라고만 적었다. |
| 17 | AVD software renderer를 hardware GPU 근거로 오해 | roadmap은 hardware GPU 보장을 하지 않고 실행 근거도 software AVD 경계를 기록한다. |
| 18 | default FFI artifact 검사로 최종 앱 bundle 크기를 주장 | 문서 링크는 static library symbol/string 분리 검사이며 앱 크기 주장은 없다. |
| 19 | fixture FFI가 제품 ABI로 노출됨 | artifact 기록은 기본 0/20·0 symbol, opt-in positive control 20/20·3 symbol이다. |
| 20 | 내부 계약 숫자 버전이 작업마다 상승 | `0.1.0` 그대로라고 기록했다. |
| 21 | 새 public API나 CSS 지원 표가 생긴 것처럼 표시 | public roadmap은 내부 profile slice로 설명하며 공개 API/지원 선언을 추가하지 않았다. |
| 22 | GitHub required check이 녹색이었다고 오해 | repository protection/ruleset과 PR checks를 확인했으며 필수 status check 미설정을 명시했다. |
| 23 | 기존 workspace 실패를 PR 변경 실패로 숨김 | expected 154/actual 162 baseline 실패와 구현 전 재현을 evidence에 연결했다. |
| 24 | CSS source-shape 검사 실패를 새 C11.4 회귀로 표시 | 변경 파일 밖의 기존 4개 정적 검사 실패라고 evidence에서 분리했다. |
| 25 | internal index의 상대 링크가 저장소 root를 벗어남 | `spec/internal/README.md`에서 `../../plan`과 `./evidence`가 실제 파일을 가리킨다. |
| 26 | Rspress HTML route 링크가 틀림 | roadmap은 `/spinon/plan/...html`, `/spinon/internal/...html`, evidence `README.html` route를 사용한다. |
| 27 | contract fragment anchor가 일치하지 않음 | Agent Browser 사전 확인에서 `grid-auto-*`의 `*`가 heading id에 추가 하이픈으로 반영된 점을 찾아 roadmap fragment를 실제 DOM id `c114-positive-implicit-grid-auto--track-size-lists`에 맞췄다. Tailnet pass17 Agent Browser에서 해당 fragment의 `anchor_found=true`를 확인했다. |
| 28 | 계획/상태/evidence가 merge 상태에서 서로 어긋남 | master plan, focused plan, internal index와 evidence README가 PR #375/merge SHA를 공유한다. |
| 29 | Tailnet 미리보기가 이미 갱신됐다고 주장 | 이 diff는 source roadmap 변경만 기록하며 Tailnet sync를 후속 PR로 분리한다. |
| 30 | 선택 문서가 C11.4에서 남은 작업을 감춤 | negative implicit, nested/anonymous, auto-repeat 전체 조합, WPT와 상위 완료를 명시적으로 남겼다. |

이 검토는 PR #375 후속 문서 diff에 한정한다. Tailnet 파일 동기화·served hash·Agent Browser 화면 검증은 별도 sync review와 별도 PR diff review를 거친다.
