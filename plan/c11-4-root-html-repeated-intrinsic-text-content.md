# C11.4 · 반복 intrinsic row의 제한된 텍스트 content grammar

**상태:** 구현 PR [#1292](https://github.com/ohah/spinon/pull/1292) 병합 (`2026-11-08T18:02:21Z`, merge SHA `d2b0a2dd3cf74f868ae7faf3dcb629e75cb8d13c`) · 이 bounded slice는 병합됐고 상위 C11.4는 미완료 · 다음 slice는 아직 선정하지 않음

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**직전 slice:** [반복 intrinsic row의 고정 텍스트 기여](./c11-4-root-html-repeated-intrinsic-text-contributions.md) · PR [#1290](https://github.com/ohah/spinon/pull/1290)

**사전 비교:** [Chrome 관측과 geometry 810장면](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-precomparison-2026-11-09/README.md) · [기대 JSON](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-precomparison-2026-11-09/chrome157-expected.json) · [HTML fixture](../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-text-content-precomparison.html)

**계획 실패 검토:** [서로 다른 실패 관점 20개](./c11-4-root-html-repeated-intrinsic-text-content-plan-review-20.md)

**계획 PR diff 검토:** [서로 다른 변경 실패 관점 20개](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-precomparison-2026-11-09/plan-pr-diff-review-20.md)

## 구현 중간 검증

문법 admission을 Layout과 Style projection에 연결했고, 810개 Chrome scene의 Grid·item·text payload geometry를 각 frame component ±0.01 CSS px로 비교한다. 각 corpus에 30개 tail order가 모두 있고, 순서마다 네 cap × 두 alignment와 100px overflow 장면이 있는지도 검사한다. 기존 #1290 corpus, varied ASCII, 32-byte 경계 corpus 및 문자·줄 수가 전체 matrix에서 통과했다.

처음 row 경계를 각각 1/64 CSS px 아래로 내리는 후보는 1,176개 frame component에서 oracle 허용치를 벗어났다. 현재 후보는 Taffy track-sizing trace의 intrinsic growth limit를 사용해 row별 floor 뒤에 남은 1/64 단위를 growth-potential 순으로 배분한다. 이 원인 설명은 Chrome 157 관측값과 Chromium Grid track sizing 소스에서 추론한 것이며, Chrome 157 바이너리 내부 trace 계측 결과는 아니다. [Chromium Grid track sizing source](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/third_party/blink/renderer/core/layout/grid/grid_track_sizing_algorithm.cc).

RuntimeSession의 810-scene matrix와 거부·동일 session 복구 2개, Layout track sizing 127개, Layout/Style grammar 각 2개, 변경 3 crate Clippy와 rustfmt를 확인했다. 전체 Style crate 테스트는 아직 green이 아니다. 변경 파일과 직접 관련 없는 `c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`가 `1.7777778 / 1`과 pinned 기대값 `1.77778 / 1`의 computed-value 문자열 차이로 실패했고, focused 재실행에서도 재현됐다. 기준 branch에서의 재현 여부는 확인하지 않았다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML 앱 경로에서 세 corpus 모두 `layout=ready`로 실행됐다. OS screenshots는 visual smoke이며 PixelCopy/Metal surface readback은 아니다. 세션 데이터 Documents 18개 파일은 재설치 전후 SHA-256이 같았고 OS Saved Application State는 재생성되지 않았다. 자세한 실행·화면·데이터 범위는 [simulator/runtime README](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-runtime-2026-11-09/README.md)에 기록했다. 이 상태는 C11.4/C11 완료 판정이 아니다.

구현/runtime 실패 관점 검토는 [20개 표](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-runtime-2026-11-09/implementation-runtime-review-20.md)에 기록한다.

구현 PR 변경은 [별도 실패 관점 30개](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-text-content-runtime-2026-11-09/pr-diff-review-30.md)로 검토했다.

## 결손과 목표

PR #1290은 `RootHtmlFitContentAutoRowsV1`의 반복 intrinsic row 경로에서 정확히 여덟 문장만 허용한다. 검증기는 `repeated_text::PAYLOADS`와 문자열을 비교하고, 그 밖의 실제 텍스트는 profile에서 거부한다. 다음 bounded slice는 이 exact-string gate를 같은 topology·computed text style의 제한 문법으로 바꾼다.

허용 문법은 ASCII 대문자 단어 한 개 이상이며 단어 사이는 ASCII 공백 한 칸이다. 각 단어는 1–6자이고 전체 문자열은 1–32 ASCII byte다. 정규식으로 쓰면 `^[A-Z]{1,6}( [A-Z]{1,6})*$`이며 입력은 반드시 32 byte 이하여야 한다. 빈 문자열, 소문자, 숫자, 구두점, 선행·후행·연속 공백, 탭·개행, 7자 이상 단어, 33 byte 이상은 거부한다.

이 slice의 허용 범위:

- 기존 40px 단일 column, 10px explicit row, 한 개의 빈 explicit-row item, 여덟 개의 implicit-row item과 각 item 안의 단일 payload `<div>`를 유지한다.
- track list는 첫 `fit-content(<finite non-negative px|direct percentage>)` 뒤에 `[min-content ×2, max-content ×2, auto ×1]`을 둔 30개 고유 순서만 허용한다.
- 각 payload는 한 paragraph/source, inline ancestor/run style 없음, `monospace` 10px·line-height 10px·weight 400·normal style·zero spacing·`white-space:normal`·LTR horizontal text를 유지한다.
- Grid block-size 220px과 visible overflow 검증용 100px만 허용한다. `align-content:start|stretch`; cap은 `20px`, `20%`, `0%`, `120%`다.
- 실제 text measure가 줄 수와 row 기여를 결정한다. payload별 기대 높이 또는 줄 수를 구현에 하드코딩하지 않는다.

임의 Unicode, 임의 font/fallback, inline formatting, paragraph flow, content 길이 32 초과, 단어 길이 6 초과, 다른 Grid geometry·track grammar와 일반 CSS admission은 이 범위에 들어오지 않는다. 이는 웹 전체 텍스트 intrinsic sizing 지원이나 C11.4 완료를 뜻하지 않는다.

## 독립 기준과 Chrome 관측

Chrome oracle은 Agent Browser HeadlessChrome 157.0.0.0, DPR 1이다. 캡처는 `getComputedStyle(grid).gridTemplateRows`, Grid/item/payload의 `getBoundingClientRect()`, 각 payload의 Range client rect에서 얻은 고유 y 좌표 수를 기록한다.

세 콘텐츠 corpus를 각각 30개 tail 순서 × 4 cap × 2 alignment의 240개 220px 장면과 30개의 100px overflow 장면으로 측정했다. 총 810장면이다. 기존 #1290 문장 corpus는 `[2,2,2,1,2,1,3,1]`줄, 새 `varied-ascii` corpus는 `[3,2,2,2,2,2,2,2]`줄, 32자 경계 corpus는 `[5,6,5,4,4,1,3,4]`줄이었다. 각 corpus의 100px overflow 30장면 모두에서 텍스트 payload가 Grid 아래로 나왔다. 경계 corpus의 최대 payload bottom은 240px로 Grid 아래 140px이다.

Chrome geometry 수치는 CSS px 0.01 이내, track 순서·item/payload 개수·text length·줄 수는 exact로 비교한다. 기대 JSON은 fixture hash `803a8ee4b1be9e0c8469f828df0bb3ac8c7be1f1f2f7da1495ce2b80fc982d95`와 캡처 입력을 기록한다. 캡처 스크립트, jq 축약 필터, per-scene JSON formatter도 같은 evidence 디렉터리에 보관한다.

## 구현 및 검증 관문

1. 이 계획의 서로 다른 실패 관점 20개와 계획 PR diff 별도 20개를 완료한 다음 구현을 시작한다.
2. validator는 위 ASCII 문법과 전체 길이·단어 길이 경계를 fail-closed로 판정한다. #1290의 여덟 기존 payload와 세 사전 비교 corpus를 받아들인다. 나머지 topology, style, placement, cap, alignment 검사는 좁게 유지한다.
3. RuntimeSession matrix는 세 corpus × 270 scene과 독립 Chrome JSON을 대조한다. Grid/track/item/payload geometry는 각 값 최대 0.01 CSS px, typed order와 line count는 exact다. Layout와 style projection의 범위 검사는 acceptance/rejection 양쪽을 확인한다.
4. 음성 검사는 빈·33자 text, 7자 word, lowercase, digit/punctuation, repeated/leading/trailing whitespace, tab/newline, 추가·누락 source, inline span, 다른 font/size/weight/line-height, `white-space:nowrap`, 변형 topology/track/cap/alignment를 포함한다. 각 거부 뒤 같은 RuntimeSession의 유효 입력이 복구되는지와 실패 결과가 publish되지 않는지 확인한다.
5. 영향을 받는 layout/style/runtime focused tests, 변경 crate Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행한다. 전체 suite나 CI는 실제 실행 결과만 기록한다.
6. 세 corpus source를 각각 같은 SHA로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route에서 실행한다. 화면과 `layout=ready`를 남기되 screenshots를 PixelCopy/Metal readback으로 부르지 않는다. 기기는 검증 후 종료한다. 새 전용 S03 Pilot simulator는 만들지 않는다.
7. 구현/runtime은 계획과 다른 실패 관점 20개, 구현 PR diff는 별도 20개로 검토한다. PR 본문에 Android/iOS 실행 화면 링크가 가능하면 포함한다. 병합 뒤 C11.4/C11 완료 체크와 공식 `78/214`는 유지하고 Tailnet roadmap을 main과 맞춘다.

## 범위 판정

이 후보는 #1290의 exact-string 목록을 제한된 ASCII grammar로 넓히는 한 단계다. 구현이 통과해도 HTML text layout 전체, 모든 writing system, 일반 Grid track sizing, WPT 전체 또는 상위 C11.4/C11 완료로 승격하지 않는다.
