# C11.4 row-axis 구현/runtime 검토 · 20개 실패 관점

**검토일:** 2026-10-25

**범위:** `direct-row-min-content-forward/reversed`, `direct-row-max-content-forward/reversed`의 Rust runtime adapter, FFI scene source/route, Android·iOS host adapter와 row-only capture harness를 소스만 읽어 검토했다.
**실행 상태:** 이 검토에서는 빌드·테스트·AVD/Simulator·Agent Browser를 실행하지 않았다. 이후 전달받은 focused test, Clippy/fmt, artifact scanner 결과는 아래 최신 실행 재검토에 따로 기록했다. Android/iOS 첫 캡처는 pre-fix 실패였고, 수정 후 전체 matrix 실행은 아직 pending이다. 소스 검토와 focused metadata 테스트는 runtime/platform pass를 뜻하지 않는다.

## 초기 검토에서 발견한 차단 항목

1. **Runtime root style의 fail-closed 검사 누락 — 소스 수정 확인, 실행 미검증.** 최초 검토 당시 `validate_grid_style`에 root 방향 검사가 없었다. 아래 재검토에서 row scene 전용 `direction:ltr`와 `writing-mode:horizontal-tb` 검사 및 FFI 입력 고정을 확인했다. root style 변형 거부 자체는 테스트하지 않았으므로 실행 gate는 남는다.
2. **Capture source provenance owner 누락 — 소스 수정 확인.** 최초 검토 당시 `sourceInputPaths`에서 Ahem/PNG owner가 빠져 있었다. 아래 재검토에서 `c11_4_text.rs`와 `c11_4_image.rs`가 입력 목록에 추가된 것을 확인했다. capture/hash는 아직 실행하지 않았다.

## 실패 관점 20개

