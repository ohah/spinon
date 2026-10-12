# C11.1 iOS Metal surface 표본 구현 검토 · 2026-10-12

**범위:** C11 고정 Grid 표면 표본 7개를 Chrome 154 PNG와 비교하는 내부 test hook, Rust batch readback, iOS Simulator 실행기와 증거 수집기

**실행 근거:** [Metal 표면 실행 결과](./c11-1-ios-metal-surface-pixels-2026-10-12/run-c11-ios-metal-20261011T205937099Z-a996f7ef/README.md) · [비교 JSON](./c11-1-ios-metal-surface-pixels-2026-10-12/run-c11-ios-metal-20261011T205937099Z-a996f7ef/comparison.json) · [20개 계획 검토](./c11-1-ios-metal-surface-pixels-plan-review-2026-10-12.md)

## 실패 관점 20개

| # | 공격 관점 | 확인·대응 |
| ---: | --- | --- |
| 1 | Pixel expectation이 Android 표본을 복사해 두 플랫폼 공통 오류를 숨기는가 | sample RGBA는 고정 Chrome 154 DPR 1 PNG에서 직접 decode하고 PNG·reference JSON SHA-256을 fixture에 고정했다. 비교기는 각 기대 색을 원본 PNG에서 다시 추출해 검사한다. |
| 2 | Chrome 360×800과 Spinon 320×240 차이를 같은 전역 좌표라고 가정하는가 | fixture에 두 viewport를 별도로 기록한다. C11 scene-local (0,0) 기준의 공통 영역만 비교하며 screenshot 전체를 겹쳐 비교하지 않는다. |
| 3 | Simulator screenshot이나 UIKit view를 Metal 표면 pixel로 오인하는가 | sample은 WGPU `CAMetalLayer` drawable texture에서 읽는다. Simulator screenshot은 보조 이미지로만 저장하고 결과가 compositor 전 pixel임을 기록한다. |
| 4 | 화면 전역 `UIScreen.scale`을 drawable scale로 간주하는가 | 요청 좌표는 실제 drawable 크기/320×240 runtime viewport로 계산한다. 반환된 `surface_texture`가 layout 시점 drawable과 같은지도 다시 확인한다. |
| 5 | 비정방 drawable이나 잘못된 scale을 숨기는가 | Swift와 Node 비교기가 가로·세로 scale 동일성을 확인한다. 960×600, x/y 배율 변경 negative control을 거부했다. |
| 6 | 좌표를 픽셀 모서리에 찍어 rounding 차이가 색 경계를 넘는가 | 고정 CSS 표본 중심을 `floor((cssPx + 0.5) × scale)`로 바꾸고 Node comparator가 실제 device coordinate를 다시 계산한다. off-by-one 변형은 실패했다. |
| 7 | 표본 점이 drawable 밖인데 API가 copy를 제출하는가 | Rust renderer에서 0-based width/height 경계를 검사한다. 빈 점, x==width, y==height를 단위 경계에서 거부한다. |
| 8 | drawable 재생성 중 이전 surface generation의 sample을 새 실행에 붙이는가 | render lane은 current generation이 아니면 요청하지 않는다. begin/sample/summary에 surface generation을 묶고 다른 generation 표본을 comparator가 거부한다. |
| 9 | CSS layout revision과 실제 draw snapshot revision이 달라 stale pixel을 통과시키는가 | draw report에 document/render-tree/style/environment revision과 snapshot generation을 추가했다. begin과 summary revision이 다르면 비교 실패다. 초기 environment revision 0은 유효 입력으로 허용한다. |
| 10 | 이전 프로세스나 stale unified log가 새 실행의 sample로 섞이는가 | run ID, 앱 PID, 단일 begin/summary marker를 요구한다. 다른 PID, 다른 run, 중복 marker, 누락 marker negative control을 거부했다. |
| 11 | 하나의 표본이 빠지거나 같은 ID가 두 번 나와도 7개로 합산하는가 | 고정된 7개 ID 집합을 정확히 비교한다. missing, duplicate, unknown, extra sample 변형이 실패한다. |
| 12 | Swift 반환 배열 순서가 바뀌었는데 ID와 색이 뒤섞이는가 | 반환 JSON의 각 index 좌표·RGBA를 fixture의 같은 sample 위치와 대조한 뒤 식별 로그를 남긴다. 순서가 바뀌면 좌표 또는 색 비교에서 실패한다. |
| 13 | alpha를 무시하거나 색상 차이를 평균으로 숨기는가 | RGBA 네 채널을 sample별 exact 비교하며 최대 허용 채널 오차는 0이다. 한 채널 차이 변형은 실패했다. |
| 14 | BGRA surface의 channel order나 sRGB 변환이 결과를 바꾸는가 | RGBA/BGRA 8-bit 포맷만 허용하고 BGRA를 RGBA 순서로 decode한다. 단위 시험에서 채널과 alpha를 확인하고 16-bit·depth 포맷은 거부한다. |
| 15 | 1×1 copy의 row alignment/offset이 여러 sample에서 겹치는가 | 각 점은 256-byte aligned row offset에 복사하고 buffer 크기는 표본 수와 alignment의 checked multiplication이다. 7개 Metal 결과가 색·좌표별로 맞았다. |
| 16 | FFI의 pointer/count 입력이 null, 0, 상한 초과 또는 overflow일 때 잘못 읽는가 | `count×2` overflow, 0·65개, null coordinates와 null host negative test를 추가했다. 허용 범위는 1–64다. |
| 17 | 동시에 두 readback을 걸어 이전 callback을 덮는가 | renderer는 pending sample batch가 있으면 다음 요청을 거부한다. draw가 제출한 batch는 mapping 완료 뒤 가져가며 이전 C12 single-sample 경로도 재검증했다. |
| 18 | map timeout, map callback 오류, 미지원 포맷 또는 JSON 실패가 빈 성공 표본이 되는가 | Rust 경로는 각 오류를 실패 report로 반환하고 success JSON은 실제 mapped 7개 결과에서만 생성한다. 실패 collector는 원본 build/log와 단계 이름을 evidence directory에 남긴다. |
| 19 | 진단 entry가 기본 iOS 앱이나 일반 public API에 포함되는가 | 별도 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST` Cargo/Xcode gate를 추가했다. 일반 C11 matrix 빌드 log에서 flag 0 및 Cargo feature 미포함을 확인했다. |
| 20 | 공통 batch 변경이 Android C11/C12 또는 iOS C12 결과를 깨뜨리는가 | Android 실기기 C11 10/10·40/40 및 fixed PixelCopy 7/7, C12.4 26-scene runtime과 PixelCopy 21/21·22/22를 재실행했다. iOS C11 10/10·40/40와 C12 Metal reverse 표본 8/8도 통과했다. |

## 결과와 경계

- iPhone 17 Pro / iOS 26.2 Simulator에서 drawable 960×720, CSS draw viewport 320×240, scale 3×3으로 실행했다. Chrome oracle viewport는 360×800 DPR 1이다.
- 고정 `fixed-tracks-gap` scene의 7 sample 모두 실제 WGPU Metal drawable byte와 Chrome PNG의 RGBA가 exact match했다.
- 이는 한 scene의 drawable texture가 compositor에 제출되기 전 표본이다. 최종 화면 색상 관리, iPhone 실기기, 다른 9개 C11 scene, WPT 적합성 또는 전체 CSS 지원을 증명하지 않는다.
- Android 실기기·iOS Simulator의 C11 geometry 10 scene, C11 Android pixel canary, C12.4 Android/iOS pixel 회귀 근거는 각 플랫폼·fixture 범위에서만 해석한다.
