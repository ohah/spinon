# C11.4 `grid-auto-fit-columns-001.html` 계획 PR #694 변경 diff 적대 검토 20개

검토 대상은 PR #694의 실제 base `14e57811` 대비 12개 파일 diff와 연결된 WPT·Chrome·규범 자료다. 계획 자체의 사전 실패 검토와 분리해, 문서·입력·증거·PR 표시 사이의 실제 불일치 가능성을 확인했다.

| # | 적대적 관점 | diff에서 확인한 근거·조치 | 판정 |
|---:|---|---|---|
| 1 | 계획이 WPT 일부를 골라 전체 범위처럼 보이게 하는가? | Scene family 범위가 index 1–35를 연속으로 덮는다. expected 입력 35 root·37 direct item을 재생성해 개수 대조했다. | 통과 |
| 2 | 문서와 oracle이 다른 WPT revision을 참조하는가? | Plan, README, expected JSON과 source manifest 모두 revision `c999c58338ee1d223df5ad62e48be1202ced0d37`을 사용한다. | 통과 |
| 3 | 고정 source/helper hash가 저장소의 WPT checkout과 다른가? | manifest의 5개 원본 파일 SHA-256을 현재 pinned checkout에서 재계산해 모두 일치했다. repeat 계산기는 WPT HTML 및 `grid.css` hash도 검증한다. | 통과 |
| 4 | WPT geometry 기대값이 Chrome observation에 종속되는가? | `derive_expected.py` 재생성 결과가 checked-in `expected.json`과 byte-identical이다. 전체 37 item에서 Chrome `offset*`와 WPT-authored 값 차이는 최대 0 CSS px였다. | 통과 |
| 5 | Chrome 결과가 Spinon 구현 또는 적합성 통과로 표현되는가? | WPT harness 원문은 `Found 35 tests / 35 Pass`다. README·plan·PR 본문에서 Chrome baseline과 미실행 Spinon 구현을 반복 분리했다. | 통과 |
| 6 | 반복 횟수 표가 Chrome CSSOM에서 역산되거나 source CSS가 바뀌어도 남는가? | `derive_repeat_expectations.py`는 authored track-list declarations, root width, gap, padding, helper hash를 검증하며 분수 산술로 독립 계산한다. 출력 35 case는 checked-in JSON과 byte-identical이다. | 통과 |
| 7 | CSS Grid repeat sizing에서 border를 content width로 잘못 세는가? | WPT의 200px CSS width는 content box이며 2px border는 바깥이다. base Chrome root frame 204px와 문서 기대값이 맞는다. | 통과 |
| 8 | case 2 padding이 repeat basis를 174px로 축소하는가? | Chrome observation은 outer width 230px, item offset 13px·width 180px을 기록한다. source `width:200px` content box에서 30%는 60px이고 count는 3이다. 초안의 잘못된 174px 가정은 수정되어 최종 diff에는 없다. | 통과 |
| 9 | definite `minmax()` max<min이나 no-fit positive fallback을 누락하는가? | case 4–5는 max를 min 30px로 floor한다. 독립 table은 case 29–31 및 33에서 첫 repetition도 200px 안에 맞지 않아 count 1 강제 규칙을 기록한다. | 통과 |
| 10 | fixed prefix/suffix와 gap이 repeat count에 빠지는가? | case 9–14, 15–20, 21–25, 26–33의 count 입력에 prefix/suffix, integer fixed repeat와 모든 inter-track gap이 각각 들어간다. case 32 gap 0, case 33 gap 20이다. | 통과 |
| 11 | `fr` minimum을 최종 track width로 착각하거나 `2em` basis를 고정하지 않는가? | repeat count는 `minmax(50px,1fr)`의 definite 50px minimum을 쓰고 flex 결과는 별도다. case 32의 비어 있지 않은 2em track은 CSSOM에서 32px이며 case 33의 2em track은 auto-fit collapse로 0px이다. em 기준은 case 32에만 고정한다. | 통과 |
| 12 | auto-fit empty 판단 순서나 spanning occupancy가 모호한가? | Plan은 repeat count 뒤 placement, span 점유 판정, 이후 empty-track/gutter collapse 순서를 검증하도록 고정한다. auto-fill case 3–5는 collapse 기준으로 처리하지 않는다. | 통과 |
| 13 | repeated/fixed 인접 line names나 occurrence placement 입력이 빠지는가? | WPT source의 repeated `foo`/`start`/`bar` groups, numeric·negative·named placement와 missing-name fallback이 전체 source-authored scene 목록에 있다. matcher는 line index별 occurrence를 검증하도록 계획한다. | 통과 |
| 14 | implicit `157px` columns가 auto-repeat 수에 섞이는가? | Chrome CSSOM extra tracks는 index 7(1), 8(1), 11(1), 14(2), 19(1)이다. README와 plan은 이들을 explicit repeat count와 분리한다. | 통과 |
| 15 | overflow matrix가 root 밖으로 나가는 WPT 입력을 놓치는가? | WPT expected x+width로 다시 계산한 overflow index는 정확히 7, 8, 11, 14, 19, 31이며 plan의 Android/iOS readback 목록과 같다. | 통과 |
| 16 | border 바깥 item 관측을 root frame·screenshot만으로 대신하는가? | 계획은 여섯 장면 모두 border, border 바깥 item 내부, item 끝 다음 background, 별도 control을 PixelCopy/Metal drawable에서 표본화하고 RGBA·좌표 manifest를 남긴다. | 통과 |
| 17 | source helper의 넓은 허용치를 앱 허용치로 가져오는가? | WPT helper 1 CSS px 실패 경계와 계획의 정수 frame 0.02 CSS px 허용치가 구분되어 있다. used track count, lines, collapse와 surface RGBA는 exact 기준이다. | 통과 |
| 18 | 고정 fixture 계획이 제품 CSS 지원이나 artifact exclusion으로 오해되는가? | 내부 opt-in 경로, exact allowlist/rejection, static FFI positive·negative control을 계획했다. APK/IPA 전체 제외나 일반 CSS admission은 증명하지 않는다고 명시한다. | 통과 |
| 19 | 공식 완료 수나 상위 계획의 우선순위를 바꾸는가? | STATUS의 C11.4 checkbox와 `72/213`은 유지한다. 상위 계획은 grid-auto-fit 전체 WPT slice를 현재 다음 작업으로 명시하고 별도 fit-content 후보와 구분한다. 기존 잘못된 evidence link도 실제 파일명으로 고쳤다. | 통과 |
| 20 | PR 화면·metadata·검증 상태가 구현 증거로 과장되는가? | PR에는 `planning`·`area: docs` 라벨이 있다. Agent Browser에서 baseline image의 natural size 1280×3348을 확인한 뒤 세션을 닫았고, checks는 보고되지 않았다. 화면은 Chrome WPT임을 명시하며 simulator 검증은 미실행으로 적었다. | 통과 |

**계획 PR diff 적대 검토:** GitHub PR #694의 실제 변경 파일과 diff를 대상으로 20/20 통과, 미해결 0. GitHub에 status check은 보고되지 않았다. 이 기록은 계획 검토 20개와 후속 구현/runtime 검토 20개를 대체하지 않는다.
