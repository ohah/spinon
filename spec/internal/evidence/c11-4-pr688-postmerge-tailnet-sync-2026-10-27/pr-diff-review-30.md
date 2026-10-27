# PR #688 병합 상태·Tailnet roadmap 후속 diff 검토 30개

검토 대상은 `spec/STATUS.md`, `plan/c11-4-grid-layout-auto-tracks-wpt.md`, `spec/public/roadmap.html` 및 이 폴더의 동기화·브라우저 근거다. 30개 관점에서 병합 사실, 공식 상태, 범위, 동기화 보호, 화면 증거를 따로 확인했다. 미해결 차단 항목은 없다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 잘못된 PR 번호 | #688로 고정했다. |
| 2 | PR 상태 stale | GitHub 조회에서 `MERGED`를 확인했다. |
| 3 | merge 방식 혼동 | Rebase and merge로 기록했다. |
| 4 | merge 시각 오기 | GitHub의 `2026-10-27T08:19:30Z`와 일치한다. |
| 5 | merge SHA를 branch SHA로 표기 | GitHub merge commit `0b434e16ac66d0a742057e6e1e5749151bd3e08f`를 사용했다. |
| 6 | check rollup을 CI 통과로 확대 | 비어 있음을 기록하고 CI 통과 주장을 하지 않는다. |
| 7 | branch protection 상태 추측 | main required status checks가 설정되지 않은 상태로 기록했다. |
| 8 | C11.4 완료로 잘못 표시 | C11.4 항목은 미완료로 유지했다. |
| 9 | 공식 완료 수 임의 증가 | `72/213`을 유지했다. |
| 10 | 체크박스와 수치 불일치 | 상위 C11.4 체크박스는 그대로 미완료다. |
| 11 | 제한 scene을 전체 Grid로 확대 | 고정 WPT scene 하나의 제한 runtime slice라고 명시했다. |
| 12 | WPT 전체 실행으로 확대 | 전체 WPT는 미검증으로 명시했다. |
| 13 | 일반 author CSS admission 주장 | 미검증으로 남겼다. |
| 14 | Chrome 관찰을 runtime pass로 혼합 | Chrome 원본 oracle와 Android/iOS runtime evidence의 역할을 분리했다. |
| 15 | root/track geometry 오기 | columns `[100,50]px`, rows `[10,10]px`, root `100×20px`를 고정했다. |
| 16 | Android trace 범위 오기 | Android 11/11을 기록했다. |
| 17 | iOS trace 범위 오기 | iOS 21/21을 기록했다. |
| 18 | overflow 표본 누락 | `(125,5)`, `(149,5)`, `(150,5)`를 연결했다. |
| 19 | pixel 비교 표본 수 확대 | 각 플랫폼 표본을 3/3으로 제한했다. |
| 20 | Android renderer를 hardware GPU로 오인 | ANGLE/SwiftShader로 명시했다. |
| 21 | iOS drawable을 compositor 이후로 오인 | 제출 전 표본으로 명시했다. |
| 22 | static archive를 APK/IPA 전체 증거로 확대 | archive marker 0/8 결과만 설명한다. |
| 23 | fixture positive control 누락 | opt-in marker 8개 positive control을 기록했다. |
| 24 | real device 결과로 오인 | 실기기 미검증을 명시했다. |
| 25 | screenshot이 다른 source revision을 가리킴 | 화면 링크는 PR #688 merge SHA에 고정했다. |
| 26 | plan 경로/slug 오기 | 현재 계획 문서 경로와 route를 확인했다. |
| 27 | evidence README 경로 오기 | merge SHA에 고정된 실행 근거를 연결했다. |
| 28 | public card가 추가되지 않거나 중복됨 | Agent Browser에서 42개 hero card 중 PR #688 항목 하나를 확인했다. |
| 29 | Tailnet stale page 제공 | local/Tailnet HTTP 200 body hash가 source SHA와 일치한다. |
| 30 | browser·sync 부작용 누락 | byte-exact backup, 단일 route 교체, server 미재시작, browser 종료를 기록했다. |

## 검토 범위 한계

이 문서는 병합 후 상태·로드맵 동기화 변경을 검토한다. PR #688의 구현 검토 20개와 merge 전 diff 검토 30개를 대체하거나 재사용하지 않는다.
