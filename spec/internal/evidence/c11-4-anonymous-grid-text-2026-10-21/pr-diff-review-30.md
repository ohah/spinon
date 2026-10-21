# C11.4 anonymous Grid text 구현 PR diff 적대 검토 30개

검토 대상은 anonymous Grid text collector, intrinsic metadata, runtime frame export, fixture FFI·Android/iOS adapter, 캡처 도구와 관련 상태·계획 문서다. 30개 관점은 서로 다른 입력 실패, 소유권, feature 경계 또는 외부 관찰을 다룬다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 기본 artifact에 fixture API가 새는지 | anonymous item 생성 API와 runtime fixture 진입점이 opt-in feature 아래 있다. 최신 기본 `spinon-ffi` archive에는 C11.4 fixture symbol과 scene payload가 없다([검사](./fixture-artifact-default-after-frame-fix-check.log)). |
| 2 | fixture feature가 활성화돼도 positive control이 사라지는지 | fixture-test archive에 C ABI constructor/eval symbol과 hidden-child scene payload가 있다([빌드](./fixture-artifact-fixture-test-after-frame-fix-build.log), [검사](./fixture-artifact-fixture-test-after-frame-fix-check.log)). |
| 3 | 잘못된 UTF-16 source가 조용히 치환되는지 | collector는 `String::from_utf16` 실패를 `InvalidTextEncoding`으로 반환해 손실된 provenance를 게시하지 않는다. |
| 4 | 여러 원본 text node가 합쳐질 때 source ID·문자열이 유실되는지 | source node 목록과 paragraph source 목록을 정확히 대조하고 중복 source를 거부한다. adjacent-node layout test와 FFI trace가 각 source를 보존한다. |
| 5 | 첫 source가 공백이면 stable anchor가 잘못 선택되는지 | 첫 비공백 source node를 anchor로 잡으며 leading whitespace source도 같은 anonymous item에 남긴다([layout tests](./layout-anonymous-tests-after-fix.log)). |
| 6 | 빈 text node만 있을 때 phantom Grid item이 생성되는지 | 비공백 anchor가 없으면 run flush에서 source를 비우고 item을 만들지 않는다. |
| 7 | collapsible-only whitespace가 auto-placement cell을 소비하는지 | normal whitespace 아래 anchor 없는 run은 버린다. `whitespace-does-not-create-item`은 두 block이 첫 두 cell을 차지하고 다섯 장면에서 Android/iOS 모두 통과한다. |
| 8 | comment·text-node 경계가 연속 run을 분리하는지 | collector는 source sequence를 direct block에서만 flush한다. comment가 생략된 adjacent text-node case와 Chrome 155 비교가 통과한다. |
| 9 | `display:none` direct child가 run을 잘못 끊는지 | 숨김 child에서는 subtree를 제외하지만 run을 flush하지 않는다. layout test와 `anonymous-hidden-child-continuation` 양 플랫폼 실행이 통과한다. |
| 10 | 숨김 subtree가 layout node 또는 paint box로 남는지 | hidden elements는 excluded set에서 layout tree·runtime frame·render boxes에 들어가지 않는다. 다섯 scene의 expected box count와 draw marker가 통과한다. |
| 11 | visible direct block separator가 text item 두 개를 합쳐버리는지 | block 직전 run을 flush하고 block을 child order에 넣은 뒤 다음 run을 별도 item으로 만든다. source-order layout test와 simulator scene이 통과한다. |
| 12 | 지원하지 않는 visible inline/flex/grid/flow-root child가 평탄화되는지 | 제한 collector는 해당 display 종류에서 명시 오류를 반환하고 부분 layout을 만들지 않는다. |
| 13 | `display:none`인 상위 grid가 anonymous text를 수집하는지 | `is_hidden_by_ancestor`가 숨김 ancestor를 확인해 text profile을 수집하지 않는다. |
| 14 | `white-space:pre` 등 다른 whitespace 정책이 잘못 정규화되는지 | fixture collector는 `WhiteSpaceMode::Normal`만 허용하고 다른 mode를 오류로 거부한다. |
| 15 | anonymous item에 명시 placement가 들어오는지 | runtime validator가 일반 item의 네 placement 값을 모두 `Auto`로 제한하며 negative mutation test가 이를 확인한다. |
| 16 | nonzero `order`가 source order인 것처럼 처리되는지 | grid owner와 separator item의 `order:0`만 fixture에 허용한다. |
| 17 | anonymous item이 grid owner의 Ahem text style을 잃는지 | paragraph style은 owner에서 가져오며 runtime oracle가 font, size, line-height 및 collapse/wrap profile을 고정한다. |
| 18 | direction·writing mode·line breaking 차이를 숨기는지 | 현재 제한 profile의 computed typography 속성을 검사하고 LTR/horizontal-tb 등 고정 profile 밖 입력을 거부한다. |
| 19 | synthetic item id가 여러 run 또는 실제 element id와 충돌하는지 | 첫 text source를 anchor로 쓰고 중복 source/paragraph insertion을 검출한다. `NodeId`는 document 전체에서 유일하다. |
| 20 | synthetic node를 ID 정렬 순으로 배치해 DOM order가 바뀌는지 | parent child vector에서 anchor를 원래 첫 text child 위치에 삽입하고 intrinsic `source_order`는 그 vector로 계산한다. block 전후 scene이 통과한다. |
| 21 | intrinsic min/max 값이 최종 callback의 available width에서 역산되는지 | Parley paragraph를 통한 min/max-content measure를 유지하고 source kind·font revision·크기와 stage trace를 함께 기록한다. 다섯 simulator scene이 통과한다. |
| 22 | 익명 item frame을 DOM element frame으로 노출하는지 | text anchor frame은 일반 runtime frame 목록에서 분리하되 `gridItemFrames` 진단에 유지한다. 누락·비유한 frame은 오류다. |
| 23 | 일반 runtime의 기존 `display:none` frame 계약이 깨지는지 | diff review에서 숨김 output frame을 무조건 건너뛸 회귀 경로를 발견했다. output에 frame이 있는 일반 숨김 node는 기존 목록에 남기고, C11.4처럼 layout 자체에서 제외된 경우만 subtree를 건너뛰도록 수정했다. 기존 runtime regression test 1/1 통과([로그](./runtime-hidden-frame-regression.log)). |
| 24 | anonymous item을 분리한 뒤 output frame 집합 검사가 약해지는지 | frame 집합 차이는 알려진 anonymous item ID 수로만 보정한다. 기타 누락·추가 frame은 `frame_set_mismatch`로 거부한다. |
| 25 | FFI trace가 원본 text provenance 또는 익명 frame을 생략하는지 | diagnostics JSON에 `textSources`와 `gridItemFrames`를 포함하고 FFI five-scene matrix를 실행했다([로그](./ffi-anonymous-fixture-tests.log)). |
| 26 | 잘못된 FFI scene ID·pointer가 fixture host를 탈출하는지 | FFI entrypoint의 null·buffer·scene allowlist 검사를 확인했고 module boundary 검사는 6/6 통과했다. |
| 27 | Android adapter가 새 scene을 거부하거나 다른 frame을 기대하는지 | Java allowlist, node frames, box counts를 다섯 scene에 연결했다. 최신 API 37 ARM64 SwiftShader AVD 실행 5/5 및 APK hash 일치. |
| 28 | iOS adapter가 Android와 scene/geometry를 다르게 기대하는지 | Swift allowlist, root/node frames, box counts가 Android와 일치한다. 최신 iPhone 17 Pro/iOS 26.2 Simulator 실행 5/5. |
| 29 | retry suffix가 platform argument의 80자 제한을 넘어 재실행이 깨지는지 | capture script가 가장 긴 retry ID까지 실행 전에 검사한다. 이전 실패 run ID 로그를 보존했고 새 bounded ID로 양 플랫폼 다섯 장면이 통과했다. |
| 30 | 부분 fixture 결과가 제품 지원 또는 C11.4 완료로 과장되는지 | 계획·내부 계약·STATUS·roadmap은 opt-in subset과 WPT/glyph/border/실기기 한계를 구분하고 공식 체크·완료 집계를 미완료로 유지한다. |

## 최종 확인

검토 중 발견한 일반 runtime hidden-frame 회귀를 수정하고 기존 regression test, focused C11.4 tests, 기본/feature Clippy, Android AVD와 iOS Simulator 행렬을 다시 실행했다. PR diff에 남은 차단 결함은 없다. 이 결과는 glyph paint, CSS border stroke, WPT, Samsung 실기기, 전체 C11.4 지원을 증명하지 않는다.
