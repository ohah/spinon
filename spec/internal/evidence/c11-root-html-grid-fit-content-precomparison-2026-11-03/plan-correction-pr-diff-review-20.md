# C11.1 fit-content 계획 정정 PR 변경 검토 20개

계획 정정 PR의 실제 diff(상위·전용 계획, Chrome 고정 fixture/expected/관찰/캡처, 검증 경계 근거, 계획·PR diff review)를 다음 20개 적대 관점으로 검토했다.

| # | PR diff 관점 | 결과 |
| --- | --- | --- |
| 1 | plan이 Root HTML 기본 profile scope에서 벗어나는지 | `RuntimeRootHtmlTextPaintV1` admission과 typed Grid path는 그대로 유지했다. |
| 2 | Ahem/browser evidence가 OS font 실행으로 잘못 표현되는지 | Chrome Ahem oracle과 default renderer font path 차이를 명시했다. |
| 3 | 주장에 근거 코드가 연결되는지 | `root_html_default_render`의 runtime engine 생성과 명시적 Ahem 등록 경로를 대조해 검증 경계 근거에 연결했다. |
| 4 | test-only Ahem을 product feature로 바꾸는지 | product font loading, font API, CSS parser 변경을 scope 밖에 뒀다. |
| 5 | 검증 단계가 이름만 나뉘고 결과는 합쳐지는지 | layout Ahem, projection, default runtime, simulator surface 주장을 분리했다. |
| 6 | Ahem matrix가 실제 cap/floor를 계속 확인하는지 | 세 열 cap·정수 repeat·row floor 기대값을 보존했다. |
| 7 | named-area coverage가 없어지는지 | fixed-area 조합을 projection에 남기고 `grid-area: fixed/fit` default runtime child-frame assertion도 추가했다. |
| 8 | default route runtime의 fit-content declaration이 빠지는지 | 직접 track, 고정 named-area, 고정 row 세 scene 각각에 explicit `fit-content()` 선언을 요구한다. |
| 9 | runtime track과 item frame을 같은 값으로 혼동하는지 | `[40,10]`, `[20,40]`, `[40]` track과 각 item의 x/y/width/height를 별도 기댓값으로 고정한다. |
| 10 | overflow clipping을 false pass로 만들지 | CSS 표본 x=25/35/42px와 row y=10/25px를 기록된 DPR/표면 크기로 변환해 PixelCopy/Metal 경계 안팎을 검사한다. |
| 11 | system font에 따라 screenshot이 흔들리는지 | runtime smoke는 크기가 고정된 색상 box를 사용해 글꼴 측정값을 제거한다. |
| 12 | AVD sample이 hardware evidence처럼 보이는지 | ANGLE/SwiftShader AVD 한계를 유지한다. |
| 13 | Metal sample이 compositor output으로 보이는지 | pre-compositor limitation을 유지한다. |
| 14 | auto-repeat/implicit/percentage/calc scope가 새어 나오는지 | 기존 negative boundary list와 tests는 변하지 않았다. |
| 15 | 기존 Ahem Chrome baseline이 삭제되는지 | 여섯 Ahem scenes와 독립 probes를 보존하고, 추가 fixed-box row observation 및 갱신된 fixture/screenshot hashes를 별도 기록한다. |
| 16 | Chrome oracle가 Spinon runtime 근거로 확대되는지 | 추가 row 캡처는 Chrome 사전 비교만 뜻하며 제품 runtime/simulator 주장은 하지 않는다. |
| 17 | C11 checkbox/counter가 변하는지 | `spec/STATUS.md`는 수정하지 않아 C11/C11.1이 미완료로 남는다. |
| 18 | 상위 index가 오래된 review 경로를 가리키는지 | 계획에서 검증 경계 근거와 정정 계획 검토 20개·이번 PR diff 검토 20개를 따로 연결한다. |
| 19 | review count가 구현 검토를 대체하는지 | 계획 정정 검토는 향후 구현/runtime 20개와 구현 PR diff 20개를 대체하지 않는다. |
| 20 | 정정 병합 전에 구현 branch에서 작업이 시작되는지 | 정정 PR 병합을 구현 진입 조건으로 두고 기존 branch에는 제품 코드가 없다. |

최종 correction diff에서 미해결 결함은 없다. 이 record는 correction PR만 review하며 product implementation/runtime evidence를 대체하지 않는다.
