# Android API 29 unwind 연결 수정 실행 근거

## 변경

고정 Android NDK 27.1.12297006의 AArch64 libunwind archive를 Android bootstrap DSO에 정적으로 연결했다. NDK에서 archive 후보가 없거나 여러 개면 link 전에 중단하고, llvm-nm이 _Unwind_Resume 정의를 찾지 못해도 기존 산출물을 덮어쓰지 않는다. libunwind 구현 심볼은 DSO dynamic export에서 제외한다. V8 revision, NDK pin, C++ runtime 선택, JS API는 변경하지 않았다.

## 산출물과 검사

- 수정 Debug APK SHA-256: 1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e, 48,405,806 bytes.
- APK 내부 stripped bootstrap DSO SHA-256: af5783e5a989467a5d2e1ce822d89d9dc472102859b277e200a77c763f16305a, 48,166,104 bytes.
- 링크 직후 unstripped DSO SHA-256: bbd40e9e8b0d297457ed405ad19f49e35b181b032a4c4cfbbabc20bf255bbdf0, 128,983,232 bytes.
- dynamic undefined _Unwind_* 0개, dynamic export _Unwind_* 0개, libunwind DT_NEEDED 0개.
- PT_LOAD alignment 0x4000 유지, GNU_RELRO 끝 0x2dd4000, APK zipalign -P 16 검사 통과.
- APK 및 설치본은 API 29·34 AVD와 API 36 실기기에서 동일한 APK hash로 확인했다. 대형 APK 사본은 git에 포함하지 않고 build/spinon/android/api29-unwind-link-evidence 아래 보존한다.
- build-android.sh의 bash -n, Debug DSO 링크, Gradle assembleDebug가 통과했다. Gradle은 AGP 8.13.2가 compile SDK 37.2에 검증되지 않았다는 경고를 냈지만 빌드는 성공했다.
- archive 누락, _Unwind_Resume 정의 누락, archive 후보 중복의 세 preflight 부정 대조는 모두 명확히 실패했고 이전 DSO hash를 보존했다.

## API 29·34 ARM64 AVD

새 APK로 Android Emulator 37.2.12 SwiftShader AVD를 각각 실행했다. API 29와 API 34에서 기본 JS bootstrap, async fence의 api_below_35 종료, GLES present-fence와 FrameTimeline fallback의 draw 및 색상 갱신을 확인했다. API 29의 이전 실패 원인이던 native library load는 새 APK에서 통과했다. 이 두 경계는 API 30–33 전체를 증명하지 않는다.

## 사용자 요청 Android 실기기 회귀 확인

Samsung SM-S731N / Android 16 / API 36 / 1080×2340 실기기에 같은 Debug APK를 설치해 cold launch와 별도 R05 WGPU probe process를 실행했다. 설치 APK hash가 빌드 APK와 일치했다. JS bootstrap 결과가 기록됐고 R08 WGPU는 Samsung Xclipse 940의 Vulkan backend로 준비됐다.

ADB synthetic tap 13회는 input sequence/revision 1–13, WGPU submit 13회, 실제 TransactionStats callback 13회, usable async fence signal 13회로 1:1 연결됐다. 마지막 완료 시 pending/active/queue depth는 모두 0이었다. 화면 캡처에서 activation count 13과 파랑에서 주황으로 바뀐 GPU 도형을 확인했다. raw sec_touchscreen 수집에서 synthetic tap contact는 0건이었다.

직접 손가락 입력 positive sample은 수집하지 못했다. sec_touchscreen을 direct 입력 장치로 확인하고 앱 화면을 foreground로 둔 약 30초 capture에서 raw event는 0건이었다. 따라서 이 실행은 실기기 합성 입력 회귀 확인이지 직접 touch 검증은 아니다. 앱의 input_source는 unknown이고 target_vsync_id는 -1이므로 event-to-present latency, VSync, optical scanout을 계산하지 않는다.

실기기 검증 후 앱 process를 종료하고 Chrome을 foreground로 복귀시켰다. 화면 timeout, 밝기 모드와 밝기 값은 전후 동일했다. 기기 일련번호는 기록하지 않았다.

## 남은 경계

API 30–33, 직접 손가락 입력, iOS 실행, 성능 비교와 표시 지연, Release APK의 NDK notice/license 배포 처리는 이번 변경으로 검증하지 않았다. NDK 설치 디렉터리의 NOTICE 파일 존재만 확인했으며 Release 배포에 필요한 고지 파일 수록 여부와 문구는 별도 확인이 필요하다. R05.3과 R05는 계속 미완료다.

상세 실행 로그, 환경, 화면과 checksums는 이 디렉터리의 api29, api34, link, physical 하위에 있다. [계획과 사전 검토](../../../../plan/r05-android-api29-unwind-link.md) 및 [구현 후 검토](implementation-review.md)를 따른다. source-manifest.txt는 기반 commit과 수정 script·artifact hash를 연결한다.
