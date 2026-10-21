# C11.4 cyclic `calc()` 구현/runtime 실패 관점 20개

계획 검토와 구분해 최종 코드, 실행 trace, Android PixelCopy 및 iOS Metal drawable 결과를 대상으로 확인했다. 처음 드러난 세 harness 결함은 수정 후 영향받은 전체 scene/platform matrix를 다시 실행했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | `calc()`가 percentage와 px 항을 하나의 typed sum으로 보존하지 못함 | Runtime validator가 `ComputedCssMath::Sum([Percentage(0.5), LengthPx])`를 정확히 비교하고 runtime test 및 양 플랫폼 trace가 통과했다. |
| 2 | column과 row의 px 항이 서로 뒤바뀜 | column `+4px`, row `+2px` AST 및 final track `[16]`/`[10]`을 각각 대조해 통과했다. |
| 3 | percentage basis를 반대 축에서 읽음 | column-only·row-only·two-axis scene 결과가 독립 계산과 모두 일치했다. |
| 4 | cyclic percentage를 intrinsic sizing 중 auto처럼 다루지 않고 조기 resolve함 | max-content root와 item intrinsic contribution trace 뒤 최종 16px/10px track 결과를 확인했다. |
| 5 | definite axis에서 percentage basis가 container content size가 아님 | `40×20` control의 columns `[24]`, rows `[12]`를 양 플랫폼에서 확인했다. |
| 6 | cyclic column root가 item max-content width를 보존하지 못함 | root/item 각각 `24×16`, final column `[16]`과 overflow 표본이 통과했다. |
| 7 | cyclic row root가 item max-content height를 보존하지 못함 | root/item 각각 `24×16`, final row `[10]`과 overflow 표본이 통과했다. |
| 8 | 두 축 cyclic 계산이 한 축의 resolved size를 다른 축에 재사용함 | simultaneous scene에서 columns `[16]`, rows `[10]`을 독립 확인했다. |
| 9 | 길이 항이 최종 used track 계산에서 누락되거나 두 번 더해짐 | four scenes의 resolved track가 `[16]/[10]` 및 control `[24]/[12]`와 일치했다. |
| 10 | intrinsic item 크기 또는 text 측정 오염 | 단일 `XX` Ahem text의 measured min/max content `24×16`과 직접 item identity를 trace에서 확인했다. |
| 11 | 자동 배치가 다른 셀에 item을 놓음 | `grid-column:1;grid-row:1`, item `(0,0)`을 Rust 및 양 플랫폼 node frame으로 대조했다. |
| 12 | root border/padding/gap이 percentage inner basis를 바꿈 | root spacing, border, gaps를 runtime gate에서 0으로 제한하고 definite control을 비교했다. |
| 13 | final trace 검사에서 이전 pass 또는 잘못된 축을 읽음 | 두 축의 최종 pass와 마지막 stage track을 focused Rust test 및 simulator JSON parser에서 확인했다. |
| 14 | gutter/collapsed track이 expected track count에 포함됨 | trace 검사에서 gutter와 collapsed track을 제외하고 실제 단일 track을 확인했다. |
| 15 | 다른 grid node의 trace가 선택됨 | trace node가 고정 root node와 같은지 style-to-layout test에서 확인했다. |
| 16 | scene ID만 같고 임의 DOM/style 입력을 통과시킴 | exact root/item/text shape, profile, style AST, placement를 확인하고 미등록 cyclic scene을 거부했다. |
| 17 | FFI dispatcher에 scene route가 없거나 다른 scene을 돌려줌 | 네 ID source 검사와 신규 dispatcher test `exposes_cyclic_calc_track_sizing_runtime_scenes`가 통과했다. |
| 18 | Android PixelCopy가 새 sample ID·scene·색상 기대값을 누락 | 초기에 scene allowlist와 red sample color가 빠져 실패했다. 두 표를 갱신한 뒤 네 scene, 표본 `8/8` exact로 재실행했다. |
| 19 | iOS two-axis frame helper가 `cyclic-calc-two-axis`를 다른 scene으로 변환 | 공통 suffix helper가 모든 `-two-axis`를 제거하던 결함을 `nested-depth-*`에 한정했다. 기존 nested-depth six-scene 회귀가 Android/iOS에서 각각 `6/6` 통과했다. |
| 20 | device/simulator capture가 오래된 앱 또는 다른 대상에 연결됨 | Android runner가 `emulator-5558`, API 37, qemu=1과 local/install APK hash를 검증했다. iOS는 고정 iPhone 17 Pro UDID와 iOS 26.2 runtime을 확인했다. |

### 재실행 판정

최종 AVD와 iOS Simulator에서 각각 4/4 scene의 frame, axis track trace, draw/present 및 pixel summary가 통과했다. 수정된 실패 관점 18–19도 이 최종 실행에 포함된다. Android는 각 scene PixelCopy `2/2`; iOS는 각 scene Metal drawable `2/2`다. 이 표는 WPT, 실기기, hardware GPU, iOS compositor-after surface 또는 전체 CSS Grid 적합성 판정이 아니다.
