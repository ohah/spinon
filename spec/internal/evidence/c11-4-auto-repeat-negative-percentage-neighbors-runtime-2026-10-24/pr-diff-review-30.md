# C11.4 signed percentage auto-repeat `calc()` implementation PR diff 검토 30개

계획 PR의 diff review와 구현/runtime 20개 검토에 이어, 구현 PR로 제출할 tracked diff·새 fixture/evidence·양 플랫폼 결과를 다시 확인했다. 각 관점은 별개의 변경 오류나 계약 경계를 다룬다. 병합을 막는 새 코드 결함은 찾지 못했다. 최종 구현/runtime 검토는 [별도 20개 기록](implementation-runtime-review-20.md)에 둔다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | runtime slice를 C11.4 전체 완료로 올려 집계가 부풀려지는가 | `spec/STATUS.md`의 C11.4 checkbox와 `72/213`을 유지하고 제한 slice만 기록한다. |
| 2 | 내부 contract version이 구현 확장만으로 달라지는가 | 0059 numeric version `0.1.0`을 유지한다. |
| 3 | 계획의 지원 문법과 구현의 실제 typed AST gate가 어긋나는가 | exact 2-term computed `Sum`, signed length·percentage 및 단일 negate로 경계를 동일하게 설명한다. |
| 4 | authored CSS 문자열이 AST 검증과 혼동되는가 | 사전 oracle와 cascade test는 각각 Chrome observation과 실제 Stylo computed AST로 구분된다. |
| 5 | old rejection fixture 제거가 다른 회귀 case까지 삭제하는가 | 기존 14 case catalog에서 이제 지원하는 negative-percentage rejection ID 한 개만 제거하고 13 case로 고정한다. |
| 6 | CSSOM/독립 기대값/Spinon 결과를 서로 같은 증거라고 부르는가 | Chrome 사전 비교, 40px/4px 독립 계산, simulator 실행을 다른 파일·단계로 기록한다. |
| 7 | length와 percentage의 term 순서가 하나만 허용되는가 | style projection·layout은 두 term 순서를 구분 없이 해석하고 reverse-order case가 포함된다. |
| 8 | single `Negate(LengthPx)`의 기존 지원이 regression되는가 | 기존 signed length shape를 그대로 유지하고 Rust calc neighbor 회귀 control을 보존한다. |
| 9 | signed percentage direct value가 CSS AST에서 0 이상으로 제한되는가 | direct finite negative percentage를 보존하고 9-scene subset에 연결한다. |
| 10 | `Negate(Percentage)`가 두 번 이상 감싸질 때 임의 수학식으로 확장되는가 | percentage의 단일 negate만 허용하며 nested negate rejection이 projection/layout test에 있다. |
| 11 | 새 calc gate가 세 항이나 nested operator를 암묵 수용하는가 | exact two-element slice만 읽고 extra-term/product/min/max/clamp는 unsupported로 유지한다. |
| 12 | signed term negation 이후 `NaN` 또는 infinity가 허용되는가 | source와 projected values에서 finite를 검사하고 NaN/infinity rejection tests를 추가했다. |
| 13 | 중간 음수 항에서 조기 clamp하거나 최종 음수를 음수 track으로 남기는가 | 전체 expression resolve 뒤 top-level property range clamp를 적용하며 두 negative-final scene이 0 결과를 고정한다. |
| 14 | column과 row percentage가 같은 physical basis를 사용해 logical axis를 뒤바꾸는가 | columns는 inline content basis, rows는 block content basis이고 양축 positive/negative scene이 포함된다. |
| 15 | border-box outer dimension을 percentage basis로 재사용하는가 | border-box fixture와 independent 116px content basis가 outer size/padding/border를 분리한다. |
| 16 | calc ID를 통한 lookup이 missing/duplicate 항목을 첫 값으로 받아들이는가 | ID 유일성과 단일 match를 요구하며 duplicate/missing rejection test가 유지된다. |
| 17 | 다른 node/property에 속한 calc ID가 neighbor로 새어 들어가는가 | node ID와 `GridTemplateColumns`/`GridTemplateRows` property를 대조한다. |
| 18 | 합산한 prefix/suffix 크기의 aggregate가 표현 범위를 넘는데 partial layout을 내는가 | per-term/aggregate representability guard와 overflow test를 유지한다. |
| 19 | projection rejection의 node/property context가 error mapping에서 사라지는가 | unsupported track을 해당 CSS property와 node로 반환하고 앱 matrix에서 successful scene의 frame을 확인한다. |
| 20 | repeat count나 fixed-minimum repeat 내부 sizing까지 이 변경이 함께 바꾸는가 | neighbor resolver만 바꾸며 Taffy repeat count/track sizing 경로는 유지한다. |
| 21 | auto-fit empty tracks의 collapsed gutter와 새 negative neighbor 위치가 섞이는가 | auto-fit occupied scene은 독립 CSSOM 모델과 비교하며 verifier의 trailing-gutter 오류를 수정했다. |
| 22 | 두 simulator가 같은 scene ID를 다른 root/item geometry로 해석하는가 | 각 scene의 host frame allowlist와 Android/iOS observed node frames를 11/11로 대조했다. |
| 23 | 신규 scene 하나가 Android AVD allowlist에서 빠지거나 다른 axis size를 쓰는가 | Android scene allowlist, root dimensions, expected item frame 및 runtime summary가 정확히 11개 모두 통과했다. |
| 24 | 신규 scene 하나가 iOS allowlist, size mapping 또는 expected item frame에서 빠지는가 | iOS allowlist와 axis/border-box size mapping을 Android catalog와 대조하고 11/11 통과했다. |
| 25 | PixelCopy 표본의 target node가 fixture root나 item이 아닌 다른 노드인가 | 각 플랫폼 11 scene에서 root/item interior sample이 정확히 22/22이며 node ID를 fixture node에 묶는다. |
| 26 | surface sample 성공을 border stroke나 전체 화면 정확성으로 과장하는가 | 표본은 두 interior color만 확인하며 README는 border stroke와 compositor 이후 표면을 미검증으로 남긴다. |
| 27 | device scale/drawable scale 좌표 변환이 scene별로 어긋나는가 | Android AVD density 2.625와 iOS drawable scale 3.0에서 지정 좌표 표본이 각각 exact다. |
| 28 | fixture capture catalog가 전역 ID 중복 또는 선택 개수 불일치를 숨기는가 | 전체 440 scene unique assertion, new catalog 9 exact IDs, selected scene count 및 순서를 검사한다. |
| 29 | fixture source·JSON·symbol이 일반 product archive로 들어가는가 | `c04-runtime-gpu`는 0/9 marker/no symbol, fixture-test positive control은 9/9 및 expected symbols로 확인한다. |
| 30 | 문서의 계획·상태·evidence 링크가 서로 어긋나거나 미검증 범위를 완료로 보이는가 | 계획, 0059, STATUS, runtime/artifact README와 source manifest를 연결하고 APK/IPA 최종 산출물 검증 및 전체 C11.4 완료를 주장하지 않는다. |

## 근거와 남은 범위

- [구현/runtime 검토 20개](implementation-runtime-review-20.md) 및 [Android/iOS 실행 결과·화면](README.md)
- [Chrome 156 사전 비교](../c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md)
- [빌드와 product/fixture archive 비교](../c11-4-auto-repeat-negative-percentage-neighbors-artifact-2026-10-24/README.md)

실기기, Android hardware GPU, iOS compositor 이후 pixels, WPT, 일반 author CSS admission, 최종 APK/IPA 전체의 fixture 제외와 full C11.4는 이 diff의 성공 판정에 포함하지 않는다.
