# C11.3.3 implicit track-list cycling · Chrome precomparison

- Fixture: [`grid-template-areas-auto-track-list-cycling-c1133-runtime.html`](../../../../tests/fixtures/css/c11/grid-template-areas-auto-track-list-cycling-c1133-runtime.html), SHA-256 `6080bf1675f1ff292b516ea1d8907631a09bc3dd1603f0df7c8d658e93fc32bb`.
- Browser: Agent Browser 0.31.1 with HeadlessChrome 156.0.0.0, DPR 1, viewport 1280×633.
- Coverage: six scenes, 28 nodes, computed used row/column tracks, and root-relative item frames.
- Observation: [`chrome/observation.json`](./chrome/observation.json), SHA-256 `d5b338301c82502e710e3abb0839e5cb30e9dab9976be8d12add277eb4e2bb8b`.
- Independent CSS Grid §7.6 expectation: [`expected.json`](./expected.json), SHA-256 `8f64d82feabc8cc177aaa591eede328511bf2729b4afaa56ae8d1406fa297797`.
- Screenshot: [`chrome/fixture.png`](./chrome/fixture.png), SHA-256 `3df254c19ede58efae3072a10225113e941586d148cc741e7cad3040611261ed`.

The continuation cases place after an area named end line through the existing `area-name-end / span 1 ghost` named implicit-line route. Each later row or column must use the second value of its two-value implicit track list. Numeric implicit-line placement is outside this slice.

This pins a local browser oracle for the bounded implementation. It is not WPT conformance, physical-device output, or general CSS Grid support evidence. The Agent Browser session and local HTTP server were closed after capture; no Agent Browser process remained.
