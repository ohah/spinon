# C11.1 `fr` implicit columns 계획 PR diff 검토 30개

계획 자체의 20개 검토와 분리해 계획 PR 변경 파일을 30개 failure 관점에서 검토했다. 이 검토는 계획 PR의 문서·기준 fixture·상태 동기화만 다루며, 아직 작성하지 않은 구현 diff/runtime을 승인하지 않는다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | C11 parent checkbox가 실수로 완료 처리되는가 | 통과 — `spec/STATUS.md`의 C11 checkbox는 미완료다. |
| 2 | 공식 완료 수가 `72/213`에서 변하는가 | 통과 — 변경 전후 문서 모두 `72/213`을 유지한다. |
| 3 | 계획 PR이 C11.1 구현·지원 승격으로 서술되는가 | 통과 — 계획 상태와 구현 미시작을 명시했다. |
| 4 | 기존 #1059 계획이 merge-open으로 남는가 | 통과 — PR #1059의 merged 시간과 merge SHA를 전용 계획에 기록했다. |
| 5 | #1059 실행 근거 README가 merge 대기로 남는가 | 통과 — merge 결과를 갱신하고 당시 GitHub check 부재를 보존했다. |
| 6 | 상위 C11 계획이 fixed-px implicit columns를 미구현으로 보는가 | 통과 — #1059 Rebase and merge 기록으로 교체했다. |
| 7 | 상위 C11 계획에 #1063 percentage implementation이 누락되는가 | 통과 — merged PR과 범위/evidence 링크를 반영했다. |
| 8 | 새 `fr` 후보 링크가 실제 계획 파일과 연결되는가 | 통과 — master plan과 status note가 같은 새 계획을 가리킨다. |
| 9 | status note가 새 계획만으로 기능을 지원한다고 오인시키는가 | 통과 — plan-only이며 구현/완료 근거가 아님을 명시했다. |
| 10 | fixture가 앱 runtime source와 같은 것으로 오인되는가 | 통과 — 사전 비교 전용 Chrome HTML이며 evidence README가 역할을 분리한다. |
| 11 | 사전 비교 브라우저가 터미널 브라우저 또는 Spinon app인가 | 통과 — Agent Browser의 `HeadlessChrome/156.0.0.0` 관측이다. |
| 12 | Chrome observation에서 viewport나 DPR이 빠지는가 | 통과 — raw JSON에 `1280×633`, DPR 1이 들어 있다. |
| 13 | screenshot이 기록된 raw observation과 다른 source에서 오는가 | 통과 — 두 산출물은 같은 고정 file fixture에서 캡처했다. |
| 14 | source/observation/expected/screenshot hash가 문서와 불일치하는가 | 통과 — 각 artifact SHA-256을 별도 재계산해 README 값과 대조했다. |
| 15 | independent expected가 browser observation에서 역산되는가 | 통과 — definite width·track breadth·gap·factor로 산술을 별도 서술하고 browser values는 별도 기록한다. |
| 16 | list cycle이 explicit columns부터 시작한다고 잘못 가정하는가 | 통과 — 네/여섯 implicit tracks에 첫 implicit index부터 순환한다. |
| 17 | fixed px와 fr 혼합에서 fixed breadth가 빠지는가 | 통과 — `10px 1fr`의 residual 계산을 plan과 expected에 기록했다. |
| 18 | mixed px/%/fr에서 percentage basis에 gap을 빼는가 | 통과 — 300px basis의 10%를 30px로 두고 gap은 flex leftover에서만 차감한다. |
| 19 | factor sum below one이 전체 빈 공간을 채운다고 기록되는가 | 통과 — 두 `0.25fr` track에 각 20px와 trailing 40px를 기록했다. |
| 20 | `0fr`를 0px로 고정해 auto minimum을 잃는가 | 통과 — 10px item base를 포함한 `[10,70]px` 관측과 독립 계산이다. |
| 21 | 고 factor sum f32 overflow가 경로별로 누락되는가 | 통과 — 네 `1e38fr` track의 Taffy f64 accumulator regression을 요구한다. |
| 22 | fractional coordinate 차이를 code mismatch로 오인하는가 | 통과 — rational 기대와 DOMRect 1/64px 값을 분리하고 `0.02px` tolerance를 한정했다. |
| 23 | width basis override가 percent에만 적용되고 fr은 빠지는가 | 통과 — percentage와 fraction 모두 computed definite px width 및 같은 override gate를 요구한다. |
| 24 | definite zero와 indefinite width를 같은 rejection으로 다루는가 | 통과 — `0px` definite basis를 허용하고 indefinite width는 거부한다. |
| 25 | failure/over-quota가 partial placement를 공개할 수 있는가 | 통과 — 10,000-track cap, reservation failure와 no-partial-publication을 관문에 넣었다. |
| 26 | 지원 밖 intrinsic/mathematical/named grammar가 조용히 admission되는가 | 통과 — `auto`, intrinsic, minmax, fit-content, math, repeat, line/area를 거부 대상으로 적었다. |
| 27 | browser screenshot이 runtime pixel proof로 취급되는가 | 통과 — oracle 화면으로 한정하고 AVD/Simulator readback 구분을 별도 요구한다. |
| 28 | fixture FFI나 internal ABI version 변경이 계획에 숨는가 | 통과 — fixture FFI 없음, ABI 변경 없음, 숫자 버전 `0.1.0` 유지다. |
| 29 | 계획 review 20개가 PR diff review를 대신하는가 | 통과 — plan review 20과 이 PR diff review 30을 별도 문서로 둔다. |
| 30 | merge 이후 preview sync와 browser 종료가 빠지는가 | 통과 — 상태 반영 후 Tailnet hash/Agent Browser 확인과 process cleanup을 gate에 기록했다. |

검토 중 상위 C11 계획의 #1063 runtime evidence 디렉터리 링크가 실제 파일 경로와 달라 수정했고, local-link 재검사에서 해당 링크를 통과시켰다. 이 diff 검토는 구현/runtime 검토와 다르다. 구현 PR에서는 별도 20개 runtime failure 관점과 30개 PR diff 관점을 새로 적용해야 한다.
