# C11.4 · 반복 intrinsic row의 텍스트 기여

**상태:** 계획 및 Chrome 사전 비교 완료 · 구현 전

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 157 관측과 제한](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-precomparison-2026-11-08/README.md) · [270장면 기대 geometry](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-precomparison-2026-11-08/chrome157-expected.json) · [HTML fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-text-precomparison.html)

**계획 실패 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-intrinsic-text-contributions-plan-review-20.md)

## 결손과 목표

PR [#1286](https://github.com/ohah/spinon/pull/1286)은 parsed Root HTML의 `fit-content()` 뒤에 `min-content` 두 개, `max-content` 두 개, `auto` 하나가 오는 30개 고유 순서를 지원한다. 현재 `RootHtmlFitContentAutoRowsV1`은 반복 intrinsic payload를 고정 높이 목록으로 제한하고 실제 텍스트가 있는 paragraph를 거부한다. 이미 일반 Root HTML 경로에는 text paragraph 측정 입력과 `min-content` row 기여 검사가 있으므로, 다음 bounded slice는 반복 목록에서 정확히 고정된 여덟 텍스트 payload의 자연 높이가 행 크기와 overflow에 반영되는지 확인한다.

지원 후보는 [고정 fixture](../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-text-precomparison.html)의 아홉 direct grid item topology, 40px column, 10px explicit row, 8개 implicit item, 한 item당 payload `<div>` 하나다. payload 문자열은 아래 순서로 고정한다.

```text
A A A A A A
AAAA AAAA A
AA AA AA AA
A A
AAA AAA
AAAA
A A A A A A A A
AA
```

각 payload는 `monospace`, 10px font-size, 10px line-height, weight 400, normal style, zero letter/word spacing, `white-space:normal`, LTR horizontal text다. 정확히 한 paragraph source만 허용하며 inline ancestor/run style, 별도 block/flow, 이미지, 중첩 Grid, 다른 문장이나 계산되지 않은 font 설정은 범위 밖이다. 이를 웹 전체의 텍스트 intrinsic sizing 지원으로 확대 해석하지 않는다.

## 규범과 비교 모델

- CSS Grid track sizing과 implicit track list 반복을 [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)의 규범 링크 및 기존 `RootHtmlFitContentAutoRowsV1` 경계에 맞춰 확인한다.
- 기준은 제품 코드와 독립된 Agent Browser HeadlessChrome 157.0.0.0 관측이다. 기대 geometry는 `getComputedStyle(grid).gridTemplateRows`와 각 Grid/item/payload의 `getBoundingClientRect()`에서 얻는다. capture script와 reduction filter는 사전 비교 evidence에 보관한다.
- 220px definite Grid에서 30 tail order × 4 cap (`20px`, `20%`, `0%`, `120%`) × 2 alignment (`start`, `stretch`) = 240 scene을 비교한다. 별도 100px Grid에서 `20px/start` 30 scene으로 outer-border overflow를 비교한다.
- Chrome text line count는 `[2,2,2,1,2,1,3,1]`이다. canonical tail의 220px `20px/start` 기대 rows는 `[10,20,20,20,10,20,10,20,10]px`다. 100px canonical overflow scene에서 30px payload가 Grid bottom 아래로 16.65625px 이어진다. 전체 270 scene의 frames는 evidence JSON에 기록했다.
- 기하 비교 허용치는 각 수치에서 최대 `0.01 CSS px`로 한다. line count와 row/item/payload 개수는 정수·개수 exact match다. 플랫폼의 줄바꿈 결과가 다르면 원인을 조사하고 해당 범위를 fail-closed한다.

## 구현 범위와 거부 경계

- 기존 parsed Root HTML 기본 runtime route와 `RootHtmlFitContentAutoRowsV1` track sequence를 사용한다. `fit-content(px|direct percentage)`를 첫 member로 고정하고 후행 다섯 member는 기존 30개 multiset 순서만 허용한다.
- 9개 Grid item 중 첫 item은 빈 10px explicit row다. 뒤의 여덟 item은 기존 자동 배치, auto width/height, `min-height:0`, static/visible overflow wrapper 형태를 지킨다. 각 wrapper의 유일한 child만 고정 text paragraph payload가 될 수 있다.
- text payload는 위 여덟 문장과 명시 스타일을 정확히 매칭한다. 자연 paragraph measure를 통해 line count와 used row size가 결정되어야 하며 payload나 track별 결과 높이를 코드에 하드코딩하지 않는다.
- `start|stretch`, 네 cap, 30 tail order를 layout admission, style projection, RuntimeSession에서 모두 확인한다. `stretch`는 CSS 기대 track에서 `auto` track에만 free space를 배분해야 한다.
- text·style·tree·track 순서 변형은 property/node가 지정된 오류로 거부되어야 한다. 거부 후 부분 layout/draw publish가 없어야 하고, 같은 RuntimeSession의 다음 유효 revision은 복구해야 한다.
- 이 slice는 공개 API, FFI, 일반 CSS admission을 추가하지 않는다. C11.4/C11 공식 checkbox와 `78/214` 집계는 바꾸지 않는다.

## 검증 관문

1. 이 문서의 독립 계획 검토 20개를 통과한 뒤 구현에 들어간다. plan PR diff도 별도 실패 관점 30개로 검토한다.
2. fixture와 Chrome expected JSON을 기준으로 레이아웃 profile admission, exact text recognition, row/item/payload geometry, intrinsic line count, cap, stretch, visible overflow를 검증한다.
3. 음성 검사는 잘못된 문장, 추가/누락 item, inline span, `white-space:nowrap`, 다른 font/size/weight/line-height, min-height 변경, unsupported track member, 잘못된 cap·alignment를 포함한다. 실패 atomicity와 same-session recovery를 확인한다.
4. 영향받은 layout/style/runtime focused tests를 실행하고 변경 crate의 Clippy 및 `cargo fmt --all -- --check`를 확인한다. 실행하지 않은 workspace 전체 검사나 CI는 통과로 쓰지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 HTML SHA를 기본 Root HTML 제품 route로 실행한다. 화면과 `layout=ready` 기록을 남기되 screenshot을 surface pixel readback으로 표현하지 않는다. 사용한 전용 시뮬레이터와 Agent Browser session은 검증 뒤 종료하고 불필요한 전용 simulator는 제거한다.
6. 구현/runtime review는 이 계획과 다른 실패 관점 20개, 구현 PR diff는 별도 30개로 마친다. PR 본문에는 가능하면 Android/iOS 실제 실행 화면 링크를 넣는다. 병합 뒤 공식 상태는 미완료로 유지하고 Tailnet roadmap은 현재 main으로 동기화·브라우저 확인한 뒤 Agent Browser를 종료한다.

## 범위 밖

임의 텍스트, 임의 font fallback, inline formatting, paragraph flow, content mutation, nested/spanning items, arbitrary Grid dimensions, 다른 track-size 목록, CSS Grid 전체 적합성, WPT 전체, 실기기·hardware GPU 및 C11.4 완료 선언은 이 slice에 포함하지 않는다.
