# R05.3 · Android API 29 native unwind 연결 계획

**상태:** 구현 완료 · 계획/구현 독립 검토 완료 · API 29·34 ARM64 AVD 통과 · Android 16 실기기 synthetic 회귀 통과 · API 30–33 미측정

**상위:** [R05 입력→표시 계측](r05-input-to-presentation.md) · [API 29·34 fallback 계획](r05-android-api29-34-fallback.md) · [상태 대장](../spec/STATUS.md)

## 문제와 범위

API 29 ARM64 AVD에 minSdk 29의 Debug APK가 설치되지만, `MainActivity`가 `System.loadLibrary()`를 호출할 때 `libspinon_bootstrap.so`에서 `_Unwind_Resume`를 찾지 못해 시작을 종료했다. 따라서 API 29에서는 bootstrap과 R05 fallback 분기 모두 실행되지 않았다. API 34에서는 같은 이전 artifact가 시작하고 GLES fallback 화면이 갱신됐다.

현재 `tools/build-android.sh`는 V8 고정 revision의 C++ 정적 아카이브와 V8 libc++/libc++abi를 링크하면서 `-nostdlib++ --unwindlib=none`을 전달한다. Android NDK 27.1.12297006의 AArch64 `libunwind.a`는 `_Unwind_*` 구현을 포함한다. 이 계획은 그 고정 NDK archive를 bootstrap DSO에 정적으로 연결해 API 29 loader 오류를 고치는 최소 빌드 수정만 다룬다. V8·NDK 버전, C++ 표준 라이브러리 선택, 앱 API, JS 엔진, 공개 동작 계약은 바꾸지 않는다.

## 비교 모델과 사전 기준

| 관측 | 기존 기준 | 수정 합격 기준 |
|---|---|---|
| API 29 첫 실행 | `UnsatisfiedLinkError: dlopen failed: cannot locate symbol "_Unwind_Resume"` | 같은 ABI와 AVD에서 MainActivity가 native load를 통과하고 기본 JS bootstrap 결과가 기록됨 |
| ELF | `_Unwind_*`가 undefined dynamic imports에 남고 `DT_NEEDED`에 unwind runtime 없음 | `_Unwind_*` 미해결 import 0개, `libunwind` 동적 의존성 0개, 16KB `PT_LOAD`/GNU_RELRO 정렬 유지 |
| API 29 진단 gate | library load가 먼저 실패해 실행 불가 | async fence는 `api_below_35`로 종료; present-fence와 FrameTimeline은 각각 `api_below_35`로 표시하고 GLES draw·화면 변경 유지 |
| API 34 regression | 기존 APK에서 bootstrap·fallback 화면 변경 성공 | 새 APK에서도 같은 결과; callback unavailable 상태와 draw 성공을 분리 기록 |
| 결과 범위 | API 29 원인만 확정, API 30–33 미측정 | API 29 시작과 API 29·34 경계만 검증; API 30–33은 미측정 유지 |

기준 artifact는 API 29·34 실패/성공을 재현한 PR #79 APK SHA-256 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`이다. 수정 artifact는 새로 빌드되므로 새 SHA-256을 기록하고 이전 hash와 혼동하지 않는다. link 단독 실험은 현재 worktree의 기존 object와 고정 V8 archive를 재사용해 임시 경로에 출력한다. 제품 APK를 만들 때도 `tools/v8/v8-revision.txt` 및 `tools/android-ndk-version.txt`의 고정 버전을 확인하며 V8 전체를 재빌드하거나 외부 source를 내려받지 않는다.

## 구현 내용

