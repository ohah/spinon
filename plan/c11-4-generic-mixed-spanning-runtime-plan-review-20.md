# C11.4 generic mixed spanning runtime 계획 적대 검토 20개

이 검토는 [계획](./c11-4-generic-mixed-spanning-runtime.md)의 입력 계약과 판정 경계를 20개 서로 다른 실패 가설로 대조한다. 기존 multi-span 구현 결과를 새 generic route의 검증으로 잘못 세지 않으며, 아래 구현/runtime 게이트를 통과하기 전에는 계획 상태를 완료로 바꾸지 않는다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 이전 고정-ID 장면이 generic 성공으로 오인되는가 | 실패 조건이 아님. 새 scene/root/item ID를 사용하고 기존 고정 route 회귀도 함께 유지한다. |
| 2 | launcher scene ID가 기존 것이라는 이유로 고정 HostNode route가 재사용되는가 | launcher ID를 유지하되 HTML root/item ID를 새 값으로 만들고, 고정 route lookup을 통과하지 않아야 한다. Rust 입력도 새 HostNode ID로 확인한다. |
| 3 | source order 네 장면이 placement도 바꾸는가 | 허용 순서를 A/B, B/A, A/B/C, C/B/A로 고정하고 CSS placement는 항목별로 고정한다. |
| 4 | 텍스트 short/long contribution 값이 구현 결과에서 역산되는가 | 50px·180px을 고정 Ahem 문자열에서 독립 계산하고 oracle 문서에 분리한다. |
| 5 | 이미지 72px 기여가 authored width로만 처리되는가 | natural 24×16, authored height 48, width auto로 transfer 입력을 고정하고 decoded metadata를 확인한다. |
| 6 | PNG URL은 같지만 byte payload가 달라지는가 | 기존 고정 fixture asset과 hash를 재사용하고 새 source/payload 경계에서 hash를 연결한다. |
| 7 | span-2 그룹에서 text/image를 별도 group으로 계산하는가 | §11.5.1 독립 기대값 `[25,36,36,0]`으로 같은 길이 group의 최대 planned increase를 판별한다. |
| 8 | span-3 group이 span-2보다 먼저 적용되는가 | 두 source order scene의 기대 track과 trace 순서를 분리해 확인하도록 규정했다. |
| 9 | 비어 있는 4번째 track이 0 대신 implicit sizing되는가 | 네 번째 track의 기대값을 명시적으로 0px으로 고정한다. |
| 10 | row gap 또는 implicit row가 열 contribution에 섞이는가 | gap 0, 세 fixed 60px rows와 각 항목의 row를 고정하며 trace 축을 분리한다. |
| 11 | root width 220px가 track 합으로 축소되는가 | root border-box `220×180`과 used column 합을 별도 assertion한다. |
| 12 | fractional rational 결과의 반올림이 false pass를 만드는가 | 독립 rational values를 기준으로 정해진 CSS-px tolerance를 적용하고 Chrome quantization을 별도 기록한다. |
| 13 | A item wrapping/frame을 놓쳐 track만 맞아도 통과하는가 | 네 장면에서 item identity별 root-relative text/image frame을 고정했다. |
| 14 | DOM child 재정렬 때문에 text/image mapping이 뒤바뀌는가 | tag·content·placement를 결합해 종류를 식별하고 reverse order control을 둔다. |
| 15 | 두 text child 또는 nested text가 하나로 수락되는가 | 각 text item은 단일 direct text node만 허용하고 extra/nested child는 negative control이다. |
| 16 | mixed signature가 아닌 unrelated root를 오류로 삼키는가 | non-mixed root는 기존 `Ok(None)` fallback을 유지한다. |
| 17 | 잘못된 mixed span이 text-only/image-only 입력으로 강등되는가 | spanning signature 발견 뒤 malformed input은 fail-closed하며 partial output이 없어야 한다. |
| 18 | document/style revision 불일치를 현재 입력처럼 처리하는가 | generation과 document/render-tree/style revision 일치가 필수다. |
| 19 | Android 또는 iOS 결과 누락을 합산 성공으로 가리는가 | 각 플랫폼별 4/4 scene·draw/present를 별도로 요구한다. |
| 20 | opt-in fixture가 기본/product artifact에 들어가거나 C11.4 완료로 승격되는가 | 기본/product negative control과 fixture positive control을 분리하고 checkbox 및 `72/213`을 유지한다. |

**판정:** 계획 설계 PASS 20/20. 계획을 수립하며 item geometry를 원본 Chrome observation에서 root-relative 값으로 변환해 표에 명시했다. 이는 브라우저 observation을 CSS Grid normative expectation으로 승격하지 않는다. 실제 implementation/runtime와 PR diff는 별도 실패 관점으로 검토한다.
