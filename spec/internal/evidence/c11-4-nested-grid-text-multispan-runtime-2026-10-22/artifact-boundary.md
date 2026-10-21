# 기본 제품/fixture-test 정적 archive 경계

검사는 macOS debug `spinon-ffi` static archive에 한정한다. 최종 Android APK/iOS app bundle 크기나 release artifact의 fixture 제외를 뜻하지 않는다.

| 구성 | 명령 | `target/debug/libspinon_ffi.a` SHA-256 | 크기 | 네 신규 scene ID |
| --- | --- | --- | ---: | --- |
| 기본 제품 | `mise exec -- cargo build --locked -p spinon-ffi --lib` | `db4092dc501facaff8e9cf96465e8375c4c3c31bd8aea25eb574e03208702de9` | 511,462,288 bytes | `strings` 검색 결과 없음 |
| fixture positive control | `mise exec -- cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | `37c530222972c8a3cf1637553d624f0aa638888d89a3c1fa6964a236f459e2a6` | 689,349,296 bytes | 네 ID와 고정 text payload 포함 |

positive control의 `strings` 출력에서 `nested-text-span2-forward`, `nested-text-span2-reversed`, `nested-text-span2-before-span3`, `nested-text-span3-before-span2`와 해당 source-order payload를 확인했다. 기본 구성에서는 같은 네 ID를 각각 검색해 일치 항목이 없었다. 각 build 로그는 `product-default-build.log`, `fixture-positive-build.log`에 있다. feature build는 기존 C04 runtime GPU fixture-test 묶음도 포함하므로 크기 차이는 이 네 scene의 크기 측정이 아니다.
