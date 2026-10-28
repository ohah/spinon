# C11.4 overflow automatic minimum 구현/runtime 검토 · 20개 관점

**대상:** `spinon-layout` test-only module의 14개 Grid auto-minimum case, Chrome 156 비교 fixture 및 해당 구현 결과.

**경계:** Rust `TaffyLayoutEngine` 단위 테스트와 고정 browser fixture를 검토했다. 앱 route, CSS parser/admission, FFI, clipping/scroll UI, Android/iOS 앱 runtime, WPT 및 전체 C11.4 지원을 검증했다고 주장하지 않는다.

## 실패 관점

| # | 별도의 실패 경로 | 현재 방지 조건과 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | `visible`이 scrollable 값으로 분류되어 content minimum을 잃음 | Columns/Rows 둘 다 computed/test input `Visible`에서 content-based source와 min-content 값을 검사한다. | 통과 |
| 2 | `clip`을 scroll container로 처리 | 양 축 `Clip` case가 content-based minimum을 기대하며 Chrome fixture의 computed overflow도 `clip`으로 확인했다. | 통과 |
| 3 | `hidden`만 overflow zero source로 매핑되고 `auto`는 누락 | 양 축 `Auto` case는 `LayoutOverflow::Auto`를 직접 주고 overflow automatic source를 검사한다. adapter의 기존 `Auto -> Hidden` 매핑과 일치한다. | 통과 |
| 4 | `hidden`의 track/frame만 맞고 minimum source가 잘못됨 | `Minimum` contribution source가 `OverflowAutomaticMinimum`인지 별도로 assert한다. | 통과 |
| 5 | `scroll`을 `auto` 또는 `hidden` 대표값에 합쳐 개별 분기 회귀를 놓침 | Columns/Rows 각각 `Scroll`을 별도 입력으로 둔다. scrollbar geometry는 범위에 포함하지 않는다. | 통과 |
| 6 | content-based trace의 Grid 조건이 불충분함 | 한 개 auto-minimum track, 단일-track span, flexible max 조건을 trace에서 확인한다. | 통과 |
| 7 | item `min-width/height:0` control이 overflow 결과로 오인됨 | `SpecifiedMinimum` source와 0 값을 visible overflow에서 독립 검사한다. | 통과 |
| 8 | `minmax(0,1fr)` control이 item minimum 0과 같은 query로 처리됨 | 해당 control은 target item의 Minimum contribution이 없는 것을 검사한다. 독립 JSON에도 `queried:false`를 기록했다. | 통과 |
| 9 | zero-track control이 다른 Minimum query를 숨김 | contribution은 target item 및 `Minimum` kind로 좁히고, 최종 track은 별도로 검사한다. | 통과 |
| 10 | text min-content가 fallback font에 따라 달라짐 | fixture Ahem bytes를 직접 등록하고 10개 공백 없는 glyph를 사용한다. Agent Browser metadata에서 Ahem loaded를 확인했다. | 통과 |
| 11 | row axis에 text height가 작아 overflow 차이가 생기지 않음 | Rows는 10×10 intrinsic image와 5px definite Grid height로 10px/5px 경계를 만든다. | 통과 |
| 12 | row 축소가 image aspect ratio를 통해 column width도 바꿔 혼합 결과 발생 | 첫 실행에서 width가 5px로 줄어든 결함을 기록했다. HTML과 Rust style 모두 image width를 10px로 고정한 뒤 7개 row case를 다시 통과했다. | 수정 후 통과 |
| 13 | `<img>` UA 기본 `overflow:clip`이 `visible` baseline으로 오인됨 | Agent Browser computed style에서 최초 불일치를 발견했다. CSS baseline을 명시적으로 `visible`로 만들고 14개 기대값 대조를 다시 실행했다. | 수정 후 통과 |
| 14 | Rows text 10px layout과 5px item frame mismatch 때문에 C15 guard가 느슨해짐 | 처음의 `TextLayoutHeightMismatch`를 기록하고 Rows만 replaced-image 경로로 바꿨다. C15 text guard는 수정하지 않았다. | 수정 후 통과 |
| 15 | intrinsic size가 다른 document revision에서 재사용됨 | `LayoutIntrinsicSizeSnapshot` source는 같은 HostDocument snapshot의 generation/document/render-tree revision을 사용한다. | 통과 |
| 16 | contribution을 다른 Grid item의 값과 혼동 | trace의 `item`을 해당 text/image node ID와 일치시킨 뒤 source/value를 검사한다. | 통과 |
| 17 | gutter 또는 collapsed track이 target 결과로 계산됨 | final stage에서 gutter/collapsed track을 제외하고 target-axis track가 정확히 하나인지 assert한다. | 통과 |
| 18 | item frame 또는 root frame만 우연히 맞고 track base는 다름 | root frame, item frame, final track base를 각각 비교한다. `spinon-layout` 전체 295개 테스트도 통과했다. | 통과 |
| 19 | layout adapter test를 CSS parsing이나 scroll UI 지원으로 과장 | 변경은 `#[cfg(test)]`에만 추가했고 plan/evidence는 parser, scrollbar, clipping, runtime을 제외한다. | 통과 |
| 20 | narrow test를 simulator 또는 제품 C11.4 완료 증거로 오해 | 이 PR 범위에는 앱 runtime 변경이 없고 simulator를 실행하지 않았다. C11.4 checkbox는 미완료로 유지한다. | 통과 |

## 수정 후 실행 결과

- focused tests: 2 passed.
- `cargo test -p spinon-layout`: 295 passed, 0 failed; doc-tests 0.
- `cargo fmt --all -- --check`: 통과.
- `cargo clippy -p spinon-layout --all-targets -- -D warnings`: 통과. 최초 lint의 `match_like_matches_macro` 지적을 수정했다.
- Agent Browser 0.31.1 / HeadlessChrome 156: HTML fixture, 독립 기대값, computed track/frame/overflow 14개를 비교해 모두 일치했다. screenshot·metadata·observation의 해시는 manifest에 기록했다.

**결론:** 수정 뒤 남은 구현 blocker 0개다. 앞선 계획과 PR diff 검토에서 놓친 zero-track query, row text mismatch, image aspect-ratio coupling, `<img>` UA overflow는 각 기존 기록에 추가했고 여기서 실제 수정 후 결과를 다시 확인했다.
