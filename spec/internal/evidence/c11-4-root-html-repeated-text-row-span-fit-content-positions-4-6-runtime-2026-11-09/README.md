# C11.4 반복 텍스트 row-span fit-content 위치 4–6 런타임 검증

기준 실행 날짜: 2026-11-09. 이 결과는 bounded implementation slice의 근거이며 C11.4/C11 전체 완료를 뜻하지 않는다.

## iOS Simulator

iPhone 17 Pro, iOS 26.2의 기본 Root HTML 앱 경로에서 여섯 sentinel을 실행했다. `fit-4-start`, `fit-4-stretch`, `fit-5-start`, `fit-5-stretch`, `fit-6-start`, `fit-6-stretch` 모두 앱 boot, layout-ready, `SPINON_C0410_DRAW status=0 presented` 로그와 OS screenshot을 남겼다. 각 scene 디렉터리의 `app.log`, `source.txt`, `screenshot.png`를 확인할 수 있다.

이 화면 캡처는 시각 smoke 근거다. Metal readback이나 compositor pixel parity 검증은 아니다.

## Android 에뮬레이터

기존 `spinon_api37_2_presenttime_probe_20261018` AVD에서 Android 17 / API 37.2 / ARM64, Pixel 6, 1080×2400, `-gpu swangle`로 여섯 sentinel을 실행했다. 각 scene에서 source와 APK 내부 `assets/index.html` SHA-256 일치, 앱 프로세스, Root HTML boot/layout-ready, `SPINON_C0410_DRAW status=0 presented`, OS screenshot을 확인했다. 상세 기록은 [device.txt](./android/device.txt) 및 각 [scene 결과](./android/)에 있다. 스크린샷은 visual smoke이며 semantic geometry 비교, PixelCopy 또는 GPU pixel parity 검증은 아니다.

다른 기존 API 37 ARM64 AVD 두 개는 각각 `-gpu swiftshader -no-snapshot -no-boot-anim -no-window -no-audio`로 시작했지만 앱 설치 전에 QEMU `SIGSEGV`로 종료됐다. 두 실행 모두 ADB가 잠시 `device` 상태를 보인 뒤 연결을 잃었고, QEMU 로그에는 `qemu_mprotect__osdep: mprotect failed: Permission denied`가 반복됐다.

- `spinon_c114_percentage_max_20261023`: [emulator-swiftshader-segv.log](./android/emulator-swiftshader-segv.log)
- `spinon_c11_content_alignment_20261103`: [emulator-c11-content-alignment-segv.log](./android/emulator-c11-content-alignment-segv.log)
- 처음 실패한 기본 GPU/snapshot 실행: [emulator-qemu-hang.log](./android/emulator-qemu-hang.log)
- API 37.2의 최초 재시도는 호스트 여유 공간 부족으로 실패했고, 사용자가 앞서 요청한 캐시 정리 범위에 따라 `cargo clean`으로 58.8 GiB를 비운 뒤 같은 AVD가 부팅됐다. [최초 재시도](./android/emulator-api37-2-swangle-boot-probe.log) · [정리 후 부팅](./android/emulator-api37-2-swangle-after-clean.log)

실패한 두 AVD의 오류는 Android 앱 동작이나 ADB 설치 명령이 아니라 macOS 호스트 QEMU 실행 단계였다. 성공한 API 37.2 결과는 별도 AVD·별도 실행으로 기록하며 실패한 두 실행을 덮어쓰거나 일반화하지 않는다. 추가 AVD를 만들거나 기존 AVD 데이터를 지우지 않았다.

처음 실패한 QEMU/Xcode 출력의 원본은 [첫 실행 압축본](./checks/platform-logs-originals-2026-11-09.tar.gz), 성공한 Android/iOS 재실행의 원본은 [최종 smoke 압축본](./checks/platform-logs-final-smoke-2026-11-09.tar.gz)에 보존했다. 개별 `.log` 파일은 diff 검사에 필요한 끝 공백과 빈 마지막 줄만 제거한 읽기용 사본이다.

## 종료·복구 확인

- Android 성공 실행 뒤 `adb devices`에 연결된 에뮬레이터가 없고 QEMU 및 smoke 스크립트 프로세스도 종료됐다.
- `xcrun simctl list devices booted`에는 부팅된 기기가 없다. iPhone 17 Pro와 API 37.2 AVD는 재사용할 수 있도록 등록 상태로 두고 shutdown했다.
- “S03 Pilot”이라는 iOS 기기는 CoreSimulator 목록에 없다.
- 재확인 시점에 `Simulator.app` 및 simulator smoke 프로세스가 없었고, CoreSimulator 백그라운드 서비스만 유휴 상태로 남아 있었다. iOS와 Android의 `dev.spinon.bootstrap` 테스트 앱은 제거했고, 다음 확인에 재사용할 iPhone 17 Pro와 API 37.2 AVD는 shutdown 상태로 유지했다. “S03 Pilot” 이름의 iOS 기기는 등록되어 있지 않아 삭제 대상이 없었다. 기본 Xcode 기기 목록의 기본 기기들은 다른 작업에도 쓸 수 있으므로 보존했다. runner는 앞으로 실행한 테스트 앱을 종료 시 uninstall하고, 플랫폼별 실행 기록을 서로 덮어쓰지 않는다.
- `examples/bootstrap/index.html` 및 `examples/bootstrap/root-app.js`는 실행 전 SHA-256으로 복원됐다.
- 작업용 Agent Browser 세션은 이 검증에서 시작하지 않았다. 이미 종료된 Spinon 세션을 다시 열지 않았다.

## 비시뮬레이터 검사

`checks/`에는 구현 및 회귀 결과가 있다: 위치 4–6 4,320 scene, 위치 1–3 1,920 scene, 반복 텍스트 810, 기존 row-span 960, auto-placement fit-content 회귀, Grid track-sizing 127건, focused Clippy, Rust fmt, vendored rustfmt, `git diff --check` 통과.

구현/runtime 변경은 별도 [적대적 검토 20개](./implementation-runtime-review-20.md), 구현 PR 전체 diff는 서로 다른 [30개 관점](./pr-diff-review-30.md) 검토를 마쳤다. 구현 검토에서 writing-context admission 누락을 발견해 수정했고, 영향받는 거부 검사를 포맷 적용 후 다시 실행해 통과했다([첫 실행](./checks/row-span-writing-context-rejections.log), [포맷 후 재실행](./checks/row-span-writing-context-rejections-pass02.log)). Android API 37.2 시각 smoke는 통과했으며 broad layout suite 실패 한 건의 기준 브랜치 재현 여부는 미확인이다.

별도로 `cargo test -p spinon-layout --lib`은 509 pass, 1 fail이었다. 실패는 `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order`이며 기준 브랜치에서 재현 여부를 확인하지 못했다. 이를 기존 실패라고 단정하지 않는다.
