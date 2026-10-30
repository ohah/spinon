# 구현·runtime 실패 관점 검토 20개

**대상:** C11.4 direct-text definite-axis preferred-ratio 제한 fixture 구현과 최종 Android/iOS Simulator 결과. 계획 검토와 PR diff 검토는 별도 기록한다.

| # | 독립 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 독립 기준 또는 예상값 오류로 잘못된 구현을 승인할 수 있는가? | Chrome 156 Agent Browser 사전 비교의 네 장면 CSSOM/frame/Range와 `expected-values.json`을 대조했다. 독립 계산은 `60×30`, width control `60×10`, height control `40×30`, text width 40px다. 예상 행렬과 일치한다. |
| 2 | 고정 fixture가 product/default 경로에 노출되는가? | Runtime과 FFI 모듈은 `c04-runtime-gpu-fixture-test` 경계 안에 있다. 기본 release FFI archive에서는 네 scene ID와 모듈 marker가 모두 0건, fixture-test positive control에서는 네 ID와 Rust 모듈 심볼이 확인됐다. |
| 3 | 관계없는 root를 이 profile이 잘못 받아들이는가? | Root 후보 검사에서 Grid, column/row 형태를 먼저 확인하고 비대상 root는 기존 경로로 fall through한다. 활성 후보는 `validate_grid_root`의 구조·style 검사를 통과해야 한다. |
| 4 | 이전 document의 computed style을 현재 snapshot에 재사용할 수 있는가? | `validate_snapshot_revision`가 collection 전에 실행된다. 다른 document revision을 넣는 회귀 테스트가 `unsupported_c114_text_style`로 거부되는 것을 확인했다. |
| 5 | 익명 text 또는 빈 in-flow item이 두 item의 의미를 바꾸는가? | 익명 meaningful text, empty item, 두 후보를 넘는 root child를 거부한다. anonymous 및 extra-item negative tests가 통과했다. |
| 6 | nested/replaced/잘못된 text source가 일반 direct-text로 축약되는가? | 각 후보에서 `direct_text_source`로 직접 Text node를 요구하고 허용 문자는 순서상 `A`, `AAAA`뿐이다. 다른 형태는 source 오류로 닫힌다. |
| 7 | ratio가 짧은 `A` 또는 다른 node의 style에서 유입될 수 있는가? | `A` item은 automatic dimensions와 computed `aspect-ratio:auto`를 요구한다. wrong-owner negative test가 owner NodeId와 오류 코드를 확인한다. |
| 8 | 두 owner 또는 중복 paragraph input이 허용되는가? | root는 정확히 두 meaningful text item을 요구하고 중복 short/long item과 불완전 ownership을 거부한다. Ratio owner는 한 `AAAA` item만 지정될 수 있다. |
| 9 | typed ratio와 computed serialization이 서로 다른데 성공하는가? | `layout_aspect_ratio`, box-sizing basis, `computed_ratio_matches`, 정확한 `2 / 1` serialization을 함께 검사한다. `3 / 2` mutation은 NodeId가 포함된 unsupported-style 오류로 거부됐다. |
| 10 | no-ratio control이 우연히 ratio transfer를 받는가? | Control은 typed ratio `None`, computed serialization `auto`를 요구한다. 네 장면 중 두 control이 양 플랫폼 frame·trace·표면 비교를 통과했고 ratio owner는 `None`이었다. |
| 11 | 비대상 definite 값, 양축 definite 또는 auto 외 크기 조합이 들어오는가? | 입력 dimension은 정확히 `(60px, auto)` 또는 `(auto, 30px)`만 허용한다. 두 값은 projection에서도 typed dimension과 property 문자열이 함께 확인된다. |
| 12 | `min/max`, percentage, `calc()`가 이 profile에 우회 포함되는가? | paragraph profile은 min/max를 허용하지 않고 collector는 두 dimension 조합만 통과시킨다. 일반 min/max/percentage/CSS admission 경로는 바꾸지 않았다. |
| 13 | root track/gap/alignment/direction 같은 미고정 조건이 oracle 결과를 바꾸는가? | root 검증은 고정된 `max-content max-content` columns, `auto` row, zero gap, normal alignment, LTR horizontal Grid shape에 한정한다. 화면 실행에서 예상 track list와 root frame이 일치했다. |
| 14 | font·line-height·wrapping·box edges 차이로 다른 intrinsic 값을 승인하는가? | 고정 Ahem 10px/10px, nowrap, normal word-break, zero edges를 요구한다. 시스템 font fallback 없이 Ahem 등록 후 두 플랫폼 text width 40px과 height 10px을 확인했다. |
| 15 | text layout이 없는 NodeId에 ratio를 적용하는가? | `definite_axis_ratio_node`가 지정되면 runtime dispatch 전 `paragraph_styles` membership을 다시 확인한다. 누락은 layout failure로 닫힌다. |
| 16 | ratio owner metadata가 unrelated fixture나 existing all-auto ratio route를 가로채는가? | `definite_axis_ratio_node`는 새 collector에서만 설정한다. 기존 automatic-dimension ratio regression test는 기존 collector가 계속 처리하고 새 owner는 `None`임을 확인했다. |
| 17 | FFI dispatcher가 네 장면 외 임의 scene source를 제공하는가? | dedicated FFI module은 four-ID table만 조회한다. exact four source scene test 통과, unknown ID는 `None`/error로 거부됐다. 새 public ABI는 추가하지 않았다. |
| 18 | Taffy output이 CSS expected size와 다르거나 controls가 원인을 분리하지 못하는가? | Rust expected matrix, 55-scene runtime traces, root/item frames와 intrinsic inputs가 통과했다. 신규 width/height ratio/control 네 장면은 두 플랫폼에서 모두 4/4 실행됐다. |
| 19 | app report만 맞고 실제 표면 pixels가 틀린가? | 신규 scene별 Android PixelCopy 3개와 iOS Metal drawable 3개가 exact였다. Full regression의 표본은 각 177/177 exact이며 screenshot·surface generation·installed artifact hash를 result bundle에 보존했다. |
| 20 | one-item multispan validator가 새 검사에서 trap되거나 regression을 숨기는가? | 첫 iOS run에서 조건식 평가 순서의 `expectedNodes[1]` out-of-range trap을 재현했다. Scene prefix와 item count를 먼저 검사하도록 고치고 iOS app을 재빌드해 기존 multispan 포함 55/55를 다시 통과했다. 상세는 [크래시 원인 기록](./ios-crash-triage.md)에 있다. |

**검토 결론:** 이 four-scene opt-in profile과 기록된 simulator 범위에서 위 20개 실패 경로는 닫혔다. 전체 C11.4/C11, 일반 CSS admission, hardware GPU, physical-device 및 iOS compositor 이후 결과에 대한 판정은 아니다.
