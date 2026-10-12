# C11.1 iOS Metal surface pixel 계획 실패 관점 검토 · 2026-10-12

**대상:** [iOS Metal 표면 표본 계획](../../../plan/c11-1-ios-metal-surface-pixels.md)

| # | 독립 실패 관점 | 검토 결과와 계획 반영 |
| ---: | --- | --- |
| 1 | C11 Grid가 아닌 C12.4 전용 색 장면을 읽는가 | 기존 C12.4 readback fixture는 C11 장면이 아니므로 새 C11 runtime host와 별도 C11 FFI 진입점을 요구한다. |
| 2 | Android sample 표만 기대값 원본으로 사용해 두 플랫폼이 같은 실수를 공유하는가 | 기대값은 pinned Chrome DPR 1 screenshot의 path/hash와 함께 고정 JSON으로 관리하고 Android 값은 교차 확인만 하도록 범위를 정했다. |
| 3 | 고정 Chrome PNG가 교체되거나 fixture가 달라져도 과거 expected color가 통과하는가 | reference ID와 PNG SHA-256, C11 fixture hash를 결과에 묶고 drift 때 새로운 입력 버전을 만들도록 했다. |
| 4 | CSS 좌표를 정수 device pixel로 잘못 반올림하는가 | CSS pixel 중심을 `(x+0.5)×scale`로 변환하고 floor 결과를 매 표본 로그·비교 대상에 포함한다. |
| 5 | UIKit safe-area나 canvas inset을 표본 좌표에 임의로 더하는가 | 좌표 원점을 WGPU C11 surface 자체로 고정하고 UIKit view 좌표와 분리하도록 명시했다. |
| 6 | Simulator screenshot만으로 Metal texture 값을 주장하는가 | screenshot은 보조 이미지로만 허용하고 WGPU drawable texture의 직접 `COPY_SRC` 결과만 픽셀 근거로 인정한다. |
| 7 | 표면 format이 BGRA인데 RGBA로 읽거나 byte 순서를 바꾸는가 | RGBA/BGRA 8-bit format 처리와 실제 format 기록을 구현·검증 경계에 포함했다. |
| 8 | sRGB 값을 다시 gamma 변환해 exact comparison이 왜곡되는가 | texture의 byte decode를 색 공간 계산과 분리하고 raw RGBA bytes를 Chrome 표본과 비교하도록 했다. |
| 9 | alpha가 반투명인데 RGB 3채널만 비교해 결과를 통과시키는가 | 4개 채널 전체 exact 비교를 요구한다. |
| 10 | 표본이 box 경계·anti-alias 경계에 있어 미세한 raster 차이를 색상 차이로 오인하는가 | canonical fixture 검토에서 각 점이 단색 box 또는 gap 내부에 있는지 pinned Chrome screenshot으로 확인하고 경계점은 허용하지 않는다. |
| 11 | 표본 하나가 빠져도 남은 여섯 개가 aggregate 성공을 만드는가 | 고유 7개 ID 집합·순서·좌표와 실제 결과 수를 모두 비교한다. |
| 12 | 이전 Simulator 프로세스의 로그나 Pixel 값을 새 run에 결합하는가 | 매 실행 고유 run ID, 새 프로세스 PID, 현재 실행 시작 뒤의 predicate-filtered log를 요구한다. |
| 13 | resize/recreate 중 이전 `CAMetalLayer` generation의 결과를 현재 장면으로 채택하는가 | 표면 generation·크기·renderer generation을 읽기 전후 확인하고 불일치하면 폐기하도록 했다. |
| 14 | 같은 장면처럼 보여도 다른 document/style/environment revision을 캡처하는가 | C11 eval·draw의 revision 및 scene 성공 marker가 readback run과 함께 있어야 한다. |
| 15 | GPU copy가 render submit과 다른 표면·시점에서 수행되는가 | 읽기는 draw의 surface texture를 같은 encoder submission에 복사한 결과로 한정한다. 이 값과 최종 compositor/monitor pixel의 차이도 계획에 명시했다. |
| 16 | GPU mapping timeout, unsupported format, stale draw가 빈 결과로 성공 처리되는가 | 각 오류는 명시적 실패가 되어야 하며 표본이 없으면 7/7 summary를 생성하지 않는 조건을 넣었다. |
| 17 | C ABI count가 포인터 크기와 곱해져 overflow하거나 null 포인터를 역참조하는가 | FFI에서 pointer/count, `count×2`, 1–64 범위를 먼저 검사하도록 계획을 구체화했다. |
| 18 | batch 수를 크게 전달해 임시 staging buffer를 무제한 할당하는가 | 64개 상한을 두고 0·64·65 경계와 drawable 밖 좌표를 negative control로 지정했다. |
| 19 | C11 전용 변경이 기존 C12.4 단일 표본 경로나 기본 앱 빌드를 깨뜨리는가 | 공통 internal test-only copy mechanism만 공유하고 C11/C12 fixture entry는 분리하며 기본 flag-off와 기존 C12 단일 표본 회귀를 완료 조건으로 둔다. |
| 20 | 표본 통과를 전체 C11 Grid, WPT, iOS 기기 또는 최종 화면 동일성으로 과장하는가 | 범위를 고정 scene 1개·표본 7개·Simulator surface texture로 제한하고 남은 pixel/WPT/기기 경계를 분리 기록한다. |

## 계획 수정 사항

- 직접 drawable readback은 compositor 이후 physical display의 최종 pixel과 구분했다.
- sample point가 단색 면·gap 내부인지 확인하고 PNG는 reference 입력이자 보조 화면 캡처로 역할을 분리했다.
- FFI pointer/count 안전성, copy batch 상한, 기존 C12.4 단일 표본 회귀를 완료 관문에 명시했다.

## 구현 뒤 수정 경계 재확인

Chrome 360×800 oracle과 Spinon 320×240 draw viewport를 하나로 합치지 않고, sample 좌표를 scene-local 원점과 실제 drawable/viewport 비율로 계산하도록 구현했다. 표본 위치·색·hash는 고정 PNG에서 다시 검사했다. 구현 코드를 대상으로 한 별도 점검은 [구현 실패 관점 20개](./c11-1-ios-metal-surface-pixels-implementation-review-2026-10-12.md)에 기록했다.

이 계획 문서는 계약이다. 구현 완료는 별도 실행 evidence와 구현 검토로 판정한다.
