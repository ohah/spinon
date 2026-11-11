# C11.4 repeated-text five-fit row-span 실행 근거

## RuntimeSession Chrome 157 비교

- 사전 비교 원본은 [Chrome 157 독립 비교](../c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/README.md)와 같은 고정 scene 집합이다.
- `parsed_root_html_five_fit_row_span_matrix_chunk_1..4_matches_chrome`가 4/4 통과했다. 각 청크 540 track list, 총 2,160 track list와 34,560 scene을 비교했다.
- sizing/distribution 모듈 분리 후 최종 소스에서 네 청크를 다시 실행했다. 4/4, 34,560/34,560 통과, Rust test runner 완료 시간 549.79초다.
- 네 청크 모두 candidate의 text payload-floor 위반은 0건이었다. 대응 `min-height:0` baseline 위반은 청크별 3,152, 4,512, 2,784, 2,800건이다. baseline은 제품 후보의 기대값이 아니라 독립 관측 비교군이다.
- 별도 회귀 검증: five-fit track trace 7/7, atomic rejection 2/2, layout `blink_` 단위 검사 11/11, one-fit trace 1/1, three-fit trace 1/1, four-fit trace 5/5.
- 최종 five-fit fixture-test filter는 matrix 밖 regression 15/15를 통과했다. 이 필터에는 five-fit auto-placement control·trace·rejection과 row-span trace·atomic rejection이 포함된다.
- Layout admission cap/tail 검사는 2/2, style projection의 독립 cap/tail 검사는 2/2 통과했다. `cargo fmt --all -- --check`, `git diff --check`, layout/runtime affected-crate Clippy도 통과했다.
- 세 개의 `max-content` singleton tail track-list 경계도 회귀 테스트로 고정하고 통과시켰다. 이 수정은 tail sizing trace에서 0 증가가 발생해도 Taffy가 무한 growth limit을 현재 base size로 확정하는 단계를 반영한다.
- 구현/runtime 실패 관점 20개 검토는 [별도 문서](implementation-runtime-review-20.md)에 기록했다.

## Android/iOS 기본 Root HTML 앱 smoke

이 단계는 Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`와 iPhone 17 Pro / iOS 26.2 Simulator에서 **기본 앱 route**를 실행한 결과다. 앱 로그에서 root HTML boot `pass`, `layout=ready`, `SPINON_C0410_DRAW ... presented boxes=3`를 확인했다.

| 플랫폼 | light | dark | 화면·로그 |
|---|---|---|---|
| Android API 37 ARM64 AVD | [화면](android-light.png) · [로그](android-light.log) | [화면](android-dark.png) · [로그](android-dark.log) | AVD는 SwiftShader software renderer를 보고했다. |
| iOS 26.2 iPhone 17 Pro Simulator | [화면](ios-light.png) · [로그](ios-light.log) | [화면](ios-dark.png) · [로그](ios-dark.log) | `simctl` 앱 로그와 화면 캡처를 보존했다. |

light/dark 관측은 runtime environment의 `dark=false|true` 전환을 확인한다. 기본 route의 고정 초록 배경은 두 appearance에서 동일하다.

## 범위와 한계

- 앱 route에는 five-fit row-span fixture가 실리지 않는다. 따라서 이 플랫폼 smoke는 34,560-scene C11.4 feature matrix를 대신하지 않는다. 그 matrix는 위의 `RuntimeSession` Chrome 비교가 담당한다.
- 캡처는 화면 확인 자료다. Android PixelCopy, iOS Metal drawable readback, compositor 이후 pixel 비교, 실기기/hardware GPU 검증은 하지 않았다.
- `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`와 runtime `--lib` Clippy는 통과했다. runtime `--all-targets`는 기존의 `anonymous_text_hyphens.rs`·`anonymous_text_text_transform.rs`의 `too_many_arguments` 두 건과 변경하지 않은 `five_fit/matrix.rs`의 `needless_borrow` 세 건으로 실패했다.
- 공식 C11.4/C11 완료 상태와 `78/214`는 변경하지 않는다.
