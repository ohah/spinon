# C11.1 percentage implicit columns 계획 PR 변경 검토 30개

검토 대상은 계획 PR의 실제 staged diff다: 계획·별도 계획 검토·fixture·Chrome raw observation·독립 기대값·screenshot·`STATUS.md` 후보 기록·public roadmap 카드다. **통과 30, blocker 0**이다. 이는 구현 검토가 아니며 Taffy/Spinon runtime 지원을 증명하지 않는다.

| # | PR diff 실패 관점 | 결과 | 근거 |
|---:|---|---|---|
| 1 | 계획이 C11.1의 기존 parent와 선행 구현을 가리키는가 | 통과 | parent, sparse column-flow, fixed-px implicit columns, percentage rows를 연결했다 |
| 2 | 계획 상태가 구현 완료로 오인되지 않는가 | 통과 | 상단에 구현 전 계획 후보라고 명시했다 |
| 3 | 이번 변경이 fixed px를 대체하거나 삭제하는가 | 통과 | 기존 fixed-px 동작을 유지하고 목록 안에 px/% 혼합만 추가한다 |
| 4 | CSS Grid 규범 출처가 현재 문서와 날짜로 고정됐는가 | 통과 | W3C CSS Grid Level 2 2025-03-26 CRD 링크와 관련 절을 표시했다 |
| 5 | browser observation을 규범 원본으로 취급하는가 | 통과 | Chrome은 독립 관측자, 기대값은 산술·규범에서 계산한다고 구분했다 |
| 6 | browser source file hash가 plan fixture와 연결되는가 | 통과 | expected.json의 SHA-256이 fixture와 일치한다 |
| 7 | raw browser observation의 UA/viewport가 보존되는가 | 통과 | HeadlessChrome 156, 1280×633, DPR 1이 JSON에 있다 |
| 8 | fixture root width가 240px content box임을 재현할 수 있는가 | 통과 | border/padding 0이며 computed width와 DOMRect를 observation에 저장했다 |
| 9 | explicit column/row/gap 입력이 expected와 source 간 일치하는가 | 통과 | `[30,40]`, `[20,40]`, 5px/3px 값이 source·expected·plan에서 일치한다 |
| 10 | item 개수/크기가 column·row geometry와 맞는가 | 통과 | 12개 source-order 10×10 item으로 2행×6열을 만든다 |
| 11 | percentage basis가 남은 공간으로 오해되는가 | 통과 | 240px definite content width를 사용한다 |
| 12 | gap이 percentage basis에서 차감되는가 | 통과 | 24px/36px track은 basis 계산 뒤 gap을 위치 계산에 더한다 |
| 13 | auto-column list가 implicit column마다 반복되는가 | 통과 | 네 implicit columns와 cycle expected를 고정했다 |
| 14 | scalar percentage case가 빠지는가 | 통과 | `10%` scalar 관측·기대값이 별도다 |
| 15 | mixed px/% 목록에서 pattern 순서가 사라지는가 | 통과 | `10px 15%`의 `[10,36,10,36]` 사용값을 기록했다 |
| 16 | 0%가 auto sizing 또는 invalid input으로 합쳐지는가 | 통과 | zero-width implicit columns와 고정 item overflow를 분리했다 |
| 17 | 120%가 clamp되어 overflow 경계를 숨기는가 | 통과 | `288px` tracks와 후속 item starts를 저장했다 |
| 18 | definite zero width가 indefinite width로 취급되는가 | 통과 | `0px` basis와 0px percentage tracks를 별도 case로 뒀다 |
| 19 | `grid-auto-columns` computed serialization과 used tracks가 섞이는가 | 통과 | raw JSON이 computed property와 used `gridTemplateColumns`를 둘 다 담는다 |
| 20 | DOM item geometry가 expected 순서에서 누락되는가 | 통과 | cycle 12개 DOMRect를 expected 및 raw JSON으로 대조했다 |
| 21 | independent expected를 Chrome output에서 자동 복사했는가 | 통과 | 240×0.10/0.15, explicit tracks와 gaps로 직접 계산했고 검증기에서 여섯 case와 비교했다 |
| 22 | raw observation의 six-case labels가 기대값 case와 잘못 매핑되는가 | 통과 | verifier가 scalar/mixed/zero/over-100/zero-basis와 cycle을 각각 대조했다 |
| 23 | screenshot이 변형된 zero-basis browser state를 보여주는가 | 통과 | fixture를 다시 열어 초기 `10% 15%` state를 캡처했다 |
| 24 | screenshot 크기/hash가 evidence README와 일치하는가 | 통과 | 1280×633, `3e41efa6…84b7`을 확인했다 |
| 25 | evidence의 relative fixture/expected links가 잘못됐는가 | 통과 | 경로 깊이와 파일 존재를 staged diff 기준으로 확인했다 |
| 26 | plan error boundary가 explicit track·row mode까지 무심코 넓히는가 | 통과 | definite direct percentage `grid-auto-columns`로만 한정한다 |
| 27 | CSS invalid fallback과 profile rejection의 차이를 지우는가 | 통과 | 두 경로를 별도로 검증하도록 적었다 |
| 28 | status ledger가 C11.1/C11 완료나 `72/213` 증가를 주장하는가 | 통과 | 추가 문장은 plan 후보이며 구현·승격 전, 숫자는 유지라고 명시한다 |
| 29 | public roadmap card가 공식 상태 anchor나 계획 route가 아닌 다른 항목을 가리키는가 | 통과 | 계획 route, Agent Browser precomparison, C11 anchor를 확인했다 |
| 30 | Agent Browser 프로세스가 증거 수집 후 남아 있는가 | 통과 | `agent-browser close --all` 이후 Agent Browser/Chrome 프로세스 수는 0이다 |

## 실행 증거

- Independent arithmetic와 raw Chrome JSON 대조: 6/6 case의 used tracks·column starts 일치, cycle item frame 12/12 일치.
- source, raw observation, screenshot hash 확인 통과.
- staged whitespace 검사 `git diff --cached --check` 통과.
- 문서 전체 빌드나 구현 테스트는 이 plan-only diff에서 아직 실행하지 않았다.
