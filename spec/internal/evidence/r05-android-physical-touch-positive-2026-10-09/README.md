# Android 실기기 입력 출처 대조와 Release smoke

- **실행일:** 2026-10-09
- **기준 commit:** `85b39b48d28f5df7067d751dc9d47722d7b50e9e` (PR #80 병합 후)
- **기기:** Samsung SM-S731N · Android 16 · API 36 · `SDK_INT_FULL=36.1` · ARM64 · 1080×2340
- **연결:** 실기기 USB 1대만 연결됨. 일련번호는 기록하지 않음.
- **전체 판정:** Release 컴파일·기기 실행·debug-only 거부·원래 debug APK 복구는 통과. 직접 손가락 입력 positive sample은 수집되지 않아 R05.3은 미완료.

## 실제 touchscreen 입력 표본

PR #79의 debug APK를 새 process로 실행했고 설치본 SHA-256은 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`로 기존 고정 artifact와 일치했다. R08은 Android의 Vulkan backend에서 Samsung Xclipse 940을 선택했고 1080×2340 `SurfaceView`를 준비했다. 시작 화면은 [실기기 캡처](physical/launch.png)다.

`/dev/input/event6`은 실행 시점에 `sec_touchscreen` 이름과 `INPUT_PROP_DIRECT` capability로 확인했다. 32분 30초 동안 raw event와 PID 29740의 app log를 함께 수집했지만 raw touchscreen 파일은 0바이트였고, app log에도 R05 input action이 없었다. 따라서 직접 입력 sample 수는 0이며, 입력→submit→callback→fence 연결이나 지연을 이 실행에서 주장하지 않는다. 두 원본은 [raw touchscreen](physical/raw-touchscreen.txt), [app log](physical/app-logcat.txt)다.

대조군의 ADB synthetic tap 한 건은 app `input_seq=1`에서 R08 submit, Android `TransactionStats` callback, async fence `signaled`까지 연결됐고 종료 queue는 0/0/0이었다. 함께 수집한 touchscreen raw event는 0건이라 synthetic 입력과 direct touchscreen 입력을 구분할 수 있었다. 이 대조군은 physical sample이 아니다. 상세 원본은 [synthetic control 로그](control/app-logcat.txt), [raw 입력](control/raw-touchscreen.txt), [control 화면](control/after-one-synthetic-tap.png)이다. 이 Android API 36 실행의 `target_vsync_id`는 `-1`이므로 VSync, 패널 scanout, 광학 표시 시각, 제품 입력 지연 또는 p95를 주장하지 않는다.

## Android Release 빌드와 기기 smoke

V8 checkout은 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`에서 완료됐고, `tools/v8/build-android-macos.sh`가 Android arm64 Release 정적 라이브러리를 빌드했다. SDK에 설치된 NDK `27.1.12297006`을 사용했다. V8 빌드 스크립트가 임시로 바꾼 `BUILDCONFIG.gn`은 복구됐고 임시 NDK 링크는 제거됐다.

`ANDROID_HOME`과 `ANDROID_SDK_ROOT`를 설치된 SDK로 지정한 `:app:assembleRelease`가 성공했다. 결과는 `dev.spinon.bootstrap`, version `0.1.0-bootstrap`, target API 36의 unsigned APK이며 SHA-256은 `ab0fbea974b70e6c24ec2db06abdae66015740cd7a56eb51530f14b7dd146a83`이다. 전체 로그는 [release-build.log](release-build.log)다. 빌드 로그에는 Android Gradle Plugin 8.13.2가 compile SDK 37.2까지 검증되지 않았다는 경고와 SDK XML version 4 경고가 남아 있다.

기기 설치 시험을 위해 unsigned APK의 임시 사본을 로컬 Android debug keystore로 서명했다. 서명본 SHA-256은 `cdd9cadd1ff7d7bb26a317a187b7119200e268b97b8af8f0aab3477ec86bfdcf`이고 설치 전 서명 SHA-256 지문이 현재 설치본과 같은 `65c84c2a21e0f0fe777858993c5484f9da8f66ba623203af2d351d5237ad1a97`임을 확인했다. 따라서 같은 패키지·version·인증서의 update 설치만 했고 앱을 uninstall하지 않았다. 이 서명은 Release runtime smoke용 debug 인증서이며 production signing 검증이 아니다.

Release 기본 실행은 cold launch `Status: ok`, `TotalTime: 251 ms`를 반환했고, PID 528에서 `SPINON_BOOTSTRAP_EXECUTION is_main_thread=false`와 `SPINON_BOOTSTRAP_RESULT=nodes=2 ... document_nodes=10`을 기록했다. 기본 bootstrap 경로는 GPU fixture가 아니며 [화면은 빈 배경](physical/release-default.png)이다. 이어 `spinon_r05_async_fence_wait=true`를 전달한 Release 실행은 PID 1617에서 `SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=debug_only`를 남기고 Activity를 종료했다. 즉 release에서 debug 전용 실험을 실행하지 않는 동작을 확인했다. [기본 실행 로그](physical/release-default.log) · [debug-only 거부 로그](physical/release-debug-only-rejection.log).

smoke 후 PR #79와 동일한 debug APK를 다시 설치했다. 복구된 설치본 SHA-256은 다시 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`였다. Chrome을 foreground로 복귀시켰다. 전후 설정은 밝기 248, 화면 timeout 30,000 ms, 충전 중 화면 켜짐 값 3, 활성 mode 2 / 60 Hz로 같았고 화면은 `Awake`였다.

## 남은 확인

- 사용자가 target을 직접 누른 `sec_touchscreen` contact와 app `ACTION_UP`이 함께 관측된 positive sample.
- positive sample마다 input sequence, revision, surface generation, WGPU submit, 실제 transaction callback, async fence signal을 exact join하는 결과.
- 위 physical sample이 없으므로 latency 계산, VSync·scanout 검증, R05.3 완료 판정은 보류한다.
- production signing key로 서명한 배포 APK의 설치·실행은 이번 smoke 범위가 아니다.

별도 실패 관점 검토는 [runtime-review.md](runtime-review.md)에 있다.

저장소 diff 검사를 위해 텍스트 로그의 행 끝 공백만 제거했다. timestamp와 메시지 내용은 유지했다.
