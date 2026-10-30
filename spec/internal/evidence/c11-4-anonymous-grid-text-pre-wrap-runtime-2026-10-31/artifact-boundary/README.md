# C11.4 PreWrap fixture static archive boundary

## 판정

Rust 1.96.1, macOS ARM64에서 같은 release target을 product feature와 C11.4 fixture-test feature로 각각 빌드하고 정적 `libspinon_ffi.a`를 검사했다. 제품 빌드에는 신규 PreWrap scene marker 7개가 없고 C11.4 host fixture export도 없었다. fixture positive control에는 marker 7개와 host fixture export 두 개가 모두 있었다.

| 모드 | 크기 | SHA-256 | scene marker | C11.4 host exports |
| --- | ---: | --- | ---: | ---: |
| product | 75,254,648 bytes | `cad13f32b2ff38af217a7bddda2ca60f63fade0690b0ae6305f9fe59ca81ff26` | 0/7 | 0 |
| fixture positive control | 80,970,496 bytes | `2d72e8f81040b2a650b0dae6fb636cd93c46819ec97a2a50a45fdf595aed8d6c` | 7/7 | 2 |

## 원본

- [product build log](product-build.log), [product scan JSON](product-artifact.json)
- [fixture positive-control build log](fixture-build.log), [fixture scan JSON](fixture-artifact.json)
- [검사 스크립트](scan-artifact.mjs)

검사 대상은 macOS ARM64의 Rust static archive다. 전체 바이너리의 모든 가능한 payload, Android APK, iOS IPA, 앱 번들 크기 또는 플랫폼별 strip/link 결과를 증명하지 않는다. archive 결과를 최종 배포물 전체의 fixture 부재로 확대 해석하지 않는다.
