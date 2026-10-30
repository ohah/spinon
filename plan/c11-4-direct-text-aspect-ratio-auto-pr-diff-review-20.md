# C11.4 direct-text `auto <ratio>` 계획 PR 변경분 검토 20개

**대상:** 계획 PR 변경 파일 8개: C11.4 master plan 상태, 신규 content-box 계획과 계획 검토, Chrome fixture·원장·screenshot·evidence README, 이 diff 검토 기록

**기준:** `38359a4212550fab9e29544367a16e4eba45e06a` 기반 계획 PR staged diff

**판정:** 20/20 PASS. 구현/runtime 동작은 이 PR에서 검증하지 않는다.

| # | 변경분 실패 관점 | 결과 |
|---:|---|---|
| 1 | Master plan이 이미 병합한 #842를 계속 “미구현”으로 표시 | PASS — finite-positive 항목을 #842 병합 결과와 16/16 두 simulator 근거로 갱신했다. |
| 2 | 신규 `auto <ratio>` 계획을 숫자 ratio 선행 slice와 혼동 | PASS — #842는 별도 선행 slice로 남고 새 계획은 content-box basis 차이를 다룬다. |
| 3 | C11 status checkbox 또는 완료 집계가 fixture 후보만으로 올라감 | PASS — master plan과 계획이 C11/C11.4 unchecked 및 `72/213`을 명시한다. |
| 4 | Working Draft를 확정 규범 계약으로 소개 | PASS — W3C Level 4 §4.1을 exploratory Working Draft로 표시하고 단독 지원 계약이 아니라고 한정한다. |
| 5 | 숫자 ratio와 `auto <ratio>`의 basis가 같은 값으로 단순화 | PASS — typed `BoxSizing`/`ContentBox` 제안과 `91px`/`90px` oracle을 함께 둔다. |
| 6 | `ComputedElementStyle` 숫자 타입에 box basis가 실수로 묻힘 | PASS — 현재 `Option<f32>`의 정보 손실을 문제로 명시하고 독립 enum/type를 제안한다. |
| 7 | Cascade support가 비-C11 runtime profile에 전파 | PASS — 허용 profile과 기존 다른 profile rejection을 제한 계약에 고정한다. |
| 8 | CSS source 문자열에서 ratio 값을 유추 | PASS — Stylo same-revision typed value를 계산 원본으로 요구하고 serialization은 일관성 검사로 둔다. |
| 9 | `aspect-ratio:auto`를 positive ratio로 수락하거나 no-ratio control로 오인 | PASS — typed ratio 없음은 computed serialization이 `auto`일 때만 control로 허용한다. |
| 10 | Degenerate `auto 0/0`이 `None`을 거쳐 ratio positive로 둔갑 | PASS — degenerate/invalid/synthetic overflow input은 후보 성공 대신 fail-closed한다. |
| 11 | `auto <ratio>`의 vertical padding/border를 ratio height에 포함 | PASS — content height `40/1.5` 뒤 6px edge를 outer box에 더하는 독립 기대값을 기록한다. |
| 12 | definite-row ratio에서 content height와 border-box height 혼동 | PASS — `60−6=54`, `54×1.5=81`, outer `+10=91` 경로를 명시한다. |
| 13 | numeric border-box ratio를 `auto <ratio>`처럼 재해석 | PASS — numeric control은 `60×1.5=90`과 Chrome 90px track으로 분리된다. |
| 14 | Ahem path나 server root가 잘못돼 fallback font 관측 | PASS — fixture-relative font URL은 `tests/fixtures` server root에서 200, `fontCheck:true`, 저장소 font hash를 남긴다. |
| 15 | 브라우저 viewport/버전/font 상태가 원장에서 빠짐 | PASS — JSON에는 HeadlessChrome 156, 1280×633, DPR 1, URL, Ahem readiness와 7 scenes가 보존된다. |
| 16 | Chrome layout quantum을 exact rational과 혼동 | PASS — 독립 32⅔/33⅓와 32.65625/33.328125 관측을 나누고 1/64px quantum을 적었다. |
| 17 | Grid track, root frame, item frame 및 overflow가 한 수치에 합쳐짐 | PASS — 표는 track/frame을 분리하고 100px root 대비 111px+ track overflow를 설명한다. |
| 18 | Screenshot 또는 raw JSON이 fixture/source revision과 분리 | PASS — fixture·font·JSON·PNG 각각 SHA-256을 README에 연결했다. |
| 19 | 계획 PR 단계에서 simulator/API 완료를 이미 주장 | PASS — simulator/build/artifact는 implementation acceptance gates이며 현재 PR은 Chrome precomparison만 근거로 한다. |
| 20 | browser/server 및 작업 범위 밖 자료가 PR에 포함 | PASS — Agent Browser와 port 8769 server를 종료했으며 stage 대상은 위 계획/evidence의 8개 경로로 제한했다. |

이 검토는 plan PR diff의 설계·재현자료 경계를 본다. 실제 Taffy ratio propagation, Android/iOS 실행, API 계약 merge 후 상태, 일반 CSS admission과 WPT는 구현 PR 검증으로 남긴다.
