# Android API 37 Emulator 실패 분석

이 기록은 API 37 Android smoke를 준비하는 중 관측한 두 Emulator 종료 원인을 구분한다. 둘 다 앱 프로세스가 아니라 QEMU/에뮬레이터 시작 단계에서 일어났다.

## 새 임시 AVD cold boot

- `spinon_c114_fourfit_smoke_20261113` ARM64 profile은 첫 cold boot 중 정상 ADB device로 등록되기 전에 멈췄다.
- Emulator 로그에는 `qemu_mprotect__osdep: mprotect failed: Permission denied`가 반복됐다.
- macOS crash report는 `qemu-system-aarch64-headless`의 `EXC_BAD_ACCESS / SIGSEGV`를 기록했고, hang detector가 dump를 만드는 경로에서 종료됐다. CPU/main-loop hang 뒤 crash reporter가 실패한 것으로 나타난다.
- 이 임시 profile은 이후 재사용하지 않으므로 제거했다. 원본 console/log 기록은 [`emulator.log`](./emulator.log)와 [`avd-create.log`](./avd-create.log)다. macOS `.ips`에는 host 세부 정보가 있어 저장소에 복사하지 않았다.

## 기존 공유 AVD의 GUI 시작

- 기존 `spinon_api37_0_applied_txn` snapshot profile을 GUI로 열 때 QEMU가 이전 crash-consent dialog를 표시하려 했다.
- macOS crash report의 stack은 `QBrush::QBrush()`에서 QPainter, QWidget repaint, `CrashConsentProvider::showDialog`로 이어진다. 이는 Android 앱 렌더링이 아니라 crash-consent 창 자체의 Qt paint 경로에서 난 종료다.
- 이 실패 기록은 [`shared-avd-emulator.log`](./shared-avd-emulator.log)에 있다. 공유 profile은 다음 C11 검증에서 쓸 수 있어 삭제하지 않았다.

## 같은 공유 AVD의 headless smoke

- 동일 profile을 `-no-window -no-snapshot-save -no-boot-anim -no-audio -gpu swiftshader_indirect`로 시작하자 API 37 / Android 17 / ARM64 device로 등록됐다.
- 기본 `index.html` route를 light와 dark로 실행했으며 두 경우 모두 `layout=ready`, `boxes=3`, `SPINON_C0410_DRAW status=0 presented`를 기록했다. 원본은 [`shared-avd-headless-smoke.log`](./shared-avd-headless-smoke.log), [`light.log`](./light.log), [`dark.log`](./dark.log)다.
- APK는 smoke 뒤 제거했고 night mode를 복구하고 emulator를 종료했다. 현재 이 작업에서 booted Android device는 없다. 이 smoke는 Root HTML startup 확인이며 four-fit mixed Grid fixture, PixelCopy 또는 GPU/hardware parity 확인이 아니다.

따라서 처음 관측한 실패를 “ADB가 앱을 실행하지 못했다”로 요약하면 부정확하다. ADB device 부재는 cold-boot 실패의 결과였고, GUI 시작은 별도 Qt crash-consent 경로에서 실패했으며, snapshot headless 경로에서는 앱 smoke가 통과했다.
