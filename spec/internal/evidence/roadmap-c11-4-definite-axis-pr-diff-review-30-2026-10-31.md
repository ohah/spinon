# C11 roadmap PR #878 반영 diff 실패 관점 검토 30개

**대상:** `spec/public/roadmap.html`의 C11.4 definite-axis 후보 기록. 구현 PR #878의 상태와 근거를 공개 로드맵에 옮기는 문서 diff이며 공식 완료 상태 변경은 포함하지 않는다.

| # | 별도 실패 경로 | 결과 |
| ---: | --- | --- |
| 1 | 새 기록이 C11 작업과 무관한 로드맵 구역에 들어가는가? | C11.4 최근 항목인 PR #873 바로 다음에 배치했다. |
| 2 | PR 링크가 원래 계획 PR이나 다른 기능 PR을 가리키는가? | 링크와 표시 번호 모두 구현 PR #878을 가리킨다. |
| 3 | PR merge SHA가 구현 PR의 최종 rebase commit과 다른가? | `b1b7356e2e2afe397c2d41ebf111863a7f25ccd9`가 `gh pr view 878`의 `mergeCommit.oid`와 일치한다. |
| 4 | 기록된 merge 시각이 현지 시각처럼 오해될 수 있는가? | GitHub가 반환한 UTC 시각 `2026-10-30T21:47:44Z`를 그대로 표기했다. |
| 5 | 후보가 미완료인 상태에서 완료 작업처럼 제목을 붙였는가? | 제한 opt-in fixture라고 명시하고 PR merge만 기록했다. |
| 6 | definite width 방향에서 기대 높이를 뒤집어 기록하는가? | `width:60px`과 `2 / 1` 비율의 axis transfer로 기록했다. |
| 7 | definite height 방향에서 기대 너비를 뒤집어 기록하는가? | `height:30px`과 `2 / 1` 비율을 함께 표기했다. |
| 8 | preferred ratio가 다른 수치 또는 단위와 혼동되는가? | numeric `2 / 1`임을 코드 표기로 고정했다. |
| 9 | 대조군도 ratio 적용 성공 scene처럼 합쳐지는가? | 네 신규 scene 중 ratio 두 개와 no-ratio control 두 개의 구분을 기록했다. |
| 10 | 회귀 matrix의 구성 수가 새 장면 수와 혼동되는가? | 기존 51개와 신규 4개를 분리하고 합계 55개를 적었다. |
| 11 | Android 전체 matrix를 신규 4개만 통과한 것으로 과장하는가? | 기존 포함 Android AVD 55/55로 구분했다. |
| 12 | iOS 전체 matrix를 신규 4개만 통과한 것으로 과장하는가? | 기존 포함 iOS Simulator 55/55로 구분했다. |
| 13 | Android PixelCopy 표본을 장면 수로 오해하는가? | PixelCopy surface sample 177/177 exact를 별도로 표기했다. |
| 14 | iOS Metal sample 수를 전체 장면 수로 오해하는가? | Metal drawable sample 177/177 exact를 별도로 표기했다. |
| 15 | AVD와 실기기 검증 환경을 혼동하는가? | Android API 37 ARM64 AVD라고 정확히 부르고 실기기 근거를 배제했다. |
| 16 | Android AVD 결과가 hardware GPU 결과로 읽히는가? | hardware GPU와 실기기 검증이 아님을 명시했다. |
| 17 | iOS drawable readback이 compositor 완료 화면으로 읽히는가? | compositor 제출 이후 픽셀 검증이 아님을 명시했다. |
| 18 | Simulator pixel match가 운영체제 최종 표시와 동일하다고 주장하는가? | 표본 경계를 Simulator surface readback으로 제한했다. |
| 19 | 고정 fixture가 Spinon WPT 적합성을 뜻한다고 읽히는가? | Spinon WPT를 입증하지 않는다고 명시했다. |
| 20 | fixture 경로가 일반 author CSS admission으로 확대 해석되는가? | 일반 CSS admission을 지원하지 않는다고 명시했다. |
| 21 | 내부 검증 route가 public API로 읽히는가? | PR 기록은 fixture 범위이며 근거 문서는 API 없음으로 구분한다. |
| 22 | C11.4 하위 후보 merge를 C11.4 완료로 올리는가? | C11.4 미완료를 명시했다. |
| 23 | C11.4 slice가 상위 C11 완료를 뜻하는가? | C11 미완료를 별도로 적었다. |
| 24 | 전체 완료 집계 수를 후보 merge 때문에 증가시키는가? | 공식 `72/213`을 유지한다고 적었다. |
| 25 | PR check가 없는데 CI 통과로 표현되는가? | check rollup이 비어 있어 CI 성공으로 표시하지 않는다고 적었다. |
| 26 | main required status checks 설정 부재와 PR checks 결과를 혼동하는가? | 두 상태를 별도로 조회했고 로드맵에서도 CI 성공 주장을 하지 않는다. |
| 27 | 계획 링크 slug가 실제 추가 계획 파일과 다른가? | `c11-4-direct-text-aspect-ratio-definite-axis` 경로를 사용했다. |
| 28 | 내부 계약 링크가 기존 0059 Grid 계약이 아닌 다른 계약을 가리키는가? | `0059-c11-grid-layout.html`로 연결했다. |
| 29 | 실행 근거 링크가 신규 evidence README로 연결되지 않는가? | `c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/README.html`을 사용했다. |
| 30 | 공식 상태 링크나 기존 최근 항목을 훼손하는가? | 상태 체크리스트 anchor를 연결하고 PR #873 기존 문단은 보존했다. |

**검토 결론:** 문서 diff는 C11.4의 제한 후보, 실행 범위, 근거와 한계를 추가하며 공식 완료 수나 체크박스를 변경하지 않는다.
