# C11.4 `fit-content()` column limit 구현/runtime 실패 관점 검토 20개

**검토 대상:** `feat/c11-4-fit-content-column-axis-runtime`의 runtime adapter, FFI report, Android/iOS fixture route 및 실행 evidence. 2026-10-30 구현 변경을 검토했다. 공식 C11.4 checkbox와 완료 수 `72/213`은 유지한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 다른 C11.4 scene ID나 접미사 주입이 새 fixture를 오인하는가 | 네 ID allowlist 외 scene은 선택되거나 계산되지 않는다. FFI exact router도 unlisted ID를 거부한다. |
| 2 | HTML 외 namespace, 다른 tag 또는 root가 아닌 부모가 들어오는가 | namespace/tag/root-parent를 고정하고 일치하지 않으면 fail-closed한다. |
| 3 | root 직계 item 수가 0개·복수이면 기존 scene으로 떨어지는가 | 정확히 하나만 허용하며 mismatch는 unsupported fixture 오류다. |
| 4 | item의 direct-child ownership나 고정 ID가 바뀌는가 | parent handle과 root/item/child ID를 각각 검사한다. |
| 5 | child 수·ownership·ID 변경을 놓치는가 | child 하나, direct ownership, exact ID를 계산 전에 확인한다. |
| 6 | 스타일 snapshot에 fixture 밖 노드·중복 style·누락 node가 섞이는가 | node ID set의 cardinality와 root/item/child 포함 여부를 검사한다. |
| 7 | 다른 computed-style profile이나 root CSS 값으로 실행되는가 | profile, display, dimensions, grid properties, spacing, border, overflow를 typed 및 serialized 형태로 고정한다. |
| 8 | `grid-auto-flow`, explicit tracks, rows, gaps, repeat/area metadata가 달라지는가 | `columns` empty, 고정 row, `fit-content(limit)` auto column, column flow 및 미지원 metadata 부재를 확인한다. |
| 9 | Grid projection이 authored value와 다른 typed input을 생성하는가 | `GridContainerInput`의 rows, auto-columns, auto-rows, flow와 placement mode를 exact 비교한다. |
| 10 | `min-width:0` scene이 automatic minimum을 받거나 control이 0으로 축소되는가 | 세 기본 scene은 specified minimum 0, control은 automatic minimum 100과 branch source를 요구한다. |
| 11 | child max-content가 limit 이하·초과로 잘못 고정되는가 | 네 scene 모두 child width와 min-/max-content contribution 100px을 확인한다. |
| 12 | `fit-content()` argument가 automatic minimum을 clamp하는 것으로 처리되는가 | 40px limit, 100px auto minimum control에서 used column 100px을 검사한다. |
| 13 | raw `growth_limit`을 used base나 fit-content argument로 잘못 기록하는가 | 모든 scene에서 raw growth limit 100px, finite/non-infinite 상태와 used base를 따로 확인한다. |
| 14 | min-content/minimum/max-content contribution이 순서·값·source를 바꿔도 통과하는가 | item, span, flexible-span flag, 세 contribution kind·값 및 nullable minimum source를 고정한다. |
| 15 | 다른 axis pass, 누락 trace 또는 추가 trace가 결과를 가리는가 | root trace 하나에 Columns와 Rows pass 각 하나만 허용하고 두 axis의 단계/track 상태를 검증한다. |
| 16 | root/item/child frame이 누락·중복되거나 expected width 밖으로 나가는가 | 모든 node frame의 정확한 집합과 root/item/child x·y·width·height를 비교한다. |
| 17 | style/document/render/environment revision mismatch가 오래된 결과를 재사용하는가 | observation style revision과 layout key/diagnostic revision을 결합해 일치를 요구한다. |
| 18 | Android route가 다른 scene을 허용하거나 잘못된 JSON/geometry를 성공 처리하는가 | allowlist와 strict report matcher를 적용하고 frame, contributions, full trace, draw/presentation identity를 검사한다. |
| 19 | iOS route, Metal readback flag 또는 drawable/sample identity가 빠져 검증을 과장하는가 | 네 scene을 strict match하고 sample 14/14 exact를 확인했다. readback은 compositor 제출 전임을 evidence에 한정했다. |
| 20 | fixture가 product artifact에 들어가거나 실행 근거가 다른 binary에 대응하는가 | product static archive marker/symbol 0, fixture-test positive control 25 strings/25 symbols이며 설치물 SHA와 build SHA가 일치한다. 앱 bundle 전체 제외로 확대하지 않았다. |

## 수정 후 재확인

초기 통합에서 발견한 matcher 결함은 CSS `grid-auto-flow` 소문자 직렬화, Android 정수 frame 표기, Swift `Double` 문자열의 중복 `.0`였다. 해당 strict matcher를 고친 뒤 Android/iOS 각각 네 scene 전체 matrix를 다시 실행해 통과했다. iOS 검증은 Metal readback opt-in flag를 켜고 다시 빌드했다. 이 보정은 고정된 독립 기대 geometry를 바꾸지 않았다.

검토 중 533줄이던 exact validator를 style contract와 track trace contract의 두 하위 모듈로 나눴다. 모듈 경계 변경 후 Rust fixture test 4/4가 통과했고, Android/iOS app matrix와 static archive 경계를 새 source로 다시 빌드·검증했다.

모듈 분리 뒤 CSS fixture boundary 검사가 새 validator 파일 위치를 확인하지 못해 실패했다. 검사를 두 하위 모듈까지 읽도록 고친 뒤 관련 boundary test 2/2가 통과했고, focused Clippy도 runtime/FFI 모두 통과했다.

## 실행 근거와 한계

[시뮬레이터·빌드·focused checks·archive boundary 결과](./README.md)에 source/build hash, simulator 식별자, screenshots와 raw logs를 기록했다. Android는 API 37 software AVD, iOS는 Simulator이며 iOS sample은 compositor 제출 전이다. 실기기/hardware GPU, iOS compositor-after pixel, 일반 author CSS admission, WPT 전체 및 C11.4 완료는 이 검토에 포함하지 않는다.
