# C11.4 three-fit row-span implementation PR diff 검토 30관점

검토일: 2026-11-11

검토 범위는 실제 staged implementation PR diff, 새 RuntimeSession fixture·회귀, 플랫폼 smoke 및 연결 문서다. 계획 PR diff 검토와 계획 review 20관점, 구현/runtime review 20관점을 재사용하지 않았다.

| # | 별도 실패 경로·불변 조건 | 확인 및 결과 |
|---:|---|---|
| 1 | 3-fit list의 전체 길이나 multiplicity가 잘못되는 입력 | Layout과 projection 모두 six-member list를 요구하고 `four_fit` mutation을 거부한다. |
| 2 | 세 intrinsic tail 중 중복이나 누락이 생기는 입력 | 정확히 min-content/max-content/auto 한 개씩 요구하고 중복-tail test를 거부한다. |
| 3 | 세 fit 위치의 조합 일부만 통과하는 오류 | 1–6 범위의 20개 오름차순 triple 모두 테스트했다. |
| 4 | tail 순서에 따라 admission이 달라지는 오류 | 세 tail의 여섯 순열을 각각 layout/projection에서 검사했다. |
| 5 | fit 위치가 duplicate, 역순, 0 또는 7로 주어지는 오류 | position helper 경계 test에서 거부했다. |
| 6 | six-row profile이 더 적거나 많은 implicit row에서 열리는 오류 | 3-fit span은 positive implicit row가 정확히 6일 때만 허용한다. |
| 7 | 새 span 허용이 auto-placement로 새는 오류 | 120개 typed layout 조합 전부 auto-placement predicate에서는 거부되고 RuntimeSession dense/row-auto mutation도 거부된다. |
| 8 | 3-fit row sizing profile이 four-fit/1-fit/two-fit admission을 바꾸는 오류 | 다른 fit cardinality의 기존 predicates는 그대로 두고 two-fit matrix/control/rejection을 회귀했다. |
| 9 | negative/non-finite/direct cap 검증이 percent와 px에서 달라지는 오류 | 공유 cap validator가 finite non-negative px/percentage만 허용하며 negative px, infinity, NaN, calc typed cases를 검사했다. |
| 10 | Stylo의 `calc(0px)` 정규화가 direct px와 구별되지 않는 오류 | winning `grid-auto-rows` 선언의 authored calc AST를 `layout_math_values`에 보존하고 0px·mixed percentage calc를 거부한다. |
| 11 | losing declaration의 calc가 winning direct declaration을 오염시키는 오류 | cascade rule walk가 winning property만 기록하는 경로를 사용하며 calc rejection/recovery를 RuntimeSession에서 확인했다. |
| 12 | definite basis가 없는데 percentage cap이 허용되는 오류 | 220px definite grid height를 유지하고 `height:auto` mutation을 거부한다. |
| 13 | cap이 authored track 위치와 다르게 적용되는 오류 | 모든 cap assignment와 runtime trace의 typed `auto_rows` order를 Chrome record와 대조했다. |
| 14 | calc gate가 3-fit profile 외 기존 direct px/percentage 경로를 넓게 막는 오류 | 추가 gate는 typed six-member span list에서 fit 수가 3일 때만 적용한다. 기존 one/two-fit matrix도 통과했다. |
| 15 | item span이 unsupported column, topology 또는 implicit row를 요구하는 입력 | wrong column, 비정형 span mutation, seventh positive implicit row를 각각 atomically 거부한다. |
| 16 | visible children 외 hidden variant가 자동 minimum uniformity를 깨뜨리는 오류 | layout candidate는 hidden subtree를 먼저 거르고 visible text item의 uniform `min-height:auto`를 확인한다. |
| 17 | explicit empty row control에 automatic minimum이 적용되는 오류 | 첫 explicit-row item은 `min-height:0`을 유지하고 새 mixed-mode test에도 계속 제외한다. |
| 18 | 일부 text item에만 automatic minimum이 적용되는 오류 | dark media query에서 B만 0으로 바꾸면 layout을 거부하고 light 전환 뒤 같은 session에서 복구한다. |
| 19 | scrollable overflow에서도 content-based minimum이 잘못 적용되는 오류 | item overflow `auto` mutation을 거부한다. |
| 20 | span이 auto-min track 외 조건을 포함해도 eligibility를 잘못 판정하는 오류 | trace는 auto-min track 포함, multi-track span, non-flexible max, non-replaced 조건을 확인한다. |
| 21 | automatic minimum이 text content floor 아래로 내려가는 오류 | eligible item별 line count×10px를 traced minimum·최종 spanned row 합과 비교한다. |
| 22 | row remainder가 잘못된 방식으로 distribute되는 오류 | three-fit row-span auto-minimum에만 `UpLaterTrack`을 적용하고 Chrome row values와 1/64px 내 trace 비교했다. two-fit은 기존 `Up`을 유지한다. |
| 23 | 적합한 3-fit span에서 CSS cascade와 layout revision이 달라지는 오류 | 같은 RuntimeSession의 cascade/layout key, Grid owner node와 item input revision/owner를 trace에서 대조한다. |
| 24 | content order 반전이 track 또는 item frame을 바꾸는 오류 | 5,760 forward/reverse pair에서 row values와 id별 item frame이 같음을 검사한다. |
| 25 | `align-content:stretch`가 auto 외 track을 늘리는 오류 | 5,760 start/stretch pair에서 auto row만 증가 가능하고 다른 row는 0.01px 내 동일함을 검사한다. |
| 26 | text payload grammar나 shaping 설정이 지원 경계를 넘는 입력 | 고정 ASCII payload·monospace metrics를 사용하고 비ASCII mutation을 거부하며 Chrome/runtime line count를 맞춘다. |
| 27 | 실패 revision이 일부 layout/render output을 게시하는 오류 | rejection matrix 각 case는 `Failed`, `completed=None`, node/property 식별을 요구한다. |
| 28 | 같은 session이 invalid revision 뒤 복구하지 못하는 오류 | mixed minimum과 four-fit regression 모두 invalid → valid 전환 및 새 revision을 확인한다. |
| 29 | 기존 960-scene test sentinel의 HTML·oracle field가 실 scene과 어긋나는 오류 | 사용하지 않는 D/E paint rule만 sentinel 비교에서 제거하고 `rows`/`rowValues`를 모두 읽어 960 scene을 회귀했다. two-fit 거부 입력도 이제 지원된 3-fit이 아닌 four-fit으로 바꿨다. |
| 30 | clean checkout에서 test input 또는 플랫폼 근거가 빠지거나 과장되는 오류 | 기존 C11.1 test의 `include_str!` fixture를 함께 추가했다. iOS/Android 로그·OS screenshot은 fixture SHA와 draw를 보이며 visual smoke 한계, software GPU, profile shutdown/상태 복원까지 기록한다. |

## 검토 근거와 한계

- 전체 11,520 RuntimeSession scene, 기존 one/two-fit row-span·auto-placement·text 회귀와 focused rejection/control, C11.1 percentage fixture test를 실행했다.
- Rust formatting, staged diff whitespace 검사, 변경 layout/style/style-to-layout Clippy를 `-D warnings`로 통과했다. Runtime-only Clippy는 변경하지 않은 두 helper의 `too_many_arguments` lint만 제외해 통과했다. combined multi-package Clippy는 변경 밖 `spinon-style` test helper compile 오류가 있어 통과로 기록하지 않았다.
- Android API 37.0 ARM64 AVD는 SwiftShader, iOS 26.2 Simulator는 Metal simulator GPU다. 화면은 OS screenshot이며 실기기, GPU pixel readback, compositor 후 parity를 보증하지 않는다. 공식 C11.4/C11 상태는 미완료로 둔다.
