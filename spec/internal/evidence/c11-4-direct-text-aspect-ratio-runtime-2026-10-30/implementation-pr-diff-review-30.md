# C11.4 direct-text preferred aspect-ratio implementation PR diff review — 30 checks

**검토 대상:** 0906ed5f47b4d3964c14ee0f96eca1b24a4bd945 기준의 계획 상태 갱신, C11.4 runtime/FFI/platform fixture, CSS oracle 입력과 source-hashed Android/iOS evidence. 최종 simulator receipt는 [Android](./android-avd/c114-text-ratio-mvud6qie-49123a5e/result.json)와 [iOS](./ios-simulator/c114-text-ratio-mvud70h8-993b852b/result.json)다. 각 항목은 서로 다른 diff 실패 경계를 확인한다.

| # | PR diff 실패 관점 | 근거와 판정 |
| --- | --- | --- |
| 1 | 제한 fixture가 일반 author CSS admission이나 public API/ABI를 추가하는가 | 변경은 C11.4 fixture runtime/FFI 경로와 existing test scene에 한정되고 일반 CSS 지원 상태는 그대로 미완료다. **PASS** |
| 2 | 비-replaced direct text가 Grid의 replaced-item transferred-size suggestion 규칙으로 설명되는가 | 계획·internal contract가 §6.2/§11.5 sizing을 분리하고 Grid §6.6 replaced shortcut을 명시적으로 제외한다. **PASS** |
| 3 | 표준 기대값이 관측과 독립적으로 고정되어 있는가 | Chrome 156 observation JSON과 2:1 산식/normalization을 설명한 precomparison README가 구현 파일과 분리되어 있다. **PASS** |
| 4 | app fixture root width 변경 뒤 screenshot, JSON, hash가 서로 다른 입력을 가리키는가 | 100px root로 fixture와 Chrome observation을 다시 캡처했고 fixture/screenshot SHA-256을 갱신했다. **PASS** |
| 5 | positive/control scene에서 효과 방향이나 ratio 대조가 섞이는가 | row auto pair와 definite 40px row pair가 각각 ratio/no-ratio scene으로 고정되어 있다. **PASS** |
| 6 | author CSS oracle와 app-generated V8 DOM 입력이 다른 핵심 box style을 쓰는가 | 두 경로 모두 100px root, max-content 두 열, 0 gap, Ahem 10px, 같은 text/ratio를 사용한다. **PASS** |
| 7 | CSS fixture가 자체 기대값을 실행 결과에서 역산하는가 | 기대 track/frame은 Chrome observation 및 독립 CRD 계산으로 별도 기록되어 있고 test constants는 그 값을 고정한다. **PASS** |
| 8 | 내부 contract numeric version이 fixture 추가만으로 상승하는가 | contract 0059는 기존 0.1.0을 유지하고 신규 범위를 제한 candidate로 기록한다. **PASS** |
| 9 | 공식 상태표가 simulator success를 C11.4/C11 완료로 과장하는가 | C11.4와 C11 parent checkbox는 unchecked이고 공식 완료는 72/213으로 유지한다. **PASS** |
| 10 | 상태표 설명이 구현 전 상태로 남거나 근거 링크가 비어 있는가 | STATUS entry에 구현·양 플랫폼 결과·검증 수와 runtime evidence README를 갱신했다. **PASS** |
| 11 | fixture implementation이 product runtime 소스 루트에 놓이는가 | 신규 FFI scene module은 runtime_gpu/fixtures/c11_4 아래 있고 runtime collector는 기존 C11.4 fixture boundary에 있다. **PASS** |
| 12 | new FFI scene payload가 fixture feature 밖에서 컴파일되는가 | parent runtime_gpu fixtures module과 C header route는 기존 c04-runtime-gpu-fixture-test opt-in 경계에 속한다. **PASS** |
| 13 | 임의/미등록 FFI scene ID가 source payload를 반환하는가 | fixed four-entry catalog만 source를 만들고 unknown scene은 None을 돌려준다. registry test 1/1이 통과했다. **PASS** |
| 14 | ratio route가 기존 generic intrinsic collector에 먼저 흡수되어 ratio를 거부하는가 | dispatcher는 ratio-specific prepare_aspect_ratio를 existing prepare보다 먼저 호출한다. **PASS** |
| 15 | existing ratio-free collector와 legacy fallback의 의미가 바뀌는가 | Intrinsic profile은 기존 defaults를 보존하고 generic/legacy fallback tests가 focused 26/26에 포함된다. **PASS** |
| 16 | helper signature 변경이 baseline fixture의 aspect-ratio 허용을 넓히는가 | shared paragraph helper의 기존 intrinsic/baseline callers는 allow_aspect_ratio=false를 전달한다. **PASS** |
| 17 | root candidate prefilter만 통과하면 빠진 gap/alignment/line-name constraint를 놓치는가 | 후속 root validator가 typed tracks, gap, axes, alignments, repeats, line names, dimensions, edges를 재검사한다. **PASS** |
| 18 | computed ratio와 author scene ID가 서로 다른 item을 지정해도 pass하는가 | collector는 HostDocument text와 Stylo computed item style을 연결해 ratio를 확인하고 Rust tests가 wrong-item/value controls를 둔다. **PASS** |
| 19 | unsupported shape/edges/dimensions가 error 대신 partial layout을 남기는가 | negative tests wrong text, extra item, ratio, min/max, margin/padding/border, line-height를 검사한다. **PASS** |
| 20 | typed text intrinsic source와 ratio-aware Grid contribution이 혼동되는가 | tests와 validators는 text min/max width/height, line layout, item frame, contribution trace를 별도 필드로 비교한다. **PASS** |
| 21 | Android scene generator가 iOS/Rust fixture와 다른 node identity, CSS, ratio placement를 쓰는가 | Android expectation과 FFI four-scene catalog의 root/item IDs, source order, row templates, ratio target이 일치한다. **PASS** |
| 22 | iOS scene generator가 Android/Rust fixture와 다른 node identity, CSS, ratio placement를 쓰는가 | iOS fixed scene table과 FFI catalog가 같은 four-scene ID/text/ratio arrangement를 사용한다. **PASS** |
| 23 | Android track matcher가 extra/missing stage, wrong gutters, duplicate contribution을 통과시키는가 | Android ratio-only matcher는 exact stage/cardinality/track fields와 contribution count를 검사한다. **PASS** |
| 24 | iOS trace matcher가 Android보다 느슨하게 stage order나 intermediate track base를 놓치는가 | iOS ratio-only matcher는 stage array 전체를 검증하며 row/column/fixed-row stage sequence와 base sizes를 확인한다. **PASS** |
| 25 | verifier 자체 실패가 제품 layout regression으로 잘못 보고되는가 | strict iOS matcher에서 collapsed bool 식 오류를 분리해 고치고, 실패 시 geometry/pixels exact 로그와 최종 4/4 rerun을 함께 보존했다. **PASS** |
| 26 | contribution matcher가 중복 expected entries 때문에 실제 누락을 통과시키는가 | ratio paths와 capture runner가 exact count를 요구하도록 수정했고 final simulator capture가 네 장면 모두 통과했다. **PASS** |
| 27 | trace chunk 누락/순서 변경이 incomplete JSON을 성공 처리하는가 | capture runner는 run ID·part index·총 chunk count를 검사하고 합친 diagnostics JSON을 parse한다. 최종 run part counts는 Android 10/10/7/8, iOS 19/20/14/15다. **PASS** |
| 28 | pixel oracle가 ratio geometry를 글리프 pixel 또는 잘못된 root 좌표로 판단하는가 | sample point는 red/blue item fill과 흰 unused root 안에 있고 12/12 exact; 문서는 glyph paint 및 compositor 이후 pixels를 제외한다. **PASS** |
| 29 | 캡처가 빌드되지 않았거나 다른 installed app binary에 묶이는가 | Android local/installed APK SHA와 iOS local/installed executable SHA가 각 simulator.json에서 같다. Capture finally path는 양 플랫폼 앱을 종료한다. Android row-control 진단 log는 끝 공백 한 곳만 정리했고 payload를 보존했다. **PASS** |
| 30 | artifact/plan/evidence 결과가 final app bundle exclusion, 전체 WPT, hardware GPU 또는 completion을 과장하는가 | archive JSON은 inspected static library raw-byte markers만 판정하고 scope limits는 APK/IPA, WPT, physical GPU, compositor, general CSS와 C11 completion 주장을 제외한다. **PASS** |

**결과:** 최종 diff를 30개의 독립 관점에서 확인했다. contribution cardinality 및 iOS stage 검증 경계를 강화한 뒤 simulator를 재실행했다. 후속으로 precomparison의 시간 경계를 현재형에서 역사적 설명으로 다듬고 Android log 끝 공백을 정리해, 관련 oracle/status, trace/log, screenshot/hash와 scope 관점을 최종 staged diff에서 다시 확인했다. archive positive/negative control도 근거에 연결했다. 남은 CI/merge state는 PR 생성 뒤 GitHub에서 확인해야 하며 이 문서는 그 상태를 선취하지 않는다.
