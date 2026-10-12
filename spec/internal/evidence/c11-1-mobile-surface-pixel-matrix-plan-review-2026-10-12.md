# C11.1 모바일 표면 픽셀 행렬 계획 검토 · 2026-10-12

**대상:** [10장면 표면 픽셀 비교 계획](../../../plan/c11-1-mobile-surface-pixel-matrix.md)
**결과:** 아래 20개의 서로 다른 실패 관점으로 완료 조건·표본 공간·platform evidence를 검토했다.

| # | 실패 관점 | 계획에서 확인한 방어 |
| ---: | --- | --- |
| 1 | Chrome PNG가 다른 실행의 artifact로 바뀜 | 10 PNG·precomparison JSON hash를 fixture에 묶는다. |
| 2 | reference scene 순서가 inventory와 달라짐 | 10 scene ID·NodeId preorder·개수를 양쪽에서 일치시킨다. |
| 3 | node 중심 좌표가 화면 밖이라 다른 위치를 읽음 | painted item 및 track 배경 후보를 320×240 viewport와 교차시켜 생성한다. |
| 4 | 부모 node 중심이 자식/track 경계에 걸림 | 최초 generator 실행에서 `grid-under-flex` root 중심의 3×3이 비균일해지는 것을 확인했다. 투명 구조 부모는 무조건 표본화하지 않고, visible item과 scene별 track sample로 계획을 좁혔다. |
| 5 | zero-size item이 표본 0,0으로 둔갑 | 선택한 painted item의 폭·높이 0은 fixture 생성 오류다. |
| 6 | PNG viewport 360×800과 runtime 320×240을 혼합 | Chromium CSS 좌표와 플랫폼 drawable 배율을 별도로 계산한다. |
| 7 | Chrome DPR 2 screenshot이 기대값에 섞임 | DPR 1 reference를 고정하고 hash를 검증한다. |
| 8 | 평균 오차가 한 node의 색상 실패를 숨김 | 36개 visual sample의 RGBA 채널을 독립 비교한다. |
| 9 | 예상 painted item이 빠져도 scene이 통과 | HTML class inventory와 scene별 색상 item ID 집합·sample 수를 고정한다. |
| 10 | 중복 좌표/ID가 부족한 coverage를 채움 | scene별 ID 및 좌표 중복을 생성 단계에서 거부한다. |
| 11 | runtime이 다른 scene을 평가하고 sample을 읽음 | scene ID allowlist, app process, report scene ID를 결합한다. |
| 12 | 이전 앱 로그가 현재 run을 채움 | 고유 run ID와 PID, 앱 산출물 hash를 수집한다. |
| 13 | PixelCopy가 이전 surface generation을 복사 | capture 전후 surface·renderer generation이 유지돼야 한다. |
| 14 | Metal readback이 이전 revision의 drawable을 사용 | draw revision, generation, drawable metadata를 같은 표본 run에서 검사한다. |
| 15 | screenshot만으로 Metal surface 성공을 주장 | iOS 실제 drawable readback을 pass 기준으로 둔다. |
| 16 | AVD software GPU를 하드웨어 성능으로 확대 | ANGLE/SwiftShader와 물리 GPU를 구분해 기록한다. |
| 17 | RGBA/BGRA 또는 alpha 오류가 감춤 | RGBA 4채널 exact 비교 및 raw sample 보존을 요구한다. |
| 18 | FFI count overflow나 64 초과를 읽음 | 1–64 sample 계약과 null/overflow 검증을 요구한다. |
| 19 | 한 scene 실패 뒤 전체 결과가 부분 성공 처리 | 10/10 scene와 40/40 표본이 모두 있어야 pass다. |
| 20 | 검증 결과를 WPT/CSS 완료로 확대 | 문서에 실기기, WPT runner, color management, 전체 CSS 제한을 명시한다. |

## 검토 판정

계획은 Chromium 기대값의 독립성, coordinate mapping, platform surface 증거, run identity, 불완전 결과 거부 기준을 포함한다. 초기 40-node-center 가설은 부모 box가 child/track 경계를 가로지르는 scene에서 실패했다. 계획을 모든 data node 표본에서 visible item 색상과 scene별 track 배경 표본으로 수정하고 예상 합계를 36으로 재산정했다. 구현과 실행은 아직 시작 전이며, 이 검토를 구현 검토나 테스트 통과로 간주하지 않는다.
