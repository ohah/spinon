# C11.2.3 모바일 표면 픽셀 구현 검토

**일자:** 2026-10-13 · **범위:** Android PixelCopy 좌표 환산, Android/iOS 표면 비교기, 고정 scene 실행기와 결과 기록

## 실행 근거

| 플랫폼 | run ID | 실행 결과 | 앱 SHA-256 |
| --- | --- | --- | --- |
| Android 16/API 36 · SM-S731N 실기기 | `c1123-a-mv5yld3e-4ded3cc6` | 25 scene, 107 node, 성공 19·기대 거부 6, geometry 오차 0 CSS px, 99/99 표본 exact | `75106de0c921b3916027e5337e6461a037112d974300edbae8dfee39c550346f` |
| iPhone 17 Pro · iOS 26.2 Simulator | `c1123-i-mv5ymb0x-10eed1fb` | 25 scene, 107 node, 성공 19·기대 거부 6, geometry 오차 0 CSS px, 99/99 표본 exact | `c17932a03edda9a58389c7e526f051785b0895a99d17b0adde7ebfe4883222fc` |

공통 pixel matrix SHA-256은 `7e57c8fedf662882242307f998298df2355c6a63e7eb537f15f3a59a5b0fb489`이다. Android는 PixelCopy bitmap 900×675 px를 runtime CSS viewport 320×240 px로 환산했다. iOS 비교는 Metal drawable의 compositor 이전 readback이다. 이전 density 기반 Android run은 이 수정의 증거로 사용하지 않는다.

## 실패 관점별 검토

