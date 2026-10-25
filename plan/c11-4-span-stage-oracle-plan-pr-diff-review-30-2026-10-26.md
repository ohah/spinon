# C11.4 span-stage 계획 PR diff 적대적 검토 · 30관점


**검토일:** 2026-10-26

**검토 대상:** [상위 C11.4 계획 변경](./c11-4-grid-track-sizing.md), [상세 span-stage 계획](./c11-4-span-stage-oracle.md), [계획 review pass01](./c11-4-span-stage-oracle-plan-review-pass01-20-2026-10-26.md), [pass02](./c11-4-span-stage-oracle-plan-review-pass02-20-2026-10-26.md), [최종 pass03](./c11-4-span-stage-oracle-plan-review-pass03-20-2026-10-26.md), [precomparison 자료](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md), [산출 스크립트](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/derive_expected.py), [expected.json](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/expected.json). 30-view report와 2-row addendum을 포함한 최종 payload는 parent plan 수정 1개와 신규 파일 9개다.

**판정 기준:** PASS는 파일 간 역할·수치·경로가 일치하고 상태/범위가 오해 없이 보존되는 경우다. PARTIAL은 review 이력이나 merge 후 탐색성 보완이 필요한 경우다.

**최초 diff 판정:** PASS 26 · PARTIAL 4. 제품 구현 또는 산술 오차 blocker는 찾지 못했다. 최종 parent-plan 수정에서 #8/#11을 재확인한 [addendum](./c11-4-span-stage-oracle-plan-pr-diff-review-addendum-2026-10-26.md)은 둘 다 PASS로 판정한다. 이에 따른 최종 유효 판정은 PASS 28 · PARTIAL 2이며, #13/#14는 복원하지 않기로 한 역사적 review record 한계로 남긴다. Rust tests, browser, simulator는 실행하지 않았다.

