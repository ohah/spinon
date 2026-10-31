# C11.3.3 typed area auto-track 병합 후 상태 문서 검토 30개

**검토 대상:** PR #911 병합 뒤 `spec/STATUS.md`, 내부 계약 0059, C11 계획, 전용 제한 slice 계획, 실행 근거의 상태 정합성.

**기준 사실:** PR #911은 `main` 대상 `Rebase and merge`로 2026-10-31T08:06:44Z 병합됐고 merge SHA는 `eeba00fa9faf05ffcc8160f8adbb220fe90204db`다. `gh pr checks 911 --required`에는 보고된 check이 없으며 main의 required status checks도 설정되어 있지 않다.

**결과:** 문서 내부 모순 0 · PASS 29 · PARTIAL 1. PARTIAL은 문서 저장소와 별도로 확인해야 하는 Tailnet 미리보기 갱신 경계다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | PR head SHA가 merge commit SHA로 기록된다 | PASS — `910d2099…`는 head, `eeba00fa…`는 merge 결과로 구분한다. |
| 2 | 잘못된 pull request number가 링크된다 | PASS — 변경된 문서는 GitHub PR #911을 가리킨다. |
| 3 | `mergedAt` 대신 PR 생성 시각이 기록된다 | PASS — GitHub의 `2026-10-31T08:06:44Z`를 기록한다. |
| 4 | 병합 방식이 repository 규칙과 다르다 | PASS — `Rebase and merge`로 명시한다. |
| 5 | required check 부재가 통과로 둔갑한다 | PASS — 보고된 check 없음과 required checks 비설정을 각각 적는다. |
| 6 | `mergeStateStatus: UNKNOWN` 사후 응답을 병합 실패로 오인한다 | PASS — `state=MERGED`, `mergedAt`, `mergeCommit`을 최종 증거로 기록한다. |
| 7 | 병합 완료 뒤 “PR 전” 문구가 남는다 | PASS — 변경된 C11 문서에서 병합 완료/제한 slice 표현을 사용한다. |
| 8 | 전용 계획이 PR 생성·병합을 미완료로 표시한다 | PASS — PR #911의 병합 사실과 check 경계를 반영한다. |
| 9 | 공식 상태 대장의 C11 checkbox가 병합 때문에 체크된다 | PASS — C11 상위는 미완료 그대로 둔다. |
| 10 | C11.3.3 parent가 limited slice 병합만으로 완료된다 | PASS — 상위/하위 완료 checkbox를 유지하지 않는다. |
| 11 | C11.4가 C11.4 typed value reuse 때문에 완료로 표시된다 | PASS — C11.4 전체 미완료와 전체 sizing 교차 범위 잔여를 유지한다. |
| 12 | official `72/213` 집계가 PR merge만으로 증가한다 | PASS — 각 변경 문서에서 `72/213`을 유지한다. |
| 13 | 39 scene·126 node가 공식 완료 항목 수로 해석된다 | PASS — matrix count와 공식 roadmap counter를 구분한다. |
| 14 | 제한 slice를 일반 CSS author admission으로 확대한다 | PASS — public CSS admission 미검증 경계를 명시한다. |
| 15 | fixture matrix 성공을 전체 Grid 적합성으로 확대한다 | PASS — Spinon WPT runner와 전체 WPT 완료를 주장하지 않는다. |
| 16 | Android AVD 결과가 실기기/hardware GPU로 표현된다 | PASS — API 37 ARM64 AVD와 SwiftShader software renderer로 한정한다. |
| 17 | iOS Metal drawable sample이 compositor 후 pixel로 표현된다 | PASS — compositor 제출 전 readback 한계를 유지한다. |
| 18 | Android PixelCopy의 7/7 표본이 새 named-line scene을 포함한다고 오인된다 | PASS — 표본은 typed area two scenes라고 한정하고 named-line phase test는 Rust layout test로 분리한다. |
| 19 | fixed overflow control이 typed scene overflow proof로 계산된다 | PASS — 기존 fixed-track control임을 따로 기록한다. |
| 20 | `fit-content()` cap이 검증됐다고 표현된다 | PASS — min/max contribution을 분리한 cap fixture가 남았다고 명시한다. |
| 21 | 기본 static archive 검사 범위가 최종 앱 전체 artifact로 확대된다 | PASS — Android APK/iOS IPA 전체 exclusion은 별도 범위로 둔다. |
| 22 | PR 화면 캡처가 PixelCopy/Metal 표본과 혼동된다 | PASS — 화면 링크는 실행 화면, 표본 결과는 별도 실행 근거로 구분한다. |
| 23 | evidence README가 실행 당시 dirty-tree source를 현재 상태로 표현한다 | PASS — source base와 해당 diff의 후속 PR 병합 사실을 과거 실행 맥락으로 기록한다. |
| 24 | merge 후 docs에서 internal contract version이 오르거나 달라진다 | PASS — 출시 전 `0059` 숫자 버전 `0.1.0`을 유지한다. |
| 25 | C11.3 plan row의 PR #911 상태가 C11.4 row와 충돌한다 | PASS — 두 row 모두 제한된 typed-area slice와 PR #911 병합을 기록한다. |
| 26 | C11.4의 PR #893/#905/#909 범위가 C11 전체 완료로 바뀐다 | PASS — 모두 제한 fixture/test slices로 남고 C11.4 parent는 미완료다. |
| 27 | 내부 계약, 전용 계획, 실행 근거 링크가 서로 다른 scope를 설명한다 | PASS — 모든 문서가 같은 제한 slice와 남은 한계를 가리킨다. |
| 28 | docs PR 근거가 implementation PR의 merge proof를 대체한다 | PASS — 정확한 PR/merge SHA와 local evidence를 각각 연결한다. |
| 29 | 문서 수정만으로 runtime/browser evidence가 새로 생성된 것처럼 보인다 | PASS — 시뮬레이터와 test 결과는 기존 PR #911 evidence만 참조한다. |
| 30 | GitHub main merge만으로 Tailnet preview가 갱신됐다고 오인된다 | PARTIAL — Tailnet은 별도 preview source이며, 해당 서버의 갱신·served hash는 이 docs change와 분리해 확인한다. |

이 문서 검토는 PR #911의 구현 PR diff 검토 30개를 대체하지 않는다. 이 follow-up PR은 구현 동작이나 공식 완료 checkbox를 변경하지 않는다.
