# S04.10 iOS Metal drawable API SDK·Simulator 확인

실행일: 2026-10-18<br>
Xcode: 26.2 (17C52)<br>
iPhoneOS SDK: `iPhoneOS26.2.sdk`<br>
iPhoneSimulator SDK: `iPhoneSimulator26.2.sdk`<br>
iOS Simulator runtime: 26.2 (23C54)<br>
대상: iPhone 17 Pro Simulator, `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`

## 판정

Apple API 이름을 잘못 사용한 것이 아니라 SDK 차이다. iPhoneOS 26.2 SDK의 `MTLDrawable.h`는 `addPresentedHandler`, `presentedTime`, `drawableID`를 선언하고, 같은 Swift probe가 `arm64-apple-ios26.2` target에서 통과했다. 반면 iPhoneSimulator 26.2 SDK의 `MTLDrawable.h`는 이 프로토콜 멤버를 선언하지 않으며, 동일한 source가 `arm64-apple-ios26.2-simulator` target에서 세 멤버를 찾지 못해 실패했다.

기기 SDK header는 callback과 세 property의 사용 가능 버전을 iOS 10.3으로 표시한다. Apple 문서도 `addPresentedHandler` callback과 `presentedTime`의 표시 시각 의미를 설명한다. 이 확인은 iOS 기기에서 callback이 실제 호출되는지, 특정 wgpu frame과 연결되는지 검증하지 않았다.

실제 Simulator runtime probe는 `CAMetalLayer.nextDrawable()`로 `CAMetalDrawable`을 받아 Objective-C selector 응답을 확인했다. 현재 runtime은 `addPresentedHandler:`, `presentedTime`, `drawableID` 모두에 응답하지 않았다. probe는 drawable을 화면에 제출하지 않았고 표시 시각도 측정하지 않았다. 그러므로 iOS 26.2 Simulator에서 이 callback의 positive-path 동작을 확인할 수 없다. Simulator에서 API가 없다는 사실을 전체 iOS 기기 SDK의 부재로 확대하면 안 된다.

이 결과로 S04.10/S05.2를 완료 처리하지 않는다. 향후 iOS 구현은 기기 SDK용 경로와 Simulator에서의 신호 미지원 경로를 분리해 표현해야 한다. Simulator build·fallback만으로 기기의 callback 동작을 통과 처리할 수 없다. 기기 SDK type-check 역시 실제 기기 runtime 근거가 아니다.

## 재현 입력과 관찰값

### iPhoneOS / iPhoneSimulator SDK type-check

고정 source는 `metal-api-probe.swift`다. 같은 source를 각 SDK에 대해 실행했다.

```sh
xcrun swiftc \
  -sdk "$(xcrun --sdk iphoneos --show-sdk-path)" \
  -target arm64-apple-ios26.2 \
  -typecheck spec/internal/evidence/s04-10-ios-metal-sdk-simulator-2026-10-18/metal-api-probe.swift
```

결과: 종료 코드 0.

```sh
xcrun swiftc \
  -sdk "$(xcrun --sdk iphonesimulator --show-sdk-path)" \
  -target arm64-apple-ios26.2-simulator \
  -typecheck spec/internal/evidence/s04-10-ios-metal-sdk-simulator-2026-10-18/metal-api-probe.swift
```

결과: 종료 코드 1. `drawableID`, `addPresentedHandler`, `presentedTime` 모두 `any CAMetalDrawable`의 member가 아니라는 진단을 받았다. SDK header SHA-256은 iPhoneOS `6ded45a7ca00e7ff0e8cfbe9799aedc0d84bcb65c35c679da2eadf0c5e777eaa`, iPhoneSimulator `4597a380c5ef65dc9f0f64a1b8582be5ea17b01d8114fab0ec343ccb167dec76`이다.

### iPhone 17 Pro Simulator Objective-C runtime selector 확인

`simulator-selector-probe.m`는 화면에 `CAMetalLayer`를 만들고 다음 selector에 `respondsToSelector:`를 호출한다.

- `addPresentedHandler:`
- `presentedTime`
- `drawableID`

임시 app bundle은 `Info.plist`를 사용했다. 앱 빌드·설치·실행 후 로그는 `simulator-runtime.log`에 보존했다.

관찰: drawable class `CAMetalDrawable`, 세 selector 응답값 모두 `0`. 확인 뒤 임시 probe app process를 종료했다. UI 화면 캡처는 만들지 않았다. 이 도구는 Spinon 제품 화면이나 실제 presentation callback을 테스트하는 앱이 아니다.

## 범위와 다음 단계

- 이 증거는 Xcode 26.2/iOS 26.2 Simulator에 한정한다. 다른 Xcode·iOS SDK 버전으로 일반화하지 않는다.
- iOS 기기에서의 `addPresentedHandler` 실행, `presentedTime == 0`/양수 callback, callback 수명·surface 교체·중복·늦은 전달은 미실행이다.
- wgpu 30.0.1 Metal HAL의 private drawable 소유권과 public wgpu 경계는 바뀌지 않았다. 앱 runtime의 `FrameId`/surface generation 귀속과 입력 직렬화도 미실행이다.
- Apple 문서의 API 설명: [`addPresentedHandler`](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler%28_%3A%29), [`presentedTime`](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime), [Metal Best Practices Guide: Drawables](https://developer.apple.com/library/archive/documentation/3DDrawing/Conceptual/MTLBestPracticesGuide/Drawables.html).

## Tailnet 문서 미리보기

Rspress 2.0.22 선택 빌드는 33 HTML route와 90개 output file을 생성했다. 실행 결과 텍스트는 [selected-build.log](selected-build.log)에 기록했고, 표의 줄 끝 공백만 정리했다. 이 선택 빌드는 전체 문서의 dead-link 검사 결과를 대신하지 않는다.

Tailnet 미리보기의 Agent Browser 확인 결과와 화면은 [검증 요약](browser-verification.json), [로드맵 화면](tailnet-roadmap-agent-browser.png), [상태 대장 화면](tailnet-status-agent-browser.png), [SDK 근거 화면](tailnet-evidence-agent-browser.png), [네트워크 기록](agent-browser-network.log)에 둔다. [브라우저 종료 기록](browser-session-shutdown.json)으로 확인 뒤 해당 브라우저 세션과 프로세스를 닫았는지 남긴다. [대체 파일 백업 검증](tailnet-final-backup-verification.json)도 함께 남긴다. 전체 Tailnet URL·SHA-256 검증은 같은 evidence 디렉터리의 `tailnet-final-served-verification.json`에 둔다. 이 화면은 문서 미리보기이며 Spinon 앱 동작 캡처가 아니다.
