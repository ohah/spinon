# C11.4 `visibility:hidden` mixed-track 계획 · 실패 관점 검토 20개

**대상:** [mixed flex/fixed track `visibility:hidden` 계획](./c11-4-mixed-flex-visibility-hidden.md), [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/README.md), 기존 [display:none slice](./c11-4-mixed-flex-hidden-item.md).

**검토 경계:** 계획과 기준 입력만 읽어 구현 전 요구조건을 점검했다. 제품 코드 수정, runtime/build 실행 또는 simulator 검증은 이 계획 검토에 포함하지 않는다.

| # | 서로 다른 실패 관점 | 계획 검토와 필요한 방지 조건 |
| ---: | --- | --- |
| 1 | `visibility:hidden`을 `display:none`처럼 item sizing에서 제외 | CSS 2.1 §11.2의 layout participation을 고정하고 hidden min-content `150px` 및 final track `[150,40,90]px`를 요구한다. 통과. |
| 2 | 숨김 상자 자체가 생성되지 않아 box/frame assertion 없이 성공 | Hidden span은 `display:block`, client/layout box 1개와 full-size frame을 요구하며 draw 여부와 분리한다. 통과. |
| 3 | `150px` 기대값이 Chrome 결과에서 역산 | Ahem 글자수·size와 §11.5/§11.7 freeze 산술을 별도 `derive_expected.py`/JSON에 둔다. 통과. |
| 4 | `1fr` freeze와 `2fr` 재계산을 마지막 총합만 보고 놓침 | 초기 `240/3=80px`, first freeze `150px`, 남은 `90px`, final `[150,40,90]`를 각각 계획에 적고 trace로 판정한다. 통과. |
| 5 | row 축 scene이 사실 columns sizing으로 연결 | Root dimensions, track/gap axis와 `vertical-rl` text를 고정하고 두 축을 각각 strict matcher에서 확인한다. 통과. |
| 6 | orthogonal text의 writing mode가 A 또는 hidden item 한쪽에서 누락 | Rows scene에서 두 text item 모두 vertical writing-mode와 동일 Ahem style을 검증한다. 통과. |
| 7 | visible A item이 이미 150px을 주어 hidden contribution 누락이 감춰짐 | Hidden/visible contribution trace에서 hidden span owner NodeId와 150px contribution을 직접 확인하고, visible control과 동일 final size만으로 합격하지 않는다. 통과. |
| 8 | hidden text NodeId가 다른 item의 contribution에 연결 | Semantic item ID→HostNodeId mapping, source order, axis/span/value를 함께 assert한다. 통과. |
| 9 | same-cell overlap에서 source-order 변화가 결과를 섞음 | A 다음 hidden span의 순서를 고정하고, placement가 둘 다 정확히 first cell인지 검증한다. 통과. |
| 10 | visible control이 hidden scene과 다른 CSS·DOM 입력을 사용 | 네 scene을 동일 markup/style에서 visibility만 바꿔 비교하며 Chrome JSON을 각각 보존한다. 통과. |
| 11 | visible control sample 지점이 잘못되어 hidden paint 실패를 못 찾음 | Hidden item이 A보다 나중에 paint되는 source order와 root-local blue/purple 표본을 screenshot/PixelCopy/Metal에서 고정한다. 통과. |
| 12 | visibility hidden의 layout 성공을 hidden paint 성공으로 오해 | Frame/track assertions와 draw omission/sample checks를 분리하고, layout frame만으로 draw 결과를 주장하지 않는다. 통과. |
| 13 | `visibility:collapse`를 `hidden`과 같은 값으로 넓혀 받음 | Collapse는 기존 fail-closed 경계로 두고 명시 negative control에 넣는다. 통과. |
| 14 | exact fixture 외 root/item/subtree 변형이 generic fallback으로 합격 | Root, namespace/tag/parent, child count/order, text, style, placement, image와 revision을 exact allowlist로 고정한다. 통과. |
| 15 | 특정 scene ID에 visibility를 바꿔 끼워 bypass | Hidden ID는 `hidden`, visible control ID는 `visible`만 허용하고 반대 변조를 거부한다. 통과. |
| 16 | 이전 `display:none` adapter 회귀 | PR #734의 두 scene을 별도 regression controls로 12-scene matrix에 포함한다. 통과. |
| 17 | 앱 host가 scene allowlist 누락 또는 플랫폼마다 다름 | Android와 iOS의 동일한 신규 네 ID를 검증하고 unknown/cross-profile controls를 양 host에서 거부한다. 통과. |
| 18 | fixture 코드가 기본 product archive에 들어감 | Default archive negative와 fixture-test positive control을 별도로 검사하며 APK/IPA 전체 exclusion으로 확대하지 않는다. 통과. |
| 19 | 단일 simulator 결과를 기기·OS 보편 결과로 확대 | API 37 AVD 및 고정 iOS Simulator에 한정하고 hardware/physical/compositor-after 경계를 분명히 기록한다. 통과. |
| 20 | 작은 slice를 전체 C11.4 완료나 `72/213` 증가로 표현 | 공개 CSS admission·공식 checkbox·완료 count를 유지하고 실제 runtime 근거만 plan/roadmap에 반영한다. 통과. |

**판정:** PASS 20/20, 계획 blocker 0개. 기준 산술, 두 axis, draw-vs-layout 경계, positive control과 fail-closed 경계가 구현 전 판정에 충분히 구체적이다. 구현/runtime 검토 20개와 implementation PR diff 검토 30개는 실제 코드가 나온 뒤 새 관점으로 별도 수행한다.
