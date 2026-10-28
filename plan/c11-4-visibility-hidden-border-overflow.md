# C11.4 `visibility:hidden` 혼합 Grid item의 border 바깥 overflow 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 `visibility:hidden` slice](./c11-4-mixed-flex-visibility-hidden.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 156 관측·독립 기대값·화면](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/README.md) · [관측 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/chrome156-observation.json) · [기대값](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/expected.json)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-visibility-hidden-border-overflow-plan-review-20.md) · **계획 PR diff 검토:** [별도 실패 관점 30개](./c11-4-visibility-hidden-border-overflow-pr-diff-review-30.md)

## 결손과 목표

PR [#738](https://github.com/ohah/spinon/pull/738)은 `visibility:hidden` item이 mixed flexible/fixed tracks의 intrinsic sizing과 layout frame에는 남고 paint에서 빠지는 네 장면을 구현했다. 그 장면의 root 축 길이는 300px라 track이 root border 바깥으로 나가지 않는다. 기존 [anonymous Grid border-overflow slice](./c11-4-anonymous-grid-border-overflow.md)는 별도의 text/image topology를 다루므로 hidden item과 visible paint control의 결합을 증명하지 않는다. 이번 후속은 **같은 visibility-hidden topology**에서 Columns와 Rows의 첫 item을 border 바깥까지 뻗게 하고, hidden 장면과 visible positive control의 surface pixels를 비교한다.

범위는 C11.4 opt-in fixture-test adapter와 검증 harness다. 일반 CSS admission, 전체 `visibility`/Grid 지원, 실기기·hardware GPU 및 C11.4 완료를 주장하지 않는다. 구현은 이 계획의 Rebase and merge 뒤 시작한다.

## 규범과 고정 기준

- [CSS 2.1 §11.2](https://www.w3.org/TR/2011/REC-CSS2-20110607/visufx.html#visibility)는 `visibility:hidden` box가 그려지지 않지만 layout에는 영향을 유지한다고 규정한다.
- [CSS Overflow Module Level 3 §3.1](https://www.w3.org/TR/css-overflow-3/#overflow-properties)는 Grid container에도 `overflow`가 적용되고 `visible`에서 box 바깥에 위치한 content를 렌더링한다고 정의한다. 이 slice는 `overflow:visible`만 다루며 clipping/scrolling은 다루지 않는다.
- [CSS Grid Layout Module Level 1, 26 March 2025 CRD §11.4·§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)의 track base, definite available space와 flexible-track 계산을 사용한다. Chrome의 serialization은 규범 기대를 만드는 데 쓰지 않는다.
- [고정 HTML fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-mixed-flex-visibility-hidden-border-overflow-c114-precomparison.html)와 [Chrome 156 Agent Browser 관측](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/README.md)은 사전 비교 자료다. 이 관측은 Spinon runtime 실행 결과가 아니다.

## 입력과 독립 기대값

Fixture는 Ahem `10px/10px`의 direct text `XXXXXXXXXX`, 그 위에 겹치는 15자 `<span>`, natural PNG `24×16`, 그리고 네 글자 text의 정확한 direct child 순서를 사용한다. A와 span은 첫 cell에 겹치며 span은 A 뒤에 온다. Hidden 장면과 visible control의 차이는 span의 `visibility` 하나다. PNG·text·style·placement·source order는 고정한다.

두 축 모두 content-size가 `96px`이고 target axis의 gap이 각각 `10px` 두 개다. 가운데 고정 track `40px`을 빼면 flex tracks에 남는 크기는 `36px`이다. 15자 Ahem text의 min-content base는 `150px`이므로 첫 `minmax(min-content,1fr)` track이 `150px`로 유지된다. 마지막 `minmax(0px,2fr)` track은 음수로 남는 공간을 갖지 않고 최소 `0px`에 고정된다. 독립 기대값은 hidden과 visible control 모두 target-axis tracks `[150,40,0]px`다.

| 축·장면 | Root border box | CSS used tracks | 첫 item·span frame (root-local) | Root 밖 양성 표본 | 비가림 border 표본 |
| --- | --- | --- | --- | --- | --- |
| Columns hidden / visible control | `100×120px` | columns `[150,40,0]`, rows `[116]` | `(2,2,150,116)` | `(120,60)` · hidden `#2864dc`, visible `#8e44ad` | `(1,60)` · `#f0c674` |
| Rows hidden / visible control | `120×100px` | columns `[116]`, rows `[150,40,0]` | `(2,2,116,150)` | `(60,120)` · hidden `#2864dc`, visible `#8e44ad` | `(60,1)` · `#f0c674` |

Root는 `box-sizing:border-box`, 2px solid `#f0c674`, `overflow:visible`이다. border를 뺀 target-axis inner size는 `96px`이며 첫 track item 끝은 root-local x/y `152px`이다. 따라서 visible A item이 오른쪽/아래 border-box 끝 `100px`을 `52px` 넘는다. 표본 `(120,60)` 또는 `(60,120)`은 root 바깥에 있고 첫 item 안에 있다. Hidden scene은 그 위치에서 아래 A item의 파랑을, visible control은 같은 곳에서 뒤에 그려지는 보라 span을 보여야 한다. Border 표본은 첫 item이 덮는 오른쪽/아래 edge를 피하고 가려지지 않는 왼쪽/위쪽 border에서 읽는다.

기대 root·item frame, CSSOM tracks, `overflow`, `display`, `visibility`, `getClientRects()` 개수, sample 좌표와 색은 [expected.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-border-overflow-precomparison-2026-10-28/expected.json)에 고정했다. 브라우저 캡처의 CSS viewport 좌표를 모바일 surface 좌표로 재사용하지 않는다. Simulator에서는 layout frame과 surface scale/offset을 통해 root-local 표본을 변환하고 PixelCopy/Metal 원본 표본을 보존한다.

## 구현 계약과 검증 관문

1. 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in route에 아래 네 exact scene만 추가한다. 새 public API, ABI, dependency, 일반 stylesheet admission은 만들지 않는다.

   - `column-visibility-hidden-border-overflow`
   - `column-visibility-visible-border-overflow-control`
   - `row-visibility-hidden-border-overflow`
   - `row-visibility-visible-border-overflow-control`

2. Runtime adapter는 root HTML `div`, parent, scene ID, direct child 네 개의 tag·namespace·ID·순서, Ahem text, PNG URL/decode/natural size, track placement, axis와 writing mode, `visibility`, root dimensions/border/overflow, revision을 고정 검증한다. 새로운 ID에 기존 `300×120`/`120×300` geometry를 허용하지 않는다. Unknown ID, child mutation/reorder, hidden↔visible ID 혼동, style mutation과 stale revision은 fail closed한다.
3. Runtime 결과는 hidden span의 `150px` min-content contribution과 non-zero full-size frame을 확인하면서 draw 목록에서 빠졌음을 별도로 확인한다. Hidden surface 표본은 파랑, visible positive control은 같은 root-local 위치에서 보라여야 한다. Layout frame을 paint 증거로 대신하지 않는다.
4. 기존 #738 mixed visibility matrix 14개를 회귀 대조군으로 유지하고 새 네 scene을 더해 최종 matrix 18개를 Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 확인한다. 모든 scene의 root/item frame, `[150,40,0]` target tracks, contribution/stage trace, scene/node identity, draw omission/presence와 presentation completion을 검사한다.
5. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 root-local border와 root 밖 표본을 확인한다. sample transform과 surface scale/offset을 실행 manifest에 기록한다. iOS 결과를 compositor-after/scanout 또는 실기기 결과로 확대하지 않는다. Screenshot은 추가 시각 자료이며 geometry와 surface sample을 대체하지 않는다.
6. 변경한 runtime/FFI focused tests, fixture-module boundary, `cargo fmt --all -- --check`, 영향받은 Clippy, 기본 및 fixture-test opt-in archive 경계, Android/iOS Simulator builds, `git diff --check`를 실행한다. 기본/product artifact에서 새 fixture marker·symbol·payload가 빠지고 opt-in positive control에 들어가는지 확인한다. Static archive 결과는 APK/IPA 전체 exclusion을 증명하지 않는다.
7. 실행 전에 simulator boot 상태를 기록하고 이번 작업이 직접 boot한 simulator만 종료한다. 기능 구현/runtime을 새 실패 관점 20개로, 구현 PR diff를 별도 30개 관점으로 검토한다. PR에는 immutable full-SHA Android/iOS screenshot 링크를 넣고 Agent Browser에서 화면 크기와 image load를 확인한 뒤 browser session/process를 종료한다. Terminal Browser는 사용하지 않는다.
8. Merge 후 계획, C11.4 상위 plan, public roadmap과 Tailnet 미리보기를 현재 main에 맞춘다. `spec/STATUS.md`의 C11.4 checkbox와 공식 `72/213`은 그대로 둔다.

## 미포함 범위

`overflow:hidden`/`clip`/`scroll`, clipping edge나 scroll container 동작, rounded/alpha border, right/bottom border 아래 합성 색, hidden ancestor/visible descendant, `visibility:collapse`, animation/mutation, arbitrary item tree/style, 일반 CSS/Grid admission, 전체 WPT, 실기기/hardware GPU, iOS compositor 이후 pixels와 전체 C11.4 완료는 검사하지 않는다.
