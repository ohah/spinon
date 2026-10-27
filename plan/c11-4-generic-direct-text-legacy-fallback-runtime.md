# C11.4 · direct-text `TEXT_CASES` legacy fallback 13-scene runtime matrix

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [generic direct-text collector](./c11-4-generic-direct-text-intrinsic-input.md) · [6-scene app matrix](./c11-4-generic-direct-text-runtime-matrix.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태 · 2026-10-28:** 새 런타임 실행은 하지 않았다. 기존 `TEXT_CASES`의 13개 one-item scene은 FFI registry와 Android/iOS matcher에 이미 등록되어 있다. PR #704가 `TEXT_CASES[0]` 하나에 대해 generic collector의 `None` 이후 `c11_4_text::prepare` 전체 dispatch 결과를 Rust에서 확인했지만, 나머지 legacy case 각각의 collector-to-layout 경로와 13개 Android/iOS app matrix는 아직 source-bound 결과가 없다. 이 계획은 그 범위만 검증한다. 공식 C11.4 checkbox와 `72/213`은 유지한다.

## 목표와 범위

Generic direct-text collector가 아직의 제한 계약대로 one-item legacy scene에서 `Ok(None)`을 내고, `prepare_c11_4_text`의 기존 dispatcher가 해당 입력을 허용된 `TEXT_CASES` 경로로 처리하는지 13개 scene 전부 확인한다. 같은 13개 FFI scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 실행해 computed intrinsic input, Columns/Rows trace, root/item frame, draw/present를 비교한다.

이 작업은 기존 Rust/FFI/app route의 회귀 검증과 test/evidence 보강이다. 일반 HTML parser, generic collector acceptance 확대, 제품 CSS admission, text shaping/paint, 새 public API 또는 layout algorithm은 추가하지 않는다. 검증 중 route나 기대값에 결함이 발견되면 범위를 나눠 별도 수정 관문을 만들고, 이 13-scene pass에 섞어 기록하지 않는다.

## 고정 입력과 독립 기대값

입력 source는 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/cases.rs`의 정확한 13개 `TEXT_CASES`와, `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`의 같은 scene ID/root/item/text source다. Chrome 관찰과 독립 CSS Grid/Ahem 계산은 기존 evidence를 재사용한다. 새 브라우저 관찰은 필요하지 않다.

| Scene | Root content size | Columns / Rows | Item root-local frame |
| --- | ---: | --- | --- |
| `auto-text-max-content` | `120×40` | `[100]` / `[10]` | `(0,0) 100×10` |
| `min-content-text-normal` | `120×40` | `[70]` / `[20]` | `(0,0) 70×20` |
| `max-content-text-normal` | `120×40` | `[100]` / `[10]` | `(0,0) 100×10` |
| `fit-content-text-minimum` | `120×40` | `[70]` / `[20]` | `(0,0) 70×20` |
| `fit-content-calc-limit` | `100×120` | `[15]` / `[80]` | `(0,0) 15×80` |
| `min-content-text-edge` | `120×50` | `[80]` / `[20]` | `(0,0) 80×20` |
| `min-content-text-min-width` | `120×50` | `[90]` / `[20]` | `(0,0) 90×20` |
| `min-content-text-max-width` | `120×50` | `[40]` / `[20]` | `(0,0) 40×20` |
| `min-content-text-break-all` | `120×40` | `[10]` / `[80]` | `(0,0) 10×80` |
| `min-content-text-nowrap` | `120×40` | `[100]` / `[10]` | `(0,0) 100×10` |
| `cyclic-fit-content-column` | `50×20` | `[25]` / `[20]` | `(0,0) 25×20` |
| `cyclic-fit-content-row` | `20×40` | `[20]` / `[20]` | `(0,0) 20×20` |
| `definite-fit-content-column` | `40×20` | `[20]` / `[20]` | `(0,0) 20×20` |

첫 열 개의 Chrome used tracks, root/item dimensions와 text content는 [Chrome 155 observation](../spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.md) 및 [`chrome155-observation.json`](../spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json)에 있다. 세 cyclic `fit-content(%)` scene의 Chrome 156 관찰과 별도 기대값은 [cyclic fit-content precomparison](../spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/README.md) 및 [`chrome156-ahem-observation.json`](../spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/chrome156-ahem-observation.json)에 고정돼 있다. Chrome 값은 기준 구현 관찰이며 CSS Grid의 독립 계산이나 저장된 Ahem font metrics를 대신하지 않는다.

## 검증 설계

1. `generic_direct_text::prepare`에 13개 HostDocument/style fixture를 각각 넣어 `Ok(None)`을 확인한다. 이 scene들은 모두 정확히 한 direct HTML `div` item과 하나의 의미 있는 Text node를 가진다. 둘 이상의 text item을 허용하는 새 generic collector 경로로 들어가면 실패다.
2. 같은 immutable snapshot과 computed-style revision을 `c11_4_text::prepare` 전체 dispatcher에 전달한다. 각 case에서 generic collector 이전의 specialized adapters가 `None`을 반환하고, generic collector도 `None`을 반환한 뒤 `TEXT_CASES` legacy resolver가 `Some`을 반환하는지 분리해 검사한다. Full dispatcher output은 legacy resolver의 paragraph style, Ahem intrinsic width/height, typed Grid input과 `two_axis_nested_grid` mode에 맞아야 한다. Unknown ID, 바뀐 text, 다른 root/item, 잘못된 style은 성공한 부분 input을 만들지 않아야 한다.
3. FFI runtime source가 13개 case의 scene ID, root/item ID, source text와 style을 일치시킨다. Android/iOS matcher는 기존 source에 이미 존재하므로 이를 재사용하되, 실제 실행 로그에서 13개 모두 같은 source commit, `layout=ready`, expected frame/track/trace와 draw/present를 내는지 검사한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 **13/13**을 실행한다. 한 실행 중 scene별 document/style/environment revision tuple이 일치해야 하며, scene 수 누락·중복, stale trace, 잘못된 axis, track/frame mismatch 또는 미등록 scene의 성공은 실패다. 실제 glyph raster나 surface pixel equality는 검사하지 않는다.
5. 기존 app matcher의 min/max-content text intrinsic assertions가 scene별 raw source, Ahem font revision과 final item width에 맞는지 확인한다. App telemetry의 `runtime_text_count=0` 또는 `glyph_batch_draw_count=0`이면 glyph paint를 검증했다고 쓰지 않는다. Representative Android/iOS screenshot은 scene UI 증거로만 연결한다.
6. 변경되는 FFI ABI/source가 없으면 artifact exclusion을 새로 주장하지 않는다. Rust test/runner/docs만 바뀌면 현재 source tree의 변경 범위를 보고 필요한 focused checks만 실행한다. App 검증을 위해 직접 실행한 Android/iOS 앱 프로세스는 run 종료 때 닫고, 이미 켜져 있던 다른 simulator/AVD는 보존한다.
7. 공식 `STATUS.md`와 공개 roadmap의 C11.4 완료 수 및 checkbox를 바꾸지 않는다. 실제 merge source와 13-case evidence를 함께 기록하고, Tailnet preview를 갱신하는 경우 지정된 route만 바꾸고 Agent Browser로 확인한 뒤 해당 Agent Browser session/process를 종료한다. Terminal Browser는 사용하지 않는다.

## 독립 검토와 순서

1. 이 계획 자체를 별도의 20개 실패 관점으로 검토하고, 발견한 기대값·경로 누락을 계획과 근거 index에서 먼저 수정한다.
2. 계획 PR을 `Rebase and merge`로 병합한 뒤 최신 `origin/main`에서 같은 checkout의 구현 branch를 만든다. 별도 worktree나 병렬 subagent lane은 만들지 않는다.
3. Rust full-dispatch matrix와 simulator runner를 순서대로 연결하고 focused tests/formatter/lint를 실행한다. 앱 matrix 뒤 앱 프로세스 종료와 run/source/artifact hash를 확인한다.
4. 새 구현/runtime 20관점 검토와 PR diff의 별도 20관점 검토를 완료한다. 발견한 결함을 수정하면 영향받은 case·matcher·문서의 검토 관점을 다시 실행한다.
5. PR 본문에는 변경된 화면이 있으면 Android/iOS representative image를 immutable commit 링크로 넣고, Agent Browser에서 실제 이미지가 로드되는지 확인한 뒤 그 세션/process를 종료한다. Empty GitHub check rollup은 CI pass로 표현하지 않는다. Merge 후 official status, public roadmap, parent plan과 evidence source SHA를 대조한다.

## 명시적 제외

이 matrix는 현재 고정된 13개 legacy text case만 검증한다. 임의 HTML/text source, `TEXT_CASES` ID 기반 fallback을 일반화하는 변경, direct-text generic positive 6개, anonymous/nested/mixed image text, 일반 CSS admission, WPT pass, glyph paint, physical device, Android hardware GPU, iOS compositor 이후 pixels와 전체 C11.4는 범위 밖이다. 성공해도 C11.4 checkbox나 공식 `72/213`을 올리지 않는다.

## 근거

- [13개 legacy `TEXT_CASES` source](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/cases.rs)
- [generic collector 경계와 fallback 설명](./c11-4-generic-direct-text-intrinsic-input.md)
- [직전 one-case fallback closure](../spec/internal/evidence/c11-4-generic-direct-text-review-closure-2026-10-28/README.md)
- [6-scene generic app matrix 및 재사용 matcher](./c11-4-generic-direct-text-runtime-matrix.md)
- [Chrome 155 baseline](../spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.md)
- [Chrome 156 cyclic fit-content baseline](../spec/internal/evidence/c11-4-cyclic-fit-content-precomparison-2026-10-22/README.md)
