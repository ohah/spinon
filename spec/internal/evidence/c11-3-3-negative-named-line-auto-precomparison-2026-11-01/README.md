# C11.3.3 음수 named-line pair 사전 비교

## 범위

CSS Grid 2025 CRD §§7.6, 8.1.3, 8.3.1에서 도출한 leading implicit track 확장과 named occurrence 배치의 독립 기대값을 Chrome 156 관찰과 비교했다. 이 비교는 Spinon code, WPT harness 또는 앱 runtime을 실행하지 않았다.

## 입력과 기대값

- 고정 HTML fixture: [grid-template-negative-named-lines-auto-precomparison.html](../../../../tests/fixtures/css/c11/grid-template-negative-named-lines-auto-precomparison.html)
- 장면 요약: [input.json](./input.json)
- 규범 독립 계산: [expected-values.json](./expected-values.json)
- Agent Browser 관찰: [chrome-observation.json](./chrome-observation.json)
- 실행 확인: `node spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/verify-precomparison.mjs`가 browser viewport `1280×1040`, DPR `1`과 root dimensions를 확인하고 10/10 expected track sequence와 root-relative item frame의 일치를 확인했다.
- 화면: [Chrome 156 screenshot](./chrome156.png)은 각 scene의 resolved track list와 item frame을 함께 표시한다.

Agent Browser 0.31.1은 `HeadlessChrome/156.0.0.0`을 보고했다. Browser에서 Chromium revision/binary hash를 읽을 수 없어 이 결과는 버전 표기 기준의 재현이며 binary-pinned Chrome 증거가 아니다. 규범 기대값은 브라우저 출력에서 역산하지 않고 explicit track 수, negative occurrence 방향, endpoint conflict 규칙과 fixed auto-track cycle로 계산했다. Authored `ghost` line control에서는 `-1 ghost`가 explicit line에 먼저 매칭되고 `-2 ghost`만 leading implicit line을 만들어 `[7px] [ghost] 20px 30px 40px`가 된다.

`-4 ghost / -1 ghost`는 네 leading implicit track을 만들고 세 track을 item span에 포함한다. Fixed `[5px,7px]` input의 CSSOM start-to-end order는 `5,7,5,7`이다. Equal endpoint `-2 ghost / -2 ghost`는 §8.3.1에 따라 end line이 제거되고 남은 definite line에서 기본 1-track span을 사용한다. Auto input의 18px item은 세 span track에 6px씩 분배되고 span 밖 leading auto track은 0px다. Row-axis 결과는 column-axis 결과와 동일한 계산을 쓴다.

## Hashes

SHA-256 입력은 HTML source, scene summary, independent expected values, browser observation 및 screenshot이다. HTML을 변경하면 observation/screenshot/verifier를 다시 실행한다.

| 파일 | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c11/grid-template-negative-named-lines-auto-precomparison.html` | `c64acb1db6f332602e5b5affa0858c4cdaaeb126f866455337991d3f205084b3` |
| `input.json` | `8ca6430fcc975ef464626934188eb1fb75e6963a709f16ca70ecaa5b39d93ca7` |
| `expected-values.json` | `fd47610701173ce706ec5fd1b1d4e4069ba393318dcb9e50d9a1ad4b97255ac4` |
| `chrome-observation.json` | `7b29815c3cf4e1718aea70cc324c8da2c51401f2274c6586eba6d4abf40972d9` |
| `chrome156.png` | `20dbf90eb3dfdae8bca8ddd1b64095d307dad14882cee63e1951ad17282016ef` |
