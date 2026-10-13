# C11.3.1 정수 repeat Chromium 사전 비교

## 고정 fixture와 실행 환경

- Inventory: [`grid-template-repeat-c1131-inventory.json`](../../../../tests/fixtures/css/c11/grid-template-repeat-c1131-inventory.json) · SHA-256 `ebc4716049e775ec1116b7c487102ec97e3b54cd1a7f91980a1caa413cd63914`
- HTML: [`grid-template-repeat-c1131.html`](../../../../tests/fixtures/css/c11/grid-template-repeat-c1131.html) · SHA-256 `0a109d053a3a7e80abb3a652451a3e1b3a1ec667a1b507d11d036d625601ee7b`
- Chromium 기준 JSON: [`c11-3-1-grid-repeat-precomparison-v1.json`](../../../../tests/fixtures/css/references/c11-3-1-grid-repeat-precomparison-v1.json) · SHA-256 `3b98a994a0fd1ec52a838887c71a54b0afad62527d7a289557cc7ed728c24788`
- 캡처 도구: [`capture-c11-3-1-grid-repeat-precomparison.mjs`](../../../../tools/css-reference/capture-c11-3-1-grid-repeat-precomparison.mjs) · SHA-256 `d5067aca4f985539c9c66f8ca4d8055d022f75b4ea7dd8744e30b4f35ae793ed`
- 실행 ID: `c1131-chrome-20261013T035742923Z-f59a108e`
- Chrome for Testing `154.0.8037.92`, CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`, 실행 파일 SHA-256 `ce0c09ea98a7592e7799e54c1867797ccf99d4d6e821a11067c332e454732349`
- macOS 26.5.1 arm64, Node `v24.3.0`, viewport `320×240 CSS px`, DPR 1·2, `en-US`, UTC, light, LTR, `horizontal-tb`, coarse pointer

12개 scene·29개 node에서 CSSOM resolved value, line-name group, node별 geometry를 기록했다. 두 DPR의 geometry 기준과 CSSOM 값을 비교할 수 있도록 저장했으며 DPR 1 screenshot 12개를 보관했다. geometry 허용 오차는 각 축 `0.5 CSS px`다. [fixture 캡처 요약](c1131-chrome-20261013T035742923Z-f59a108e/capture-summary.json).

## Chrome 관측

| Scene | Chrome 관측 |
| --- | --- |
| `multi-track-repeat` | 7 tracks: `20px 80px 40px 36px 40px 36px 20px` |
| `line-name-seam` | repeat 끝/시작 이름이 한 line group으로 합쳐짐. 순서: `outer-start`; `outer-end repeat-start item-start`; `seam`; `item-end item-start`; `seam`; `item-end trailing`; `outer-last` |
| `duplicate-line-names` | 동일 line의 중복 이름이 CSSOM group에 보존됨: `[same same]` 3개 line, 뒤에 `[end]` |
| `mixed-units-and-gaps` | resolved columns: `50px 64px 55px 64px 55px` |
| `limit-10000` / `limit-10001` | Chrome은 10,000과 10,001 explicit tracks를 모두 계산. Spinon 계획 상한 10,000은 별도 정책으로 유지하고 10,001은 거부한다. |
| `auto-fill-outside-profile` / `minmax-outside-profile` | Chrome 기준값은 각각 8 tracks / 2 tracks. 두 구문은 C11.3.1 지원 subset 밖이다. |
| `nested-repeat-invalid` / `zero-repeat-invalid` | 잘못된 선언은 CSS cascade fallback 뒤 implicit auto track의 resolved 값 `320px`를 보임. 이는 repeat 지원 성공이나 Spinon runtime 오류 결과가 아니다. |

![repeat seam의 Chrome 기준 화면](c1131-chrome-20261013T035742923Z-f59a108e/line-name-seam-dpr1.png)

## 고정 WPT 후보

WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`다. Chrome 154 runner 실행은 5 tests, 5 PASS, unexpected 0이다. 상세 원문은 [runner 로그](wpt-chrome154.0.8037.92-run02.log)와 [구조화 결과](wpt-chrome154.0.8037.92-run02.json)다.

| 후보 | SHA-256 | Chrome 결과 |
| --- | --- | --- |
| `grid-repeat-integer-001.html` | `f48604bbd0316467f1b004052db9425d132c56632b034f99eb7aaf34b530ad1c` | PASS |
| `grid-repeat-integer-002.html` | `27b78abd184512f3f5371dd13fe8bd49e7aee41dc0ae7a7cc1689d54ab571a10` | PASS |
| `grid-repeat-integer-003.html` | `baba3dbdfc7476bd2937cdffd2cc7e8c8c2b73ed8c80849d15e9e846e40544ac` | PASS |
| `grid-repeat-integer-004.html` | `d87c350e49934251cbbf346086fd89f6dd815b5ecde0dec31ed2ba843263a333` | PASS |
| `grid-repeat-integer-005.html` | `34a7548251bb2ad1a0d45677d6a8bbf99684f71157d6aa5ef587d25b42c154d4` | PASS |

WPT JSON의 `run_info.headless`는 `false`로 기록되어 있다. runner의 실제 표시 모드는 이 결과만으로 확정하지 않는다. 별도 Chromium fixture 캡처 JSON은 `--headless=new` 인수와 `HeadlessChrome` user agent를 기록했다. 위 WPT 결과는 Chrome oracle 실행일 뿐 Spinon WPT 적합성 결과가 아니다.

## 완료 경계

- 완료: C11.3.1 고정 Chromium 입력·CSSOM·geometry·line-name 기준, WPT 후보 원문 hash 및 Chrome oracle 실행 기록.
- 후속 구현: Rust cascade·typed projection·layout profile을 추가했고, Android 17/API 37 AVD와 iOS 26.2 Simulator에서 `multi-track-repeat` 단일 장면의 runtime geometry를 실행했다. 상세 로그와 화면은 [runtime 근거](../c11-3-1-grid-repeat-runtime-2026-10-13/README.md)다.
- 미완료: Android 실기기, Android PixelCopy 및 iOS Metal drawable pixel readback, 구현 실패 관점 검토, Spinon WPT 적합성.
- 이 문서와 Chrome 관측은 구현 완료, 공개 CSS 지원 선언 또는 전체 CSS Grid 지원을 뜻하지 않는다.

fixture 자체 검사: `bun test ./tools/css-reference/c11-3-1-grid-repeat-precomparison.test.mjs` — 5 pass, 0 fail.


## 로그 처리

고정 WPT 후보 실행 로그에서 빈 진단 줄과 줄 끝 공백만 제거했다. 실행 결과와 본문 내용은 바꾸지 않았다.
