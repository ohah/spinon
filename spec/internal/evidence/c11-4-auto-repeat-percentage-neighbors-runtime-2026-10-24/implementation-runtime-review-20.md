# C11.4 percentage auto-repeat neighbors · 구현/runtime 실패 관점 20개

대상은 현재 미커밋 cascade/layout, opt-in FFI scene adapter, Android/iOS runner 및 capture boundary 변경이다. 계획 검토 20개와 PR diff 검토 30개는 별도다. 이 검토 중 발견한 aggregate overflow 결함은 회귀 테스트와 함께 수정했다. 최초 Android/iOS capture에서 드러난 runner 및 build-flag 실패도 수정한 뒤 전체 13-scene matrix로 재실행한다.

| # | 적대적 실패 관점 | 코드·근거 대조 | 판정 |
|---:|---|---|---|
| 1 | percentage가 auto-repeat fragment 내부까지 무심코 허용되는가 | Stylo는 neighbor predicate로 prefix/suffix만 분류하고 repeat fragment는 fixed-minimum gate를 따로 검사한다. [cascade](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs) | 통과 |
| 2 | definite axis가 아닌데 percentage basis가 있는 것처럼 repeat count를 계산하는가 | layout caller의 축별 `definite_axes` gate와 definite-basis negative test를 대조했다. | 통과 |
| 3 | auto min/max, indefinite dimension, 0 이하 축이 새로 지원되는가 | `explicit_positive_content_axis`가 fixed positive dimension, fixed zero minimum, automatic maximum을 요구한다. | 통과 |
| 4 | border-box percentage가 외곽 크기를 basis로 잘못 쓰는가 | helper가 border와 padding을 빼고, columns/rows 136px border-box scenes는 116px content basis의 독립 track 배열과 일치한다. | 통과 |
| 5 | negative, NaN, infinity percentage가 Taffy로 전달되는가 | layout rejection test가 두 축에서 음수·NaN·infinity를 거부한다. | 통과 |
| 6 | 유한 비율과 basis 곱셈이 `f32` infinity를 만드는가 | 두 축 test의 `f32::MAX` percentage 입력은 곱 결과가 비유한이면 fail-closed 한다. | 통과 |
| 7 | 각 used size는 유한하지만 prefix+suffix 합만 overflow 하는가 | 초기 재현은 통과해 결함이었다. percentage가 섞인 neighbor 전체를 `f64` 합산하고 `f32::MAX` 초과 시 거부하도록 고쳤다. `percentage_neighbors_with_individually_finite_sum_overflow_fail_closed` 회귀가 통과한다. | 수정 후 통과 |
| 8 | 기존 fixed-px-only neighbor의 gate가 불필요하게 바뀌는가 | percentage가 없는 경로는 기존 finite nonnegative px 검증을 그대로 사용한다. fixed-px 이웃의 기존 runtime matrix를 보존했다. | 통과 |
| 9 | columns percentage가 rows의 width basis 또는 rows가 columns의 height basis를 참조하는가 | layout test가 두 축의 basis와 13-scene scene matrix를 별도 비교한다. | 통과 |
| 10 | one-side prefix/suffix 및 양쪽 neighbor 순서가 뒤집히는가 | prefix-only, suffix-only, both scene에서 expanded track 배열과 item frame을 확인한다. | 통과 |
| 11 | auto-fill의 빈 repeat track이 auto-fit처럼 collapse 되는가 | fill scenes는 모든 repeat track `collapsed=false`; fixed expected arrays와 Android/iOS track trace가 일치한다. | 통과 |
| 12 | auto-fit에서 item이 놓인 첫 track까지 collapse 되는가 | col/row fit-first는 첫 repeat 200px, 뒤 세 repeat collapsed이며 item frame x/y 38px와 일치한다. | 통과 |
| 13 | item이 없는 auto-fit이 정상 early-return을 실패로 오인하는가 | col/row empty scenes는 `FixedTrackEarlyReturn`의 final track trace를 비교하고 네 repeat를 collapse 한다. | 통과 |
| 14 | gap과 repeat-count fit가 빠져 one-too-many/one-too-few를 내는가 | 4px gap을 고정한 양축 matrix에서 반복 수와 used track 합을 CRD 산술 및 Chrome 관측과 분리 비교했다. | 통과 |
| 15 | 양쪽 neighbor 합이 content axis를 넘을 때 반복을 0회 또는 여러 회 선택하는가 | 80%+30% overflow scene은 두 축 모두 최소 1회 반복과 `[192,40,72]` used tracks를 확인한다. | 통과 |
| 16 | 0% neighbor가 사라지거나 auto-fit collapsed track으로 잘못 처리되는가 | zero-prefix scene에서 0px non-collapsed prefix와 다섯 repeat track을 양 플랫폼에서 확인한다. | 통과 |
| 17 | 규범 200px auto-fit 결과를 Chrome의 196px 관측으로 덮어쓰는가 | normative fixture와 browser observation을 분리하고 end-alignment 진단 차이도 별도 기록한다. | 통과 |
| 18 | fixture가 다른 C11.4 scene ID를 탈취하거나 13개가 서로 달라지는가 | Rust exact catalog test와 Node boundary test가 JSON, normative cases, Chrome cases, capture selector를 ID 순서대로 대조한다. | 통과 |
| 19 | Android/iOS scene routing, expected box count, sample node/좌표가 서로 어긋나는가 | 초기 AVD에서 allowlist 거부와 empty-scene box count 오류를 발견해 두 runner를 수정했다. boundary assertion과 최종 13-scene runtime/pixel matrix가 route를 재검증한다. | 수정 후 통과 |
| 20 | 기본 앱/FFI에 fixture adapter나 JSON payload가 들어가는가 | 기본 static archive의 full-stream marker scan은 absent, opt-in fixture-test archive는 symbol/scene marker present다. 이 판단을 APK/IPA 전체 exclusion으로 확대하지 않는다. | 통과 |

## 발견 및 수정 기록

- Android run 01: scene ID가 PixelCopy preflight allowlist에 없어 capture가 거부됐다. 13개 scene ID를 allowlist에 연결했다.
- Android run 02: item 없는 scene의 expected box count 기본값 2가 root-only 실제 1과 달랐다. 두 플랫폼이 item-frame table에서 box count를 계산하도록 고쳤다.
- iOS run 01: Metal sample build flag가 꺼져 있어 pixel hook이 `status=-90`을 반환했다. feature-on 앱을 다시 빌드해 전체 matrix를 통과시켰다.
- 추가 구현 검토: percentage 항별 resolution은 finite지만 두 neighbor 합이 `f32`에서 overflow 하는 layout을 재현했다. 전체 합계 검증을 추가했고, fail-closed regression이 통과했다.
- 첫 archive 검증 스크립트는 `rg -q`가 pipe를 일찍 닫는 탓에 positive control을 오판했다. 전체 stream 소비 방식으로 부정·긍정 archive를 다시 확인했다.

최종 앱 build 및 시뮬레이터 실행은 [README](README.md)에 연결한다. 실기기, Android hardware GPU, iOS compositor 이후 pixels, 일반 CSS admission 및 전체 C11.4는 이 검토에서 완료로 판정하지 않는다.
