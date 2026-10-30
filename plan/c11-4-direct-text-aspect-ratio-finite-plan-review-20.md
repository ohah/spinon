# C11.4 direct-text finite-positive ratio 계획 실패 관점 검토 20개

**대상:** [finite-positive ratio 확장 계획](./c11-4-direct-text-aspect-ratio-finite.md)과 [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-range-precomparison-2026-10-30/README.md)
**기준:** `main` / `origin/main` `3fb170450d82614d35c772845fa68c1b880505d0`
**판정:** 20/20 PASS, 구현 전 신규 계획 범위에 대한 검토. 구현 코드를 평가하거나 C11.4 완료를 주장하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | CSS ratio의 numerator/denominator 방향을 반대로 적용 | PASS — `width / height`로 명시하고 row/column 양방향 계산을 둔다. |
| 2 | preferred ratio를 replaced-item §6.6 shortcut에 잘못 연결 | PASS — §6.2/§11.5 ordinary non-replaced sizing으로 제한한다. |
| 3 | Ahem font가 준비되기 전에 expected width를 계산 | PASS — font face loaded와 check 결과를 사전 근거에 기록한다. |
| 4 | platform font fallback이 결과에 섞임 | PASS — 저장소 Ahem font만 profile 입력으로 허용한다. |
| 5 | `normal` alignment를 stretch로 가정해 ratio를 무시 | PASS — preferred ratio가 있는 `normal` auto-size를 별도 경로로 다룬다. |
| 6 | ratio 없는 control도 ratio 경로에서 왜곡 | PASS — row 및 각 definite row 높이별 no-ratio controls가 있다. |
| 7 | 단일 2/1 regression만 보존하고 다른 값에서 새 회귀 발생 | PASS — 기존 four scene regression과 네 신규 ratio를 요구한다. |
| 8 | 소수 ratio의 browser quantization을 정확한 decimal로 오판 | PASS — 1/64 CSS px used-value quantization을 기록한다. |
| 9 | max-content columns를 preferred ratio size로 덮어씀 | PASS — `column-1-2`에서 40px text track과 30px ratio preference를 구분한다. |
| 10 | ratio box의 row overflow를 track size와 합쳐 은폐 | PASS — `40×80` item과 `100×60` root를 분리하고 20px overflow를 검사한다. |
| 11 | `NaN`, infinity, zero 또는 degenerate ratio가 typed `None` 경로로 새어 no-ratio control이 되거나 finite input이 layout dimension overflow를 만듦 | PASS — computed serialization과 typed snapshot을 함께 검사하고 degenerate numeric serialization을 거부한다. `f32::from_bits(1)`의 `40px / ratio`와 `f32::MAX`의 `60px × ratio`는 Taffy 전에 거부하며 failure atomicity를 확인한다. |
| 12 | `auto <ratio>`가 숫자 ratio로 잘못 허용 | PASS — 기존 Stylo rejection을 유지하는 게이트와 adversarial case를 명시한다. |
| 13 | invalid CSS declaration의 computed `auto`를 잘못된 numeric input으로 단정 | PASS — computed value와 invalid source declaration을 구분한다. |
| 14 | scene 이름이나 author-source text에서 ratio를 추측 | PASS — 동일 revision의 Stylo typed snapshot만 ratio 출처로 쓴다. |
| 15 | ratio를 가진 두 item이 허용돼 검증 행렬을 벗어남 | PASS — 최대 한 ratio owner와 두-ratio rejection을 고정한다. |
| 16 | ratio를 short `A` item에 배치해 기여 추적이 모호 | PASS — short item ratio를 rejection에 포함한다. |
| 17 | 새 profile adapter가 기존 direct-text/legacy fallback을 대체 | PASS — 기존 generic 및 one-item fallback 회귀를 계속 요구한다. |
| 18 | static fixture FFI가 product artifact로 유출 | PASS — default와 fixture positive-control archive 대조를 implementation gate로 둔다. |
| 19 | simulator evidence를 실기기 GPU 또는 compositor-after 픽셀로 확대 | PASS — AVD/Metal readback의 한계를 별도로 명시한다. |
| 20 | 제한 fixture가 C11.4/C11 완료 또는 WPT pass로 게시 | PASS — 계약·상태·로드맵의 unchecked 및 `72/213` 유지가 계획의 명시적 gate다. |

## 계획 검토 후 조정

계획에 browser 관측보다 일반화 범위가 넓다는 점을 명시했다. 코드 계약은 `finite && > 0`인 typed ratio 중 derived geometry를 표현할 수 있는 값을 허용하지만, 정량 Chrome matrix는 `1/2`, `4/3`, `3/2`, `5/2` 네 값만 직접 관찰한다. 이는 모든 CSS/폰트/비율에 대한 상호 운용 검증으로 해석하지 않는다.

## 검토 후 보완

검토 중 `computed_layout_aspect_ratio`가 CSS Sizing Level 4에 따라 degenerate ratio를 오류가 아닌 `None`으로 투영하는 것을 Stylo 0.22 소스에서 확인했다. 이를 “degenerate 입력 rejection”으로 넓혀 말하지 않도록 계획을 고쳤다. Fixture validator는 `None`만 보고 no-ratio로 분류하지 않고 computed `aspect-ratio` 문자열이 `auto`인지 확인하며, degenerate numeric serialization은 거부한다. `f32::from_bits(1)`의 `40px / ratio`와 `f32::MAX`의 `60px × ratio`가 derived dimension overflow를 만들므로 Taffy 호출 전 fail-closed 기준을 추가했다. 영향받은 검토 항목 #11을 다시 확인했다.
