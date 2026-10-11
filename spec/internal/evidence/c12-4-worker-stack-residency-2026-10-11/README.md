# C12.4 CSS worker stack residency · Simulator/AVD 결과

**판정:** Android API 37 AVD와 iOS 26.2 Simulator에서 CSS worker별 pthread 범위와 순간 mincore page 표본을 수집했다. 이 결과는 stack-only commit/RSS, high-water mark, 실기기 메모리, WPT 적합성 또는 C12.4 전체 완료를 뜻하지 않는다.

## 실행

| 대상 | 실행 ID | 동시 worker 단계 | 공개 가능한 실행 요약 |
| --- | --- | --- | --- |
| Android API 37 AVD (`sdk_gphone16k_arm64`) | `c124-android-20261011T124627532Z-f8f10c7b` | 0·1·2·4·8 | [주소를 제외한 실행 요약](c124-android-20261011T124627532Z-f8f10c7b/README.md) · [동시 session process memory 요약](../c12-4-concurrent-session-stack-cost-2026-10-11/c124-android-20261011T124627532Z-f8f10c7b/README.md) |
| iPhone 17 Pro / iOS 26.2 Simulator | `c124-ios-20261011T124751045Z-3b4c9256` | 0·1·2·4·8 | [주소를 제외한 실행 요약](c124-ios-20261011T124751045Z-3b4c9256/README.md) · [동시 session process memory 요약](../c12-4-concurrent-session-stack-cost-2026-10-11/c124-ios-20261011T124751045Z-3b4c9256/README.md) |

각 live stage에서 session 수와 CSS worker 수가 일치했고 worker마다 3회 연속 표본을 얻었다. 두 대상 모두 16 KiB page 기준 각 worker의 513개 page 표본 중 1개 page가 resident로 관측됐다.

## 관측값 해석

- CSS worker stack 설정 요청은 session당 8 MiB다. pthread API가 보고한 Android 범위는 guard 16,384 bytes를 제외한 뒤 8,396,736 bytes였고, iOS 범위는 8,400,896 bytes였다. 요청과 OS 범위를 별도 값으로 보존했다.
- page 표본은 시작 partial page를 제외하고 끝 경계 page를 포함한다. 끝 page에는 pthread 범위 바깥 8,256 bytes(Android), 4,096 bytes(iOS)가 포함된다. resident page가 stack 전용이라고 단정하지 않는다.
- Android는 `run-as`로 실제 worker TID별 `maps`·`smaps`를 읽었다. `stack_and_tls:<tid>` VMA가 mincore sample span을 덮는 것을 확인했고 8개 worker의 VMA RSS/PSS는 각각 16 KiB였다. VMA에는 TLS도 들어가므로 이 숫자는 CSS stack 단독 RSS/PSS가 아니다. 해제 뒤 기록한 8개 worker VMA는 남아 있지 않았다.
- iOS mincore 표본은 얻었지만 `footprint`는 process 전체 `stack` category(8-session sample: 70 regions, 976 KiB dirty)만 제공한다. worker별 RSS/PSS나 stack-only commit으로 분리하지 않는다.
- mincore는 표본 시점의 page residency다. worker의 총 commit, stack high-water mark, 최대 안전 깊이, 누수 또는 physical GPU 성능을 보이지 않는다.

## 원시 주소와 공개 요약의 구분

pthread stack 주소와 Android `/proc` mapping 주소는 공개 저장소와 Funnel 미리보기에서 제외한다. Tailscale Funnel은 인터넷 공개이므로 원본 계측 파일은 로컬 검증 증거에만 보존하고, 이 문서에는 stack 크기·페이지 수·성공/거부 결과만 기록한다. 이 요약으로 원시 주소를 역산하거나 stack-only commitment를 주장하지 않는다.

## 계측 오류를 수정한 점

초기 AVD 수집은 세 구현 가정을 실제 환경과 대조하지 않아 실패했다. Android page boundary가 pthread bounds에 정렬될 것이라 가정했고, Bionic guard를 제외하지 않았으며, end partial page 때문에 mincore 전달 길이를 page-size 배수로 만들지 못했다. 각 실패 실행은 evidence 디렉터리에 보존하고, bounds/page 계산과 TID VMA 검증을 고쳤다. 최신 실행은 Android와 iOS 모두 정상 종료했다.

수집기 검증은 [worker stack validation 테스트](../../../../tools/css-reference/c12-4-worker-stack-validation.test.mjs)에서 누락·중복 표본, cross-session TID 중복, guard/range/page 산술 및 resident page 초과를 거부한다. 구현 검토의 [20개 실패 관점](../c12-4-worker-stack-residency-implementation-review-2026-10-11.md)도 함께 확인한다.

## 다음과 구분하는 항목

- 공식 WPT를 Spinon에서 실행하는 browser/testharness 연결은 별도 구현이 필요하다. Chrome 154에서 실행한 후보 결과는 Spinon 적합성 판정이 아니다.
- Android 실기기는 이번 사용자 지시에 따라 사용하지 않았다. AVD의 ANGLE/SwiftShader는 hardware GPU 근거가 아니다.
- `Vec::try_reserve` 실패 주입은 명시적 painter reservation 경로만 검증한다. 전역 allocator OOM recovery를 증명하지 않는다.
- 이 worker stack probe 결과만으로 C12.4나 전체 CSS 지원을 완료 처리하지 않는다.
