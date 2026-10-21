# C11.4 cyclic percentage `fit-content()` artifact 경계

`spinon-ffi` release `.rlib`를 commit `9af5ee8fa29269c589a02d6f1aafe3f72d53d1d0`에서 기본 feature와 opt-in `c04-runtime-gpu-fixture-test`로 각각 빌드했다. 두 검사는 신규 C11.4 fixture 문자열·module marker의 정적 라이브러리 포함 여부만 판정한다.

| Profile | SHA-256 | 신규 fixture marker·scene ID |
| --- | --- | --- |
| 기본 `cargo build --locked -p spinon-ffi --release --lib` | `4a7ffcb491be785d54e8b48f62a69f9dc7585a3b73530f3ea02e8307ad33ddca` | 모두 없음 |
| `--features c04-runtime-gpu-fixture-test` | `ad67198712a2b8dd251d4f9c838e92d94f4ccd68628851e79a0039f077658c5a` | module marker, fixture entry point, 세 scene ID와 HTML payload 있음 |

재현 로그는 `default-build-post-commit.log`, `fixture-test-build-post-commit.log`이고, `strings` 결과는 `default-fixture-string-scan-post-commit.txt`(비어 있음) 및 `fixture-test-fixture-string-scan-post-commit.txt`다. digest는 `default-rlib-post-commit.sha256`과 `fixture-test-rlib-post-commit.sha256`에 보존했다.

이 비교는 release Rust archive 한 개를 대상으로 한다. Android/iOS release APK·IPA에서 fixture ABI 또는 payload가 제외되는지, 최종 앱 크기나 배포 artifact를 검사한 결과로 확대하지 않는다.
