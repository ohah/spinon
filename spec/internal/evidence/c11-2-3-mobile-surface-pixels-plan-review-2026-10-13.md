# C11.2.3 모바일 표면 픽셀 계획 실패 관점 검토 · 2026-10-13

검토 대상은 [표면 픽셀 검증 계획](../../../plan/c11-2-3-mobile-surface-pixels.md)과 그 상위 C11.2.3 완료 관문이다. 기존 C11.2.3 구현 검토와 구분해, 수집 계획의 실패 가능성만 평가한다.

| # | 실패 관점 | 계획의 방어·판정 |
| --- | --- | --- |
| 1 | geometry 결과나 present marker를 실제 pixel 결과로 오해 | Android PixelCopy PNG와 iOS drawable RGBA만 pixel 근거로 인정한다. |
| 2 | C11.2.2 또는 다른 Chromium revision을 섞음 | C11.2.3 HTML v6, runtime matrix v1, Chrome reference v7의 schema·SHA-256을 capture 전에 고정한다. |
| 3 | 25개 scene 전부가 성공이라고 과장 | 성공 19개만 pixel 대상, expected rejection 6개는 별도 no-request 확인이다. |
| 4 | 거부 입력의 잔여 surface를 이전 frame으로 성공 판정 | 거부 장면은 새 process/host에서 실행하고 pixel request 0회·부분 frame 부재를 확인한다. |
| 5 | Chrome DPR 2 좌표를 DPR 1 표본에 혼용 | reference observation과 screenshot을 명시적으로 DPR 1에 고정한다. |
| 6 | CSS 좌표와 drawable 픽셀 좌표를 동일시 | 실제 surface 크기와 viewport로 scale을 구하고 같은 중심·floor 규칙으로 변환한다. |
| 7 | Android density를 정수 상수로 가정 | PixelCopy 결과 크기와 surface geometry를 기록하고 density는 진단 정보로만 남긴다. |
| 8 | 이전 Android surface generation의 늦은 PixelCopy callback을 결합 | 요청·callback 모두 generation, renderer, 크기, scene을 재검증하며 달라지면 실패한다. |
| 9 | Android PixelCopy PNG가 SurfaceView 외부 UI를 포함한다고 오해 | capture source를 `SurfaceView`로 제한하고 보조 screenshot을 surface 증거로 쓰지 않는다. |
| 10 | iOS Simulator screenshot을 Metal drawable readback으로 대체 | FFI readback 결과만 표본 비교에 사용하고 UIKit/simctl screenshot은 보조로 분류한다. |
| 11 | iOS readback을 OS compositor 뒤 화면으로 과장 | 표면은 compositor 제출 전 drawable이라고 문서·요약에 한정한다. |
| 12 | iOS drawable resize와 queued readback이 교차 | renderer, generation, drawable 크기와 runtime revision이 동일한 callback만 수용한다. |
| 13 | color edge 또는 antialiasing을 sample로 골라 결과를 불안정하게 만듦 | 기본은 Chrome PNG의 3×3 동일 RGBA 내부점이다. 한 축이 3 CSS px 미만인 opaque flat-color box만 정수 경계가 맞는 1×1 표본을 허용하고 유형을 기록한다. |
| 14 | hidden/occluded/out-of-viewport 노드에 표본을 강제 | visible interior가 없는 노드는 표본에서 제외하고 사유를 manifest에 적는다. |
| 15 | 긴 track-limit item이 viewport 밖 픽셀까지 검증됐다고 오해 | viewport 안 visible sample만 선택하며 clipped 영역은 검증 범위에서 제외한다. 1px 높이 item은 visible pixel 1개만 비교한다. |
| 16 | sample 누락·중복·잘못된 ID를 일부 성공으로 집계 | scene별 ID 집합·개수·좌표를 정확히 비교하고 중복/누락 즉시 실패한다. |
| 17 | 오래된 log marker나 병렬 앱 실행 결과를 현재 run에 섞음 | 실행별 고유 run ID와 PID를 필수로 하고 marker 전체의 소속을 검사한다. |
| 18 | fixture 생성기가 기준 CSS/PNG와 분리되어 기대 색이 임의 지정됨 | HTML palette와 Chromium screenshot에서 sample RGBA를 생성하고 원본 hash를 함께 고정한다. |
| 19 | RGBA mismatch를 tolerance 확대로 숨김 | exact 판정을 capture 전에 고정한다. mismatch는 원인 조사 대상으로 남긴다. |
| 20 | 플랫폼 근거를 다른 범위로 승격 | Android 실기기와 iOS Simulator를 분리하고 iOS 실기기·compositor·전체 Grid/WPT·성능을 명시적으로 제외한다. |

## 검토 반영

- 대상 장면 수를 25 전체가 아닌 19개 성공 scene으로 한정하고, 거부 6개는 readback하지 않도록 분리했다.
- Chrome oracle DPR 1, surface 크기 기반 좌표 변환, exact RGBA, generation/revision/run ID 검사를 사전 판정 기준으로 고정했다.
- 숨김·가림·viewport clipping·iOS compositor 경계를 별도 한계로 명시했다.
- track-limit scene의 1px 높이 단색 item에 3×3 표본을 요구하는 불가능한 조건을 찾아, 정렬된 단일 pixel 예외를 추가하고 전체 box 검증으로 과장하지 않도록 고쳤다.
- 결과 fixture·원본 로그·PNG와 문서/Tailnet 동기화까지 실행 순서에 포함했다.

이 계획 검토는 pixel readback 구현·Android/iOS 실행 또는 C11.2.3 구현 완료를 뜻하지 않는다.
