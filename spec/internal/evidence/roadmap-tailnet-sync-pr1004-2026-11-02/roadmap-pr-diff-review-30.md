# C11.1 named-area 계획 병합 후 상태·로드맵 diff 검토 30개

검토 대상은 계획 PR #1004 merge SHA `77cc880c37a14dff0b26ad7202d97e1947c894b5` 뒤의 `spec/STATUS.md`, C11 계획 두 문서와 공개 `roadmap.html` 변경이다. named-area 기능 구현 PR 검토와 구분해 merge provenance, 공식 완료 상태, 문서 간 범위, 링크와 공개 문구를 확인한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #1004 merge SHA를 변경 원본 commit으로 혼동함 | GitHub merge commit SHA `77cc880c37a14dff0b26ad7202d97e1947c894b5`를 merge SHA로 구분한다. |
| 2 | merge 시각을 local timezone으로 재해석함 | GitHub 값 `2026-11-01T21:15:51Z`를 UTC 그대로 쓴다. |
| 3 | 계획 PR을 구현 PR로 분류함 | 세 문서는 계획 PR이라고 명시한다. |
| 4 | 계획 merge를 runtime 구현 완료로 확장함 | 구현/runtime 시작 전이라고 명시한다. |
| 5 | named-area 계획 merge로 C11 상위 checkbox를 완료함 | C11 checkbox는 `[ ]` 그대로다. |
| 6 | C11.1 named-area를 완료로 표시함 | C11.1 관련 미완료 상태를 유지한다. |
| 7 | C11.3.3 template-area candidate를 완료로 승격함 | C11.3.3 미완료 경계를 유지한다. |
| 8 | C11.4의 독립 Grid text slice를 C11 전체 완료로 읽음 | C11.4 전체와 C11 상위 모두 미완료라고 쓴다. |
| 9 | 공식 완료 수에 계획 PR을 더함 | 공식 `72/213`을 네 문서에서 그대로 유지한다. |
| 10 | 단순 하위 checkbox 수를 공식 진행률로 게시함 | 체크박스 개수나 퍼센트를 새 진행률로 제시하지 않는다. |
| 11 | Chrome 156 관측을 Spinon 결과로 표현함 | 이를 구현 전 Agent Browser 관측으로 한정한다. |
| 12 | Chrome 관측을 CSS 명세 판정으로 과장함 | 독립 기대값과 browser 관측의 역할을 구분한다. |
| 13 | source-order reverse를 paint-order 변경으로 해석함 | 계획 문서에서 semantic frame 보존과 DOM/source paint order 보존을 구분한다. |
| 14 | Android 실기기 검증이 완료됐다고 주장함 | Android runtime·simulator 검증은 시작 전이라고 쓴다. |
| 15 | Android AVD 또는 PixelCopy 표본을 완료 근거로 덧붙임 | 새 Android 실행·PixelCopy를 주장하지 않는다. |
| 16 | iOS Simulator 검증이 완료됐다고 주장함 | iOS simulator 검증은 시작 전이라고 쓴다. |
| 17 | iOS Metal 결과를 compositor pixel 비교로 넓힘 | 새 iOS drawable/compositor 결과를 주장하지 않는다. |
| 18 | 실기기 또는 hardware GPU 검증을 암시함 | 실기기·hardware GPU 완료 주장을 추가하지 않는다. |
| 19 | 일반 CSS author admission이 추가됐다고 오해하게 함 | 후보를 제한된 parsed Root HTML 계획으로 설명한다. |
| 20 | fixed-ID C11 fixture 경로를 default Root HTML 통합으로 취급함 | 계획 문서가 default parsed Root HTML runtime 경계를 유지한다. |
| 21 | PR #1000 또는 #1002의 기존 제한 범위를 확장함 | 기본 Grid와 fixed-px gap을 각각 기존 제한 slice로 남긴다. |
| 22 | PR #1004의 GitHub 상태와 문서 metadata가 다름 | PR은 MERGED이며 제목·merge time·SHA가 일치한다. |
| 23 | PR check rollup이 비었는데 CI 통과로 기록함 | CI 성공 주장을 추가하지 않는다. |
| 24 | 계획 실패 관점 검토 20개 링크가 사라짐 | named-area plan 문서의 기존 20개 검토 링크를 보존한다. |
| 25 | 계획 PR diff 검토 30개 링크가 사라짐 | precomparison evidence의 기존 계획 PR diff 검토 파일을 보존한다. |
| 26 | C11 상위 계획 index가 candidate 상태를 혼동함 | index를 #1004 계획 merge와 구현 미시작으로 동기화한다. |
| 27 | 이전 index의 중복·깨진 문장이 남음 | `있다.은`로 이어지던 문장을 단일 Root HTML 진행 요약으로 교체한다. |
| 28 | 공개 roadmap의 로컬 plan 링크가 잘못됨 | `/spinon/plan/c11-root-html-grid-template-areas.html` 경로와 source plan을 대조한다. |
| 29 | 사전 비교 README 또는 PNG 링크가 누락됨 | 두 링크가 precomparison evidence 파일을 가리킨다. |
| 30 | Tailnet preview 갱신이나 Agent Browser 종료를 완료 사실로 선기록함 | 이 diff는 preview deployment를 완료했다고 주장하지 않으며, live sync·browser 확인은 별도 단계로 기록한다. |

문서 diff 검사에서 별도 구현 검증이나 전체 docs 링크 성공을 주장하지 않는다.