1. 선택된 고정 NDK host prebuilt 아래 `lib/clang/*/lib/linux/aarch64/libunwind.a`를 찾는다. 없거나 후보가 여러 개면 기존 `.so`를 덮어쓰기 전에 명시 오류로 중단한다.
2. 해당 archive를 `libc++abi.a` 뒤에 정적 입력으로 추가한다. V8 Clang은 NDK Clang과 resource version이 다르므로 자동 `--unwindlib=libunwind` 검색에 의존하지 않는다. 기존 `-nostdlib++ --unwindlib=none`과 NDK archive 직접 지정을 명확히 주석으로 설명한다.
3. `-Wl,--exclude-libs,libunwind.a`로 runtime 구현 심볼을 DSO의 공개 dynamic symbol table에 노출하지 않는다.
4. 16KB page-size·common-page-size, ARM64 ABI, V8/NDK revision, existing C++ archives 및 빌드 대상은 그대로 유지한다.
5. preflight가 실제 archive의 AArch64 unwind symbol을 제공하는지 확인하고, link 후 ELF와 APK에 대해 미해결 `_Unwind_*`, 동적 `DT_NEEDED`, export symbol, 16KB 정렬을 검사한다.

직접 archive link 실험에서 undefined가 사라지지 않거나 API 29 실행 결과가 개선되지 않으면 추가 flag를 이어붙이지 않는다. Android NDK linker/runtime 입력을 다시 조사해 계획과 비교 모델을 갱신한다.

## 실행 순서와 합격 기준

1. 기존 APK/DSO hash, V8·NDK revision, link command, API29 crash signature를 보존한다. 실제 Android 기기는 이 실행 대상이 아니며 설치·foreground·설정 상태를 건드리지 않는다.
2. 현재 object/archive를 사용해 임시 output에 unwind archive 포함/미포함 link 대조를 한다. 포함 결과에서 `_Unwind_*` undefined가 없어지고 unwind archive 심볼이 dynamic export에 나타나지 않는지 검사한다.
3. `tools/build-android.sh`로 bootstrap을 다시 링크한다. V8 checkout/아카이브 검증, Rust/Bun 단계가 예상 외로 전체 재빌드를 요구하면 중단하고 공간·입력을 재평가한다.
4. 새 ARM64 Debug APK를 만든다. APK에 든 `.so` hash가 방금 생성한 DSO hash와 일치하는지 확인한 뒤 API 29·34 AVD에만 설치한다.
5. 두 API에서 각각 새 process로 기본 bootstrap, async-fence guard, present-fence GLES fallback, FrameTimeline GLES fallback을 실행한다. API29의 세 진단 조건은 API35 전용 API를 호출하지 않고 graceful fallback해야 한다. 두 fallback 조건은 별도 process에서 실행한다.
6. 각 process의 guest API·APK hash·PID logcat·화면을 보존한다. fatal/linkage/verifier/ANR 부재와 기대 marker·화면 변경을 함께 확인한다. 모든 입력은 ADB synthetic이며 finger touch·latency 표본으로 계산하지 않는다.
7. AVD 두 개를 종료하고 정의·system image는 재사용을 위해 유지한다. Android 실기기는 연결 상태로 보존하고 ADB 연결 상태를 확인한다.

기능 합격은 API 29의 native load 및 bootstrap이 성공하고, API 29/34의 세 guard/fallback 분기가 사전 기준과 일치하며, 새 APK와 DSO hash가 일치하고, ELF ABI/page 정렬이 유지되는 것이다. API 29–34 전체 호환, API 30–33, physical touch, 성능·latency, optical scanout은 이 작업으로 완료 처리하지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

구현 전 계획을 저장소 스크립트·고정 dependency·현재 ELF·기존 API 29/34 실행 자료·임시 재링크 결과에 대조했다. 각 관점은 서로 다른 원인 또는 잘못된 판정 경로를 다룬다.

