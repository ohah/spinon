# 기본/fixture-test static rlib 경계

| Build | SHA-256 | 신규 module/scene marker |
| --- | --- | --- |
| `mise exec -- cargo build --locked -p spinon-ffi --release --lib` | `4a7ffcb491be785d54e8b48f62a69f9dc7585a3b73530f3ea02e8307ad33ddca` | `cyclic_percentage_minmax` 및 세 scene ID 없음 |
| 위 명령 + `--features c04-runtime-gpu-fixture-test` | `7776fd2486ef64925012d421a63028f2ca569f351501680f1ad04780faac42ce` | module marker, 세 scene ID, fixture entry point 포함 |

Cargo JSON build 기록은 [기본](./default-build.jsonl)과 [fixture positive control](./fixture-test-build.jsonl), byte-scan 결과는 [기본](./default-check.txt)과 [positive control](./fixture-test-check.txt)에 있다. 이는 macOS ARM64 host `spinon-ffi` release `.rlib`의 feature 경계 검사다. Android/iOS 최종 release APK/IPA의 크기나 구성은 검증하지 않는다.
