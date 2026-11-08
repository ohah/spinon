# C11.4 four-member intrinsic implicit-row list · 구현/runtime 실패 관점 20개

계획 자체의 사전 리뷰와 cap-normalization 수정 리뷰를 재사용하지 않고, 변경 Layout admission·Style projection·RuntimeSession 결과를 별도 실패 경계에서 검토했다. 근거는 이 디렉터리의 최종 focused test logs, 코드 경계, pinned Chrome 157 비교 및 [CSS Grid Layout Module Level 2 §7.6, §12.3–§12.8](https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/)다.

| # | 적대적 실패 경로 | 확인한 방어·관찰 | 결과 |
|---:|---|---|---|
| 1 | fit-content, min-content, max-content, auto 밖의 track type이 섞인다 | Layout·projection 모두 exact four-member matcher가 다른 variant에서 실패한다. | PASS |
| 2 | 유효한 24개 순서 중 일부를 누락하거나 정렬 과정에서 바꾼다 | RuntimeSession 24 permutation 각각에서 computed string과 typed member order를 모두 비교했다. | PASS |
| 3 | member가 빠져 3개 list가 들어온다 | RuntimeSession invalid-list case와 기존 3-member projection/runtime 회귀가 거부·유지된다. | PASS |
| 4 | min/max/auto가 중복되거나 추가 member가 붙는다 | Layout, projection, RuntimeSession의 duplicate/extra grammar negative controls가 통과했다. | PASS |
| 5 | px cap이 음수·무한대이거나 percentage cap이 음수·비유한 값이다 | `is_finite && >= 0` gate와 negative typed-projection matrix를 확인했다. 네-member 목록은 24순서 모두에서 `0%`와 `120%`를 projection/Layout으로 허용하고 RuntimeSession에서 zero-height item frame과 payload paint를 보존한다. | PASS |
| 6 | percentage cap이 typed percentage identity 대신 px 또는 잘못된 값으로 전달된다 | 20%는 typed `Percentage(0.2)`로 보존되고 100px basis에서 20px에 대응한다. | PASS |
| 7 | percentage cap에 indefinite block-size basis를 사용한다 | Height를 `auto`로 바꾼 RuntimeSession scene은 `grid-auto-rows` 오류로 원자 거부된다. | PASS |
| 8 | absolute `calc()`의 authored spelling이 computed projection 뒤에도 있다고 가정한다 | RuntimeSession은 `calc(12px + 8px)`의 `FitContent(LengthPx(20))` 및 canonical `fit-content(20px)`를 확인한다. | PASS |
| 9 | unresolved percentage-mixed `calc()`를 direct percentage로 오인한다 | `calc(12px + 8%)`는 partial layout 없이 실패하고 property를 식별한다. | PASS |
| 10 | fit-content cap보다 큰 payload에서 track이 cap을 넘거나 payload가 잘린다 | Layout·RuntimeSession은 24순서×20px/20% cap의 48 positive scene에서 네 순환 위치 각각 used track 20px와 60px payload를 분리해 확인했다. 첫 위치의 다섯째 row는 Grid 아래 18px, 네 번째 위치는 12px spill하고 가운데 두 위치는 Grid 안에서 item 밖으로 그려진다. non-fit-content 위치 payload는 `height`로 원자 거부된다. | PASS |
| 11 | `align-content:start`가 남은 공간을 소비한다 | 96 RuntimeSession cases에서 start 결과가 78px rows와 22px free space를 유지한다. | PASS |
| 12 | stretch가 fit/min/max track에도 free space를 더한다 | 96 cases에서 22px을 `auto` max track에만 나누고 item/payload frame을 확인했다. | PASS |
| 13 | 네 번째 뒤 implicit row의 목록 순환이 어긋난다 | 다섯 번째 row가 첫 list member를 다시 사용하도록 각 순서의 frame을 독립 계산과 대조했다. | PASS |
| 14 | list order와 placement order가 서로 다른 cycle 기준을 쓴다 | Explicit row 뒤 첫 implicit row를 cycle index 0으로 놓고 다섯 row item을 검사했다. | PASS |
| 15 | 40×100 root, 40px column 또는 10px explicit row 제약이 우회된다 | Layout/Style profile은 정확한 dimensions·track shape·zero edges/gaps·visible overflow를 검사한다. | PASS |
| 16 | 네-member list가 3/5-child legacy shape 또는 임의 child 수를 허용한다 | 기존 3/5 direct-child 경계를 보존하고 6개 외 5·7개 shape를 거부한다. | PASS |
| 17 | payload 순서·높이와 parent topology가 바뀐다 | 다섯 direct item/payload 높이 allowlist와 direct-child 조건을 검증한다. | PASS |
| 18 | align-content 거부가 엉뚱한 CSS property로 귀속된다 | 검토 중 `center`를 `grid-auto-rows`로 보고하던 오류를 발견했다. Layout 및 Style projection이 `align-content`를 반환하도록 수정하고 RuntimeSession·직접 Layout 검사를 통과시켰다. | PASS |
| 19 | 실패 revision이 일부 scene을 남기거나 다음 유효 revision을 막는다 | 중복 list와 mixed calc 실패에서 completed snapshot이 없고 동일 RuntimeSession의 유효 revision 복구가 성공한다. | PASS |
| 20 | 96 Rust cases 또는 simulator overflow screenshot을 전체 CSS Grid/표면 픽셀 증거로 확대한다 | README는 bounded profile과 Android/iOS OS screenshots의 visual smoke까지만 주장한다. Chrome/RuntimeSession frame 경계와 simulator pixels·WPT·임의 min/max contribution·전체 C11 완료를 구분한다. | PASS |

Chrome 157 관측·독립 계산은 [96-scene 사전 비교](../c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/README.md)에 있다. §7.6은 `grid-auto-rows`를 반복되는 track-size list로 정의하고 §12.8은 positive definite free space를 auto max-sizing track에 균등 분배한다. 현재 fixed-height payload는 min-content와 max-content contribution이 같으므로, 이 검토는 type identity·bounded admission·자동 stretch를 확인하며 임의 content의 intrinsic algorithm 적합성을 주장하지 않는다.
