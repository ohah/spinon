# C11.4 fixed-repeat `minmax(px, fr)` 사전 비교

## 범위와 기준

CSS Grid Level 1 [2025 Candidate Recommendation Draft §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)는 fixed-size repeat 안에 `minmax(<fixed-breadth>, <track-breadth>)`를 허용하고, 같은 track list에 auto-repeat은 하나만 둘 수 있도록 한다. [§7.2.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#fr-unit)는 0fr과 1 미만 factor를 허용해 남는 공간을 비워 둘 수 있도록 한다. §11.7–§11.7.1의 flexible track algorithm은 최소값보다 작은 hypothetical flex fraction을 가진 track을 고정하고 남은 공간으로 다시 계산한다. 이 문서는 CRD를 기준으로 한 제한 fixture의 독립 계산을 담으며, 최종 CSSWG 합의나 일반 CSS 지원을 주장하지 않는다.

현재 `track_size()`는 `minmax(12px, 1fr)`를 typed `MinMax`로 보존한다. 다만 `computed_auto_repeat_tracks()`의 fixed-repeat preflight와 확장 단계가 `LengthPx`만 허용하므로, 유효한 계산 결과 형태가 기존 opt-in profile에서 거부되는 것을 계획의 출발점으로 삼는다.

## 재현 환경

- 입력: [`grid-fixed-repeat-minmax-flex-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-fixed-repeat-minmax-flex-c114-precomparison.html)
- 비교기: Agent Browser, `HeadlessChrome/156.0.0.0`, macOS user agent
- viewport: 1280×633, DPR 1
- 원본 관찰: [`chrome156-observation.json`](./chrome156-observation.json)
- 화면: [`chrome156-agent-browser-precomparison.png`](./chrome156-agent-browser-precomparison.png), 1280×633, 6개 scene
- 규범에서 독립 도출한 기대값: [`spec-derived-expectations.json`](./spec-derived-expectations.json)

Chrome은 여기서 구현 비교기일 뿐 규범 판정자가 아니다. `CSS.supports()`는 `0fr`, `0.5fr`와 `1e38fr`를 포함한 입력을 parse-valid로 관찰했고, 두 valid-but-out-of-profile controls를 지원함과 invalid `minmax(1fr, 12px)`를 거부함도 확인했다. 특히 큰 finite factor가 parse-valid여도 Taffy의 f32 aggregate에서 overflow할 수 있어 새 opt-in profile이 받아들여야 하는 것은 아니다.

## 독립 산술

단일 fixed-repeat 예제의 최소 폭 부등식은 `24 + 24m + 4(m + 1) = 28m + 28`이다. 따라서 폭 200px에서는 6개, 높이 120px에서는 3개의 auto-repeat이 들어간다. `columns-groups`에서는 fixed prefix의 최소 폭 40px, suffix의 최소 폭 32px, `m`개의 24px auto tracks, `m+5`개 gutter를 합쳐 `28m + 92 <= 300`이며 `m=7`이다. `columns-flex-factor-low`는 fixed minimum 12px, `m`개의 24px repeated minimum과 `m`개 gutter를 합쳐 `28m + 12 <= 200`이며 `m=6`이다. 여섯 0fr track은 24px에 머물고 0.5fr track은 16px로 늘어나 16px의 자유 공간은 쓰지 않는다. 반복 수는 authored `minmax(px, fr)`의 fixed minimum으로 계산한다.

flex 기대값, auto-fit collapse와 모든 item 시작 좌표는 JSON에 scene별로 분리해 기록했다. CSSOM은 fractional used size를 1/64 CSS px 단위로 양자화하므로 `rows-fit`의 세 1fr track은 화면에서 37.328125px로 관찰되지만 독립 계산값은 각각 112/3px이다. 이 값들을 동일한 수치 출처인 것처럼 섞지 않는다.

Chrome 관찰은 일반 float track sizing 결과만 고정한다. Spinon profile의 overflow control은 CSS에서 유효한 `repeat(4,minmax(12px,1e38fr))` 값이 Stylo computed `Fraction`으로 유한하게 보존되는지 확인하고, 반복 확장 후 factor 합이 f32 비유한이면 Taffy 호출 전에 거부해야 한다. negative/NaN/infinity 값은 CSS syntax 비교가 아니라 synthetic typed-value test 대상이다. component·projection descriptor와 1 MiB line metadata 경계는 [`spec-derived-expectations.json`](./spec-derived-expectations.json)의 별도 resource controls로 지정했다. Projection descriptor는 auto-repeat pattern 1회분만 포함하며, Taffy의 used auto-repeat 확장에는 별도의 10,000 track ceiling이 적용된다.

## 파일 해시

해시는 계획 PR 작성 시점에 고정한다. 해당 파일을 수정하면 이 표도 다시 계산한다.

| 파일 | SHA-256 |
| --- | --- |
| 입력 HTML | `f6ce488c13abd922d105273373232170dbfcb996c7d58d1a47998b1b9064ef61` |
| Chrome 관찰 JSON | `f26ced2f8e7d73bca0a0e693ca7c36ecd03adacdfbc188765e35d543ba0ab6af` |
| Agent Browser 화면 | `aef6a7e5edfc2e0df8f0e2430479f08092ea2171a92f4a2edfb6baeb8b1cac1c` |
| 규범 기대값 JSON | `9af8e900835b43a8c7347af2878a646b6d724fdb8b3da1881fbfe8336138ac6c` |

## 판정 한계

이 사전 비교는 앱 runtime, Android/iOS simulator, 실기기, GPU surface, CSS 일반 노출, WPT 또는 전체 C11.4를 검증하지 않는다. 구현 단계에서는 고정 fixture를 opt-in runtime 경로에만 연결하고, 기본 artifact 제외와 fixture-test positive control을 따로 검증해야 한다.
