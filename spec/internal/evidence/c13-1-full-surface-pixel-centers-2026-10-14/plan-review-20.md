# C13.1 전체 CSS surface pixel-center 계획 실패 관점 검토 · 2026-10-14

구현 전에 Chrome PNG·Android AVD PixelCopy 간 비교 입력, 물리/CSS 좌표 mapping, paint-order 수정 경계와 증거 한계를 검토했다. 계획은 [CSS 2.2 Appendix E의 stacking context paint 순서](https://www.w3.org/TR/CSS22/zindex.html#painting-order)와 실제 고정 Chrome fixture를 함께 대조한다.

| # | 실패 관점 | 사전 판정 기준 |
|---:|---|---|
| 1 | Chrome screenshot이 다른 source/revision의 fixture에서 생성됐다 | Observation metadata의 Chrome version/revision, user agent, source SHA, screenshot SHA를 검사한다. 변경이 발견되면 oracle을 재고정한 뒤 구현에 들어간다. |
| 2 | CSS viewport와 Chrome screenshot dimensions가 다르다 | Chrome PNG와 metadata가 모두 정확히 300×100, DPR1, scroll `(0,0)`인지 검사한다. |
| 3 | AVD PixelCopy가 이전 실행·다른 app route에서 왔다 | 최종 실행 log의 run ID, PID, fixture route, APK build hash와 PNG SHA를 서로 연결한다. |
| 4 | surface가 capture 중 교체돼 다른 generation의 픽셀을 섞는다 | PixelCopy 요청 전후 renderer handle·generation·physical size가 일치해야 성공으로 처리한다. |
| 5 | CSS 중심점을 physical pixel 좌표로 옮길 때 density 반올림을 무시한다 | x/y scale을 각각 `788/300`, `263/100`처럼 실측 dimensions에서 계산한다. 고정 scale 상수나 DPR 추정값을 쓰지 않는다. |
| 6 | pixel-center convention이 helper와 comparator에서 달라 off-by-one이 생긴다 | 양쪽이 `floor((coord+0.5)×scale)`을 공유하고 시작점 0, 끝점 299/99, 표본 52점을 직접 대조한다. |
| 7 | 한 행/열 또는 마지막 경계가 반복되거나 누락된다 | 비교 count가 30,000이고 좌표 집합이 30,000 unique인지 확인하며 각 output grid pixel에 1:1 대응을 확인한다. |
| 8 | image crop/origin이 0이 아니거나 status bar 포함 화면을 비교한다 | PixelCopy 원본은 SurfaceView bitmap이어야 하고 CSS origin은 `(0,0)`이다. 전체 앱 screenshot을 surface 입력으로 허용하지 않는다. |
| 9 | PNG decoder가 color type, alpha, CRC 오류를 조용히 넘긴다 | decoder가 CRC·dimensions·bit depth·color type·interlace를 검증하고 RGBA로 정규화하는지 확인한다. |
| 10 | alpha channel을 빼고 RGB만 맞으면 success로 처리한다 | 매 좌표의 RGBA 네 channel을 exact 비교하고 투명/반투명 mismatch mutation을 거부한다. |
| 11 | 이미지 dimensions 오류를 resampling으로 가려 통과시킨다 | 기준은 300×100, PixelCopy는 positive physical dimensions이어야 하며 CSS center mapping 이외의 resize/crop 보정은 거부한다. |
| 12 | 비교는 전체라 주장하지만 52개 고정 표본만 다시 읽는다 | comparator가 300×100 각 좌표를 순회하고 30,000개 결과를 output·digest·count로 증명한다. |
| 13 | positioned-escape의 box background/border처럼 이전 고정점에 없던 색이 빠진다 | 860 mismatch가 있던 `(200,60)`–`(263,73)` 영역과 네 palette 색을 전수 결과에 포함한다. |
| 14 | static in-flow box를 parent 뒤로 보내면서 sibling positioned escape target 순서를 바꾼다 | renderer fixture가 parent, static child, positioned sibling 각각의 paint order를 단언하고 hit order는 별도 assertion으로 유지한다. |
| 15 | nested clip/background가 parent reorder로 clip ancestry를 벗어난다 | visible·clip·hidden·mixed `clip/auto` regression과 nested clip sample를 유지하고 clip edge를 별도 검증한다. |
| 16 | 별도 stacking context/positioned descendant까지 static child 목록에 섞인다 | traversal은 out-of-flow/stacking-context descendant 하위 subtree를 분리하고 복합 fixture의 순서를 검사한다. |
| 17 | 실제 renderer fix 없이 comparator tolerance나 expected file을 바꿔 860 mismatch를 없앤다 | pass 기준은 exact 0 mismatch로 고정한다. Chrome oracle와 기존 52-point expectations는 변경하지 않고 input hash를 남긴다. |
| 18 | mismatch JSON이 과대 로그나 integer overflow를 만든다 | full machine-readable mismatch artifact와 bounded console preview를 분리한다. pixel index arithmetic은 checked 또는 검증된 유한 dimensions 안에서 수행한다. |
| 19 | iOS Simulator 화면을 Metal drawable readback으로 간주한다 | iOS regression은 41 frame·52 hit 및 Simulator 화면으로만 분류하고 drawable readback 미실행을 유지한다. |
| 20 | Android AVD fixture 결과를 전체 CSS/실기기/C13 완료로 확대한다 | 결과는 이 13-case 300×100 Android AVD grid slice로만 기록하고 hardware, event/CSSOM, WPT와 C13.1 gate는 미완료로 둔다. |

검토는 코드 변경 전에 완료했다. 현재 보존된 PNG의 30,000-point 사전 scan은 860 mismatch를 보였으므로, renderer 결함 원인을 수정하고 같은 oracle로 다시 측정해야 한다. 구현 후에는 이 계획표를 재사용하지 않고 별도 신규 20개 implementation failure review를 작성한다.
