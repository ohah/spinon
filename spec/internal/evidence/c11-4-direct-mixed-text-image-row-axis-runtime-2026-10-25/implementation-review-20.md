# C11.4 row-axis 구현/runtime 검토 · 20개 실패 관점

**검토일:** 2026-10-25

**범위:** `direct-row-min-content-forward/reversed`, `direct-row-max-content-forward/reversed`의 Rust runtime adapter, FFI scene source/route, Android·iOS host adapter와 row-only capture harness를 소스만 읽어 검토했다.
**실행 상태:** 빌드, 테스트, AVD/Simulator, Agent Browser를 실행하지 않았다. 아래 소스 검토 결과는 runtime pass를 뜻하지 않으며 Android/iOS 실행 근거는 미수집이다.

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
7. **Root style mutation 경계** — 재검토에서 row scene 전용 root `direction:ltr`/`writing-mode:horizontal-tb` guard와 FFI 고정을 확인해 소스 누락은 닫혔다. row root는 `justify-items`에 기대지 않고 item별 self-alignment를 지정한다. 변형 입력 거부 실행은 대기다.
8. **Item style 및 placement mutation 경계** — runtime item guard는 explicit column/row line, span 1, auto dimensions, static positioning, zero edges, Ahem metrics와 image intrinsic sizing을 검사한다. 다만 이 guard 자체의 양성·음성 mutated-input 실행은 수행하지 않았다.
9. **Intrinsic input identity와 revision** — Android·iOS adapter는 grid/node/sourceOrder/kind, text font registry revision 1 및 text metrics, image natural metadata, `layoutInputRevision.intrinsicSize=1`을 비교한다. 실제 diagnostics 값은 Simulator/AVD 로그로 확인해야 한다.
10. **Rows contribution과 trace 순서** — 재검토에서 Android·iOS row trace helper와 row capture harness가 각 pass의 모든 stage를 검사하며, 선언된 non-gutter track 수만 허용하고 collapsed non-gutter track은 거부하며 gutter는 유한한 0 크기만 허용하는 것을 확인했다. 각 track의 `gutter`/`collapsed` boolean 형식과 finite `baseSize`도 확인한다. 열 `[50,24]`, 행 `[20]`, `Columns`→`Rows`, row text/image 기여 20/16 및 scene별 contribution kind를 요구한다. 기존 column scenes는 strict row-shape 옵션을 끄고 이전 active-track 비교를 유지한다.
11. **Root·item frames와 box 수** — Android·iOS는 root 74×20, text `(0,0) 50×20`, image `(50,0) 24×16`, 총 3 boxes를 scene별로 확인한다. forward/reversed 로그의 node 순서까지 기대값이 구분된다.
12. **FFI dispatch 경로** — `c11_4.rs`는 새 row module을 선언하고 기존 scene source dispatcher보다 row source resolver를 추가로 호출한다. FFI `fixtures` module은 `c04-runtime-gpu-fixture-test` cfg 아래 연결된다. 실제 opt-in artifact 포함 여부는 미검증이다.
13. **알 수 없는 scene 거부** — 새 row module은 자체 네 ID만 반환하고 unknown ID는 `None`을 돌려준다. FFI wrapper에는 unknown scene 오류 경로가 있으며 새 변형이 그 경로를 우회하는지 테스트는 실행하지 않았다.
14. **기존 #611 column 회귀** — 기존 column scene 값과 scene predicate는 별도로 유지되고 row scenes는 별도 runtime enum/FFI module 경로를 사용한다. 소스 대조상 기존 기대값 변경은 보이지 않지만, 회귀 테스트와 캡처는 실행 전이다.
15. **Rust runtime 테스트의 범위** — `c11_4_mixed/tests.rs`의 새 테스트는 SCENES의 ID, child order, placement metadata와 기대 track 배열을 확인한다. 실제 `prepare` 후 CSS projection, computed contribution, root/item frame, `Columns→Rows` diagnostics를 실행해 검증하는 내용은 이 파일에 없다. 해당 runtime 결과는 별도 실행 gate로 남긴다.
16. **Android adapter의 fail-fast 동작** — Java adapter는 네 ID를 allowlist에 추가하고 row-specific root/item/node frames, intrinsic inputs, source revision 및 trace를 기존 C11.4 assertion 경로에 연결한다. 실제 Android API 37 AVD에서 launch·summary·draw/present는 미확인이다.
17. **iOS adapter의 독립 검사** — Swift adapter도 네 ID, node frame, root/item frame, source/revision, track pass와 row contribution을 별도로 확인한다. Android 결과와 함께 하나의 성공으로 합산하지 않으며 실제 iOS 26.2 Simulator 로그는 미수집이다.
18. **Browser 입력과 runtime CSS parity** — 최초 검토에서 보인 alignment 차이는 수정됐다. FFI row root는 `horizontal-tb`/`ltr`이고 text는 `stretch/start`, image는 `start/start`를 명시해 Chrome 고정 HTML의 해당 layout 스타일과 맞는다. 두 입력의 full style serialization 동일성까지 주장하지 않는다.
19. **Capture provenance와 화면 산출물** — 재검토에서 `sourceInputPaths`에 row FFI/runtime/Android/iOS owners 및 Ahem/PNG owner가 포함된 것을 확인했다. 새 harness는 고유 run prefix/폴더와 log, app/source hash, screenshot signature·dimensions·hash, iOS OCR helper hash를 기록하도록 구성됐지만 capture는 미실행이다. Chrome 156 사전 비교는 oracle 관찰이지 Spinon runtime 결과가 아니다.
20. **Artifact 경계와 완료 상태** — 소스상 fixture route는 opt-in runtime GPU fixture-test module 아래 있으나 product/default negative-control 및 fixture positive-control artifact 비교는 실행하지 않았다. C11.4 전체 checkbox와 공식 완료 `72/213`은 변경하지 않으며, row-axis 구현·플랫폼·artifact gate가 통과하기 전에는 C11.4 완료 근거로 집계하지 않는다.

