# C11.1 Root HTML Grid minmax runtime evidence

**상태:** 구현 중인 후보 코드의 runtime 증거. C11.1 또는 C11 공식 완료 증거가 아니다.

**비교 원본:** [사전 비교 README](../c11-root-html-grid-minmax-precomparison-2026-11-03/README.md), [독립 expected 값](../c11-root-html-grid-minmax-precomparison-2026-11-03/expected.json), [런타임 fixture](../../../../tests/fixtures/css/c11/root-html-grid-minmax-runtime.html)

고정 fixture SHA-256: `92e346d6e06e6017bd22db1063e7fb4968ab60ebf72ba93fa807c457bf54052e`.

후보 소스는 base HEAD `773cfe232` 위의 아직 커밋되지 않은 변경이다. 아래의 simulator 빌드는 feature-enabled test configuration을 사용했다. fixture HTML과 fixture 전용 작은 root-app script를 앱 asset으로 임시 넣어 실행한 뒤, `examples/bootstrap/index.html`, `examples/bootstrap/root-app.js`, `build/spinon/bootstrap` 원본을 복구했다.

## Android

- **환경:** API 37 ARM64 AVD `emulator-5556`; renderer는 ANGLE / SwiftShader 소프트웨어 경로이며 실제 기기 GPU 증거가 아니다.
- **빌드:** [build log](android/build-candidate-run08.log), `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`.
- **run08:** Root HTML 기본 경로, layout `ready`, 5 boxes, `draw status=0`; [runtime log](android/runtime-run08.log), [PixelCopy PNG](android/pixelcopy-run08.png), [화면 PNG](android/device-screenshot-run08.png), [표본·해시](android/pixel-samples-run08.json).
- **run09:** 같은 후보 APK로 연속 재실행해 PixelCopy 표본 2/2가 다시 일치했다. [runtime log](android/runtime-run09.log), [PixelCopy PNG](android/pixelcopy-run09.png), [표본·해시](android/pixel-samples-run09.json).
- 두 PixelCopy run 모두 grid 내부 `(77,27)`에서 `(16,24,32,255)`, root 오른쪽 바깥 marker `(116,11)`에서 `(59,130,246,255)`를 읽었다. PNG SHA-256은 두 run 모두 `5e580452a61b83526a37fcf67fef7347caf5185e94669230e467e685121ca4be`.
- run08/run09는 현재 feature-enabled APK의 연속 실행이며, 이 두 run만 이번 PR candidate의 positive PixelCopy 증거로 사용한다. 앞선 fixture 준비·capture 시도는 성공 증거와 구분해 작업 체크아웃에 보존했다.

## iOS

- **환경:** iPhone 17 Pro Simulator, iOS 26.2, simulator UUID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`; Metal drawable `1206×2334`, CSS viewport `402×778`.
- **빌드:** feature-enabled [build log](ios/build-candidate-run03.log); C04.10 runtime GPU, C04 fixture-test, C11 Metal surface pixel test 활성화.
- **run03:** Root HTML 기본 경로에서 layout `ready`, 5 boxes, `draw status=0`; in-process Metal drawable readback이 grid 내부 `(88,31)`의 `(16,24,32,255)`와 overflow marker `(133,13)`의 `(59,130,246,255)`를 각각 exact-match했다. [runtime log](ios/runtime-run03.log), [화면 PNG](ios/device-screenshot-run03.png), [log 수집 진단](ios/log-query-stderr-run03.txt).
- 화면 PNG 크기 `1206×2622`, SHA-256 `8d84cc1f167b69482e24c5e72dd80b931c5b18a4bf42326f13fd8a8d7c17419e`.
- 이 결과는 Simulator의 Metal drawable 표본이다. 실제 iPhone 하드웨어 검증이나 compositor 이후 화면의 독립 캡처라고 주장하지 않는다.

## Rust 및 빌드 검사

- `cargo test --locked -p spinon-runtime c11_1_root_html_`: 42 passed.
- `cargo test --locked -p spinon-style-to-layout root_html_profile_projects_bounded_minmax_explicit_tracks`: 1 passed.
- `cargo test --locked -p spinon-style-to-layout root_html_profile_rejects_invalid_minmax_breadths`: 1 passed.
- `cargo test --locked -p spinon-style root_html_negative_fraction_track_recovery_ignores_only_simple_invalid_track_values`: 1 passed.
- `cargo fmt --all -- --check`, `git diff --check`, 변경 크레이트 전체 대상 `cargo clippy ... -- -D warnings`: 통과.
- Android feature-on 전체 debug APK assemble과 feature-off/on Java compile을 통과했다. iOS feature-enabled Simulator build도 성공했다.
- 명령 출력은 [rust evidence logs](rust/)에 고정했다.

## 검토 결과와 남은 단계

현재 후보 구현/runtime 변경을 20개 독립 실패 관점으로, PR diff를 별도의 30개 patch 관점으로 검토했다. 두 검토 모두 현재 diff에서 미해결 결함을 찾지 못했다. GitHub PR 생성, CI 결과 확인, 병합, 그리고 병합 SHA 기준 문서·로드맵 동기화는 남아 있다. 이 candidate는 `spec/STATUS.md` 체크박스를 바꾸지 않으며 C11/C11.1은 미완료 상태다. 현재 Tailnet 표시 수와 원본 대장 간 차이는 계획 문서에 기록했다.
