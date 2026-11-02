# C11.1 named-area + `fr` 구현·runtime 실패 관점 20개

구현 후보의 production allowlist, typed projection, parsed Root HTML runtime, Android/iOS 실행을 기존 Chrome 156 기준 및 Rust 검증 결과와 대조했다. 각 항목은 서로 다른 실패 경로를 다룬다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | `fr` 배분이 구현 상수나 Chrome 관측치에 맞춘 보정값에 의존하는가? | PASS. definite `120×70` 축의 독립 §11.7 계산은 열 `[40,80]`, 행 `[70/3,140/3]`이다. runtime frames를 이 계산에 직접 비교한다. |
| 2 | Chrome DOMRect 양자화가 이상적인 분수 결과를 실패로 만들거나 오차를 감추는가? | PASS. Chrome 관측치는 독립 계산과 분리 기록하고 독립값에 대한 허용 오차는 0.02 CSS px로 고정한다. |
| 3 | 열 track typed value가 임의 비율·count까지 허용하는가? | PASS. named-area allowlist는 columns `[Fraction(1), Fraction(2)]`만 별도로 허용한다. wrong ratio와 세 track은 실패한다. |
| 4 | 행이 열 조건을 따라오지 않아 fixed/percentage 혼합이나 무제한 분배가 되는가? | PASS. rows도 정확히 `[Fraction(1), Fraction(2)]`여야 하며 단독 row-axis fraction은 기존 area 입력에서 거부된다. |
| 5 | 열 fraction의 free-space basis가 indefinite width가 되는가? | PASS. shared fractional-axis width admission을 그대로 사용한다. max-width와 min-width 경계 변형은 scene 없이 실패한다. |
| 6 | 행 fraction의 free-space basis가 indefinite height가 되는가? | PASS. `height:auto` 변형은 완료 snapshot 없이 실패한다. |
| 7 | min/max constraint가 분배 분모를 바꾸면서도 profile을 통과하는가? | PASS. width/height min/max override 변형은 모두 fail-closed하고 completed scene을 만들지 않는다. |
| 8 | `aspect-ratio`가 definite axis의 크기를 다시 조정하는가? | PASS. aspect-ratio override는 완료 scene 없이 거부된다. |
| 9 | 기존 40/80 × 20/50 named-area baseline이 새 분수 조건 때문에 바뀌는가? | PASS. 기존 fixed-px validator branch는 그대로 유지되고 65 Root HTML 회귀군에 포함되어 통과한다. |
| 10 | 다른 template topology, area alias, spanning shape가 제한 map을 통과하는가? | PASS. 기존 validator의 정확한 2×2 `header/header`, `side/content` rectangle 요구와 Root HTML out-of-scope matrix가 유지된다. |
| 11 | 잘못된 factor가 area map의 px 경로 또는 기본 cascade fallback을 통해 성공 처리되는가? | PASS. `1fr 1fr` 및 다섯 개 이상의 track 변형은 named-area fraction path에서 거부된다. |
| 12 | track 개수가 area dimensions와 다를 때 일부 track만 조용히 사용되는가? | PASS. 각 축 2개가 아닌 명시 track은 완성 scene 없이 실패한다. |
| 13 | mixed px/fr 또는 percentage가 definite fraction처럼 오인되는가? | PASS. 양쪽 변형 모두 allowlist에서 빠지고 negative control이 `grid-template-areas` 진단을 확인한다. |
| 14 | nonzero `row-gap`이 row free-space 계산을 바꾸고도 통과하는가? | PASS. 1px row gap은 `row-gap` unsupported 진단과 no-scene을 낸다. |
| 15 | nonzero `column-gap`이 column free-space 계산을 바꾸고도 통과하는가? | PASS. 1px column gap은 `column-gap` unsupported 진단과 no-scene을 낸다. |
| 16 | `minmax()` intrinsic sizing이 direct `fr` allowlist로 우회하는가? | PASS. CSS cascade 단계에서 `cascade_failed`로 닫히며 partial/completed scene은 없다. 이 단계에 projection 진단을 강제하지 않는다. |
| 17 | semantic area placement가 selector/id나 child source order에 종속되는가? | PASS. ID와 class-like selector를 바꾸고 item DOM 순서를 바꾼 변형도 같은 semantic frames를 낸다. |
| 18 | Grid metadata와 render frame이 서로 다른 style/document revision에서 합쳐지는가? | PASS. test가 cascade/render completion key를 대조하고 typed `[1fr,2fr]` rows/columns와 2×2 areas를 확인한다. |
| 19 | rejection 또는 새 environment 뒤에 이전 named-area scene이 재사용되는가? | PASS. 각 분수 변형 거부 시험은 `completed=None`을 요구하고 stale-new-environment 회귀도 통과한다. |
| 20 | simulator screenshot을 surface-pixel parity 또는 실기기 지원으로 과장하는가? | PASS. Android AVD와 iPhone Simulator는 동일 fixture로 `layout=ready`·6-box draw/present를 보였다. 자료는 screenshot/draw 관찰로만 한정하며 PixelCopy·Metal readback 및 실기기 GPU 결과를 주장하지 않는다. |

검토에서 발견한 두 검증 문제를 정리했다. Clippy `nonminimal_bool`는 allowlist 조건을 named boolean으로 바꿔 해소했다. intrinsic `minmax()` 테스트는 cascade 거부인데 projection error를 기대하고 있어 이를 fail-closed/no-partial-scene 계약으로 낮췄다. 이후 focused runtime, projection, C11.3.3 회귀, scoped Clippy와 rustfmt가 통과했다. 기능 결함은 발견되지 않았으며 제한 사항과 첫 simulator harness mismatch는 [실행 README](README.md)에 기록했다.
