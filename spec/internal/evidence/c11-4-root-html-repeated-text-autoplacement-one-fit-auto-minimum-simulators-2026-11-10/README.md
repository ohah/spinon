# C11.4 one-fit auto-placement 자동 최소 크기 근거

이 자료는 여섯-member `grid-auto-rows` 목록에 `fit-content()` member 하나가 있는 제한 repeated-text `min-height:auto` profile을 다룬다. Chrome 157 관측을 기준 oracle로 사용한다. 일반 CSS Grid 지원, WPT 완료, hardware GPU 일치 또는 C11.4/C11 완료를 뜻하지 않는다.

## RuntimeSession

`parsed_root_html_one_fit_auto_minimum_matches_all_1440_chrome_scenes`는 기존 `min-height:0` corpus와 동일 key의 one-fit `min-height:auto` 장면을 비교한다. 행렬은 fit 위치 1–6, cap `0%`, `20%`, `120%`, `20px`, `align-content:start|stretch`를 포함한다. Parsed/computed track 값, Grid 및 item frame, text payload frame·줄 수, content floor와 overflow를 검사한다. 결과는 1,440 scenes, 반복 fit-row 1,920회, text floor까지 회복된 row 600개, content overflow 0개다.

Feature-gated trace는 zero-percentage fit-content row의 automatic minimum contribution 및 content floor와 같은 final track base size를 확인한다. 대조 입력은 cap보다 큰 min-content floor, `fit-content(7px)`, `fit-content(13%)`, profile 밖 mutation의 atomic rejection, 같은 세션에서 유효 입력으로의 복구를 다룬다. 기존 baseline과 two-fit 행렬도 별도 실행했다.

현재 candidate 로그는 `runtime-one-fit.log`(5 passed, 1,440 candidate scenes 포함), `runtime-min-height-zero-baseline.log`(1 passed, 1,440 baseline scenes), `runtime-one-fit-trace.log`(1 passed)다. `cargo fmt --all -- --check`와 `git diff --check`가 통과했다. `clippy.log`는 opt-in fixture feature를 사용한 변경 세 crate 검사 결과다. 변경하지 않은 anonymous-text 테스트의 기존 `too_many_arguments` 두 건은 해당 명령에서 허용했다. `clippy-first-pass.log`에는 첫 lint 실행에서 이 행렬의 불필요한 인덱스 순회를 찾아 수정한 이력이 남아 있다.

Row-span 회귀 검사도 통과했다: two-fit row-span 3,840 scenes와 대조·거부 검사를 수행했고, repeated-text row-span 960-scene profile도 통과했다. 기존 row-span profile은 별도 gate를 유지한다.

## iOS·Android 시뮬레이터 smoke

같은 Root HTML fixture를 iPhone 17 Pro / iOS 26.2와 Android API 37.2 ARM64(Pixel 6 profile, 1080×2400)에서 build·launch했다. Android emulator는 gfxstream 및 lavapipe/llvmpipe software rendering을 사용했다. hardware GPU 근거는 아니다.

| 기기·appearance | Fixture | 근거 |
| --- | --- | --- |
| iPhone 17 Pro, iOS 26.2, light | position 4, `fit-content(0%)`, `align-content:start`, 3줄 text | `ios-light.log`, `ios-iphone17pro-ios26_2-light.png` |
| iPhone 17 Pro, iOS 26.2, dark | position 4, `fit-content(20px)`, `align-content:stretch`, 3줄 text | `ios-dark.log`, `ios-iphone17pro-ios26_2-dark.png` |
| Android API 37.2 ARM64, light | position 4, `fit-content(0%)`, `align-content:start`, 3줄 text | `android-light.log`, `android-api37_2-light.png` |
| Android API 37.2 ARM64, dark | position 4, `fit-content(20px)`, `align-content:stretch`, 3줄 text | `android-dark.log`, `android-api37_2-dark.png` |

각 플랫폼 로그는 Root HTML asset과 앱 marker, `layout=ready`, light/dark 환경, `SPINON_C0410_DRAW ... presented`를 기록한다. 두 scheme에서 fit row는 30 CSS px인 세 줄 content floor를 유지한다. Stretch 장면에서는 남는 공간이 `auto` row에 배분된다. RuntimeSession은 여섯 fit 위치와 모든 cap/alignment 조합을 다루며 native screenshot은 position 4의 대표 장면이다.

`root-html-one-fit-auto-minimum-runtime.html`과 `root-app-one-fit-auto-minimum-runtime.js`가 사용한 fixture 입력이다. 플랫폼 앱 build를 위해 `examples/bootstrap`에 잠시 복사한 뒤 기존 파일을 복구했다. Fixture capture 뒤 두 플랫폼 앱을 기본 bootstrap 입력으로 다시 build·install하고 정지했다. iPhone profile과 기존 Android AVD를 종료했다. task 전용 simulator profile이나 AVD는 만들지 않았다.

최종 코드 fixture build hash:

- iOS `SpinonBootstrap` executable: `0ccaecd4ab9474404fbf6867f8fe4640c003a468f68cf8b9fa40ef8fc43891a4`
- Android `app-debug.apk`: `8b05aa306fe5d814d30728c2e3211267d7358518d8e55056f1c08d3c4baff2c8`

복구된 원본 bootstrap source hash는 `index.html` `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `root-app.js` `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다. 기본 앱을 다시 build·install한 뒤 앱은 모두 정지했고 iOS/Android profile은 shutdown 및 auto-night 상태다. task 전용 profile은 만들지 않았다. 현재 확인에서도 S03 profile, 실행 중인 Simulator.app/Android emulator, 연결된 ADB 기기는 없었다. CoreSimulator 서비스는 macOS 공용 서비스여서 종료하지 않았다.

Build 로그는 `ios-build.log`, `android-build.log`, 기본 입력 재-build는 `ios-default-build.log`, `android-default-build.log`다. Source code는 작업 tree HEAD `a7f96c78c3169a4a3f9aeed9f01d7e6f18d5a1d8`를 기준으로 했다.

기록 로그의 내용과 시각은 유지하고, Git 공백 검사를 위해 줄 끝 공백과 마지막 반복 빈 줄만 정규화했다. [PR diff 실패 관점 20개](./pr-diff-review-20.md)도 원격 PR #1322의 실제 파일 목록과 head SHA를 대조해 별도로 작성했다.

이 폴더의 화면·로그·fixture·검토 파일 SHA-256은 `SHA256SUMS`에 기록한다. Manifest는 자기 자신의 해시는 포함하지 않는다.
