# C11.4 `display:none` mixed-track 계획 PR · 변경분 적대 검토 30개

**대상 변경:** hidden mixed-track 계획, 상위 C11.4 계획·상태 동기화, Chrome fixture와 관측, 독립 `Fraction` 기대값 및 계획 실패 검토. 이 diff 검토는 계획 자체의 20관점 검토와 별도로 변경 연결성·재현성·상태 표현을 대조한다.

| # | 변경 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 상위 Grid 계획에서 hidden slice를 찾지 못함 | **통과.** `plan/c11-4-grid-track-sizing.md`가 새 집중 계획과 precomparison을 연결한다. |
| 2 | C11.4 상태 대장에 계획이 빠짐 | **통과.** `spec/STATUS.md`가 계획만 기록하고 `display:none` 구현 성공으로 표현하지 않는다. |
| 3 | 상위 C11.4 checkbox가 제한 fixture 때문에 체크됨 | **통과.** checkbox는 계속 미완료다. |
| 4 | 공식 완료 수가 제한 slice에서 증가 | **통과.** 계획·상태 모두 `72/213` 미완료를 유지한다. |
| 5 | PR #732가 아직 열려 있거나 병합 SHA가 잘못 기록됨 | **통과.** `gh pr view 732`에서 merged, `2026-10-28T02:35:51Z`, `9e9355369a48e5fa6bbed25a76954dbc63a5fcac`를 확인했다. |
| 6 | #732 검증 후보 표현이 병합 뒤에도 pending으로 남음 | **통과.** 상위 mixed plan의 기록을 병합 결과로 바꾸고 check rollup 미보고를 별도로 명시했다. |
| 7 | 사전 비교 README fixture 링크가 저장소 밖으로 벗어남 | **수정 반영.** 링크가 다섯 단계가 아닌 저장소 root 기준 네 단계 상향으로 고쳐졌다. |
| 8 | README hash manifest가 수정 후 파일과 다름 | **통과.** fixture, script, expected JSON, Chrome observation, screenshot 다섯 hash를 재계산해 일치시켰다. |
| 9 | expected JSON이 생성되지 않거나 JSON 문법이 깨짐 | **통과.** 기본 `python3` 재생성 뒤 `json.tool` parse를 확인했다. |
| 10 | 독립 산출 script가 설치된 Python에서 실행 불가 | **수정 반영.** 기본 Python 3.9.6에서 annotation `|` 평가 오류를 확인하고 postponed annotations를 추가했다. |
| 11 | oracle이 browser/production trace에서 수치를 가져옴 | **통과.** script는 입력 `Fraction`과 Grid sizing 산식만 사용하고 Chrome JSON을 읽지 않는다. |
| 12 | flex space에서 fixed 40px 또는 gutter를 누락 | **통과.** 독립 식은 300−40−20=240px이며 최초 fraction 80px이다. |
| 13 | hidden first base와 visible control base가 혼동됨 | **통과.** hidden 100px 및 visible 150px 결과가 별도 scene/property로 JSON에 있다. |
| 14 | freeze 뒤 2fr 잔여 공간 산술이 틀림 | **통과.** hidden `[100,40,140]`, visible `[150,40,90]`이며 네 장면 모두 track/gutter 합이 300px이다. |
| 15 | Columns와 Rows 결과가 같은 축으로 잘못 기록 | **통과.** 네 scene의 axis와 Chrome computed columns/rows를 expected JSON에 대조했다. |
| 16 | row test에서 text min-content가 sizing axis와 직교하지 않음 | **통과.** Row scene은 `vertical-rl`; Chrome 관측에서 row tracks 및 root-relative visible frames가 기대 배열과 일치한다. |
| 17 | Chrome document 좌표를 root-relative runtime frame처럼 사용 | **통과.** independent verifier는 각 item의 root origin을 빼며 변환 roundoff는 0.001px tolerance로 확인했다. |
| 18 | 숨김 browser rect 0을 visible control과 같은 layout box로 처리 | **통과.** `display`, `getClientRects().length`, zero DOMRect와 expected `hiddenBox`가 분리돼 있다. |
| 19 | hidden span visible positive control이 실제 contribution 차이를 증명하지 않음 | **통과.** Chrome의 두 visible controls는 track `[150,40,90]`과 한 client rect를 관찰했다. |
| 20 | image fixed neighbor intrinsic metadata가 준비되지 않음 | **통과.** Chrome 관측은 네 PNG decode 완료와 natural `24×16`을 기록하고 기대 frame과 대조했다. |
| 21 | screenshot이 깨졌거나 화면 설명과 장면이 다름 | **통과.** 저장 screenshot을 시각 확인했고 네 장면의 hidden/visible, Columns/Rows 배치를 구분할 수 있다. |
| 22 | Chrome 결과를 표준의 normative truth로 과장 | **통과.** README는 browser output을 cross-observation으로 한정하고 CRD 산술을 별도로 기록한다. |
| 23 | `display:none`과 `visibility:hidden` 지원 계약을 혼합 | **통과.** hidden box generation과 visibility의 layout participation을 구분하고 visibility runtime 변조를 거부한다. |
| 24 | 기존 세-child adapter negative input이 새 네-child scene으로 오승인 | **통과.** 새 계획은 정확한 direct child shape를 요구하고 기존 8 scene을 회귀한다. |
| 25 | known scene id지만 changed child/style 입력이 generic fallback됨 | **통과.** plan은 unsupported로 fail-closed하고 partial layout/draw 생성을 금지한다. |
| 26 | zero frame bookkeeping이 CSS box/draw 존재로 오해됨 | **통과.** 계획이 zero-frame bookkeeping, CSS box 부재, draw item 부재를 명시적으로 구분한다. |
| 27 | implementation test와 Chrome oracle control이 섞임 | **통과.** runtime은 hidden scene 두 개만 구현하고 visible controls는 Chrome 사전 비교로 남긴다. |
| 28 | Simulator gate가 Chrome screenshot만으로 통과 처리됨 | **통과.** 계획 병합 뒤 두 platform에서 기존 8 + hidden 2 scene의 앱 실행을 요구한다. |
| 29 | Agent Browser 프로세스가 계획 변경 뒤 남음 | **통과.** 현재 process inventory에는 이전 세션 외의 Agent Browser/server가 없고 새 browser를 열지 않았다. |
| 30 | 변경이 구현, 전체 CSS admission, WPT 또는 C11.4 완료를 암시 | **통과.** PR diff는 plan/evidence만 수정하고 범위 밖 및 공식 미완료 상태를 유지한다. |

**판정:** 두 개의 실제 준비 결함(기본 Python annotation 처리, README fixture 링크)을 발견해 수정했다. hash, JSON 재생성, 네 Chrome scene track/frame 대조와 screenshot을 다시 확인했다. 별도 검증 코드의 엄격한 float equality도 root-relative 좌표 변환의 `0.001px` 허용오차로 바꿔 판정했으며, 이 변화는 제품/runtime 기준을 완화하지 않는다. GitHub PR 병합 전 최종 diff를 한 번 더 대조한다.
