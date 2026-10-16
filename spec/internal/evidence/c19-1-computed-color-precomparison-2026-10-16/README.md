# C19.1 computed color 사전 비교

## 기준과 고정 환경

- 작업 전 main 기준은 `236d58f970dae15c6f2bb42d613a2db163b5158c` (`24a4d9c7d18701c17f0cb0963b18ec3f89c90bec` tree)다. 제품 코드 변경 전 Agent Browser로 Chromium 결과를 고정했다.
- Agent Browser 0.31.1이 Google Chrome 155.0.8059.85를 실행했다. Chrome for Testing 패키징 여부는 확인되지 않아 설치된 Google Chrome으로 기록한다. CDP `Browser.getVersion`은 protocol `1.3`, V8 `15.5.35.25`, `webKitVersion=537.36 (@0e66b94549a8bd85f03fbdf189b752d6e7b63c32)`을 반환했다. 2026-10-17 Agent Browser의 `chrome://version`에서 같은 hash가 Chromium revision `refs/branch-heads/8059@{#1304}`임을 확인했다. 실행 파일과 engine framework SHA-256은 [browser-metadata.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/browser-metadata.json)에 있다. 색 변환 소스 분석은 [고정 Chromium revision 근거](./chromium-source-analysis-2026-10-17.md)를 참고한다.
- macOS 26.5.1 arm64, Chromium navigator platform `MacIntel`, viewport `1280×633` CSS px, screen `800×600`, DPR 1, `ko-KR`/`ko`, `Asia/Seoul`로 실행했다. OS 선호 색상은 dark지만 fixture와 모든 element에 `color-scheme: light`를 명시했다. `forced-colors`는 꺼져 있고 `color-gamut`은 `srgb`다.
- 색 출력 oracle은 Canvas 2D `srgb` / `unorm8` context의 `getImageData()` RGBA8이다. context는 alpha를 사용하고 tone mapping은 `standard`다. 결과는 GPU surface, wide-gamut display 또는 platform compositor 측정이 아니다.

## 입력과 원본

- [chromium-color-fixture.html](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/chromium-color-fixture.html)이 44개 고정 행을 실행한다. named/system color, `transparent`, 3/4/6/8자리 hex, legacy/modern RGB·HSL, HWB, Lab/LCH, OKLab/OKLCH, `color()`의 sRGB·linear sRGB·Display-P3·A98·ProPhoto·Rec.2020·XYZ D50/D65/alias, `none`, alpha, 상속된 `currentColor`, 잘못된 RGB·Lab·색 공간을 포함한다.
- [chromium-color-observations.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/chromium-color-observations.json)은 `CSS.supports`, computed style serialization, Typed OM serialization, sRGB Canvas RGBA8을 보존한다. 41행은 유효, 3개 invalid control은 거부됐다. 세 invalid 행의 `color`는 기존 `rgb(1, 2, 3)`을 보존하고 `background-color`는 transparent 기본값을 보존했다.
- [Agent Browser 화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/chromium-color-oracle-agent-browser.png)은 같은 fixture의 실제 Chromium 색상 표본을 보여준다. 눈으로 확인하는 화면 자료이며 수치 oracle은 JSON이다.
- CSS Color 4 규범 문서: <https://www.w3.org/TR/css-color-4/>. WPT는 commit `d3519c37fa17d5b2e27979fc2466c724660bdb9f`로 고정했다. 실제 사용한 parsing/computed color 테스트 16개 경로의 upstream blob SHA와 원본 byte SHA-256은 [wpt-source-manifest.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c19-1-computed-color-precomparison-2026-10-16/wpt-source-manifest.json)에 기록했다. WPT는 이 작업에서 실행하지 않았으며 Chromium 결과를 Spinon WPT 적합성으로 해석하지 않는다.

## 코드 변경 전 판정 기준

