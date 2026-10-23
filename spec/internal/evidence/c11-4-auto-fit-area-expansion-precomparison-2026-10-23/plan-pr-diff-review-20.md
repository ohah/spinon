# C11.4 auto-fit + area 확장 계획 PR diff · 실패 관점 20개

계획 PR에 포함한 계획, 20관점 계획 검토, CSS fixture, Chrome JSON/screenshot/hash diff를 병합 전 별도 관점으로 검토했다. 계획 검토 표 자체를 다시 세지 않고, 실제 변경 파일이 도입할 수 있는 경로·불변 조건·범위 오류를 확인했다.

| # | diff에서 확인한 실패 경로 | 결과 |
| --- | --- | --- |
| 1 | 신규 계획이 제품 지원이나 C11.4 완료를 주장한다. | 계획 상태를 구현 전으로 고정하고 C11.4 checkbox와 공식 `77/213`을 유지한다. |
| 2 | 계획의 대상 profile이 실제 두 repeat + areas 경계와 다르다. | `RuntimeBlockFlexGridTrackSizingV1`과 cascade `allow_auto_repeat`/projection 경계를 특정하고 이전 profile은 제외했다. |
| 3 | W3C link가 area expansion과 collapse 근거를 각각 지원하지 않는다. | §7.1 explicit grid, §7.2.3.2 auto-repeat, §7.5 implicit grid를 직접 링크했다. |
| 4 | auto-repeat 수를 area dimension에 맞춰 먼저 늘린다. | 52px/20px/4px에서 2 repeats를 분리 계산하고 3번째 area track은 뒤에서 확장한다. |
| 5 | `fit-fit-empty`가 area dimensions보다 작게 작성되어 extension을 실제로 건드리지 않는다. | 세 행에 세 cell을 둬 두 축 모두 3×3 area로 만든다. |
| 6 | empty-area test에서 실제 in-flow item이 생긴다. | 이 scene에는 child를 넣지 않아 area 선언만으로 occupancy가 되는지 분리한다. |
| 7 | added-cell item이 반복 track에 놓여 collapse 기대가 부정확하다. | `hero`는 3행 3열뿐이어서 반복 두 track을 직접 점유하지 않는다. |
| 8 | full-span scene의 item이 일부 반복 track을 span하지 않는다. | `fit-fit-span`에서 hero를 3×3 전체에 선언한다. |
| 9 | `fit-fill-item` / `fill-fit-item` 이름과 축별 auto-fill/auto-fit 설정이 서로 바뀐다. | CSS, label, JSON과 계획 표에서 columns mode를 먼저 표기해 대조했다. |
| 10 | area-added implicit track이 repeat track 크기를 복제해 overflow 계산이 틀린다. | 새 track은 16px/12px, repeat은 20px로 분리하고 scroll extent를 fixture에서 수집한다. |
| 11 | one-axis controls가 빈 반복의 independent collapse를 검증하지 않는다. | added-column control에서 rows `[20,0]`, added-row control에서 columns `[20,0]`을 본다. |
| 12 | origin-relative item frame의 기준점이 불분명하다. | README와 계획이 모든 item frame을 grid content origin 기준으로 명시한다. |
| 13 | fixed root의 CSS box model이 52px repeat basis와 다르다. | `box-sizing:border-box`, zero padding/border, fixed width/height를 공통 fixture에 둔다. |
| 14 | browser observation을 normative expected data로 오인한다. | README와 plan은 독립 CSS 계산과 Chrome 관측을 별도 표기하고 browser JSON은 observation만 저장한다. |
| 15 | Chrome binary version, Agent Browser version, viewport 또는 DPR가 빠져 재현할 수 없다. | README와 `observation.json`에 버전·환경을 기록하고 DPR 1을 명시한다. |
| 16 | screenshot이 raw HTML observation과 다른 입력 revision을 가리킨다. | JSON·screenshot·fixture 모두 같은 fixed input에서 생성했으며 SHA-256 manifest에 각각 기록한다. |
| 17 | fixture 또는 evidence 링크의 상대 경로가 저장소 내 실제 파일을 가리키지 않는다. | plan→evidence, evidence→fixture, screenshot 링크를 staged path와 대조했다. |
| 18 | Chrome oracle 화면을 Spinon 또는 mobile simulator 성공 화면으로 확대 해석한다. | 사전 비교 README에서 browser-only 한계를 명시하고 실제 runtime 검증을 별도 merge 후 gate로 남겼다. |
| 19 | runtime 변경이 기존 profile/fixture artifact 경계를 느슨하게 만든다. | plan은 현재 dedicated fixture-test feature의 default negative와 opt-in positive archive 대조를 유지한다. |
| 20 | 새 7개만 테스트하거나 실제 기기·Tailnet 변경을 계획 PR에서 먼저 수행한다. | 구현 gate를 12+13+9+7=41 scene으로 고정하고 AVD/iOS Simulator와 별도 sync PR을 명시한다. |

**판정:** 이 계획 diff에서 수정이 필요한 결함을 찾지 못했다. 이 검토는 계획 PR 변경에 대한 검토이며 구현/runtime 검토를 대신하지 않는다.
