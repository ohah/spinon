# C11.2.1 모바일 표면 픽셀 행렬 계획 · 실패 관점 검토

검토 대상은 [C11.2.1 모바일 표면 픽셀 행렬 계획](../../../plan/c11-2-1-mobile-surface-pixels.md)이다. 체크리스트 수를 채우는 대신, 입력이 잘못되거나 실행이 일부만 성공했을 때 완료로 오인될 지점을 각각 확인했다.

| # | 공격 관점 | 검토 결과와 계획 반영 |
| ---: | --- | --- |
| 1 | 기존 numeric-lines 샘플 좌표·기준을 전체 runtime 행렬에 그대로 재사용하면 다른 장면을 잘못 확인하지 않는가? | scene별 pinned Chrome screenshot과 sample을 새로 생성하고 scene ID/hash 연결을 검사한다. |
| 2 | Chrome 캡처가 360×800이고 GPU viewport는 320×240인데 좌표가 runtime 바깥을 가리킬 수 있는가? | fixture 생성 단계에서 두 이미지와 viewport 경계를 모두 검사한다. |
| 3 | Spinon이 지원하지 않는 네 장면을 Chrome과 pixel parity가 맞아야 하는 입력으로 세지 않는가? | 성공 후보 6개만 픽셀 대조하고 4개는 기대 거부·부분 결과 부재·non-empty present 부재로 별도 판정한다. |
| 4 | 10개 중 일부 scene만 빠져도 성공 summary를 쓸 수 있는가? | 성공 후보 6개 및 reject 4개 전부의 결과 개수와 identity를 요구한다. |
| 5 | 중첩 Grid에서 parent/child node의 중심이 같은 색 영역을 가리켜 잘못 통과할 수 있는가? | reference screenshot의 실제 visible color 영역에서 표본을 뽑고 node 순서·geometry를 대조한다. |
| 6 | overlap 장면에서 아래 item 중심의 색이 위 item에 가려졌는데 아래 item 색으로 비교하지 않는가? | 각 node의 visible paint color가 있는 3×3 영역을 찾고, 가려진 item은 다른 노출 영역을 사용한다. 노출 영역이 없으면 fixture 생성 실패다. |
| 7 | sample 좌표 선택 결과가 순회 순서·플랫폼에 따라 달라져 fixture diff가 반복되는가? | 중심에서 가까운 유효 pixel을 고르고 동률은 y, x 순으로 정렬한다. |
| 8 | anti-alias 경계나 gap edge가 표본에 포함될 수 있는가? | screenshot에서 sample 주위 3×3 RGBA가 같은 점만 채택한다. |
| 9 | 투명한 구조 node, 0 크기 node 또는 hidden node를 실제 paint로 오인하는가? | 성공 candidate scene의 computed geometry 및 HTML paint class를 확인하고, paint가 없는 node에는 표본을 요구하지 않는다. |
| 10 | screenshot, Chrome JSON, HTML 또는 sample fixture가 바뀌어도 오래된 좌표를 수집할 수 있는가? | 원본 bytes의 SHA-256을 고정하고 matrix 입력 hash 불일치 시 앱을 실행하지 않는다. |
| 11 | Chrome screenshot이 다른 scene의 파일인데 ID만 맞춰 통과하는가? | reference screenshot 목록의 case ID·경로·해시를 같은 scene에 연결한다. |
| 12 | Android PixelCopy 결과 대신 화면 전체 screenshot을 PixelCopy 근거로 부르는가? | PixelCopy PNG 자체의 해시와 표본을 대조하고 화면 캡처는 보조 자료로 분리한다. |
| 13 | iOS Simulator screenshot을 Metal drawable readback으로 잘못 해석하는가? | sample RGBA는 WGPU draw report의 drawable texture readback만 사용하고 screenshot은 시각 확인용으로 제한한다. |
| 14 | Android density 또는 iOS drawable scale을 잘못 적용해 우연히 같은 픽셀을 읽는가? | 실제 surface 크기 ÷ 320×240 CSS viewport 비율로 좌표를 계산하고 drawable scale 일관성을 확인한다. |
| 15 | 표면이 재생성된 뒤 이전 generation의 capture를 현재 scene 결과로 채택하는가? | capture 전후 surface generation·renderer handle·size 검증이 불일치하면 실패시킨다. |
| 16 | 이전 앱 process의 log marker 또는 PixelCopy 파일이 현재 성공을 가장할 수 있는가? | scene별 새 process와 run ID를 사용하고 PID/run/generation/revision marker가 일치하는 로그·파일만 수집한다. |
| 17 | 빌드마다 다른 앱 binary로 실행한 성공 pixel과 실패 runtime을 한 행렬로 합칠 수 있는가? | 각 플랫폼에서 한 번 빌드하고 그 동일 binary hash로 10개 scene을 실행한다. |
| 18 | 한 scene 실패 후 앞선 성공 결과가 전체 summary를 success로 남기는가? | 6개 sample 대조 및 4개 reject guard가 모두 끝난 뒤에만 행렬 성공 보고서를 쓴다. |
| 19 | reject case가 잘못된 오류여도 “거부”라는 이유만으로 통과하는가? | 각 scene의 expected code·node·property와 누락된 partial frame·non-empty present를 구체적으로 확인한다. |
| 20 | AVD 결과가 하드웨어 GPU 동작이나 iOS/Android 실기기 parity로 과장되는가? | 대상·renderer를 결과에 기록하고 SwiftShader/Metal Simulator의 경계를 명시한다. Android 실기기가 연결되지 않으면 실기기 항목을 완료하지 않는다. |

검토 중 드러난 좌표 재현성 및 같은 실행 binary 귀속 문제를 계획에 반영했다. 지원 밖 네 scene은 픽셀 차이를 주장하는 대신 no-draw 계약에 귀속한다. 계획 자체의 완료 판정은 위 조건으로 제한한다.
