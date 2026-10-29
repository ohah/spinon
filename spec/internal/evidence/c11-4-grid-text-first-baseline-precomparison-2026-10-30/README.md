# C11.4 direct text first-baseline 사전 비교

**판정:** CSS Grid 2025 CRD §11.5의 baseline shim 규칙으로 직접 text Grid item 두 개의 독립 기대값을 계산했고, Agent Browser가 연 HeadlessChrome 156의 네 장면과 일치했다. 이는 Chrome 기준 자료이며 Spinon 동작 검증은 아니다.

## 고정 입력과 환경

- Fixture: [grid-first-baseline-direct-text.html](../../../../tests/fixtures/css/c11/grid-first-baseline-direct-text.html), SHA-256 `abbc3c3f289093e633b34dec18358637aa0f6ec53f7100ab8698cd75019757d8`.
- Font: 저장소 [Ahem.ttf](../../../../tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`; `unitsPerEm=1000`, `hhea ascent=800`, `hhea descent=-200`.
- Browser: Google Chrome `156.0.8078.76`, Agent Browser HeadlessChrome UA `156.0.0.0`, binary SHA-256 `a0cd10cb5c31255959ffce60457762f0cf44709ae5f583a765f4b0b0535071cc`; macOS `26.5.1 (25F80)` arm64; viewport `1280×633`, DPR 1, locale `ko-KR`.
- Capture hashes: observation JSON SHA-256 `d47aa75b22513f57326c8e97f9a5827d67fc44e34dbd7ca553260ea514412d34`; screenshot SHA-256 `acc63999dcf16197e41f4e4ec4138393eee29d29be944af663f587f0b14f5777`.
- Agent Browser는 font load 확인과 관찰·screenshot 수집 뒤 `close --all`로 종료했다. 종료 직후 `doctor --offline --quick`은 active daemon 0개를 보고했다.

## 독립 규범 계산

[CSS Grid Level 1 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)는 baseline-sharing group의 각 item에 start-side shim을 더해 같은 row에서 baseline을 정렬하고 그 shim을 intrinsic contribution에 포함한다.

한 줄 Ahem text의 line-height는 font-size와 같다. 저장소 font의 ascent ratio는 `800 / 1000 = 0.8`이므로 10px/20px line의 first baseline은 각각 line top +8px/+16px이다. 두 item을 같은 baseline에 맞추면 row baseline은 16px이고 shim은 작은 item 8px, 큰 item 0px이다. auto row의 최소 크기는 `max(10+8, 20+0) = 20px`. 각 glyph width는 두 글자이므로 columns는 20px·40px, gap을 포함한 grid 폭은 68px다.

## Agent Browser 관찰

고정 결과는 [raw observation JSON](./chrome-observation.json), [화면](./chrome-156-grid-text-first-baseline.png), [독립 expected JSON](./expected-values.json)에 저장했다.

| Scene | Grid columns / row | item y from row start | glyph baselines from row start |
|---|---:|---:|---:|
| `align-items:first baseline` | `20px 40px` / `20px` | `8px`, `0px` | `16px`, `16px` |
| `align-items:start` control | `20px 40px` / `20px` | `0px`, `0px` | `8px`, `16px` |
| two `align-self:first baseline` items | `20px 40px` / `20px` | `8px`, `0px` | `16px`, `16px` |
| reversed source order | `40px 20px` / `20px` | `0px`, `8px` | `16px`, `16px` |

모든 observed geometry와 baseline이 independently derived values와 일치했다. Screenshot의 빨간 guide는 expected row baseline 16px에 놓은 시각 확인선이며 layout 입력 크기에는 영향을 주지 않는다.

## 한계

관찰은 pinned Ahem font와 네 고정 scene에 한정한다. Spinon Rust/runtime, Android/iOS Simulator, public CSS admission, 임의 text/font/wrapping, WPT 실행 또는 C11.4 전체를 검증하지 않았다.