1. Stylo의 고정 `stylo 0.22.0` cascade 결과에서 색 공간, 채널, alpha, `none` 정보와 `currentColor` 상속을 typed color 값으로 보존한다. 별도 CSS 색상 parser를 만들지 않는다.
2. `AbsoluteColor`의 CSS Color 4 변환/gamut mapping과 Chrome-compatible sRGB surface 변환을 구분한다. 37개 비-system 유효행은 Chrome Canvas oracle과 exact 비교하고, 네 system-color 행은 Stylo Servo palette로 별도 검증한다. P3 gamut-edge는 Stylo Local MINDE `[255,100,0,255]`와 Chrome-compatible `[255,87,0,255]`, 상속 currentColor는 `[251,0,101,255]`다.
3. partial alpha와 component `none`을 opaque-only 값으로 축약하지 않는다. Invalid syntax는 CSS cascade에서 버리고 앞선 유효 선언을 유지한다. 런타임 `CssParseDiagnostic` 생성은 pass 조건이 아니다. `currentColor`는 상속된 색으로 계산한다.
4. 비유한 source component와 mutation된 out-of-range gamut-mapped channel은 오류로 거부되어야 한다. 기존 C18.1–C18.4 이미지 경로와 alpha 동작은 색상 carrier 변경으로 바뀌지 않아야 한다.
5. 이번 C19.1 slice는 computed color carrier와 sRGB 변환 근거다. CSS box opacity/group opacity, partial-alpha GPU blending, glyph paint, hit-test 변화, contrast/WCAG 적합성은 이 slice의 통과로 완료하지 않는다. 공식 C19 상태는 미완료로 둔다.

## 최종 검증

- `workspace-test-final.log`: 전체 workspace 908 passed, 0 failed, 4 ignored.
- `workspace-clippy-final.log`: `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.
- `cargo fmt --all -- --check`와 `git diff --check` 통과.
- Rspress 전체 빌드는 기존 문서 dead link로 실패했다. 새 C19 문서와 review 페이지에는 dead link가 없다. [문서 빌드 판정](./docs-build-result.md).

## 구현 중 재현한 상속 경계

- runtime mount의 합성 `body`에 Display-P3 `color`를 지정하고 mount root가 `color:inherit` / `background-color:currentColor`를 사용하도록 했다. 초기 C19 profile은 synthetic `html/body` cascade를 생략해 root 색 공간을 sRGB로 계산했다. 수정 전 실패 log는 `c19-runtime-body-inheritance-before-fix.log`다.
- C19 profile에 합성 document cascade를 추가하고 layout geometry validation은 계속 건너뛰도록 했다. 수정 후 결과는 Display-P3이며 Chrome fixture의 `[251,0,101,255]`와 일치한다. 전체 runtime style profile에 geometry 제약을 추가하지 않았다.

## 입력 hash

- fixture HTML SHA-256: `ab210d543f58e71dd4fdccce4b16f3b213f5302502fa85588cafe59321c41cc5`
- Chromium observations SHA-256: `aa31312ddb901a65d7db17cae625620e67b9d873f029731dd882de7d827933ee`
- Agent Browser screenshot SHA-256: `4e7b961d5acf6255e9fc2162522c1f008f9903562fb4c49a4606dee4aa7be91d`
- WPT manifest SHA-256: `4a737c140d2157c3d2d1a454aa04d2db7f657f6573914a0cd6dd22f9232b0d08`

opaque probe는 8-bit 양자화 전 Chrome computed color의 sRGB 변환값을 기록한다. premultiplication 손실을 피하려고 RGB 채널은 별도 probe에서, alpha는 원본 Canvas readback에서 가져왔다. 이 값은 캡처한 Chrome 출력이며 Chrome과 Stylo가 같은 CSS Color 4 gamut-mapping 알고리즘을 사용한다는 독립 증거는 아니다. 구현 중 차이 목록은 [C19.1 호환성 분류](../c19-1-computed-color-compatibility-triage-2026-10-16.md)에 있다.
