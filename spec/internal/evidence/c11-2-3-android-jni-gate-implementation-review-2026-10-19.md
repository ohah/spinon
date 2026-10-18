# C11.2.3 Android JNI gate 구현 실패 관점 검토 20개

| # | 실패 관점 | 확인한 방어·결과 |
|---:|---|---|
| 1 | 기본 feature-off에서 JNI create가 숨겨진 Rust symbol을 참조하는가 | origin/main 빌드로 undeclared identifier 오류를 재현했고 create 호출을 gate 안으로 옮겼다. |
| 2 | 기본 feature-off에서 JNI eval이 숨겨진 Rust symbol을 참조하는가 | origin/main 빌드에서 eval의 undeclared identifier 오류도 재현했고 eval 호출을 gate 안으로 옮겼다. |
| 3 | feature macro가 정의되지 않은 빌드에서 우연히 on으로 간주되는가 | `defined(...) && ...` 조건을 두 entry에 적용했다. |
| 4 | macro가 0으로 정의될 때 off branch가 선택되는가 | Android 기본 build flag는 0이며 off ARM64 APK build가 통과했다. |
| 5 | off build에서 Java의 JNI linkage가 사라지는가 | stripped APK ELF에도 두 JNI method export가 남는다. |
| 6 | feature-off create가 null 아닌 잘못된 handle을 내는가 | disabled branch에서 0을 반환하고 AVD 요청 로그에서 disabled marker를 확인했다. |
| 7 | Java가 handle 0으로 fixture eval을 진행해 잘못된 pointer를 역참조하는가 | 현재 Java route는 handle 0에서 즉시 중단한다. AVD 화면은 일반적인 host-create 실패 상태이며 native crash가 없었다. |
| 8 | 직접 eval이 off branch에서 성공으로 위장되는가 | status `-90 feature-disabled`를 반환하고 정적 Node 검사에서 확인했다. |
| 9 | disabled eval이 scene/run `jstring`을 변환하는가 | disabled branch는 세 인자를 무시하고 `GetStringUTFChars`를 호출하지 않는다. |
| 10 | enabled eval에서 기존 UTF 문자열 해제가 손상되는가 | 기존 on branch는 이동하지 않았고 positive-control native/APK build와 FFI symbol presence가 통과했다. |
| 11 | native method 이름 또는 JNI ABI 시그니처가 바뀌는가 | 선언과 method 이름을 수정하지 않았고 off/on APK 모두 두 JNI export를 포함한다. |
| 12 | 헤더와 C++가 서로 다른 feature gate를 쓰는가 | 같은 macro 이름을 사용하고 header/build/Gradle 경계를 정적 검사했다. |
| 13 | Gradle이 feature 값을 받아도 native compiler에 다른 값을 전달하는가 | off/on을 각각 다시 포장한 APK의 ELF symbol 차이를 확인했다. |
| 14 | C12.4 worker-stack diagnostic이 C11.2.3을 암묵적으로 켜는가 | worker diagnostic on/C11.2.3 off build와 APK가 통과했고 fixture FFI symbol은 없다. |
| 15 | C04 runtime GPU가 C11.2.3 fixture를 암묵적으로 켜는가 | C04 on/C11.2.3 off APK에서 두 fixture FFI symbol이 없음을 확인했다. |
| 16 | C04 off/C11.2.3 on의 불가능 조합이 허용되는가 | 기존 build-script guard가 exit 2로 거부하는 로그를 보존했다. |
| 17 | 잘못된 feature flag 값이 permissive하게 처리되는가 | 기존 build-script 0/1 검증을 보존했고 관련 파일은 수정하지 않았다. |
| 18 | Gradle stale artifact를 새 native build 결과로 오인하는가 | 초기 실행에서 stale opt-in APK를 발견했다. off APK를 재포장하고 APK 내부 ELF·SHA-256을 확인한 뒤 AVD에 재설치했다. |
| 19 | 기본 앱이 off APK에서 회귀하는가 | API 37 ARM64 AVD에서 기본 root HTML route boot/draw 로그와 화면을 확인했다. 이는 no-regression smoke만 의미한다. |
| 20 | build gate 수정이 Grid 구현 완료나 제품 CSS 지원으로 과장되는가 | C11.2.3 완료 체크를 그대로 두고 Android build integration 후속 범위로만 기록했다. |

## 결과

20개 관점을 서로 다른 symbol, ABI, feature propagation, app route, artifact freshness와 완료 범위로 나눠 확인했다. stale APK는 검토 중 발견해 정확한 off APK 재포장·재설치·재검증으로 해결했다. 직접 JNI eval 호출은 ABI harness가 없어 런타임에서 호출하지 않았고, 해당 disabled 반환값은 static guard 검사로만 검증했다.
