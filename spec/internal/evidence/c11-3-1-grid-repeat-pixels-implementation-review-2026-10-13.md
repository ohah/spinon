# C11.3.1 구현·표면 픽셀 경계 검토

이 검토는 C11.3.1 정수 `repeat()` 경로와 Android PixelCopy·iOS Metal drawable 검증을 대상으로 한다. 계획 검토와 별개이며, 한 scene의 결과를 CSS Grid 전체 지원으로 일반화하지 않는다.

## 실패 관점 검토

1. **CSS parser와 adapter 경계** — 선언 문자열을 새로 해석하지 않고 Stylo의 typed computed `TrackList`를 읽는다. authored stylesheet 입력은 cascade test를 통과한다. 임의 CSS 문자열을 native runtime에서 재파싱하지 않는다.
2. **지원하지 않는 track 문법** — auto-repeat, 중첩 repeat, zero·음수·소수 count, `minmax()`와 unsupported area 선언이 성공 경로에 들어오지 않는지 봤다. 고정된 cascade 테스트가 기대 오류를 확인한다.
3. **반복 개수 산술 overflow** — track-list 길이, repeat count 곱, 합산 track 수, N+1 line 수를 checked arithmetic으로 계산한다. 10,000개는 수용하고 10,001개는 부분 track 없이 거부하는 테스트가 있다.
4. **metadata 예산 계산** — line-group header, name header, 문자열 bytes의 예상치를 checked arithmetic으로 합산하고 축별 1 MiB 초과를 거부한다. 제한은 예상치이며 프로세스 RSS hard cap이라고 표현하지 않는다.
5. **repeat seam과 중복 이름** — 반복 경계가 같은 physical line group으로 합쳐지고 name 순서·중복이 유지되는지 봤다. N+1 line-name invariant와 duplicate-name fixture가 이를 검사한다.
6. **allocation 실패 후 부분 결과** — track, line group, line-name vector, name text reserve를 독립 실패시킨다. 각 경우 contextual error로 끝나고 다음 정상 cascade 계산이 성공하는 테스트가 있다.
7. **기존 profile 회귀** — line-name projection은 `RuntimeBlockFlexGridTemplateV1`에서만 켜지고 이전 Grid profile은 빈 metadata 경로를 유지한다. projection fixture는 새 track 결과가 layout core까지 도달하는지 대조한다.
8. **runtime 입력 범위** — FFI runtime matrix는 `multi-track-repeat` 하나만 노출하고 schema, profile, node 수, source 크기와 scene ID를 검사한다. 임의 JS source나 알 수 없는 scene ID는 받지 않는다.
9. **Chromium oracle 구별** — Chrome for Testing 버전·revision·fixture hash·WPT revision이 고정돼 있다. Chrome WPT 후보 5/5는 Chrome oracle 결과이며 Spinon WPT 적합성으로 기록하지 않는다.
10. **runtime geometry 범위** — Android와 iOS에서 같은 한 scene의 item frame을 확인했지만 HostDocument 4 node / 단일 장면뿐이다. scene matrix 전체 실행 또는 C11.3.1 전체 완료라고 승격하지 않는다.
11. **픽셀 oracle 변조** — Chrome reference JSON과 DPR 1 PNG hash를 확인하고 실제 PNG에서 각 fixture RGBA를 다시 추출한다. 표면 비교기는 이 고정 색과 위치를 독립적으로 쓴다.
12. **표본 위치의 가장자리 혼입** — 표본은 배경·단색 item 안쪽과 두 gap 내부에 두어 색 경계·안티앨리어싱을 밟지 않는다. 이 다섯 점만으로 전체 픽셀 parity를 주장하지 않는다.
13. **Android 표면 좌표** — CSS 좌표를 density 값으로 곱하지 않고 실제 PixelCopy buffer와 320×240 CSS viewport 비율로 환산한다. begin·capture·PNG 크기 및 좌표가 서로 맞아야 비교가 통과한다.
14. **iOS drawable 좌표와 revision** — drawable 크기에서 x/y scale을 계산하고 두 축이 같은지 확인한다. begin·summary의 run ID, PID, surface generation, document/render/environment revision이 동일해야 한다.
15. **잘못된 표본 요청의 조기 거부** — `item-blue.xCssPx`를 1 증가시킨 negative control을 양 플랫폼에서 실행했다. Android PixelCopy와 iOS Metal readback 시작 전에 고정 allowlist가 요청을 거부했다.
16. **중복·오래된 로그 결합** — 비교기는 실행별 begin/summary가 하나씩이고, sample ID 집합이 fixture와 같으며 PID·scene·generation이 일치하는지 확인한다. 이전 실행 로그가 섞여도 다른 run ID는 합격으로 계산하지 않는다.
17. **PNG와 로그 불일치** — Android 비교기는 marker에 적힌 PNG SHA를 원본 bytes와 대조하고 PNG의 각 pixel도 sample log RGBA와 비교한다. 전체 sample 수, 채널 수와 0–255 범위를 검사한다. 비교기 회귀 테스트의 합성 C11.3.1 로그가 runtime `expected`·`exact` 필드를 빠뜨려 실패한 것을 확인해 테스트 입력을 실제 marker 계약과 맞췄다.
18. **FFI 포인터·크기 경계** — null host/output/coordinates, 1–64 count, `count * 2` overflow, `try_reserve` 실패를 검사한다. Rust `unsafe` 계약에 호출자 buffer 길이와 동일 host 직렬 호출 조건이 적혀 있다.
19. **readback 전용 빌드 경계** — pixel request FFI는 내부 test feature가 켜졌을 때만 컴파일하고 iOS build log에서 Rust feature와 C++ define 전달을 확인했다. 공개 CSS/API 지원 선언으로 취급하지 않는다.
20. **근거 생성과 실제 실패** — Android 검증 중 Java marker 식의 컴파일 오류를 발견해 수정하고 APK를 다시 빌드했다. 첫 negative-control 재실행은 UI가 오류를 표시했지만 진단 로그를 수집하지 못했다. 생성자에서 입력 거부 marker를 남기고 실패 시 log/JSON을 assert 전에 저장하도록 고쳐, 이후 AVD·iOS 재실행에서 조기 거부를 확인했다. 결과의 해시 필드도 Android APK와 iOS executable의 실제 설치 산출물 경로를 함께 기록하도록 명확히 했다.

