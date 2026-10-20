# Fixture artifact 경계 검사

최신 코드에서 host debug `spinon-ffi` rlib을 기본 feature와 fixture-test positive control로 각각 다시 빌드했다. 각 빌드 뒤 `strings target/debug/libspinon_ffi.rlib | rg -q 'text-span2-forward'`로 고정 scene ID 문자열을 검사했다.

| Cargo 조합 | 크기 | SHA-256 | scene 문자열 |
|---|---:|---|---|
| `mise exec -- cargo build --locked -p spinon-ffi --lib` | 2,663,048 bytes | `5276ecd535c41db1fb9c5125bff5114e75f45bdebb29be81a83aa5f4a24be17c` | 없음 |
| `mise exec -- cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | 20,688,688 bytes | `11c24c9d46557c00a31ac1f278acda5a2bdae19b7b27fd7f99c8d741a799b1a` | 있음 |

기본 artifact에서 고정 scene ID가 빠지고 opt-in fixture positive control에 포함된다. 이는 host debug `.rlib` 경계만 증명하며 Android/iOS release artifact 크기나 최종 배포 번들을 판정하지 않는다.
