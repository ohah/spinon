# C11.4 auto-fit named-line implicit fallback 구현/runtime 검토 20개

계획 검토와 PR diff 검토에서 분리해, 실제 fixture·adapter·시뮬레이터 결과의 서로 다른 실패 경계를 점검했다. 표의 판정은 성공 run [Android](./android-avd-c114-auto-fit-named-implicit-fallback-android-avd-20261022T225053720Z-747e4035/result.json) 및 [iOS](./ios-simulator-c114-auto-fit-named-implicit-fallback-ios-simulator-20261022T224801263Z-5b88c62c/result.json)에 근거한다.

| # | 실패 관점 | 관측 및 판정 |
| ---: | --- | --- |
| 1 | 신규 scene ID가 기존 ID와 충돌해 fixture가 덮어써지는가 | 28 scene, 28 unique ID assertion 통과. |
| 2 | auto-fill 회귀 10개가 auto-fit 추가 과정에서 빠지는가 | 두 플랫폼 결과에 기존 10 ID가 모두 포함되고 각 장면이 통과했다. |
| 3 | positive missing-name이 auto-fit의 선행 empty repeat를 collapse한 뒤 implicit line을 계산하는가 | item `(100,0) 20×10`, Columns `[0,0,0,20,20,20,20,20,20]` 일치. |
| 4 | positive insufficient occurrence가 authored `cell` 수를 넘어선 위치를 잘못 선택하는가 | `5 cell` item `(20,0)`, track list `[0,0,0,20,20]` 일치. |
| 5 | negative missing-name이 grid 시작 쪽 implicit line에서 방향을 뒤집는가 | `-5/-4 ghost` item `(0,0)`, trailing empty repeat만 collapse된 `[20,20,20,20,20,0,0,0]` 일치. |
| 6 | negative insufficient occurrence가 explicit `cell` occurrence와 trailing collapse를 혼동하는가 | `-5/-4 cell`의 `[20,0,0,0]` track list 및 item 시작 위치가 oracle과 일치. |
| 7 | forward named span이 명시 start line 기준 implicit line을 세지 않는가 | line `4`에서 `span ghost`까지 `[0,0,0,20]`, item 20px 확인. |
| 8 | backward named span이 반대 방향 implicit line을 세지 않는가 | `span ghost`에서 line `1`까지 `[20,0,0,0]`, item 20px 확인. |
| 9 | 한 repeated track을 점유한 item까지 empty로 잘못 collapse하는가 | occupied control 최종 Columns `[0,40,0]`, item width 40px 확인. |
| 10 | span item이 반복 track 하나만 점유한 것으로 처리되는가 | span control `[0,40,40]`, item width 80px 확인. |
| 11 | row-axis positive fallback이 column 전용 adapter에 막히거나 x 좌표에 적용되는가 | row positive scene item y=100, Rows `[0,0,0,20,20,20,20,20,20]` 확인. |
| 12 | row-axis negative fallback이 trailing collapse 방향을 column과 다르게 계산하는가 | row negative scene item y=0, Rows `[20,20,20,20,20,0,0,0]` 확인. |
| 13 | implicit track 크기나 gap이 case별로 달라 기대 placement를 우연히 만드는가 | 20개 scene 모두 20px implicit size, gap 0으로 고정됐고 JSON·runner 기대값이 일치한다. |
| 14 | 0px collapsed track과 그 인접 gutter를 잘못 색 sample 대상으로 삼는가 | 표본은 root 내부와 item 시작/끝에 한정되고 collapsed track interior를 읽지 않는다. PixelCopy/Metal 각 60 sample exact. |
| 15 | fixture node ID와 화면 sample의 owner가 섞이는가 | sample마다 `c114-<scene>-grid` 또는 `...-item` owner를 사용하고 Android/iOS verifier가 각 sample exact를 대조했다. |
| 16 | Android root-frame metadata가 row auto-fit을 column 크기로 보고하는가 | row fixtures는 `20×125`, column fixtures는 `125×20`으로 분리됐다. 20개 결과에서 모두 일치. |
| 17 | Android 신규 scene이 runtime allowlist에는 있으나 pixel-matrix eligibility/expected-color lookup에 빠지는가 | pixel matrix allowlist와 `auto-fit-named-*` RGB prefix를 연결했다. 최초 `-1` 기대값 실패를 고친 뒤 60/60 exact. |
| 18 | iOS 신규 scene이 track allowlist와 고정 Metal sample selector에서 불일치하는가 | 두 목록에 10개 ID를 연결했다. 최초 `invalid-run-scene-or-samples` 경계를 고친 뒤 20/20 scene과 60/60 exact. |
| 19 | capture runner의 fixture 수 또는 requested scene parsing이 예전 matrix에 고정되는가 | expected fixture count 28, 전체 C11.4 count 307, 20개 requested IDs가 양 플랫폼에서 동일하다. |
| 20 | simulator 결과가 최신 앱·source 또는 더 넓은 CSS/hardware 범위를 증명한다고 과장되는가 | APK/app hash와 source diff hash를 저장했다. artifact boundary는 host staticlib에 한정하며 README에서 실기기, hardware GPU, WPT, border 및 iOS compositor 이후 pixels를 제외했다. |

결함은 #17과 #18에서 확인해 각각 수정한 뒤 최신 source로 전체 matrix를 다시 실행했다. 별도 PR diff 검토는 [30개 관점 기록](./pr-diff-review-30.md)에 둔다.
