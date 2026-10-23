# C11.4 auto-repeat calc 최소값 계획 수정 검토 20개

이 표는 구현 중 확인한 Stylo computed-value 정규화에 맞춰 수정한 계획만 새로 검토한다. 기존 계획 검토 20개와 구현/runtime 검토 20개, PR diff 검토 30개에 합산하지 않는다.

| # | 독립 실패 관점 | 재검토와 판정 |
| --- | --- | --- |
| 1 | 계획이 거부 원인을 여전히 auto-repeat allowlist로 오인한다. | `LengthPercentage::unpack()`이 고정 입력을 `LengthPx(40)`으로 정규화하고 기존 allowlist가 이를 받는다는 테스트 결과를 적었다. 신규 evaluator가 필요하다는 문장을 삭제했다. |
| 2 | 구현 범위가 수정 목표와 달라 죽은 evaluator를 요구한다. | 범위를 `calc(20px + 20px)`과 fixed `40px` control의 기존 경로 비교로 좁혔다. 연산 evaluator·텍스트 재파싱·일반 admission은 추가하지 않는다. |
| 3 | 열거한 10개 산술식을 이번 런타임 검증 결과로 과도하게 일반화한다. | 산술식 10개는 초기 cascade 조사 결과로만 남겼고, 이번 고정 runtime fixture에서 입증하는 입력은 `calc(20px + 20px)` 하나로 명시했다. |
| 4 | CSS 값이 computed `Calc`로 남는다는 오래된 타입 전제를 유지한다. | plan 본문에서 “Stylo computed `Calc`”라는 전제를 초기 반증 사실로만 기록했다. 최신 범위는 computed `LengthPx`와 기존 fixed-size path를 사용한다. |
| 5 | absolute calc 결과를 모든 차원·범위 조합에서 지원한다고 확대 해석한다. | 이 fixture의 canonical expression과 `40px` 값에만 결론을 한정했다. 다른 연산자·범위·차원은 미검증으로 둔다. |
| 6 | percentage calc의 거부를 새 구현의 결함으로 오인한다. | percentage 식을 별도의 fail-closed control로 두고 `UnsupportedComputedGridValue`와 node/property 문맥을 검사한다. |
| 7 | fixed-size repeat count의 최소 track math가 잘못됐다. | 184px definite axis, 최소 40px, gap 0에서 repeat count 4를 독립 계산하고 남은 24px을 네 `1fr` track에 나눠 각각 46px로 고정했다. |
| 8 | `auto-fill`과 `auto-fit`의 차이를 item placement와 무관하게 계산한다. | fill은 네 item과 네 46px track을 유지하고 fit은 한 item 배치 뒤 세 빈 반복 track을 collapse해 `[184,0,0,0]`을 기대한다. |
| 9 | rows matrix가 columns와 다른 자동 배치 축을 가진다. | rows scene은 `grid-auto-flow:column`을 고정하고 vertical item frame·row track을 JSON expectation, 두 platform의 runtime trace와 비교한다. |
| 10 | 고정 size·gap 조건이 불필요한 sizing 변수를 만든다. | 각 장면에서 한 축만 repeat하고 다른 축과 gap을 고정해 분배 결과가 식별 가능하도록 했다. |
| 11 | fixed numeric control이 실제 같은 typed track인지 증명하지 못한다. | four axis/mode 조합에서 calc snapshot track 전체를 동일 조건의 `minmax(40px,1fr)` snapshot과 비교하고 `LengthPx(40)`/`Fraction(1)`을 따로 assert한다. |
| 12 | test가 잘못된 cascade node의 결과를 읽는다. | 각 snapshot에서 fixture root의 `node_id`를 찾아 grid layout을 읽도록 구현돼 있다. |
| 13 | errors가 다른 node나 property에 귀속돼도 통과한다. | percentage negative control은 error variant, root node ID, `grid-template-columns` property를 동시에 고정한다. |
| 14 | 새 fixture가 fixture-only module boundary 밖에서 컴파일된다. | scene data와 runtime adapter를 기존 `c04-runtime-gpu-fixture-test` opt-in `fixtures` 경로에 두며 기본/product archive 부정 대조와 positive control을 요구한다. |
| 15 | Rust fixture scene과 Android/iOS allowlist가 서로 어긋난다. | 캡처 검증기는 6개 ID 모두 양 platform allowlist에 있는지 확인하고 Android/iOS 런타임에서 각각 6/6 exact ID를 실행했다. |
| 16 | host 기대값 코드가 node 순서·box 수를 틀려도 scene 캡처가 통과한다. | scene 별 root/item frame, node index, box count를 고정하고 실제 Android/iOS 관측 배열이 기대값과 exact match하는지 확인했다. |
| 17 | capture catalog 수나 중복 ID 때문에 새 fixture route가 누락된다. | boundary test가 전체 capture catalog의 기대 scene 수와 unique ID, fixture route, platform allowlist를 검사한다. |
| 18 | 시뮬레이터 화면을 pixel equality 또는 실기기 근거로 과장한다. | 계획에 screenshot은 장면 표시 근거이며 PixelCopy/Metal pixel equality는 실행하지 않았다고 적었다. AVD/iOS simulator 범위도 분리한다. |
| 19 | draw/present 이전 layout 결과만으로 앱 runtime 성공을 선언한다. | scene summary마다 `layout=ready`, node frames, stage trace와 `drawPresented=true`가 있어야 통과하며 superseded retry는 0이어야 한다. |
| 20 | 제한 fixture를 C11.4 완료 또는 잘못된 공식 집계로 승격한다. | C11.4 checkbox는 미완료로 두고 internal contract `0.1.0`, 공식 `74/207`을 유지하며 이번 slice가 집계 숫자를 바꾸지 않는다고 기록한다. |

## 남은 확인

표의 계획 주장 중 simulator 실행, workspace 검증, static archive 경계, evidence hash는 구현/runtime 결과가 생성된 뒤 별도 구현 검토와 evidence README에서 재확인한다. 이 문서는 계획 수정 검토만 기록한다.
