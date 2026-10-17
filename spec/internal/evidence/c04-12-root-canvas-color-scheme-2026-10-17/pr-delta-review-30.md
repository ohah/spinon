# C04.12 root Canvas color-scheme PR diff 검토 30개

- **대상:** 현재 PR 변경 전체(런타임·FFI·Android/iOS adapter·계약·STATUS·계획·실행 증거)
- **기준 checkout:** `03c981726923f8638160f9918046d8316647848a`에서 시작한 전용 worktree
- **검토 방식:** 각 항목을 서로 다른 계약 경계나 실패 경로에 대조했다. 계획 리뷰와 구현 리뷰 표를 복사해 세지 않았다.

## 변경 검토

| # | 검토 관점 | 확인 결과 |
|---:|---|---|
| 1 | 내부 계약 숫자 버전이 출시 전 수정만으로 올라가는가? | 계약 0077은 `0.1.0-draft`를 유지한다. |
| 2 | 제한된 Canvas slice를 C04.12 전체 완료로 표시하는가? | `spec/STATUS.md`의 C04.12는 unchecked이고 완료 수 증가도 없다고 적었다. |
| 3 | native background만 구현한 것을 HTML body paint로 표현하는가? | 계약·README·STATUS가 body, text, box layout, paint 미지원 범위를 명시한다. |
| 4 | Canvas 색이 모든 CSS system color 또는 OS palette와 같다고 주장하는가? | 두 개의 pinned Chromium 155 sRGB byte만 적고 `CanvasText`, controls, status/navigation UI는 제외한다. |
| 5 | 실제 실행 screenshot이 Chrome oracle screenshot과 혼동되는가? | README에서 Chrome Agent Browser 캡처와 Android/iOS 앱 캡처를 구분한다. 로컬 이미지 링크를 `./` 경로로 지정해 RSPress가 캡처를 실제 이미지로 번들하도록 했고, C04.12 문서 subset build에서 다섯 캡처가 출력됨을 확인했다. |
| 6 | root CSS 값 대신 body/첫 child의 scheme을 사용할 수 있는가? | runtime은 root snapshot의 `root_node_id`에 해당하는 computed style만 선택한다. |
| 7 | `color-scheme: normal`에서 metadata 지원 scheme을 건너뛰는가? | metadata list와 OS preference를 결합하고, 없거나 invalid이면 light를 쓴다. |
| 8 | 명시적 root CSS 값을 meta declaration이 덮어쓰는가? | computed root 값은 `normal`일 때만 meta fallback을 사용한다. |
| 9 | `prefers-color-scheme`가 Canvas used scheme에 맞춰 변경되는가? | OS preference를 유지하고 Canvas used scheme을 별도로 반환한다. |
| 10 | `only light`/`only dark`가 OS에 의해 반전되는가? | `only`는 지정 scheme을 선택한다. 기존 metadata selection tests와 Chrome 표가 이를 구분한다. |
| 11 | 한 개의 invalid meta가 이후 invalid content를 선택적으로 채우는가? | 단일 invalid declaration은 supported list가 없는 상태이고 `normal` root는 light fallback이다. |
| 12 | 중복 meta의 두 번째 정상 값을 임의 선택하는가? | 중복 `meta color-scheme`이면 Canvas resolver가 fail-closed하며 해당 invalid-first/valid-second fixture로 검사했다. |
| 13 | 지원되지 않는 computed CSS token이 light로 조용히 바뀌는가? | unknown computed token은 별도 오류를 반환한다. `sepia` negative test가 통과한다. |
| 14 | Stylo cascade가 계산 전이거나 failed인 상태에서 색을 publish하는가? | pending timeout 및 failed state는 오류로 반환하며 성공 색은 completed snapshot 뒤에만 만든다. |
| 15 | 오래된 environment revision의 색이 새 theme으로 적용되는가? | requested/completed key, cascade environment revision과 반환 environment revision이 같아야 한다. |
| 16 | 문서·render tree·style revision이 서로 다른 root output을 결합하는가? | generation, document, render-tree, style 및 environment revision을 모두 비교한다. |
| 17 | root/style 누락 또는 둘 이상의 root에서 임의 computed property를 고르는가? | root 개수·root node style·property 누락은 각각 fail-closed 오류로 처리한다. |
| 18 | Rust의 큰 `Duration`에서 deadline 덧셈이 panic하는가? | 리뷰 중 `Duration::MAX` overflow 가능성을 찾았다. `checked_add`, bounded condition wait를 적용하고 timeout test에서 `Duration::MAX` no-panic을 추가 확인했다. |
| 19 | zero timeout이나 느린 cascade가 이전 completion을 새 결과처럼 반환하는가? | wait test가 pending 결과와 이전 completion 부재를 확인한다. |
| 20 | C ABI 성공 전에 결과 struct 일부가 덮이는가? | 인자/timeout 검사는 출력 write보다 앞서고 성공 경로에서 완성 struct를 한 번 쓴다. |
| 21 | C ABI scheme enum 또는 최대 timeout이 플랫폼과 다르게 해석되는가? | `0=light`, `1=dark`, timeout `0..60,000 ms`를 header·Rust·adapter에서 맞췄다. |
| 22 | C ABI 오류 code를 앱에 불투명한 배경색으로 바꾸는가? | Android/iOS bridge는 `status=0`일 때만 색을 파싱·적용하고, 오류는 로그로 남긴다. |
| 23 | Android cascade 대기가 main/UI thread를 막는가? | 초기와 변경 처리는 `bootstrapExecutor`에서 수행하고 색 적용만 UI thread에 보낸다. |
| 24 | Android 늦게 끝난 이전 요청이 더 최신 theme을 덮는가? | session id와 request generation을 UI 적용 직전에 재검사한다. |
| 25 | Android `onDestroy` 중 native session이 in-flight 환경 요청보다 먼저 free되는가? | root session을 free하기 전에 bootstrap executor 종료를 기다린다. 종료 race 자체는 별도 stress 실행으로 주장하지 않는다. |
| 26 | iOS serial background 호출이 UI view를 background thread에서 변경하는가? | UIView/window 색상 적용은 main queue에서 current session/generation을 재검사한 뒤 수행한다. |
| 27 | iOS terminate 중 cascade worker가 session을 사용 중인데 free하는가? | `applicationWillTerminate`가 environment queue를 drain한 뒤 session을 free한다. |
| 28 | 상태·계약이 테스트 harness 한계를 실제 C ABI 검증으로 과장하는가? | Rust root HTML 27개와 FFI 9개 테스트를 기록하고, FFI harness의 실제 V8 isolate 한계는 Android/iOS 앱 실행으로 보완했다. |
| 29 | 마지막 Rust 수정 뒤 플랫폼 앱이 다시 빌드되지 않았는가? | release Android APK와 iOS 26.2 Simulator app을 timeout 수정 뒤 재빌드했다. Android Gradle의 compile SDK 호환 경고는 evidence README에 남겼다. |
| 30 | 시뮬레이터 화면을 기기/renderer compositor proof로 확대하는가? | API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 캡처의 중앙 raw RGB 및 `0→1→2` key를 재확인했고, 실기기·GPU scene·compositor 뒤 픽셀은 명시적으로 제외한다. Tailnet roadmap도 native Canvas 배경만 반영하고 C04.12 미완료 및 HTML layout·paint 미지원임을 표시하며 새 계획·실행 근거 링크를 제공한다. |

## 판정

검토 중 찾은 Rust timeout overflow는 수정했고 영향을 받는 경계 테스트를 실행했다. 남은 diff에서 위 계약 경계와 검증 한계를 넘는 미해결 동작 결함은 발견하지 못했다. Rust root HTML 27/27, FFI cascade 9/9, timeout 대기 1/1, workspace Clippy, rustfmt, Android/iOS Simulator app build와 현재 앱의 light→dark→light evidence를 연결했다. 이는 전체 workspace test suite나 실제 기기 검증을 뜻하지 않는다.
