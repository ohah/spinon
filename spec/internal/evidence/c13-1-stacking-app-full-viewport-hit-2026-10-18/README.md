# C13.1 중첩 stacking descendant 전체 viewport hit oracle

이 자료는 `visible`·`clip`·`hidden` 중첩 stacking descendant 장면의 Chrome `elementFromPoint()` 결과를 고정한다. 현재는 사전 oracle와 계획 검토만 완료했다. Android/iOS 앱 runtime 전체 grid 연결은 구현되지 않았다.

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

이 파일의 캡처는 브라우저 기준만 제공한다. 앱 runtime, WGPU/Metal surface, DOM event, CSSOM, WPT 또는 C13.1 제품 지원 근거가 아니다.
