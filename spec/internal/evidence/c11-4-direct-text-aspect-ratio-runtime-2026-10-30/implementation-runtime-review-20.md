# C11.4 direct-text preferred aspect-ratio implementation/runtime review — 20 checks

**검토 대상:** plan PR #833 이후 추가된 direct-text aspect-ratio collector, four-scene FFI adapter, Android/iOS verifier, capture runner와 최종 source-hashed simulator evidence. 최종 run은 [Android](./android-avd/c114-text-ratio-mvud6qie-49123a5e/result.json), [iOS](./ios-simulator/c114-text-ratio-mvud70h8-993b852b/result.json)이며, 공식 완료 72/213과 C11.4/C11 미완료 상태를 유지한다.

| # | 실패 관점 | 근거와 판정 |
| --- | --- | --- |
| 1 | 다른 profile, stale snapshot 또는 다른 document revision이 ratio collector로 들어오는가 | validate_snapshot_revision은 RuntimeBlockFlexGridTrackSizingV1, generation, document revision, render-tree revision을 snapshot과 비교한다. 일치하지 않으면 오류로 끝난다. **PASS** |
| 2 | 임의 Grid root가 ratio 전용 collector를 선택하는가 | is_aspect_ratio_grid_root_candidate는 정확히 두 max-content 열, auto 또는 40px 단일 행, 100px width, auto height만 후보로 둔다. 나머지는 ratio profile을 주장하지 않는다. **PASS** |
| 3 | HTML이 아닌 root나 item namespace가 허용되는가 | validate_grid_root와 direct_text_source가 HTML namespace 및 item의 root 직접 자식 관계를 검사한다. **PASS** |
| 4 | anonymous non-whitespace text 또는 2개 초과 text item이 배치되는가 | root 직계 anonymous text는 거부되고, meaningful text가 있는 direct element 후보가 2개를 넘으면 세 번째 candidate node를 지목해 거부한다. **PASS** |
| 5 | 중첩 element, 다중 Text leaf, replaced content가 단일 direct-text item으로 오인되는가 | item은 의미 있는 direct Text node 하나만 소유해야 한다. descendant element는 거부되며 root subtree의 HTML img도 profile에서 제외된다. **PASS** |
| 6 | whitespace-only Text leaf가 입력 text나 item 수를 바꾸는가 | is_only_collapsible_html_whitespace를 사용해 collapsible HTML whitespace만 무시하고 meaningful source는 그대로 센다. **PASS** |
| 7 | text shape나 ratio 대상이 scene ID에서 추론되는가 | 실제 HostDocument Text를 읽어 한 A와 한 AAAA를 요구한다. aspect-ratio는 computed item style에서만 읽고 AAAA가 아닌 text에 있으면 거부한다. **PASS** |
| 8 | 2 / 1 이외 값, 다른 item, 중복 ratio가 조용히 허용되는가 | typed layout_aspect_ratio == Some(2.0), computed property 2 / 1, 대상 AAAA, ratio item 수 최대 1을 확인한다. wrong item/value를 별도 negative test로 고정했다. **PASS** |
| 9 | ratio 제한이 auto ratio 또는 arbitrary ratio까지 넓어지는가 | 이 collector는 typed ratio 2.0과 exact computed property만 허용한다. 여러 값을 일반 CSS 의미로 추정하지 않는다. **PASS** |
| 10 | Ahem 이외 폰트·크기·line-height·writing axis가 계산에 섞이는가 | item style은 Ahem 10px/400/normal, line-height 10px, collapse, LTR, horizontal-tb/Mixed orientation을 검사하고 font engine에 Ahem을 등록한다. **PASS** |
| 11 | item의 width/height/min/max, box edge, position, placement, order 변형이 지원되는 것처럼 처리되는가 | paragraph_style_for_aspect_ratio는 자동 dimensions와 min-size, 최대값 없음, zero margin/padding/border, static position과 auto placement를 검사한다. adversarial item edge/dimension/line-height case가 거부된다. **PASS** |
| 12 | root gap, alignment, auto-flow, auto-repeat, named lines/areas, border가 fixture 밖으로 새는가 | root validator가 zero gap, normal alignments, row flow, 고정 explicit tracks, auto implicit sizing, no repeat/areas/line names 및 zero root edges를 확인한다. **PASS** |
| 13 | non-replaced text가 image transferred-size-suggestion shortcut을 사용하거나 효과 축이 뒤바뀌는가 | Runtime matrix에서 row ratio는 40px inline contribution으로 auto row 20px, column ratio는 definite 40px block size에서 80px max-content column을 만든다. no-ratio control은 각각 10px row 및 40px column이다. **PASS** |
| 14 | 계산된 item frame과 실제 direct-text line box가 같다고 잘못 가정되는가 | column ratio scene은 Grid item 80×40px와 text line 40×10px을 각각 별도 expectation으로 검사한다. Android/iOS trace JSON의 text layout fields도 확인한다. **PASS** |
| 15 | track sizing contribution 누락, 추가, 순서 바뀜 또는 다른 span이 감춰지는가 | Runtime test는 두 axis pass와 exact item/kind/span/value를 검사한다. 플랫폼 matcher는 ratio path에서 contribution 개수를 exact하게 요구하고 crossesFlexibleTrack=false를 확인한다. **PASS** |
| 16 | ratio 없는 direct-text 경로 또는 legacy one-item fallback이 회귀하는가 | 기존 generic direct-text suite 및 legacy fallback tests를 포함한 focused 26 tests가 통과했다. Aspect-ratio profile은 별도 entrypoint이며 기존 prepare()는 Intrinsic profile을 유지한다. **PASS** |
| 17 | 등록되지 않은 scene ID나 fixture feature-off route가 payload/scene을 노출하는가 | FFI catalog는 네 고정 ID만 source를 만들고 unknown ID는 None이다. FFI route test 1/1 및 fixture module/build-flag boundary tests 36/36이 통과했다. **PASS** |
| 18 | Android verifier가 다른 stage 순서, extra batch, 틀린 track index/gutter/collapse/base size 또는 infinite growth를 통과시키는가 | Java ratio verifier는 pass sequence/null fields, exact stage count/name, 각 stage의 track 수·index·gutter·collapsed·base size와 infinitelyGrowable=false를 검사한다. 최종 Android 네 scene에서 통과했다. **PASS** |
| 19 | iOS verifier가 Android보다 느슨하거나 checker 자체 오류를 runtime 실패로 오인하는가 | Swift verifier도 ratio pass의 exact stage sequence/cardinality와 각 stage track values를 검사한다. 첫 strict run에서 checker의 collapsed bool 조건식 오류를 찾아 expected bool과 직접 비교하도록 고쳤고, 최종 iOS 4/4 실행에서 통과했다. 실패 당시 frame과 pixel은 exact였으며 원본 log를 보존했다. **PASS** |
| 20 | 표면 결과, 설치본 또는 archive 경계가 더 넓은 보장을 하는 것처럼 기록되는가 | 양쪽 4/4 scene·12/12 background sample exact, local/installed hashes 일치, default/fixture archive marker 0/4·4/4를 각각 기록했다. Surface sample은 item/root fill만 확인하며 glyph/compositor pixel과 APK/IPA 전체 exclusion은 주장하지 않는다. **PASS** |

**결과:** 최종 소스와 양 simulator run에 대해 20/20 실패 관점을 확인했다. 검토 중 Android/iOS contribution matcher의 중복 허용을 발견해 ratio scenes에서 exact cardinality로 제한했고, iOS strict-stage 검증기의 boolean 판정 오류를 고친 후 iOS를 재빌드하고 양 플랫폼을 다시 캡처했다. 남은 일반 CSS/WPT/기기·compositor 범위는 이 후보 구현의 완료 주장에 포함되지 않는다.
