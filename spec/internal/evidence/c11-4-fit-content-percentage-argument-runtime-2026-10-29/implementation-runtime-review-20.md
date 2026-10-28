# C11.4 percentage `fit-content()` row · 구현/runtime 실패 관점 20개

계획 실패 관점 검토와 겹치지 않게 exact runtime admission, typed Grid input, trace, app identity와 실제 surface 결과를 대상으로 검토했다. 판정은 마지막 Android/iOS source snapshot과 focused test 결과에 한정한다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 미등록 scene ID가 이 입력의 CSS payload를 돌려받는다. | FFI exact-route test는 pinned `fit-content-percentage-row-wpt`만 노출한다. Unknown ID는 source를 받지 않는다. 통과. |
| 2 | Fixture module이나 payload가 일반 `c04-runtime-gpu` 제품 archive에 포함된다. | Default static archive marker 0개, opt-in fixture-test archive marker 4개다. 전체 APK/IPA exclusion은 검사 범위에서 제외했다. 통과. |
| 3 | HTML 이외 namespace, 다른 tag 또는 Grid root가 아닌 노드가 root로 채택된다. | Runtime `prepare`는 HTML namespace, `div`, exact root ID, `HostParent::Root`, `RuntimeBlockFlexGridTrackSizingV1`를 확인한다. 통과. |
| 4 | 추가 direct item, extra child, parent 변경 또는 node 재정렬이 같은 세 frame을 통과한다. | Root/item child count와 parent handle, 세 exact element ID, exact computed-style node set을 gate한다. 통과. |
| 5 | Definite height가 사라지거나 percentage basis가 달라진 입력을 150px로 오인한다. | Root computed property와 layout dimension 양쪽에서 `height:300px`을 exact 검사한다. Upstream pinned WPT의 observed root도 300px다. 통과. |
| 6 | `fit-content(50%)`가 다른 percentage나 `40px` 등 다른 track으로 바뀐다. | Computed CSS `fit-content(50%)`, typed `Percentage(0.5)`를 각각 검사한다. Runtime mutation test에서 `0.51`을 거부한다. 통과. |
| 7 | Implicit row가 explicit row, auto column, repeat, template area 또는 다른 flow로 바뀐다. | `grid-template-rows:none`, explicit rows `[]`, auto row `FitContent(Percentage(0.5))`, row flow, no repeat/areas, TrackSizing mode를 검사한다. `explicitRows` mutation test가 거부된다. 통과. |
| 8 | Item minimum이 auto라서 expected 150px이 automatic minimum에 의해 만들어진다. | Item style은 `min-height:0` 및 typed `LengthPx(0)`을 exact 검사한다. Runtime contribution도 `Minimum 0` / `SpecifiedMinimum`이다. 통과. |
| 9 | Unsupported `overflow:visible` author declaration이 profile에 조용히 추가된다. | Fixture CSS에 declaration이 없고 source-boundary test가 이를 막는다. Computed layout overflow는 initial `visible`로 검사한다. 통과. |
| 10 | Child size가 바뀌어 percentage row 및 overflow 기대의 원인이 달라진다. | Computed child style과 LayoutOutput frame 모두 `[0,0,30,400]`을 검사한다. 통과. |
| 11 | 긴 overflow 표본이 viewport 바깥에 있어 black host background를 child paint로 오인한다. | Percentage scene만 `320×400` CSS px canvas를 사용한다. `(15,350)` child 표본은 두 simulator surface에 들어간다. 통과. |
| 12 | Root/item/child identity가 뒤섞이거나 frame이 오래된 revision에서 온다. | Node IDs 1/2/3, 세 pre-render CSS-px frame, document/render-tree/style/environment/layout revision을 대조한다. 통과. |
| 13 | Trace에는 Columns pass가 없거나 Rows pass가 중복돼도 final value 하나만 남는다. | Rust validator는 root trace 1개, Columns/Rows 각각 한 pass 및 Rows sequence 1을 요구한다. 통과. |
| 14 | Stage가 빠지거나 순서가 뒤바뀌어도 final row 150만 맞추면 통과한다. | Rust, Android, iOS와 capture verifier가 six-stage 순서와 bases `[0,0,0,150,150,150]`을 검사한다. Rust negative test가 stage swap을 거부한다. 통과. |
| 15 | Collapsed gutter가 visible track이 되거나 track index가 섞인다. | Rust validator는 매 stage의 세 internal entries, indices 0/1/2, outer collapsed gutters와 center row base를 검사한다. 통과. |
| 16 | Minimum/min-content/max-content contribution 중 하나가 달라도 track 결과만 보고 통과한다. | Rust validator는 item 2/span 1의 `Minimum 0`, `MinContent 400`, `MaxContent 400` 순서·source·값을 검사한다. Negative tests는 contribution과 item ID 변조를 거부한다. 통과. |
| 17 | Raw trace growth limit 400px을 used row size 400px로 오인하거나 150px로 임의 덮어쓴다. | Android/iOS에서 raw growth limit 400, final base 150이 관찰됐다. CSS Grid §7.2.1·§11.5.1은 fit-content argument가 base-size distribution을 150px으로 제한하므로 이 trace는 일관된다. 통과. |
| 18 | Native matcher가 잘못된 JSON, 누락된 diagnostics 또는 strict failure를 성공으로 바꾼다. | Android JSON parser와 iOS typed casts는 필수 report field 누락 시 false를 반환한다. Capture wait는 strict failure/presentation mismatch에서 즉시 반환한다. 통과. |
| 19 | Geometry는 맞지만 root 바깥 child가 clipping되거나 표본 좌표가 틀린다. | Android PixelCopy와 iOS Metal drawable에서 child/item/root-background/outside-root 표본 각각 4/4 RGBA exact다. Draw identity도 matched다. 통과. |
| 20 | AVD 또는 iOS drawable 결과를 실기기 GPU·최종 OS 화면으로 확대 주장한다. | Android 근거는 API 37 ARM64 AVD PixelCopy, iOS는 compositor 제출 전 Metal drawable로 한정했다. 실기기/hardware GPU/post-compositor 결과는 미검증이다. 통과. |

## 최종 검증과 수정 기록

- Runtime focused tests 2/2, FFI exact-route test 1/1, runtime/FFI fixture-feature Clippy, fixture-module boundary 18/18, formatter, capture-script syntax와 `git diff --check`가 통과했다. 두 simulator는 최신 strict stage matcher로 각각 1/1 scene을 통과했다.
- 초기 Android parser의 240px 높이 가정은 `y=350` sample을 거부했다. 이 scene 전용 400px viewport와 동적 sample bound로 수정했다.
- Profile이 명시적 author `overflow:visible`을 거부해 style declaration을 제거하고 computed initial value를 검사하도록 수정했다.
- iOS viewport 호출 부위와 host presentation/environment counter 비교가 맞지 않아 strict identity가 실패했다. Scene 전용 viewport와 내부 style/layout revision 비교로 고쳤고 final presentation identity는 양 플랫폼에서 matched다.
- Capture runner는 strict 실패 뒤 150초 대기할 수 있었다. Strict mismatch를 즉시 반환하게 바꿨다.

이 검토는 pinned WPT assertion 하나와 두 simulator surface에 한정된다. 전체 WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixel 및 전체 C11.4를 증명하지 않는다.
