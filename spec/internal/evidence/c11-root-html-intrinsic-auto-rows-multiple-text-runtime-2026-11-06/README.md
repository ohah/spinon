# C11.4 여러 direct text item intrinsic implicit-row runtime 근거

후보 base HEAD는 `11d3e73f9899adc326963af051c2fdcd37ad4e6f`다. 구현 source는 이 HEAD 위에 아직 PR 전인 working-tree 변경으로, `candidate-source.sha256`이 변경 파일 각각을 고정한다. 기준 fixture SHA-256은 `3232e75cc346718a1a7970ec71b7810b3d34a196caa0919610e5deb454657970`이다. Chrome 156 사전 관측과 독립 기대값은 [비교 근거](../c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/README.md)에 있다.

## Rust 검증

- style-to-layout intrinsic auto-row module: 4/4 통과. 여러 direct text item, source-order 역전, intrinsic columns 결합 및 10,000/10,001 track 경계를 포함한다.
- style-to-layout helper: 3/3 통과. ASCII `A` 문법, CSS whitespace 분류, checked row-count 경계를 확인한다.
- 기본 `RuntimeSession`: 7/7 통과. 서로 다른 intrinsic contribution과 item identity/order, 두 text paint의 line count와 revision, DOM-order paint, hidden item, 두 번째 text item의 invalid source/style/markup atomic rejection을 확인한다.
- 같은 runtime 모듈의 기존 intrinsic-column + text-row regression: 1/1 통과 (`runtime-intrinsic-columns-regression.log`).
- `spinon-layout` filtered grid track-sizing regression: 1/1 통과.
- 변경 crate scoped all-target Clippy와 `cargo fmt --all -- --check`가 통과했다. 원본 로그는 이 디렉터리의 `*.log`에 있다.
- 텍스트 로그는 끝 공백만 제거해 저장소 diff 검사를 통과시켰다. 출력 내용과 명령 상태는 바꾸지 않았다.

10,000행 입력은 실제 10,000 direct item으로 계산했다. 레이아웃은 성공하고 마지막 item frame은 `[0,10060,1,1]`이다. 이 크기에서 Taffy의 선택 진단 trace 예산을 넘기므로 해당 Grid trace만 생략하고 frame 결과를 유지한다. 10,001번째 item은 `grid-auto-rows`의 10,000-track 상한 오류로 layout 계산 전에 거부된다. 부분 trace나 일부 frame은 반환하지 않는다.

## Android·iOS 시뮬레이터

동일 HTML fixture를 default Root HTML application route에 넣어 실행했다. 실행 script는 기본 APK/app build 뒤 assets만 후보 fixture로 바꿔 다시 패키징/설치하며, native C11 fixture feature는 양쪽 모두 꺼져 있다. 사용한 두 기기에는 실기기가 없었다.

- Android API 37 ARM64 AVD `emulator-5554`: `layout=ready`, 8 boxes, root-app DOM/item-order smoke 통과. APK 안 `assets/index.html`과 root-app의 hash는 기준 입력과 일치한다. 화면 캡처는 `android/screenshot.png`이다.
- iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`: `layout=ready`, 8 boxes, root-app evaluation 성공. `.app`에 설치된 `index.html`과 root-app hash가 기준 입력과 일치한다. 화면 캡처는 `ios/screenshot.png`이다.
- 두 screenshot은 명시 10px 행, 각기 두 줄·한 줄인 text item 행, 고정 34px 마지막 행이 Grid 아래로 넘치는 화면을 보인다. Android glyph 줄바꿈은 4+2, iOS는 3+3이지만 각 text item은 두 플랫폼에서 각각 두 줄과 한 줄이다. 이 폰트 차이는 browser 고정 상수가 아니다.

시뮬레이터 앱 로그는 runtime의 전체 per-node frame이나 text-line 수를 내보내지 않는다. 해당 exact CSS frame, line count, contribution identity는 Rust layout 및 RuntimeSession assertion이 확인한다. simulator screenshot은 display capture이며 Android PixelCopy, Metal drawable readback, compositor 이후 pixel 비교는 하지 않았다.

실행 중 수정한 `build/spinon/bootstrap/index.html`과 `root-app.js`는 script 종료 trap으로 원래 파일을 복원했다. `original-bootstrap/`에는 복구 기준 원본이 보존돼 있다. APK/app asset 해시, 빌드·설치·launch·화면 로그 및 환경은 각각 `android/`, `ios/`, `environment.txt`에서 확인할 수 있다. 에이전트 브라우저를 실행하지 않았다.

## PR 검토

- [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)
- [실제 PR diff 적대 검토 30개](./pr-diff-review-30.md): PR #1199, 30/30 통과. Agent Browser에서 두 raw screenshot 링크의 실제 렌더링과 이미지 크기까지 확인했다.
