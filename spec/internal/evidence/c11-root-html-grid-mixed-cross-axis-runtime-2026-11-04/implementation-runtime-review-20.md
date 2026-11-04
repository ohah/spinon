# C11.1 mixed cross-axis 구현/runtime 실패 관점 검토 20개

대상: 현재 candidate source diff, focused Rust tests, Chrome 156 precomparison, Agent Browser fixture, Android API 37 AVD PixelCopy, iPhone 17 Pro/iOS 26.2 Simulator Metal drawable. 이 검토는 일반 CSS Grid 적합성 검증이 아니다. `코드 확인`은 해당 불변식의 fixture runtime 실행을 뜻하지 않는다.

| # | 별도 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 서로 다른 Grid owner의 row-locked와 column-locked item을 합쳐 새 mode를 선택하는가 | Selector는 `items_by_owner`로 분리한 뒤 owner별로 두 유형을 센다. **PASS, 코드 확인.** |
| 2 | 한 축만 definite한 입력이 새 mixed route에 잘못 들어오는가 | 새 selector는 같은 owner에 row-locked와 column-locked가 각각 있어야 한다. 기존 row-only/column-only selector는 별도로 둔다. **PASS, selector 구조 확인.** |
| 3 | all-auto-only Grid가 새 profile로 선택되는가 | fully-auto는 허용 item 종류지만 두 partial-axis count를 만들지 않아 단독으로 선택되지 않는다. **PASS, guard 확인.** |
| 4 | fully-definite-only Grid가 새 profile로 선택되는가 | 두 partial-axis count 조건을 만족하지 않는다. 기존 fully-definite route는 그대로다. **PASS, guard 확인.** |
| 5 | 뒤에 선언된 fully definite item의 occupied cell을 놓쳐 partial item이 겹치는가 | shared sparse normalizer가 definite placement phase를 먼저 적용하며 Chrome `d-last` control과 Root HTML frames가 이를 확인한다. **PASS, Chrome/Rust/앱 실행.** |
| 6 | row-locked와 column-locked의 DOM interleave가 placement phase 순서를 바꾸는가 | CSS sparse row oracle을 따르고 fixture의 `dom-cross-axis`가 교차 순서를 고정한다. **PASS, Chrome·Android·iOS frames와 표면 표본.** |
| 7 | column-locked item이 고정 column을 벗어나거나 row cursor가 잘못 이동하는가 | definite column을 유지하는 sparse placement 결과를 Chrome 독립 frame과 대조한다. **PASS, Chrome/Rust/앱 실행.** |
| 8 | fully-auto item이 locked item의 occupancy를 무시하는가 | fully-auto가 mixed mode에서만 허용되고 shared sparse occupancy를 사용한다. `auto-last` frame을 확인한다. **PASS, Chrome/Rust/앱 실행.** |
| 9 | DOM/source order를 placement 계산을 위해 실제 child order에서 재정렬하는가 | 새 selector는 owner/item 집합을 분류할 뿐 HostDocument children을 수정하지 않는다. reverse/interleaved control의 frame을 비교했다. **PASS, 코드·fixture 확인.** |
| 10 | `display:none` item이 유형 수나 occupancy를 바꾸는가 | `RuntimeGridNodes`에서 제외된 item만 selector가 분류한다. fixture control의 zero frame과 successor 배치를 확인했다. **PASS, Chrome/앱 실행.** |
| 11 | `visibility:hidden` item이 눈에 안 보인다는 이유로 occupancy에서 빠지는가 | hidden은 display 기반 후보 필터에 의해 제거되지 않고 shared layout occupancy에 남는다. 후속 item 위치와 고정 sample을 확인했다. **PASS, Chrome/Rust/앱 실행.** |
| 12 | 양수 row/column span이 점유한 후속 cell을 놓치는가 | sparse normalizer의 span occupancy와 `mixed-spans` frame을 대조했다. **PASS, focused Rust 및 Chrome/앱 실행.** |
| 13 | negative numeric line이 explicit grid 경계 밖에서 wrap/overflow되는가 | explicit grid 내부 negative line 해석 test와 out-of-range rejection test가 있다. **PASS, layout tests.** |
| 14 | zero span 또는 음수 span이 빈/역방향 영역을 만드는가 | endpoint validator는 양수 numeric span만 통과시키며 zero/invalid span focused controls가 있다. **PASS, 코드·Rust tests.** |
| 15 | 한 축의 endpoint만 `auto`, 다른 endpoint는 numeric/span인 placement가 조용히 정규화되는가 | `locked_axis`는 definite-axis 양 endpoint 형태만 허용한다. **PASS, validator 확인.** |
| 16 | named line/span이 숫자 위치로 잘못 추측되는가 | selector와 layout validator 양쪽에서 named placement를 candidate에서 제외/거부한다. **PASS, fail-closed guard 확인.** |
| 17 | named area, repeat line-name 또는 area alias가 numeric sparse path로 들어오는가 | area, line names, fixed/auto repeat metadata가 selector에서 candidate를 제외하고 validator도 area/repeat를 거부한다. **PASS, 두 경계 확인.** |
| 18 | dense 또는 column flow가 sparse row 결과로 오인되는가 | selector는 row flow와 non-dense owner만 받고 validator는 `GridAutoFlow::Row`를 요구한다. **PASS, 코드 확인.** |
| 19 | out-of-grid explicit placement로 implicit extent가 필요할 때 부분 geometry가 publish되는가 | implicit extent를 허용하지 않고 기존 normalizer의 error 반환 경로를 사용한다. Chrome rejection boundary와 Rust explicit-grid rejection이 있다. **PASS, rejection boundary 확인.** |
| 20 | Android/iOS run ID 또는 표본 정의가 다른 scene에 재사용되어 잘못 match되는가 | 양 플랫폼 adapter는 고정 run ID·scene ID·26 sample 집합을 선택한다. PixelCopy와 final Metal summary 모두 26/26 exact다. Flag-off APK DEX와 iOS executable scan에는 새 adapter/run marker가 없다. **PASS, AVD/Simulator runtime 및 default/opt-in artifact scan.** |

## 종합

20개의 서로 다른 실패 관점을 검토했다. 확인 중 blocker는 남지 않았다. Rust focused 검사와 두 Simulator 표면 결과는 각각의 제한된 입력만 입증한다. 실기기, hardware GPU, iOS compositor 이후 pixels, WPT 및 전체 C11은 미검증이다. 별도 실제 implementation source diff review와 PR diff review는 각각 따로 남긴다.
