# C13.1 전체 물리 표면 RGBA 비교 계획 · 실패 관점 검토 20개 · 2026-10-18

코드 변경 전에 고정할 비교 계획을 독립적인 실패 경로 20개로 검토했다. 구현 검토와 PR 검토에서 이 관점을 재사용하지 않는다.

| # | 실패 관점 | 계획 판정 |
|---:|---|---|
| 1 | DPR1 Chrome PNG를 확대해 고DPI oracle로 오인 | 각 DPR로 별도 캡처하고 browser DPR와 PNG 크기를 함께 검사한다. 통과. |
| 2 | `788×263` Android view 크기를 CSS `300×100`과 혼동 | 물리 크기와 CSS 크기를 분리하고 모든 물리 좌표를 직접 대응시킨다. 통과. |
| 3 | Android 앱 전체 screenshot의 status bar/navigation bar가 PixelCopy에 포함 | surface PixelCopy만 comparator 입력으로 고정한다. 통과. |
| 4 | iOS 화면 screenshot을 Metal drawable 증거로 해석 | Metal texture readback을 요구하고 compositor screenshot을 거부한다. 통과. |
| 5 | 비교기에서 CSS pixel-center만 검사해 subpixel 누수 누락 | 전체 surface의 texel 수와 channel 수를 exact 고정한다. 통과. |
| 6 | iOS 1×1 texture copy의 256-byte padded row를 RGBA에 섞거나 다음 sample을 잘못 읽음 | 기존 64-point batch에서 각 padded row의 첫 네 byte만 내보내고 padding 혼입 mutation을 거부한다. 통과. |
| 7 | BGRA 값을 RGBA로 잘못 읽거나 두 번 swizzle | surface format과 RGBA export 경계를 manifest에서 확인한다. 통과. |
| 8 | sRGB 값을 선형화하거나 alpha를 재계산해 byte 변경 | 색상/alpha 변환을 금지하고 불일치는 원본 byte로 판정한다. 통과. |
| 9 | surface resize와 readback mapping이 경합해 다른 generation 혼합 | run마다 render key와 generation을 고정하고 도중 변경은 실패한다. 통과. |
| 10 | 이전 앱 process의 캡처가 새 run 결과로 재사용되거나 단일 실행의 우연한 일치 | 새 process ID, run ID, artifact hash를 결합하고 각 플랫폼에서 독립된 세 process/generation 결과를 요구한다. 통과. |
| 11 | Android AVD 두 serial 중 다른 에뮬레이터를 실수로 선택 | 계획에서 `emulator-5554` 하나만 허용하고 serial을 기록한다. 통과. |
| 12 | iOS 다른 기기/런타임의 drawable을 비교 | 지정 UDID와 iOS 26.2, drawable 크기·format을 함께 검증한다. 통과. |
| 13 | capture PNG dimensions가 예상과 달라도 comparator가 축소/늘림 | 스케일·리샘플링 없이 크기 불일치를 사전 실패시킨다. 통과. |
| 14 | surface outside 영역이 누락되거나 잘못된 crop에서 비교 | PixelCopy/Metal drawable 전체 바운드를 증거에 기록하고 모든 texel 수를 확인한다. 통과. |
| 15 | 한 channel 1-byte 차이가 평균/허용오차로 숨겨짐 | tolerance 없이 exact RGBA와 최대 오차 0만 통과시킨다. 통과. |
| 16 | 4,219개 iOS batch 경계에서 한 texel이 중복·누락·재정렬됨 | manifest byte length/cardinality와 index→row-major coordinate를 전부 검사하고 순서 mutation을 거부한다. 통과. |
| 17 | mismatch JSON이 콘솔 truncation 때문에 일부만 보존 | 전체 mismatch JSON을 artifact에 저장하고 콘솔은 요약만 출력한다. 통과. |
| 18 | PixelCopy/Metal readback 완료 전 성공 로그가 먼저 생성 | capture completion과 manifest 검증 후에만 성공 summary를 기록한다. 통과. |
| 19 | 새 FFI/JS/payload가 기본 앱 artifact에 섞임 | default negative와 opt-in positive artifact 검사를 별도로 둔다. 통과. |
| 20 | Simulator/AVD exact 결과를 제품, 물리 GPU 또는 WPT 완성으로 확대 | 명시적 제외와 STATUS 미완료 표기를 유지한다. 통과. |

계획 범위는 C13.1 고정 13-case fixture의 두 simulator surface다. 이 검토는 구현·실행 결과를 증명하지 않는다.
