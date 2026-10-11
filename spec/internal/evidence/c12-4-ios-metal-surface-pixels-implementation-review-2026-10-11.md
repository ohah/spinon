# C12.4 iOS Metal 표면 pixel 구현 실패 관점 검토

**대상:** [계획](../../../plan/c12-4-ios-metal-surface-pixels.md), iOS 내부 표면 복사 경로와 [실행 evidence](./c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/)  
**판정:** 확인한 결함을 수정해 재실행했다. C12.4 완료나 WPT 적합성을 주장하지 않는다.

| # | 독립 검토 관점 | 확인 결과와 조치 |
| --- | --- | --- |
| 1 | offscreen 결과를 실제 표면 값처럼 취급 | 비교기는 `renderer=metal-surface`만 허용하고 WGPU가 surface texture를 `COPY_SRC`로 복사한다. |
| 2 | test-only surface usage가 제품 기본 설정을 변경 | 전용 feature/env flag로만 surface readback 경로를 사용한다. |
| 3 | 다른 CSS profile의 host를 pixel oracle에 사용 | C12.4 stacking fixture host를 만들고 실제 V8·Stylo·Taffy·paint 경로를 실행한다. |
| 4 | 앱 초기 스타일 변경이 V8 밖에서 이뤄짐 | 앱의 reverse fixture를 V8에서 변경한 뒤 새 render를 요청한다. |
| 5 | 표본 좌표가 기대 box 밖에 위치 | 첫 run의 row-reverse에서 배경색이 나와 실패로 기록했다. microfixture 크기와 좌표를 대조했다. |
| 6 | fixture parent geometry가 Rust/WGPU 비교 fixture와 다름 | parent를 100×100 CSS px로 맞춰 겹침 좌표를 보장하고 새 8-process run을 했다. |
| 7 | CSS→physical pixel rounding이 달라짐 | 비교기가 `(css + 0.5) × scale`의 floor 좌표와 drawable 크기를 case마다 확인한다. |
| 8 | Retina scale을 1로 가정 | 실제 Simulator scale=3과 960×720 drawable을 기록하고 좌표 211×466/526을 비교한다. |
| 9 | row padding으로 채널 오독 | WGPU copy row pitch를 정렬하고 mapped buffer에서 pixel 위치를 계산한다. |
| 10 | BGRA를 RGBA로 잘못 읽음 | 지원 포맷을 제한하고 BGRA/RGBA 디코딩 단위 테스트가 있다. |
| 11 | sRGB/지원하지 않는 포맷을 무조건 해석 | 지원 목록 밖 형식은 성공 sample로 대체하지 않고 실패한다. |
| 12 | map callback이 영원히 기다림 | render worker에서 최대 5초 제한을 둔다. 이번 정상 run에서 timeout은 없었다. |
| 13 | UIKit main thread가 GPU mapping을 기다림 | readback은 전용 render queue 경로로만 요청한다. |
| 14 | stale surface generation sample이 새 frame으로 섞임 | surface presentation sequence가 바뀌면 pending sample을 버린다. 실제 경합 fault injection은 이번에 하지 않았다. |
| 15 | 이전 app/log 실행의 결과가 포함 | case별 새 app process와 고유 run ID를 쓰고 통합 비교가 정확히 8개 집합을 요구한다. |
| 16 | 실패 run을 성공 결과에서 숨김 | 잘못된 첫 run을 보존하고 수정 후 run을 별도 디렉터리에 기록했다. |
| 17 | batch/run metadata가 원본과 불일치 | 성공 결과 JSON을 원본 surface log와 case별 run ID로 재생성했다. |
| 18 | pinned oracle 버전이 비어 있거나 다른 값 | Chromium 154.0.8037.98과 reference/fixture SHA-256을 함께 기록했다. |
| 19 | 비교기 실패 입력을 검증하지 않음 | 8개 Node negative test에서 missing/duplicate/stale run/color/coordinate/offscreen/error를 거부했다. |
| 20 | 일부 pixel을 전체 CSS/WPT 적합성으로 확대 | evidence·계획·상태에 8개 Simulator 표본이며 WPT/전체 C12.4가 아니라고 제한했다. |

실행 중 고친 것은 sample parent geometry와 결과 metadata였다. iOS hardware GPU, runtime capability/map failure injection 및 전체 21-scene iOS matrix는 이 검토 범위 밖에 남긴다.

