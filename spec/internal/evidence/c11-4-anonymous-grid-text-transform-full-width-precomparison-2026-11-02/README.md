# C11.4 anonymous Grid text `text-transform: full-width` 사전 비교

Agent Browser 0.31.1의 HeadlessChrome 156에서 고정 HTML, computed style, Noto Sans KR 10px/400의 track·candidate 폭을 관측했다. CSS Text Level 3 CRD 2026-08-14 §2.1은 `full-width`를 정의하지만 이 값은 at-risk로 표시된다. Chrome 156은 선언을 computed `none`으로 돌려주므로 변환 동작의 browser oracle로 쓸 수 없다. 이 차이를 숨기지 않고 Chrome의 plain-text measurements를 control로만 보존한다. [CSS Text Level 3 §2.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-transform-property) · [Unicode 17 UnicodeData.txt](https://www.unicode.org/Public/17.0.0/ucd/UnicodeData.txt) · [Agent Browser screenshot](./chrome156-full-width.png)

규범 기대 문자열은 Unicode 17 compatibility decomposition `<wide>`를 역으로 적용한다. 이 제한 입력에서는 `A`→`Ａ`, `B`→`Ｂ`, U+0020→U+3000, `1`→`１`, `2`→`２`가 되어 `AB 12`가 `ＡＢ　１２`가 된다. `white-space:normal` Phase I에서 원본의 연속 U+0020 세 개를 하나로 collapse한 뒤 변환하므로 `AB   12`도 같은 결과다. 기대 문자열과 Chrome의 literal candidate 폭은 [expected.json](./expected.json)과 [chrome156-observations.json](./chrome156-observations.json)에 분리했다.

Chrome 156의 10px Noto Sans KR 측정은 `AB 12` max-content 25.984375px, 문자 그대로 입력한 `ＡＢ　１２` max-content 50px·min-content 20px, 3개의 ideographic spaces를 포함한 candidate 70px이다. 즉 변환 결과의 고정 font width를 비교할 수는 있지만 `text-transform:full-width` 동작 자체를 비교할 수는 없다. screenshot도 Chrome이 지원하지 않는 선언을 표시하며 변환 성공 화면이 아니다.

이 사전 비교는 계획 후보를 고정한다. 구현이 채택되면 정확한 fixture owner, computed `full-width`, direct `lang="ja"`, Noto Sans KR 10px/400, horizontal LTR, `white-space:normal`, `AB 12`와 `AB   12`만 허용한다. 일반 CSS admission, 임의 Unicode mapping, arbitrary font, WPT 적합성, 일반 `full-width` support, full-size-kana, 전체 C11.4/C11 완료는 검증하지 않는다. CSS Text가 at-risk로 분류하는 점은 기능의 표준 상태 한계로 남긴다.

## SHA-256

| 입력 | SHA-256 |
|---|---|
| HTML fixture | `1aa5e5a69a6faa182e2c5214cc678de54ab41b12e8fe23f1f8906a61d4555780` |
| Noto Sans KR variable font | `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76` |
| Chrome observation JSON | `4dcc6fe1ebab7927f89e87972aefaab68fae11bf857d9210f2744e98c1df226d` |
| Standard expected JSON | `01ae3c350ca764d6ccf2ede7877c816caba0352fc5e600b8d2866189341301e7` |
| Chrome screenshot | `d39089da44d925cc2e2b15155c9f8e8ad1b73e2a9bb41c46602f4748b69c65e7` |
