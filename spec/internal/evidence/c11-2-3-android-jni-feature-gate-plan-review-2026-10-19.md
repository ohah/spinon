# C11.2.3 Android JNI gate 계획 검토 20개

| # | 실패 관점 | 계획에 둔 방어 경계 |
|---:|---|---|
| 1 | 기본 feature-off 빌드가 계속 C11.2.3 symbol을 호출하는가 | off 설정으로 API 37 Android build를 수행해 C++ compile을 확인한다. |
| 2 | Cargo feature와 C header guard가 다르게 켜지는가 | Gradle 환경 변수, Cargo feature, header macro, C++ macro를 같은 이름으로 대조한다. |
| 3 | worker-stack feature가 C11 fixture를 암묵적으로 켜는가 | C12.4 진단 on/C11.2.3 off 조합을 별도 build matrix에 둔다. |
| 4 | C04 runtime GPU가 켜지면 C11.2.3도 자동으로 켜지는가 | C11.2.3 환경 변수 기본값을 0으로 고정하고 off 빌드를 확인한다. |
| 5 | feature off에서 JNI method 자체가 없어 Java linkage가 깨지는가 | JNI export symbol은 남기고 method body만 조건부로 분기한다. |
| 6 | 비활성 create가 유효하지 않은 포인터를 반환하는가 | disabled branch는 `0`을 반환한다. |
| 7 | 비활성 eval이 성공으로 오인되는가 | `status=-90 feature-disabled` 명시 결과를 돌려준다. |
| 8 | disabled branch도 gated C declaration에 닿는가 | fixture symbol 참조를 `#if` 분기 내부에 한정한다. |
| 9 | C++와 헤더가 서로 다른 macro 표현을 쓰는가 | 동일 feature 이름과 truth 조건을 사용한다. |
| 10 | null `JNIEnv` 또는 문자열 처리가 disabled 경로에 유입되는가 | disabled eval은 인자를 읽거나 변환하지 않는다. |
| 11 | enabled 경로 UTF 문자열을 회수하지 않는가 | 기존 scene/run ID 획득·회수 순서를 보존한다. |
| 12 | 실패한 UTF 변환 뒤 부분 획득 문자열이 누수되는가 | 기존 early-return 정리 동작을 변경하지 않는다. |
| 13 | C11.2.3 정상 fixture 실행이 비활성화되는가 | opt-in positive-control build를 별도로 수행한다. |
| 14 | 잘못된 환경 변수 값이 묵시적으로 허용되는가 | 기존 shell validation의 0/1 제한을 유지한다. |
| 15 | C04 off/C11.2.3 on의 불가능 조합이 허용되는가 | 기존 build script의 의존 feature 거부 규칙을 유지한다. |
| 16 | iOS build macro에 Android 조건이 새는가 | 변경을 Android JNI translation unit으로 한정한다. |
| 17 | public FFI ABI가 바뀌는가 | header 선언과 함수 시그니처는 수정하지 않는다. |
| 18 | Android JNI 이름 mangling이 바뀌는가 | 기존 exported native function 이름을 유지한다. |
| 19 | 다른 fixture symbol이 우연히 제거되는가 | 두 C11.2.3 진입점만 분기하고 전체 Android build로 영향 범위를 확인한다. |
| 20 | 빌드 성공을 runtime/기능 적합성 완료로 과장하는가 | 결과는 Android build gate 수정으로 한정하고 C11.2.3 전체 지원 상태는 바꾸지 않는다. |

## 판정

계획 검토는 20개 관점을 모두 분리해 확인했다. 구현 후 off/on 빌드, negative combination, AVD 기본 실행으로 계획의 핵심 판정을 대조한다.
