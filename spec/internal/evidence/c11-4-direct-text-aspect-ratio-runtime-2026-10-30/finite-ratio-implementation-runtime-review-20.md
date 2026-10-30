# C11.4 direct-text finite-positive aspect-ratio 구현/runtime 검토 — 20 checks

**검토 대상:** 현재 checkout의 finite-positive ratio collector, computed serialization, Taffy intrinsic contribution, 16-scene FFI/app verifier/capture runner 및 최종 Android/iOS simulator receipt. 최종 receipt는 [Android](./android-avd/c114-text-ratio-mvuhobpm-b96aae77/result.json)와 [iOS](./ios-simulator/c114-text-ratio-mvuhr3w3-23d87e87/result.json)다. C11.4/C11과 공식 `72/213` 상태는 유지한다.

| # | 실패 관점 | 근거와 판정 |
| --- | --- | --- |
| 1 | NaN, ±infinity, zero 또는 negative typed ratio가 fixture layout에 들어가는가 | `computed_ratio_matches`는 finite-positive 조건을 먼저 요구하고 synthetic invalid typed values를 거부하는 runtime tests가 있다. **PASS** |
| 2 | 숫자 serialization과 typed ratio가 다르거나 잘못된 분수 형식이어도 같은 것으로 취급되는가 | collector가 computed `numerator / denominator`를 parse해 typed `f32`와 일치시킨다. malformed, non-positive component, mismatched values는 negative tests에서 거부한다. **PASS** |
| 3 | Stylo가 degenerate numeric ratio를 `auto`로 직렬화해 numeric computed value를 잃는가 | `computed_aspect_ratio_serialization`은 clone한 computed `PreferredRatio`에서 numeric pair와 optional `auto` prefix를 snapshot에 쓴다. serialization carried test가 이를 고정한다. **PASS** |
| 4 | typed ratio가 `None`인 numeric-degenerate style이 no-ratio control로 오인되는가 | ratio 없는 fixture item은 computed serialization `auto`만 허용한다. degenerate pair를 가진 item은 `None` typed value만 보고 control로 통과하지 않는다. **PASS** |
| 5 | `auto <ratio>`가 plain ratio로 축약되어 지원되는 것처럼 되는가 | 이 plan의 Taffy mapping 경계에서 `auto <ratio>` cascade 입력은 거부되며, 해당 동작을 일반 CSS support로 선언하지 않는다. **PASS** |
| 6 | 너무 작거나 큰 finite ratio가 division/multiplication 중 overflow 또는 zero-sized geometry를 만들어 Taffy로 전달되는가 | `validate_ratio_geometry`가 40px/ratio와 definite row×ratio 파생값의 finite-positive 범위를 layout 전에 확인한다. `f32::from_bits(1)` 및 `f32::MAX` adversarial tests가 있다. **PASS** |
| 7 | 두 item 모두 ratio를 가져도 하나만 검사하는 branch 때문에 통과하는가 | ratio item count와 owner를 확인해 하나만 허용하고 wrong-item/double-ratio adversarial test가 거부를 고정한다. **PASS** |
| 8 | ratio가 AAAA 외 item에 있거나 text가 scene ID와 다를 때 scene label만으로 처리되는가 | 입력 text는 snapshot/HostDocument에서 읽으며 ratio owner가 `AAAA`인지 확인한다. fixture registry label은 style 값을 대체하지 않는다. **PASS** |
| 9 | root가 아닌 nested/extra item, anonymous text 또는 rich descendant가 direct-text layout으로 수용되는가 | root와 child 관계, 의미 있는 text node 수 및 descendant/replaced shape를 검사한다. unsupported shape adversarial tests는 layout 전에 실패한다. **PASS** |
| 10 | 다른 font, line-height, wrapping, direction 또는 writing mode가 ratio 계산에 섞이는가 | Ahem 10px, line-height 10px, horizontal-tb/LTR/nowrap 및 fixed paragraph contract를 계속 검사하며 profile 밖 style은 fail-closed다. **PASS** |
| 11 | root width, columns, gap, alignment, edge, auto-flow 또는 placement가 Chrome matrix와 달라도 같은 oracle를 쓰는가 | Root validator는 100px width, 두 max-content columns, zero gap/edges, supported alignment/flow와 auto placement를 확인한다. **PASS** |
| 12 | 임의 fixed row 값으로 geometry 검증 또는 overflow가 우연히 성공하는가 | Row parser는 Auto 또는 20·30·40·45·60px만 허용한다. 각각의 column ratio control이 같은 definite row height를 사용한다. **PASS** |
| 13 | column `1/2`에서 ratio-derived 30px preferred width가 40px text max-content floor를 깎는가 | Taffy helper가 auto-sized non-replaced item의 ContentSize contribution을 ratio-derived known size와 함께 유지한다. Chrome·독립 기대값·두 simulator trace는 column 40px와 item 40×80px을 검사한다. **PASS** |
| 14 | helper가 replaced item 또는 explicit width/height/min/max item까지 임의로 바꾸는가 | Taffy helper는 replaced item, ratio 부재, non-auto dimension/min/max 조건에서 early return한다. 지원된 profile도 box-edge/min/max 입력을 거부한다. **PASS** |
| 15 | min-content와 max-content measurement가 같은 available-space keyword를 잘못 사용하거나 반대 축을 잃는가 | Helper는 선택 axis의 known dimension만 비우고 반대 축 typed dimension을 유지하며 min/max-content에 맞는 available space와 keyword adjustment를 전달한다. **PASS** |
| 16 | width/height ratio 방향이 뒤집히거나 row/column contribution이 replaced shortcut과 합쳐지는가 | CSS ratio는 width/height이며 row scenes는 40px inline 기준 block contribution, column scenes는 definite block 기준 inline intrinsic size를 검증한다. 비-replaced text는 §6.6 replaced-item shortcut을 사용하지 않는다. **PASS** |
| 17 | item frame이 direct text line box와 같다고 취급되어 ratio box 안의 text geometry가 왜곡되는가 | Runtime matcher는 grid item frame과 direct-text intrinsic line metrics를 별도 비교한다. 40×80 또는 ratio-derived item box와 40×10 text line은 서로 다른 값을 유지한다. **PASS** |
| 18 | trace에서 contribution 중복, 누락, 순서 오류 또는 unsupported sizing stage가 숨겨지는가 | Runtime test와 Android/iOS verifier는 axis/pass, contribution cardinality, track count/index/base size, gutter와 status를 검사한다. 최종 trace가 두 플랫폼에서 16/16 통과했다. **PASS** |
| 19 | `1/64px` Chrome 양자화가 platform verifier 또는 runner에서 서로 다른 허용치로 처리되는가 | Java, Swift와 Node matcher가 numeric geometry 비교를 같은 tolerance로 사용한다. `3/2` row의 Chrome 26.65625px와 독립 26.666666…px 차이를 허용하고 surface color는 exact로 비교한다. **PASS** |
| 20 | root 밖 overflow, 오래된 설치 앱, fixture feature 경계 또는 renderer 범위가 누락/과장되는가 | `column-1-2`의 outside-root sample은 PixelCopy/Metal 각 4/4 exact다. local/installed app hashes 일치와 static-archive negative/positive control을 기록하며, software AVD·pre-compositor Metal의 한계도 구별한다. **PASS** |

**결과:** 코드 제약, intrinsic measurement 경계, runtime traces, 1/64px geometry tolerance와 표면 overflow를 서로 다른 실패 경로로 확인했다. 두 simulator 최종 run은 각각 16/16 scene 및 49/49 surface sample이 통과했다. 일반 CSS, 전체 WPT, real-device GPU, glyph paint와 compositor-after pixels는 이 후보의 완료 주장에 포함하지 않는다.
