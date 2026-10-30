# C11.4 `pre-line` 병합 상태 문서 변경 검토 30개

검토 범위: `spec/STATUS.md`, `plan/c11-4-anonymous-grid-text-pre-line.md`, `spec/public/roadmap.html`의 PR #867 병합 상태 동기화. 제품 코드나 runtime은 변경하지 않았다.

| # | 독립 검토 관점 | 결과 |
|---:|---|---|
| 1 | PR이 단순 closed가 아니라 merged인지 확인 | GitHub `state=MERGED` |
| 2 | 병합 시각이 GitHub `mergedAt`과 일치하는지 확인 | `2026-10-30T16:45:11Z` 일치 |
| 3 | merge commit SHA가 PR 상세와 같은지 확인 | `c4e0aab3fd749ed9f76a8cf4fb09fc9a862e0d3d` 일치 |
| 4 | 상태 대장과 계획 문서가 같은 merge SHA를 쓰는지 확인 | 두 문서 일치 |
| 5 | 공개 로드맵도 같은 merge SHA를 쓰는지 확인 | 같은 SHA 기록 |
| 6 | check 미보고 상태를 CI 성공으로 오해하게 쓰지 않았는지 확인 | checks 미보고를 명시하고 성공 주장을 하지 않음 |
| 7 | 구현 PR 상태를 “검토 중”으로 남기지 않았는지 확인 | 병합 사실로 교체 |
| 8 | 병합 사실 때문에 C11을 완료 처리하지 않았는지 확인 | C11 상위 미완료 유지 |
| 9 | 병합 사실 때문에 C11.4를 완료 처리하지 않았는지 확인 | C11.4 미완료 유지 |
| 10 | 공식 완료 개수가 임의로 늘지 않았는지 확인 | `72/213` 유지 |
| 11 | C11.2.2 완료 체크를 보존했는지 확인 | 체크 상태 변경 없음 |
| 12 | C11.2.3 완료 체크를 보존했는지 확인 | 체크 상태 변경 없음 |
| 13 | C11.3 / C11.3.3 미완료 상태를 보존했는지 확인 | 체크 상태 변경 없음 |
| 14 | 기능 범위가 기존 opt-in fixture route로 제한되는지 확인 | 제한 범위 유지 |
| 15 | 일반 author CSS admission을 완료로 암시하지 않는지 확인 | 미완료로 명시 |
| 16 | Chrome oracle 버전이 사전 비교 기록과 맞는지 확인 | Chrome 156 유지 |
| 17 | normal control의 root/track 값이 runtime 근거와 맞는지 확인 | `20×10`, Columns `[50]`, Rows `[10]` |
| 18 | pre-line scene의 root/track 값이 runtime 근거와 맞는지 확인 | `20×20`, Columns/Rows `[20]` |
| 19 | scene 수를 fixture matrix 근거보다 크게 부풀리지 않는지 확인 | 고정 두 scene만 언급 |
| 20 | Android platform label이 AVD를 실기기로 오인시키지 않는지 확인 | API 37 ARM64 AVD로 명시 |
| 21 | iOS platform label과 OS version이 근거와 맞는지 확인 | iPhone 17 Pro / iOS 26.2 Simulator |
| 22 | trace와 draw/present 관찰을 glyph pixel 증거로 확대하지 않는지 확인 | glyph equality 아님을 명시 |
| 23 | screenshot surface 제한이 숨겨지지 않았는지 확인 | harness surface 한계 기록 |
| 24 | 전체 WPT 실행·적합성 완료로 오해할 표현이 없는지 확인 | 전체 WPT 미완료 명시 |
| 25 | Android hardware GPU 검증으로 오해할 표현이 없는지 확인 | 실기기/hardware GPU 미완료 명시 |
| 26 | iOS compositor 이후 pixel 검증으로 오해할 표현이 없는지 확인 | compositor 이후 pixels 미완료 명시 |
| 27 | UTC 병합 시각과 문서 작성 날짜의 경계가 혼동되지 않는지 확인 | merge 시각에 UTC 표기 포함 |
| 28 | PR 및 계획 링크가 대상 ID를 가리키는지 확인 | PR #867과 전용 계획 경로 사용 |
| 29 | PR #861 과거 거부 기록이 현재 동작과 모순되지 않는지 확인 | “그 시점” 기록으로 한정 |
| 30 | 변경이 세 의도된 문서와 검토서에 한정되는지 확인 | tracked diff는 상태·계획·로드맵 3개와 이 review 파일; 기존 Android/iOS 캡처는 재사용하고 미관련 untracked 자료는 보존 |

판정: 현재 확인된 차단 결함 없음. GitHub PR 상세와 checks 결과, 문서 원본의 체크박스·공식 수, Chrome/시뮬레이터 기대값을 각각 대조했다. 문서 route 생성과 Tailnet 반영은 별도 배포 검증이며 이 검토는 제품 테스트, CI, WPT 또는 전체 C11 완료를 뜻하지 않는다.
