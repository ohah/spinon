# C13.1 iOS 전체 surface pixel-center 계획 PR 변경 검토 · 2026-10-15

대상 diff: 새 iOS 표면 pixel 계획, 계획 실패 관점 검토, C13 계획 진행 메모, `spec/STATUS.md`의 계획 상태 연결. 아래 PR 변경 관점 30개는 계획 자체 실패 관점 검토와 별도로 수행했다.

| # | PR 변경 관점 | 결과 |
|---:|---|---|
| 1 | C13 parent와 C13.1을 완료로 체크하지 않는가 | PASS · 둘 다 미완료로 유지 |
| 2 | 새 계획이 기존 Android 계획의 완료 결과를 덮어쓰지 않는가 | PASS · Android 계획을 수정하지 않고 iOS 후속으로 추가 |
| 3 | C13 top-level plan에서 새 문서로 가는 상대 경로가 맞는가 | PASS · 같은 `plan/` 디렉터리 링크 |
| 4 | 상태 대장에서 계획·계획 검토 링크가 실제 경로를 가리키는가 | PASS · `spec/` 기준 상대 경로 확인 |
| 5 | 계획 문서에서 Chrome evidence 상대 경로가 맞는가 | PASS · 기존 2026-10-14 artifact에 연결 |
| 6 | 구현 전 oracle이 Android/Spinon output으로 대체되지 않았는가 | PASS · 고정 Chrome 155 PNG/observation만 expected |
| 7 | viewport, DPR, scroll과 fixture source hash가 고정됐는가 | PASS · 기존 validator 재검증 명시 |
| 8 | 보이는 경계에서 사용자 질문의 바깥-border 지점을 포함하는가 | PASS · 요소 frame 바깥의 모든 viewport center 포함 |
| 9 | viewport 바깥 화면까지 테스트한다고 오해할 문구가 있는가 | PASS · viewport 안/밖을 완료 금지선에서 분리 |
| 10 | iOS Simulator 환경이 실기기나 hardware GPU로 확대되는가 | PASS · iPhone 17 Pro Simulator로 한정 |
| 11 | 기존 iOS screenshot이 실제 Metal sample 근거로 취급되는가 | PASS · 시작 기준에서 readback 미검증으로 명시 |
| 12 | pre-compositor readback이 compositor/scanout proof로 확대되는가 | PASS · 양쪽을 남은 범위에 명시 |
| 13 | batch limit 64가 실제 renderer validator와 맞는가 | PASS · `surface_readback::MAX_SAMPLES`와 일치 |
| 14 | 마지막 batch 크기/총 batch 수 산수가 맞는가 | PASS · 30,000 / 64 = 468 remainder 48, ceil 469 |
| 15 | batch 간 fixture/run identity가 유지되는가 | PASS · run ID 및 runtime generation 매 batch 검증 |
| 16 | app route가 C13 scene 이외의 fixture에 적용되는가 | PASS · 기존 C13 route와 조합 강제, 충돌 조기 거부 |
| 17 | readback 코드가 C11/C12 fixture feature를 끌고 오는가 | PASS · C13 fixture-test gate만 확장하도록 제한 |
| 18 | default artifact에서 테스트 ABI/payload가 유출되는가 | PASS · default negative와 feature positive control 명시 |
| 19 | IOS background draw lane과 UIKit thread 경계가 보장되는가 | PASS · main thread blocking 금지, executor 검증 |
| 20 | per-batch scheduling이 render lane에 무제한 queue를 쌓는가 | PASS · 성공한 batch 뒤 한 번만 다음 draw 요청 |
| 21 | surface resize/orientation이 중간에 바뀌는가 | PASS · dimensions/generation 불일치로 run 전체 거부 |
| 22 | `scaleX`와 `scaleY`가 같다는 미검증 가정을 하는가 | PASS · 축별 actual texture ratio를 계산 |
| 23 | 두 CSS point가 같은 physical sample을 재사용하는가 | PASS · scale 하한과 physical coordinate uniqueness 검증 |
| 24 | BGRA/sRGB 채널 순서가 잘못될 가능성이 다뤄졌는가 | PASS · WGPU readback의 RGBA 정규화와 format 저장 |
| 25 | log line 크기 제한이 모든 sample을 silently truncate하는가 | PASS · app-container JSONL artifact와 요약 log 사용 |
| 26 | 절반 캡처나 이전 파일이 성공으로 admission되는가 | PASS · UUID `.part`, hash, complete manifest atomic rename |
| 27 | comparator가 sample 수만 확인하고 누락/재정렬을 통과시키는가 | PASS · index·좌표·row-major 순서를 전수 검증 |
| 28 | 한 채널의 boundary mismatch를 평균/허용오차로 숨기는가 | PASS · R/G/B/A byte mutation 및 exact 120,000-channel gate |
| 29 | shared Rust/readback 변화가 Android 결과를 깨뜨리는가 | PASS · 기존 Android AVD full grid regression 포함 |
| 30 | paint 결과가 hit/event/WPT/C13 완료로 혼동되는가 | PASS · 상호 독립 gate와 미완료 범위를 분리 |

## 결과

문서 경로, 계획 전제, C13 상태, 기존 Android/iOS 증거와 scope 구분을 대조했다. 미해결 P1/P2는 없다. 실행 동작은 후속 구현 PR에서 새 evidence로 검증해야 한다.
