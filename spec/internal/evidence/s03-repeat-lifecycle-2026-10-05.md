# S03.3 · Android·iOS 반복 수명 회수 검증

## 고정 환경

- 실행 날짜: 2026-10-05
- Spinon 기준 commit: `194b406f707fe5687005cd58547aac2e85b7aada`
- V8 revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- macOS 26.5.1 (25F80), Xcode 26.2, iOS Simulator SDK 26.2
- Android 16 / API 36, `sdk_gphone64_arm64` ARM64 에뮬레이터 (`emulator-5554`)
- iPhone 17 Pro / iOS 26.2 시뮬레이터 (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)
- Rust `1.96.1`, Bun `1.4.2`
- `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 검증 빌드. 시험 전용 강제 GC 요청 함수는 이 빌드에서만 노출한다.
- iOS 검증은 `SPINON_V8_DIR`의 외부 V8 checkout을 `SPINON_V8_ROOT` 링크 설정에도 전달해 빌드했다.

## 실행

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_S03_DOM_GC_OUTPUT_DIR=/tmp/spinon-repeat-lifecycle/build/spinon/dom-lifecycle-validation/review-run-20261005-retry \
mise exec -- bash tools/verify-s03-dom-lifecycle-simulators.sh
```

검증 스크립트가 두 앱을 fixture 활성 상태로 빌드·설치하고 각 앱의 자동 시나리오를 실행했다. 성공 marker, callback root 결과, 모든 scan 계수와 6개 반복 회차를 확인한 뒤 로그와 화면을 저장했다.

## 결과

| 항목 | Android | iOS |
| --- | --- | --- |
| V8·Rust 회수 callback 오류, poisoned, deferred | 없음 | 없음 |
| `scanned = live + empty` | 모든 검사에서 일치 | 모든 검사에서 일치 |
| 초기 고정 fixture 전후 노드 / UTF-16 코드 단위 / weak wrapper | `10 / 226 / 9`로 복귀 | `10 / 226 / 9`로 복귀 |
| native strong callback closure의 분리 child wrapper 보존·호출 | 통과 | 통과 |
| callback 교체와 GC 뒤 closure 대상 회수·기준선 복귀 | 통과 | 통과 |
| 반복 회차 | `6/6`, 회차당 element·text 쌍 32개 | `6/6`, 회차당 element·text 쌍 32개 |
| 각 회차에서 회수한 wrapper / 최종 기준선 | 최대 64 / `10 / 226 / 9` | 최대 64 / `10 / 226 / 9` |

각 반복 회차는 요소와 텍스트 노드 32쌍을 만들고 부모에 붙였다가 분리한 뒤 지역 참조를 놓는다. V8 GC와 owner safe-point 회수 이후 Rust 노드 수, 저장 문자열의 UTF-16 코드 단위 수, weak wrapper registry 생존 handle 수가 시작 기준선과 같은지 확인한다. multilingual·emoji 문자열도 fixture 입력에 포함한다.

실행 로그와 화면:

- Android [원본 로그](s03-repeat-lifecycle-android-2026-10-05.log) · [검증 화면](s03-repeat-lifecycle-android-2026-10-05.png)
- iOS [원본 로그](s03-repeat-lifecycle-ios-2026-10-05.log) · [검증 화면](s03-repeat-lifecycle-ios-2026-10-05.png)

## 판정 범위와 남은 한계

이 결과는 고정 시뮬레이터 fixture에서 Rust가 계수하는 live document node와 UTF-16 저장 단위, V8 weak-wrapper registry handle이 반복 후 기준선으로 복귀함을 보여준다. 프로세스 RSS, 실제 할당 byte 수, V8 heap snapshot의 retaining path 또는 최대 registry 크기에서의 시간·메모리 비용을 측정하지 않았다. `spinon.onEvent()`가 보유하는 현재 진단용 strong callback의 closure root만 확인했으며, DOM `EventTarget` listener와 external root lease의 소유·순환 수명은 포함하지 않는다. 세션 종료 경합, 실기기, 제품 DOM 완료 여부도 판정하지 않는다. 따라서 S03.3/J10은 미완료다.

첫 iOS 빌드에서는 shell 단계가 외부 `SPINON_V8_DIR`를 사용했지만 Xcode linker가 기본 저장소 경로를 고정해 둔 불일치를 발견했다. 빌드 설정을 `SPINON_V8_ROOT`로 통일하고 같은 고정 V8 archive를 사용한 재빌드와 시뮬레이터 실행을 통과했다.
