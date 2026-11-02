# C11.1 fit-content default-route semantic gate 계획 정정 검토 20개

Projection source 뒤에 Root HTML runtime admission이 별도로 있음을 확인하고, 120×70 및 header/side/content 고정 성공 조건을 새 fit-content 경로에서 제거하되 기존 named-area 경로는 보존하도록 계획을 검토했다.

| # | 실패 관점 | 정정 및 방지 기준 |
| --- | --- | --- |
| 1 | typed projection 통과를 default runtime 지원으로 착각 | 별도 semantic gate source와 default-route fixture를 계획에 포함한다. |
| 2 | runtime gate의 현재 120×70px 고정을 누락 | 새 경로에 definite positive px width/height를 요구하고 특정 크기는 요구하지 않는다. |
| 3 | projection의 header/side/content 하드코딩을 runtime에 복제 | parsed area names 임의의 값을 허용하고 names allowlist를 금지한다. |
| 4 | runtime이 area 개수를 계속 세 개로 고정 | 각 parsed named area에 item 하나씩 연결하되 area count를 상수로 고정하지 않는다. |
| 5 | 같은 area에 여러 items가 중복 연결 | item placement name 사용 횟수가 area당 정확히 1인지 검증한다. |
| 6 | 이름 없는/missing area가 화면에 조용히 사라짐 | named area마다 정확히 한 visible in-flow direct item을 요구한다. |
| 7 | area dimensions가 explicit track보다 커 implicit track 발생 | parsed area rows/columns와 explicit track 수를 두 축 모두 비교한다. |
| 8 | percentage/calc, auto, fr, minmax를 fit path에 통과 | 새 경로의 모든 explicit tracks를 finite non-negative px/fit-content(px)로 제한한다. |
| 9 | overflow와 fit cap이 실제 차이를 보이지 않음 | 30px root 밖 40px item과 row root 밖 40px item을 유지하고 표본 위치를 고정한다. |
| 10 | area map 문자열을 직접 다시 파싱 | 기존 Stylo typed rectangular areas와 placement values만 소비한다. |
| 11 | nested or populated child가 단순 fixture를 통과 | whitespace-only direct text 외의 추가 child와 nested child를 계속 거부한다. |
| 12 | hidden, non-div, out-of-flow item이 accepted | visible empty HTML div direct item과 in-flow owner 조건을 명시한다. |
| 13 | root min/max 또는 gap 계산을 새로 허용 | min/max 기본값과 기존 gap validator를 유지한다. |
| 14 | DOM id/class, child order 또는 fixture ID를 성공 조건으로 사용 | arbitrary names 대조와 scene/ID selector 금지를 구현 기준에 둔다. |
| 15 | 기존 named-area 제품 경로를 넓히거나 바꿈 | fixed/fraction/percentage의 기존 120×70 header/side/content 경로를 별도 회귀한다. |
| 16 | Ahem test engine을 앱 기본 font로 오인 | Ahem intrinsic sizing과 system-font Root HTML runtime을 별도 근거로 둔다. |
| 17 | projection test를 Android/iOS runtime으로 보고 | default Root HTML, AVD PixelCopy, iOS Metal drawable을 각 단계로 둔다. |
| 18 | simulator surface sample을 화면 표시/compositor 완료로 과장 | AVD software renderer와 compositor 전 Metal sample 제한을 기록한다. |
| 19 | 두 번째 계획 누락으로 구현을 다시 시작 | semantic gate 계획 정정 및 별도 PR diff 20개 병합을 새 code-entry gate로 둔다. |
| 20 | 계획 정정으로 C11 완료 수가 오름 | spec/STATUS.md, C11/C11.1 checkbox와 공식 72/213을 유지한다. |

이번 정정은 projection 뒤의 Root HTML source boundary까지 계획 범위에 넣는다. 이 문서는 제품 구현이나 simulator 증거를 주장하지 않는다.
