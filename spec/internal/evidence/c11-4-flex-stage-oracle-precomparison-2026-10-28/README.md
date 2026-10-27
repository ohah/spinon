# C11.4 §11.7 flexible track expansion 사전 비교

이 자료는 **계획 단계**의 test-only Rust trace oracle이다. Spinon 구현·app runtime 통과 결과가 아니다. 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §§11.7–11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)이며 Chrome 156 CSSOM/DOMRect는 별도 browser control로 둔다.

## 입력과 독립 기대값

[CSS fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-flex-stage.html)는 definite 200px track axis와 10px internal gap을 가진 네 빈 Grid다. 두 case는 Columns, 두 case는 Rows를 사용하고, 반대 축은 definite 120px이다. Fixture는 HTML/CSS를 browser에서 관찰하는 기준 입력이며 Spinon test runner 입력이 아니다.

`derive_expected.py`는 CSS Grid §11.7.1의 fr-size 계산과 undersized flexible-track freeze loop를 `fractions.Fraction`으로 독립 재현한다. `available_size - gap - non-flexible bases`에서 남은 공간을 구하고, flex factor sum을 최소 1로 처리한다. 어떤 `factor × fr-size`가 track base보다 작으면 그 track을 inflexible로 고정하고 남은 tracks로 계산을 반복한다. [`expected.json`](./expected.json)은 그 계산 결과다.

| Case | Independent track bases | Free space after sizing |
| --- | --- | ---: |
| `weighted-columns` | `[190/3, 380/3]px` | 0px |
| `frozen-row` | `[80, 110]px` | 0px |
| `subunit-columns` | `[85, 20]px` | 85px |
| `zero-fr-row` | `[24, 166]px` | 0px |

## Chrome 관측

Agent Browser `0.31.1`의 HeadlessChrome `156.0.0.0`, viewport `1280×633 CSS px`, DPR 1에서 fixture를 열었다. 네 track list와 item DOMRect는 다음과 같다.

| Case | Chrome track list | Item sizes |
| --- | --- | --- |
| `weighted-columns` | `[63.3281, 126.672]px` | `[63.328125, 126.671875]px` |
| `frozen-row` | `[80, 110]px` | `[80, 110]px` |
| `subunit-columns` | `[85, 20]px` | `[85, 20]px` |
| `zero-fr-row` | `[24, 166]px` | `[24, 166]px` |

첫 case의 exact CRD values는 `[63⅓, 126⅔]px`이며 Chrome DOMRect 차이는 각 약 `0.00521 CSS px`, 1/64px 이하다. [원본 관측 JSON](./agent-browser-observation.json)과 [Agent Browser 화면](./chrome-flex-stage.png)을 보존했다. 화면은 Chrome이 렌더링한 reference fixture이며 Spinon 화면이 아니다. Agent Browser session 및 background process는 확인 뒤 종료했다.

## 사전 판정 기준과 범위

- Rust `ExpandFlexibleTracks` snapshot의 CSS `base_size`는 independent rational values에 대해 절대 오차 `≤1e-4 CSS px`여야 한다.
- Chrome final DOMRect는 independent rational geometry에서 `≤1/64 CSS px` 이내다. Chrome quantization을 Rust 기대값으로 사용하지 않는다.
- 각 입력은 root/viewport sizing axis가 같고, 단일 해당-axis pass, 고유 flex stage, 두 CSS tracks, internal gap 10px와 두 boundary gutters를 보존해야 한다. Trace의 vector 내부 인덱스 값에 의존하지 않는다.
- `growth_limit`, `infinitely_growable`과 선택된 fr fraction은 CSS expected trace field로 주장하지 않는다. 이 제한 test는 stage의 최종 CSS track base만 판정한다.
- Indefinite free space, min-content constraint, item span, intrinsic flex base, repeat, 일반 CSS admission, app runtime, simulator, WPT와 표면 픽셀은 실행하지 않았다. C11.4 전체 및 공식 `72/213`은 미완료다.

## 재현

```sh
python3 spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/derive_expected.py
repo_root="$(git rev-parse --show-toplevel)"
agent-browser open "file://${repo_root}/tests/fixtures/css/c11/grid-track-sizing-c114-flex-stage.html"
agent-browser screenshot spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/chrome-flex-stage.png
```

- Fixture SHA-256: `05998ac1d2320b5e0fdcbff8a0e02ca9f2423fdf8c977e4f3695d7eb90bbc0fe`
- Oracle script SHA-256: `d7b3dac83055bd0e4ee2ea6418721a25045199767d05b25e973027683f3bcc5b`
- Expected JSON SHA-256: `07bedf1b3366ef9252cc80eb123c44a14dab50d876c24f398441f3baa9529000`
- Observation JSON SHA-256: `c09e90138ef4e4697f219e19efa118c97a02c9fa54a6c1cbba0bf465551d5a1b`
- Screenshot SHA-256: `682e486c638076a8e16b6911c61bb02d81c5911e1323901e43fb29d4e7634962`
