# API 37.1 AVD 첫 부팅 launcher 수명 오류

첫 재생성 AVD boot는 `sys.boot_completed=1`, API 37.1, 16KB page size, ARM64, AVD name gate를 통과했다. emulator를 shell background child로 시작했으나, 그 명령 세션이 끝난 뒤 QEMU process가 존재하지 않았고 ADB에는 다른 프로젝트의 `emulator-5554`만 남았다.

callback fixture runner는 `emulator-5580`이 없는 것을 발견하고 APK build 전에 실패했다. 이 실패는 callback fixture 결과가 아니며 `run-02-fresh-avd` 결과 폴더도 생성되지 않았다. `emulator-startup.log`에는 boot 완료 로그가 남았고 QEMU 종료 오류는 확인되지 않았다. 따라서 Android guest crash로 단정하지 않고, launcher shell 종료 뒤 background process가 유지되지 않은 것으로 분류한다.

후속 재실행은 QEMU를 background child로 띄우지 않고 유지되는 실행 세션에서 foreground로 시작한다. fixture 직전에 QEMU PID, `adb devices`, `adb get-state`, API·page-size tuple을 다시 확인한다.

| 확인 항목 | 관측 |
|---|---|
| 실행한 AVD | `spinon_api37_1_compat16k` · Pixel 6 profile |
| 첫 boot | 완료 · API 37.1 · guest page size 16,384 · ARM64 |
| 다음 단계 직전 | QEMU PID 없음 · `emulator-5580` ADB serial 없음 |
| fixture | 시작되지 않음 · 빌드 및 assertion 미실행 |
| 보정 | 유지되는 process session에서 foreground 실행 후 재확인 |
