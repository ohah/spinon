# C11.4 · `align-content: normal` 런타임 계획 검토 20관점

**검토 대상:** `plan/c11-4-align-content-normal-runtime.md`, 관련 CSS fixture 및 사전 비교 artifact, 현재 `origin/main` 구현 제약
**기준 소스:** `origin/main` `18bb75f295e472c567e554c41e30ec4534940cb6`
**결론:** **PASS — 재검토에서 남은 두 PARTIAL도 해소됐다. 계획 PR의 별도 30관점 diff 검토와 병합은 구현 전 선행 단계다.**
**검토 경계:** 문서·소스·고정 artifact만 읽었다. build/test/browser는 실행하지 않았다. 최신 `SHA256SUMS`는 저장소 루트 기준 6/6 통과했다.

## 20개 독립 관점

| # | 관점 | 판정 | 근거 및 판단 |
|---:|---|---|---|
| 1 | 기존 runtime coverage와 새 coverage의 경계 | PASS | 이미 병합된 `fit-content-auto-neighbor-wpt`는 computed `align-content: normal`과 `StretchAutoTracks { applied: true }`를 확인하지만 auto row 하나가 `[10,10] → [10,290]`으로 늘어나는 다른 입력이다. 두 auto row의 균등 분배와 fixed 세 번째 row 유지가 새로 검증될 경계라는 설명은 구체적이다. 근거: `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_neighbor.rs:1-28`, Android host `C0410RuntimeGpuDemo.java:984-1019`. |
| 2 | 규범 oracle의 범위 | PASS | CSS Grid Layout Module Level 1 2025 CRD §11.8을 명시하고, Chrome 및 Spinon/WPT 결과를 규범 자체와 혼동하지 않는다고 적었다. 이 slice의 주장을 해당 알고리즘 단계로 제한한다. |
| 3 | definite free-space 산술 | PASS | `100 - (10+20+5) - (10+10) = 45px`, 두 대상 track에 `22.5px`씩 배분한다. 모든 항이 고정 input에 있고 계산이 재현 가능하다. |
| 4 | 자동 track과 fixed track 구분 | PASS | `auto, auto, 5px` 중 처음 두 auto max track만 늘고 fixed 세 번째 track은 `5px`에 남는다고 고정했다. 기대 track base `[32.5,42.5,5]`가 그 조건을 반영한다. |
| 5 | 단계 순서와 중복 검출 | PASS | `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks`의 단일·정순 검사를 요구한다. 각 단계의 pre-stretch bases와 마지막 적용 stage를 따로 비교하도록 범위를 정했다. |
| 6 | gutter 처리 | PASS | gutter 두 개를 CSS row base 집계에서 분리하고 각각 `10px`인지 확인하도록 했다. 이를 빼먹거나 세 번째 gutter를 잘못 포함하는 경우를 탐지할 수 있다. |
| 7 | 결과 frame의 독립 계산 | PASS | 기대 row starts `0, 42.5, 95px`와 고정 child heights를 이용한 frames가 독립 산술과 일치한다. tolerance도 `1e-4 CSS px`로 명시했다. |
| 8 | root box model과 definite block size | PASS | root는 `160×100px`, border/padding `0`, `box-sizing:border-box`다. 대상 block size 100px이 definite여서 stretch free-space 계산의 전제가 고정된다. |
| 9 | child 순서와 placement | PASS | CSS fixture, `input.json` 모두 `grid-auto-flow: row`이며 DOM 순서대로 세 direct child가 세 explicit row에 놓인다. 직전 편집에서 추가한 `auto_flow` 일치가 확인된다. |
| 10 | Chrome 관찰 viewport 일치 | PASS | 수정된 plan/README/input/observation이 모두 root와 viewport `160×100px`을 기록한다. `chrome156-observation.json`과 PNG도 `160×100`이며 raw root DOMRect `(0,0,160,100)`을 남긴다. |
| 11 | `RootSizingPolicy` 의미 | PASS | precomparison input의 root와 viewport가 모두 `160×100px`이므로 pure Rust oracle의 `Match` 조건과 일치한다. 계획/README는 앱 Grid runtime의 `ResolveWithin`과 다름을 밝히고, root dimensions를 고정해 같은 frame만 기대하며 policy parity를 주장하지 않는다. |
| 12 | Chrome DOMRect 좌표계와 README 서술 | PASS | fixture의 body padding을 0으로 바꾸고 observation helper도 offset 보정을 제거했다. 따라서 JSON의 frame은 raw `getBoundingClientRect()`와 같은 viewport CSS-px 좌표이고 README의 서술과 맞는다. |
| 13 | 병합 Rust oracle과의 geometry parity | PASS | Rust oracle의 inline width `20px`에 비해 앱 precomparison은 `160px`이지만, 최신 plan/README는 시각 캡처를 위한 고정 폭 확장이라고 명시한다. Row-axis tracks/gaps/heights/alignment 값은 oracle과 일치하고 전체 x-frame parity는 주장하지 않는다고 범위를 제한했다. 이 구분은 input과 expected frames에서 확인 가능하다. |
| 14 | Chrome 화면과 관찰 JSON의 연결 | PASS | 갱신된 PNG와 observation은 모두 `160×100`이다. 화면에는 세 row와 두 내부 gutter가 보인다. Chrome observation은 `justifyContent: flex-start`도 기록하고, 계획은 이미지를 보조 표본으로만 취급하며 RGBA pixel equality를 주장하지 않는다. |
| 15 | `justify-content` 입력 parity | PASS | 최신 input/plan/README 및 CSS fixture는 `flex-start`로 통일됐고 Chrome observation의 computed `justifyContent`도 `flex-start`다. Rust oracle의 `LayoutJustifyContent::FlexStart`와 값이 맞는다. |
| 16 | 현재 style projection 및 Taffy 의미 | PASS | `RuntimeBlockFlexGridTrackSizingV1`은 `supports_runtime_flex` 대상이며, `parse_runtime_align_content`는 `normal`을 `LayoutAlignContent::Normal`로 만든다(`style_values.rs:902-923`, `flex_values.rs:85-90`, projection call `style_values.rs:392-397`). Taffy 변환은 `Normal`을 `AlignContent::STRETCH`에 매핑한다(`taffy_style.rs:437-444`). 새 CSS admission은 필요 없어 보인다. |
| 17 | 새 scene runtime의 computed-style 관찰 가능성 | PASS | 계획은 generic runtime report가 새 scene의 computed-style snapshot을 내보내지 않는다고 명시하고, 이 scene의 claim을 explicit `normal` source + track/stage/frame geometry로 제한한다. computed normal→typed projection은 기존 #664 fixture에 둔다. `gh pr view 664`에서 PR은 `MERGED`이며 main source의 fit-content fixture는 computed `alignContent == normal`과 positive stretch stage를 검사한다. 이 경계라면 새 report field를 추가하지 않아도 된다. |
| 18 | scene allowlist와 fail-closed dispatch | PASS | 기존 `c11_4_grid_track_sizing_runtime_source`는 고정 ID만 dispatch하고 FFI eval도 미등록 scene을 오류로 반환한다(`crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:1140-1155`, `ffi/fixtures/c11_4.rs:40-75`). 고정 ID와 JavaScript suffix가 붙은 ID를 구분하는 계획은 해당 경계에 맞는다. 구현 시 fixture source-shape 검사와 Android/iOS scene allowlist도 함께 갱신해야 한다. |
| 19 | trace revision과 플랫폼 strict assertion | PASS | generic trace report는 completed runtime key와 source/style/environment revision 불일치를 거부한다(`trace_report.rs:19-35`). 계획은 Android/iOS 양쪽에서 이 단일 scene의 exact stages/rows/frames, node identity, revision을 검사하도록 명시했고, 기존 host가 scene별 기대값을 두는 구조와 맞는다. 실제 구현에서는 stale draw가 이전 layout revision과 섞이지 않게 현재 scene의 draw/presentation identity도 연결해야 한다. |
| 20 | opt-in 격리 및 완료 주장 경계 | PASS | 계획은 기존 fixture-test 경로를 쓰고 새 FFI ABI를 만들지 않으며 default/product artifact 제외와 fixture-test positive control을 다시 확인한다. simulator/AVD 증거를 실기기·hardware GPU·compositor-after 픽셀로 확대하지 않고 C11.4 및 `72/213`을 유지한다고 제한했다. |

## 재검토 결과

부모 작업자가 width 차이를 캡처용 inline-axis 조정으로 명시하고 full x-frame parity 주장을 제외했다. `justify-content`는 `flex-start`로 일치했으며 최신 Chrome observation에도 computed value가 기록됐다. 따라서 이전 두 PARTIAL은 해소됐다.

이번 재검토는 plan/evidence와 정적 observation만 확인했다. plan이 요구하는 별도 30관점 diff 검토, PR 병합, 구현 단계 검증은 아직 수행하지 않았다.

최신 `SHA256SUMS`의 6개 항목은 모두 통과했다. viewport/root-sizing metadata, 좌표계, inline-axis scope, alignment keyword 관련 이전 지적은 최신 파일에서 해소됐다. 최종 판정은 **20 PASS, 0 PARTIAL, 0 FAIL**이다. 계획 diff 30관점 검토와 계획 PR 병합은 별도 선행 단계다.
