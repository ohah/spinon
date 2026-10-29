# PR #819 병합 후 상태·로드맵 동기화 검토 30개

검토일: 2026-10-30
검토 대상: PR #819 병합 이후 `spec/STATUS.md`, C11 계획 두 문서, 구현 근거 README와 공개 로드맵의 상태 동기화 변경
기준 커밋: `1e4e93401e4959ef6aec121c0a2a721d56e45b11`
판정: 30개 독립 실패 관점을 확인했고 차단 결함은 발견하지 않았다.

| # | 적대적 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | PR 번호가 다른 작업을 가리키는가 | GitHub PR 응답의 URL이 `ohah/spinon/pull/819`다. 통과. |
| 2 | 병합 여부를 닫힘 상태만으로 추정했는가 | GitHub 응답에서 `state=closed`, `merged=true`를 각각 확인했다. 통과. |
| 3 | 병합 시각이 생성·갱신 시각과 혼동됐는가 | `merged_at=2026-10-29T20:53:09Z`를 사용했다. 통과. |
| 4 | 문서 SHA가 PR head SHA를 merge SHA라고 잘못 부르는가 | PR 응답의 `merge_commit_sha`와 문서 SHA가 모두 `1e4e9340…`다. 통과. |
| 5 | 로컬 체크아웃이 다른 PR 후보를 가리키는가 | 현재 `HEAD`와 `origin/main`은 모두 병합 커밋 `1e4e9340…`다. 통과. |
| 6 | 기반 브랜치가 다른 변경을 포함하는가 | PR #819의 base는 `main`이고 현재 변경은 그 merge commit에서 시작한다. 통과. |
| 7 | 계획 PR #818과 구현 PR #819를 혼동했는가 | 계획 링크·계획 merge SHA와 구현 PR·merge SHA를 각각 유지했다. 통과. |
| 8 | 인접한 PR #814의 track-list 구현을 #819 성과로 귀속하는가 | 공개 요약은 #819의 unknown-area 기본 auto sizing만 설명하고 track-list cycling을 주장하지 않는다. 통과. |
| 9 | 공식 전체 완료 수를 제한 profile 때문에 올렸는가 | `72/213`을 모든 갱신 문서에서 유지했다. 통과. |
| 10 | C11 상위 checkbox를 완료로 바꿨는가 | `spec/STATUS.md`의 C11 항목은 `[ ]` 그대로다. 통과. |
| 11 | C11.3.3 candidate를 공식 완료로 바꿨는가 | 상태 문구는 미완료를 명시하고 완료로 표시하지 않았다. 통과. |
| 12 | C11.4의 기존 상태를 함께 변경했는가 | C11.4 상태와 별도 증거는 이 diff에서 바뀌지 않았다. 통과. |
| 13 | 지원 범위를 모든 Grid mode로 확대해 썼는가 | area가 있는 `TemplateGridImplicitTracks` 제한 profile로 범위를 한정했다. 통과. |
| 14 | trailing 외 implicit row/column을 지원한다고 오해하게 하는가 | 요약에 trailing implicit row/column만 적었다. 통과. |
| 15 | area 없는 mode도 지원한다고 오해하게 하는가 | area 없는 mode는 거부한다고 명시했다. 통과. |
| 16 | signed 또는 leading implicit sizing을 지원한다고 오해하게 하는가 | signed/leading mode는 계속 거부한다고 명시했다. 통과. |
| 17 | arbitrary `grid-auto-*` list를 지원한다고 오해하게 하는가 | list 지원을 fixed-px profile로 한정했다. 통과. |
| 18 | 과거 기대 거부 기록을 성공 값으로 덮어썼는가 | matrix v1 보존과 source-bound v2 갱신 설명을 유지했다. 통과. |
| 19 | 전체 matrix의 scene/node 수가 실행 README와 다른가 | 최종 두 플랫폼 README의 37 scene·118 node와 일치한다. 통과. |
| 20 | cascade fallback을 성공이나 거부로 잘못 셌는가 | 32 성공·5 cascade fallback·0 기대 거부를 별도 수로 적었다. 통과. |
| 21 | 0 CSS px 오차를 모든 CSS로 일반화했는가 | 오차 주장은 고정된 37-scene matrix 범위에만 붙였다. 통과. |
| 22 | Chrome oracle을 Spinon WPT 결과로 표현했는가 | 공개 요약에 Spinon WPT 적합성 결과가 아니라고 명시했다. 통과. |
| 23 | Android AVD를 실기기나 hardware GPU로 표현했는가 | Android 대상은 API 37 ARM64 AVD/SwiftShader로 표기했다. 통과. |
| 24 | iOS Simulator를 실기기 실행으로 표현했는가 | iPhone 17 Pro/iOS 26.2 Simulator로 정확히 표기했다. 통과. |
| 25 | Metal drawable을 compositor 이후 화면으로 확대했는가 | 이 문서 변경은 compositor 이후 pixel을 주장하지 않는다. 통과. |
| 26 | 공개 Android 화면 링크가 오래된 run을 가리키는가 | 링크 파일이 PR #819 최종 Android run `203832317Z-36dd5796`에 존재한다. 통과. |
| 27 | 공개 iOS 화면 링크가 오래된 run을 가리키는가 | 링크 파일이 PR #819 최종 iOS run `204016316Z-f19f0361`에 존재한다. 통과. |
| 28 | 계획·근거·상태 링크가 서로 다른 작업을 여는가 | 집중 계획, 구현 README, 공식 STATUS 경로를 각각 확인했다. 통과. |
| 29 | 병합되지 않은 상태나 CI 통과를 잘못 게시하는가 | PR 검사 결과는 빈 배열이고 저장소에 필수 상태 검사가 없다고 적었으며 CI 통과를 주장하지 않는다. 통과. |
| 30 | 최상위 로드맵 최신 항목 순서와 변경 검토 근거가 빠졌는가 | #819 항목을 #816보다 앞에 추가했고, 이 검토 문서 경로를 공식 상태에서 연결한다. 통과. |

## 검토 범위와 한계

이 기록은 문서 diff와 이미 병합된 PR·source-bound 실행 근거를 대조한 검토다. 새 제품 코드, 새 앱 실행 또는 전체 CSS Grid/WPT 검증을 추가하지 않았다. 문서 미리보기 배포는 별도 URL에서 확인한다.
