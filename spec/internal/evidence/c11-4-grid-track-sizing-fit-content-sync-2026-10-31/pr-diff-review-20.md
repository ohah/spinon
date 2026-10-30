# C11.4 fit-content 상위 계획 상태 동기화 PR diff 적대적 검토 20개

**검토 대상:** `plan/c11-4-grid-track-sizing.md`의 fit-content growth-limit 및 automatic-minimum floor 항목
**원본 대조:** PR #764/#787, 세부 계획, runtime evidence
**결과:** 서로 다른 20개 관점에서 상태·수치·범위 오류를 발견하지 못했다.

| # | 적대 검토 관점 | 결과 |
|---:|---|---|
| 1 | #787이 구현 전 계획으로 표시되는가? | heading을 runtime slice와 병합 상태로 수정했다. |
| 2 | #787 merge SHA가 GitHub 병합 commit과 같은가? | `328e4e416b80012caac6297f1fabd556b85e2074`로 대조했다. |
| 3 | #787의 세 WPT assertion ID가 계획과 일치하는가? | `contentLargerThanArgument`, `smallArgument`, `largeArgument`를 확인했다. |
| 4 | 세 scene의 사용 row `[40,30,100]px`와 기대값 순서가 바뀌었는가? | 세부 plan의 assertion 표와 맞는다. |
| 5 | upstream harness 13/13을 Spinon WPT 통과로 확대하는가? | Chrome upstream 관측이라고 구분돼 있고 WPT suite 통과를 주장하지 않는다. |
| 6 | original WPT의 auto width와 60px normalized runtime root를 혼동하는가? | 기존 본문은 두 입력을 구분한다. |
| 7 | raw finite growth limit을 used row size와 혼동하는가? | 본문은 100px raw limit과 40/30px used rows를 구별한다. |
| 8 | Android 결과를 실기기나 hardware GPU 결과로 확대하는가? | Android API 37 ARM64 AVD 범위만 기록한다. |
| 9 | iOS 표면 결과를 compositor 이후 pixel 증거로 확대하는가? | compositor 제출 전 Metal 표본으로 한정한다. |
| 10 | #787의 three-scene simulator 결과가 해당 plan과 연결되는가? | 링크된 plan은 Android/iOS 각 3/3을 기록한다. |
| 11 | #764가 계획 단계로 남아 있는가? | heading을 runtime slice 및 PR 병합 상태로 수정했다. |
| 12 | #764 merge SHA가 PR 정보와 맞는가? | `db9b51c6ae72328c408b2a0ed15d602a90938d47`로 대조했다. |
| 13 | #764 UTC 시각과 제목 KST 날짜가 맞는가? | 14:30Z는 KST 2026-10-28이므로 날짜가 일치한다. |
| 14 | automatic-minimum control의 `[10,40]px`를 main `[10,100]px`와 혼동하는가? | 두 scene 결과를 기존 상세 plan에서 별도로 확인했다. |
| 15 | 두 scene이 일반 CSS admission 또는 전체 WPT 지원으로 과장되는가? | opt-in fixture와 단일 WPT assertion 범위다. |
| 16 | #764 platform result가 실기기로 잘못 표기되는가? | AVD와 Simulator 경계를 유지한다. |
| 17 | floor slice 표본이 glyph paint/hardware 표시를 증명한다고 쓰는가? | runtime evidence의 제한을 유지하고 새 pixel 주장을 추가하지 않았다. |
| 18 | C11.4 전체 checkbox나 `72/213`을 완료로 바꾸는가? | 상위 항목은 unchecked, 공식 수는 72/213으로 유지한다. |
| 19 | 계획·runtime 링크가 존재하지 않거나 section 순서가 어긋나는가? | 변경된 두 section의 모든 상대 링크 대상을 확인했다. |
| 20 | 자동 검증이나 CI 결과를 과장하는가? | 이 PR diff 검토와 링크·whitespace 검사만 기록하며 clean docs build/CI 통과를 주장하지 않는다. |

검증 범위는 `git diff --check`, GitHub PR metadata 대조, 변경한 상대 링크 대상 존재 확인이다. 이전 workspace-wide Rspress 실행은 미추적 evidence 문서에서만 dead link를 보고했으므로 clean checkout 빌드 결과로 사용하지 않는다.
