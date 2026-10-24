# C11.4 fixed-repeat named-line merge · 병합 후 상태·로드맵 문서 검토 30개

**검토 범위:** PR #600 병합 후 수정한 plan, 공식 status 대장, runtime evidence와 공개 로드맵 HTML의 사실 일치·검증 한계·링크·화면 게시를 별도 관점으로 대조했다. 코드 동작의 재검토나 새로운 플랫폼 실행 근거가 아니다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | PR이 실제 병합되지 않았는데 병합으로 기록 | `gh pr view 600`에서 state `MERGED` 확인 |
| 2 | 다른 PR의 merge SHA를 사용 | PR mergeCommit과 문서 SHA가 `d1d6c3c20b7a27d6af84e982df507193799a7f7d`로 일치 |
| 3 | 병합 UTC 시각이 잘못됨 | GitHub `mergedAt` `2026-10-24T13:12:01Z`와 일치 |
| 4 | 병합 방법 표기가 다름 | PR 기록의 Rebase and merge와 문서 표기가 일치 |
| 5 | 병합 PR 링크가 다른 저장소를 가리킴 | ohah/spinon PR #600 경로 사용 |
| 6 | 빈 check rollup을 CI 통과로 과장 | 빈 rollup과 required checks 부재를 명시하고 CI 통과 주장을 배제 |
| 7 | post-merge 기준 커밋이 다른 HEAD | 확인 당시 `main`, `HEAD`, `origin/main`이 merge SHA와 일치 |
| 8 | style test 수치를 과장 | focused `fixed_repeat` 결과 16/16만 기록 |
| 9 | layout 통합 test 수치를 과장 | `fixed_repeat_line_names` 결과 4/4만 기록 |
| 10 | FFI scene test 범위를 확대 | 단일 named auto-repeat scene test 1/1로 한정 |
| 11 | fixture boundary 결과를 제품 적합성으로 취급 | fixture module boundary 11/11로 분리 표기 |
| 12 | Clippy 결과 범위를 과장 | workspace all-target Clippy 실행 성공만 기록 |
| 13 | 전체 workspace test를 실행한 것처럼 오해 | 전체 workspace test 통과 주장은 추가하지 않음 |
| 14 | C11 parent를 완료 처리 | 공식 checkbox를 미완료로 유지 |
| 15 | 공식 완료 집계를 임의 증가 | `72/213`을 모든 수정 문서에서 유지 |
| 16 | C11.3.3 areas 후속을 완료 처리 | template-areas follow-up을 남은 범위로 명시 |
| 17 | C11.4 전체를 완료 처리 | intrinsic sizing 전체가 열려 있음을 명시 |
| 18 | fixture slice를 일반 runtime으로 확대 | opt-in fixture runtime 한정으로 표기 |
| 19 | CSS 지원을 공개 admission으로 과장 | 일반 CSS admission 미완료를 명시 |
| 20 | Chrome 결과를 WPT 적합성으로 확대 | WPT 미완료로 표기 |
| 21 | Android AVD를 실기기 결과로 오해 | API 37 ARM64 AVD라고 명시 |
| 22 | Android simulator를 hardware GPU로 표현 | hardware GPU 미검증이라고 명시 |
| 23 | iOS drawable을 compositor output으로 표현 | compositor 제출 전 Metal sample이라고 명시 |
| 24 | simulator 검증을 모든 OS/device로 확대 | Android AVD와 iOS Simulator 두 환경에 한정 |
| 25 | pixel sample 수를 scene 수와 혼동 | scene 3/3과 sample 12/12를 구분 |
| 26 | plan이 PR open 상태로 남음 | plan 상태·하단 병합 요약을 갱신 |
| 27 | runtime evidence가 merge/post-merge 미완료로 남음 | 제목과 review/remaining gates를 병합 기준으로 갱신 |
| 28 | 공식 status에 PR open 문구가 남음 | #600 merge 시각·SHA와 check 제한으로 교체 |
| 29 | 공개 roadmap에 PR #600 및 화면 링크가 누락 | 새 C11.4 항목에 PR, Android/iOS screenshot, status 링크를 추가 |
| 30 | 새 리뷰 링크가 문서 사이트에 누락 | status·plan·runtime에서 리뷰 문서 링크를 연결 |

**결과:** 30개 관점에서 병합 상태·공식 완료 수·검증 범위의 문서 불일치를 확인해 수정했다. C11 완료 여부는 바꾸지 않았다.
