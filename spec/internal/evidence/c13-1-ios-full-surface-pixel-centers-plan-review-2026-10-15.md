# C13.1 iOS 전체 surface pixel-center 계획 검토 · 2026-10-15

계획: [iOS 전체 CSS pixel-center 표면 대조](../../../plan/c13-1-ios-full-surface-pixel-centers.md)

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획 반영 / 판정 |
|---:|---|---|
| 1 | UIKit 전체 화면 screenshot이 Metal drawable의 direct pixel 결과로 오인된다. | screenshot은 보조 자료로만 허용하고 WGPU drawable readback만 비교 입력으로 고정했다. |
| 2 | readback이 compositor 제출 뒤 색 변환/scanout을 증명한다고 과장된다. | 결과 경계를 pre-compositor drawable sample로 한정하고 compositor·display는 미검증으로 썼다. |
| 3 | iOS 화면 density를 drawable scale이라고 가정해 x/y 좌표가 밀린다. | 매 run의 actual texture width/height로 별도 scale을 계산하며 density 사용을 금지했다. |
| 4 | 300×100 viewport 크기와 drawable 크기가 바뀌는데 이전 좌표를 재사용한다. | batch 전후 viewport, generation, texture dimensions/format 불일치 시 run 전체를 실패시킨다. |
| 5 | row padding 또는 drawable RGBA/BGRA format이 잘못 해석된다. | actual format 기록과 readback API의 RGBA 정규화 후 exact 비교를 요구했다. |
| 6 | 30,000 samples 한 번에 요청해 버퍼·JSON·stack 용량을 넘긴다. | 현재 64점 bound를 유지하고 최대 469개의 고정 크기 batch로 나눴다. |
| 7 | batch 요청이 아직 pending인데 다음 batch가 덮어쓴다. | 같은 host readback 직렬화와 batch index/response 대응을 성공 조건에 넣었다. |
| 8 | batch 사이에 hit query나 재렌더가 scene generation을 바꾼다. | pixel result를 같은 runtime generation과 run ID에 귀속하고 혼합 결과를 거부한다. |
| 9 | GPU map wait가 UIKit main thread를 멈춰 화면 상태가 거짓으로 보인다. | 실제 executor를 확인하고 main-thread blocking을 금지하며 batch progress deadline을 둔다. |
| 10 | 한 batch timeout 뒤 부분 artifact가 complete로 남는다. | `.part` 임시 파일과 complete manifest의 atomic rename을 요구한다. |
| 11 | 이전 앱 실행의 artifact가 새 실행 결과로 채택된다. | fresh process마다 UUID run ID와 app executable/source revision/hash를 묶는다. |
| 12 | sample JSON 하나가 unified log 길이 제한에서 잘려 조용히 통과한다. | 샘플은 app-container JSONL 파일에 저장하고 logs에는 요약만 남긴다. |
| 13 | JSONL sample 누락·중복·재정렬이 단순 count만으로 가려진다. | index와 row-major CSS 좌표를 전수 검증하고 각 좌표의 unique 조건을 둔다. |
| 14 | 서로 다른 CSS center가 같은 drawable pixel로 매핑되어 비교 범위가 줄어든다. | physical 좌표 uniqueness와 scale `>=1`을 명시적으로 검증한다. |
| 15 | fixture element 바깥/clip 경계 픽셀이 full-grid라는 말 아래 실제로 빠진다. | 300×100 모든 CSS center를 요구하고 border 바깥 지점의 각 채널 mutation을 실패시킨다. |
| 16 | viewport 밖 좌표까지 확인했다고 오해한다. | viewport 안의 frame/border 바깥과 화면 viewport 바깥을 문서에서 분리했다. |
| 17 | renderer 계산 color나 Android PNG가 iOS observed pixels를 대신한다. | Metal texture에서 읽은 RGBA sample만 실제 값으로 받고 Android는 회귀 비교로만 쓴다. |
| 18 | C13 fixture readback을 위해 C11/C12 fixture symbol과 payload가 제품 artifact에 들어간다. | C13 전용 feature gate/API와 default/positive-control artifact 검사를 요구했다. |
| 19 | Android 공통 FFI/readback 회귀가 생겨 iOS 결과만 맞는다. | 같은 변경 후 Android API 37 AVD full-grid PixelCopy 30,000/30,000 재실행을 둔다. |
| 20 | 성공 pixel capture가 clip hit rejection, event dispatch, WPT나 C13 완료로 확대된다. | paint와 hit/event를 분리하고 scroll/CSSOM/WPT/제품 지원을 완료 금지선에 적었다. |

## 검토 결과

초기 위험에서 확인한 64점 backend bound, framebuffer-vs-screenshot 경계, per-run drawable mapping, thread ownership, partial file admission, stale generation, log truncation 및 default artifact isolation을 계획에 반영했다. 미해결 P1/P2는 없다. 계획 검토는 구현 검토나 PR 변경 검토를 대신하지 않는다.
