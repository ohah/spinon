# C15.3 scroll-transform 계획 PR diff 검토 20개

**범위:** oracle fixture/관찰, C15 계획, 상태 대장, conformance index, Tailnet roadmap 문구. 제품 코드는 변경하지 않았다.

| # | 적대적 확인 | 결과 |
|---:|---|---|
| 1 | 기존 C15.3 shaping, glyph paint, owner clip/hit-test 범위를 덮어써 완료로 승격하는가? | 계획에 후속 slice만 추가했다. C15.3 및 C15 상위는 미완료로 명시했다. |
| 2 | 새 slice가 고정 Ahem input 밖의 폰트 fallback이나 일반 DOM text admission을 은근히 지원하는가? | Ahem `XX` fixture만 관찰하고 CSS projection·fallback은 제외했다. |
| 3 | Chrome 버전이 user agent의 `155.0.0.0`만으로 잘못 고정됐는가? | Agent Browser `chrome://version`에서 `155.0.8059.85`, revision `0e66b945…`를 확인했다. |
| 4 | headless Chrome을 그래픽 compositor 결과로 과장하는가? | 관찰은 DOM Range/rect/`elementFromPoint`로 표시했다. GPU·앱 적합성은 주장하지 않는다. |
| 5 | font load 실패가 system fallback 결과를 통과시키는가? | 관찰 JSON에 `fontsStatus=loaded`, Ahem `document.fonts.check=true`를 기록했다. |
| 6 | horizontal 기준이 border-box와 scrollport를 혼동하는가? | parent `15×22`, scrollport `13×20`, `clientWidth=13`, `scrollWidth=21`을 따로 기록했다. |
| 7 | offset 방향/크기가 주장과 raw geometry에서 맞지 않는가? | x `+2→-1`에서 3px 이동, x `+2→-4`에서 6px 이동을 JSON과 대조했다. |
| 8 | vertical 값이 horizontal 좌표로부터 추론됐는가? | 별도 `30×15` parent와 `scrollHeight=40`에서 y `+23→+5`를 관찰했다. |
| 9 | nested 4+5가 실제 두 container offset인가? | JSON에서 outer `scrollLeft=4`, inner `scrollLeft=5`, text x 변화 `-9`를 각각 확인했다. |
| 10 | nested clip이 outer/inner scroll을 어느 좌표에서 따르는지 누락되는가? | outer와 inner client boxes를 분리 저장하고 visible/changed hit target을 비교했다. |
| 11 | repeated scroll update가 한 번의 lucky transition만 검증하는가? | `3→6→0`, `18→7→0`, `4+5→2+3→0+0`을 사전 기준에 넣었다. |
| 12 | reset이 기저 geometry 복원을 확인하지 않는가? | 각 axis와 nested case의 reset rect가 시작 rect와 같아야 한다. |
| 13 | viewport 절대 y 좌표를 다른 capture 배치에 고정하는가? | acceptance는 parent-relative rect 차이를 사용하고 absolute 위치를 기준에서 제외했다. |
| 14 | screenshot이 reset 상태인데 scroll 상태로 설명되는가? | screenshot capture 직전 세 case에 계획 offset을 적용했고 PNG를 육안 확인했다. |
| 15 | screenshot이 Spinon 제품 화면처럼 표시되는가? | README와 이미지 설명을 Chrome 155 oracle로 한정했다. |
| 16 | 새 CSS/scroll 동작의 WPT 적합성이 입증됐다고 쓰는가? | WPT 실행·spinon pass는 없다고 판정 및 계획에서 분리했다. |
| 17 | 계획 PR에서 Simulator 검증이 끝난 것으로 오해되는가? | Android/iOS는 다음 구현 slice의 pass gate로만 적었고 미실행을 명시했다. |
| 18 | repo의 사용자 작업이나 dirty main checkout에 변경을 기록하는가? | 별도 worktree와 `plan/c15-3-text-scroll-transform-20261017` branch에서만 변경했다. |
| 19 | full Rspress build 실패를 성공으로 보고하거나 새 문서 문제를 숨기는가? | exit 1과 기존 490 dead-link 진단을 별도 기록했고 신규 scroll 문서 dead-link는 로그에서 찾지 못했다. |
| 20 | 공개 Tailnet roadmap이 C19 판정 또는 C15 상태와 모순되는가? | Chrome C19 결론은 “source-based suspicion, final verdict pending”으로 제한하고 C15 scroll은 “plan oracle only”로 표시했다. |

**판정:** 이 계획·사전 비교 변경에서 별도 실패 경로 20개를 확인했다. full 문서 build 실패는 미해결 기존 dead-link이며 새 계획의 pass로 취급하지 않는다.
