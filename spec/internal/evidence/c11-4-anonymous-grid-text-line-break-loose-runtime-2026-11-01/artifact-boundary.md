# C11.4 `line-break: loose` FFI archive 경계

Rust 1.96.1 release static archive 두 개를 같은 checkout에서 비교했다. Fixture build는 기존 `c04-runtime-gpu-fixture-test`만 추가로 활성화했다.

| Build | Archive SHA-256 | C11.4 fixture host symbol | loose scene marker | 고정 Noto Sans KR 전체 bytes |
| --- | --- | --- | --- | --- |
| `cargo build --locked --release -p spinon-ffi --lib` | `21bc922d2697fa5e99f07ea39f5120d5d433ddf9df86f5a8f90e20eb4b8a06fc` | 없음 | 없음 | 없음 |
| `cargo build --locked --release -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | `2d08483f556e35102eaaabbc8906a44d2c52346d72e1e9805b2bdae70fcfce45` | 있음 | 있음 | 있음, byte offset `13,441,028` |

`llvm-nm`은 Rust 1.96.1의 `llvm-tools` component를 사용했다. `new_c11_4_grid_track_sizing_fixture` symbol은 기본 archive에서 없고 fixture archive에서 발견됐다. `strings`에서 `line-break-ja-loose-min-content`, `line-break-en-loose-min-content`, `line-break-ja-normal-prefix-control-min-content`는 기본 archive에 없고 fixture archive에 있다. 고정 font `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf`는 10,415,420 bytes이며 기본 archive에서는 연속 byte sequence가 없고 fixture archive에서는 위 offset에서 발견됐다. Font SHA-256은 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`다.

이 비교는 두 static FFI archive에 한정된다. 완성 APK/IPA의 모든 fixture 자산이 모든 설정에서 제거된다고 주장하지 않는다. 비교용 archive 사본은 `/tmp/spinon-c11-4-line-break-loose-default.a`와 `/tmp/spinon-c11-4-line-break-loose-fixture.a`에 있으며 저장소에는 넣지 않았다.
