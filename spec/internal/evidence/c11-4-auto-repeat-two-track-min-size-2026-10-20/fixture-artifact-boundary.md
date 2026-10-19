# C11.4 fixture artifact 경계 확인

**날짜:** 2026-10-20

Opt-in simulator fixture가 기본 `spinon-ffi` library에 포함되지 않고 전용 feature에서만 들어가는지 확인했다.

```sh
mise exec -- cargo build --locked -p spinon-ffi --lib
mise exec -- cargo build --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib
```

| Build | Artifact SHA-256 | 신규 scene ID scan |
| --- | --- | --- |
| 기본 feature | `target/debug/deps/libspinon_ffi-0098a99528f2b0e6.rlib` · `0a6bd769d907d4404ef3c2c6110fadfd34a127b7f05d133eaf19dd8916e5c109` | `rg -a -l -F 'auto-fill-two-track-columns-both-190' ...` 결과 없음 |
| `c04-runtime-gpu-fixture-test` | `target/debug/deps/libspinon_ffi-128e9f0db415ce69.rlib` · `4e0880ce81c85018c215a6cecbdd8d144c62cd77a5a4ee671a1526b7ff11ca28` | 같은 검색이 opt-in rlib을 찾음 |

이 검사는 정적 library의 scene 문자열 경계만 다룬다. 최종 앱 번들 크기나 release artifact에서 fixture가 빠진다는 주장을 하지 않는다. Android/iOS harness는 의도적으로 opt-in fixture-test build flag를 켠 simulator 실행이다.