1. **네 scene ID의 경계** — FFI row module, runtime `SCENES`, Android·iOS allowlist가 min/max × forward/reversed 네 ID를 각각 명시한다. 소스상 목록은 대응하지만 실행 확인은 대기다.
2. **행 keyword 선택** — FFI source는 `grid-template-rows:min-content|max-content`를 scene에 따라 나누고 runtime `SceneGrid`도 min/max row variant를 둔다. 두 결과가 모두 20px라는 기대는 계획 및 Chrome 사전 비교와 일치하며, 이 matrix가 두 알고리즘을 구별한다고 주장하지 않는다.
3. **고정 열과 단일 행** — runtime style guard는 columns `[50,24]`, row function 하나, zero gaps, auto height 및 root width 74를 scene 종류별로 확인한다. 예상 최종 geometry는 root 74×20이다.
4. **명시적 line placement와 source order** — FFI item은 두 열의 1번/2번 line 및 같은 첫 행에 배치된다. runtime scene 배열은 forward/reversed에서 ID와 배열 순서를 모두 바꾸며 Android/iOS 기대 node ID도 그 순서를 따른다.
5. **일반 direct text item topology** — Rust adapter는 root 아래 정확히 두 direct child를 요구하고 text item을 `div`와 단일 text leaf로 제한한다. 익명 Grid text나 추가 descendant가 같은 경로로 받아들여지지 않도록 검사한다.
6. **PNG source·decode·자연 크기** — runtime adapter는 고정 image URL을 확인하고 내장 PNG를 decode한 뒤 24×16 natural dimensions를 검사한다. host adapters는 `replaced`, natural size 및 null aspect ratio를 기대한다. 앱 실행 전 실제 decode 성공은 미검증이다.
7. **Root style mutation 경계** — row FFI root는 `direction:ltr`를 author style로 지정하고 `writing-mode` 선언은 생략한다. runtime guard는 computed `writing-mode:horizontal-tb`를 계속 요구한다. 이 값은 CSS 초기값 경로로 공급된다. `writing-mode`를 명시한 root declaration은 현재 C11.4 author-property allowlist에서 거부되므로, 수직 writing mode 변형은 이 제한 profile에서 입력 단계에 차단된다. row root는 `justify-items`에 기대지 않고 item별 self-alignment를 지정한다. 변형 입력 거부 실행은 대기다.
8. **Item style 및 placement mutation 경계** — runtime item guard는 explicit column/row line, span 1, auto dimensions, static positioning, zero edges, Ahem metrics와 image intrinsic sizing을 검사한다. 다만 이 guard 자체의 양성·음성 mutated-input 실행은 수행하지 않았다.
9. **Intrinsic input identity와 revision** — Android·iOS adapter는 grid/node/sourceOrder/kind, text font registry revision 1 및 text metrics, image natural metadata, `layoutInputRevision.intrinsicSize=1`을 비교한다. 실제 diagnostics 값은 Simulator/AVD 로그로 확인해야 한다.
10. **Rows contribution과 trace 순서** — 재검토에서 Android·iOS row trace helper와 row capture harness가 각 pass의 모든 stage를 검사하며, 선언된 non-gutter track 수만 허용하고 collapsed non-gutter track은 거부하며 gutter는 유한한 0 크기만 허용하는 것을 확인했다. 각 track의 `gutter`/`collapsed` boolean 형식과 finite `baseSize`도 확인한다. 열 `[50,24]`, 행 `[20]`, `Columns`→`Rows`, row text/image 기여 20/16 및 scene별 contribution kind를 요구한다. 기존 column scenes는 strict row-shape 옵션을 끄고 이전 active-track 비교를 유지한다.
11. **Root·item frames와 box 수** — Android·iOS는 root 74×20, text `(0,0) 50×20`, image `(50,0) 24×16`, 총 3 boxes를 scene별로 확인한다. forward/reversed 로그의 node 순서까지 기대값이 구분된다.
12. **FFI dispatch 경로** — `c11_4.rs`는 새 row module을 선언하고 기존 scene source dispatcher보다 row source resolver를 추가로 호출한다. FFI `fixtures` module은 `c04-runtime-gpu-fixture-test` cfg 아래 연결된다. 최신 artifact scanner에서 fixture positive control의 네 marker와 기존 ABI가 확인됐고 product artifact에는 row marker와 ABI가 없었다. 내부 Rust `runtime_source` symbol은 양쪽 symbol 목록 모두에서 발견되지 않았으며 scanner 판정 기준은 아니다.
13. **알 수 없는 scene 거부** — 새 row module은 자체 네 ID만 반환하고 unknown ID는 `None`을 돌려준다. FFI wrapper에는 unknown scene 오류 경로가 있으며 새 변형이 그 경로를 우회하는지 테스트는 실행하지 않았다.
14. **기존 #611 column 회귀** — 기존 column scene 값과 scene predicate는 별도로 유지되고 row scenes는 별도 runtime enum/FFI module 경로를 사용한다. 소스 대조상 기존 기대값 변경은 보이지 않지만, 회귀 테스트와 캡처는 실행 전이다.
15. **Rust runtime 테스트의 범위** — 전달받은 `cargo test --locked -p spinon-runtime ... exposes_four_fixed_mixed_text_image_intrinsic_row_scenes`는 1/1 통과했다. 이 테스트는 `c11_4_mixed/tests.rs`에서 SCENES의 ID, child order, placement metadata와 기대 track 배열을 확인한다. 실제 `prepare` 후 CSS projection, computed contribution, root/item frame, `Columns→Rows` diagnostics 실행을 검증하는 것은 아니므로 해당 runtime 동작은 플랫폼 재실행 gate로 남긴다.
16. **Android adapter의 fail-fast 동작** — Java adapter는 네 ID를 allowlist에 추가하고 row-specific root/item/node frames, intrinsic inputs, source revision 및 trace를 기존 C11.4 assertion 경로에 연결한다. pre-fix Android 캡처에서는 `UnknownProperty`로 첫 scene이 실패해 전체 0/4였으며, 수정 source의 Android API 37 AVD launch·summary·draw/present는 pending이다.
17. **iOS adapter의 독립 검사** — Swift adapter도 네 ID, node frame, root/item frame, source/revision, track pass와 row contribution을 별도로 확인한다. pre-fix iOS 캡처 역시 `UnknownProperty`로 첫 scene이 실패해 전체 0/4였으며, 수정 source의 iOS 26.2 Simulator 검증은 pending이다. 두 플랫폼 결과를 합산하지 않는다.
18. **Browser 입력과 runtime CSS parity** — alignment와 row 계산에 쓰이는 computed `writing-mode`는 Chrome 고정 HTML의 명시값과 runtime의 CSS 초기값이 모두 `horizontal-tb`이므로 이 fixture에서 일치한다. 다만 author input은 다르다. Chrome은 root에 `writing-mode:horizontal-tb`를 선언하지만 runtime FFI는 이 선언이 C11.4 author allowlist 밖이라 제거했다. 따라서 현재 증거는 computed-value parity이며, authored declaration parity나 임의의 사용자 `writing-mode` 지원을 입증하지 않는다.
19. **Capture provenance와 화면 산출물** — 재검토에서 `sourceInputPaths`에 row FFI/runtime/Android/iOS owners 및 Ahem/PNG owner가 포함된 것을 확인했다. Android와 iOS의 pre-fix 첫 캡처는 명시적 `writing-mode:horizontal-tb`에 대한 runtime CSS `UnknownProperty`로 각각 0/4 scene을 통과했다. FFI 선언 제거 뒤 source hash가 달라졌으므로 두 플랫폼의 post-fix 네 scene 캡처, source hash 재생성, screenshots 및 README evidence는 pending이다. Chrome 156 사전 비교는 oracle 관찰이지 Spinon runtime 결과가 아니다.
20. **Artifact 경계와 완료 상태** — 최신 scanner run `20261025T015232Z-4628ce31`에서 product/default negative control은 C11.4 marker 0/4 및 ABI 없음, fixture positive control은 marker 4/4 및 기존 ABI present로 확인됐다. 내부 Rust `runtime_source` symbol이 두 symbol 목록에서 모두 빠진 것은 판정 기준이 아니며, artifact 경계 확인을 무효화하지 않는다. 따라서 이 artifact gate는 통과했지만, Android/iOS post-fix 네 scene 실행, runtime geometry/trace, screenshots 및 README evidence는 pending이다. C11.4 전체 checkbox와 공식 완료 `72/213`은 변경하지 않으며, 남은 row-axis 플랫폼 gate가 통과하기 전에는 완료 근거로 집계하지 않는다.

