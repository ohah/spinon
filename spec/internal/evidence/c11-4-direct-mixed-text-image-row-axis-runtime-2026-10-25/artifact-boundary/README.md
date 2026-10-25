# C11.4 direct mixed row-axis fixture 바이너리 경계

macOS ARM64 `spinon-ffi` release static archive에서 네 row scene marker와 기존 fixture ABI를 비교했다. 결과와 재현 로그는 [artifact-boundary.json](artifact-boundary.json) 및 [runs](runs/)에 있다.

| 빌드 | archive SHA-256 | scene marker | fixture ABI |
| --- | --- | ---: | --- |
| 기본 product `c04-runtime-gpu` | `31571769b42aa332e9d0b774c7f24b540e9588436a2eed38bd4d14fada1fb87b` | 0/4 | 없음 |
| opt-in `c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test` | `f131d12d46d37d4c19adea449b75c8c6f62600990f26d4d7d06aca8c1604db04` | 4/4 | 기존 fixture symbol 포함 |

`mixed_direct_text_image_intrinsic_rows::runtime_source` 내부 Rust 함수 이름은 두 release archive의 정의 symbol 목록에서 관측되지 않았다. 따라서 scanner는 이를 product/fixture 판정의 필수 조건으로 취급하지 않는다. 판정은 기본 archive의 row scene 문자열과 fixture ABI가 모두 없는지, positive control에 네 scene marker와 ABI가 모두 있는지로 한다. 첫 시도에서 내부 Rust symbol 존재를 요구해 실패한 기록은 [이전 시도 로그](prior-attempt-internal-symbol-assertion.log)에 보존했다.

실행 명령은 `python3 spec/internal/evidence/c11-4-direct-mixed-text-image-row-axis-runtime-2026-10-25/artifact-boundary/scan-ffi-archives.py`다. 이 검사는 Rust static archive의 fixture 경계만 확인하며 최종 Android APK/iOS app bundle 전체의 fixture exclusion, package size, hardware GPU 또는 화면 출력을 증명하지 않는다.