| # | 공격 관점 | 대조 근거 및 판정 |
|---:|---|---|
| 1 | NDK host prebuilt 경로를 macOS/Linux에서 고정해 archive를 못 찾거나 다른 host runtime 사용 | 현재 helper가 선택한 `darwin-x86_64` 아래에서만 찾았고 AArch64 후보가 1개였다. 구현은 helper가 반환한 host root 기준 탐색으로 제한한다. |
| 2 | V8 Clang이 NDK Clang보다 새로워 automatic unwind search가 틀림 | V8 Clang은 24, pinned NDK Clang runtime은 18이다. `--unwindlib=libunwind -###`는 `-l:libunwind.a`를 만들지만 V8 resource/runtime search path를 NDK Clang resource dir로 바꾸지 않는다. explicit archive가 필요하다. |
| 3 | archive가 없거나 여러 개일 때 임의 선택 또는 기존 산출물 덮어쓰기 | 현재 후보 1개를 확인했다. 계획에 후보 0/복수일 때 link 이전 fail-closed 조건을 둔다. |
| 4 | 잘못된 ABI archive를 연결 | 선택 archive member header의 `Machine`은 `AArch64`이며 경로에도 `aarch64`가 있다. |
| 5 | Android NDK 버전이 설정 환경에 따라 바뀜 | 저장소는 27.1.12297006을 고정하고 `spinon_android_ndk_dir`는 SDK 안의 해당 버전을 선택한다. 계획은 그 pin을 유지한다. |
| 6 | archive가 첫 crash symbol만 갖고 다른 unwind ABI를 빠뜨림 | `llvm-nm`에서 Resume, GetIP, RaiseException, DeleteException 등 사용되는 구현을 확인했고 full link 결과의 모든 undefined `_Unwind_*`는 0이다. |
| 7 | archive 순서가 libc++abi의 undefined 참조 해소보다 앞섬 | 기존 순서의 libc++abi 뒤에 archive를 넣은 임시 실제 link가 성공했다. 계획도 같은 순서를 고정한다. |
| 8 | duplicate symbols 또는 V8 ABI 중복으로 link 충돌 | 동일 object/archive 전체 link에서 lld duplicate-definition 오류가 없었다. 본 빌드 재실행에서도 link stderr를 보존한다. |
| 9 | static runtime symbols가 앱 DSO 공개 ABI로 새어 나감 | `--exclude-libs,libunwind.a`를 적용한 임시 DSO의 dynamic symbol table에서 `_Unwind_*` export는 0이었다. |
| 10 | 일부 unresolved unwind symbol이 남아 API29가 다른 이름으로 계속 실패 | 임시 DSO dynamic undefined `_Unwind_*`는 0, `DT_NEEDED`의 unwind 항목도 0이었다. 실제 API29 cold launch는 별도 합격 조건으로 남겼다. |
| 11 | API 29 linker가 새로운 symbol version 또는 loader ABI에서 거절 | API29 guest가 같은 old DSO에서 실제 `_Unwind_Resume`를 거절한 원본이 있다. 수정 APK의 cold launch와 logcat을 필수로 해 간접 ELF 근거만으로 통과 처리하지 않는다. |
| 12 | bootstrap을 통과하지만 API35 class resolution/verifier가 실행을 중단 | 기존 guard는 `Build.VERSION.SDK_INT < 35`를 먼저 검사한다. async/present/FrameTimeline 각 분기를 API29 새 process에서 따로 실행해 linkage·fatal·ANR을 확인한다. |
| 13 | unavailable 기록만 남기고 renderer draw를 생략 | API34 기존 실행에서 두 GLES 경로에 `draw_requested=true`와 화면 변경이 있다. API29 새 실행에서도 로그와 전후 캡처를 모두 요구한다. |
| 14 | 두 진단 extra가 합쳐져 하나의 branch만 타고 다른 branch를 잘못 통과 | MainActivity의 branch 우선순위가 있으므로 present-fence와 FrameTimeline은 독립 fresh process로 둔다. |
| 15 | API29 결과만 보고 API34 regression을 놓침 | 같은 수정 APK로 API34 bootstrap 및 세 진단 gate를 반복하는 별도 조건이 계획에 있다. |
| 16 | Gradle cache가 이전 JNI `.so`를 패키징해 거짓 통과 | 수정 DSO, APK 안의 `.so`, 설치 후 pull한 APK SHA-256을 실행 manifest에 단계별로 연결한다. 앱 logcat은 새 실행 PID 및 APK hash를 기록한 manifest와 연결한다. |
| 17 | 불필요한 4.5GB V8 rebuild 또는 외부 checkout으로 디스크를 소진 | V8 source revision은 pin과 일치하고 monolith·libc++·libc++abi 산출물이 이미 있다. 18GB 여유를 확인했으며 checkout/build 스크립트는 실행하지 않는다. |
| 18 | unwind 수정 과정에서 16KB loader 정렬을 깨뜨림 | 기존 link flags의 max/common page size는 16384다. 임시 DSO `PT_LOAD` 정렬은 0x4000이고 GNU_RELRO end는 0x2dd4000으로 유지됐다. |
| 19 | APK를 실기기에 설치하거나 사용자의 앱 상태를 바꿈 | ADB 현재 목록에서 SM-S731N serial과 AVD serial을 분리한다. 신규 APK 설치 명령은 API29·34 AVD serial로만 실행한다. |
| 20 | 테스트 후 emulator가 남거나 AVD 자료를 삭제 | 현재 실행 emulator는 없고 기존 두 fallback AVD/image가 보존돼 있다. 테스트 후 emulator process 종료·AVD 목록·물리 serial 보존을 재확인한다. |

