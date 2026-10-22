# C11.4 mixed cross-axis span groups 구현·runtime 실패 관점 검토 20개

이 검토는 Rust sizing fixture와 trace, FFI scene adapter, Android/iOS runtime validators, 실제 Simulator/AVD 기록을 대상으로 한다. 계획 문서 검토와 PR diff 검토는 별도 파일이다. 이 slice의 판정은 부분 검증이며, Android definite presentation은 통과로 세지 않는다.

| # | 실패 경로·불변 조건 | 확인 결과 |
| ---: | --- | --- |
| 1 | 신규 fixture가 제품 기본 빌드에 유입되는가 | fixture-test feature 경로에 제한되고 default release archive에서 ID와 entry symbol이 없다. 통과. |
| 2 | scene prefix가 pure anonymous text를 text/image fixture로 잘못 보낼 수 있는가 | Android full matrix에서 `anonymous-min-content-wrap` 오류를 재현했다. matcher를 `anonymous-text-image-`로 좁혔고, five pure-text controls와 Android/iOS 전체 matrix에서 통과했다. |
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
| 15 | forward/reverse에서 final track order나 geometry가 달라지는가 | Android/iOS 네 장면씩 모두 draw/present됐고 같은 intrinsic/definite tracks와 order에 맞는 A/B/C frames를 반환했다. 통과. |
| 16 | root frame은 맞지만 node 순서·위치·너비·높이가 다른가 | Android/iOS validators가 root와 A/B/C 네 frame을 비교했다. intrinsic Columns의 실제 `[15.666666]*3`은 독립 기대값과 1/64 CSS px 이내이고 definite tracks/frame은 exact다. 통과. |
| 17 | definite B가 root 밖으로 나오지 않거나 clear 표본이 item에 겹치는가 | Android PixelCopy와 iOS Metal에서 forward/reverse 각각 overflow red와 past-item clear black을 2/2 exact, 플랫폼별 4/4로 확인했다. |
| 18 | iOS drawable readback이 compositor 이후 화면 증거로 과장되는가 | 근거 문서와 PR 설명에서 Metal readback을 compositor 제출 전으로 한정한다. 통과. |
| 19 | Android layout 완료를 화면 draw/present 성공으로 오인하는가 | 최초 SwiftShader AVD는 WindowManager/system_server watchdog 중단으로 실패했다. host GPU Apple M4 Max로 재실행한 Android API 37 ARM64 AVD는 전체 263/263 draw/present, PixelCopy 219/219 exact다. 실기기 증거로 확대하지 않는다. |
| 20 | 주변 회귀 또는 검사 도구의 경고를 숨기는가 | formatter, feature Clippy, fixture boundary 18개와 기존 focused tests를 기록했다. Android/iOS C11.4 263-scene matrix도 각각 263/263 draw/present를 통과했다. Pixel oracle border 2곳을 함께 수정했다. |

## 판정

Rust sizing/trace, 고정 scene shape, 네 PR 장면과 C11.4 전체 simulator regression matrix가 근거에 있다. Pixel 표본 대상 scene은 플랫폼별 63/63, 표본은 219/219 exact다. 이 결과는 제한 fixture와 simulator 범위로 남기며 C11.4 전체 적합성으로 승격하지 않는다. 수정 전 anonymous routing failure와 SwiftShader AVD watchdog 근거, 수정 후 양 플랫폼 결과는 [PR #515 최종 회귀 근거](../c11-4-pr515-prereq-regression-2026-10-22/README.md)에 있다.
