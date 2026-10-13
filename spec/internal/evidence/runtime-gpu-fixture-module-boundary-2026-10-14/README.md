# Runtime GPU 픽스처 경계 산출물 재검증 · 2026-10-14

## 실행 조건

- 후보 source: `origin/main` 기반의 runtime GPU fixture module/build-boundary 변경. C11.3.1 feature source는 이 후보에 포함하지 않는다.
- 대상: macOS arm64 `spinon-ffi` Debug static library. 최종 Android APK, iOS app, Release/LTO 산출물의 용량·stripping 검증이 아니다.
- 심볼 도구: Rust `1.96.1` toolchain의 matching `llvm-nm` (`llvm-tools-preview`). Xcode `nm`은 LLVM bitcode attribute를 읽지 못했으므로 결과 근거로 사용하지 않았다.

## 산출물

| 조합 | 명령 | SHA-256 | 크기 | 검사 |
| --- | --- | --- | ---: | --- |
| 제품 GPU core | `cargo build --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu` | `53fe3ae79b40dd318f44742cb4dea1720427d4f233fa82bd1760a4bb49dded98` | 516,075,912 bytes | `llvm-nm`에서 C11.2.3 fixture/UA probe FFI 심볼 없음. `strings -a`에서 `grid-auto-placement-runtime-matrix`, `C11.2.3 scene`, `runtime-css-to-gpu-resize` 없음. |
| 앱 fixture positive control | `cargo build --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu-fixture-test,c11-2-3-grid-fixture-test,c04-runtime-ua-cascade-fixture-test` | `ad0bc35ef9fd044ca119e9d3b3831710d484da294a4b83dfc71f0e77b14dcf26` | 523,340,160 bytes | `llvm-nm`에서 `spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture`, `spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture`, `spinon_runtime_ua_cascade_probe` 존재. `strings -a`에서 `grid-auto-placement-runtime-matrix`와 C11.2.3 고정 장면 payload 존재. |

두 artifact는 같은 호스트의 Debug 정적 라이브러리다. 크기 차이는 app 설치 용량이나 모바일 release 증가량으로 해석하지 않는다. Positive control은 feature gating이 실제로 요청된 fixture ABI/payload를 켜는지 확인한다.

## 범위

이 비교는 기본 GPU core와 opt-in fixture feature 사이의 선택 ABI·고정 payload 경계만 검증한다. Android/iOS 최종 앱 stripping, Release/LTO, store bundle, 전체 workspace 회귀는 확인하지 않았다.