## 판정과 미완료 경계

- 코드·비교기·고정 Chrome 기준과 실제 플랫폼 readback 경로는 위 조건에서 통과했다.
- C11.3.1 픽셀·사전비교·Android 비교기 자동 테스트는 40/40 통과했고, iOS/Android 빌드 flag 검사를 포함한 49개 JS 테스트도 통과했다. 전체 Rust workspace 테스트는 658 passed, 0 failed, 4 ignored이며 workspace Clippy `-D warnings`도 통과했다. Android native bootstrap과 C04·C11.2.3·pixel flag를 켠 iOS Simulator 앱 빌드도 성공했다.
- 테스트 작성 중 Android 비교기용 합성 로그가 runtime의 `expected`·`exact` 필드를 빠뜨린 실패를 발견해 fixture를 수정했다. 이 수정은 테스트 입력만 바꾸며 pixel 기준·비교 허용치를 바꾸지 않는다.
- Android API 37 AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 renderer다. 초기 SM-S731N/Android 16 결과는 혼합 작업 트리 진단이다. 이후 단독 후보 source tree `96f2c8c2a75bd2ce43be735a27b212271bd0c51a`에서 SM-S731N 실기기 PixelCopy와 iOS Simulator Metal drawable gate를 각각 5/5 RGBA exact로 통과했다. iOS 값은 OS compositor 제출 전 drawable이다.
- iOS 결과는 iPhone 17 Pro / iOS 26.2 Simulator의 Metal drawable, OS compositor 제출 전 값이다.
- 테스트 표본은 각 플랫폼 5개이며 screenshot은 화면 상태를 보여 주는 증거다. pixel comparator는 screenshot을 입력으로 쓰지 않는다.
- 최초 구현 실패 검토 시점에는 Android 실기기 단독 소스 gate가 남아 있었다. 아래 후속 source gate에서 그 기기 gate를 통과했다. Spinon WPT 적합성·전체 CSS Grid·전체 화면 pixel parity는 여전히 완료되지 않았다.

## 단독 후보 source gate 후속

초기 격리 후보를 iOS로 실행하며 두 결함을 찾았다. C11.3.1 scene 인수가 iOS 앱의 GPU 화면 라우팅에 연결되지 않아 C11.1 validator 경로로 빠졌고, Metal 표면 비교기는 `C1131` marker와 해당 schema를 허용하지 않았다. iOS 전용 host 생성·scene 평가·표본 readback 호출과 marker/parser 회귀 검사를 추가한 다음, 후보 source tree `96f2c8c2a75bd2ce43be735a27b212271bd0c51a`에서 Android 실기기 PixelCopy 및 iOS Simulator Metal drawable gate를 다시 실행했다. 양쪽 모두 다섯 표본 RGBA exact, 최대 채널 오차 0이었고 좌표 negative control은 표면 readback 전에 거부됐다. 실제 실행 ID와 산출물 hash는 [source gate manifest](./c11-3-1-grid-repeat-pixels-2026-10-13/source-gate.json)에 있다.

이 통과 결과는 고정된 `multi-track-repeat` 한 장면의 다섯 표본과 지정 기기/시뮬레이터에 한정한다. 전체 화면 픽셀 parity, 다른 scene, 다른 GPU, iOS 실기기, OS compositor 제출 이후 결과, Spinon WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.
