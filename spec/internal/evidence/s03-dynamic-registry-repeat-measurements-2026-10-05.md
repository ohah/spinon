# S03.3 · 16,385 wrapper registry 반복 scan 관측

## 실행 조건

- 실행 날짜: 2026-10-05
- Spinon 기준 commit: `c2fe051c6d1985aabdea5055b8d3720b03d53f5d`와 이 기록을 추가하는 검증 fixture 변경
- V8 revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- Android: Android 16 / API 36 ARM64 에뮬레이터 (`emulator-5554`, `arm64-v8a`)
- iOS: iPhone 17 Pro / iOS 26.2 시뮬레이터 (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)
- Android는 앞서 재현된 `autib1716` SIGILL을 피하려고 이 에뮬레이터 검증용 V8 생성 인자에서만 CFI를 껐다. 추적 중인 `tools/v8/android-v8.args.gn`은 바꾸지 않았다. 이 실행은 CFI 활성 배포 빌드나 실기기 결과가 아니다.
- Android·iOS 모두 `v8_jitless = false`다. 강제 GC hook은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 검증 빌드에만 포함된다.

## 측정 방법

각 실행은 기준선 문서에 container 1개와 text node 16,384개를 추가해 새 wrapper 16,385개를 만든다. 기존 기준선 wrapper 9개까지 포함하면 live registry는 16,394개다. 전체 wrapper를 JavaScript strong array로 보존한 상태에서 Rust reachability callback까지 수행한 뒤, 같은 Isolate에서 빈 JavaScript 평가 10회를 순차 실행했다. 각 빈 평가도 V8 owner safe point에서 같은 registry scan을 호출한다. 첫 setup scan에서 root vector가 커진 다음이므로 이 10개 표본은 root vector 할당 이후의 재scan 비용을 관찰한다. 이어서 큰 tree를 해제하고 기존 lifecycle 검증을 계속 실행했다.

별도 실행 방식의 시작·종료 편차를 보기 위해 테스트 앱을 각 플랫폼에서 7번 force-stop 후 다시 시작하고, 매 실행의 setup scan 하나를 기록했다. 원본 값은 [fresh-process CSV](s03-dynamic-registry-fresh-process-2026-10-05.csv)에 있다. 같은 프로세스의 10회 표본 원본은 [Android 로그](s03-dynamic-registry-repeat-android-2026-10-05.log)와 [iOS 로그](s03-dynamic-registry-repeat-ios-2026-10-05.log)에 있다.

계측은 `CollectDocumentAtSafePoint` 진입부터 끝까지 `std::chrono::steady_clock`으로 잰 경과 시간이다. 강제 `LowMemoryNotification()`은 계측 시작 전에 수행한다. 반복 표본은 CPU time이 아닌 wall time이며 스케줄러에 의해 실행이 밀린 시간도 포함한다. V8 GC 시간, JavaScript tree 생성 시간, 앱 프레임 시간은 이 scan 수치에 포함하지 않는다.

## 결과

| 측정 방식 | Android | iOS |
| --- | ---: | ---: |
| 프로세스 재시작 7회: 중앙값 (최소–최대) | `63.153 ms` (`8.043–133.193 ms`) | `13.473 ms` (`8.085–19.068 ms`) |
| 같은 Isolate 재scan 10회: 중앙값 (최소–최대) | `42.125 ms` (`7.498–174.678 ms`) | `17.707 ms` (`8.732–25.980 ms`) |
| 최신 실행의 최초 setup scan | `132.520 ms` | `9.528 ms` |
| live registry 수 | `16,394` | `16,394` |
| 재scan 표본 수와 자원 검사 | `10/10 통과` | `10/10 통과` |
| 전체 회수 후 Rust 기준선 | `노드 10 / 문자열 단위 226 / wrapper 9` | `노드 10 / 문자열 단위 226 / wrapper 9` |

초기 한 번 측정한 Android `15.273 ms`와 iOS `7.067 ms`는 대표값이 아니었다. 새 표본은 플랫폼 안에서도 변동 폭이 크다. 특히 Android wall-time 분포와 iOS 중앙값은 이 16k registry scan이 무시할 수 있는 비용이라고 보기 어렵다는 경고 신호다. 빈 평가도 이 전체 scan을 실행하므로 JavaScript owner 실행기가 이 동안 다른 작업을 처리하지 못한다.

이 데이터는 성능 목표를 만족한다는 증거가 아니며, Android와 iOS의 성능 순위를 정하는 근거도 아니다. 에뮬레이터·시뮬레이터의 CPU 사용률과 스케줄링 편차를 따로 계측하지 않았고 CPU time도 수집하지 않았다. 따라서 큰 표본값 중 얼마가 scan 계산이고 얼마가 실행 지연인지는 분리할 수 없다. 한 Isolate의 반복 측정 또한 실기기 결과를 대신하지 않는다.

## 다음 판정과 한계

현재 전체 registry를 매 JavaScript 평가·dispatch의 owner safe point마다 훑는 비용은 더 조사해야 한다. 다음 성능 변경 전에는 반복 가능한 release 성능 fixture와 CPU/wall 시간 분리, 호출 간 scan 동작을 고정하고, 정확한 생존 wrapper·HostDocument 결과를 유지하는 비교 모델을 마련한다. 비동기 수거·증분 수거 또는 호출 빈도 변경은 GC 시점과 identity 계약을 바꿀 수 있으므로 별도 검증 없이 적용하지 않는다.

이 검증은 Android ARM64 실기기, 다른 SoC·구형 ARM64, release 앱 전체의 p50/p95, 메모리 압박·장기 실행, `std::bad_alloc` 주입, DOM listener/external root 수명을 다루지 않았다. S03.3과 J10은 미완료다.
