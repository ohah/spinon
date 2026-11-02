# C11.1 Root HTML Grid `fr` track 사전 비교

## 고정 입력

[HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-fr-precomparison.html)는 parsed Root HTML route에서 처리할 수 있는 CSS source와 Agent Browser precomparison source를 함께 고정한다. Fixture SHA-256은 [expected.json](./expected.json)의 `fixtureSha256` 값이다. CSS Grid 2025 CRD §11.7과 §11.7.1을 기준으로 fraction track을 독립 계산하고, expected 파일에는 계산식과 CSS 0.02px frame tolerance를 저장한다.

## Agent Browser 관측

Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0에서 computed track list와 DOMRect를 수집했다. 각 item frame은 root DOMRect 기준이다. [관측 원본](./chrome156-observation.json)에는 browser user agent, source URL, computed rows/columns/gap, item의 의미 class와 frame을 기록했다. [화면](./chrome-156-agent-browser.png)은 같은 고정 fixture를 보여준다.

| 장면 | Chrome used columns / rows | 독립 기대 |
|---|---|---|
| `column-1-2` | `40px 80px` / `40px` | `(130−10)/(1+2)` → 40px, 80px |
| `column-2-1` | `80px 40px` / `40px` | `(130−10)/(2+1)` → 80px, 40px |
| `mixed-tracks` | `30px 36.65625px 73.34375px` / `40px` | 남은 110px을 1:2로 나눔; Chrome의 1/64 CSS px quantization은 각 rational track에서 0.011px 이내 |
| `row-1-2` | `40px` / `40px 80px` | `(130−10)/(1+2)` → 40px, 80px |
| `subunit-sum` | `25px 25px` / `40px` | flex factor 합 0.5를 1로 계산해 각 25px, 끝에 50px 여유 |
| `zero-factor` | `0px 120px` / `40px` | 0fr track은 0px, 1fr track은 120px |
| `large-factor` | `40px 40px 40px` / `40px` | 세 `1e38fr` 합 `3e38`은 f32에서 유한하고 각 40px |
| `renamed-reversed` | `40px 80px` / `40px` | ID·selector와 semantic source order를 바꿔도 track 비율 유지 |

세 negative control은 CSS 자체가 유효한 `minmax()` track, fraction과 named areas 결합, indefinite row flex axis다. Chrome의 computed geometry도 기록했지만 이 런타임 제한 profile에서는 각 입력을 scene publication 전에 거부해야 한다.

## 큰 flex factor의 명세/브라우저 차이

`large-factor-overflow`는 유한한 `1e38fr` 네 개를 둔다. 독립 계산은 flex factor 합 `4e38`이 IEEE-754 binary32의 최댓값보다 크더라도 각 값을 비율로 나누므로 `[30,30,30,30]px`다. Agent Browser HeadlessChrome 156.0.0.0에서는 computed track과 item 폭이 모두 0px로 관측됐다. 이 차이는 [expected.json](./expected.json)의 `browserDivergences`에 브라우저 관측과 명세 기대를 분리해 기록했다. 구현의 oracle은 CSS Grid 알고리즘이며, 이 Chrome 결과를 기대 출력으로 복사하지 않는다.

Chromium `main`의 [`GridTrackSizingAlgorithm::ExpandFlexibleTracks`](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/third_party/blink/renderer/core/layout/grid/grid_track_sizing_algorithm.cc#1143) 코드는 flex-factor 누산에 `ClampedFloat`를, fr 결과에 `float`를 사용한다. 이는 관측된 한계에 대한 조사 단서이며 Chrome 156 바이너리와 동일한 source revision임을 입증하지 않으므로 인과 확정으로 쓰지 않는다. 규범 계산은 [CSS Grid §11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)과 [§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-find-fr-size)의 단계에 따른다.

## 판정 경계

이 자료는 browser의 CSS layout 관측과 specification에 따른 기대 계산만 증명한다. Spinon parsed HTML runtime, Android/iOS 앱, 표면 픽셀, 일반 CSS admission, WPT 적합성이나 전체 Grid 지원은 아직 검증하지 않았다. CSSOM/DOMRect의 fractional 값은 1/64 CSS px 단위 관측과 독립 rational 기대값을 분리해 비교한다. Screenshot SHA-256: `0c6bb3257509efc997a6640d21fb2993e99b4bae93baedd779102260144c1471`.