## 실행 관문 현황 · 2026-10-25

- **통과:** `cargo test --locked -p spinon-runtime ... exposes_four_fixed_mixed_text_image_intrinsic_row_scenes` 1/1, `cargo test --locked -p spinon-ffi ... exposes_only_four_direct_mixed_intrinsic_row_scenes` 1/1. 두 focused crate의 Clippy와 fmt도 통과했다고 전달받았다. 이는 해당 focused test와 정적 검사 결과다.
- **통과:** FFI artifact scanner `20261025T015232Z-4628ce31`: product marker 0/4·ABI 없음, fixture positive control marker 4/4·기존 ABI present. 내부 Rust `runtime_source` symbol은 양쪽에서 미검출됐지만 판정 기준이 아니다.
- **미검증:** fail-closed CSS/style 변형 입력 거부 및 실제 `prepare` 이후 projection, contribution, frames, `Columns→Rows` runtime diagnostics.
- **미검증:** 수정 후 Android API 37 ARM64 16 KB AVD와 iOS 26.2 Simulator의 네 scene별 frames, tracks, contributions, draw/present. 이전 source의 첫 캡처는 양 플랫폼 모두 0/4이며 수정 후 재실행은 pending이다.
- **미검증:** post-fix 네 scene screenshot, source/artifact hashes 및 row-only README 증거 수집.

위 실행 결과는 저장소 소유자가 제공한 기록이다. 이번 review에서는 다시 실행하지 않았다.

## 코드 수정 후 재검토 기록 · 2026-10-25

- FFI row source가 root `writing-mode:horizontal-tb;direction:ltr`, text `justify-self:stretch;align-self:start`, image `justify-self:start;align-self:start`를 명시하는 것을 확인했다. 앞서 제기한 Chrome 입력 parity 관점(18)은 소스 수준에서 닫혔다.
- Runtime style owner가 row scenes에 대해 root `direction`/`writing-mode`와 item별 `justify-self`/`align-self`를 scene 종류에 따라 검증하는 것을 확인했다. 기존 column scenes는 `auto/auto` 분기를 유지한다. 앞서 제기한 root style 관점(7)과 parity 변경 뒤 생겼던 FFI/runtime item-style 불일치도 소스 수준에서 닫혔다.
- Capture `sourceInputPaths`에 `c11_4_text.rs`와 `c11_4_image.rs`가 포함된 것을 확인했다. source provenance 관점(19)의 누락은 소스 수준에서 닫혔다.
- 관련 관점은 당시 소스만 재검토했다. Runtime style mutation rejection, 네 scene의 Android/iOS frames·trace·draw/present, screenshots 및 artifact 경계는 그 시점에는 미검증이었다. 현재 관문 결과는 아래 최신 실행 재검토 기록을 참조한다. 관점 10의 track-shape 세부 검토는 이어지는 재검토 기록을 참조한다.

## Track trace shape 재검토 · 2026-10-25

