# C11.4 mixed cross-axis span groups 구현·runtime 실패 관점 검토 20개

이 검토는 Rust sizing fixture와 trace, FFI scene adapter, Android/iOS runtime validators, 실제 Simulator/AVD 기록을 대상으로 한다. 계획 문서 검토와 PR diff 검토는 별도 파일이다. 이 slice의 판정은 부분 검증이며, Android definite presentation은 통과로 세지 않는다.

| # | 실패 경로·불변 조건 | 확인 결과 |
| ---: | --- | --- |
| 1 | 신규 fixture가 제품 기본 빌드에 유입되는가 | fixture-test feature 경로에 제한되고 default release archive에서 ID와 entry symbol이 없다. 통과. |
| 2 | scene prefix나 유사 ID가 고정 adapter에 들어오는가 | Rust FFI adapter는 네 exact ID만 매칭하며 다른 입력을 거부한다. 통과. |
| 3 | scene별 intrinsic/definite width가 뒤바뀌는가 | intrinsic은 min-content, definite는 40px content box로 고정하고 shape 검사가 profile을 구분한다. 통과. |
| 4 | percentage minimum의 cyclic basis 또는 gap 공제가 달라지는가 | 세 열에 `minmax(calc(10% + 2px), max-content)`, gap 2px을 고정하고 독립 기대값과 trace를 비교한다. 통과. |
| 5 | implicit track 생성이나 명시적 Grid 크기가 scene마다 달라지는가 | 고정 3×2 topology 및 `55×24`/`44×44` root geometry를 validator가 검사한다. 통과. |
| 6 | span 그룹 정렬이 span-3을 span-2보다 먼저 처리하는가 | 두 span-2 item의 contribution 뒤 span-3 item contribution을 Rust trace에서 검사한다. 통과. |
| 7 | C의 3-column placement가 2-column span으로 축소되는가 | adapter source와 fixture shape validator가 A/B/C span 2/2/3을 확인한다. 통과. |
| 8 | 모든 item이 두 행을 span하지 않거나 row placement가 바뀌는가 | 세 direct item의 row span 2를 exact style/placement 검사로 확인한다. 통과. |
| 9 | text 측정 입력이 예상한 wrapping 차이를 만들지 않는가 | A/B/C min-content·max-content width와 height가 플랫폼 runtime diagnostics에서 세 항목 모두 확인된다. 통과. |
| 10 | reverse scene이 DOM order만 바꾸지 않고 item identity도 바꾸는가 | 안정된 node ID와 item ID를 유지하고 sourceOrder 및 A/B/C metric 배열을 반전한다. 통과. |
| 11 | font 측정이 다른 registry revision에 기대는가 | Android/iOS validator가 각 intrinsic input의 `fontRegistryRevision=1`을 요구한다. 통과. |
| 12 | 초기 style pass의 유효 revision 0을 잘못 거부하는가 | runtime 진단에서 확인된 최초 revision에 맞춰 불필요한 1 가정을 두지 않았다. 통과. |
| 13 | min-content bases와 intermediate planned increase가 혼동되는가 | 테스트는 bases `[9,19,19]`, planned increase `[7,17,17]`, final Columns를 각각 별도 trace 값으로 대조한다. 통과. |
| 14 | columns sizing 결과가 rows에서 wrap 계산에 반영되지 않는가 | Rust focused test와 runtime scene은 intrinsic Rows `[9,9]`, definite Rows `[19,19]`를 확인한다. 통과. |
| 15 | forward/reverse에서 final track order나 geometry가 달라지는가 | focused geometry test, iOS 네 scene, Android intrinsic 두 scene에서 동일한 기대 geometry가 확인됐다. Android definite는 추가 표본 없음. 부분 통과. |
| 16 | root frame은 맞지만 node 순서·위치·너비·높이가 다른가 | 두 platform validators가 root와 A/B/C의 4개 frame 및 stable node order를 `1/64 CSS px` 이내로 확인한다. Android definite forward는 runtime frame까지 통과했다. |
| 17 | definite B가 root 밖으로 나오지 않거나 clear 표본이 item에 겹치는가 | iOS 두 definite scene에서 B overflow red와 이후 clear black Metal 표본이 4/4 exact다. Android PixelCopy는 미수집. |
| 18 | iOS drawable readback이 compositor 이후 화면 증거로 과장되는가 | 근거 문서와 PR 설명에서 Metal readback을 compositor 제출 전으로 한정한다. 통과. |
| 19 | Android layout 완료를 화면 draw/present 성공으로 오인하는가 | intrinsic 2/2는 draw/present됐다. definite forward는 frame 이후 SurfaceFlinger EGL/QEMU pipe에서 멈춰 renderer/draw/PixelCopy가 없고, reverse는 미실행이다. Android gate 미완료로 기록했다. |
| 20 | 주변 회귀 또는 검사 도구의 경고를 숨기는가 | fixture boundary 18개, focused tests, formatter와 feature Clippy 두 crate가 통과했다. 기존 adjacent test의 Clippy `.values()` 수정도 diff review에 기록했다. 기존 C11.4 전체 회귀 matrix는 미실행이다. |

## 판정

Rust sizing/trace, 고정 scene shape, iOS Simulator 4/4와 Android intrinsic 2/2는 근거가 있다. Android definite 장면의 draw/pixel evidence 및 기존 C11.4 전체 회귀 matrix가 남아 있어 전체 플랫폼 통과나 C11.4 완료로 승격하지 않는다. Android 제한 원인 자료는 [AVD recovery 기록](./android-avd-recovery-readonly-20261022/)에 있다.
