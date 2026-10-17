# C15.3 WGPU glyph paint PR 변경 실패 관점 검토 30개

**대상:** `62af515afa853ba9acd669d1a82c505c6356cd77` 이후의 전체 후보 변경. 2026-10-17
**구분:** 이 문서는 변경 묶음이 기존 계약·플랫폼·기본 artifact에 미치는 위험을 본다. [구현 소스 실패 관점 20개](./implementation-review-20.md)와는 질문을 나눠 검토했다.

## 검토

1. **내부 runtime 타입이 공개 CSS API 완료로 해석될 수 있음** — Rust `RuntimeRenderText`는 내부 render model에만 추가됐다. contract 0072와 C15 상태는 DOM text admission/style projection이 없음을 명시하고 상위 체크를 유지한다. **허위 완료 없음.**
2. **새 text paint가 기존 box/image paint 순서를 바꿀 수 있음** — 별도 `DrawText` variant와 전체 command 소비가 추가됐고, 기존 `DrawBox` 명령을 제거하거나 재정렬하지 않았다. interleave test 및 WGPU command path가 순서를 확인한다. **회귀 없음.**
3. **snapshot 복제 시 text Arc가 유실되거나 오래된 revision에 붙을 수 있음** — scroll/rebind/clone 경로를 확인했다. text bytes는 Arc로 유지되고 `with_document_snapshot_revision`은 text가 있는 snapshot의 다른 document revision 재발행을 막는다. **안전하게 거부.**
4. **이미지 snapshot 재부착 중 text 명령이 깨질 수 있음** — `with_texts_and_paint_commands`가 원래 image 목록을 다시 검증해 연결한다. 새 scene에 image/text가 함께 있을 때의 full-frame 시각 matrix는 없음; 해당 결합은 이 fixed fixture의 목표 범위가 아니다. **코드 경계 확인, 혼합 resource 화면은 후속.**
5. **Rust API의 단위·경계가 앱 fixture 가정과 다름** — 좌표는 CSS px, mask는 physical raster px로 분리되고 scale·baseline·bearing이 snapshot에 남는다. 앱 route와 readback에서 viewport 360×240·scale 1을 교차 확인했다. **고정 입력 일치.**
6. **오류 variant 이동 중 기존 render API diagnostic 호환성 파손** — `RuntimeRenderError`를 별도 파일로 분리하고 기존 variant와 Display 텍스트를 대조했다. 추가된 text 오류 및 `InvalidPaintCommands` 문구는 목적을 나타낸다. **검토 중 box-only 진단을 고쳐 재확인.**
7. **repository 기본 빌드에 Noto Sans KR fixture bytes가 들어감** — font `include_bytes!`는 opt-in `spinon-ffi` feature 아래 `c15_3` 모듈에서만 컴파일된다. archive whole-byte negative/positive 대조에서 기본에는 font bytes가 없고 opt-in에는 있다. **기본 artifact 격리 통과.**
8. **새 ABI 함수가 feature 없이 링크되어 외부 계약으로 노출됨** — Rust `no_mangle`, C header 선언, JNI와 Objective-C++ wrapper가 동일 feature define으로 보호된다. 기본/opt-in static archive 검사에서 symbol 부재/존재를 대조했다. **통과.**
9. **새 optional dependency가 default dependency graph를 불필요하게 실행** — `spinon-text-layout`은 FFI feature에 optional로 연결되고 fixture 모듈 import도 cfg 아래다. default archive에서 fixture/font가 빠지는 positive/negative artifact 확인이 이를 보완한다. **통과.**
10. **root와 spike lockfile이 새 text-layout transitive dependency를 놓침** — locked root FFI fixture build와 Android/iOS build가 통과했고 두 lockfile에 feature graph가 반영됐다. **locked build 통과.**
11. **C15 feature를 켰지만 C04.10 GPU route를 끄고 실행** — Gradle, Android script, iOS native script와 iOS simulator script가 두 값을 검증하고 조합 오류를 내도록 연결됐다. 실제 positive simulator build flags에서 C04=1·C15.3=1이었다. **통과.**
12. **잘못된 feature 값이 조용히 true 또는 false가 됨** — Gradle·shell entrypoint가 `0|1`만 받는다. 기존 실행에 feature-off negative route rejection을 별도로 보존했다. **통과.**
13. **Android opt-in Java adapter가 기본 source set에 실수로 들어감** — `build.gradle.kts`가 flag=1일 때만 `src/c15_3_wgpu_glyph_paint_fixture_test/java`를 추가한다. feature-off APK가 성공했고 route는 `SPINON_C153_DISABLED`로 거부했다. **통과.**
14. **Android JNI symbol과 source-set native method 이름이 다름** — `GlyphPaintFixtureNativeAdapter.createHost/evalFixture` signature와 JNI mangled names를 대조했고 AVD에서 두 경로의 log를 받았다. **통과.**
15. **Android handle 전달 중 넓이·부호 변환 문제** — `uintptr_t ↔ jlong`가 64-bit AVD에서 왕복해 앱 route가 실행됐다. 32-bit ABI용 별도 런타임은 사용하지 않았고 project target은 현재 64-bit simulator/AVD matrix다. **현재 대상 통과, 32-bit 미검증.**
16. **JNI report buffer가 고정 font/layout 보고서보다 작아 조용히 잘림** — host 생성은 2 KiB, eval은 32 KiB buffer를 쓰고 Rust write path가 capacity 실패를 반환한다. AVD에서 성공 report에 glyph count가 있었고 Java가 잘못된 report를 거부한다. **통과.**
17. **Android extra가 feature-off일 때 다른 CSS fixture로 우회** — `MainActivity`가 raw extra를 feature check보다 보존하고 즉시 reject한다. feature-on은 C0410 route에서 다른 fixture와 동시 선택을 거부한다. **off/on 양쪽 확인.**
18. **Android 여러 fixture extra의 ternary 우선순위가 C15.3 결과를 가장할 수 있음** — 선택된 route 수를 검증한 뒤 C15.3 extra를 단독 허용한다. AVD run log는 fixture route, glyph attachment와 한 batch 제출을 각각 확인했다. **통과.**
19. **simulator 검증이 실기기 ADB를 의도치 않게 대상으로 삼음** — evidence command는 `adb -s emulator-5554`를 고정했고, property snapshot은 API 37/Android 17 ARM64 AVD를 가리킨다. 실기기 ID에는 명령을 보내지 않았다. **범위 준수.**
20. **iOS app이 launch argument가 있어도 runtime GPU 화면으로 진입하지 않음** — `AppDelegate`가 `--spinon-c153-glyph-paint`를 runtime GPU route 선택에 포함하고, Simulator log에 생성·eval·Metal draw가 남았다. **통과.**
21. **iOS fixture flag가 Xcode default 또는 Swift conditional에 빠짐** — Debug/Release target의 default=0, C preprocessor define과 Swift `_1` condition을 대조하고 positive/negative build를 실행했다. **통과.**
22. **Objective-C++와 Swift 선언·구현 signature가 서로 다름** — `SpinonRunner.h/.mm` selector, `uint64_t` handle, `NSString` report 선언/구현이 일치하고 iOS Simulator positive route가 실행됐다. **통과.**
23. **iOS argument collision에서 복수 fixture가 함께 실행됨** — C15.3 flag를 포함하는 selected-route count와 raw `--spinon-c*` argument 거부를 확인했다. feature-off rejection은 저장했고 positive route는 단독으로 실행했다. **통과.**
24. **실패한 iOS host handle을 성공 route로 오인** — host 생성 실패는 0을 반환하고 Swift eval은 `status=0`, attached text, glyph count를 요구한다. **오류 전파 확인.**
25. **Swift task 취소 후 이전 glyph batch가 성공 로그로 남음** — draw path는 latest-task validation을 통과하고 WGPU 제출 뒤도 revision을 검사한다. cancellation 전용 C15 mutation은 없지만 기존 latest-task gate를 그대로 사용하며 stale는 ERR_STALE로 반환한다. **기존 gate 연결, C15 취소 stress는 미검증.**
26. **Android/iOS viewport와 app density가 fixture layout을 바꾸어 서로 다른 입력을 비교** — 두 route에서 viewport 360×240·scale 1과 36 glyph를 로그로 검사했다. Android surface 945×630/SwiftShader, iOS 1080×720/Metal은 각각 구분해 기록했다. **입력 일치, device DPR 독립 fixture.**
27. **full-screen screenshot을 surface pixel proof로 잘못 표시** — evidence README와 STATUS가 OS screenshot을 PixelCopy/Metal drawable readback과 분리한다. 실제 coverage/clip pixel 근거는 offscreen WGPU test에만 귀속했다. **과장 없음.**
28. **fixture route가 default artifact와 opt-in artifact에서 똑같이 활성화** — exact byte scans가 C15.3 ABI symbol·fixture JS·font bytes의 기본 부재와 opt-in 존재를 모두 확인했다. **양성·음성 대조 통과.**
29. **PR 뒤 roadmap·공식 상태가 parent C15.3을 완료로 바꿈** — 계획·STATUS는 이 slice를 작은 runtime paint 단계로 따로 기재하며 C15/C15.3 parent 체크를 유지한다. **허위 완료 없음.**
30. **로그·hash·화면이 다른 실행이나 잘못된 API 이름에 연결** — Android 디렉터리를 실제 API 37/Android 17로 고치고 iOS device identity를 Simulator UDID/runtime으로 다시 기록했다. README는 현재 final-reviewed 화면, APK/app hash와 device log를 직접 링크한다. 최종 소스 tree로 Android API 37/Android 17 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 다시 빌드·실행했다. 새 화면·실행 log·APK/app executable hash를 기록하고, iOS `RawWindowMetalLayer` 중복 링크 경고도 성공 결과와 분리해 기존 C13.1 log와 대조했다. **닫힘, 경고를 숨기지 않음.**

## 결과

이 변경 묶음의 기본/opt-in 경계, FFI 이름, route 선택, viewport, paint command와 상태 표기를 확인했다. 현재 slice에 공개 text admission이나 CSS layout 지원을 추가하지 않는다. 최종 Simulator 재실행과 hash 대조 뒤 이 검토의 모든 30개 항목을 닫았다. 별도 negative control은 feature-off route 기록을 함께 보존한다.
