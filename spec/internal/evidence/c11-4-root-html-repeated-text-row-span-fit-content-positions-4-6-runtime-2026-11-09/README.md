# C11.4 반복 텍스트 row-span fit-content 위치 4–6 런타임 검증

기준 실행 날짜: 2026-11-09. 이 결과는 bounded implementation slice의 근거이며 C11.4/C11 전체 완료를 뜻하지 않는다.

## iOS Simulator

iPhone 17 Pro, iOS 26.2의 기본 Root HTML 앱 경로에서 여섯 sentinel을 실행했다. `fit-4-start`, `fit-4-stretch`, `fit-5-start`, `fit-5-stretch`, `fit-6-start`, `fit-6-stretch` 모두 앱 boot, layout-ready, `SPINON_C0410_DRAW status=0 presented` 로그와 OS screenshot을 남겼다. 각 scene 디렉터리의 `app.log`, `source.txt`, `screenshot.png`를 확인할 수 있다.

이 화면 캡처는 시각 smoke 근거다. Metal readback이나 compositor pixel parity 검증은 아니다.

## Android 에뮬레이터

Android 화면 결과는 미확인이다. 기존 API 37 ARM64 AVD 두 개를 각각 `-gpu swiftshader -no-snapshot -no-boot-anim -no-window -no-audio`로 시작했지만 둘 다 앱 설치 전에 QEMU `SIGSEGV`로 종료됐다. 두 실행 모두 ADB가 잠시 `device` 상태를 보인 뒤 연결을 잃었고, QEMU 로그에는 `qemu_mprotect__osdep: mprotect failed: Permission denied`가 반복됐다.

- `spinon_c114_percentage_max_20261023`: [emulator-swiftshader-segv.log](./android/emulator-swiftshader-segv.log)
- `spinon_c11_content_alignment_20261103`: [emulator-c11-content-alignment-segv.log](./android/emulator-c11-content-alignment-segv.log)
- 처음 실패한 기본 GPU/snapshot 실행: [emulator-qemu-hang.log](./android/emulator-qemu-hang.log)

실패 위치는 Android 앱 동작이나 ADB 설치 명령이 아니라 macOS 호스트 QEMU 실행 단계다. 따라서 이 결과로 Android 화면 동작을 판정하지 않는다. 추가 AVD를 만들거나 기존 AVD 데이터를 지우지 않았다.

QEMU/Xcode 출력의 원본 로그는 [압축 원본](./checks/platform-logs-originals-2026-11-09.tar.gz)에 보존했다. 개별 `.log` 파일은 diff 검사에 필요한 끝 공백만 제거한 읽기용 사본이다.

## 종료·복구 확인

- 두 Android 실행 뒤 `adb devices`에 연결된 에뮬레이터가 없고 QEMU 및 smoke 스크립트 프로세스도 종료됐다.
- `xcrun simctl list devices booted`에는 부팅된 기기가 없었다. iPhone 17 Pro도 shutdown 상태다.
- “S03 Pilot”이라는 iOS 기기는 CoreSimulator 목록에 없다.
- 재확인 시점에 `Simulator.app` 및 simulator smoke 프로세스가 없었고, CoreSimulator 백그라운드 서비스만 유휴 상태로 남아 있었다. 검증 뒤 설치되어 있던 `dev.spinon.bootstrap` 테스트 앱은 제거했고, 다음 iOS 확인에 재사용할 기본 iPhone 17 Pro 기기는 shutdown 상태로 유지했다. 기본 Xcode 기기 목록과 다른 프로젝트 소유 기기는 작업 전용 인스턴스가 아니므로 삭제하지 않았다.
- `examples/bootstrap/index.html` 및 `examples/bootstrap/root-app.js`는 실행 전 SHA-256으로 복원됐다.
- 작업용 Agent Browser 세션은 이 검증에서 시작하지 않았다. 이미 종료된 Spinon 세션을 다시 열지 않았다.

## 비시뮬레이터 검사

`checks/`에는 구현 및 회귀 결과가 있다: 위치 4–6 4,320 scene, 위치 1–3 1,920 scene, 반복 텍스트 810, 기존 row-span 960, auto-placement fit-content 회귀, Grid track-sizing 127건, focused Clippy, Rust fmt, vendored rustfmt, `git diff --check` 통과.

구현/runtime 변경은 별도 [적대적 검토 20개](./implementation-runtime-review-20.md), 구현 PR 전체 diff는 서로 다른 [30개 관점](./pr-diff-review-30.md) 검토를 마쳤다. 구현 검토에서 writing-context admission 누락을 발견해 수정했고, 영향받는 거부 검사를 포맷 적용 후 다시 실행해 통과했다([첫 실행](./checks/row-span-writing-context-rejections.log), [포맷 후 재실행](./checks/row-span-writing-context-rejections-pass02.log)). Android API 37 실행과 broad layout suite의 미확인 실패는 별도 검증 한계로 남아 있다.

별도로 `cargo test -p spinon-layout --lib`은 509 pass, 1 fail이었다. 실패는 `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order`이며 기준 브랜치에서 재현 여부를 확인하지 못했다. 이를 기존 실패라고 단정하지 않는다.
