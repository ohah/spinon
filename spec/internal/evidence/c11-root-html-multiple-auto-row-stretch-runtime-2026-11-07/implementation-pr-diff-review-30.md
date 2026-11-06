# C11.4 두 implicit auto 행 구현 PR diff 검토 30개

계획 PR #1230의 범위와 현재 구현 PR에 포함될 코드·fixture·RuntimeSession·simulator evidence를 서로 다른 관점에서 대조했다. 이 문서는 계획 검토 20개나 구현/runtime 검토 20개를 대체하지 않는다.

| # | PR diff 확인점 | 결과 |
|---:|---|---|
| 1 | 변경이 계획된 C11.4 두 implicit `auto` 행 장면으로 한정되는가 | 통과. 일반 Grid admission이나 WPT 지원은 넓히지 않았다. |
| 2 | Root HTML 기본 route가 유지되는가 | 통과. 새 API나 route selector 없이 기존 parsed Root HTML path를 쓴다. |
| 3 | 새 public API, enum variant, profile ID가 추가되는가 | 추가 없음. |
| 4 | C11.4/C11 완료 상태를 앞당기거나 공식 완료 개수를 올리는가 | 해당 상태 문서를 변경하지 않았다. `78/214`를 유지한다. |
| 5 | 규범 근거와 구현 전 비교 모델이 연결되는가 | 통과. 계획의 CSS Grid §11.8 근거와 HeadlessChrome 157 comparison을 사용한다. |
| 6 | 런타임 fixture가 40×40 definite content box인가 | 통과. |
| 7 | explicit column/row와 auto-flow가 계획값과 일치하는가 | 통과. 40px column, 10px row, row auto-flow다. |
| 8 | gap, padding, border가 독립 기대값에 섞이는가 | 없음. 0 gap 및 0 padding/border다. |
| 9 | 첫 child가 40×10 fixed empty item인가 | 통과. |
| 10 | 후속 text가 계획한 `A`, `B` direct text인가 | 통과. source order도 검사한다. |
| 11 | text item의 line contribution이 각각 10px인가 | 통과. runtime fixture trace가 각 item의 10px minimum contribution을 확인한다. |
| 12 | auto track에 남은 free space가 10px인가 | 통과. explicit base 10px와 두 minimum base 10px 이후 40px container에 10px이 남는다. |
| 13 | 분배량이 코드 상수로 하드코딩되는가 | 없음. projection은 shape만 승인하고 기존 layout track sizing이 분배한다. |
| 14 | 두 auto row가 각각 5px을 받는가 | 통과. trace 결과 `[10,15,15]`다. |
| 15 | direct item frame이 계획값과 일치하는가 | 통과. `[0,0,40,10]`, `[0,10,40,15]`, `[0,25,40,15]`를 검사한다. |
| 16 | single implicit auto-row scene이 회귀하는가 | 통과. 기존 `[10,30]` scene과 trace 테스트가 유지된다. |
| 17 | min-content 및 fixed-row 경로가 영향을 받는가 | production route를 바꾸지 않았다. 기존 intrinsic row 20개 및 runtime 22개 회귀가 통과한다. |
| 18 | `align-content:start`를 새 지원 장면으로 잘못 받아들이는가 | 아니다. Chrome control이며 projection은 거부한다. |
| 19 | text grammar가 일반 임의 text로 확대되는가 | 아니다. 두 text scene은 exact `A`/`B`; 기존 한 text scene grammar도 유지한다. |
| 20 | 네 번째 child, 순서 변경, A/B 오입력이 partial result를 게시하는가 | 아니다. 모두 `completed=None`으로 거부된다. |
| 21 | whitespace node나 hidden child가 shape indexing을 혼란시키는가 | 기존 whitespace/hidden filtering 뒤 visible source order를 검사한다. |
| 22 | source `NodeId`, paint order, style/layout revision 검사가 있는가 | 통과. 각 text frame identity와 revision 및 draw command 순서를 확인한다. |
| 23 | fixture-test 전용 trace가 release code를 바꾸는가 | 아니다. 진단 assertion만 `c11-4-grid-track-sizing-fixture-test` 아래 있다. |
| 24 | release rlib에 새 HTML fixture marker가 유입되는가 | 없다. artifact scan에서 marker 부재를 확인했다. |
| 25 | Android build가 C11 fixture feature를 켜는가 | 아니다. C04 runtime GPU만 켜고 C11 fixture flag는 0이다. |
| 26 | Android APK 안 fixture가 source fixture와 일치하는가 | 통과. SHA-256이 `05b781873d7fa88b73f19f4dde4acb069f1f44bdbbb204c6613576653917d061`다. |
| 27 | Android 앱이 기본 Root HTML에서 ready/draw하는가 | 통과. API 37 ARM64 AVD에서 layout ready, 6 boxes, 2 runtime text 기록을 확인했다. |
| 28 | iOS 앱 안 fixture가 source fixture와 일치하는가 | 통과. SHA-256이 Android와 같다. |
| 29 | iOS 앱이 기본 Root HTML에서 ready/draw하는가 | 통과. iPhone 17 Pro iOS 26.2 Simulator에서 layout ready, 6 boxes, 2 runtime text 기록을 확인했다. |
| 30 | screenshot·simulator 관찰을 pixel parity나 실기기 성능으로 과장하는가 | 아니다. 화면은 visual smoke이고 software/emulator/simulator 범위를 벗어나지 않는다. |

PR에 포함되는 simulator screenshot은 [Android](./pass01/android/simulator.png), [iOS](./pass01/ios/simulator.png)다. 전체 실행 로그와 restoration hash는 [pass01](./pass01/environment.txt)에 있다.
