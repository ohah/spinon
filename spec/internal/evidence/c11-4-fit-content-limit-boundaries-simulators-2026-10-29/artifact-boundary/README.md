# Static archive fixture marker 검사

2026-10-29 macOS ARM64 `spinon-ffi` release static archive에서 신규 C11.4 `fit-content()` limit marker의 opt-in 경계를 확인했다.

| Build | SHA-256 | Marker scan |
| --- | --- | --- |
| `--features c04-runtime-gpu` | `582344bf1dd6f77ceef2ab925cf321ec4eff664c8133a09f0dc5f3a2bd21e563` | 0 match |
| `--features c04-runtime-gpu-fixture-test` | `24726ec0d0ae31a113f12030ed39de41db1bdb0671c125a9d3f3cf2f7da057ea` | scene ID, DOM prefix, `fitContentLimitBoundaries` report key 포함 |

각 빌드 로그, `strings` 출력은 같은 디렉터리에 있다. Apple `nm`는 Rust 1.96 LLVM 22.1.2가 만든 object의 attribute를 읽지 못해 `Unknown attribute kind`를 반환했다. 따라서 이 결과는 심볼 테이블 검사가 아니라 정적 archive 전체의 고유 scene/prefix/report marker scan에 근거한다. 이 작업은 새 C header나 전용 C ABI symbol을 추가하지 않았다.

이 결과는 조사한 macOS ARM64 static archive에 한정된다. Android APK, iOS app, IPA 또는 release bundle 전체에서 fixture payload가 빠짐을 증명하지 않는다.