- **Android:** row scenes만 `requireUncollapsedNonGutterTracks=true`로 전달한다. `c114DirectMixedTrackPassMatches`는 모든 stage에서 `c114DirectMixedTrackStageShapeMatches`를 적용하고, 각 stage의 non-gutter track 수를 기대값과 맞추며 collapsed 또는 non-finite non-gutter track을 거부한다. gutter는 finite `baseSize`가 `0.0001` 이내 0일 때만 통과한다. 기존 column scenes는 이 strict 분기를 `false`로 두어 이전 collapsed/gutter 필터링 동작을 유지한다.
- **iOS:** row scene은 initial/final size 비교 외에도 모든 stage에서 `stageTrackShapeMatches`를 호출한다. 이 검사는 non-gutter track count를 정확히 맞추고 collapsed/non-finite tracks를 거부하며, gutter는 finite 0-size만 허용한다. column scenes는 `requireExactNonGutterTracks=false`여서 기존 active-track 비교를 유지한다.
- **Capture harness:** `checkedFinalTracks`가 모든 stage에서 `gutter`와 `collapsed`가 실제 boolean인지, `baseSize`가 finite number인지 검사한다. 이어 `gutter:false` track 수를 기대 배열 길이와 맞추고 collapsed CSS track을 거부하며, `gutter:true`는 finite zero size만 허용한다.
- 따라서 추가 collapsed non-gutter track 및 malformed gutter/collapsed/baseSize flag 허용 잔여 문제는 소스 수준에서 닫혔다. 기존 column scene trace 동작은 strict row-shape 검증을 적용하지 않아 이전 비교를 유지한다. 이 재검토에서도 실행 검증은 하지 않았다.

### Track flag schema 재검토 · 2026-10-25

- Android strict row helper는 각 track의 `gutter`/`collapsed`가 `Boolean`, `baseSize`가 `Number`이고 유한한지 먼저 검사한 뒤 모든 stage의 정확한 non-gutter 개수·collapsed 상태·zero-size gutter를 확인한다.
- iOS row helper는 모든 stage의 각 track에서 `gutter`/`collapsed`를 `Bool`, `baseSize`를 finite `NSNumber`로 확인하고 동일한 count/size 조건을 적용한다.
- Capture harness는 각 stage에서 두 flag의 `boolean` 형식과 finite `baseSize`를 확인한 뒤 exact CSS track count, uncollapsed 상태, zero-size gutter를 검사한다.
- 세 경로 모두 관점 10의 trace-shape 계약을 소스 수준에서 만족한다. Android/iOS의 기존 column scenes에는 row strict gate가 적용되지 않는다. 어떤 테스트·빌드·시뮬레이터·브라우저 실행도 하지 않았다.

### Row `writing-mode` declaration 실패 재검토 · 2026-10-25

- Android/iOS 첫 scene 캡처에서 당시 row FFI root에 있던 `writing-mode:horizontal-tb` declaration이 runtime CSS의 `UnknownProperty` 오류로 거부됐다. 두 캡처는 이 변경 전 입력의 실패 증거이며, 네 scene 동작의 통과 증거는 아니다.
- 소스에서 원인을 확인했다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs`의 `runtime_block_flex_grid_track_sizing_author_properties()`에는 `direction`은 있으나 `writing-mode`는 없다. 반면 computed-property 목록에는 `("writing-mode", LonghandId::WritingMode)`가 있다. 이 profile은 `crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs`에서 `first_unsupported_author_feature(allowed)`를 확인해 허용 밖 author declaration을 `UnsupportedAuthorCss`로 반환한다. 따라서 computed 값 수집은 가능하지만 author inline 선언은 허용되지 않는 경계다.
- 수정된 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic_rows.rs`는 root의 `writing-mode` declaration을 제거했고, fixture source assertion도 declaration이 없음을 확인한다. runtime `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs`는 여전히 computed root 값이 `horizontal-tb`인지 검사한다.
- 기본값 경로는 고정된 Stylo 0.22.0 소스 `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/stylo-0.22.0/properties/longhands.toml:2679`의 `[writing-mode] initial = "computed::WritingModeProperty::HorizontalTb"`에서 확인했다. Spinon은 `runtime_grid.rs` computed property profile에 `WritingMode`를 넣고, `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs:164-170`에서 각 `LonghandId`의 computed value를 `computed_value_to_string(...)`으로 읽는다. 그러므로 이 fixture의 root computed 값은 author declaration이 없어도 CSS 초기값 `horizontal-tb`로 검사된다.
- 남은 parity 한계: Chrome oracle HTML은 `writing-mode:horizontal-tb`를 author style로 명시하고 runtime FFI는 초기값에 의존한다. 현재 확인되는 일치는 이 고정 fixture의 computed value뿐이다. runtime CSS profile에 해당 author property가 추가되기 전까지는 authored-source parity 및 사용자가 선언한 다른 writing mode의 처리를 보장하지 않는다.
- 이 리뷰에서는 수정된 입력을 재실행하지 않았다. 후속 검증은 Android/iOS 네 scene을 다시 캡처하고 source hash를 갱신한 뒤 geometry, tracks, trace 및 draw/present를 확인해야 한다. 변경 전 첫-scene 캡처만으로 post-fix 통과나 전체 C11.4 완료를 선언할 수 없다.
