# 구현 후 독립 검토 · API 29 unwind 연결

수정된 Android link script, Debug DSO/APK, API 29·34 AVD 실행, 사용자 요청 실기기 회귀와 원본 evidence를 별도 실패 관점으로 다시 확인했다. 이는 구현 후 검토이며 plan/r05-android-api29-unwind-link.md의 계획 검토를 재사용하지 않는다.

| # | 공격 관점 | 확인 결과와 경계 |
|---:|---|---|
| 1 | 다른 NDK host runtime을 선택 | 고정 SDK NDK 27.1.12297006의 현재 macOS prebuilt 경로를 확인했고 그 경로 아래에서만 archive를 찾는다. |
| 2 | archive glob이 0개일 때 literal path가 link로 전달 | 누락 archive mock은 link 전에 실패했고 이전 DSO hash가 변하지 않았다. |
| 3 | 여러 Clang resource version 중 임의 선택 | 복수 후보 mock은 fail-closed였고 이전 DSO가 보존됐다. |
| 4 | 잘못된 archive가 필요한 함수를 제공한다고 오인 | _Unwind_Resume 정의가 없는 llvm-nm mock은 명시 오류로 중단됐다. |
| 5 | archive ABI가 Android arm64와 다름 | archive path와 object metadata가 AArch64이며 API 29·34 ARM64 AVD에서 새 DSO가 실제 load됐다. |
| 6 | V8 Clang이 NDK runtime 자동 탐색을 할 것이라고 가정 | 직접 지정된 NDK archive를 link command에서 확인했다. V8 전체 toolchain을 바꾸지 않았다. |
| 7 | libc++abi보다 앞선 archive 순서로 참조가 남음 | libc++abi 다음 위치에 archive를 두고 실제 lld 링크가 성공했다. |
| 8 | 중복 runtime symbol 정의로 link 실패 또는 비결정 결과 | 본 빌드와 AVD runtime이 성공했고 duplicate-definition 오류가 없었다. |
| 9 | unwind 구현 심볼이 앱 DSO 공개 ABI로 노출 | APK DSO dynamic export에서 _Unwind_*가 0개다. |
| 10 | 일부 unresolved unwind import가 남음 | 최종 APK DSO dynamic undefined _Unwind_*가 0개이고 API 29 native load가 성공했다. |
| 11 | 동적 libunwind 의존성으로 API 29 loader가 다시 실패 | 최종 DSO에 libunwind DT_NEEDED가 없고 API 29 cold launch에서 linker 실패가 없다. |
| 12 | 16KB page-size link 설정 손상 | PT_LOAD alignment 0x4000, GNU_RELRO 끝 0x2dd4000, APK zipalign -P 16 검사가 유지됐다. |
| 13 | Gradle cache가 이전 DSO를 패키징 | Debug APK, API 29·34 설치본, API 36 기기 설치본 SHA-256이 모두 1639d939…572e로 일치한다. |
| 14 | APK strip 이후 DSO가 다른 결과 | APK 내 stripped DSO hash af5783e5…6305a를 unstripped linker output bbd40e9e…5df0와 별도로 기록했다. |
| 15 | API 29 loader 성공 뒤 bootstrap이 실패 | 새 API 29 process에서 SPINON_BOOTSTRAP_RESULT nodes=2가 기록됐다. |
| 16 | API 29 API35 전용 class/verifier 경계가 깨짐 | async fence는 api_below_35로 종료했고 present-fence/FrameTimeline은 독립 process에서 GLES draw와 화면 변경을 유지했다. API 34도 같은 조건으로 회귀 확인했다. |
| 17 | 실기기 확인을 API 29 증거로 과장 | 실제 Samsung 기기는 Android 16/API 36이다. API 29 통과 근거는 전용 API 29 ARM64 AVD에만 귀속했다. |
| 18 | ADB synthetic input을 손가락 입력으로 집계 | 13회 입력은 synthetic으로 분류했고 direct sec_touchscreen capture에서 positive sample을 주장하지 않았다. |
| 19 | fence signal을 VSync나 광학적 표시 시각으로 해석 | target_vsync_id=-1이다. usability와 queue drain만 확인했고 latency·VSync·scanout을 산출하지 않았다. |
| 20 | 검증용 NDK archive를 Release 배포 고지까지 완료한 것으로 표시 | NOTICE 존재만 확인했다. Release packaging, API 30–33, direct touch, iOS와 R05.3 완료는 남은 경계로 유지했다. |

### 검토 판정

20개 독립 관점에서 수정 회귀는 찾지 못했다. 실행 로그 요약에서 submit field의 중간 순서를 고정한 최초 정규식이 0건을 반환했으나, 원본 로그를 확인해 parser만 보정한 뒤 input/submit/callback/wait sequence 13개가 전부 일치함을 다시 계산했다. 원본에 submit 누락은 없었다. 검증 주장 범위는 API 29·34 AVD 경계와 API 36 실기기 synthetic regression으로 제한한다.
