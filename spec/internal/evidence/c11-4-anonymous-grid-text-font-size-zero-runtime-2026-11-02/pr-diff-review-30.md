# C11.4 anonymous Grid text `font-size: 0` implementation PR diff review 30

**검토 범위:** 구현 후보의 staged diff, runtime evidence, plan/internal contract/STATUS 변경. 앞서 작성한 구현/runtime 실패 관점 20개와 겹치지 않도록 PR 제출 경계와 교차 파일 계약을 별도로 확인했다. 현재 리뷰 시점에는 PR 생성 전 candidate diff다.

| # | 독립 검토 관점 | 판정 |
|---:|---|---|
| 1 | 공용 `ParagraphStyle` 모델이 finite zero를 받더라도 기존 모든 caller가 예기치 않게 허용되는가 | **범위 기록.** 모델 값 표현은 넓어지지만 런타임 admission은 네 C11.4 fixture scene으로 제한했다. text-layout 전체 99/99가 통과했다. |
| 2 | IEEE `-0.0`이 CSS serialization과 다른 값으로 노출되는가 | **통과.** computed value가 정확히 `0px`일 때만 signed zero도 typed zero와 일치한다. |
| 3 | 음수, NaN, 양·음 infinity가 zero allowlist를 우회하는가 | **통과.** finite/negative/property/typed equality gate와 negative tests가 있다. |
| 4 | 별도 inline-run text style이나 rasterizer에 zero size가 새어 들어가는가 | **통과.** `InlineRunStyle`/C15 raster API 변경이 없고 문서에서 제외 범위를 밝혔다. |
| 5 | 신규 scene 이름이 기존 positive font-size scene을 덮거나 목록 밖 scene을 받는가 | **통과.** 독립 four-ID allowlist, 기존 five-size matrix 유지, unknown ID 거부를 확인했다. |
| 6 | FFI generator가 fixture feature 없이 제품 코드에 연결되는가 | **통과.** 기존 fixture 모듈 dispatch 안에만 등록했고 positive-control archive에서만 장면 marker가 발견된다. |
| 7 | runtime source builder의 scene/root/node 문자열 삽입이 JS 구문을 깨뜨리는가 | **통과.** 고정 scene ID와 Rust debug-string encoding을 사용하며 generated source 테스트가 각 ID/text를 확인한다. |
| 8 | 생성 HTML root가 Grid root identity와 HostDocument root를 혼동하는가 | **통과.** 고정 root ID와 node list를 반환하고 runtime fixture가 exact direct-child topology를 검증한다. |
| 9 | CSS root shorthand 또는 누락된 longhand가 기대값을 바꾸는가 | **통과.** width, auto height, zero edges/gap, explicit line-height와 supported text properties를 각 scene에 고정했다. |
| 10 | min/max-content scene에서 source text나 track function이 서로 바뀌는가 | **통과.** `XX XX` 입력은 공유하고 `min-content`/`max-content`만 분리한다. |
| 11 | zero text가 following box 뒤로 이동해 source-order auto-placement를 잃는가 | **통과.** text를 먼저 append하고 box를 다음 sibling으로 추가하며 양 플랫폼의 block y=10 검사가 있다. |
| 12 | following box가 intrinsic size를 가리거나 다른 box model을 사용하는가 | **통과.** 고정 border-box 5×3px, margin/padding/border 0을 generator와 validator에 고정했다. |
| 13 | whitespace-only text가 zero-width anonymous Grid item으로 생성되는가 | **통과.** `none` columns control에서 no frame/input/layout을 각각 검사한다. |
| 14 | explicit line-height가 zero font size로 0이 되는가 | **통과.** Rust와 simulator trace에서 한 줄, 10px height와 finite baseline을 요구한다. |
| 15 | computed CSS, typed font size, text owner, style revision이 서로 다른 snapshot에서 합쳐지는가 | **통과.** 기존 C11.4 validator의 owner/revision binding을 유지하고 canonical property 값과 typed zero를 짝지었다. |
| 16 | intrinsic source에서 inline 0과 block 10 기여가 바뀌거나 한 축만 검증되는가 | **통과.** min/max inline 및 min/max block fields를 Android/iOS trace에서 모두 비교한다. |
| 17 | item cell frame 폭이 paragraph layout 폭으로 오인되는가 | **통과.** text-before-box에서 5px item frame, 0px paragraph width, 5px available width를 별도 필드로 확인한다. |
| 18 | scene별 computed column type이 다른 sizing function을 허용하는가 | **통과.** 네 scene의 exact Columns input과 final track size를 검증한다. |
| 19 | empty `columns:none` scene이 line-name invariant를 임의로 완화하는가 | **통과.** empty columns일 때 empty line-name list만 받으며 scene의 expected column list도 별도 일치해야 한다. |
| 20 | auto row 결과에서 빈 Grid와 zero-height row를 혼동하는가 | **통과.** whitespace-only는 `[0]` auto row와 no item, text scene은 `[10]` 또는 `[10,3]`을 분리한다. |
| 21 | runtime frame-set accounting이 anonymous item frame을 누락하거나 두 번 더하는가 | **수정 후 통과.** frame vector를 element와 anonymous item으로 완성한 뒤 output map cardinality와 직접 비교한다. 두 simulator의 전체 four-scene trace가 통과했다. |
| 22 | Android native 기대값이 anonymous frame을 빼거나 root/item 순서를 바꾸는가 | **통과.** zero text-before-box node frame list는 root, block, anonymous item 순서를 명시한다. |
| 23 | iOS native 기대값이 Android와 geometry/ordering 의미가 다른가 | **통과.** 동일 CSS px root/frame 기대값과 item geometry를 사용하며 4/4 실행했다. |
| 24 | `boxes` 개수가 element box와 anonymous layout frame을 혼동하는가 | **통과.** summary box count는 element 수를 세고 node frame trace는 anonymous item frame을 별도로 포함한다. |
| 25 | Android/iOS scene selector가 fixture-off app에 남거나 기존 scene 분기를 가리는가 | **통과.** 새 selector는 fixture-test compile flag로 감싸고 기존 positive-font-size 분기는 보존했다. |
| 26 | 설치된 simulator app이 이번 source build와 다른 binary인가 | **통과.** Android APK와 iOS executable의 built/installed SHA-256이 일치해야 runner가 계속한다. |
| 27 | log truncation 또는 중복 trace chunk가 성공처럼 보이는가 | **통과.** runner는 complete part count, unique index, total 일치와 JSON marker를 요구한다. |
| 28 | 이전 실행 로그나 stale document state가 현재 scene을 통과시키는가 | **통과.** 장면별 고유 run ID, matching summary, HostDocument revision과 layout revision tuple을 확인한다. |
| 29 | simulator evidence가 surface pixel/glyph parity를 과장하거나 앱/브라우저 process를 남기는가 | **통과, 한계 고정.** 화면은 runtime smoke로만 설명하고 checksum을 보존한다. runner는 앱을 종료하며 Agent Browser/terminal-browser는 사용하지 않았다. |
| 30 | archive scan이나 문서가 전체 제품 지원/공식 완료를 과장하는가 | **통과, 제한 명시.** static archive scene IDs 0/4 vs 4/4만 기록한다. APK/IPA exclusion, WPT, CI, 일반 CSS admission 또는 C11.4/C11 완료를 주장하지 않으며 공식 `72/213`을 유지한다. |

## 알려진 넓은 gate 결과

`spinon-text-layout --lib`는 99/99, fixture-enabled `spinon-ffi --lib`는 189 passed·2 ignored다. feature-enabled runtime 전체는 425 passed·4 failed·3 ignored이고, default runtime 전체는 219 passed·2 failed·3 ignored다. 실패는 Root HTML grid/rejection 및 anonymous-text profile에 있으며 [runtime README](./README.md)에 test ID와 경계를 기록했다. runtime all-target Clippy는 기존 `anonymous_text_hyphens.rs`의 `too_many_arguments` lint에 막혔고 scoped `--lib` Clippy는 통과했다. CI 결과는 없다.

**결론:** 제한 C11.4 candidate와 명시된 simulator/archive evidence는 제출 범위에 맞다. 넓은 Root HTML runtime suite와 all-target Clippy는 green으로 표시하지 않았다. 이 PR은 C11.4 또는 C11 공식 완료를 표시하지 않는다.
