# C11.4 flexible track runtime 계획 PR 변경 검토 · 30관점

대상 diff: [flex-stage runtime 계획](./c11-4-flex-stage-runtime.md), [독립 계획 검토 20개](./c11-4-flex-stage-runtime-plan-review-20.md), [상위 계획 인덱스](./c11-4-grid-track-sizing.md). 코드·공식 상태·제품 명세 변경은 포함하지 않는다.

| # | 실패 관점 | 변경 검토 결과 |
|---:|---|---|
| 1 | 계획이 이전 runtime slice를 새 작업으로 재제안할 수 있다. | 신규 계획은 PR #717의 pure Rust flex oracle 후속으로 한정했고 다른 scene ID를 제안하지 않는다. PASS |
| 2 | 활성 상위 항목을 잘못 표시해 공식 상태와 충돌할 수 있다. | C11.4 미완료와 `72/213` 유지가 계획과 상위 문서에 명시돼 있다. PASS |
| 3 | 규범 source가 다른 CSS Grid 판본일 수 있다. | expected JSON과 기존 계획에서 고정한 CSS Grid 2025 CRD §11.7.1을 참조한다. PASS |
| 4 | 브라우저 serialization이 exact oracle이 될 수 있다. | Chrome 156 control과 exact rational expected를 분리한다. PASS |
| 5 | scene 누락이나 임의 확장으로 oracle이 바뀔 수 있다. | expected JSON의 네 ID만 허용하고 정확한 집합 검사를 요구한다. PASS |
| 6 | 각 축의 definite 크기가 뒤바뀔 수 있다. | sizing axis는 200px, 반대 축은 120px로 scene별 기대 크기를 고정한다. PASS |
| 7 | 10px gap이 누락되거나 양축에 적용될 수 있다. | target 축 한 곳의 gap과 반대 축 0을 matcher가 확인한다. PASS |
| 8 | stage 존재만 보고 잘못된 순서를 통과시킬 수 있다. | Maximize → ExpandFlexible → Stretch의 순서와 stage 유일성을 요구한다. PASS |
| 9 | vector 위치 변경으로 gutter를 track처럼 읽을 수 있다. | trace flags로 CSS track을 선택하고 gutter/boundary topology를 별도 확인한다. PASS |
| 10 | root dimensions 또는 viewport가 unspecified일 수 있다. | root와 viewport의 definite size 및 root-relative frame 표를 고정한다. PASS |
| 11 | padding/border/margin이 content box를 바꿀 수 있다. | root/marker의 box style zeroing과 border-box sizing을 plan에 기록했다. PASS |
| 12 | 눈에 보이는 marker item이 intrinsic sizing에 개입할 수 있다. | Chrome과 같은 빈 marker, zero min-size, 무content를 요구하고 runtime output을 oracle에 비교한다. PASS |
| 13 | marker DOM order와 semantic ID가 섞일 수 있다. | `a`, `b` marker ID 및 cell mapping을 fixture/source test에서 확인하도록 했다. PASS |
| 14 | `weighted-columns` 분수 결과가 반올림 오류로 오판될 수 있다. | runtime 수치는 rational expected에 `1e-4px`, Chrome 값에 별도 `1/64px` 기준을 쓴다. PASS |
| 15 | subunit factor의 남는 85px를 놓칠 수 있다. | track 합·gap·남은 공간을 명시적으로 비교한다. PASS |
| 16 | frozen minimum과 zero-fr floor case가 일반 비율 case로 오인될 수 있다. | 네 JSON case의 배열을 각각 보존하고 구별한다. PASS |
| 17 | platform 상수 table이 JSON과 달라질 수 있다. | 독립 verifier가 두 플랫폼의 상수와 source JSON, raw runtime logs를 대조한다. PASS |
| 18 | rational parser가 0 denominator, overflow 또는 non-finite를 허용할 수 있다. | 양수 denominator, checked parsing 및 finite `f32` 검사와 negative controls를 요구한다. PASS |
| 19 | expected report를 빠뜨리거나 중복 scene 실행으로 통과시킬 수 있다. | platform별 4개 report와 정확한 unique scene 집합을 offline verifier가 검사한다. PASS |
| 20 | `expected == null` fallback이 matcher 성공으로 오인될 수 있다. | Android/iOS 전용 strict matcher와 unknown/profile mismatch 거부를 계획했다. PASS |
| 21 | 다른 profile의 trace가 같은 scene ID로 들어올 수 있다. | `RuntimeBlockFlexGridTrackSizingV1` profile 확인을 요구한다. PASS |
| 22 | trace 누락과 빈 stage 배열을 완료로 기록할 수 있다. | root trace, axis pass, stage sequence 및 track values를 모두 요구한다. PASS |
| 23 | 예상한 frame만 임의로 검사하고 root/node 누락을 숨길 수 있다. | 전체 3 node의 root-relative frame을 scene별로 고정한다. PASS |
| 24 | ready까지만 수행되고 surface에 그리지 않을 수 있다. | raw run 로그에서 draw와 presented 결과를 둘 다 보존한다. PASS |
| 25 | screenshot을 pixel exact 증거로 과장할 수 있다. | screenshot은 배치 보조 자료로 제한하고 별도 readback 없이 equality를 주장하지 않는다. PASS |
| 26 | Android AVD와 iOS Simulator 결과를 실기기 근거로 확대할 수 있다. | target 종류·OS를 manifest에 기록하고 실기기/hardware GPU를 제외 사항으로 둔다. PASS |
| 27 | fixture ABI가 기본 Rust archive에 들어갈 수 있다. | default/opt-in static archive와 positive control을 각각 검사한다. PASS |
| 28 | static archive 부재를 앱 전체 제외로 잘못 해석할 수 있다. | APK/IPA 전체 exclusion은 이 증거 범위가 아니라고 명시한다. PASS |
| 29 | 실행이 기존 booted simulator나 관계없는 세션을 종료할 수 있다. | pre-run boot 상태를 보존하고 fixture app만 종료하도록 했다. PASS |
| 30 | 계획 PR이 구현·지원 완료를 암시하거나 브라우저 운영 경계를 어길 수 있다. | 계획과 구현을 구분하고 일반 CSS/WPT/C11.4 완료 주장을 금지하며 Terminal Browser를 사용하지 않는다. PASS |

### 결론

30개 관점은 plan diff의 source, scene topology, 수치 허용치, 앱/플랫폼 경계, 오류·artifact·상태 보고를 각각 확인했다. 계획 문서 링크는 로컬에서 확인했다. 이 검토는 구현 실행이나 PR CI 통과를 뜻하지 않는다.
