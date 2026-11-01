# C11.4 anonymous Grid text-indent `hanging`·`each-line` 사전 비교

## 범위

이 자료는 기존 C11.4 anonymous direct-text Grid opt-in fixture가 거부하는 유효한 computed `text-indent` 키워드 `hanging`과 `each-line`을 제한 profile에 연결하기 전의 기준이다. Chrome 관찰은 Spinon 구현 또는 적합성 결과가 아니다.

## 입력과 환경

- Agent Browser를 실행한 HeadlessChrome `156.0.0.0`, macOS arm64, viewport `1280×633`, DPR 1.
- 고정 Ahem 10px/10px LTR font. `document.fonts.check('10px Ahem')`은 `true`다.
- `soft`: direct text `XXX X`, `white-space:normal`.
- `forced`: direct text `XXX\nX`, `white-space:pre-line`.
- 두 입력을 `min-content`·`max-content` column 및 `0px`, `10px`, `10px each-line`, `10px hanging`, `10px hanging each-line`과 조합해 20개 case를 만든다.
- 독립 기대값은 CSS Text Level 3 §8.1, CSS Sizing Level 3 §5.1, CSS Grid Level 1 §11 및 고정 Ahem advance에서 계산한다. Chrome CSSOM과 line rectangles는 별도 교차 확인이다.

고정 observation은 [`chrome156-observation.json`](./chrome156-observation.json), 독립 기대값은 [`expected-values.json`](./expected-values.json), 실제 fixture는 [`HTML 입력`](../../../../tests/fixtures/css/c11/c11-4-anonymous-grid-text-indent-flags-precomparison.html)이다. 재비교는 `node verify-observation.mjs`로 한다.

## 규범에서 도출한 기준

Ahem 10px에서 각 `X`와 공백의 advance는 10px이다. Soft-wrap 입력의 min-content line advances는 `[30,10]px`, max-content advance는 `50px`이다. `each-line`은 forced line break 뒤의 line에 indent를 적용하지만 soft-wrap 뒤에는 적용하지 않는다. `hanging`은 첫 formatted line 대신 continuation line에 indent를 적용한다. Forced-break 입력은 두 line의 text advances가 `[30,10]px`이며, 해당 computed flags에 따라 각 line의 inline origin이 달라진다.

20개 case의 used track width, root height, computed serialization, line origin 및 line advance는 독립 기대값 JSON에 고정했다. Browser observation과 기대값이 일치해도 이 profile 밖의 값이나 구조가 지원된다는 뜻은 아니다.

## 관측 한계와 hash

[Chrome 156 full-page screenshot](./chrome156-full-page.png)은 고정 matrix와 scene ID를 보여준다. Ahem glyph는 검은 기하 박스로 그려지므로 glyph raster equality 증거가 아니다. 이 사전 비교에서는 Spinon runtime, Android/iOS 앱, WPT runner, 실기기, GPU 또는 compositor pixel test를 실행하지 않았다.

- Fixture HTML SHA-256: `f7d1de0edf2ab3cfec3a794a746e0d556e107f55b843f0af7a4ac0bca14e3565`
- Ahem TTF SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Browser observation SHA-256: `532c5178ca473760ea6cb3e34632980a0ad1581938c21a7c08d745f3b2a9fca6`
- Screenshot SHA-256: `7e895c7a9febe70ef47a5d457c6d2b49aea0e3b87389e36ac4256dc99690b323`

Agent Browser session과 daemon을 닫았고 후속 process scan에서 Agent Browser 프로세스가 남지 않은 것을 확인했다.
