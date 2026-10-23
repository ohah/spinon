# C11.4 병합 상태·roadmap 동기화 · PR diff 실패 관점 30개

대상은 구현 PR #580 병합 뒤의 공식 상태표, 집중 계획, 공개 roadmap, runtime evidence 및 이 동기화 evidence다. 구현 PR #580의 diff review 30개와 별도로 문서의 사실·시점·상태 전이를 확인했다.

| # | 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | PR 번호가 다른 C11.4 slice를 가리키는가 | 링크 대상은 구현 PR #580이다. | 통과 |
| 2 | Rebase and merge 결과 SHA가 틀렸는가 | GitHub `mergeCommit.oid`는 `33b00d077147ad9f2102af3220daa2c6cf74e2f9`다. | 통과 |
| 3 | merge 시간이 현지/UTC 혼동으로 변형됐는가 | GitHub 원본 `mergedAt`은 `2026-10-23T20:46:27Z`다. | 통과 |
| 4 | merged PR의 현재 `mergeStateStatus=UNKNOWN`을 병합 전 실패로 오독하는가 | 병합 전 live query의 `CLEAN`과 post-merge `UNKNOWN`을 구분한다. | 통과 |
| 5 | check이 없는데 CI green이라고 주장하는가 | required check 응답은 no checks reported이고 branch protection 조회는 미설정 404였다. | 통과 |
| 6 | 테스트 결과를 GitHub check으로 오해하는가 | 로컬 test/build와 GitHub status check 상태는 별도 문장으로 유지한다. | 통과 |
| 7 | merge SHA만 적고 source PR을 놓치는가 | plan·STATUS·roadmap 모두 #580 링크를 둔다. | 통과 |
| 8 | 상위 C11.4 checkbox가 implementation PR merge만으로 체크되는가 | `spec/STATUS.md` 체크 상태와 총 완료 수를 바꾸지 않았다. | 통과 |
| 9 | 공식 완료 수가 과거 74/207이나 raw checkbox 개수로 되돌아가는가 | 현재 공식 집계 `72/213`을 유지하고 이전 숫자는 역사 기록으로 둔다. | 통과 |
| 10 | C11.4 slice merge를 전체 Grid 지원으로 확대하는가 | 모든 갱신 문구는 제한된 percentage-neighbor slice로 한정한다. | 통과 |
| 11 | 13개 고정 runtime scene 수를 혼동하는가 | Android/iOS 각 13/13을 기록한다. | 통과 |
| 12 | 15개 PixelCopy 표본을 전체 화면 exact로 확대하는가 | 선언 표본 15/15로만 기록한다. | 통과 |
| 13 | 제출 전 Metal drawable을 iOS compositor 이후 픽셀로 오해하는가 | compositor 이후 pixels는 미검증으로 유지한다. | 통과 |
| 14 | AVD/Simulator 결과를 실기기로 확대하는가 | 실기기와 hardware GPU 결과는 미검증으로 명시한다. | 통과 |
| 15 | Chrome 196px 관측을 규범 200px 기대값과 합치는가 | 두 값과 end-alignment 차이를 분리한다. | 통과 |
| 16 | `72/213` 카운터와 미완료 C11.4가 모순처럼 보이는가 | 숫자는 필터된 official task 집계이고 slice merge가 상위 완료를 의미하지 않는다고 유지한다. | 통과 |
| 17 | stale “candidate branch not merged”가 남아 있는가 | roadmap, status, plan, runtime evidence의 현재 문구를 병합 결과로 갱신했다. | 통과 |
| 18 | C13 전체를 완료로 보이게 하는가 | C13 상위 항목은 미완료로 유지한다. | 통과 |
| 19 | C13 미완료와 현재 active lane을 같은 상태로 혼동하는가 | 문서가 C13 비활성, C11.4 활성이라고 구분한다. | 통과 |
| 20 | C13.2 shadow overflow fail-closed를 누락하는가 | `box-shadow` ink-only overflow 미지원 사실을 유지한다. | 통과 |
| 21 | C13 public `scroll` event/CSSOM 존재를 잘못 주장하는가 | 공개 event와 CSSOM은 미구현이라고 적는다. | 통과 |
| 22 | OS-native wheel/trackpad 근거를 완전한 입력 지원으로 확대하는가 | 플랫폼별 부분 검증으로 제한한다. | 통과 |
| 23 | C13.4–C13.6 잔여 범위를 숨기는가 | scrollbar/gutter, scroll snap, logical axis 미완료를 나열한다. | 통과 |
| 24 | PR #185 merged 상태와 active lane을 혼동하는가 | PR #185를 완료된 후속으로 남기고 C13 전체는 비활성·미완료로 표시한다. | 통과 |
| 25 | 내부 계약 숫자 버전이 implementation merge로 오르는가 | 내부 계약 `0.1.0`을 유지한다. | 통과 |
| 26 | plan과 STATUS가 서로 다른 최신 구현 범위를 말하는가 | 두 문서는 동일 PR, merge SHA, scene 수와 제한을 연결한다. | 통과 |
| 27 | public roadmap 링크가 PR diff review route를 잃는가 | 구현 review와 PR diff review 링크를 유지한다. | 통과 |
| 28 | PR screenshot이 image file 자체가 아닌 화면 capture 설명만 있는가 | PR #580 body의 raw image URL 두 개를 Agent Browser로 확인했다. | 통과 |
| 29 | 사용자 질문의 대상인 “overflow·scroll 진행 중” 혼동이 남는가 | C13이 backlog로 미완료 상태인 것과 현재 실행 lane인 것을 구분한다. | 통과 |
| 30 | private preview를 GitHub Pages 배포와 혼동하거나 너무 일찍 갱신하는가 | Tailnet은 이 문서 PR 병합 후 동기화하며 Pages workflow는 실행하지 않는다. | 통과 |

검토 결과, 문서 변경은 merge metadata와 공식 상태를 맞추며 구현 완료 범위나 체크 집계를 넓히지 않는다.
