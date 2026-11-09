# Android 시뮬레이터 실행 상태

- 재사용 AVD `spinon_api37_2_presenttime_probe_20261018`에서는 앱 설치 후 기본 Activity 실행이 실패했다. 재부팅 후 PackageManager가 `/data/app/.../dev.spinon.bootstrap.../base.apk`를 parse하지 못해 invalid package로 삭제했고, 기존 `PackageSetting`은 metadata 누락으로 건너뛰었다. APK manifest는 로컬 `aapt` 검사에서 launchable `MainActivity`를 포함했다.
- 기존 앱 데이터는 삭제하지 않았다. 사전 설치 APK `0.1.0-bootstrap`을 `adb install -r -d`로 복구했고 명령 결과는 `Success`였다. 다만 복구 확인은 사전 재부팅 때 기록한 오래된 APK 경로를 사용해 `No such file or directory`가 났으므로, 복구된 설치의 launch 동작은 확인되지 않았다. AVD는 종료했다.
- 재사용 프로필을 건드리지 않도록 별도 `spinon_c114_tmp_20261109` AVD를 `system-images;android-37.2;google_apis_ps16k;arm64-v8a`, Pixel 7로 만들었다. Android Emulator 37.2.12.0은 QEMU 프로세스와 console/ADB 포트를 열었지만 5분 39초 뒤에도 `adb devices -l`에서 `emulator-5554 offline`이었고 `getprop sys.boot_completed`는 `device offline`으로 실패했다.
- 임시 AVD는 앱을 설치하거나 데이터를 쓰지 않았으며 종료·삭제했다. 따라서 이번 slice의 Android 앱 실행/화면 검증은 미수행이다. 관찰값만으로 PackageManager parse 실패의 근본 원인을 앱 또는 에뮬레이터 중 하나로 확정하지 않는다.
