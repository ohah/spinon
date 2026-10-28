# C11.4 generic mixed text/image spanning runtime 근거

## 범위와 판정 기준

기존 네 mixed text/image multi-span fixture를 그대로 두고 HTML root/item `id`가 새 `generic-host-*` 값이어도 제한된 C11.4 runtime adapter가 처리하도록 했다. authored CSS, launcher scene ID, sizing algorithm, 일반 CSS admission은 바꾸지 않았다. 사전 비교는 [계획](../../../../plan/c11-4-generic-mixed-spanning-runtime.md)과 [기존 Chrome 155 관측·독립 기대값](../c11-4-mixed-text-image-multispan-precomparison-2026-10-21/README.md)을 재사용한다. Chrome 관측은 규범 oracle이 아니며 별도 기대 track은 CSS Grid §11.5.1 계산에서 온다.

- span 2: columns `[25, 36, 36, 0]px`
- span 3 포함: columns `[158/3, 191/3, 191/3, 0]px`
- 모든 scene: rows `[60, 60, 60]px`, root `220×180px`
- rational/f32 frame 비교 허용오차: `1/64 CSS px`

계획 실패 관점 검토는 [계획 검토 20개](../../../../plan/c11-4-generic-mixed-spanning-runtime-plan-review-20.md), 구현·runtime 검토는 [별도 20개](./implementation-runtime-review-20.md)다. 실제 변경분은 [PR diff 검토 20개](./pr-diff-review-20.md)에 따로 기록한다.

## Rust 검사

| 검사 | 결과 |
|---|---|
| `cargo test --locked -p spinon-runtime generic_spanning --features c11-4-grid-track-sizing-fixture-test` | 6 passed |
| `cargo test --locked -p spinon-ffi exposes_only_four_mixed_text_image_multispan_scenes --features c04-runtime-gpu-fixture-test` | 1 passed |
| `cargo fmt --all -- --check` | 통과 |
| feature-enabled strict Clippy | 첫 실행에서 이번 코드의 `redundant_guards`, `map_identity`를 발견해 수정. 남은 실패는 변경 밖의 `c11_4_grid_layout_auto_tracks/validation.rs:329` `type_complexity`, `generic_direct_text_image/fixture.rs:311` `map_identity` |
| 변경 crate·feature-enabled Clippy, 위 두 기존 lint만 허용 | 통과 |

## 시뮬레이터 실행

실행 명령과 캡처 재생산 절차는 [`run-simulator-matrix.sh`](./run-simulator-matrix.sh), 각 scene의 trace·로그·PNG·JSON은 `runs/` 아래에 보존했다. 최종 캡처는 마지막 코드 정리 뒤 다시 빌드한 결과이며 소스 파일 SHA-256과 앱 artifact SHA-256이 플랫폼별 JSON에 있다.

| 플랫폼 | 환경 | 장면 | 결과 |
|---|---|---:|---|
| Android | API 37 ARM64, 16 KB AVD `spinon_api37_1_compat16k` | 4/4 | 기대 columns/rows, root·item frame, trace, draw/present 통과 |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator | 4/4 | 기대 columns/rows, root·item frame, trace, draw/present 통과 |

Android 최종 run: `c114-generic-mixed-spanning-android-avd-20261028T000520532Z-4f50cc1d`. iOS 최종 run: `c114-generic-mixed-spanning-ios-simulator-20261028T000542768Z-d38bbe71`. 두 run 모두 scene 순서를 바꾼 네 입력을 별도로 처리했다. 스크린샷은 span-2 forward 및 span-2/span-3 교차 scene만 캡처했다. 앱 화면의 색상 사각형은 fixture의 배치 표식이므로 glyph, PNG texel, 하드웨어 GPU 또는 iOS compositor 이후 pixel 검증으로 해석하지 않는다.

초기 full capture에서는 native `52.666664`와 JavaScript rational `52.666666…`를 exact-equality로 비교해 캡처 검증기가 실패했다. 검증기를 계약상 `1/64 CSS px` 허용오차로 고친 뒤 전체 matrix를 다시 실행했고, 초기 실패 run도 보존했다. 이는 런타임 track 실패가 아니라 검증기 비교 방식의 결함이었다.

## Fixture artifact 경계

[`verify-archive-boundary.sh`](./verify-archive-boundary.sh)는 Rust static archive를 세 번 빌드해 scene ID 네 개와 `generic-host-root-` marker를 검사한다.

- 기본 `spinon-ffi`: marker 5개 모두 없음
- 제품 `c04-runtime-gpu`: marker 5개 모두 없음
- opt-in `c04-runtime-gpu-fixture-test`: marker 5개 모두 있음

해시와 개별 probe 결과는 [`archive-boundary/`](./archive-boundary/)에 있다. 이 결과는 정적 라이브러리 범위이며 최종 APK·IPA 전체에서 fixture가 제외됨을 증명하지 않는다.

## 상태와 미검증 범위

이 slice는 `spec/STATUS.md`의 C11.4 상위 항목을 완료 처리하지 않는다. 공식 완료 집계 `72/213`도 유지한다. PR CI 결과, 실기기, Android hardware GPU, 브라우저 재측정, arbitrary HTML/CSS, WPT 및 앱 번들의 fixture 전체 제외는 이 근거에 포함하지 않는다.
