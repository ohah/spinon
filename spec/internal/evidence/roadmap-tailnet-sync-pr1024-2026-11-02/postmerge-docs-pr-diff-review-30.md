# C11.1 implicit rows 병합 후 문서 PR diff 실패 관점 검토 30개

검토 대상은 기준 `origin/main` `90723452724656a985c91428eaabc75976d8ad45`에 대한 C11 계획, 전용 계획, 공식 상태, 실행 README와 공개 roadmap의 병합 상태 반영이다. 구현 PR #1024의 merge API 결과·시각을 확인하고, 각 문서가 같은 제한과 미완료 경계를 전달하는지 대조했다.

| # | 실패 관점 | 확인 근거와 판정 |
|---:|---|---|
| 1 | 문서가 다른 구현 PR을 완료 처리하는가 | 변경 지점 모두 C11.1 `grid-auto-rows` implicit-row PR #1024를 가리킨다. 다른 C11 PR 텍스트는 변경하지 않았다. 통과. |
| 2 | 잘못된 merge SHA가 공개되는가 | `gh pr view 1024`의 `mergeCommit.oid`와 문서 SHA `90723452724656a985c91428eaabc75976d8ad45`가 일치한다. 통과. |
| 3 | 병합 시각이 UTC가 아닌 로컬 시각으로 잘못 적히는가 | GitHub 응답 `mergedAt=2026-11-02T03:01:28Z`와 문서 시각이 일치한다. 통과. |
| 4 | rebase merge 방식을 다른 방식으로 기록하는가 | 문서는 `Rebase and merge`로 표기한다. 저장소 필수 규칙 및 PR 상태와 일치한다. 통과. |
| 5 | GitHub Actions가 없는데 CI green이라고 말하는가 | PR check rollup이 빈 배열이며 문서에는 check run 부재와 CI 미주장을 기록한다. 통과. |
| 6 | 전역 완료 수가 제한 구현 때문에 증가하는가 | 모든 새 설명이 공식 완료 `72/213`을 유지한다. 통과. |
| 7 | CSS 구현 상위 C11 checkbox를 완료 처리하는가 | `STATUS.md` checkbox는 이 변경에서 수정하지 않았고 설명은 C11 미완료를 유지한다. 통과. |
| 8 | C11.1 checkbox 또는 전체 C11.1이 완료된 것으로 읽히는가 | 카드와 계획은 limited implementation slice로 한정하고 C11.1 전체 미완료를 명시한다. 통과. |
| 9 | C11.3.3 전체 완료로 확대되는가 | C11.3.3 미완료와 profile 경계를 C11 plan 및 roadmap에서 유지한다. 통과. |
| 10 | C11.4 text/intrinsic work와 합쳐 완료로 보이는가 | C11.4 미완료를 두 plan summary와 공개 카드에서 계속 표시한다. 통과. |
| 11 | 일반 CSS 지원/admission으로 확대되는가 | no-area 기본 Root HTML route와 단일 finite non-negative px auto row라는 한계를 그대로 쓴다. 통과. |
| 12 | arbitrary implicit track list가 지원 범위에 추가되는가 | 계획과 evidence가 단일 값만 말하고 여러 track list는 거부 경계로 남긴다. 통과. |
| 13 | implicit columns가 row feature로 혼동되는가 | 문서는 fixed implicit row만 다루며 implicit columns는 한계로 유지한다. 통과. |
| 14 | named-area 결합을 지원한다고 오해하는가 | no-area scope와 named-area rejection을 기존 plan/evidence에서 유지한다. 통과. |
| 15 | fixed explicit tracks, item shapes, placement 제한이 빠지는가 | 공개 요약은 bounded no-area row flow로 연결하고 상세 전용 계획에 fixed px, auto placement, fixed-size empty block 제한을 보존한다. 통과. |
| 16 | resource cap을 생략해 무제한 implicit growth를 암시하는가 | roadmap 및 C11 계획은 10,001-row cap 근거를 보존하며 전용 계획은 `GRID_MAX_TRACKS`를 설명한다. 통과. |
| 17 | Root HTML 기본 route가 opt-in fixture route로 바뀌어 읽히는가 | plan과 실행 README는 기본 `index.html` route를 명시하고 fixture-test flags가 꺼졌음을 유지한다. 통과. |
| 18 | Rust geometry 결과와 화면 캡처를 같은 증거로 부르는가 | exact frames는 Rust test 근거이며 Android/iOS screenshot은 시각 및 draw/present 자료로만 적는다. 통과. |
| 19 | PixelCopy 또는 Metal readback을 수행했다고 주장하는가 | 문서는 surface readback이 없다고 명시한다. 통과. |
| 20 | Android AVD를 물리기기로 표기하는가 | 문서는 Android API 37 ARM64 AVD로 정확히 표기한다. 통과. |
| 21 | iOS Simulator를 iPhone 실기기로 표기하는가 | 문서는 iPhone 17 Pro / iOS 26.2 Simulator를 시뮬레이터로 표기한다. 통과. |
| 22 | root frame, box 수 또는 `layout=ready` 관찰이 바뀌는가 | 기존 실행 근거의 viewport/root frame/6-box 관찰은 수정하지 않았고 같은 한계로 링크한다. 통과. |
| 23 | 테스트 통과 숫자가 기록과 다른가 | plan, status, roadmap에 기재한 74/74, 10/10, 26/26, 34/34 및 stale revision 1/1은 runtime validation log와 일치한다. 통과. |
| 24 | 제한된 테스트 결과가 전체 workspace 성공을 뜻하는가 | scoped test/Clippy 결과만 쓰고 전체 suite 또는 WPT 통과를 주장하지 않는다. 통과. |
| 25 | 계획 PR #1023과 구현 PR #1024의 역할을 뒤바꾸는가 | C11 plan은 #1023을 계획으로, #1024를 해당 범위 구현으로 구분한다. 통과. |
| 26 | 로컬 계획이 아직 PR open 상태로 남아 있는가 | 전용 계획 및 실행 README는 PR #1024의 병합 SHA와 날짜를 표기한다. 통과. |
| 27 | PR #1023 계획 자체도 구현 완료로 혼동되는가 | 문서는 #1023을 계획 산출물로 남기고 구현 증거는 #1024에 연결한다. 통과. |
| 28 | 새 문서 링크가 레포 경로와 맞지 않는가 | plan→evidence relative links 및 roadmap의 `/spinon/` route가 현재 tracked file 경로와 일치한다. 통과. |
| 29 | PR 링크 또는 roadmap 카드 anchor가 빠지거나 잘못되는가 | #1024 링크와 기존 `STATUS.html`/전용 계획/실행 근거 routes를 유지한다. 통과. |
| 30 | 문서 동기화가 수동 Pages 배포 또는 전체 지원 게시를 암시하는가 | 이번 diff는 ledger/계획/roadmap의 저장소 원본 갱신만 수행하며 공식 지원이나 Pages 게시를 주장하지 않는다. Tailnet은 이 PR 병합 후 선택 route로 별도 확인한다. 통과. |

## 결과

30개 관점에서 상태 모순을 찾지 못했다. 공식 전체 완료 `72/213`과 C11 계층의 미완료 checkbox는 유지한다. 이 문서는 PR #1024 구현/runtime 검토를 재사용하지 않고 병합 후 상태 문서 변경만 검토한다.
