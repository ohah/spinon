# `white-space:pre` fixture archive boundary

같은 macOS ARM64 Release target에서 기본 `spinon-ffi`와 `c04-runtime-gpu-fixture-test` positive control을 각각 빌드해 static archive를 검사했다. 기본 archive에는 신규 `pre` scene marker 5개와 C11.4 host fixture export가 없고, opt-in fixture archive에는 marker 5개와 host export 2개가 있다.

| 모드 | bytes | SHA-256 | 신규 marker | C11.4 host exports |
| --- | ---: | --- | ---: | ---: |
| product | 60,291,696 | `02973061955a5067a5ad285a1c000db7d36adc0ea41e139f75f2e4356b984260` | 0/5 | 0 |
| fixture positive control | 80,987,096 | `5783457493915f77a92554baa5c551682b132cca8c3cd7cfc23948530d2c4e67` | 5/5 | 2 |

원본 build log, scan JSON과 재실행 가능한 [검사 스크립트](scan-artifact.mjs)를 이 디렉터리에 보관했다. 이는 Rust static archive의 scene marker와 두 host export만 검사한다. APK/IPA 전체, 최종 앱 strip/link 결과 또는 모든 fixture payload의 배제를 증명하지 않는다.
