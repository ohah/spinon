# C11.4 cyclic percentage `minmax()` 계획 PR 변경 검토 30개

검토 대상 변경: 상위 C11.4 track sizing 계획에 후속 slice를 연결하고, `plan/c11-4-cyclic-percent-minmax.md`, 계획 검토 20개, 독립 기대값/Chrome 156 사전 비교 fixture·자료를 추가했다. 계획 변경은 runtime 구현을 포함하지 않는다.

| # | 실패 관점 | 확인한 경계와 결과 |
| ---: | --- | --- |
| 1 | 새 계획이 C11.4 전체를 완료처럼 보임 | 상위 checkbox와 `72/213` 유지 문구를 명시했다. 통과. |
| 2 | 새 계획이 user-facing CSS 지원 선언으로 해석됨 | opt-in fixture-test 전용임을 plan/status에 적었다. 통과. |
| 3 | Chrome 수치가 규범 기대값의 출처가 됨 | 기대값은 CRD §2.3에서 독립 도출하고 Chrome은 관찰 열로 분리했다. 통과. |
| 4 | 관찰 환경에서 Chrome 155를 사용한 것으로 오기 | 문서와 JSON은 `HeadlessChrome/156.0.0.0`을 기록하고 155 baseline과 분리한다. 통과. |
| 5 | fixture title·경로가 실제 파일과 불일치 | 새 fixture 파일명과 evidence의 상대 링크를 대조했다. 통과. |
| 6 | CSS declaration이 실제 브라우저에서 parse되지 않음 | Agent Browser CSSOM에 `12px`, `8px`, `20px` used track이 기록되어 있다. 통과. |
| 7 | track 값이 root frame 기대와 혼동됨 | JSON은 computed track과 root/item rect를 각각 보존한다. 통과. |
| 8 | cyclic height가 width sizing에 영향을 줌 | row scene width는 24px definite, height만 max-content다. 통과. |
| 9 | cyclic width scene에서 row 기여가 불명확 | item height는 16px이고 `grid-template-rows:auto` used row가 16px이다. 통과. |
| 10 | definite control이 다른 track grammar를 사용 | control도 같은 `minmax(0px,50%)`를 사용하고 width만 40px definite다. 통과. |
| 11 | 단일 item의 auto-placement가 두 번째 track을 만듦 | root마다 item 하나와 explicit track 하나를 사용하고 Chrome computed list는 한 track이다. 통과. |
| 12 | intrinsic sizing basis에 border/padding이 더해짐 | 입력은 border/padding/gap 0으로 고정했다. 통과. |
| 13 | item 크기 변화가 track contribution을 흔듦 | 모든 item은 24×16px fixed size다. 통과. |
| 14 | item overflow가 계획에서 누락 | column x=12..24와 row y=8..16 구간의 track overflow를 설명했다. 통과. |
| 15 | item overflow를 root overflow로 잘못 부름 | cyclic root/item은 같은 24×16px frame이고 overflow는 track 경계 안쪽에 한정됨을 기록했다. 통과. |
| 16 | surface boundary coordinate가 device coordinate와 혼동 | implementation 단계에서 raw surface coordinate/display scale을 기록하도록 요구한다. 통과. |
| 17 | PixelCopy가 실제 Android window 전체를 증명 | PixelCopy를 SurfaceView 표본으로 한정했다. 통과. |
| 18 | iOS drawable 표본이 compositor-after 검증으로 확대 | Metal 표본을 compositor 제출 전으로 한정했다. 통과. |
| 19 | screenshot만으로 layout trace가 대체됨 | track arrays·node frames·trace·draw/present를 별도 acceptance로 둔다. 통과. |
| 20 | draw/present 전에 화면을 캡처 | 실행 순서에 해당 scene draw/present 후 표본을 요구한다. 통과. |
| 21 | 기본 product archive에 fixture ABI/payload가 유입 | default negative와 fixture-test positive artifact 대조를 각각 요구한다. 통과. |
| 22 | 실패 장면의 부분 layout이 성공처럼 남음 | allowlist/source validation과 non-partial error acceptance를 명시했다. 통과. |
| 23 | 여러 percentage/minmax 조합까지 자동 지원으로 해석 | three-scene fixed allowlist 외 입력 거부를 요구한다. 통과. |
| 24 | `calc()`가 포함된 max function까지 포함으로 오해 | `calc()`와 percentage minimum은 범위 밖으로 별도 명시했다. 통과. |
| 25 | `fit-content(%)` 검증으로 확대 | percentage fit-content는 범위 밖으로 둔다. 통과. |
| 26 | 동시에 cyclic columns/rows를 다룬 것으로 오해 | 각 cyclic scene에서 한 축만 intrinsic이며 simultaneous case는 범위 밖이다. 통과. |
| 27 | nested/anonymous item coverage로 확대 | direct root + single item topology만 plan에 둔다. 통과. |
| 28 | WPT 결과나 적합성 완료로 확대 | WPT는 미실행/범위 밖이며 적합성 주장을 금지한다. 통과. |
| 29 | 계획 단계에서 simulator를 실행한 것으로 오기 | 사전 비교 README에 parser/runtime/simulator 미검증을 명시했다. 통과. |
| 30 | 계획 검토가 구현 검토로 재사용됨 | plan review 20, 향후 implementation review 20, PR delta review 30을 각각 분리했다. 통과. |

판정: 위 변경은 계획·독립 계산·Chrome 156 observation에 한정된다. implementation code, runtime, simulator 결과 또는 지원 범위를 추가하지 않는다.
