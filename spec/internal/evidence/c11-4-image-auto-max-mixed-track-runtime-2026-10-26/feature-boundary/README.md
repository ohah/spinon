# C11.4 image fixture artifact 경계

PR #643의 source revision `e4570a04f1f8d0595836e3f35d7fa6a88b7c8b83`에서 macOS ARM64 `spinon-ffi` release static archive 두 조합을 빌드했다. 상세 명령·도구 버전·archive SHA-256·marker 판정은 [artifact-boundary.json](../artifact-boundary/artifact-boundary.json), 재현 scanner는 [scan-image-auto-max-ffi-archives.py](../artifact-boundary/scan-image-auto-max-ffi-archives.py), build 원문과 `strings`/`llvm-nm` 출력은 [성공한 run](../artifact-boundary/runs/artifact-boundary-20261026T005842Z-4b4b1ea0/)에 있다.

| 산출물 | Feature | 크기 | SHA-256 | 신규 scene/source marker | C11.4 fixture ABI 정의 symbol | 경계 결과 |
| --- | --- | ---: | --- | ---: | ---: | --- |
| product archive | `c04-runtime-gpu` | 75,135,352 bytes | `0411b4a4d8005265612acded45c41b14f7aa9fcc6ceb3df9b9df2d16485a4abc` | 0/5 | 0/2 | negative control 통과 |
| opt-in fixture archive | `c04-runtime-gpu-fixture-test` | 78,492,488 bytes | `c90612c1785fb6299a23c0a26416eacdeeff607b26465520f9438e2aaf7025da` | 5/5 | 2/2 | positive control 통과 |

다섯 marker는 `image-auto-max-mixed-tracks`, root/item ID, `minmax(auto, max-content) 17px`, 그리고 기존 PNG URL이다. product archive에서는 이 scene/source 문자열, C11.4 fixture ABI 문자열 및 정의 symbol이 모두 없다. opt-in archive의 `strings` 결과에는 다섯 marker가 모두 있고, `llvm-nm --demangle --defined-only`는 다음 두 정의를 확인했다.

- `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`
- `_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`

97-byte `c14-1-png-basic.png`는 product archive에는 없고 opt-in archive에는 있다. 다만 이는 새 scene이 단독으로 추가한 byte payload가 아니다. 기존 opt-in [`anonymous_grid_border_overflow_paint`](../../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_grid_border_overflow_paint.rs#L11) fixture도 같은 asset을 `include_bytes!`로 사용하므로, PNG byte 존재 여부는 shared fixture asset의 경계를 확인할 뿐 새 scene에만 귀속하지 않는다. 이번 변경에서 새로 gated된 payload는 scene/source 문자열이다.

재현 명령은 저장소 루트에서 다음과 같다.

```sh
python3 spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/artifact-boundary/scan-image-auto-max-ffi-archives.py
```

첫 scanner 시도는 product archive를 만든 뒤 macOS `strings` 출력의 UTF-8 디코딩 단계에서 종료됐다. binary-safe streaming으로 scanner를 보완해 별도의 성공 run을 만들었고, 첫 시도 원문 build log도 보존했다: [이전 시도 로그](../artifact-boundary/runs/artifact-boundary-20261026T005705Z-de756aca/product-build.log). 이는 compiler나 feature-boundary 판정 실패가 아니다.

이 검사는 Rust `spinon-ffi` release static archive의 feature 경계만 확인한다. 최종 Android APK/iOS app bundle 전체의 fixture exclusion, simulator 화면, image paint/pixel equality를 증명하지 않는다. 이 확인 과정에서 앱, 시뮬레이터, 브라우저를 실행하지 않았다.