| # | 공격 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 바뀐 HTML·inventory·Chromium 기준을 현재 실행 결과에 섞기 | pixel matrix generator가 HTML·inventory·runtime matrix·Chromium reference hash를 다시 고정한다. 결정성 및 hash 변경 거부 테스트가 통과했다. |
| 2 | 성공·거부 scene 집합을 바꾸어 표본 누락을 숨기기 | 고정 25 scene 중 성공 19·거부 6을 Android와 iOS 실행기 모두에서 확인했다. |
| 3 | Chrome viewport와 앱 runtime viewport를 혼용하기 | Android begin/capture marker의 viewport를 320×240으로 기록하고 comparator가 fixture viewport와 일치하는지 검사한다. 잘못된 viewport는 negative test에서 거부된다. |
| 4 | 실제 PixelCopy 크기 대신 화면 크기나 예상 크기를 쓰기 | PixelCopy PNG의 실제 900×675 크기, begin/capture 기록, decoded PNG 크기를 서로 대조한다. |
| 5 | x 좌표를 Android density로 환산하기 | 계산은 `floor((cssX + 0.5) × bitmapWidth / viewportWidth)`로 바꿨다. CSS x=20, bitmap 900, viewport 320의 결과 57을 단위 검증했다. |
| 6 | y 좌표에 x scale 또는 density를 재사용하기 | y도 실제 bitmap 높이/viewport 높이로 별도 계산한다. CSS y=10, bitmap 675, viewport 240의 결과 29를 검증했다. |
| 7 | 서로 다른 x/y 표면 비율을 하나로 뭉개기 | 별도 축 계산 테스트에서 5/4와 7/4 scale이 각각 다른 표본 좌표를 만든다. |
| 8 | 잘못된 density 로그가 비교 결과를 바꾸지 않는지 확인 | Android comparator positive fixture는 density=1, 실제 표면 scale=2.8125로 실행되며 통과한다. density는 진단 값으로 남긴다. |
| 9 | 이전 density 환산 좌표가 우연히 통과하기 | 이전 공식을 사용한 좌표를 넣으면 comparator가 거부한다. |
| 10 | 픽셀 경계 좌표와 픽셀 중심 좌표를 혼동하기 | CSS sample center에 `+0.5` 후 `floor`하는 규칙을 Android·iOS 비교기에서 고정하고 표본 좌표 negative test를 둔다. |
| 11 | 원본 CSS sample 좌표나 sample ID를 실행 로그에서 바꾸기 | scene별 CSS 좌표와 고정 matrix를 1:1 비교한다. off-by-one CSS 좌표와 unknown ID는 거부된다. |
| 12 | 다른 run/process의 로그를 현재 실행에 섞기 | run ID로 marker를 분리하고 begin PID와 각 sample PID를 대조한다. 다른 run interleave는 별도 테스트했다. |
| 13 | 이전 Surface generation의 PixelCopy 결과를 사용하기 | begin/capture/sample의 generation을 대조하고 요청 전·PixelCopy 완료 후 source generation과 renderer·크기를 다시 확인한다. 실기기 행렬의 scene별 generation도 기록했다. generation 경합을 강제 주입한 시험은 이번 범위에 포함하지 않았다. |
| 14 | 재시도 중복·무제한 PixelCopy 요청 | 수집기는 최대 3회 재시도하고 비교기는 attempts 상한을 확인한다. 현재 양 플랫폼 19 scene은 모두 한 번의 표면 수집으로 끝났다. PixelCopy 실패 코드를 강제한 실기기 주입 시험은 하지 않았다. |
| 15 | 누락·중복 begin/capture/summary를 성공으로 처리하기 | Android comparator의 marker 개수와 상태 검사가 missing·duplicate·error 상태 테스트를 거부한다. |
| 16 | 다른 경로의 이미지나 손상 PNG를 정상 표면으로 취급하기 | 앱 cache 경로·run 파일명·PNG decode·bitmap 크기·SHA-256을 묶어 검증한다. 손상 bytes와 hash 변조가 거부된다. |
| 17 | 빠지거나 중복된 표본을 평균 통과로 숨기기 | 기대 sample ID 집합·개수와 실제 집합을 정확히 대조한다. 누락·중복·unknown sample negative test가 통과했다. |
| 18 | 일부 RGBA 채널 또는 일부 scene만 비교하기 | 두 플랫폼 모두 19 scene·99 sample의 네 채널을 pinned Chrome 값과 exact 비교했다. 최대 채널 오차는 0이다. |
| 19 | 예상 거부 scene에 stale/부분 표면을 붙이기 | 6개 거부 scene은 Android/iOS 실행기에서 pixel begin marker가 없음을 검사한다. frame·양수 draw 부재도 geometry 실행 로그에서 확인했다. |
| 20 | 플랫폼 경계나 검증 결과를 과장하기 | Android 근거는 SM-S731N 실기기 PixelCopy, iOS 근거는 iOS 26.2 Simulator Metal drawable이다. iOS 실기기, OS compositor 이후 화면 픽셀, 전체 WPT 적합성, 전체 CSS Grid 지원의 근거로 승격하지 않는다. |

## 별도로 고친 비교기 취약점

- Android 표본 좌표가 계획의 실제 surface/viewport 비율 대신 device density를 사용했다. 이번 검토에서 발견해 `requestForCssViewport` 경로와 비교기 검증을 추가했다. 예전 Android pixel 결과는 새 run으로 대체한다.
- iOS comparator의 PID·surface generation·document/render revision은 `Number(...) > 0`만 확인해 소수 또는 안전 정수 범위 밖 값을 허용할 수 있었다. 정수 문자열과 JavaScript safe integer 범위를 확인하도록 고치고 fractional/unsafe counter negative test를 추가했다.

## 판정

수정된 Android 좌표 계약, C11.1·C11.2.1·C11.2.2·C11.2.3 공용 PixelCopy comparator, iOS Metal comparator에 대해 위 실패 경로를 확인했다. Android PixelCopy/iOS Metal 표면 gate는 고정 C11.2.3 profile에서 통과했다. 전체 Grid/CSS 지원, Spinon WPT 적합성, iOS 실기기, compositor 이후 pixel parity, PixelCopy 오류 주입은 미검증으로 남긴다.
