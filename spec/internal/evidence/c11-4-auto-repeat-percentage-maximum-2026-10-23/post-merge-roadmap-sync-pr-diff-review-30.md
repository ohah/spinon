# C11.4 PR #568 병합 상태 문서 동기화 PR diff 검토 30개

검토 대상은 `origin/main` 대비 후속 문서 diff다. 코드/runtime 검토 20개 및 구현 PR diff 검토 30개와 분리해 merge facts, 공식 상태, scope 표현, 공개 페이지 간 링크·일관성의 별도 실패 경로를 확인했다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | PR 번호가 다른 작업으로 잘못 연결되는가? | 통과: GitHub API에서 PR #568 제목과 branch를 조회했다. |
| 2 | 병합 여부를 open/draft 상태로 잘못 기록하는가? | 통과: PR state는 `MERGED`다. |
| 3 | 병합 시각이 로컬 시간과 UTC 사이에서 섞이는가? | 통과: `2026-10-23T14:16:01Z`로 UTC를 고정했다. |
| 4 | merge SHA가 PR head SHA로 잘못 기재되는가? | 통과: merge SHA `c436a38388fe8a191fd374495c516c88d1e3cd54`와 head `7ed088a5b08a847627f7fab5ac4ce0d14d5fc61a`를 구분했다. |
| 5 | 병합 방식을 merge commit/squash로 잘못 표현하는가? | 통과: Rebase and merge로 기록했다. |
| 6 | 빈 PR check rollup을 “CI 통과”로 부풀리는가? | 통과: rollup이 비어 있었다고 표기한다. |
| 7 | main required status check 부재를 혼동하는가? | 통과: GitHub API는 required checks not enabled(HTTP 404)라고 응답했다. |
| 8 | base branch를 다르게 표시하는가? | 통과: GitHub PR base는 `main`이다. |
| 9 | diff 규모가 로컬과 GitHub에서 달랐는가? | 통과: 둘 다 243 files, 13,064 additions, 25 deletions였다. |
| 10 | C11.4 전체가 완료된 것으로 표현되는가? | 통과: 계획·상태·로드맵에서 상위 체크를 미완료로 유지한다. |
| 11 | 공식 완료 수가 실험 결과만큼 증가하는가? | 통과: `72/213`을 그대로 유지한다. |
| 12 | README의 78 상위 항목과 213 표시 작업 설명이 바뀌는가? | 통과: 집계 설명을 유지한다. |
| 13 | active lane이 C13으로 다시 보이는가? | 통과: 로드맵 실행 설정은 C11.4이며 C13은 비활성이다. |
| 14 | C13이 전체 구현 완료로 오해되는가? | 통과: C13 checkbox는 미완료이고 제한된 C13.1/2/3 범위만 적는다. |
| 15 | C13 ink-only shadow overflow fail-closed 경계가 사라지는가? | 통과: 상태와 로드맵에서 유지한다. |
| 16 | public DOM scroll event/CSSOM 구현으로 오해되는가? | 통과: 아직 구현되지 않았다고 기록한다. |
| 17 | native wheel/trackpad 부분 근거가 일반 지원으로 확대되는가? | 통과: 플랫폼별 부분 검증으로 제한한다. |
| 18 | C11.4 고정 fixture를 일반 CSS admission으로 표현하는가? | 통과: opt-in fixed scene subset으로 제한한다. |
| 19 | Android 소프트웨어 renderer를 hardware GPU로 과장하는가? | 통과: ANGLE/SwiftShader라고 명시한다. |
| 20 | iOS Metal drawable readback을 compositor 이후 픽셀로 과장하는가? | 통과: compositor 제출 전이라고 명시한다. |
| 21 | Simulator 실행을 실기기 실행으로 잘못 표시하는가? | 통과: Android API 37 AVD와 iOS 26.2 Simulator로 기록한다. |
| 22 | 28 scene 또는 78 sample 집계가 한 플랫폼에만 해당하는데 두 플랫폼으로 확장되는가? | 통과: 각 플랫폼별 28/28 및 78/78로 구분한다. |
| 23 | 초기 workspace test 실패를 숨기는가? | 통과: 최종 회귀 로그와 초기 실패 원인을 README에서 같이 설명한다. |
| 24 | workspace Clippy나 focused test 성공 근거가 빠지는가? | 통과: 각각의 링크와 범위를 실행 근거에 보존한다. |
| 25 | fixture module boundary test 수가 잘못 바뀌는가? | 통과: 현재 로그는 7개 통과로 연결한다. |
| 26 | static archive 검증을 APK/IPA 전체 제외로 과장하는가? | 통과: FFI static archive에만 한정한다고 명시한다. |
| 27 | 20개 구현/runtime 검토와 30개 PR diff 검토가 같은 문서로 합쳐지는가? | 통과: 두 별도 문서 링크를 유지한다. |
| 28 | 완료된 PR 검토 문서가 PR 미생성 상태라고 계속 주장하는가? | 통과: 최종 GitHub diff·이미지 확인 및 merge facts를 기록한다. |
| 29 | PR 화면 이미지 결과 크기를 잘못 연결하는가? | 통과: Android 1080×2400, iOS 1206×2622 DOM 값을 확인했다. |
| 30 | 공식 상태·계획·evidence README·공개 roadmap 중 하나가 다른 merge 상태를 표시하는가? | 통과: 네 문서의 C11.4 상태, PR #568, merge SHA와 미완료 경계가 같은 내용을 담는다. |

**결과:** 이 문서 변경에서 미해결 P1/P2를 찾지 못했다. PR #568 병합은 제한된 runtime slice를 기록하는 사실이며 C11.4 완료 판정이나 공식 집계 변경 근거가 아니다. Tailnet 정적 preview는 병합 뒤 별도 Agent Browser와 HTTP hash 확인으로 검증한다.
