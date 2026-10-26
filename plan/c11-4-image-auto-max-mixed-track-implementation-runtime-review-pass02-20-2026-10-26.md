# C11.4 image auto/max mixed-track implementation/runtime review · pass02

**판정 상태: 임시 검토.** 2026-10-26 기준 PR head `e4570a04f1f8d0595836e3f35d7fa6a88b7c8b83`와 현재 미커밋 worktree의 fail-closed test, plan, Android matcher delta를 검토했다. Android matcher가 바뀐 현재 source에서의 최종 Android/iOS build 및 simulator rerun은 아직 기록되지 않아 최종 PASS/PARTIAL 판정을 보류한다. 이 review는 pass01의 20개 관점을 재사용하지 않고 변경된 rejection/JSON boundary에 맞춰 새로 구성했다.

## 20개 관점

| # | 확인한 실패 경로·불변 조건 | 결과 |
| ---: | --- | --- |
| 1 | 새 scene ID `image-auto-max-mixed-tracks`가 runtime case, FFI scene registry, 플랫폼 allowlist에서 일치하는가 | PASS |
| 2 | 새 root/item ID, 100×40 root, 24×16 natural item, 5px gap이 케이스 기대값과 같은가 | PASS |
| 3 | FFI source가 `minmax(auto, max-content) 17px`와 `auto` row만 구성하고 overflow/추가 item을 넣지 않는가 | PASS |
| 4 | 새 scene source와 fixture ABI가 opt-in fixture 경계에만 들어가며 product archive negative control과 opt-in positive control이 있는가 | PASS — archive 검사는 이전 Rust source revision `e4570a04`에서 수행됐고 이후 변경은 Android/test/docs 경로다. 해당 revision 연결은 최종 증거에서 다시 설명해야 한다. |
| 5 | grid validator가 box geometry, gap, border, row/column list, repeat/areas/line-name 및 auto-flow 조건을 거부하는가 | PASS — 구현은 exact typed values를 비교한다. |
| 6 | `minmax(auto,max-content)` 및 17px fixed neighbor mapping이 새 `Track` variant로 정확히 허용되는가 | PASS |
| 7 | `min-content` minimum 대체, fixed neighbor 17→18, row breadth 변이가 `track functions`로 닫히는가 | PASS — direct validator tests가 error code/detail을 고정한다. Row list 길이 변이는 별도로 테스트하지 않았다. |
| 8 | root width/height/border/column-gap/row-gap 변이가 `container box geometry`로 닫히는가 | PASS |
| 9 | `align-items`·`align-content`·`justify-content` 변이가 실제 선행 `require_property` boundary인 `unsupported_c114_image_style`로 닫히는가 | PASS |
| 10 | item line/span 지정이 `replaced item box or placement` error로 닫히는가 | PASS |
| 11 | allowlisted scene의 비-HTML root, wrong item ID 및 wrong source가 computed-style lookup 이전에 거부되는가 | PASS — `prepare()` tests가 error code/detail을 확인한다. |
| 12 | 미등록 root ID가 `unsupported_c114_image_fixture_root`로 거부되는가 | PASS |
| 13 | 정확한 HTML root/item과 computed styles를 둔 `prepare()` 경로가 잘못된 root width를 거부하고 `C11_4ImageInput` 성공값을 만들지 않는가 | PASS — 이 검사는 `prepare()` error 반환까지만 관측한다. |
| 14 | Rust rejection tests가 fixture-only module 아래 있고 테스트 파일이 권장 500줄 한도 이내인가 | PASS — 테스트 파일은 483줄이다. |
| 15 | Android trace matcher가 `grid`, item/span/index, axis/stage/kind 필드에 JSON type coercion을 허용하지 않는가 | PASS — Number/Boolean/String를 명시적으로 검사한다. |
| 16 | Android contribution 값·base size·growth limit이 누락, null, 문자열, NaN 또는 무한대일 때 통과하지 않는가 | PASS — `c114ImageFiniteNear`가 Number와 finite 값을 요구하고 오차 상한을 `<= 0.0001`로 검사한다. |
| 17 | Android gutter/collapsed/infinitely-growable/crosses-flexible booleans가 문자열 변환으로 통과하지 않는가 | PASS — Boolean 객체 형식과 값 모두 검사한다. |
| 18 | 두 axis와 6개 stage의 배열 cardinality/order, 5/3 raw slots, boundary sentinel 및 실제 gap 상태가 기대와 일치하는가 | PASS — 새 matcher는 모든 배열 길이·순서를 고정한다. 최종 simulator rerun은 대기 중이다. |
| 19 | `Initialize` intrinsic slot의 JSON null growth projection과 이후 finite growth, minimum-source 문자열 및 non-minimum null provenance를 구분하는가 | PASS — null은 `has`와 `isNull`을 함께 확인한다. |
| 20 | screenshot, paint, artifact, PR image-link와 browser 종료 근거가 검증 범위를 넘겨 주장하지 않는가 | PASS — 기존 README는 image draw 0을 한정하고, Agent Browser JSON은 두 URL intrinsic dimensions와 session/process 종료를 기록한다. |

## 임시 PARTIAL 및 최종 판정 대기

- Java matcher 변경 뒤 현재 source로 Android build/AVD rerun이 아직 없고 iOS final-source simulator run도 대기 중이다. pass01의 simulator 결과는 matcher 변경 전 결과이므로 현재 변경을 검증하지 않는다.
- `c114ImageInteger`, `c114ImageBoolean`, `c114ImageString`, `c114ImageFiniteNear`에 대해 malformed JSON field mutation을 직접 주입하는 Android unit test는 없다. Static code boundary와 정상 simulator trace만 확인했으며, 이를 mutation-test 통과로 확대하지 않는다.
- 계획 표의 extra-child, nested/second/non-leaf, adapter profile/decode/dimension/stale branch는 새 scene용 `prepare()` test로 모두 고정되어 있지 않다. 일부 generic decode/stale lifecycle는 `spinon-image` 하위 테스트가 담당한다. 제한 범위와 재사용 근거는 targeted plan review로 확정한다.
- fail-closed evidence README의 계획 source SHA는 현재 plan 수정 후 값과 맞지 않는다. 새 plan hash로 evidence를 맞춘 뒤 최종 review에서 확인한다. Runtime README의 한글화도 PR 전에 마쳐야 한다.

새 Android/iOS simulator 및 source/app hash 근거가 고정되면 이 review의 영향 관점을 재확인해 final addendum 또는 판정을 갱신한다.
