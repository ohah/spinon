# S04 정적 snapshot hit-test 사전 비교 기준

**상태:** 구현 전 기준 고정 · 결과는 [S04 hit-test 실행 근거](s04-hit-test-platforms-2026-10-07.md)에 기록

## 목적과 제외 범위

S04의 불변 `StaticRenderSnapshot`에서 CSS px 좌표가 어느 노드를 가리키는지 계산하고, Android·iOS 시뮬레이터의 실제 터치 좌표를 같은 좌표 변환에 통과시킵니다. HTML의 `document.elementFromPoint()`를 고정 Chromium oracle로 사용합니다.

이 실험은 고정 snapshot의 기하 hit-test와 플랫폼 좌표 연결만 다룹니다. CSS 일반 paint order·stacking context·clip·transform, 화면에 실제 표시 완료된 프레임 추적, 동적 snapshot, stale document/node 폐기, DOM 이벤트 전파·취소, 접근성 및 JavaScript callback은 지원하지 않습니다. `Queue::present()` 호출은 표시 완료 증거로 쓰지 않습니다.

## 입력과 oracle

- 렌더 fixture: `S04-asymmetric-y-v1`, viewport `301×65 CSS px`.
- hit-test 점과 사전 예상 대상: [`hit-test.v1.json`](../../../../tests/fixtures/css/s04/hit-test.v1.json).
- 기존 화면 CSS·Chromium geometry reference는 [S04.8 기준](s04-asymmetric-y-precomparison-2026-10-07.md)을 그대로 사용하고 덮어쓰지 않습니다.
- Chromium은 각 fixture 점을 root의 global rect 원점만큼 이동해 `document.elementFromPoint()`로 측정합니다. root 밖이나 viewport의 right/bottom 경계는 `null`로 기록합니다.
- oracle 사전 확인에서 viewport `(0,0)`은 부모의 왼쪽 위가 아니라 첫 번째 자식 내부로 판정됐습니다. 따라서 root만 노출되는 공백 판정점은 `(0,14)`로 잡아 자식 영역과 혼동되지 않게 했습니다.

## 사전 판정 기준

1. 유한 좌표는 `left ≤ x < right`, `top ≤ y < bottom`의 half-open 사각형으로 판정합니다. right/bottom 경계는 해당 box에 포함하지 않습니다.
2. 겹치는 box는 snapshot에 기록된 가장 큰 `paint_order`가 대상을 소유합니다. 이는 기록된 fixture 순서의 규칙이며 CSS stacking context 계산을 주장하지 않습니다.
3. 표면 입력은 `scale = min(density, surfaceWidth / viewportWidth, surfaceHeight / viewportHeight)` 및 가운데 letterbox offset으로 CSS px에 역변환합니다. letterbox 바깥은 대상이 없습니다.
4. 표면 입력은 현재 surface generation과 성공적으로 제출한 같은 generation의 frame이 있어야 합니다. generation이 오래됐거나 새 surface에서 아직 frame을 제출하지 않은 요청은 오류로 거부합니다.
5. Chromium 점별 target ID와 Rust fixture의 대상 ID가 모두 정확히 일치해야 합니다. 플랫폼 탭은 중앙 자식 band 세 곳과 gap/root 영역을 확인하고, NodeId·frame sequence·surface generation을 로그에 남깁니다. 최대/평균 오차 집계는 사용하지 않습니다.

Android의 `MotionEvent` 좌표는 SurfaceView local physical pixel이고 iOS의 `UITouch` 좌표는 UIView point이므로 iOS 입력만 현재 drawable density를 곱합니다. 양 플랫폼에서 입력 callback을 실제로 주입하지 못하면 그 플랫폼 입력 연결은 미검증으로 남기며, Rust 단위 테스트를 대신 증거로 쓰지 않습니다.
