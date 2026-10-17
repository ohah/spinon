# S04.10 iOS SDK·Simulator 근거 PR 변경 검토 30개

검토 대상: iOS device/simulator SDK 비교 근거, S04.10 상태·내부 인덱스 정정, 원본 API 조사 보충.<br>
검토 기준: 각 항목은 별도의 증거 경계·잘못된 확대 해석·재현 실패 경로를 확인한다.

| # | 적대적 실패 관점 | 확인 결과 |
|---|---|---|
| 1 | Simulator SDK만 보고 모든 iOS SDK에서 API가 없다고 결론낸다 | **PASS.** iPhoneOS 26.2 SDK를 별도 검사했고 결과를 Simulator 결과와 분리했다. |
| 2 | Device SDK에 선언이 있다는 이유로 Simulator 빌드도 컴파일된다고 주장한다 | **PASS.** 동일 Swift source를 두 SDK에 각각 type-check했고 Simulator 실패를 기록했다. |
| 3 | API 표기 차이가 Swift overload/import 문제일 수 있다 | **PASS.** SDK `MTLDrawable.h` 원문과 컴파일 결과를 함께 대조했다. |
| 4 | `CAMetalDrawable`과 `MTLDrawable` 타입을 혼동한다 | **PASS.** probe는 실제 사용 경계인 `any CAMetalDrawable`을 받는다. |
| 5 | 세 API 중 일부만 누락됐는데 모두 없다고 과장한다 | **PASS.** `addPresentedHandler`, `presentedTime`, `drawableID`를 각각 compile/runtime 확인했다. |
| 6 | type-check 성공을 실제 기기 API 실행으로 확대한다 | **PASS.** device SDK 결과는 컴파일 근거라고만 적고 기기 runtime은 미실행으로 분리했다. |
| 7 | selector 부재를 실제 callback이 호출되지 않았다는 결과로 둔갑시킨다 | **PASS.** Simulator drawable은 세 selector에 응답하지 않았다고만 적었다. callback 제출/호출은 시험하지 않았다. |
| 8 | selector 응답 결과를 실제 표시 시각 측정으로 오해한다 | **PASS.** probe가 drawable을 present하지 않았고 시각값을 측정하지 않았다고 명시했다. |
| 9 | Simulator selector 결과를 모든 iOS Simulator/Xcode 버전으로 일반화한다 | **PASS.** Xcode 26.2, iOS 26.2 Simulator runtime에 한정했다. |
| 10 | 서로 다른 SDK 버전에서 비교해 차이를 잘못 귀속한다 | **PASS.** 두 SDK 모두 26.2이며 동일 Xcode 설치에서 사용했다. |
| 11 | SDK 파일이 로컬에서 바뀌어 결과를 재현할 수 없다 | **PASS.** 두 `MTLDrawable.h`의 SHA-256을 근거에 기록했다. |
| 12 | 과거 probe 실패를 API 전체 부재 증거로 유지한다 | **PASS.** 기존 Simulator probe 결과의 범위를 정정하고 device SDK 결과를 보충했다. |
| 13 | Apple 공식 API 설명과 SDK availability가 모순된다고 숨긴다 | **PASS.** Apple 문서 링크와 실제 SDK별 차이를 같이 남겼다. |
| 14 | wgpu가 공개적으로 drawable callback을 제공한다고 오해한다 | **PASS.** wgpu 30.0.1 public API와 private Metal drawable 경계가 바뀌지 않았다고 적었다. |
| 15 | callback이 FrameId나 surface generation에 귀속됐다고 주장한다 | **PASS.** 둘 다 미실행 및 S04.10 미완료로 남겼다. |
| 16 | 제품 app UI가 검증됐다고 해석한다 | **PASS.** 이 probe는 selector 확인용 임시 app이며 제품 UI 캡처가 없다고 기록했다. |
| 17 | simulator diagnostic app을 제품 산출물로 포함한다 | **PASS.** source와 plist만 evidence에 두고 binary app은 repo에 추가하지 않았다. |
| 18 | S04.10/S05.2를 진행률 완료 처리한다 | **PASS.** 두 상태를 체크하지 않고 기존 미완료 사유를 유지했다. |
| 19 | 내부 evidence 링크가 인덱스/상태 문서에서 끊긴다 | **PASS.** 새 evidence 경로를 `spec/internal/README.md`, `spec/STATUS.md`, API audit에서 연결했다. |
| 20 | 문서 변경이 선택 문서 사이트에서 렌더링되지 않는다 | **PASS.** Rspress 2.0.22 선택 빌드에서 33 HTML route와 90 output files를 생성했다. 전체 저장소 dead-link 검사를 통과했다고 확대하지 않는다. |
| 21 | 두 SDK type-check가 서로 다른 probe source를 사용한다 | **PASS.** 두 명령이 같은 `metal-api-probe.swift` 경로를 사용한다. |
| 22 | Simulator target triple이 실제 Simulator ABI를 지정하지 않는다 | **PASS.** probe target은 `arm64-apple-ios26.2-simulator`; device 비교 target은 `arm64-apple-ios26.2`다. 결과는 해당 Xcode 26.2 설치에 한정한다. |
| 23 | Selector probe가 임의 drawable 대신 예상 타입만 기록한다 | **PASS.** 실행 로그는 `CAMetalLayer.nextDrawable()`에서 받은 객체 class가 `CAMetalDrawable`임을 기록한다. nil drawable·layer 생성 실패를 API 부재로 세지 않는다. |
| 24 | Objective-C `respondsToSelector:` 결과를 callback 호출 실험으로 오해한다 | **PASS.** 결과는 세 selector의 dispatch 지원 여부만 기록한다. `present()` 호출이나 callback 등록·호출은 없었다. |
| 25 | `presentedTime`의 0 또는 양수 값에 대한 관찰값이 있다고 주장한다 | **PASS.** probe가 화면 제출을 하지 않았으므로 시간값 표본이 없다고 명시한다. |
| 26 | Simulator API 누락을 OS 구현의 모든 버전 또는 플랫폼으로 확대한다 | **PASS.** 근거는 iOS 26.2 Simulator runtime과 iPhoneSimulator 26.2 SDK에만 한정하며, Android와 다른 Xcode 버전은 판단하지 않는다. |
| 27 | 기기 SDK type-check 성공을 기기 실행이나 wgpu 연동 증거로 승격한다 | **PASS.** 실제 iOS 기기, wgpu drawable 소유권, FrameId 귀속은 별도 미검증이라고 분리했다. |
| 28 | API 가용성 근거가 가변 로컬 경로만 가리켜 SDK 변경을 식별하지 못한다 | **PASS.** 검사 시점의 두 `MTLDrawable.h` SHA-256과 Xcode/SDK build 번호를 함께 기록했다. 이는 그 파일의 식별자이며 전체 SDK 무결성 주장으로 확대하지 않는다. |
| 29 | Tailnet에 기존 문서만 갱신하고 새 evidence 원문 또는 빌드 페이지는 빠뜨린다 | **PASS.** 최종 동기화 preflight에 선택 빌드 산출물과 변경된 source evidence를 포함하고, 사전 백업 뒤 각 대상 경로·SHA-256·HTTP 상태를 별도로 대조한다. |
| 30 | 선택 문서 빌드를 전체 사이트 검사 성공으로 오해하거나 PR을 S04.10 완료로 본다 | **PASS.** 선택 빌드는 제한된 33 HTML route에 한정하고 `checkDeadLinks`·`checkDeadImages`를 비활성화한다. 전체 문서 검사는 이 선택 빌드 범위에 포함하지 않으며, callback/device/runtime 미검증과 S04.10/S05.2 미완료 상태를 분리한다. |

## 판정

검토에서 발견한 문구 과장은 SDK 전체 API 부재라는 범위 오류였다. evidence, 상태 대장과 API audit에서 이를 기기 SDK와 Simulator SDK/runtime 차이로 한정했다. 새 callback 구현이나 S04.10 완료를 주장하는 문장은 추가하지 않았다.
