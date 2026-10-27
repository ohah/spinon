# C11.4 §11.7 병합 후 상태·로드맵 변경 검토 30개

**검토 대상:** 구현 PR #717 병합 이후의 집중 계획, 상위 C11.4 계획, `spec/STATUS.md`, 공개 `spec/public/roadmap.html`, 구현 evidence README 변경

**대조 기준:** GitHub PR #716/#717 병합 메타데이터, 현재 `main` HEAD, PR #717 focused test·fmt·Clippy 로그, `docs/project-rules.md`

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | #716 계획 PR을 구현 PR로 잘못 표기할 수 있음 | 계획 PR과 구현 PR을 각각 #716, #717로 구분했다. |
| 2 | PR #717 상태가 실제와 다를 수 있음 | GitHub에서 `MERGED`와 Rebase and merge를 확인했다. |
| 3 | 병합 시각의 시간대를 혼동할 수 있음 | GitHub `mergedAt`의 UTC `2026-10-27T21:58:53Z`를 그대로 기록했다. |
| 4 | 병합 SHA가 PR head commit과 혼동될 수 있음 | PR의 `mergeCommit.oid`인 `c29482aa8fc3ac39cdc342eae138c3ad10a59683`를 기록했다. |
| 5 | PR check rollup 빈 배열을 CI 성공으로 볼 수 있음 | rollup이 비어 있음을 적고 CI 미보고로 표현했다. |
| 6 | main required checks 부재를 PR green으로 확대할 수 있음 | required-check 통과를 주장하지 않았다. |
| 7 | 한 개 test를 전체 Grid 검증으로 확대할 수 있음 | 네 고정 definite empty-grid 입력의 test-only trace 범위로 한정했다. |
| 8 | assertion stage 순서가 검증된 결과에서 빠질 수 있음 | 공개 요약은 flexible expansion stage test임을 명시하고 상세 계획으로 연결한다. |
| 9 | 1fr:2fr만 검증한 것으로 축소될 수 있음 | 네 조건의 목적을 요약해 1fr:2fr, freeze/recompute, factor sum, 0fr floor를 구별한다. |
| 10 | focused test 개수와 filtered 수가 틀릴 수 있음 | README의 원시 실행 로그와 `1 passed · 292 filtered`에 맞춰 `1/1`로 썼다. |
| 11 | fmt가 전체 workspace 성공인 것처럼 넓혀질 수 있음 | 정확한 명령 `cargo fmt --all -- --check`만 적었다. |
| 12 | Clippy 범위가 workspace 전체로 오해될 수 있음 | `spinon-layout --all-targets` package 범위를 유지했다. |
| 13 | 일반 `cargo test` 전체가 성공한 것으로 오해될 수 있음 | focused test 외의 전체 테스트 통과를 주장하지 않았다. |
| 14 | `git diff --check`를 코드 test로 볼 수 있음 | 상태 서술에서 이를 별도 test 수치로 합치지 않았다. |
| 15 | `expected.json` 독립 oracle을 런타임 결과로 혼동할 수 있음 | CSS Grid 기대값·Rust trace 대조로 한정했다. |
| 16 | Chrome 관측을 규범 기대값으로 혼동할 수 있음 | 공개 링크와 기존 계획은 CSS Grid 산출값과 Chrome 관측을 분리한다. |
| 17 | WPT 전체 또는 Spinon WPT 통과를 암시할 수 있음 | WPT 미검증을 공개 roadmap와 공식 상태에 명시했다. |
| 18 | Android 실기기 검증을 암시할 수 있음 | Android simulator도 검증하지 않았다고 기록했다. |
| 19 | Android AVD 검증과 실기기 검증을 혼동할 수 있음 | 어느 쪽도 실행하지 않았다고 기록했다. |
| 20 | iOS Simulator 또는 iOS 실기기 검증을 암시할 수 있음 | iOS simulator 미검증을 명시했다. |
| 21 | 앱 runtime이나 pixel equality를 확인한 것처럼 보일 수 있음 | 앱 runtime 미검증을 공개 roadmap에 명시했다. |
| 22 | test-only 변경을 CSS 제품 지원으로 승격할 수 있음 | CSS admission과 production algorithm이 바뀌지 않았다고 유지했다. |
| 23 | C11.4 checkbox가 완료로 바뀔 수 있음 | 공식 checkbox를 수정하지 않았다. |
| 24 | 공식 완료 집계가 임의로 증가할 수 있음 | `72/213`을 모든 상태 기록에서 유지했다. |
| 25 | C11.4 하위 slice와 상위 C11.4를 동일시할 수 있음 | 상위 C11.4 미완료 문구를 유지했다. |
| 26 | 오래된 “PR 생성 준비 중” 상태가 남을 수 있음 | 집중 계획, 상위 계획, 상태 대장과 evidence를 병합 후 상태로 갱신했다. |
| 27 | roadmap의 GitHub 링크·코드 표기가 어긋날 수 있음 | 공개 항목은 PR #717 링크를 사용하고 `ExpandFlexibleTracks`는 HTML `<code>` 태그로 표시한다. |
| 28 | roadmap에서 연결한 evidence route가 없는 파일을 가리킬 수 있음 | 계획, 사전 비교, 실행 README, 구현 검토와 PR diff 검토 파일을 확인했다. |
| 29 | 새 문서 변경이 동작 화면을 제공하는 것처럼 오해될 수 있음 | 공개 roadmap는 앱 화면이 아니라 test-only Rust 변경이라고 명시한다. |
| 30 | 대량의 기존 untracked evidence가 이 변경에 섞일 수 있음 | 변경 대상은 계획 2개, 공식 상태, 공개 roadmap, 구현 README와 이 검토 기록으로 제한했다. |

**판정:** 초기 검토에서 공개 HTML의 함수명을 backtick 문자로 적은 표현을 발견해 `<code>ExpandFlexibleTracks</code>`로 고쳤다. 30개 관점을 최종 변경분에 다시 적용한 결과, 추가 수정이 필요한 상태·근거 오류는 없었다. 공식 구현 상태와 완료 집계는 미완료 범위로 유지한다.
