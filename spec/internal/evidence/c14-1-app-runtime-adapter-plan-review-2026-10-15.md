# C14.1 Android/iOS 앱 fixture adapter · 계획 적대 검토

검토 대상은 [Android/iOS 앱 fixture adapter 계획](../../../plan/c14-1-app-runtime-adapter.md)이다. 구현은 아직 시작하지 않았으며, 기존 C14.1 계획 검토를 재사용하지 않고 새 cross-platform 범위를 대상으로 다시 확인했다.

| # | 독립 실패 관점 | 계획의 판정·완화 |
| ---: | --- | --- |
| 1 | 앱 report가 live V8 HostDocument를 사용했다고 과장한다. | fixture-owned synthetic HostDocument라고 명시하고 live V8 경로는 호출하지 않는다. |
| 2 | Android만 확인한 뒤 iOS도 확인된 것처럼 보고한다. | 각 simulator에 별도 build/run/log/screenshot 성공 조건을 둔다. |
| 3 | iOS UI route는 있지만 ObjC++ symbol 또는 Swift 호출 선언이 없어 실행되지 않는다. | Xcode preprocessor define부터 C ABI header, ObjC++ wrapper와 Swift screen까지 계층을 나눠 확인한다. |
| 4 | Android Gradle 값과 native compile define/Cargo feature가 서로 다른다. | 단일 environment flag를 Gradle `BuildConfig`, C define, Cargo feature로 연결하고 feature-off/on artifact를 검사한다. |
| 5 | iOS Xcode 값이 Debug/Release 또는 shell 빌드 경로 중 하나에서 누락된다. | 두 설정 구성과 공통 build script 전달을 확인 항목에 포함한다. |
| 6 | C14 fixture code가 기본 앱 ABI/정적 라이브러리에 노출된다. | 기본값 0을 고정하고 default artifact symbol/payload 부재를 성공 조건으로 둔다. |
| 7 | 기본 app route가 fixture HTML/PNG 문자열을 포함한다. | include/payload가 opt-in Cargo feature 내부에만 있도록 경계를 확인한다. |
| 8 | 앱 intent가 C11/C13/기타 fixture route와 함께 와도 하나를 조용히 선택한다. | 다른 fixture 인자와 동시에 들어오면 native host 생성 전에 거부한다. |
| 9 | 두 앱이 서로 다른 fixture 입력 또는 viewport로 실행된다. | 동일 U05.1 CSS/PNG/reference, 120×80 CSS px viewport, Chrome oracle을 고정한다. |
| 10 | device density나 iOS scale/safe area가 natural CSS px를 변형한다. | CSS viewport만 layout에 전달하고 OS 화면 metrics는 비교 계산에서 제외한다. |
| 11 | 두 auto 축 계산은 맞아도 한 축 auto, ratio, min/max까지 지원한다고 확대한다. | 고정 fixture 성공만 주장하고 다른 sizing 시나리오는 범위 밖으로 둔다. |
| 12 | frame 좌표 일부나 다른 node를 비교해 실제 대상 image가 틀려도 통과한다. | node identity, x/y/width/height 네 값을 각각 Chrome과 비교한다. |
| 13 | image request/document generation이나 intrinsic revision이 어긋난다. | 매 실행 새 HostDocument와 snapshot을 쓰고 report에서 source 및 revision identity를 확인한다. |
| 14 | decode 오류, 잘못된 report, 부족한 output buffer가 성공 화면으로 바뀐다. | native 반환 상태·UTF-8·capacity를 검사하고 오류 경로는 success marker를 금지한다. |
| 15 | Android JNI 계산이 UI thread를 막거나 결과 view를 경쟁적으로 갱신한다. | 계산은 background executor에서 하고 main looper에서만 화면을 갱신한다. |
| 16 | iOS ObjC++ 계산이 main thread에서 돌거나 label이 앱 전환 전에 사라진다. | GCD background queue에서 호출하고 main queue에서 결과 view를 갱신한다. |
| 17 | report text screenshot이 이미지 pixel paint 또는 WGPU 결과라고 해석된다. | 두 계획과 PR에서 텍스트 계산 report로 한정하며 paint/GPU 증거 주장은 금지한다. |
| 18 | Android ADB가 연결된 실기기를 자동 선택한다. | 모든 실행에 AVD serial을 지정하고 AVD/API/ABI 증거를 저장한다. |
| 19 | iOS 캡처가 다른 booted 기기나 physical device와 섞인다. | 명시한 Simulator UDID로 launch/screenshot하고 대상 모델·OS를 기록한다. |
| 20 | C13.1의 border/overflow 검증이나 상위 C14가 이번 adapter로 재완료 처리된다. | C13.1 근거는 별도 링크와 제한을 명시하고 C14.1/C14 상위 상태는 미완료로 둔다. |

## 판정

20개의 서로 다른 Android/iOS 경계·fixture·revision·오류·화면 해석 관점으로 계획을 다시 검토했다. 양쪽 앱은 같은 opt-in C ABI를 호출하지만 OS bridge, build define, route, execution thread와 캡처는 별도 확인한다. overflow paint는 C13.1의 기존 evidence로 구분되며 이번 C14.1 report-only route에 합쳐지지 않는다. 구현 전에 해결할 차단 결함은 없다.