| # | PR diff 고유 실패 경로 | 판정 | 교차 파일 근거 및 판단 |
|---:|---|:---:|---|
| 1 | **예정 payload에 제품 코드 변경이 섞임** — 계획 review PR에서 runtime/test 구현이 조기 포함되는 위험 | PASS | 변경 목록은 상위 계획 문서 수정과 새 계획·review·산술 자료뿐이다. 기존 Rust helper와 HTML fixture는 tracked source로 참조되며 worktree 변경 목록에는 없다. 계획 PR과 후속 구현 PR 경계가 유지된다. |
| 2 | **문서상 범위와 실제 diff 범위 불일치** — 계획은 test-only라고 하나 FFI/runtime이 함께 바뀌는 위험 | PASS | parent entry는 이를 계획으로 표현하고 child plan은 향후 변경을 기존 test assertion에 한정한다([상위 계획 5–7행](./c11-4-grid-track-sizing.md#L5); [상세 계획 35–39행](./c11-4-span-stage-oracle.md#L35)). source 변경은 없다. |
| 3 | **이번 계획으로 C11.4를 완료 처리** — roadmap과 plan chronology 불일치 | PASS | 상위 계획의 새 entry, child plan, 공식 상태 행은 C11.4 미완료 및 72/213을 유지한다([상위 계획 3, 7행](./c11-4-grid-track-sizing.md#L3); [STATUS C11.4](../spec/STATUS.md#L273); [상세 계획 5, 54행](./c11-4-span-stage-oracle.md#L5)). |
| 4 | **공식 상태 대장을 바꿔 계획을 구현 완료처럼 게시** | PASS | spec/STATUS.md는 diff 대상이 아니다. C11.4 checkbox는 unchecked로 남고 parent entry도 공식 완료 수 증가를 주장하지 않는다(상위 계획 7행; STATUS 273행). |
| 5 | **parent 현재 날짜와 현재 공식 수치 불일치** | PASS | parent plan과 child plan은 모두 2026-10-26 및 72/213을 기록한다. 2026-10-23 등 과거 결과는 chronology에서 당시 수치로 구분한다(상위 계획 3행; 상세 계획 5행). |
| 6 | **상위 계획에서 구체 계획으로 이동 불가** — 새 요약이 다른 파일 또는 없는 파일을 링크 | PASS | parent entry의 상세 계획 링크는 이 span-stage plan을 가리키며 파일이 존재한다(상위 계획 7행; [상세 계획](./c11-4-span-stage-oracle.md)). |
| 7 | **새 evidence 링크가 이름 비슷한 기존 디렉터리를 가리킴** | PASS | parent와 child plan의 precomparison 링크가 새 evidence README로 해석된다. README는 같은 폴더의 산출 script와 expected JSON에 링크한다(상위 계획 7행; [README](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md#L24)). |
| 8 | **“변경 대상은 test assertion 한 파일뿐”이 현재 문서 PR의 파일 수를 뜻한다고 오독** | PARTIAL | 문단은 계획과 별도 구현 PR을 말하므로 의미는 추론 가능하지만, 이 PR 자체에는 test assertion 변경이 없다. “향후 구현 변경 대상” 또는 “구현 단계의 변경 대상”으로 바꾸면 계획 PR의 문서 diff와 분리된다(상위 계획 7행; 상세 계획 35, 45행). |
| 9 | **summary에서 구현 evidence가 이미 있다고 오인** — planned Rust trace test가 실행된 것으로 읽힘 | PASS | parent entry는 “계획했다”, “별도 구현 slice로 진행한다”고 쓴다. child plan도 코드/test/browser/app 실행이 없다고 명시한다(상위 계획 7행; child plan 5행). |
| 10 | **final plan review를 이전 PARTIAL 보고서로 잘못 연결** | PASS | 상위 계획은 “최종 계획 review 20개”로 pass03을 연결한다. pass03은 PASS20/PARTIAL0이며 별도 plan PR diff 관문을 명시한다(상위 계획 7행; [pass03](./c11-4-span-stage-oracle-plan-review-pass03-20-2026-10-26.md#L5)). |
| 11 | **상위 계획에서 이 PR diff review를 찾지 못함** — report 추가 후에도 review 증거가 한 방향으로만 연결 | PARTIAL | 현재 parent entry는 plan review pass03은 링크하지만 이 PR diff report는 링크하지 않는다. 이 파일을 추가한 뒤 parent entry에 링크하면 plan과 review 기록 사이의 탐색 경로가 완성된다. |
| 12 | **여러 plan review를 60개 관점으로 중복 합산** | PASS | 상위 계획은 최종 pass03만 “계획 review 20개”로 가리킨다. child plan도 plan review 20과 plan PR diff 30을 별도 gate로 두며 각 pass가 새로운 PR 관점 수를 더했다고 주장하지 않는다(상세 계획 44–47행; pass03 32–36행). |
| 13 | **이전 review 결과가 최신 결론으로 오인** | PARTIAL | pass01은 16/4, pass02는 19/1, pass03은 20/0이다. parent entry가 pass03을 최종으로 지정해 완화하지만 pass01/02 파일 자체에는 후속 pass로 대체됐다는 표시가 없다. 과거 두 report를 단독 열람하면 오래된 PARTIAL 결론을 현재 판정으로 받아들일 수 있다(pass01 32–34행; pass02 4–5행, 30–34행; pass03 5행).
| 14 | **과거 plan revision/hash 및 line citation 복원 제한** | PARTIAL | pass01의 baseline commit은 source code revision만 식별한다. 당시 plan 문서는 uncommitted였고 pass01의 L58–L63 표기 link는 현재 child plan의 #L5로 이동한다. pass02에도 당시 plan revision hash가 없다. parent는 과거 plan hash를 복원하지 않기로 했으므로, 이 기록은 현재 plan의 근거로 쓰지 않고 감사 이력의 제한으로 남긴다(pass01 4, 11행; pass02 header; child plan 현재 54행).
| 15 | **현재 pass03의 대상·link 자체가 다른 revision을 가리킴** | PASS | pass03 대상 링크는 현 child plan, evidence README, script, JSON을 가리키며 판정도 final scope의 serializer projection과 일치한다. 이는 pass01/02의 역사적 link 문제와 구별되는 최신 review 참조다(pass03 1–5행, 32–36행). |
| 16 | **새 계획과 precomparison input parameter 불일치** | PASS | 둘 다 네 minmax(0,max-content) column, 0 gap, definite 220px content width, A/B/C의 80/120/200px contribution을 명시한다(child plan 17행; README 7–10행). |
| 17 | **case ID가 계획/JSON/HTML 사이에 변형** | PASS | 네 ID는 competing-span2-forward, competing-span2-reversed, span2-before-span3, span3-before-span2로 일치한다. 기존 HTML의 section data-case/id도 같은 이름을 사용한다([HTML fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-multispan-ordering.html); expected.json results). |
| 18 | **논리 A/B/C item의 width와 placement를 잘못 연결** | PASS | 기존 HTML 및 Rust helper에서 A/B/C는 80px line 1→3, 120px line 2→4, 200px line 1→4다. README와 script가 같은 source-order matrix를 사용한다(HTML fixture; [Rust helper](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L261); README 8–10행; script 16–25행). |
| 19 | **CSS box model/root sizing을 수치 산술에서 누락** | PASS | 기존 CSS root는 width 220px, 네 minmax track, gap 0이고 item margin/padding/border가 0이다. Rust helper는 fixed 220×40 root와 같은 item widths/heights를 구성한다(HTML fixture; Rust helper 76–129행; README 7–8행). |
| 20 | **row scene 변화가 새 row-axis oracle로 과장** | PASS | HTML/Rust case는 기존 네 장면을 유지한다. child plan은 row-axis stage를 제외하고 definite Columns pass만 검사한다. 추가 implicit row는 이 계획에서 별도 적합성 주장 대상이 아니다(child plan 27, 36, 39행; README 8, 30행). |
| 21 | **Chrome used value를 새 normative expected로 복사** | PASS | plan/README는 Chrome 155 값을 final geometry 확인에만 사용하고 normative ideal thirds와 구분한다. 새 script는 이 Chrome observation을 입력으로 읽지 않는다(child plan 29–31행; [Chrome observation](../spec/internal/evidence/c11-4-multispan-ordering-physical-android-2026-10-21/chrome155-observation.json); script 9–25행). |
| 22 | **새 screenshot/app 실행이 이번 계획 증거라고 오인** | PASS | 새 evidence 폴더는 README, script, JSON으로 한정되며 binary screenshot/log를 추가하지 않는다. plan과 README는 새 browser capture 및 Spinon 실행 근거가 아니라고 밝힌다(child plan 5, 39행; README 30, 34행). |
| 23 | **표준 판본과 수치 근거 사이 provenance 단절** | PASS | plan과 README는 동일한 CSS Grid 2025 CRD §11.5.1/§11.6 URL을 쓰고, README가 source HTML SHA-256을 기록한다(child plan 15행; README 3행). |
| 24 | **trace serializer 제한이 문서와 구현에서 다르게 표현** | PASS | plan 및 README는 serializer가 finite growth limit만 Some으로 내보내며 다른 비유한값은 None으로 합친다고 설명한다. Taffy source의 is_finite().then_some(...) 구현과 일치한다([child plan 37행](./c11-4-span-stage-oracle.md#L37); [README 22행](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md#L22); [Taffy serializer](../vendor/taffy/src/compute/grid/mod.rs#L1615)). |
| 25 | **산출 script 실행 경로가 JSON 제출본과 분리** | PASS | child plan은 저장소 root에서 실행할 script 전체 경로를 제시하고 README는 해당 폴더에서의 script 명을 쓴다. script는 표준 Python Fraction/JSON만 사용한다(child plan 46행; README 24, 34행; script 9–10행). |
| 26 | **JSON fractional 값이 decimal로 반올림되거나 null이 값 부재로 취급** | PASS | script는 정수는 JSON number, 비정수는 numerator/denominator 문자열, expected unbounded는 null로 출력한다. README는 Fraction과 null의 expected 의미 및 trace projection 제한을 설명한다. Rust test가 JSON을 직접 읽는다고 주장하지 않아 parser 의존을 과장하지 않는다(script 29–35, 70–75, 102–127행; README 17–24행). |
| 27 | **control 배열 순서와 expected result 순서 혼동** | PASS | script CASES, expected.json results, README 및 HTML의 four-case sequence가 같은 source order로 대응한다. 각 JSON 결과에도 case id와 source_order가 따로 남는다(script 21–26행; expected.json results; README 9행). |
| 28 | **상대 문서 링크가 새 PR 폴더 구조에서 깨짐** | PASS | 신규 계획/evidence/review Markdown의 local relative targets를 확인했다. 새 링크는 존재하며 STATUS #L273은 C11.4 unchecked row다. audit에서 나온 /spinon/ web-site aliases와 기존 parent-plan broken link는 이번 diff lines 밖이다(parent plan 변경 [3–7행](./c11-4-grid-track-sizing.md#L3)). |
| 29 | **plan/PR/runtime review gate를 하나의 review count로 계산** | PASS | child plan은 계획 review 20 + 계획 PR diff 30을 구현 전에 분리하고, 구현 후 runtime review 20 + 실제 PR diff 30을 따로 둔다. 이번 report는 계획 20관점 표를 반복하지 않고 예정 diff 전체의 파일 연결/merge 위험을 본다(child plan 41–54행; pass03 20, 34행). |
| 30 | **merge 후 계획 slice가 공식 C11.4 완료로 승격** | PASS | parent plan은 이 기록을 C11.4 chronology에 추가하면서 공식 상태를 유지한다. 상세 plan은 후속 구현, focused Rust checks, 새 runtime 20관점, 실제 PR diff 30관점 뒤 해당 test slice만 완료 판정하도록 한다(parent 5–7행; child plan 41–54행). |

## 최종 addendum 상태

최초 diff review에서 parent 문구(#8)와 PR diff review 링크(#11)는 PARTIAL이었다. parent plan 수정 후 두 항목을 [별도 addendum](./c11-4-span-stage-oracle-plan-pr-diff-review-addendum-2026-10-26.md)에서 재확인해 PASS로 바꿨다. #13/#14는 과거 review record의 superseded 표시와 정확한 plan revision 복원 제한으로 남는다. parent는 과거 revision hash 복원을 하지 않기로 했으므로 이 두 항목은 허용된 이력 한계이며 구현 blocker가 아니다.

이 검토에서는 code tests, browser, simulator를 실행하지 않았다.
