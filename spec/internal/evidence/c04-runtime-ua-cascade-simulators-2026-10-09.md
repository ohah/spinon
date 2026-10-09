# C04.8 · Runtime UA cascade 시뮬레이터 실행 근거

## 실행 범위

고정 V8 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`를 연결한 Debug 앱을 Android emulator와 iOS Simulator에서 각각 빌드·실행했다. 두 앱 모두 실제 V8 DOM façade에서 `div`, `span`, `button`, `p` root를 만들고 CSS 환경을 전달한 뒤 `spinon.runtime.ua-cascade.v1` JSON을 읽었다.

첫 task는 `div`에 inline `display:inline` 및 잘못된 color 선언을 설정하고 나머지 세 root를 추가한 뒤 의도적으로 JavaScript 예외를 던진다. 예외 반환에도 document revision과 cascade가 게시되는지, 네 root의 UA computed value·inline override·CSS 진단이 맞는지 확인했다. 다음 task는 detached element 256개를 만들어 document revision이 증가하는지, 연결된 root 결과는 그대로인지 확인했다.

## 플랫폼 결과

| 플랫폼 | 실행 환경 | 실제 V8 probe | 주요 결과 |
|---|---|---|---|
| Android | API 37.1, ARM64 emulator, 16 KiB page, `sdk_gphone16k_arm64` | 통과 | 4 roots, revision 9→265, UA 값·inline style·진단·예외 뒤 commit 통과 |
| iOS | iPhone 17 Pro Simulator, iOS 26.2, arm64 | 통과 | 4 roots, revision 9→265, UA 값·inline style·진단·예외 뒤 commit 통과 |

실행 로그의 측정값은 다음과 같다. 시간은 µs 단위 단일 표본이다.

| 측정값 | Android | iOS Simulator |
|---|---:|---:|
| CSS worker 준비 | 79 | 613 |
| RuntimeSession 초기화 | 3,980 | 21,755 |
| snapshot 복제: 기본 문서 | 1 | 1 |
| snapshot 제출: 기본 문서 | 9 | 249 |
| cascade 계산: 첫 요청 | 12,903 | 5,531 |
| snapshot 복제: detached 256개 추가 | 9 | 8 |
| snapshot 제출: detached 256개 추가 | 14 | 5 |
| cascade 계산: 후속 요청 | 221 | 111 |

첫 cascade와 후속 cascade는 초기화·캐시 warm 상태가 다르므로 계산 시간끼리 비교하지 않는다. 값은 에뮬레이터/시뮬레이터의 기능 진단 표본이며 벤치마크, 제품 성능 보장, 실제 기기 또는 GPU 프레임 결과가 아니다. worker stack/RSS와 더 큰 DOM의 반복 분포도 측정하지 않았다.

## 재현 명령

```sh
SPINON_V8_DIR=/private/tmp/spinon-r05-present-api-audit/build/v8-source/v8 mise exec -- bun run build:android
adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c048_ua_cascade true
SPINON_V8_DIR=/private/tmp/spinon-r05-present-api-audit/build/v8-source/v8 mise exec -- bun run build:ios-sim
xcrun simctl install booted build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app
xcrun simctl launch booted dev.spinon.bootstrap --spinon-c048-ua-cascade
```

## 화면 증거

- Android: [실행 화면](c04-runtime-ua-cascade-android-2026-10-09.png)
- iOS Simulator: [실행 화면](c04-runtime-ua-cascade-ios-2026-10-09.png)

## 경계

이 실행은 내부 runtime snapshot을 검증한다. CSSOM, author stylesheet registry, OS 환경 자동 감지, Taffy layout, render snapshot, GPU 표시, 실제 기기, 공개 제품 API를 검증하지 않는다. iOS 화면을 처음 확인할 때 래퍼가 `status=0`을 중복 표시하는 결함을 발견해 결과 전달을 고쳤고, 최종 iOS 재빌드·실행 화면에는 상태가 한 번만 표시된다.
