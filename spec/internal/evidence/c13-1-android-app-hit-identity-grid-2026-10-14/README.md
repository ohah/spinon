# C13.1 Android 앱 hit identity grid 사전 비교 · 2026-10-14

## 범위

다음 구현 증분 후보는 기존 Android compact fixture 앱 route에서 고정 52개 hit query를 300×100 CSS integer point 전체로 확장하는 것이다. 이 문서는 코드를 바꾸기 전 Chrome oracle와 입력 경계를 고정한다. Android 앱/FFI 실행 결과나 구현 통과를 기록하지 않는다.

Oracle은 `c13-1-compact-surface-matrix.html`과 해당 JS source를 그대로 쓴다. agent-browser `0.31.1`, HeadlessChrome `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, viewport 300×100 CSS px, DPR 1, scroll `(0,0)`이다. 기존 고정 observation SHA-256은 `515fc95f811399fda9e336a13e7583f2d22555fce4727f1fe7f734981abd0299`, HTML source는 `431f4b4452969adcae80c4e5f89e5e45b1e8756fc3c4d8995b7a5eccfbc6f355`, JS source는 `d36bc1f7d3c791cdc91f6d878c3961e067efc2f88ca1486db8a065c5fd69c760`다.

## Chrome 고정 입력과 관찰

- `capture-hit-identity-grid.js`를 agent-browser `eval --stdin`으로 실행해 `document.elementFromPoint(x, y)`를 정수 CSS 좌표 `x=0..299`, `y=0..99`에서 한 번씩 호출했다. pixel center로 바꾸거나 coordinate rounding을 적용하지 않는다.
- 30,000개 query는 100행의 row RLE 870 run으로 압축했다. node registry 41개, 모든 run이 각 행의 `[0,300)`를 gap/overlap 없이 덮고, 미등록 hit은 0개다.
- 각 query의 topmost fixture node index를 1 byte로 펼친 SHA-256은 `93cccd042e82d8cd30cb7a8ab34a5154073d4a9eccbc4038294d3ca5b9dfce87`다. node histogram 합도 30,000이다.
- 기존 Chrome observation의 52 fixed sample 좌표는 이 전체 grid의 subset에서 모두 같은 `hitId`를 낸다. 전체 grid validator는 source/browser/hash/row coverage와 52 subset을 검사한다. [검증기](validate-hit-identity-grid.mjs) 및 [mutation tests](validate-hit-identity-grid.test.mjs)는 각각 5/5 통과했다.
- [전체 Chrome oracle JSON](chrome155-hit-identity-grid.json), [재현 capture script](capture-hit-identity-grid.js), [검증 로그](precomparison-validation.log), [browser edge probe](fractional-point-edge-probe.json)을 보존한다.

### 정수 좌표 선택과 fractional edge 관찰

첫 시도는 각 CSS 정수 칸의 `(x+0.5,y+0.5)` 중심에서 `elementFromPoint`를 호출했다. 30,000점 중 399점이 Chrome에서 `null`이었다. 이는 오른쪽 마지막 열 100점과 아래 마지막 행 300점의 합에서 우하단 1점을 뺀 수다. 별도 경계 probe에서 viewport/client 크기와 root frame은 300×100 및 `[0,300)×[0,100)`인데, `x=299`는 root ID·`x=299.5`는 null이며 `y=99`는 요소 ID·`y=99.5`는 null이었다. 최초 scan과 집계는 [보존 probe](fractional-grid-initial-probe.json), [분석 JSON](fractional-grid-initial-probe-analysis.json)에 두고 passing oracle로 사용하지 않는다.

[CSSOM View `elementFromPoint`](https://www.w3.org/TR/cssom-view-1/#dom-document-elementfrompoint)는 viewport 밖 좌표에 null을 반환하고, viewport 안에서 hit target이 없으면 root element를 반환하도록 기술한다. 현재 Chrome 결과와 차이가 있어 half-CSS-pixel hit semantics는 별도 미해결 경계로 남긴다. 이번 후보는 기존 C13.1 oracle와 PR #160의 viewport query처럼 정수 좌표를 그대로 비교한다. 이 선택은 fractional coordinate의 규범 동작을 통과로 간주하지 않는다.

## 이후 앱 구현의 pass 기준

별도 계획 PR과 20개 계획 실패 관점 검토를 먼저 병합한 다음 코드에 들어간다.

1. 기존 fixture-test 전용 Android compact route에서 Chrome oracle의 30,000 integer coordinate를 exact fixture NodeId로 비교한다. fixed 52 query와 기존 PixelCopy 동작은 그대로 유지한다.
2. 앱/FFI 응답은 bounded summary로 `points=30000`, `matched=30000`, `mismatches=0`, oracle identity SHA를 남긴다. mismatch이면 총수와 앞부분 좌표/expected/actual ID를 보이고, JSON truncation을 success로 오인하지 않는다. 30,000건의 개별 Logcat 행이나 unbounded allocation은 만들지 않는다.
3. Rust fixture test는 oracle rows의 gap/overlap, unknown/duplicate node, source/browser/hash 변조를 거부하고 Android report가 사용한 매핑 함수를 검증한다. 단일 좌표·edge·clip fallback 변경 mutation은 실패해야 한다.
4. Android API 37 ARM64 AVD의 `spinon_api37_1_compat16k` route를 실행해 동일 run의 `layout=ready`, 41 frames, 52 fixed queries, 30,000 full-grid matches, 52 PixelCopy samples와 frame/surface generation 연결을 남긴다. 전체 화면과 원본 PixelCopy를 보존한다.
5. FFI fixture symbol/payload는 기존 `c13-1-overflow-clip-fixture-test` opt-in에만 둔다. 기본 artifact 부재와 opt-in artifact 존재를 확인한다. route는 기존 단일 compact route와 충돌하지 않는다.
6. paint pixel-center 30,000 비교, app hit identity integer grid 30,000 비교, host full viewport 1,408,000 query는 별도 oracle이며 수치를 합산하거나 서로 대신하지 않는다.
7. 실제 구현 PR 본문에 Android app 실행 화면을 inline 첨부하고 agent-browser로 이미지 로드와 natural dimensions를 검사한다. 이 단계는 Android AVD 통합에 한정되어 iOS runtime hit matrix, fractional CSS coordinates, DOM event dispatch, WPT와 C13.1 완료를 주장하지 않는다.

고정 Chrome oracle 화면은 [기존 compact surface reference](../c13-1-compact-surface-matrix-2026-10-14/chrome155-surface.png)다. 이 파일은 browser 기준 화면이며 Spinon 앱 실행 화면이 아니다.
