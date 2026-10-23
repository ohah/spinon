# C11.4 positive percentage auto-repeat implementation/runtime 실패 관점 20개

이 검토는 수정된 계획 검토와 분리해 cascade, layout validator, fixture adapter, 양 플랫폼 실행 결과를 대상으로 했다. 각 항목은 서로 다른 입력 거부·축 매핑·계산·증거 경계를 확인한다.

| # | 실패 관점 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | `calc(0px + 10%)`가 authored Calc로 남는다고 가정해 실제 computed style을 잘못 제한하는가 | Stylo 0.22는 `Percentage(0.1)`로 normalize한다. [cascade normalization probe](../c11-4-auto-repeat-positive-percent-minimum-precomparison-2026-10-23/stylo-cascade-normalization.log)와 cascade 회귀 12개가 direct percentage와 같은 typed 값을 고정한다. |
| 2 | percentage 허용이 opt-in track-sizing profile 외 computed-style profile에 노출되는가 | 변경은 C11.4 track-sizing cascade validator 내부에 있고 기본 빌드에서 새 scene marker가 없다. fixture boundary 19개와 [archive marker scan](./builds/product-archive-marker-scan.log)을 확인했다. |
| 3 | bare percentage track이 `minmax()`의 fixed-size minimum으로 잘못 취급되는가 | `repeat(auto-fill,10%)` cascade에서 fail-closed한다. cascade의 negative case와 FFI route test가 회귀를 고정한다. |
| 4 | 0% minimum을 양수 percentage와 같이 허용하는가 | 0%와 `minmax(0%, 1fr)`가 computed-profile 검사에서 거부되고, layout API 직접 입력도 Taffy 전에 `InvalidGridTrackList`를 반환한다. |
| 5 | percentage 최대값 또는 intrinsic maximum까지 새 범위에 포함되는가 | 새 cascade arm의 최대값은 finite `LengthPx` 또는 `Fraction`만 허용한다. `auto`, `max-content`, percentage maximum은 테스트에서 거부한다. |
| 6 | 음수·NaN·무한 percentage가 layout track 계산으로 넘어가는가 | `p`의 finite 및 `p > 0` 조건과 `p × content_basis`의 finite·positive 검사가 Taffy 전달 전에 실패시킨다. |
| 7 | columns repeat이 height를, rows repeat이 width를 basis로 삼는가 | 축 property별 dimension, padding, border를 선택한다. 16개 fixture 및 양 플랫폼 trace가 각 축 결과를 확인한다. |
| 8 | ContentBox에서 padding/border를 빼 basis를 과소 계산하는가 | ContentBox는 지정 fixed content dimension을 그대로 사용한다. ContentBox/BorderBox 모두 10% 결과를 fixed 12px control과 비교하는 layout 회귀가 있다. |
| 9 | BorderBox가 outer dimension 전체를 percentage basis로 잘못 사용하는가 | BorderBox는 해당 축 padding과 border를 뺀 content dimension을 사용한다. 124px border-box의 내부 기준이 120px임을 layout tests에서 확인한다. |
| 10 | min-size auto 또는 제한된 max-size가 고정 definite axis로 가장되는가 | minimum은 정확히 `Fixed(0)`이어야 하며 maximum은 `Auto`(내부 계약의 none)여야 한다. auto minimum과 fixed max-size 입력이 fail-closed한다. |
| 11 | 0 또는 indefinite axis가 auto-repeat 계산에서 0으로 나뉘거나 반복 상한을 만드는가 | zero, auto/indefinite, cyclic-percentage 및 intrinsic max-content dimensions가 Taffy 전에 거부된다. |
| 12 | BorderBox padding이 음수·비유한 값이면 잘못된 positive basis가 되는가 | 일반 layout input validation이 모든 padding과 border edge의 음수·비유한 값을 먼저 거부한다. 새 basis 계산은 그 validated input을 소비한다. |
| 13 | border/padding 뒤 content basis가 0 이하인데 percentage branch가 허용되는가 | 124px border-box 안에 60px씩의 양쪽 padding을 넣은 경우를 0 이하 content test로 거부한다. |
| 14 | percentage 곱셈 overflow가 유한한 fixed track으로 전달되는가 | `f32::MAX × basis`를 테스트하고 `InvalidGridTrackList`로 차단한다. |
| 15 | 10% auto-fill 반복 개수나 track base가 fixed control과 다른가 | 양 축에서 10개 track 각 12px이며 fixed 12px control과 item frame이 같다. Android/iOS 최종 결과 12개가 각각 통과했다. |
| 16 | auto-fit이 빈 track을 남기거나 채운 track까지 collapse하는가 | 양 축에서 첫 120px occupied track만 남고 나머지 9개가 collapse된다. 6개 auto-fit 장면을 두 플랫폼에서 확인했다. |
| 17 | `max=12px`와 `max=1fr` 지원을 섞거나 fixed 최대값을 놓치는가 | `minmax(10%,12px)` 전용 layout/cascade case가 10개 반복 및 12px frame을 확인한다. flex maximum은 나머지 10% matrix에 사용한다. |
| 18 | 150% frame이 계산은 맞아도 실제 border 밖으로 나오지 않거나 고정 대조군과 다른가 | 양 축에서 180px track/item, 124px border-box 밖 58px overflow를 layout test, runtime JSON, 화면 캡처로 확인했다. |
| 19 | fixture scene ID·platform allowlist·runtime source 사이 routing 누락이 성공처럼 보이는가 | JSON catalog 16개, Android/iOS allowlist, FFI exact dispatch, unknown ID 거부, capture frame/track assertion을 확인했다. 최종 simulator 결과는 양 플랫폼 16/16 `drawPresented=true`다. |
| 20 | 최종 simulator가 Clippy 수정 전 archive를 실행하거나 fixture가 제품 archive로 새는가 | 최종 Android/iOS simulator metadata가 동일 tree와 tracked diff SHA를 기록한다. workspace Clippy 통과 후 재빌드·재캡처했고 default archive marker는 absent, fixture-test archive marker는 present다. 전체 APK/IPA 제외와 Chrome pixel parity는 주장하지 않는다. |

**검토 결과:** 발견한 남은 구현/runtime 결함은 없다. 첫 Android 시도에서 빠진 native expected-frame mapping과 Clippy의 함수 인자 lint는 수정 후 최종 build 및 두 simulator matrix에서 다시 확인했다. 미검증 범위는 README에 명시했다.
