# PR diff 실패 관점 검토 30개

**검토 시점:** 2026-10-28

**대상:** [PR #706](https://github.com/ohah/spinon/pull/706), C11.4 legacy fallback 13-scene test/evidence 변경

**기준:** `origin/main` 이후의 Rust test diff, 실행 runner·Android/iOS 산출물, 상태·계획·공개 roadmap, PR 본문을 서로 다른 실패 경로로 확인했다. 제품 layout/FFI/native matcher의 실행 동작은 바뀌지 않았다.

## 검토 결과

| # | 실패 관점 | 결과 | 확인 |
| ---: | --- | --- | --- |
| 1 | 새 Rust test module이 제품 build에 포함되거나 기존 fixture 경계를 바꾸는가 | PASS | 등록 변경은 `generic_direct_text_tests` test module에만 있고 제품 dispatcher·fixture source는 수정하지 않았다. |
| 2 | 13개 case 중 하나가 빠지거나 다른 case로 바뀌어도 통과하는가 | PASS | `TEXT_CASES` length, 기존 scene/root/item uniqueness test, scene ID별 expected-size match를 함께 확인했다. |
| 3 | 선행 specialized adapter 호출 순서가 실제 full dispatcher와 다른가 | PASS | test는 dispatcher의 기존 순서대로 14개 adapter를 먼저 호출한 뒤 generic collector와 full `prepare`를 실행한다. |
| 4 | adapter 오류가 fallback 성공으로 숨겨지는가 | PASS | 각 adapter `Err`는 adapter 이름과 scene ID를 포함해 test 실패로 전환된다. |
| 5 | one-item legacy shape를 generic collector가 새로 claim하는가 | PASS | 13개 positive 모두 collector의 `Ok(None)`을 요구하고 full dispatcher에서 기존 fallback `Some`을 요구한다. |
| 6 | HostDocument가 root 아래 한 direct item·한 Text node 이외 topology를 우연히 만든다 | PASS | fixture 생성이 case root와 item을 가진 HTML `div` 및 단일 Text node로 제한된다. |
| 7 | case root/item identity가 app fixture registry와 어긋나는가 | PASS | Rust `TEXT_CASES` identity를 생성에 사용하고 native app route의 13 ID와 대조했다. |
| 8 | Rust Text source와 paragraph source가 달라 다른 text 결과를 측정하는가 | PASS | 둘 다 `case.content`를 사용하고 app matcher는 source text도 검사한다. |
| 9 | full dispatcher가 legacy resolver를 건너뛰거나 `None`으로 조용히 끝나는가 | PASS | test가 `Some(input)`을 강제하고 negative helper는 `Err`가 아닌 `Ok(None)`도 실패로 처리한다. |
| 10 | fallback paragraph style에서 Ahem family·size·line-height·wrap mode가 빠지는가 | PASS | 13개 결과에서 Ahem, 10px font/line-height, white-space와 word-break mode를 직접 비교한다. |
| 11 | paragraph style 소유자나 two-axis mode가 잘못되는가 | PASS | item paragraph style 하나만 존재하고 `two_axis_nested_grid == false`인지 확인한다. |
| 12 | Ahem intrinsic 측정이 fixture가 반환한 engine/style/source를 사용하지 않는가 | PASS | full dispatcher의 engine 및 paragraph style로 `intrinsic_inline_sizes`를 직접 호출한다. |
| 13 | min/max-content 기대값 표가 새 scene을 빠뜨리거나 값을 조용히 기본 처리하는가 | PASS | 13 scene의 고정 값을 ID match로 나누며 알 수 없는 ID는 panic으로 실패한다. |
| 14 | typed track variant가 CSS fixture와 다르게 직렬화되는가 | PASS | 현재 13 `TextTrack` case를 stylesheet에 직렬화하며 dispatcher success와 app typed trace를 대조한다. |
| 15 | root dimensions 또는 item edge/min/max style이 기대값에서 빠지는가 | PASS | case별 px/auto dimensions, box-sizing, padding, border, min/max를 stylesheet에 반영한다. |
| 16 | percentage fit-content scene이 cascade admission에 필요한 alignment를 받지 못하는가 | PASS | percentage track이면 `align-items: normal`, 다른 scene은 `start`를 넣는다. |
| 17 | stale computed style/snapshot을 다른 document에서 재사용하는가 | PASS | 각 fixture의 동일 snapshot에서 cascade, adapter 호출, full dispatch를 수행한다. |
| 18 | 잘못된 root ID 또는 바뀐 source text가 legacy allowlist를 통과하는가 | PASS | 별도 negative test가 unknown root와 changed text의 기대 error code를 검사한다. |
| 19 | 알 수 없는 item ID가 allowlist를 통과하는가 | PASS | item ID만 바꾼 fixture가 `unsupported_c114_text_fixture_case`로 거부된다. |
| 20 | 같은 root에 추가 direct item이 있어도 one-item route가 성공하는가 | PASS | 빈 sibling도 추가 item으로 계산되고 `invalid_c114_text_grid_owner`로 거부된다. |
| 21 | 허용되지 않은 typed Grid track이 기존 case처럼 처리되는가 | PASS | first fixed `auto` column을 `min-content`로 바꾼 stylesheet가 `unsupported_c114_text_grid_style`로 거부된다. |
| 22 | FFI/Android/iOS scene registry가 달라 다른 장면을 실행하는가 | PASS | 세 source registry의 고정 ID 13개가 같은지 확인했고 실제 matrix에서 Android/iOS 각각 13개 ID가 실행됐다. |
| 23 | native matcher가 layout ready만 보고 잘못된 source/frame/track을 통과시키는가 | PASS | route는 matcher의 status, root/item frame, typed trace 및 run ID를 남기며 13 scene 모두 status 0이다. |
| 24 | ADB 명령이 연결 실기기나 다른 emulator를 대상으로 하는가 | PASS | script는 `emulator-5554`만 사용하고 emulator serial, API 37, ARM64 ABI, AVD 이름을 실행 전 검사한다. |
| 25 | iOS 결과가 다른 simulator/model에서 나오는가 | PASS | script는 지정 UDID `A7A10E3F-09B1-440F-8320-4F20E19E88D1`와 `iPhone18,1` model을 확인한다. |
| 26 | 이전 scene summary가 현재 run으로 오인되거나 trace/draw가 summary보다 먼저 잡히는가 | PARTIAL | summary/trace는 scene run ID에 묶이고 iOS log도 그 뒤 draw를 요구한다. Draw line 자체에는 run ID가 없지만 scene을 순차 실행하며 각 앱을 terminate한다. |
| 27 | 중간 실패·잘못된 ABI 또는 이전 app binary가 성공으로 기록되는가 | PASS | 빌드 로그, Android APK/iOS app hash, `source-head.txt`, API/ABI/model, pass03 사후 검증 JSON이 함께 보존됐다. |
| 28 | runner 종료 뒤 직접 실행한 앱이 남거나 기존 simulator/기기가 종료되는가 | PASS | runner는 지정 emulator와 새로 만든 simulator의 app만 force-stop/terminate한다. 실행 뒤 새 iOS simulator는 제거했고 기존 simulator와 실기기는 보존했다. |
| 29 | representative screenshot을 glyph paint나 최종 compositor pixel 검증으로 확대 해석하는가 | PARTIAL | PR 본문 두 image는 실제 로드됐고 화면은 box/surface만 보여준다. `runtime_text_count=0`, `glyph_batch_draw_count=0` 한계도 본문에 적었다. |
| 30 | 부분 test 결과가 전체 C11.4 완료, API 변경 또는 CI 통과로 승격되는가 | PASS | `spec/STATUS.md`, plan, roadmap, PR 본문 모두 C11.4 미완료·`72/213`을 유지하고 기존 내부 계약 `0.1.0`은 바꾸지 않았다. GitHub PR check rollup은 비어 있어 CI 통과로 표시하지 않았다. |

## 검토 메모

차단 결함은 발견하지 못했다. 두 PARTIAL은 draw event에 scene run ID가 없는 점과 screenshot의 glyph/compositor pixel 한계를 범위 그대로 표현한다. 전체 diff whitespace check의 끝 공백·빈 EOF 경고는 원본 도구 `.log` 13개에만 있다. 해당 log를 정규화하거나 수정하지 않고 보존했으며, code·문서·metadata check는 통과했다.
