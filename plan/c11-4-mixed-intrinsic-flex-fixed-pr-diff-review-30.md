# C11.4 mixed intrinsic/flexible/fixed track plan PR diff review · 30 관점

**검토 대상:** plan-only PR diff, 고정 Chrome 156/독립 rational precomparison, 상위 계획·공식 상태 연결

| # | 독립 PR diff 실패 가설 | 결과 | 확인 결과 |
|---:|---|---|---|
| 1 | 변경이 plan-only라고 주장하지만 production Rust/C/Swift/Java가 바뀌었다. | PASS | Staged name diff는 두 계획, 두 review, 상위 plan, STATUS, 고정 fixture와 precomparison evidence만 포함한다. |
| 2 | unrelated user-owned/untracked files가 PR에 들어간다. | PASS | Explicit path staging만 사용했다. 나머지 1,700여 개 untracked evidence entries는 staged diff에 없다. |
| 3 | branch base가 최신 main이 아니어서 계획이 다른 revision을 설명한다. | PASS | branch는 `main`과 `origin/main`이 일치한 `728acc3ac0738ada0eff629d8cc2903a83dd2507`에서 시작했다. |
| 4 | 이전에 완료된 unbounded fixed-repeat처럼 오래된 plan 상태를 신규 기능으로 다시 제안한다. | PASS | 현재 제안은 direct intrinsic/flex/fixed interaction이고, fixed-repeat 상한 제거와 별도다. 현재 source/PR 기록과 precomparison을 확인했다. |
| 5 | parent plan의 최신 상태 요약과 신규 plan-only slice가 충돌한다. | PASS | 신규 상위 entry는 현재 72/213·C11.4 미완료와 precomparison만 추가하며 implementation 완료를 주장하지 않는다. |
| 6 | STATUS 변경이 C11.4 checkbox를 완료하거나 aggregate count를 증가시킨다. | PASS | C11.4는 unchecked이고 신규 row는 계획 전용, `72/213` 유지라고 적는다. |
| 7 | PR diff가 새 API나 내부 numeric contract version을 암묵적으로 추가한다. | PASS | API·layout source·contract version 변경이 없고 계획은 fixture-only 경계를 명시한다. |
| 8 | CSS authoring support나 전체 WPT 적합성을 암묵적으로 약속한다. | PASS | 목표, 범위 밖, precomparison README 모두 general CSS admission/WPT/C11.4 전체를 분명히 제외한다. |
| 9 | Oracle fixture와 계획의 track list가 다르다. | PASS | HTML과 plan 모두 `minmax(min-content,1fr) 40px minmax(0px,2fr)` 및 두 10px gutter를 사용한다. |
| 10 | Root dimensions·content box가 expected arithmetic와 다르다. | PASS | Columns 300×120, Rows 120×300, zero border/padding이며 fixture와 plan이 같은 수치를 선언한다. |
| 11 | HTML scene 수, plan scene 표, observation, expected JSON의 ID 집합이 다르다. | PASS | Consistency check에서 여덟 ID 집합이 observation/expected JSON 사이 정확히 일치했다. Fixture에는 freeze/no-freeze/equality/reverse controls가 있다. |
| 12 | 반복 scene 이름이나 source order 대조가 중복되어 독립 결과로 세어진다. | PASS | 각 axis에 freeze, no-freeze, equality가 있고 reverse scene은 freeze의 별도 DOM-order control이라고 명시한다. |
| 13 | Ahem metrics가 파일 revision이나 font readiness와 연결되지 않는다. | PASS | `document.fonts.ready`, `document.fonts.check`, loaded status와 Ahem SHA-256을 precomparison에 기록했다. |
| 14 | 10/8/4글자 기대 폭이 100/80/40px input이 아니다. | PASS | Ahem 10px `X` advance, 고정 문자열 길이, Chrome used tracks를 별도 기록했고 expected script input이 동일하다. |
| 15 | Row-axis case가 orthogonal text 없이 columns 결과를 복사한다. | PASS | Rows fixture에는 `vertical-rl` Ahem text가 있으며 Chrome observation에서 row 100/80/40 boundaries를 따로 얻었다. |
| 16 | Source-order reverse control의 explicit placement도 함께 뒤집혀 결과 비교가 무효다. | PASS | `data-item` selectors로 grid lines를 유지한 채 DOM 순서만 `[a,b,c] → [c,b,a]`로 바꾼다. Browser frames와 tracks가 일치한다. |
| 17 | PNG가 decode되지 않거나 authored size가 natural dimensions를 대체한다. | PASS | CSS width/height는 auto, image self-alignment는 start, 8개 decode 완료와 24×16 natural size/hash를 기록한다. |
| 18 | Image의 fixed-track 출처를 intrinsic base 크기 결정으로 과장한다. | PASS | 독립 계산은 text-driven min-content base만 사용한다. README는 image를 mixed fixed-track control로 분리해 기록한다. |
| 19 | Flex available space 산술이 fixed track 또는 gutter를 놓친다. | PASS | `300 - 40 - 2×10 = 240`, total factors 3, initial fraction 80을 plan와 rational generator에서 일치시킨다. |
| 20 | Freeze case가 100px base를 고정한 뒤 remaining 2fr share를 잘못 계산한다. | PASS | Independent generator와 table은 `100 + 40 + 140 + 20 = 300`을 기록한다. |
| 21 | Equal-boundary case가 strict comparison을 실제로 검증하지 않는다. | PASS | Columns/Rows 모두 80px base와 80px hypothetical을 대조하고 freeze 없음 및 `[80,40,160]`을 기대한다. |
| 22 | No-freeze control이 equality와 같은 입력이라 threshold 아래 결과를 확인하지 않는다. | PASS | 별도 40px contribution control을 두 축에 모두 둔다. |
| 23 | Rational generator가 floating point 또는 production/browser 결과를 expected 입력으로 쓴다. | PASS | `fractions.Fraction`만으로 §11.7.1 산술을 구성하며 output은 checked-in JSON과 재생성 비교했다. |
| 24 | Frame 생성기가 fractional 값을 조용히 정수로 잘라낸다. | PASS | `integer_px`가 denominator 1을 요구하고 정수가 아니면 오류를 발생시킨다. |
| 25 | Browser global positions가 app frame 기준으로 잘못 재사용된다. | PASS | Global Chrome coordinates는 observation에만 있고 expected JSON은 root-relative frames다. 자동 consistency check에서 변환 후 8/8 일치했다. |
| 26 | Chrome `1/64px` quantization과 app `1e-4px` acceptance를 한 tolerance로 섞는다. | PASS | Plan은 browser cross-check와 independent rational/app comparison에 별도 허용치를 준다. |
| 27 | Browser metadata에 version, viewport 또는 DPR가 빠져 재현이 어렵다. | PASS | HeadlessChrome 156.0.0.0, Agent Browser 0.31.1, 1280×633 CSS px, DPR 1과 raw user agent가 관측 JSON에 있다. |
| 28 | 새 계획 링크가 누락 파일을 가리키거나 기존 #717 검토 placeholder가 남아 있다. | PASS | 새 계획·사전 비교 링크가 해석되고, 상위 계획의 `${review}` placeholder는 tracked #717 merge 후 검토 보고서로 수정했다. |
| 29 | Android/iOS simulator matrix나 default/fixture artifact boundary가 implementation gate에서 빠진다. | PASS | Plan은 Android API 37 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator 양쪽 8 scene 및 default negative/fixture positive controls를 요구한다. |
| 30 | 계획 검토 20개가 PR diff 검토와 같은 표를 재사용하거나 결과가 count-only다. | PASS | Plan review는 20개의 pre-implementation 실패 가설, 이 report는 diff/link/scope/hash consistency 30개를 별도로 기록한다. |

**결과:** PASS 30 / PARTIAL 0 / FAIL 0. 최종 staged diff 검토 중 기존 상위 계획의 `${review}` 링크 placeholder를 발견해 tracked #717 merge 후 검토 보고서로 연결하고, 변경된 링크를 재검증했다. Precomparison consistency check는 scene ID 8/8, track과 root-relative frames 8/8을 Chrome observation과 rational expected 사이 `1/64 CSS px` 이내에서 확인했다. Expected JSON은 independent generator와 byte-for-byte 일치한다. Browser session은 종료됐고 `agent-browser session list`는 비어 있으며 `agent-browser-darwin-arm64` process도 관찰되지 않았다. 이 review는 구현/runtime review를 대신하지 않는다.
