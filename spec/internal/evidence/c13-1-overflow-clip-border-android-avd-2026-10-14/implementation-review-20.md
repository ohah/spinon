# Android border paint 구현 적대 검토 · 20개 관점 · 2026-10-14

각 항목은 별도 실패 경로를 확인했다. 검토 대상은 최종 Rust style/render/WGPU 경로와 C13.1 Android PixelCopy 재현이며, 전체 C13.1 지원 판정은 아니다.

1. **배경 paint 순서** — positioned 부모가 자식 뒤에 와서 내부 파랑을 덮던 경로를 확인했다. 단순 정적 subtree의 부모를 먼저 내보내고 AVD `(315,40)`이 파랑으로 바뀌었다.
2. **자식 border-box 오버플로** — child width가 clip보다 넓을 때 `paint_clip` 경계 `[204,316]`을 그대로 쓴다. `(318,40)`은 자식 밖 border 색으로 남는다.
3. **테두리와 부모 배경 겹침** — 같은 부모 사각형에서 border quads가 배경 위에, 자식 앞에 생성된다. 정적/absolute 오른쪽 strip 모두 Chrome gold와 일치했다.
4. **자기 clip과 조상 clip 분리** — border는 자기 overflow clip으로 자르지 않고 `RuntimeRenderBox`에 누적된 ancestor clip만 적용한다. 잘못된 전체 box clipping 회귀는 geometry helper 단위 검사에서 막는다.
5. **x/y 축별 clip** — 한 축이 visible인 기존 C13.1 route의 11 PixelCopy 표본이 모두 유지됐다. border layer가 axis intersection을 지우지 않는다.
6. **paint와 hit 순서 독립성** — paint 순서 부모→자식과 `(318,40)` hit target 자식이 동시에 유지되고, 6개 query가 고정 ID와 맞는다.
7. **C12 stacking 격리** — 재정렬 함수는 `RuntimeBlockFlexOverflowClipV1`에서만 불린다. 일반 stacking profile의 분류와 painter 순서는 바뀌지 않는다.
8. **currentColor** — border color가 `currentcolor`일 때 computed `color`로 해석한다. 기존 `border:4px solid` C13 layout fixture가 회귀로 실패했다가 해결 후 통과했다.
9. **투명 border** — alpha 0은 `RuntimePaint::None`으로 skip한다. 부분 alpha는 불투명 renderer에 몰래 합성하지 않는다.
10. **부분 alpha fail-closed** — `rgb(... / 50%)`에서 `UnsupportedComputedBorderColor`를 반환하는 전용 test가 통과했다.
11. **비단색 style fail-closed** — dashed/dotted 등 폭이 있는 비-solid edge를 solid로 그리지 않고 `UnsupportedComputedBorderStyle`로 거절한다. dashed negative test가 통과했다.
12. **폭 입력 검증** — NaN, 무한대, 음수 폭은 `RuntimeBorderSide::new`에서 거부한다. 단위 검사가 세 경우를 포함한다.
13. **면별 색/폭 매핑** — top/right/bottom/left computed 값을 각 runtime side에 전달한다. C13 fixture의 right 4px 색상 테스트와 PixelCopy가 일치한다.
14. **모서리 분할** — quads는 top/bottom을 전체 폭으로, side를 그 사이 높이로 만든다. 면 색이 다른 CSS 모서리의 대각선 join이나 radius는 아직 구현하지 않았고 README에 범위를 한정했다.
15. **높은 노드 수의 메모리 예약** — reserve는 scene당 30 vertex를 무조건 잡지 않고 opaque background/활성 border quad 상한을 계산한다. allocation은 `try_reserve` 실패를 오류로 돌려준다.
16. **빈/영 폭 edge** — width 0인 면의 사각형은 clipping 검사에서 생략된다. 불필요한 삼각형이 draw count에 들어가지 않는다.
17. **display:none 후손** — hidden node 및 subtree를 재배치 후보에서 제외한다. render node 목록에도 없는 hidden parent는 paint 순회에서 건너뛴다.
18. **복잡한 stacking 후손** — 절대/고정 부모의 모든 가시 후손이 static이고 z-index auto일 때만 subtree를 옮긴다. positioned/z-index 후손이 있으면 순서를 강제하지 않으며, 해당 조합은 미검증 범위로 남겼다.
19. **viewport·DPR 좌표** — CSS 좌표를 `400×160`에서 `1050×420` surface로 보낸 뒤 PixelCopy가 내부 표본 좌표와 exact RGBA를 기록한다. DPR=2.625의 실제 AVD 실행이 이를 확인했다.
20. **비동기 surface 교체** — route capture는 surface generation, renderer handle, 크기가 아직 일치하는지 callback 전에 다시 검사한다. 최종 PixelCopy가 현재 generation 2에서 완료됐다.

결함 하나를 확인하고 고쳤다. 기존 scene에는 border paint가 없었고 positioned 부모 paint가 자식 뒤에 있었다. 최종 기준 fixture는 구현 단위 18/18, AVD 5/5 geometry·6/6 hit·6/6 paint로 통과했다. 복잡한 stacking subtree, border radius/대각선 면 join, 실제 GPU 기기는 검증 범위 밖이다.
