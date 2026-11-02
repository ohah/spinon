# C11.1 `fit-content(px)` 검증 경계 계획 정정 검토 20개

구현 전 확인 중 발견한 Ahem/default renderer 불일치를 반영하고, 수정된 계획을 다음 20개 실패 관점으로 검토했다.

| # | 실패 관점 | 정정 및 방지 기준 |
| --- | --- | --- |
| 1 | Browser `@font-face` Ahem을 앱 default renderer가 로드한다고 가정 | 기본 Root HTML renderer의 `TextLayoutEngine::new_for_runtime()` 경로를 확인하고 evidence class를 분리했다. |
| 2 | Ahem exact frame을 Android/iOS system font에 적용 | Ahem intrinsic matrix는 fixed Ahem test engine으로 제한한다. |
| 3 | test fixture의 explicit font registration이 product registration이라고 주장 | 별도 `new_without_system_fonts()`/`register_font()` 호출 근거를 boundary note에 기록한다. |
| 4 | CSS `@font-face` resource loader를 이 slice에서 새로 구현 | font loading은 scope 밖으로 고정하고 새 API나 parser work를 금지한다. |
| 5 | Ahem unit test를 Root HTML integration으로 혼동 | `spinon-layout`, `spinon-style-to-layout`, default Root HTML runtime을 세 층으로 분리해 기록한다. |
| 6 | layout test에서 cap/minimum 계산을 생략 | Ahem test engine에서 cap 아래·사이·위 세 열, 정수 repeat, row floor를 독립 기대값과 비교한다. |
| 7 | named-area scene을 intrinsic matrix 안에 강제로 넣음 | `grid-area: fixed/fit`를 둔 2×1 fixed-area runtime scene에서 확인하고 Ahem intrinsic cases와 구분한다. |
| 8 | parsed Root HTML profile admission이 검증되지 않음 | computed px `FitContent`의 rows/columns, repeat, 고정 area를 projection tests로 검증한다. |
| 9 | runtime smoke가 fit-content declaration을 통과하지 않아도 성공 | source fixture에 실제 `grid-template-columns: fit-content(40px)`를 두고 completed frame 확인을 요구한다. |
| 10 | runtime smoke가 cap frame과 item frame을 혼동 | first item 40px, cap 40px, following marker origin x=40px를 별도 assertion으로 둔다. |
| 11 | overflow content가 root 경계에서 clip되는지 | direct marker x=40–48px, fixed-area child x=20–60px, fixed-row child y=0–40px로 root-relative CSS sample x=25/35/42px 및 y=10/25px의 안팎 membership을 검사한다. |
| 12 | system font metrics가 geometry를 좌우 | default-route smoke children은 fixed-size colored boxes이고 text shaping을 쓰지 않는다. |
| 13 | AVD screen capture를 surface pixel readback으로 표시 | PixelCopy 표본을 별도 기록하고 AVD SwiftShader/ANGLE 한계를 적는다. |
| 14 | iOS drawable sample을 compositor 결과로 과장 | Metal drawable이 compositor 제출 전이라는 한계를 유지한다. |
| 15 | Rust track-size proof를 simulator proof와 합침 | Ahem intrinsic, projection, runtime, surface capture를 별도 결과로 기록한다. |
| 16 | `grid-auto-*`, percentage, calc도 실수로 지원 | explicit finite px만 허용하고 implicit/percentage/calc/invalid values를 거부한다. |
| 17 | default app route 대신 fixture ABI를 제품 지원으로 주장 | 제품 fixture route/ABI를 추가하지 않고 기존 default entry로 simulator smoke를 실행한다. |
| 18 | 고정 repeat와 area 조합이 테스트에서 빠짐 | repeat 및 고정 area typed projection을 acceptance matrix에 남기고 고정-area runtime frame도 단언한다. |
| 19 | 계획 수정이 C11 완료를 올림 | C11/C11.1 체크박스와 공식 완료 수를 유지한다. |
| 20 | 수정된 기대값 검토 전에 코드 변경을 시작 | 계획 정정 PR과 실제 diff 20개 검토를 병합한 뒤 제품 코드를 시작한다. |

이 정정은 검증 방법만 분리하며 CSS support scope와 official status를 넓히지 않는다.
