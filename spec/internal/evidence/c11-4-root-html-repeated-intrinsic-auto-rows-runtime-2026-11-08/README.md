# C11.4 반복 intrinsic implicit-row 목록 실행 근거

## 범위

구현 PR [#1284](https://github.com/ohah/spinon/pull/1284)은 `2026-11-08T06:43:38Z` Rebase and merge로 병합됐다(merge SHA `f2f9312e39ddf6d7ca8e703c28d33bf26e5325b8`). PR checks/workflow run은 보고되지 않았고, main에는 required status check이 없었다.

기본 parsed Root HTML GPU route에서 다음 exact `grid-auto-rows` 목록과 첫 두 순환 주기를 실행했다.

```css
fit-content(20px) min-content max-content auto min-content max-content
```

이 slice는 definite 40×100 CSS px Grid, 40px column, 10px explicit row, 여덟 implicit item, 고정 payload 높이 `[20, 10, 12, 6, 6, 6, 60, 6]px`, `align-content:start|stretch`만 허용한다. 이는 general CSS admission, arbitrary intrinsic content, WPT 또는 전체 C11.4 검증이 아니다. 규범·oracle·독립 기대값과 화면 제한은 [계획](../../../../plan/c11-4-root-html-repeated-intrinsic-auto-rows.md) 및 [Chrome 157 사전 비교](../c11-4-root-html-repeated-intrinsic-auto-rows-precomparison-2026-11-08/README.md)를 참고한다.

## Rust 검증

| 범위 | 결과 |
| --- | --- |
| Layout repeated-intrinsic | 2/2 |
| Style projection repeated-intrinsic | 2/2 |
| RuntimeSession repeated-intrinsic | 3/3 |
| 세 crate의 `fit_content` focused 회귀 | 79 passed, 0 failed (Layout 35, Runtime 29, Style projection 15) |
| 전체 workspace Clippy | `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과 |
| 서식 | `cargo fmt --all -- --check` 통과 |

원본 focused test 출력은 [rust-fit-content-tests.log](./rust-fit-content-tests.log), 전체 workspace Clippy 출력은 [workspace-clippy.log](./workspace-clippy.log)에 저장했다. Layout의 두 positive test는 `start`의 used rows `[10,20,10,12,6,6,6,20,6]px`와 `stretch`의 `[10,20,10,12,10,6,6,20,6]px`를 확인한다. 네 px/percentage cap·alignment scene의 Chrome 157 관찰과 독립 기대값은 4/4 exact다.

## Android API 37 ARM64 AVD

- 기기: `emulator-5554`, Android API 37, `arm64-v8a`, emulator 확인값 `1`; 실기기가 아니다.
- 기본 route `index.html`에 fixture SHA-256 `77874483d57af866e9215118d559e3140c4fbf61cba568493288bf94a62d231f` (3007 bytes)가 들어간 것을 설치 전후 확인했다.
- 새 앱 PID 18422에서 `layout=ready boxes=20`과 `SPINON_C0410_DRAW status=0 presented boxes=20`을 확인했다.
- [화면](./android/pass02/final/screenshot.png), SHA-256 `0b6d2379fbf039203e54c7310faeb897caba5cb42235293c8469405bd965695e`.
- 캡처 후 기존 Android APK를 다시 설치해 앱을 실행했다. 복원 launch는 [로그](./android/pass02/final/restore-launch.log)에 있다.

## iPhone 17 Pro / iOS 26.2 Simulator

- 기기 이름 `Spinon-C114-FitContent-Pass04`, UDID `97E99AA8-D144-4DA8-B1BF-7F6B34007F8B`; simulator metadata는 [device-metadata.txt](./ios/pass02/final/device-metadata.txt)에 있다.
- 동일 fixture hash와 `html_bytes=3007`을 확인했다. 새 앱 PID 12125에서 environment layout과 Root HTML boot 모두 `layout=ready boxes=20`이었다.
- [화면](./ios/pass02/final/screenshot.png), SHA-256 `7d0d9a711fc0d2dd82855587679a664f83d672bad00838b0a0c91bb3a38a47b0`.
- 설치 전과 기존 앱 복원 후 app-data manifest는 각각 11개 파일이며 [before](./ios/pass02/final/data-container-before.sha256)와 [after](./ios/pass02/final/data-container-after-restore.sha256)가 byte-for-byte 같다. 경로가 변경되어 파일들을 기존 백업에서 복원했고 기존 app bundle도 재설치했다. 복원 launch/process 로그는 `restore-*.log`에 있다.

## 실행기 수정과 해석 한계

첫 capture 시도는 Android runtime이 `boxes=20`과 draw-presented를 보고했는데 실행기가 예상값을 `18`로 고정해 검증을 중단했다. 이는 제품 layout 실패가 아니라 harness의 기대 count 오류였다. 원본 pass는 `android/final/` 및 `ios/final/`에 보존했고, 실행기 기대값을 20으로 고친 뒤 위 pass02를 새로 실행했다. pass02 스크립트는 종료 handler로 이전 설치 앱을 복원하고 iOS app data를 백업 파일에서 되돌린다. 결과를 확인해 스크립트의 완료 문구도 실제 20 boxes로 정정했다.

화면은 OS screenshot을 눈으로 확인한 visual smoke evidence다. Android SurfaceView PixelCopy, iOS Metal drawable readback, hardware GPU, compositor 후 pixel parity를 주장하지 않는다. build와 캡처에서 쓴 백업 artifact는 `build/spinon/c11-4-root-html-repeated-intrinsic-auto-rows-20261108/` 아래 보존했고 삭제하지 않았다.