### 계획 검토 판정

20개 대조에서 설계 결함은 발견하지 않았다. 실제 재링크가 계획한 archive를 포함하는지, API29 loader가 통과하는지, 앱의 세 R05 guard/fallback 분기와 API34 regression은 아직 구현 전이므로 계획 검토의 통과로 간주하지 않는다. 이들은 기능 구현 후 새로운 20개 관점으로 검토한다.

## 구현 및 실행 결과 · 2026-10-09

tools/build-android.sh에서 고정 NDK AArch64 libunwind archive를 후보가 정확히 하나일 때만 선택하고, _Unwind_Resume 정의를 link 전에 확인하도록 했다. 정적 archive를 libc++abi 뒤에 링크하고 libunwind 심볼을 DSO export에서 제외한다. archive 없음·필수 symbol 없음·후보 중복 세 부정 대조는 모두 link 전에 실패했고 이전 output hash를 유지했다. V8 전체 rebuild는 하지 않았다.

새 Debug APK hash는 1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e다. APK 내부 stripped DSO hash는 af5783e5a989467a5d2e1ce822d89d9dc472102859b277e200a77c763f16305a, 링크 직후 unstripped DSO hash는 bbd40e9e8b0d297457ed405ad19f49e35b181b032a4c4cfbbabc20bf255bbdf0다. 최종 DSO는 undefined/exported _Unwind_* 및 libunwind DT_NEEDED가 각각 0이고 PT_LOAD 0x4000 정렬, GNU_RELRO 끝 0x2dd4000, APK zipalign -P 16을 유지했다.

API 29와 API 34 ARM64 AVD에서 동일 APK의 cold launch와 JS bootstrap, async-fence guard, 별도 present-fence/GLES 및 FrameTimeline/GLES fallback을 재검증했다. 두 API에서 API 35 기능은 api_below_35로 표시하면서 draw와 화면 색상 갱신을 유지했고 fatal/linkage/verifier/ANR은 없었다. 실행 환경과 원본은 [실행 근거](../spec/internal/evidence/r05-android-api29-unwind-link-2026-10-09/README.md)에 있다.

사용자 요청으로 Android 16/API 36 Samsung SM-S731N 실기기에서 같은 APK를 설치해 별도 cold launch와 WGPU/Vulkan 회귀를 확인했다. 설치 hash가 빌드 APK와 일치했고, ADB synthetic tap 13회는 input/submit/TransactionStats callback/usable fence wait까지 13/13 연결됐다. 이 기기 결과는 API 29 증거가 아니며 직접 손가락 touch는 raw 입력에서 관측되지 않았다. target_vsync_id=-1이므로 latency·VSync·scanout은 측정하지 않았다. [실기기 원본과 캡처](../spec/internal/evidence/r05-android-api29-unwind-link-2026-10-09/physical/README.md)에 있다.

### 남은 범위

API 30–33, 직접 손가락 입력, iOS, 성능·latency·optical scanout, Release APK에서 NDK notice/license를 배포물에 포함하는 절차는 미검증이다. 설치 NDK의 NOTICE 존재만 확인했으며 Release 고지 처리는 별도 gate다. 이 수정은 API 29·34 및 API 36 회귀 경계를 닫지만 R05.3 또는 R05 전체를 완료하지 않는다.
