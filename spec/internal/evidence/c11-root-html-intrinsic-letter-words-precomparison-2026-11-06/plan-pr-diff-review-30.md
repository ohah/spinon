# C11.4 Root HTML ASCII word intrinsic sizing 계획 PR diff 적대 검토 30개

**검토 대상:** 새 계획, C11 plan index, C11.4 parent plan, 10-case fixture, browser observation, independent expected data와 capture/derive scripts.

**검토 범위:** 계획 PR의 실제 diff. 구현/runtime 검토 아님.

| # | PR diff 관점 | 실제 변경 확인 | 결과 |
|---:|---|---|---|
| 1 | 새 계획이 올바른 C11 단계에 귀속됐는가 | C11.4 intrinsic text/min-content 후속으로 연결 | 통과 |
| 2 | 상위 C11 plan index에서 검색되는가 | C11.4 행에서 새 plan으로 링크 | 통과 |
| 3 | Track sizing master plan이 진행 순서를 드러내는가 | Stage section 뒤에 계획 전 상태와 기존 `A` 경계 기록 | 통과 |
| 4 | plan PR이 공식 완료 체크를 바꾸는가 | `spec/STATUS.md` 미수정, checkbox 없음 | 통과 |
| 5 | 계획이 이미 구현된 PR #1205와 다른가 | #1205는 `A` 단독 token; 새 계획은 ASCII alphabetic word runs | 통과 |
| 6 | 과거 사전 비교 파일을 덮어썼는가 | 새 fixture/evidence directory 사용, 기존 baseline 보존 | 통과 |
| 7 | Chrome user agent와 version이 재현 가능한가 | observation에 HeadlessChrome 156.0.0.0과 viewport/DPR 기록 | 통과 |
| 8 | browser profile/OS metadata가 과장되지 않았는가 | HeadlessChrome macOS ARM, DPR 1만 기록 | 통과 |
| 9 | observation이 Spinon runtime 결과로 표기됐는가 | README와 expected JSON에서 browser oracle로 분리 | 통과 |
| 10 | screenshot이 실제 capture page에 대응하는가 | 10개 scene이 보이는 Agent Browser screenshot을 확인 | 통과 |
| 11 | capture script가 page structure에 묶여 있는가 | `.case`, `.grid`, `.word-text` 선택자와 case ID를 같은 fixture에서 추출 | 통과 |
| 12 | capture가 text-line observation을 생략했는가 | Range client rect와 line count, rows를 기록 | 통과 |
| 13 | generic font metric을 다른 CSS 결과로 대체했는가 | Canvas `monospace` word/space advances를 별도 input으로 저장 | 통과 |
| 14 | independent expected가 Chrome final tracks를 입력으로 썼는가 | derive script는 source tokens와 font advances로 계산 후 결과를 비교 | 통과 |
| 15 | `min-content`와 `max-content` contribution을 혼동했는가 | token 최대 advance와 전체 word/space 합산을 분리 | 통과 |
| 16 | definite auto column sizing 식이 incomplete인가 | `max(min-content,min(40px,max-content))`를 명시 | 통과 |
| 17 | automatic minimum이 container에 의해 clamp되는가 | 8-letter word의 48.171875px track을 기록 | 통과 |
| 18 | line-breaking input whitespace가 vague한가 | normal whitespace, ASCII letters, `lang=en`을 고정 | 통과 |
| 19 | punctuation, digits, Unicode가 의도치 않게 허용되는가 | 계획과 README에 제외를 명시 | 통과 |
| 20 | source order 검증이 누락됐는가 | min-content/auto 모두 reverse-order case가 있다 | 통과 |
| 21 | scene count와 expected matrix가 맞는가 | 8 기본 scene + 2 reverse control = 10; derivation PASS 10 | 통과 |
| 22 | root-relative item frames가 page position에 의존하는가 | expected data가 grid origin을 빼서 상대 frame을 저장 | 통과 |
| 23 | source hash가 evidence와 분리되어 stale 될 수 있는가 | README에 fixture·capture·derive·observation·expected·PNG hash 기록 | 통과 |
| 24 | expected derivation을 다시 실행할 수 있는가 | `python3 derive_expected.py`가 10개 scene 비교 후 성공 | 통과 |
| 25 | generic monospace metrics가 platform-independent라고 단정했는가 | simulator geometry를 별도로 acceptance에 두고 오차 초과 시 성공 처리 금지 | 통과 |
| 26 | outside-border paint가 acceptance에서 빠졌는가 | Android PixelCopy와 iOS Metal drawable의 grid-relative sample을 요구 | 통과 |
| 27 | pixel sample 좌표가 frame에서 해석 가능한가 | grid-relative x=42px와 text row center로 고정 | 통과 |
| 28 | 기본 HTML/product 경로에 새 FFI가 필요한가 | 계획은 기존 root route와 typed adapter만 변경하고 새 FFI를 추가하지 않음 | 통과 |
| 29 | C11.4 stage 완료를 새 plan으로 승격하는가 | master plan이 계획 전이며 stage gate 미변경이라고 명시 | 통과 |
| 30 | 계정/secret, terminal browser 또는 user data가 evidence에 포함되는가 | 로컬 fixture screenshot과 공개 browser metadata만 기록; Agent Browser session 종료 후 HeadlessChrome 없음 확인 | 통과 |

**결과:** 계획 PR diff 검토 30/30 통과. 구현 리뷰와 실제 구현 PR diff review는 별도 수행한다.
