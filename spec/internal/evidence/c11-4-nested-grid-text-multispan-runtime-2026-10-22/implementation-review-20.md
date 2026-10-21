# C11.4 depth-three nested Grid text multi-span 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 별도로 새 allowlist adapter, FFI source, track/input trace, native harness와 simulator 검증을 대상으로 서로 다른 실패 경계를 확인했다. 검토 중 500줄을 넘은 통합 테스트의 DOM fixture 조립을 별도 helper module로 나누고, Clippy가 찾은 불필요한 명시 lifetime 및 미사용 import를 정리한 뒤 fixture-feature Clippy와 Rust tests를 다시 실행했다.

| # | 실패 관점 | 확인 및 결과 |
| --- | --- | --- |
| 1 | 등록되지 않은 `nested-text-span*` ID가 일반 nested adapter에서 일부 layout을 만드는가 | runtime adapter가 접두사와 정확한 scene ID를 검사한다. 등록 외 ID는 즉시 거부하고 FFI allowlist test는 unlisted ID를 생성하지 않는지 확인한다. |
| 2 | 다른 C11.4 scene ID가 새 adapter에 잘못 흡수되는가 | `nested-text-span` 접두사가 아닌 scene은 `Ok(None)`으로 기존 adapter 순서에 돌려보낸다. 네 direct-leaf text multi-span control을 두 simulator에서 각각 4/4 실행했다. |
| 3 | root가 HTML `div`, 허용 profile, document root 소유가 아니어도 통과하는가 | HTML namespace/tag, `RuntimeBlockFlexGridTrackSizingV1`, `HostParent::Root`, 빈 `data-node`를 모두 요구한다. |
| 4 | root→middle→branch→inner 중간 Grid가 생략·재배치·교체되는가 | 네 role의 exact ID, tag, parent handle과 marker를 각 단계에서 대조한다. 각 Grid는 다음 단계 하나만 child로 가질 수 있다. |
| 5 | outer Grid node가 span/line placement가 바뀐 direct item으로 기여하는가 | 세 nested Grid item의 네 placement edge가 모두 `Auto`인지 확인한다. |
| 6 | outer와 inner Grid의 width/height 또는 edge 값이 달라도 동일 기대값을 통과하는가 | 네 Grid 모두 `min-content × 60px`, `min-width/height:auto`, max 없음, zero margin/padding/border를 고정한다. |
| 7 | inner 4×3 track matrix가 다른 수, track sizing 또는 implicit track으로 바뀌는가 | inner Columns 4개 `min-content`, Rows 3개 `20px`와 outer 단일 `min-content` Columns/`60px` Rows를 exact 비교한다. auto-repeat, areas, line names, auto-flow/implicit sizing은 허용하지 않는다. |
| 8 | gutter, alignment, box sizing이 contribution이나 frame에 섞이는가 | gap 0, `border-box`, start alignment와 zero border를 고정한다. |
| 9 | inner item 누락·추가·순서 변경이 같은 source order로 오인되는가 | scene별 item count와 DOM order를 suffix, text, span, row placement까지 대조한다. extra child negative test가 fail-closed를 검사한다. |
| 10 | text가 direct leaf가 아니거나 text node가 추가/누락되는가 | 각 item은 정확한 HTML `div` direct child이고 단일 direct text child만 허용한다. text 값과 owner를 확인한다. |
| 11 | 잘못된 Ahem/style 값이나 시스템 font fallback이 측정 결과에 섞이는가 | typography·white-space·wrapping·direction·spacing properties를 제한하고 system font 없는 engine에 저장소 Ahem만 등록한다. |
| 12 | 기대하는 min/max-content 값이 입력과 달라도 track만 우연히 맞아 통과하는가 | trace test에서 A/B/C의 min/max inline/block measurement와 font registry revision 1을 별도로 검사한다. |
| 13 | span-2·span-3 또는 row placement가 틀려도 값만 맞는가 | exact start line/span/end-auto와 row placement를 adapter가 검사한다. trace는 각 text source의 min-content contribution kind와 span을 검사한다. |
| 14 | span 분배로 생긴 item x/width/height 또는 중첩 Grid frame이 달라지는가 | Rust unit test와 Android/iOS verifier가 모든 node ID 순서 및 root-relative frame을 1/64 CSS px 이내로 비교한다. |
| 15 | ancestor가 nested Grid 대신 leaf text를 중복 source로 받는가 | 네 Grid trace, inner direct text input owner/source order, 각 parent의 직계 child Grid contribution을 따로 대조한다. |
| 16 | trace 일부가 없거나 Columns/Rows의 final stage를 못 읽는데 성공하는가 | 네 trace와 양 축 pass/final tracks를 요구한다. simulator verifier도 chunk 수, Grid node identity, track 수를 확인한다. |
| 17 | collapsed/gutter track이 활성 track count로 잘못 합쳐지는가 | 비교 전에 gutter/collapsed entries를 제외하고 나머지 Columns/Rows 개수와 size를 검사한다. |
| 18 | source-order 순서의 span contribution이 누락·중복·재배치되는가 | capture verifier는 inner Column pass의 전체 contribution 목록을 독립 기대 순서와 deep-equality로 비교한다. forward/reversed 두 scene 쌍 모두 통과했다. |
| 19 | 세 parent 중 하나에서 MinContent 전달이 끊기거나 다른 child로 귀속되는가 | Rust와 JS 검증기는 root→middle, middle→branch, branch→inner 각 단계에서 직계 child node, span 1, 예상 95/180px contribution을 확인한다. |
| 20 | Android/iOS가 서로 다른 scene set 또는 오래된 app artifact를 실행하는가 | 양 harness의 allowlist, frame/source 기대값과 네 scene 실행을 확인했다. Android local/installed APK SHA가 같고 iOS app hash·UDID가 기록됐다. 기본 FFI static archive에서는 네 scene ID가 없고 fixture-test positive control에는 모두 있다. |

재검증 로그는 [focused runtime/FFI tests](./runtime-multispan-focused.log), [depth controls](./runtime-depth-controls.log), [mixed span control](./runtime-mixed-span-control.log), [fixture Clippy](./fixture-runtime-clippy.log), [workspace Clippy](./workspace-clippy.log), [simulator 실행 근거](./README.md), [기본/fixture artifact 비교](./artifact-boundary.md)에서 확인할 수 있다. 고정 입력 밖의 일반 CSS Grid·WPT·실기기 동작을 보장하지 않으며 C11.4 완료 판정에도 사용하지 않는다.
