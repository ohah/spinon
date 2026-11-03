# C11.1 `calc()` implicit-column 구현/runtime 실패 관점 검토 20개

계획 검토와 분리해 현재 구현 후보, typed math projection, Rust regression tests, Android/iOS Simulator 실행을 검토했다. 아래는 코드 경계와 입력별 실패 가능성을 다룬다. 후보는 아직 PR 병합 전이며 C11.1/C11 완료나 공식 완료 `72/213` 승격이 아니다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | calc admission이 다른 CSS profile로 새는가 | Cascade flag는 `RuntimeRootHtmlTextPaintV1`에서만 설정되고 projection은 Root HTML column-flow 계약을 확인한다. 새 일반 author CSS admission은 없다. |
| 2 | row 또는 dense column-flow에서 calc가 허용되는가 | Cascade typed-track 경로에도 `allow_calc_track_lists`를 적용했다. `grid-auto-flow:row`와 `row dense`는 cascade에서 Unsupported로 남고 sparse column flow만 Calc를 유지한다. |
| 3 | auto-row 또는 explicit template track에 auto-column calc gate가 재사용되는가 | Auto-row option은 Calc=false이고 Root HTML column-flow의 explicit rows/columns 검증은 direct px만 받는다. |
| 4 | calc math ID가 없거나 중복되거나 다른 Grid node에서 재사용되는가 | Layout 검증은 ID 누락/중복과 node mismatch를 오류로 반환한다. focused layout binding test가 각 경우를 확인한다. |
| 5 | math record property가 다른가 | 현재 math model의 `GridTemplateColumns` binding만 받으며 `GridTemplateRows` record는 거부한다. 별도 property enum이 없는 경계를 plan에 기록했다. |
| 6 | 식 모양이 제한보다 넓어지는가 | Projection과 layout 양쪽에서 top-level 2-term Sum, px 항 하나, percentage 항 하나만 받는다. product·추가 term·nested operator negative tests가 거부를 확인한다. |
| 7 | 항 순서나 음수 항의 unary sign이 바뀌는가 | px/% 순서 두 가지와 direct/unary-negate 표현을 지원한다. projection helper tests가 양쪽 signed term을 확인한다. |
| 8 | NaN 또는 무한 입력이 계산기를 통과하는가 | 두 항의 유한성을 projection/layout admission에서 확인한다. NaN AST는 projection과 layout helper에서 거부된다. |
| 9 | 사용 결과가 음수일 때 음수 track이 배치되는가 | GridTemplateColumns math resolver의 non-negative range clamp를 사용한다. `calc(1px - 10%)`는 0px가 되고 다음 implicit item 위치도 5px gap만큼 이동한다. |
| 10 | 100% 초과 결과가 container 폭으로 잘리는가 | `calc(120% + 1px)`는 240px basis에서 289px로 유지된다. layout과 parsed Root HTML E2E의 이후 item 좌표가 이를 확인한다. |
| 11 | percentage basis가 viewport나 gap을 뺀 너비로 바뀌는가 | definite Grid content width 240px를 basis로 사용한다. `calc(10% + 5px)`는 column-gap 5px와 별도로 29px가 된다. |
| 12 | 0px definite width가 indefinite로 오인되는가 | zero basis test에서 percentage 항은 0px, length 항은 5px로 계산되어 layout이 성공한다. |
| 13 | auto/math/min/max/aspect-ratio width가 basis가 되는가 | Auto 및 `calc(50% + 40px)` width, min-width, max-width, aspect-ratio 입력은 `width` 또는 해당 constraint property로 거부된다. layout layer도 non-auto width constraints를 검사한다. |
| 14 | pattern list 순환에서 calc 또는 인접 direct track 순서가 바뀌는가 | 2항 calc pattern의 implicit tracks `[29,6.8]`이 반복되고 mixed `10px calc(...) 15%` 입력의 source order와 item 시작 좌표가 보존된다. |
| 15 | calc와 기존 `fr`가 함께 있을 때 flex free space가 잘못 배분되는가 | `calc(10% + 5px) 1fr`에서 calc는 29px, 나머지 fraction track은 43.5px로 계산된다. parsed Root HTML E2E와 Taffy layout tests가 독립 좌표를 확인한다. |
| 16 | constant calc이 새 typed calc route를 잘못 통과하는가 | `calc(2px * 3)`은 Stylo computed style에서 direct `LengthPx(6)`로 정규화된다. 기존 direct-px 경로와 그 기대 결과를 별도 E2E test로 고정했다. |
| 17 | valid하지만 미지원인 math function이 허용되는가 | `min()`, product, 2항 이외 Sum 및 다른 AST는 `grid-auto-columns` projection에서 거부된다. |
| 18 | syntax-invalid inline declaration이 valid-unsupported calc와 혼동되는가 | `calc(10% + )`는 `CascadeDiagnostic`, 유효한 미지원 `min()`은 projection의 UnsupportedComputedValue로 각각 확인된다. |
| 19 | 수치 상한·track quota·aggregate extent가 overflow되는가 | 기존 CSS math cap `33,554,428px`을 그대로 적용하는 test를 추가했다. Root HTML column limit, cumulative finite-f32 extent 및 overflow failure tests도 확인했다. |
| 20 | 두 플랫폼 runtime이 다른 결과거나 화면 캡처가 pixel parity로 과장되는가 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 모두 `layout=ready`, 15 boxes와 동일한 item 시작 좌표를 기록했다. Screenshot은 visual smoke로만 분류하고 PixelCopy/Metal readback 결과로 확대하지 않는다. |

## 검토 결과

검토 중 cascade test가 이전의 “모든 Calc auto-column은 Unsupported” 기대를 남긴 것을 발견했다. 지원 범위에 맞춰 column-flow Calc 보존과 row/row-dense rejection을 구분해 고정했고, 다른 profile·auto-row gate도 유지했다. 남은 구현 결함은 발견하지 못했다.

최종 focused verification은 cascade 13/13, projection 10/10, parsed Root HTML calc E2E 6/6, layout column-flow 25/25 및 CSS math ceiling 1/1이다. Scoped Clippy, rustfmt와 diff check도 통과했다. Simulator 결과는 한 calc fixture의 layout/draw/present다. Hardware GPU, 실기기, PixelCopy/Metal pixel equality, WPT 및 전체 C11 완료를 주장하지 않는다.
