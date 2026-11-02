# PR #1042 병합 후 roadmap·명세 동기화 실패 관점 검토 30개

대상: 구현 PR #1042 병합 뒤의 계획·상태 대장·내부 계약·실행 evidence 동기화 diff. 구현 동작 review와 PR #1042의 30개 diff 검토를 재사용하지 않고, 병합 사실·공식 상태·근거 연결 실패를 따로 확인한다.

| # | 실패 관점 | 판정과 확인 |
|---:|---|---|
| 1 | 계획 PR #1041을 미병합으로 남기는가 | 2026-11-02T06:56:07Z 및 merge SHA `1d838ba...`를 기록했다. PASS. |
| 2 | 구현 PR #1042를 여전히 open으로 표시하는가 | 계획·상태·내부 계약을 merged로 갱신했다. PASS. |
| 3 | PR #1042 merge 시간을 계획 PR 시간과 혼동하는가 | `07:49:42Z`를 별도 표기한다. PASS. |
| 4 | GitHub merge tip과 기능 소스 commit을 같은 SHA로 오기하는가 | merge tip `6f4d4a7...`과 code commit `1f81dcb...`을 나눠 기록한다. PASS. |
| 5 | Rebase merge 후 새 코드 commit을 확인하지 않는가 | `main`의 `1f81dcb...` tree가 source manifest tree와 일치한다. PASS. |
| 6 | PR 브랜치의 사전 테스트 commit을 병합된 commit이라고 주장하는가 | `source_commit`은 tested candidate로 유지하고 `merged_source_commit`을 별도로 둔다. PASS. |
| 7 | manifest의 source tree와 병합 코드 tree가 달라지는가 | 두 tree가 모두 `7a5ebed...`다. PASS. |
| 8 | merge base가 다른 main revision으로 바뀌는가 | PR #1042 base `1d838ba...`와 기록값을 대조했다. PASS. |
| 9 | merge tip과 현재 main tip이 다른가 | remote `main` 및 PR mergeCommit이 `6f4d4a7...`로 일치했다. PASS. |
| 10 | fixture hash가 merge 후 바뀌거나 stale한가 | source fixture SHA `963d5b...`와 precomparison JSON을 대조했다. PASS. |
| 11 | Android APK hash를 다른 pass 산출물로 대체하는가 | pass03 APK hash가 runtime observation 및 source manifest와 일치한다. PASS. |
| 12 | iOS executable hash를 다른 build로 대체하는가 | pass03 executable hash가 두 manifest에 일치한다. PASS. |
| 13 | Android/iOS screenshot hash를 다른 pass로 연결하는가 | pass03 PNG hashes와 JSON·README paths가 일치한다. PASS. |
| 14 | PR 본문 이미지 링크는 깨졌는데 병합 기록은 성공이라 하는가 | Agent Browser에서 두 image가 실제 로드됐고 자연 크기 1080×2400, 1206×2622를 확인했다. PASS. |
| 15 | Browser 검증용 Agent Browser 프로세스를 남기는가 | session close 후 process scan에서 0개를 확인했다. PASS. |
| 16 | Simulator screenshot을 GPU pixel equality로 확대 해석하는가 | PixelCopy·Metal drawable·compositor-after 한계를 계속 표시한다. PASS. |
| 17 | AVD를 Android 실기기 검증으로 표시하는가 | Android API 37 ARM64 AVD·SwiftShader라고 명시한다. PASS. |
| 18 | iOS Simulator를 실기기 또는 compositor-after 검증으로 표시하는가 | iPhone 17 Pro iOS 26.2 Simulator 및 draw/present 범위로 제한한다. PASS. |
| 19 | 제한적 Root HTML slice를 C11.1 완료로 올리는가 | C11.1 상위 체크는 미완료로 유지한다. PASS. |
| 20 | 한 slice 때문에 C11 상위 완료를 표시하는가 | C11 parent checkbox는 그대로 미완료다. PASS. |
| 21 | 공식 집계 72/213을 임의로 증가시키는가 | 상태 대장과 문서는 `72/213` 그대로 유지한다. PASS. |
| 22 | `grid-auto-columns`·임의 track grammar도 지원한다고 넓히는가 | 기존 fixed-px row-list scope와 거부 경계를 유지한다. PASS. |
| 23 | WPT 또는 일반 CSS Grid 적합성을 주장하는가 | 명세와 병합 기록에서 WPT 미검증을 유지한다. PASS. |
| 24 | internal contract 숫자 버전을 구현만으로 올리는가 | contract `0.1.0`을 유지한다. PASS. |
| 25 | 내부 계약과 전용 계획 간 상대 링크가 깨지는가 | plan·evidence 링크를 실제 repository paths와 대조했다. PASS. |
| 26 | C11 CSS overview에 오래된 open/candidate 상태가 남는가 | 구현을 병합된 제한 구현으로 바꾸고 PR 번호·merge SHA를 연결했다. PASS. |
| 27 | Root HTML runtime plan에 PR open 상태가 남는가 | merged state, runtime counts, 한계와 evidence link를 갱신했다. PASS. |
| 28 | status ledger가 이전 plan merge SHA를 구현 merge SHA로 사용한다 | #1041과 #1042의 서로 다른 merge records를 기록한다. PASS. |
| 29 | 비어 있는 GitHub checks를 CI 성공으로 표현하는가 | PR status rollup empty 및 main required checks disabled를 써 CI 통과를 주장하지 않는다. PASS. |
| 30 | 동기화 변경에만 필요한 docs update가 제품 동작을 바꾸거나 evidence를 지우는가 | 문서·manifest·merge record만 변경하고 기존 logs·screenshots·failure artifacts를 보존한다. PASS. |

**판정:** 실제 merge 이후의 main/PR state와 artifact hashes를 대조해 30개 동기화 실패 관점을 확인했다. C11.1/C11 상위 상태와 `72/213`은 바꾸지 않는다. PR check가 보고되지 않았으므로 CI 통과로 기록하지 않는다.
