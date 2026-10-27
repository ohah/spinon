# C11.4 · direct element-text intrinsic 입력 앱 장면 matrix 계획

**상위:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [direct element-text 입력 계획](./c11-4-generic-direct-text-intrinsic-input.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 구현·시뮬레이터 검증 완료, implementation/runtime 20관점 및 PR diff 20관점 검토 완료; PR 생성·병합과 병합 후 상태/로드맵 sync 대기. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여섯 장면이 각각 6/6 통과했다. 이 근거는 여섯 고정 positive를 같은 opt-in generic collector와 intrinsic text helper 경로로 확인한다. 일반 CSS text admission, 글리프 paint, 전체 Stage A 또는 C11.4 완료를 뜻하지 않는다. 공식 완료 `72/213` 및 C11 체크박스는 유지한다.

## 목적과 결손

[기존 direct element-text 계획](./c11-4-generic-direct-text-intrinsic-input.md)은 `normal`, `nowrap`, `break-all`, content 변형, 연속 공백, DOM 순서/style owner의 여섯 positive를 HostDocument/Stylo/C15 Rust tests에서 검증했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에는 그중 `generic-root-normal`만 등록했다. 따라서 나머지 다섯 입력은 generic collector와 typed intrinsic input을 실제 V8 document revision, opt-in FFI adapter, native app matcher까지 통과하는지 아직 확인하지 않았다.

이번 slice는 저장소에 이미 고정한 [Chrome 156 observation](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md)과 Ahem bytes를 그대로 사용한다. 브라우저에서 새 값을 수집하거나 이전 관찰을 구현 결과로 다시 쓰지 않는다. 제품 CSS admission이나 renderer text paint는 범위에 넣지 않는다.

## 고정 장면과 독립 기대값

각 scene은 고유 fixture route ID를 쓰되, DOM root/item ID와 최종 child order는 기존 HTML/Rust case와 대응시킨다. Chrome observation은 세로로 쌓인 페이지 좌표이므로 앱의 root-local frame과 비교할 때 root y를 빼고, root 자체는 `(0,0)`이어야 한다.

| 앱 scene ID | app root ID / source order | Columns | item intrinsic input: width min/max; height min/max | root / item frame (CSS px) |
| --- | --- | --- | --- | --- |
| `generic-root-normal` (기존 대조군) | `generic-root-normal`, `generic-normal-a` → `generic-normal-b` | `[20,40]` | 두 item 모두 `20/40; 20/10` | root `100×20`; items `(0,0) 20×20`, `(30,0) 40×10` |
| `generic-root-nowrap` | `generic-root-nowrap`, `generic-nowrap-a` → `generic-nowrap-b` | `[40,40]` | 두 item 모두 `40/40; 10/10` | root `100×10`; items `(0,0) 40×10`, `(50,0) 40×10` |
| `generic-root-break-all` | `generic-root-break-all`, `generic-break-a` → `generic-break-b` | `[10,40]` | 두 item 모두 `10/40; 40/10` | root `100×40`; items `(0,0) 10×40`, `(20,0) 40×10` |
| `generic-root-content-variation` | `grid-9x`, `item-zeta` → `item-3`, 각각 `A BCD` | `[30,50]` | 두 item 모두 `30/50; 20/10` | root `100×20`; items `(0,0) 30×20`, `(40,0) 50×10` |
| `generic-root-whitespace` | `generic-root-whitespace`, 두 원문 ` A  BC ` | `[20,40]` | 두 item 모두 `20/40; 20/10` | root `100×20`; items `(0,0) 20×20`, `(30,0) 40×10` |
| `generic-root-reorder` | `grid-reorder`, 최종 `item-a` (`ABCD`, break-all) → `item-z` (`A BC`, normal) | `[10,40]` | `item-a: 10/40; 40/10`, `item-z: 20/40; 20/10` | root `100×40`; `item-a` `(0,0) 10×40`, `item-z` `(20,0) 40×10` |

각 intrinsic pair는 `width min/max; height min/max` 순서다. 모든 item의 preferred/min/max size는 원 계획의 `auto`/`none` 값이고 margin·padding·border는 각 축에서 0이다. `grid-ws`의 runtime root ID는 Rust fixture가 쓰는 `generic-root-whitespace`이며 source case는 Chrome HTML의 `grid-ws`다.

최종 C15 line-advance 배열도 기존 `cases.rs` 기대값을 따른다. nowrap은 두 item 모두 `[40]`; break-all은 첫 item `[10,10,10,10]`, 둘째 `[40]`; content variation은 `[10,30]`과 `[50]`; whitespace는 `[10,20]`과 `[40]`; reorder는 `[10,10,10,10]`과 `[40]`이다. `Rows` contribution trace에서는 각 item의 최종 assigned width를 반영하고 span 1·flex-crossing false를 확인한다. 이 고정 cases에서 node별 `Minimum`, `MinContent`, `MaxContent` 행 기여는 각 item final line layout의 block size와 일치해야 한다.

모든 scene은 `100px` definite-width Grid, `min-content max-content` columns, `10px` column gap, auto rows, Ahem `10px` font/line-height, zero item edges와 row gap을 유지한다. 위 frame·track 값은 기존 Chrome observation과 독립 Ahem 산술에서 가져온다. `Rows` contribution trace는 definite column width에 맞춘 최종 item text layout과 대조하며, callback 호출 순서는 기대값으로 고정하지 않는다.

## 구현 범위

- FFI fixture registry에 나머지 5개 positive scene source를 추가하고 기존 `generic-root-normal` source와 route를 보존한다. Scene 선택은 검증 harness의 최상위 route ID에만 사용한다. generic collector가 root ID, item ID, text literal 또는 scene ID로 분기하지 않음을 Rust regression으로 확인한다.
- Android/iOS allowlist와 strict matcher를 6-scene matrix로 확장한다. 각 case에서 정확한 root/item identity와 child order, source order, HostDocument/style/layout revision tuple, `TextIntrinsicInlineSizes`의 font registry revision, raw direct Text source, typed min/max/preferred/min/max-size/edge input, Columns/Rows trace, 최종 item frame, draw/present를 검사한다. `grid-ws`는 raw `" A  BC "`와 collapse 결과를 따로 검사하고 `grid-reorder`는 `item-a`의 `break-all`이 `item-z`에 잘못 적용되지 않는지 확인한다.
- 같은 source commit에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에 6/6 scene을 각각 실행한다. `generic-root-normal`은 회귀 대조군으로 매트릭스에 포함한다. 입력/style/revision 불일치 또는 알 수 없는 scene은 성공 frame이나 draw를 내기 전에 거부한다.
- representative scene 한 개 이상씩 Android/iOS 화면 캡처와 source/build hash를 보존한다. 화면은 배치와 app route만 보여주며 text glyph paint나 surface pixel equality로 해석하지 않는다. `runtime_text_count=0` 또는 `glyph_batch_draw_count=0`이면 이를 명시한다.
- 기본/product static FFI archive와 opt-in fixture-test positive control의 신규 scene ID/fixture symbol 경계를 대조한다. 이 결과를 final APK/IPA fixture exclusion으로 확대하지 않는다.
- Rust generic collector tests 및 관련 C11.4/C15 regressions, FFI route tests, `cargo fmt --all -- --check`, workspace Clippy를 실행한다. 기준선 실패와 이번 변경 유발 실패를 분리한다.
- 성공·실패를 포함한 구현/runtime 리뷰 20관점과 별도 PR diff hostile review 20관점을 작성한다. 계획 검토 관점은 재사용하지 않는다. 실패를 수정하면 영향받은 관점을 다시 실행한다.

## 명시적 제외

- 기존 Rust-only error/rejection controls를 새 Android/iOS scene으로 전부 승격하지 않는다. 미지원 source, mixed image/text, stale revision 및 paragraph property 거부는 기존 계획에서 고정한 Rust 검사를 유지한다.
- 13개 legacy `TEXT_CASES` fallback 각각의 runtime 실행, arbitrary HTML/script parsing, anonymous/nested text grouping, CSS `order`, arbitrary fonts, aspect-ratio/box-edge transfer, 일반 text paint, 일반 CSS admission, WPT, 실기기 및 Android hardware GPU는 범위 밖이다.
- iOS Simulator 표면 값은 Metal drawable 제출 전 결과로만 기술한다. OS compositor 이후 parity는 주장하지 않는다.
- C11.4의 전체 Stage A, Stage B/C/D, 공개 CSS Grid 지원, C11 완료 checkbox와 공식 `72/213`은 변경하지 않는다.

## 검증 순서와 merge 조건

1. 이 계획과 저장된 Chrome/Ahem oracle을 서로 다른 실패 관점 20개로 검토한다. 누락을 찾으면 계획과 독립 기대값을 먼저 수정하고 영향 관점을 재검토한다.
2. 계획 PR을 최신 `origin/main`에 Rebase and merge한 뒤 구현 branch를 만든다.
3. 기존 runtime adapter, Android/iOS route, app matcher와 feature gate를 추적해 5 scene의 source→layout→trace assertions를 구현한다. 새 product API 또는 non-fixture admission을 추가하지 않는다.
4. focused Rust/FFI tests, formatter, workspace Clippy, default/fixture archive boundary를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 commit 6/6 scene을 각각 재실행해 증거와 screenshot을 보존한다. Android/iOS app validation 뒤 해당 app process를 종료한다.
5. 새로운 실패 관점 20개로 구현/runtime을 검토하고, implementation review와 별개인 20관점 PR diff 검토를 통과시킨다. 발견 수정은 영향을 받는 관점에서 재확인한다.
6. PR 본문에는 실제 검증한 시뮬레이터 화면을 이미지 링크로 첨부한다. 현재 GitHub required checks·PR check rollup을 확인한다. 필요한 check가 없다면 CI 통과로 기술하지 않는다. 최신 base, mergeability와 남은 범위를 다시 읽고 Rebase and merge한다.
7. 병합 후 `spec/STATUS.md`, `spec/public/roadmap.html` 및 실행 근거를 실제 merge source에 맞춰 갱신한다. C11.4/C11 상위 상태와 공식 `72/213`은 유지한다. Tailnet 미리보기는 문서 sync PR 뒤 제공 route/hash와 Agent Browser 화면을 확인하고 해당 Agent Browser session을 닫는다. Terminal Browser는 사용하지 않는다.

## 완료 판정

기존 normal control과 다섯 신규 scene이 같은 source revision의 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 모두 통과하고, 각 입력·text intrinsic source/revision·axis별 trace·frame·draw/present가 고정 기대값과 맞아야 한다. default/opt-in archive 대조, 구현/runtime review 20관점, 독립 PR diff review 20관점과 병합 후 상태/로드맵 대조도 필요하다. 통과해도 이는 direct-text Stage A 입력 경로의 6-scene runtime evidence이며 Stage A 전체 또는 C11.4/C11 완료가 아니다.

## 검토 기록

- 계획 독립 실패 관점 20개: [검토 보고서](./c11-4-generic-direct-text-runtime-matrix-plan-review-20.md).
- 계획 PR 변경 검토 20개: [PR diff 검토 보고서](./c11-4-generic-direct-text-runtime-matrix-pr-diff-review-20.md).
