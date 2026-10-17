# C13.1 중첩 stacking descendant 전체 viewport hit oracle

이 자료는 `visible`·`clip`·`hidden` 중첩 stacking descendant 장면의 Chrome `elementFromPoint()` 기준과 앱 runtime 전체 viewport hit identity 실행을 함께 기록한다. Chrome oracle와 계획 검토에 이어 Android/iOS fixture app route도 검증했으며, 공개 CSS 지원이나 C13.1 완료를 뜻하지 않는다.

## 입력과 환경

- Fixture: `tests/fixtures/css/c13/overflow-clip-stacking-descendants-c13-1.html`
- Fixture SHA-256: `6b691ddf5afd44543b438b47200f9a6f85981fa3d3c30e1619185a65d320791e`
- Browser: Agent Browser `0.31.1`; Chrome for Testing `155.0.8059.85` (macOS `26.5.1`, arm64)
- Viewport: `540×128` CSS px, DPR 1, scroll `(0,0)`
- Query: 정수 좌표 `(x,y)`에서 `document.elementFromPoint(x,y)`
- Screenshot: [chrome155-stacking-full-viewport.png](./chrome155-stacking-full-viewport.png), `540×128`, SHA-256 `a9727d516837df6c8a5d77e624aee3463b08ebfb991d8eb622d52d026c64585c`
- Grid oracle: [chrome-full-viewport-hit.json](./chrome-full-viewport-hit.json), SHA-256 `279364905336ef53d225c781bae68ec26d3fe4d50cfc7d0cfce689ab5354eee2`

## 관찰 결과

13개 고정 `data-id`를 정수 node index로 매핑해 69,120개 viewport 좌표를 전부 조회했다. JSON에는 128개 row를 canonical RLE로 보존했으며 1,136 run이 전체 69,120점을 빈틈·겹침 없이 덮는다. 확장 identity byte stream SHA-256은 `866073e0c6f092d4b2459fa3b5cfd2ca38dfb8935f582ee3c9614dba3bfc7483`이다.

| Node ID | hit point 수 |
| --- | ---: |
| root | 39,744 |
| visible-parent / clip-parent / hidden-parent | 각 5,568 |
| visible-layer / clip-layer / hidden-layer | 각 1,536 |
| visible-target | 2,688 |
| clip-target / hidden-target | 각 1,536 |
| visible-overlay / clip-overlay / hidden-overlay | 각 768 |

정수 viewport edge `(539,64)`, `(0,127)`, `(539,127)`는 `root`다. 반면 fractional edge `(539.5,0.5)`, `(0.5,127.5)`, `(539.5,127.5)`는 `null`이다. 네 방향 viewport 밖 control도 `null`이다. 이 fractional 결과는 기존 C13.1 Chrome probe와 맞으며 새 grid는 정수 좌표만 사용한다.

## 검증

```sh
node tools/css-reference/validate-c13-1-stacking-full-viewport-hit.mjs
node --test tools/css-reference/validate-c13-1-stacking-full-viewport-hit.test.mjs
```

validator는 Chrome/version, fixture와 screenshot hash, viewport/DPR/scroll, node order, point·row 수, 각 row RLE 연속성, canonical run, 확장 digest 및 edge controls를 확인한다. 변조 source, browser, viewport, node, point 수, 누락·gap·overlap·인접 run, digest, screenshot, fractional edge control 등 19개 negative mutation과 intact positive control을 검사한다. 현재 결과는 `20/20` pass다.

캡처 입력은 [Agent Browser eval script](../../../../tools/css-reference/capture-c13-1-stacking-full-viewport-hit.mjs), 검증 코드는 [validator](../../../../tools/css-reference/validate-c13-1-stacking-full-viewport-hit.mjs)와 [20개 integrity test](../../../../tools/css-reference/validate-c13-1-stacking-full-viewport-hit.test.mjs)다. 실행 결과는 [validator log](./validator.log), [20/20 test log](./validator-tests.log), [artifact SHA-256 목록](./artifacts.sha256), [screenshot dimension 검사](./screenshot-dimensions.txt)에 보존했다. [구현 계획](../../../../plan/c13-1-stacking-app-full-viewport-hit-identity.md), [계획 실패 관점 검토 20개](./plan-review-20.md)와 [계획 PR 변경 검토 30개](./plan-pr-delta-review-30.md)를 함께 본다.

Chrome screenshot은 브라우저 oracle 화면만 보여준다. 앱 runtime 결과와 화면은 아래 플랫폼별 실행 기록을 따른다.

## 앱 runtime 결과

- 구현 source commit: `1375b7978e6d10d200b8d7b1cc0471dda35e5b73`.
- 같은 540×128 CSS viewport의 전체 정수 좌표 69,120점을 각각 Android/iOS fixture app의 동일 `RuntimeRenderSnapshot`에서 조회했다. 13개 node frame과 기존 18개 hit/RGBA sample은 유지했다. yellow parent/frame 바깥이지만 viewport 안인 descendant·overlay 영역도 grid에 포함된다.
- [Android API 37 ARM64 AVD 실행·화면·PixelCopy](./android-avd/c13-1-stacking-android-20261017T195210986Z-cb1a2410/README.md): `emulator-5554`의 새 process 3개가 각각 hit identity 69,120/69,120, 13/13 frame, 18/18 hit, 18/18 PixelCopy RGBA를 맞췄다. screen capture는 `1080×2400`, PixelCopy PNG는 `1418×336`이며 모두 `wm size`/draw surface dimensions와 대조했다.
- [iOS iPhone 17 Pro / iOS 26.2 Simulator 실행·화면·Metal 표본](./ios-simulator/c13-1-stacking-ios-20261017T195135274Z-ad00912d/README.md): 새 process 3개가 각각 hit identity 69,120/69,120, 13/13 frame, 18/18 hit와 Metal drawable 18/18 sample을 맞췄다.
- 두 플랫폼 모두 evaluation·draw의 generation/document/render-tree/style revision tuple이 `[1,65,13,0]`으로 같다. 서로 다른 process ID와 per-run summary에 APK/app executable SHA-256을 기록했다.
- 각 플랫폼의 stale document revision control은 표면 readback 전에 거부했고, route 충돌 control은 host 초기화 전에 거부했다. opt-in fixture artifact 경계는 [feature-boundary 기록](./feature-boundary/README.md)에 둔다.
- Android PixelCopy와 iOS Metal은 각 18개 고정 표본만 확인한다. 전체 WGPU physical pixel parity, OS compositor/scanout, 실기기/hardware GPU, DOM event dispatch, CSSOM, WPT와 일반 CSS 지원은 미검증이다. C13.1과 C13 상위 상태는 미완료다.

[구현 계획](../../../../plan/c13-1-stacking-app-full-viewport-hit-identity.md) · [내부 계약 0078](../../0078-c13-1-stacking-full-viewport-hit-identity.md) · [구현 실패 관점 20개](./implementation-review-20.md) · [PR 변경 관점 30개](./pr-delta-review-30.md).
