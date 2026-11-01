# C11.1 named-area 병합 후 문서 동기화 실패 관점 검토 30개

**범위:** PR #1006 병합 후 `spec/STATUS.md`, 전용 계획, 실행 README, PR diff review 및 hash manifest의 상태 동기화를 검토했다. 공식 완료 상태는 승격하지 않았다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 문서가 다른 구현 PR을 가리킨다. | 네 문서 모두 구현 PR #1006을 기록한다. 통과. |
| 2 | PR URL이 다른 저장소나 번호를 가리킨다. | 링크가 `ohah/spinon/pull/1006`이다. 통과. |
| 3 | merge timestamp가 GitHub 값과 다르다. | `2026-11-01T22:11:00Z`가 PR metadata와 일치한다. 통과. |
| 4 | rebase merge를 squash/merge commit으로 잘못 부른다. | 문서가 `Rebase and merge`로 기록한다. 통과. |
| 5 | merge SHA가 다른 commit을 가리킨다. | `f4bcce22566c147277bf51ab1f3eae37f12caf4f`가 PR mergeCommit과 origin/main HEAD다. 통과. |
| 6 | merged PR이 아직 열린 후보라고 표시된다. | 계획·status·evidence에서 merge pending 문구를 제거했다. 통과. |
| 7 | 체크 없는 상태를 CI 성공으로 바꿔 쓴다. | GitHub check run 없음과 CI 통과 비주장을 명시한다. 통과. |
| 8 | branch protection 필수 checks를 잘못 서술한다. | main의 required status checks가 비활성인 관측을 기록한다. 통과. |
| 9 | `MERGEABLE/CLEAN`을 CI 결과처럼 쓴다. | mergeability는 pre-merge GitHub 상태로만 기록한다. 통과. |
| 10 | review decision이 없었던 사실을 승인으로 해석한다. | review decision이 비어 있음을 승인과 구분한다. 통과. |
| 11 | PR 본문 screenshot link가 렌더링되지 않는다고 놓친다. | Agent Browser에서 두 이미지가 `complete=true`로 확인됐다. 통과. |
| 12 | Android PNG 크기를 잘못 기록한다. | 자연 크기 1080×2400을 Browser DOM에서 확인했다. 통과. |
| 13 | iOS PNG 크기를 잘못 기록한다. | 자연 크기 1206×2622를 Browser DOM에서 확인했다. 통과. |
| 14 | Files changed 페이지가 로드되지 않았는데 확인했다고 쓴다. | Agent Browser에서 PR `Files changed 56` 화면이 렌더링됐다. 통과. |
| 15 | C11.1 제한 candidate가 상위 완료로 승격된다. | C11 및 C11.1 완료 checkbox는 미완료로 유지했다. 통과. |
| 16 | C11.3.3 또는 C11.4를 이번 PR로 완료 처리한다. | 두 하위 항목도 미완료로 유지한다. 통과. |
| 17 | 공식 완료 집계를 잘못 올린다. | 공식 합계 `72/213`을 유지한다. 통과. |
| 18 | 고정 2×2 slice를 일반 CSS Grid로 확대 해석한다. | plan과 contract가 제한 입력과 일반 지원 한계를 유지한다. 통과. |
| 19 | Android AVD를 실기기로 표기한다. | Android 증거는 API 37 ARM64 AVD로 명시한다. 통과. |
| 20 | iOS Simulator를 실기기로 표기한다. | iPhone 17 Pro/iOS 26.2 Simulator로 명시한다. 통과. |
| 21 | screenshot을 PixelCopy/Metal readback으로 잘못 설명한다. | README에서 양쪽 drawable/readback을 수행하지 않았다고 유지한다. 통과. |
| 22 | draw/present를 compositor 이후 exact pixel로 과장한다. | post-compositor 결과 주장을 배제한다. 통과. |
| 23 | software/Simulator 실행을 hardware GPU 검증으로 확대한다. | hardware GPU 주장을 배제한다. 통과. |
| 24 | 전체 crate test failure를 숨긴다. | 278/279 결과와 기존 C07 실패를 그대로 공개한다. 통과. |
| 25 | C07 serialization 값이 문서에서 바뀐다. | `1.7777778 / 1` 대 `1.77778 / 1`을 그대로 보존한다. 통과. |
| 26 | PR #1004 계획 병합과 #1006 구현 병합을 혼동한다. | 계획 PR과 구현 PR 역할 및 merge SHA를 분리한다. 통과. |
| 27 | plan, status, runtime README의 merge 정보가 어긋난다. | PR 번호, merge timestamp, SHA 및 `72/213` 표기를 교차 대조했다. 통과. |
| 28 | evidence hash manifest가 수정 후 stale 상태다. | README와 review 갱신 뒤 manifest의 39개 evidence 경로 해시를 재생성해 검증했다. 통과. |
| 29 | post-merge review 링크가 빠져 추적할 수 없다. | 실행 README에서 이 review 파일을 연결한다. 통과. |
| 30 | 후속 문서 변경이 기본 runtime의 제품 API 또는 CSS 지원을 주장한다. | 이번 동기화는 내부 candidate 상태와 실행 근거만 갱신한다. 통과. |

## 판정

문서의 PR disposition, merge SHA, checks 상태, simulator 경계와 공식 완료 수가 실제 PR metadata 및 evidence와 일치한다. CI 통과를 주장하지 않고 C11/C11.1/C11.3.3/C11.4 체크와 공식 `72/213`을 유지한다.
