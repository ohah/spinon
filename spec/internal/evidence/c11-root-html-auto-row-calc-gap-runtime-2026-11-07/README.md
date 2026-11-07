# C11.4 parsed Root HTML auto-row `calc()` `row-gap` 실행 근거

## 현재 범위

기존 `RootHtmlAutoRowStretchV1`에 finite `<length>` 한 항과 finite `<percentage>` 한 항으로 이루어진 2-term `calc()` row-gap을 연결한 제한 candidate다. definite 40px content-box block-size, 40px column, 10px explicit row, auto implicit rows, direct item 2–3개, `column-gap:0`, `align-items:normal`, 기존 `align-content` 10개가 고정 경계다. positional/distributed alignment는 기존 projection 경계에 따라 direct item 3개에서 실행한다.

이 candidate는 C11.4나 C11 완료, 일반 CSS `calc()` admission 또는 WPT 적합성을 뜻하지 않는다. 내부 계약 버전 `0.1.0`을 유지한다.

## 독립 기준과 Rust 검증

[Chrome 157 사전 비교](../c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/README.md)는 Chrome geometry와 독립 기대값을 분리한다. 6 expressions × 2/3 direct item count × 10 alignments의 120 scenes, 1,500 coordinates가 `1/32` CSS px 허용차 안에서 일치했고 최대 차이는 `0.0208333334px`다. Layout 행렬은 [expected-values.json](../c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/expected-values.json)을 읽는다.

- Layout `root_html_auto_row_stretch`: 12/12. Typed calc 120-scene geometry·row trace, unsupported/missing/misbound/duplicate ID, fixed percentage regression과 기존 overflow를 포함한다.
- Projection `root_html_auto_row`: 4/4. 2-term signed length-percentage grammar와 기존 fixed profile 경계를 확인한다.
- RuntimeSession `row_gap`: 6/6. 기존 percentage 70-scene matrix·`normal`·negative direct value 진단·unsupported calc 원자적 실패 및 recovery, 새 calc positive/clamp/overflow scenes를 포함한다. 새 scenes는 `RuntimeSession::new_runtime_gpu_with_root_html()` 기본 경로에서 실행한다.
- 변경 crate Clippy `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`: 통과.
- 명령 원본: [`verification/`](./verification/).

로그 본문은 끝 공백과 마지막 빈 줄만 정규화했다. 명령 출력의 메시지와 수치 값은 바꾸지 않았다.

## Android API 37 ARM64 AVD · iPhone 17 Pro iOS 26.2 Simulator

기본 Root HTML 앱 route로 실행했다. C11 fixture-test feature는 모두 끄고 C04 runtime GPU만 켰다. 각 pass에서 Android APK와 iOS app asset의 `index.html` SHA-256이 입력 fixture와 일치한다. 하네스는 실행 전 앱·build asset hash를 저장하고 실행 뒤 기존 설치 앱과 빌드 파일을 복원했다.

| pass | scene | fixture SHA-256 | Android / iOS 실행 |
|---|---|---|---|
| [pass01](./pass01/) | `calc(6.25% + 2px)` / `align-content:normal` | `0b35e698bc56a9a8df82838fb8b21262fee85cefd468fb3ad60a6f44d92288d1` | AVD·Simulator 모두 `layout=ready`, 6 boxes draw/present, 2 text records |
| [pass02](./pass02/) | `calc(120% - 2px)` / `align-content:center` | `b70b65f128ce0bb9c77c86b46c446d47b32c215402e20d86cc356a5df408b5a8` | AVD·Simulator 모두 `layout=ready`, 6 boxes draw/present, 2 text records |

두 번째 scene의 [OS PNG pixel 분석](./pass02/screenshot-pixel-analysis.json)은 exact RGB item bounds를 확인했다. Android 2.625× screenshot에서 마지막 item은 Grid 시작보다 70.857 CSS px 아래에, iOS 3× screenshot에서는 71 CSS px 아래에 시작한다. 두 위치 모두 40px Grid 하단을 넘는다. Screenshots는 OS screenshot의 post-compositor pixel 관측이다. Android PixelCopy, iOS Metal drawable readback 또는 GPU surface parity로 확대 해석하지 않는다.

두 pass 모두 `asset_and_simulator_restoration=PASS`다. Android 기존 installed APK SHA-256 `48db6dd5748c5bb7717d20ce4b3bc023ebe93acb1dc202b3d973164544c5bc04`와 iOS 설치 실행 파일 SHA-256 `eca866e291d5ffbbe7756c354532bb69bca9d2fed264bc8181d02c61a772a929`가 각각 복원 전·후 일치한다. 전체 build/install/launch/runtime/screenshot/restore log와 device properties는 pass 폴더에 있다.

사전 비교는 Agent Browser Chrome을 사용했고 브라우저 프로세스를 닫았다. 이번 Android/iOS 검증은 native simulator harness로 실행했으며 Terminal Browser는 쓰지 않았다.

## 한계와 다음 확인

- 두-item/positional alignment 조합은 기존 projection admission이 막는다. 두 item의 10 alignment 조합은 직접 Layout matrix에 포함되며 제품 projection 경계는 변경하지 않았다.
- `calc(5px)`와 임의 math AST는 실패 원자성을 유지한다. authored negative direct `row-gap`의 parser recovery는 별도 동작이다.
- Android는 software-rendered AVD다. 실기기, hardware GPU, PixelCopy, iOS compositor 이후 surface parity, WPT는 검증하지 않았다.
- 최종 [PR diff 실패 관점 30개](./pr-diff-review-30.md)는 별도 기록한다. PR merge/CI state는 아직 확정하지 않았다.

구현/runtime 검토: [실패 관점 20개](./implementation-runtime-review-20.md). 입력·산출물의 SHA-256 목록: [source-inputs.sha256](./source-inputs.sha256).
