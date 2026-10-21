# C11.4 두 단계 nested Grid 계획 PR diff 최종 검토 20개

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 새 fixture가 실제로 root → middle → inner Grid → text topology인가? | depth-one control과 depth-two case의 DOM owner/ID 및 one-child 관계를 fixture와 두 oracle JSON에서 대조했다. 계획이 요구하는 depth 1·2 topology와 일치한다. 통과. |
| 2 | Ahem load 실패가 fallback font으로 숨겨지는가? | fixture는 font ready 후 Ahem 체크를 관찰하고, observation의 `ahemReady`가 true다. 해당 상태가 없는 결과를 비교 근거로 사용하지 않는다. 통과. |
| 3 | 네 scene 식별자가 서로 덮이거나 control/test를 뒤바꾸는가? | min/max와 depth 1/2의 네 ID가 fixture, Chrome JSON, 기대값 JSON에서 일대일로 대응한다. 통과. |
| 4 | definite px width나 fixed track이 intrinsic propagation을 우회하는가? | 각 Grid의 width와 한 column track이 대응 `min-content` 또는 `max-content`이며 gaps와 box extras는 0이다. 통과. |
| 5 | min-content 계산이 공백을 longest word에 더해 80px로 오판되는가? | 고정 Ahem `XX XXXXXXX`에서 가장 긴 unbreakable word `XXXXXXX`의 70px을 독립 기대값으로 둔다. 통과. |
| 6 | max-content가 collapsible space advance를 빠뜨리는가? | Ahem의 `20 + 10 + 70 = 100px`을 기대값 JSON 및 계획 표에서 대조했다. 통과. |
| 7 | 기대값이 Chrome 관측을 복사해 순환 oracle이 되는가? | 규범 및 Ahem metrics 기반 `spec-derived-expectations.json`과 Chrome observation이 다른 파일이며 기대값에 독립성 표시가 있다. 통과. |
| 8 | Chrome 숫자가 독립 기대값처럼 표현되는가? | 계획·README·공개 로드맵은 Chrome 156을 관측값으로 분리하고 규범 계산과 일치한 한 build 결과라고 한정한다. 통과. |
| 9 | page/viewport 좌표가 root-relative frame gate를 오염하는가? | browser page offsets는 observation에만 보관하고 계획은 root-relative geometry와 0 offset 비교를 요구한다. 통과. |
| 10 | screenshot이 다른 fixture·빈 canvas이거나 화면 근거 범위가 과장되는가? | PNG가 1280×633이며 Chrome observation의 네 scene/환경과 같은 사전 비교 자료다. runtime 또는 simulator screenshot이라고 주장하지 않는다. 통과. |
| 11 | SHA manifest가 필수 입력이나 비교자료를 빠뜨리거나 자신을 재귀 hash하는가? | fixture, Ahem asset, 두 JSON, screenshot만 포함한다. `shasum -c`에서 다섯 항목이 모두 통과했고 manifest/README는 의도적으로 제외했다. 통과. |
| 12 | 계획 자체 검토와 PR diff 검토가 한 표를 재사용하는가? | 20개 계획 실패 관점 문서와 이 최종 diff 검토를 별도 파일로 두고 서로 다른 실패 표를 유지한다. 통과. |
| 13 | 상위 C11.4 track-sizing 계획의 기존 기록을 덮거나 구현 이력과 섞는가? | 새 날짜별 절만 추가했고 직전 PR #448 구현 결과와 기존 범위를 유지한다. 통과. |
| 14 | 공식 STATUS 항목이 C11.4 하위에서 빠져 unrelated top-level roadmap 항목으로 보이는가? | 새 기록의 들여쓰기를 기존 C11.4 nested Grid 기록과 맞췄고 C12 top-level 시작 전에 놓았다. 통과. |
| 15 | 계획 단계가 C11.4 checkbox나 72/213 완료 집계를 올리는가? | STATUS·계획·README·공개 HTML에서 C11.4 미완료와 `72/213` 유지가 일치한다. 통과. |
| 16 | 공개 roadmap의 정적 경로가 markdown/PNG 원본과 맞지 않는가? | `/spinon/plan/...html`과 evidence README/review `.html` route를 대응 `.md` 파일로, Chrome PNG route를 실제 파일로 대조했다. 통과. |
| 17 | 공개 roadmap이 아직 실행하지 않은 runtime/Simulator 검증을 완료로 표기하는가? | 새 문단은 계획·사전 비교라고 쓰고 Spinon runtime, Android AVD, iOS Simulator가 미실행이라고 명시한다. 통과. |
| 18 | 계획 링크가 없는 interface/API 변경을 암시하거나 버전을 올리는가? | PR 변경은 계획·근거 문서와 precomparison fixture에 한정된다. runtime ABI, API와 internal contract version 변경은 없다. 통과. |
| 19 | depth 3+, subgrid, arbitrary CSS, WPT 및 pixel validation 제외가 모든 기록과 충돌하는가? | 집중 계획, README, parent plan과 roadmap의 supported claim/제외 범위를 교차 확인했다. 통과. |
| 20 | PR diff와 함께 CI/배포/브라우저 프로세스 상태를 이미 완료한 것처럼 쓰는가? | 문서에는 plan-stage 실행만 기록한다. PR 생성 뒤 현재 required checks를 다시 확인하고 merge 후 Tailnet preview를 Agent Browser로 갱신·검증한 다음 browser를 닫는다. Pages 배포나 CI 통과는 주장하지 않는다. 통과. |

## 판정

최종 plan PR diff의 fixture topology, 분리된 oracle, 독립 기대값, 파일·route 무결성, 공식 상태와 미검증 경계를 서로 다른 20개 관점에서 확인했다. runtime 또는 simulator 작업과 CI는 이 plan PR에서 실행·통과했다고 기록하지 않았다.
