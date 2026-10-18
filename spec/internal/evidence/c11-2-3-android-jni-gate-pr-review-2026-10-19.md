# C11.2.3 Android JNI gate PR 변경 적대 검토 30개

| # | 변경 경계 | 공격적 실패 관점 | 검토 결과 |
|---:|---|---|---|
| 1 | 기준 tree | 수정 전 `origin/main` 재현이 다른 source revision을 사용했는가 | `0168e89e2ebff3543e4d2153bf21e9557bb7544a`의 JNI 파일을 사용해 두 미선언 C symbol 오류를 재현했다. |
| 2 | create C++ body | feature-off인데 create가 C11.2.3 FFI declaration에 닿는가 | 호출 전체가 feature gate 안에 있다. |
| 3 | eval C++ body | feature-off인데 eval이 C11.2.3 FFI declaration에 닿는가 | 호출 전체가 같은 feature gate 안에 있다. |
| 4 | macro 조건 | macro 미정의 상태를 on으로 취급하는가 | `defined(...) && value`를 검사한다. |
| 5 | macro 조건 | macro가 명시적으로 0이면 FFI 호출이 남는가 | off APK symbol table에서 두 FFI symbol이 없다. |
| 6 | JNI exports | off branch에서 native method export가 사라지는가 | off APK에 create/eval JNI export 둘 다 남아 있다. |
| 7 | JNI exports | native method 이름·시그니처가 바뀌어 Java linkage가 깨지는가 | 선언·이름·인자형은 변경하지 않았다. |
| 8 | create disabled | 0이 아닌 stale pointer를 반환하는가 | off branch는 0을 반환한다. |
| 9 | create caller | Java가 0 handle을 사용해 이어서 native eval을 부르는가 | Java의 host 생성 실패 분기가 후속 eval 전에 반환한다. |
| 10 | eval disabled | off 호출이 성공 상태로 위장되는가 | `status=-90 feature-disabled`를 반환하며 Node 정적 검사가 확인한다. |
| 11 | eval arguments | disabled eval이 임의 `jstring`을 UTF 변환하는가 | off branch는 인자를 읽지 않는다. |
| 12 | eval response | disabled branch에서 유효 `JNIEnv`를 사용하지 않아 byte array 생성이 실패하는가 | JNI entry의 유효 `JNIEnv`로 기존 `ToByteArray` helper를 호출한다. |
| 13 | enabled strings | on branch에서 UTF buffer release를 빼먹거나 순서를 바꾸는가 | 기존 on branch를 그대로 유지했고 positive APK build를 확인했다. |
| 14 | C header | C declaration macro와 C++ callsite macro가 어긋나는가 | 같은 이름·조건을 사용하고 header 선언 경계를 확인했다. |
| 15 | Cargo feature | off Cargo artifact에 fixture FFI implementation이 실리는가 | off APK ELF에는 fixture FFI symbol이 없고 on APK에는 두 개가 있다. |
| 16 | native build flag | shell build script가 macro에 잘못된 default를 전달하는가 | 기존 `:-0` 전달을 유지하고 off/on native build를 확인했다. |
| 17 | Gradle value | APK가 native build와 다른 feature 값으로 포장되는가 | Gradle off/on을 각각 실행하고 패키지 내부 `.so`를 직접 검사했다. |
| 18 | C12.4 조합 | worker-stack diagnostic이 C11.2.3을 몰래 opt-in하는가 | worker diagnostic on/C11 off APK는 통과하고 C11 FFI symbol은 없다. |
| 19 | C04 dependency | C04 GPU on만으로 C11.2.3 fixture가 활성화되는가 | C04 on/C11 off artifact는 FFI symbol을 포함하지 않는다. |
| 20 | 불가능한 조합 | C04 GPU off/C11 fixture on이 허용되는가 | build script가 exit 2로 거부한다. |
| 21 | scope | Rust layout, public CSS behavior, iOS source가 함께 바뀌는가 | 수정 파일은 Android JNI, package script, 계획·증거·상태 문서뿐이다. |
| 22 | generated `.so` off | JNI symbol을 제외하다 필요한 C11.2.2 또는 다른 C ABI까지 잃는가 | 두 JNI C11.2.3 method만 gate됐고 off native build 성공 로그를 남겼다. |
| 23 | generated `.so` on | positive control에서 fixture C entry symbol이 누락되는가 | ARM64 opt-in library에서 eval/new 두 symbol을 확인했다. |
| 24 | packaged APK off | Gradle strip 이후 gate 차이가 달라지거나 JNI export가 숨는가 | stripped off APK에서도 JNI 2개 유지·fixture FFI 2개 제외를 확인했다. |
| 25 | packaged APK on | opt-in APK에서 Gradle packaging이 fixture FFI를 제거하는가 | stripped on APK에서 두 fixture FFI symbol을 확인했다. |
| 26 | artifact freshness | native `.so` 갱신 뒤 이전 APK를 off 결과로 오인하는가 | stale opt-in APK를 발견해 off APK를 Gradle로 재포장하고 재설치했다. |
| 27 | AVD disabled route | feature-off fixture Intent가 앱을 native crash시켜버리는가 | API 37 AVD 로그에 disabled marker가 있고 화면은 host-create 실패를 표시했다. |
| 28 | AVD default route | feature-off APK가 기본 root HTML boot/draw를 망가뜨리는가 | root HTML boot pass와 renderer draw 로그·화면을 확인했다. |
| 29 | platform claim | ARM64 AVD 결과를 물리 Android 또는 iOS 결과로 확대하는가 | 문서에는 API 37 AVD 한정으로 적고 물리 기기/iOS 검증은 주장하지 않는다. |
| 30 | 완료·지원 상태 | build fix를 Grid 구현 완료나 CSS 지원으로 과장하는가 | 기존 C11.2.3 완료 체크만 유지하며 제한 profile·공개 지원 경계 변경은 없다. |

## 판정

30개 관점은 source compile, JNI contract, feature propagation, packaged artifact, AVD route, 범위·완료 경계로 나눠 확인했다. stale APK 검증 오류는 정확한 off APK 재포장과 재실행으로 해결했다. AVD 기본 화면은 no-regression smoke이며 C11.2.3 시각 적합성 근거가 아니다.
