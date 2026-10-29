# C11.4 zoomed `fit-content` 구현·런타임 실패 관점 검토 20개

2026-10-29 · 계획 사전검토를 재사용하지 않고 이번 구현 코드와 최종 simulator run을 대상으로 독립 검토했다. 각 항목은 다른 실패 경로를 다룬다.

| # | 공격 관점 | 확인한 방어와 근거 | 판정 |
|---:|---|---|---|
| 1 | 기본 C11.4 cascade에 `zoom`이 새어 들어가 일반 입력이 조용히 지원되는가 | 일반 author allowlist는 계속 거부한다. Style regression에서 기본 cascade는 `zoom`을 반환하고 fixture route만 계산한다. | 통과 |
| 2 | opt-in 경로가 fixture 외 `float` 같은 미지원 선언까지 허용하는가 | zoom 전용 allowlist에 `zoom`만 추가한다. 같은 style regression에서 `float`는 계속 거부한다. | 통과 |
| 3 | 런타임이 다른 scene ID에 고정 HTML source를 반환하는가 | FFI fixture는 `fit-content-zoomed-grid-wpt`만 source를 주며 변형된 scene ID는 거부한다. FFI test 1/1. | 통과 |
| 4 | 다른 HTML element/namespace/parent를 같은 root처럼 받아들이는가 | runtime `prepare`가 root ID, HTML namespace, `div`, document fragment parent와 profile을 검사한다. | 통과 |
| 5 | item/child를 중간 노드 아래로 옮기거나 순서를 바꾸면 통과하는가 | 각각의 직접 parent, 개수, 고정 ID를 순서대로 확인한다. | 통과 |
| 6 | 누락·중복·추가 style node가 관측에 섞이는가 | node ID 집합의 중복과 예상 세 노드 외/누락 스타일을 거부한다. output 단계도 duplicate/extra frames를 거부한다. | 통과 |
| 7 | root specified/effective zoom이 다르거나 NaN/무한대가 들어오면 어떻게 되는가 | `zoom_matches`는 유한성·specified `2`·effective `2`를 epsilon 안에서 비교한다. | 통과 |
| 8 | 1, 1.5, 3, 음수 zoom이나 다른 owner로 이동한 zoom을 수용하는가 | 새 runtime regression에서 root 변형과 item/child owner 이동 총 7개 변형을 모두 거부한다. | 통과 |
| 9 | item/child에서 inherited effective zoom이 2가 아니어도 성공하는가 | item과 child는 specified `1`, effective `2`를 각각 검증한다. root/item/child 전체 cascade는 최종 trace에 기록된다. | 통과 |
| 10 | root width/height나 child height가 zoom 변환 없이 layout에 들어가는가 | typed dimensions는 `120×600`, child `800`; Android·iOS frame 결과가 `120×600 / 120×80 / 120×800`이다. | 통과 |
| 11 | fit-content 인수가 원본 `40px`로 Taffy에 전달되거나 이중 scaling되는가 | runtime은 typed `fit-content(80px)`을 exact 비교한다. upstream WPT CSSOM `40px`은 별도 값으로 유지한다. | 통과 |
| 12 | 다른 explicit row/column, auto-track, line name, template area가 조용히 들어오는가 | Grid input 검사에서 고정 column, 빈 explicit rows, auto rows, line names, placement mode 등을 모두 비교한다. 회귀 test는 40px 인수·추가 row·변경 column을 거부한다. | 통과 |
| 13 | item minimum 또는 max-content 기여가 다른 값으로 계산되는가 | rows trace는 minimum `0` / `SpecifiedMinimum`, min-content와 max-content 각각 `800`, span 1을 exact 비교한다. | 통과 |
| 14 | sizing stage가 생략·재배열되거나 fit-content가 40px로 적용되는가 | 6개 row stage와 각 track base/gutter 상태를 순서 및 epsilon으로 검사한다. 회귀 test는 40px base와 잘못된 sequence를 거부한다. | 통과 |
| 15 | 다른 grid trace, pass 또는 extra frame이 fixture 성공으로 보고되는가 | root trace 하나와 Columns/Rows 두 pass, Rows sequence 1, 세 고유 frame만 허용한다. | 통과 |
| 16 | 오래된 style/document/render-tree/environment 결과가 새 surface에 발행되는가 | `revisions_match`가 host generation/document/tree, style, environment를 key와 대조한다. 추가 test가 stale document/tree/style/environment key를 거부한다. | 통과 |
| 17 | diagnostics 누락 또는 계산 일부 실패가 성공 응답처럼 보이는가 | diagnostics, style observation, root Grid input, Rows pass가 없으면 구체적 layout failure를 반환한다. | 통과 |
| 18 | fixture URL을 만들어도 일반 HTML mount나 제품 경로에서 zoom을 계산하는가 | fixture/runtime mount 생성자는 opt-in Cargo feature에서만 제공하고 Chrome-only base URL을 고정해 임의 URL 선택을 허용하지 않는다. 기본 `c04-runtime-gpu` 빌드도 통과했다. | 통과 |
| 19 | 기본 제품 FFI archive에 fixture ID/source/symbol이 실수로 포함되는가 | `c04-runtime-gpu` archive에서 문자열·심볼 부재, fixture-test archive에서 source payload와 `runtime_source` symbol 존재를 LLVM 22 도구로 각각 확인했다. | 통과 |
| 20 | frame/screenshot만 맞고 실제 child overflow나 root clip이 잘못됐는데 통과하는가 | Android PixelCopy와 iOS compositor 제출 전 Metal에서 item 밖·root 밖 yellow 3개 및 root 밖 black control을 각각 4/4 exact 확인했다. 플랫폼과 compositor 한계는 evidence에 명시했다. | 통과 |

## 검토 중 발견 및 조치

기본 제품 feature 빌드 중 fixture 전용 생성자에 `cfg`를 붙여 기존 public fragment 생성자의 내부 호출을 막는 컴파일 오류를 발견했다. 일반 생성자는 내부 전용으로 항상 유지하고, 고정 zoom fixture view 생성자만 opt-in feature에서 노출하도록 분리했다. PR diff 검토에서는 public runtime view constructor가 임의 base URL을 받을 수 있는 경계를 발견했다. public constructor에서 URL 인자를 없애고 Chrome-only fixture URL을 고정했으며, base-URL helper를 private으로 내렸다. 두 수정 뒤 runtime test 4/4, style test 1/1, FFI test 1/1, fixture boundary 20/20, Clippy, product/fixture archive, Android/iOS app build와 두 Simulator capture를 다시 확인했다. 검토 후 미해결 구현 결함은 없다.

## 판정 범위

이 검토는 이 고정 fixture, 현재 opt-in feature, 기록한 API 37 ARM64 software AVD와 iOS 26.2 Simulator에 한정한다. 일반 CSS zoom, 임의 CSS tree, 실기기/hardware GPU, iOS compositor 최종 출력, 전체 WPT 실행 또는 전체 C11.4 완료를 검토·승인한 것이 아니다.
