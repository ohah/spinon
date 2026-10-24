# C11.4 signed percentage auto-repeat `calc()` 구현/runtime 실패 관점 20개

계획 검토와 분리해 최종 구현, fixture 경로, Android/iOS 결과를 서로 다른 실패 경로로 확인했다. 최종 11-scene matrix는 양 플랫폼에서 통과했고, 새로 남은 결함은 찾지 못했다. 사전 oracle 검증에서 발견한 auto-fit trailing-gutter 계산 오류는 capture verifier에서 수정한 뒤 Chrome·Android·iOS 결과로 다시 확인했다.

| # | 실패 관점 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | authored `calc()` 문자열만으로 지원 여부를 넓혀 Stylo가 만든 typed AST 차이를 놓치는가 | cascade test가 `calc(20px - 5%)`, `calc(5% - 20px)`, `calc(-2px - 1%)`의 실제 computed `Sum`과 부호를 검사한다. |
| 2 | 두 항의 순서가 바뀌면 length/percentage를 잘못 분류하는가 | projection/layout은 양쪽 순서를 모두 분류하며 reverse-order case는 단위 test와 runtime positive case로 고정했다. |
| 3 | 직접 음수 percentage 항이 여전히 거부되는가 | finite signed `Percentage`를 허용하고 신규 positive-final 및 negative-final 장면에서 사용값을 확인했다. |
| 4 | 한 겹의 `Negate(Percentage)`가 부호를 뒤집지 않거나 중첩 negate까지 허용하는가 | 한 겹만 부호를 반전한다. nested negate는 projection/layout rejection test에서 거부된다. |
| 5 | 음수 length와 음수 percentage가 함께 있으면 부분 합산 또는 잘못된 track이 생기는가 | `negative-percentage-col-both-negative-final`은 최종 이웃값 0과 runtime track/frame 결과를 확인한다. |
| 6 | 중간 항이 음수라는 이유로 전체 계산을 조기 clamp하는가 | `calc(20px - 5%)` 및 `calc(5% - 20px)`의 positive final case가 length와 percentage를 합산한 뒤 used size를 확인한다. |
| 7 | 최종 음수의 CSS property range clamp가 빠지는가 | column prefix와 row suffix의 negative-final case가 0px 결과를 layout unit test와 앱 runtime에서 확인한다. |
| 8 | non-finite signed term이 finite 검사 뒤 레이아웃에 도달하는가 | `NaN` length와 infinite percentage를 projection에서 거부하고 layout의 non-finite/overflow test도 통과했다. |
| 9 | finite term 조합이 `f32` 표현 한도를 넘는데 성공하는가 | calc intermediate 및 이웃 aggregate의 representability 검사와 기존 overflow regression을 확인했다. |
| 10 | extra term 또는 nested product/min/max/clamp가 제한 profile을 우회하는가 | 정확히 두 항만 받고, extra term·product·중첩 수학식은 CSS property/node가 붙은 fail-closed 경로에 남는다. |
| 11 | 누락·중복 calc ID가 임의 expression을 선택하는가 | layout validator는 정확히 하나의 ID match를 요구하며 missing/duplicate binding 회귀가 통과한다. |
| 12 | 다른 node 또는 CSS property 소유 calc가 재사용되는가 | owner node와 columns/rows property를 각각 대조한다. wrong-node/property 회귀가 통과한다. |
| 13 | percentage가 border-box 외곽 크기나 잘못된 축을 기준으로 계산되는가 | column은 inner inline size, row는 inner block size를 쓴다. border-box 장면과 column/row 양축 matrix가 Chrome 및 독립 기대값과 1/64 CSS px 이내다. |
| 14 | indefinite/cyclic basis를 0 또는 외곽 크기로 임의 대체하는가 | definite positive basis가 없으면 거부하는 기존 guard와 해당 layout regression을 확인했다. |
| 15 | prefix와 suffix 이웃 합산에서 순서 또는 aggregate overflow 검사를 놓치는가 | 양쪽 neighbor 순회와 finite/`f32` aggregate 상한은 기존 구현을 유지하며 layout invalid-shape/overflow test가 통과한다. |
| 16 | 기존 fixed/positive percentage neighbor와 auto-repeat count가 바뀌는가 | 기존 `col-fit-first`, `calc-neighbor-col-fit-first` 회귀 control이 두 플랫폼에서 통과했다. repeat track와 gutter는 이전 경로를 그대로 사용한다. |
| 17 | auto-fit occupied item 또는 collapsed track 주변 gutter가 어긋나는가 | occupied auto-fit scene과 Chrome geometry 검증을 추가했다. verifier가 trailing collapsed run의 gutter를 잘못 보존하던 결함을 수정했고 최종 oracle·simulator matrix에서 확인했다. |
| 18 | 새 fixture ID가 Android/iOS allowlist나 dispatcher에서 누락 또는 오타 나는가 | catalog exact 9-ID 검사와 플랫폼별 allowlist/runtime route 검사가 통과했고 11/11씩 장면이 draw/present까지 도달했다. |
| 19 | product build에 fixture 문자열·symbol이 포함되거나 test build에 fixture가 빠지는가 | 기본 `c04-runtime-gpu` archive는 negative control, `c04-runtime-gpu-fixture-test` archive는 9/9 marker 및 symbol positive control로 검사했다. |
| 20 | frame/trace는 성공하지만 표면에 item paint가 없거나 플랫폼 픽셀이 다른가 | Android PixelCopy와 iOS 제출 전 Metal drawable은 각각 11개 scene·22/22 interior 표본이 exact다. screenshot·표본은 전체 border stroke나 compositor 이후 surface 증거로 확대하지 않는다. |

## 근거

- [최종 runtime matrix와 screenshots](README.md)
- [Chrome 156 사전 비교 및 독립 기대값](../c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md)
- [PR diff 실패 관점 30개](pr-diff-review-30.md)

## 남은 경계

실기기, Android hardware GPU, iOS compositor 이후 pixel, WPT, 일반 author CSS admission, 최종 APK/IPA의 fixture exclusion과 전체 C11.4는 검증하지 않았다. 이 결과는 그 범위의 완료 판정에 포함하지 않는다.
