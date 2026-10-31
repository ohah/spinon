# C11.4 anonymous Grid text `tab-size` 사전 비교

## 목적과 범위

이 비교는 C11.4의 anonymous Grid text `white-space:pre` fixture에 추가할 `tab-size` 값 모델의 사전 기준을 고정한다. 비음수 unitless number, absolute length, zero, fractional stop, `0.5ch` 대체 규칙, 연속 tab, 강제 줄바꿈, min-content와 고정 track overflow를 포함한다. Spinon 지원, 일반 CSS admission, WPT 적합성 또는 C11.4/C11 완료를 주장하지 않는다.

## 규범 근거

[CSS Text Level 3 §4.2](https://www.w3.org/TR/css-text-3/#tab-size-property)는 `tab-size`를 `<number [0,∞]> | <length [0,∞]>`로 정의하고 상속되며 초기값은 8이라고 규정한다. 계산값은 지정한 number 또는 absolute length다. 현재 초안에서 `<length>` 값은 at-risk로 표시된다. 이는 W3C 절차상 상태 표기이며 기능이 제거된다는 뜻으로 단정하지 않는다. [§4.1.2](https://www.w3.org/TR/css-text-3/#white-space-phase-2)는 zero-width tab을 렌더하지 않고, 다음 tab stop까지 거리가 `0.5ch`보다 작으면 그 다음 stop을 사용하도록 정의한다.

고정 Ahem 글꼴은 10px다. Agent Browser에서 `1ch`와 보존된 U+0020 advance가 각각 10 CSS px로 측정돼 threshold는 5 CSS px다. unitless 값은 U+0020 advance의 배수이며 absolute length는 computed CSS px다. LF와 CRLF는 다음 줄 content origin으로 tab 위치를 초기화한다. 예상값은 이 고정 metrics와 tab-stop 산술로 계산했으며 Chrome의 CSSOM/geometry 값을 복사하지 않았다. `0.3`과 `0.3px`은 이진 부동소수점으로 정확히 표현되지 않는 값의 대조다.

Grid 컨테이너의 child text sequence가 공백 문자만 포함하면 anonymous grid item으로 렌더되지 않는다([CSS Grid Layout Module Level 2 §6](https://www.w3.org/TR/css-grid-2/#grid-items)). 따라서 tab-only row는 각 값의 CSSOM과 fixture 경계를 보여주는 0×0 억제 대조군이다. 이는 text-layout positive input이나 tab advance 관측으로 세지 않는다.

Chromium mainline 소스의 [`ShapeResult::CreateForTabulationCharacters`](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/fonts/shaping/shape_result.cc#1844)는 연속 tab run의 첫 tab에 현재 위치 기반 advance를 적용하고 뒤 tab에 기본 간격을 더한다. [`Font::TabWidth`](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/fonts/font.cc#307)는 half-space 대체 규칙을 처리한다. 이 소스는 unpinned mainline 참고 자료다. Chrome 156 관측을 별도로 보존하며, 해당 바이너리의 정확한 Chromium revision과 mainline 소스가 같다고 주장하지 않는다.

독립 CSS 기대값은 이상적인 CSS px 폭으로 유지한다. 현재 Chromium [`LayoutUnit`](https://chromium.googlesource.com/chromium/src/+/96b04b8744c3300c980cd87bc41cc3444d463fb0/third_party/blink/renderer/platform/geometry/layout_unit.h#67)은 1/64px 단위이며 text bounds에 쓰이는 `FromFloatEncompassRound`는 시작·끝을 바깥쪽으로 감싼다. 브라우저 기하 비교는 이 관측 좌표 변환을 적용해 `ceil(ideal × 64) / 64`와 대조한다. 예를 들어 `0.3px`의 시작 tab은 이상적으로 0.6px이고 `\tX`의 이상적 폭은 10.6px다. Chrome frame과 Range 폭은 10.609375px로 관측되어 1/64px 바깥 반올림과 일치한다. 이 보정은 CSS tab semantics의 일부로 취급하지 않는다.

## 브라우저 관측

저장소의 [HTML fixture](../../../../tests/fixtures/css/c11/anonymous-grid-text-pre-tab-size-values-precomparison.html)를 Agent Browser HeadlessChrome 156.0.0.0에서 viewport 1280×633 CSS px, DPR 1로 열었다. Ahem 글꼴이 로드됐고 `1ch` 및 U+0020 advance는 각각 10 CSS px였다. 14개 computed value와 10개 실제 text/layout 입력의 140개 양성 관측, 그리고 tab-only Grid 억제 대조군 14개를 합쳐 154개 행을 수집했다.

모든 154개 행에서 `CSS.supports()`와 computed serialization을 확인했다. 전체 frame 폭, 140개 양성 장면의 168개 줄 advance 합, 14개 whitespace-only item의 0×0 억제를 독립 기대값과 비교했다. Blink 1/64px 바깥 반올림을 적용한 뒤 mismatch는 0개다. `0.3px`의 이상적 `10.6px`과 관측 `10.609375px` 차이도 원본 행에 보존했다. 재현 가능한 비교기는 [compare-observation.py](./compare-observation.py), 원본 행은 [chrome-observation.json](./chrome-observation.json), 별도 산술은 [expected-values.json](./expected-values.json)에 있다.

주요 관측:

- `tab-size:0` 및 `0px`에서 `\tX`의 frame은 10×10px이고 tab 자체는 advance를 더하지 않는다.
- `.25`의 첫 tab stop 간격은 2.5px지만 다음 stop까지 거리가 5px보다 짧아 `\tX`에서 5px을 진행한다.
- `0.3` number는 3px interval이고 `0.3px` length는 0.3px interval이다. 후자는 이상적 advance와 1/64px browser geometry를 분리해 비교했다.
- `2px`에서 시작 tab은 4px 진행한다. 다음 stop까지 2px로 5px threshold보다 짧기 때문이다.
- `1.5em`은 `15px`, `calc(2px + 3px)`는 `5px`로 계산된다.
- `X\t\tX`는 `.25`에서 27.5px, `2px`에서 26px다. 이는 Blink의 연속 tab run 처리와 일치한다.
- LF/CRLF reset, trailing tab, min-content, 25px fixed track의 soft-wrap 없는 overflow를 각 값에서 비교했다.
- tab-only direct Grid text는 text box를 만들지 않는다. 이는 CSS Grid item suppression이며 tab advance가 0이라는 뜻이 아니다.

[chrome-156-agent-browser.png](./chrome-156-agent-browser.png)는 전체 페이지 browser screenshot이다. 글리프 ink나 플랫폼 pixel 동등성을 증명하지 않는다.

## 해시

| 입력 | SHA-256 |
| --- | --- |
| Fixture HTML | `c25b3849765e0e6f9f52be1fa9ee338c6be69c3d8f35e7ee2e2d04d8374cb39c` |
| 저장된 Ahem | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| 독립 기대값 | `72865dc5af4cdd18b1860700a6f2f81875b07bc0813a2f360885c3b6a9718b17` |
| 비교기 | `8cec7c61d4ab9ecb37ff94c163a44567b8f2c995701283d1b2c7be4337bcc3bf` |
| Raw Chrome observation | `40e2de6dd134ae9934301ef87c65d61b114feb84e7049529ca61565bf99bad2a` |
| Screenshot | `de4645eb26c2ceb5c3f04d08bbb3125a2475027947e1bd2d2ad07b8d279a115d` |

## 재현

```sh
agent-browser open file:///Users/yoonhb/Documents/workspace/spinon/tests/fixtures/css/c11/anonymous-grid-text-pre-tab-size-values-precomparison.html
agent-browser wait --fn 'Boolean(window.__spinonOracle)'
agent-browser eval --stdin <<'JS' | jq -r . > spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/chrome-observation.json
JSON.stringify(window.__spinonOracle, null, 2)
JS
agent-browser screenshot --full spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/chrome-156-agent-browser.png
agent-browser close --all
python3 spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/compare-observation.py
```

이번 관측 뒤 Agent Browser 세션을 닫았다. Terminal Browser는 사용하지 않았다.
