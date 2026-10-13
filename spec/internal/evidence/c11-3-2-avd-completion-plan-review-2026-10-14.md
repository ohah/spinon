# C11.3.2 AVD 완료 판정 예외 계획 검토 · 2026-10-14

## 검토 범위

사용자가 C11.3.2의 완료 판단에 Android AVD를 허용한 지시에 맞춰, 해당 단계의 Android gate만 한정적으로 조정했다. 일반 C11.3 검증 관문, 다른 단계의 물리 기기 요구, C11 상위 완료 범위는 바꾸지 않았다. 이 표는 계획 변경의 실패 관점을 검토하며 새 제품 코드 검증을 대신하지 않는다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검사와 결과 |
| --- | --- | --- |
| 1 | C11.3.2의 예외가 일반 Android 관문을 바꿈 | 계획의 공통 관문은 SM-S731N을 기본값으로 유지하고 예외를 C11.3.2 문단과 4번 관문에만 적었다. 통과. |
| 2 | 사용자 승인을 모든 이후 단계에 상속 | 관문은 다른 C11 단계에 자동 상속되지 않는다고 명시한다. C11.3.3은 미착수로 남는다. 통과. |
| 3 | AVD 결과를 물리 기기 결과로 이름만 바꿔 기록 | 최신 `device.json`은 `qemu=true`, `sdk_gphone64_arm64`를 기록한다. 문서는 이를 AVD/SwiftShader 결과로 부른다. 통과. |
| 4 | API 버전이 승인된 표본과 다름 | 최신 run metadata의 `sdk`는 37, Android release는 17이다. 계획은 API 37.0 AVD를 특정한다. 통과. |
| 5 | CPU ABI가 ARM64 표본과 다름 | AVD 식별자와 run metadata가 ARM64 emulator를 가리킨다. 실제 결과는 고정 API 37 device guard를 통과했다. 통과. |
| 6 | 하드웨어 renderer 결과를 SwiftShader로 오인 | metadata renderer는 ANGLE/Vulkan/SwiftShader다. hardware GPU 성능이나 동작이라는 표현을 하지 않는다. 통과. |
| 7 | 캡처가 현재 PR 소스가 아닌 오래된 APK를 재사용 | 새 run은 PR checkout HEAD `7f79186`에서 실행했다. [`source-revision.json`](./c11-3-2-grid-named-lines-runtime-2026-10-14/android-avd-c1132-android-avd-20261013T231257017Z-496b3f85/source-revision.json)은 tracked tree가 clean했고 implementation commit `0814932` 이후 implementation source가 바뀌지 않았음을 기록한다. 새 run ID와 설치·실행 로그가 있으며 앱 SHA도 이전 성공 run과 같다. 통과. |
| 8 | fixture source가 Chrome oracle와 바뀜 | 최신 `result.json` fixture SHA `460c02eb…311ea39`와 Chrome reference ID가 통합 기준 자료와 일치한다. 통과. |
| 9 | 성공 경로가 실행되지 않고 거부 경로만 통과 | 새 run 결과는 `positive-occurrence` frame과 성공 surface comparison을 기록한다. 통과. |
| 10 | 지원 밖 오류 장면이 성공으로 처리 | 같은 run은 별도 expected-rejection scene을 포함하며, 이를 positive scene과 분리한다. 통과. |
| 11 | geometry가 화면 픽셀과 혼합되거나 좌표가 달라짐 | root `160×120`, item `80×40 @ (80,0)` 결과를 이전 Chrome/앱 기준과 대조했다. 통과. |
| 12 | 표본 일부만 맞았는데 전체 exact로 확대 | `expectedSamples=2`, `matchedSamples=2`, 최대 channel error `0`을 확인했다. 2개 지정 표본으로 범위를 제한한다. 통과. |
| 13 | 변조 좌표가 PixelCopy 뒤에서야 거부 | negative control은 `rejected-before-readback`, `readbackStarted=false`다. 통과. |
| 14 | 오래된 실행 로그를 최신 marker로 채택 | result, runtime log와 process ID에 새 run ID `c1132-android-avd-20261013T231257017Z-496b3f85`가 함께 기록됐다. 통과. |
| 15 | 화면 PNG를 표면 PixelCopy와 혼동 | 계획은 표면 비교를 PixelCopy 결과로 설명하고 화면 캡처는 실행 화면 자료로 분리한다. 통과. |
| 16 | 이전 SM-S731N 표본을 최신 소스 gate로 계산 | 이전 physical run은 iterator 정리 전임을 남기고 최신-source 실기기 검증 미실행이라고 기록한다. AVD 승인으로 이 단계만 완료한다. 통과. |
| 17 | Android 판정이 iOS 한계를 흐림 | iOS는 iPhone 17 Pro Simulator의 Metal drawable 결과로 별도 기록하고 compositor 제출 전 한계를 유지한다. 통과. |
| 18 | Chrome/WPT oracle 결과를 Spinon 적합성으로 승격 | 계획은 Chrome의 2 PASS와 17 subtest OK 결과를 oracle로 한정하고 Spinon WPT 미완료를 유지한다. 통과. |
| 19 | 두 장면 결과를 전체 모바일 행렬이나 공개 CSS 지원으로 확대 | C11.3.2 범위는 1 success·1 expected rejection scene과 2 pixel sample이며 전체 matrix·공개 지원은 미완료라고 적었다. 통과. |
| 20 | 완료 표시가 C11.3 상위나 후속 단계 상태를 앞당김 | C11.3.2만 닫고 C11.3.3 미착수, C11 상위·전체 Grid 미완료, 후속 착수는 병합·회귀 이후로 유지한다. 통과. |

## 근거

- [최신 AVD 실행 결과](./c11-3-2-grid-named-lines-runtime-2026-10-14/android-avd-c1132-android-avd-20261013T231257017Z-496b3f85/README.md)
- [AVD result JSON](./c11-3-2-grid-named-lines-runtime-2026-10-14/android-avd-c1132-android-avd-20261013T231257017Z-496b3f85/result.json)
- [계획과 기존 구현 검토](../../../plan/c11-3-template-repeat-named-lines-areas.md) · [C11.3.2 구현 실패 경계 검토](./c11-3-2-grid-named-lines-implementation-review-2026-10-14.md)

검토는 이번 C11.3.2 완료 판정 예외의 범위와 증거 표현만 지지한다. API 37 AVD는 실기기나 hardware GPU 관찰이 아니다.
