# C11.4 definite percentage auto-repeat neighbors 사전 비교

## 범위와 기준

이 자료는 repeat fragment 앞뒤에 definite percentage track을 둔 제한 auto-repeat 입력의 CSS Grid 기대값과 Chrome 관측만 기록한다. Spinon runtime 결과나 지원 완료 선언이 아니다.

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1, §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat). Auto-repeat count는 content box의 definite size, non-repeating tracks와 gutter를 포함해 계산한다. 이 fixture의 percentage neighbors는 각 axis의 definite content size를 basis로 삼는다.
- 기준 브라우저: Agent Browser의 Chrome 156.0.8078.48 (HeadlessChrome/156.0.0.0), macOS 26.5.1 ARM64, DPR 1. Browser metadata와 raw observation은 같은 디렉터리에 둔다.
- 고정 입력: oracle.html. Definite content size는 240px이며 default gap은 4px. Border-box scene은 136px outer size, 8px padding과 2px border로 content basis 116px을 만든다.
- 13개의 scene 각각에서 CSSOM used track list를 exact track order로 보존한다. Item이 있는 auto-fit 장면은 grid border origin 기준 2×2px item frame도 기록한다. `spec-derived-expectations.json`은 fixture 입력에서 normative count·collapse·gutter·flex 결과를 독립 계산하고 Chrome 관측은 별도 비교한다.
- chrome156-percentage-neighbors.png는 1280×633 viewport screenshot이다. chrome156-percentage-neighbors-full.png는 1280×1532 전체 matrix screenshot이다.
- 2026-10-24 재확인에서 기존 viewport PNG는 전체 화면 PNG의 상단 1280×633 영역과 일치하지 않았다. Agent Browser Chrome 156.0.8078.48, DPR 1, viewport 1280×633, `scrollY=0`에서 [viewport를 다시 캡처](./chrome156-percentage-neighbors-viewport-rerun-2026-10-24.png)했고, 이 이미지는 전체 화면 상단과 RGB pixel 전체가 일치한다. 기존 viewport PNG는 원본 기록으로 보존한다.
- 고정 길이 auto-fit 회귀 대조는 percentage oracle과 분리해 Agent Browser에서 직접 만든 columns/rows scene으로 다시 확인했다. 공유 gutter를 남긴 Chrome CSSOM 값과 frame은 [재대조 JSON](./auto-fit-fixed-neighbor-chrome156-agent-browser-2026-10-24.json)에 있다. 이 결과에 맞춰 Rust C11.4 explicit-neighbor matrix의 162px·170px·77px 및 20px item-origin 기대를 각각 154px·73px·28px로 정정했다.
- validate-precomparison.py는 fixture 입력으로 normative 결과를 재계산하고, scene 수·Chrome track order/value·item frame을 raw observation과 별도 검사한다.

## 독립 계산과 관측

W3C repeat-to-fill 식에 따라 each repeated track을 definite sizing function으로 계산하고 non-repeating percentage tracks 및 모든 gutters를 합산했다. Count는 양의 정수 중 content box를 넘지 않는 가장 큰 값이며, 최소 1회 조건은 별도로 둔다.

**후속 정정 · 2026-10-24:** 이전 독립 계산은 collapsed run 주변 gutter가 전부 사라진다고 가정해 `col-fit-first`/`row-fit-first`를 200px로 잘못 계산했다. CSS Grid 2025 CRD §11.1은 collapsed track 양쪽 gutter가 서로 겹쳐 coincident shared gutter를 만든다고 명시한다. 따라서 prefix/repeat 사이 gap 1개와 collapsed run의 shared gap 1개가 남고, 규범 used track은 `[24,196,0,0,0,12]`다. Chrome 156 CSSOM과 end-alignment 진단(232px)도 이 값에 맞는다. 이전 독립 기대·차이 설명은 아래 보정 결과와 [정정 대조 기록](./normative-chromium-taffy-audit-2026-10-24.md)으로 대체한다. 원래 Chrome observation, screenshot과 capture metadata는 수정하지 않았다.

이 shared-gutter 정정은 percentage neighbor 장면뿐 아니라 고정 길이 auto-fit 회귀에도 적용된다. 200px content axis에서 `20px` prefix와 `10px` suffix, `8px` gap을 두고 repeat track 하나를 채우면 occupied track은 `154px`이다. 연속 빈 run의 shared gap 하나가 남아 repeat-only, 가운데·끝 점유, suffix-only, 2-track span도 CSSOM track·frame에서 같은 계산을 보인다. 해당 matrix는 [Chrome 156 Agent Browser 재대조](./auto-fit-fixed-neighbor-chrome156-agent-browser-2026-10-24.json)에 분리 보존했다.

| Scene | 독립 기대값 | Chrome CSSOM 관측 |
|---|---|---|
| col-fill-both | count 4, [24,46,46,46,46,12] | 동일 |
| row-fill-both | count 4, [24,46,46,46,46,12] | 동일 |
| col-fill-prefix | count 4, [30,48.5,48.5,48.5,48.5] | 동일 |
| row-fill-suffix | count 4, [48.5,48.5,48.5,48.5,30] | 동일 |
| col-fit-first | count 4, [24,196,0,0,0,12] | 동일; start item (38,10) 2×2 |
| row-fit-first | count 4, [24,196,0,0,0,12] | 동일; start item (10,38) 2×2 |
| col-fit-empty | count 4, [24,0,0,0,0,12] | 동일 |
| row-fit-empty | count 4, [24,0,0,0,0,12] | 동일 |
| col-fill-overflow | count 1, [192,40,72] | 동일 |
| row-fill-overflow | count 1, [192,40,72] | 동일 |
| col-fill-zero-prefix | count 5, [0,40.8×5,12] | [0,40.796875×5,12] |
| col-fill-border-box | content basis 116, count 2, [11.6,43.3,43.3,5.8] | [11.59375,43.296875,43.296875,5.796875] |
| row-fill-border-box | content basis 116, count 2, same as above | same as above |

Normative count, track order, collapsed-run shared gutters, used values and item placement are independently derived from the fixture inputs. Chrome track order/count/value and its start-aligned frame are checked against the raw browser observation; all 13 scene values match within the 1/64 CSS px tolerance. The 136px outer box is not the percentage basis; the border/padding-adjusted 116px content axis is. The end-alignment diagnostic does not add runtime alignment support.

![Chrome 156 percentage neighbor matrix](./chrome156-percentage-neighbors-full.png)

## 해석 한계

- 한 고정 Chrome 156/macOS ARM64 관측이다. 다른 browser engine, WPT, Android/iOS app runtime은 이 자료에서 확인하지 않았다.
- Scope is fixed percentage neighbors around a single auto-repeat. It does not cover percentage calc, intrinsic tracks, indefinite/cyclic basis, multiple auto-repeat, general author CSS or C11.4 completion.
- Auto-fit track widths prove post-placement collapse only in the two fixed scenes; they do not prove general auto-placement behavior.
