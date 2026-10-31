# C11.4 anonymous Grid text `letter-spacing` 사전 비교

## 범위와 입력

이 자료는 anonymous Grid text intrinsic sizing에서 computed `letter-spacing`을 전달하기 전의 oracle 기록이다. Spinon 구현 결과나 CSS 지원 완료를 뜻하지 않는다.

- Fixture: `tests/fixtures/css/c11/c11-4-anonymous-grid-text-letter-spacing-precomparison.html`
- 글꼴: 저장소의 Ahem, 10 CSS px. ASCII 문자와 공백은 각각 10 CSS px advance다.
- 입력: `XX XX` 및 단어 줄바꿈 영향을 분리하기 위한 `XX`; `normal`, `-1px`, `0px`, `0.5px`, `1px`, `2px`.
- 환경: Agent Browser 0.31.1, Chromium/Chrome 156.0.8078.80, macOS arm64, viewport 1280×633, DPR 1. Ahem font face는 loaded 상태였다.
- 원본 관찰: `chrome/chrome156-observation.json`; 화면: `chrome/chrome156-letter-spacing.png`, `chrome/chrome156-letter-spacing-single-word-controls.png`.

## Chrome 156 관찰값

폭은 CSS px이다. 두 단어 fixture의 min-content는 줄바꿈 뒤 가장 넓은 단어를, max-content는 공백을 포함해 한 줄로 유지한 폭을 보여준다.

| letter-spacing | `XX` min/max | `XX XX` min-content | `XX XX` max-content |
| --- | ---: | ---: | ---: |
| normal / 0px | 20 | 20 | 50 |
| -1px | 18 | 18 | 45 |
| 0.5px | 21 | 21 | 52.5 |
| 1px | 22 | 22 | 55 |
| 2px | 24 | 24 | 60 |

동일 문자를 `width:max-content`인 일반 block에 넣은 추가 대조에서도 `XX`는 normal 20px, -1px 18px, 0.5px 21px, 1px 22px, 2px 24px였다. 따라서 관찰 차이는 Grid track sizing이나 공백 줄바꿈만으로 생긴 것이 아니다.

## 규범 기반 기대값

[CSS Text Level 3 §7.2](https://www.w3.org/TR/css-text-3/#letter-spacing-property)는 letter spacing을 인접한 typographic character unit 사이에 적용하고, 각 단위 양쪽에 절반씩 넣되 줄의 시작과 끝에는 적용하지 않는다고 정의한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)의 min-content/max-content 정의를 이 Ahem 입력에 적용하면 다음 값이 나온다.

| letter-spacing | `XX` min/max | `XX XX` min-content | `XX XX` max-content |
| --- | ---: | ---: | ---: |
| normal / 0px | 20 | 20 | 50 |
| -1px | 19 | 19 | 46 |
| 0.5px | 20.5 | 20.5 | 52 |
| 1px | 21 | 21 | 54 |
| 2px | 22 | 22 | 58 |

정상 단어 `XX`에는 문자 사이 간격 하나만 들어간다. `XX XX`의 max-content 한 줄에는 문자·공백을 포함한 5개 typographic unit과 인접 간격 4개가 있으므로 `50px + 4 × letter-spacing`이다. min-content에서는 공백에서 줄을 나누고 각 줄의 `XX`를 계산한다.

## 차이와 구현 전 판정

Chrome 156은 각 입력 단위 뒤에 spacing 하나를 더한 값처럼 관찰되어 CSS Text Level 3의 줄 끝 억제 기대와 다르다. 구현 기준은 규범값으로 두고 Chrome 결과는 비교 관측으로 보존한다. 따라서 Chrome parity는 주장하지 않으며, 구현 후 차이도 별도로 보고한다.

현재 Parley 0.12.0은 `StyleProperty::LetterSpacing`을 제공하지만 내부 spacing 경로가 atom 뒤의 간격으로 모델링되어 있다. 단순 전달만으로 규범의 줄 끝 처리가 구현된다고 가정하지 않는다. 작업 계획은 intrinsic sizing 및 고정 익명 Grid text 경로에서 줄 끝 간격을 검증하고, 구현/runtime review에서 같은 경계를 다시 확인한다.

범위는 이 제한 runtime fixture profile의 `normal` 및 유한한 px 값에 한정한다. 퍼센트·상대 단위·일반 CSS admission·공개 CSS 지원·C11/C11.4 완료 승격은 포함하지 않는다.