## 미검증 실행 관문

- focused runtime/FFI 검증과 fail-closed 변형 입력 거부
- 기본/product 및 opt-in fixture-test artifact의 심볼·문자열·payload 대조
- Android API 37 ARM64 16 KB AVD와 iOS 26.2 Simulator의 네 scene별 frames, tracks, contributions, draw/present
- 네 scene screenshot, source/artifact hashes 및 row-only README 증거 수집

위 항목은 이번 review에서 실행하지 않았다.

## 코드 수정 후 재검토 기록 · 2026-10-25

- FFI row source가 root `writing-mode:horizontal-tb;direction:ltr`, text `justify-self:stretch;align-self:start`, image `justify-self:start;align-self:start`를 명시하는 것을 확인했다. 앞서 제기한 Chrome 입력 parity 관점(18)은 소스 수준에서 닫혔다.
- Runtime style owner가 row scenes에 대해 root `direction`/`writing-mode`와 item별 `justify-self`/`align-self`를 scene 종류에 따라 검증하는 것을 확인했다. 기존 column scenes는 `auto/auto` 분기를 유지한다. 앞서 제기한 root style 관점(7)과 parity 변경 뒤 생겼던 FFI/runtime item-style 불일치도 소스 수준에서 닫혔다.
- Capture `sourceInputPaths`에 `c11_4_text.rs`와 `c11_4_image.rs`가 포함된 것을 확인했다. source provenance 관점(19)의 누락은 소스 수준에서 닫혔다.
- 관련 관점은 소스만 재검토했다. Runtime style mutation rejection, 네 scene의 Android/iOS frames·trace·draw/present, screenshots 및 artifact 경계는 여전히 미검증이다. 관점 10의 track-shape 세부 검토는 아래 재검토 기록을 참조한다.

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
