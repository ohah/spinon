# C04.12 · 앱 기본 시작 경로의 root HTML 화면

- **문서 유형:** 구현 계획 · 공식 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위:** [`plan/c04-12-root-html-default-rendering.md`](c04-12-root-html-default-rendering.md) · C04.12/C15/C16/C19는 계속 미완료
- **상태:** 기본 Android/iOS app route 후속 slice 구현을 마쳤다. 기본 제품 빌드, API 37 ARM64 AVD·iPhone 17 Pro·iPad Pro 13-inch M5/iOS 26.2 Simulator의 launch와 content-edge sRGB 표본, AVD 표면 크기 변경, product/fixture-positive staticlib boundary 대조, 새 구현 실패 관점 20개 및 PR diff 실패 관점 30개 검토를 마쳤다. [PR #326](https://github.com/ohah/spinon/pull/326)이 2026-10-18 UTC에 Rebase and merge됐고 병합 커밋은 `b99594d1b188e3addc0bd02802d12a53db1b8d95`다. C04.12/C15/C16/C19 상위 지원 범위는 계속 미완료다.
- **사전 기준:** [Agent Browser Chrome 155 관찰](../spec/internal/evidence/c04-12-default-app-launch-precomparison-2026-10-19/README.md)
- **계획 검토:** [서로 다른 실패 관점 20개](../spec/internal/evidence/c04-12-default-app-launch-precomparison-2026-10-19/plan-review-20.md)
- **구현 검토:** [구현 실패 관점 20개](../spec/internal/evidence/c04-12-default-app-launch-2026-10-19/implementation-review-20.md)

## 목표

Android와 iOS에서 실험 인수 없이 앱을 정상 실행하면 `examples/bootstrap/index.html`을 기본 GPU host의 root HTML로 파싱하고, `examples/bootstrap/root-app.js`를 그 host의 같은 V8 document session에서 실행한 첫 화면을 표시한다. 현재 별도 DOM-only `RuntimeSession`이 처리하는 root 문서 경로를 GPU host로 옮겨 문서를 중복 파싱하거나 root script를 다른 문서에서 실행하지 않는다. 기존 `app.js` smoke 실행은 이번 slice에서 동작 의미를 바꾸지 않는다.

화면은 native app content surface를 채우며 진단 제목·fixture control·C04.12 실행 인수에 의존하지 않는다. HTML의 `meta viewport`, `meta color-scheme`, root CSS `color-scheme`, native OS scheme, display scale은 표준 runtime 환경으로 전달한다. 첫 장면은 asset 읽기, host 생성, environment 설정, `root-app.js` 평가, layout/render가 모두 성공한 뒤에만 게시한다.

## 구현 전 확인한 경로

- Android 기본 시작은 `index.html`과 `root-app.js`를 읽고 `nativeSessionCreateWithRootHtml()`로 DOM-only session을 만든다. `root-app.js`는 그 session에 평가되지만 화면용 WGPU surface에는 연결되지 않는다.
- iOS 기본 시작도 DOM-only root document session과 빈 `RootDocumentHostViewController`를 사용한다.
- `--spinon-c0412-root-html-default-render` 확인 경로는 C0410 demo를 열고 별도 `root-html-default-rendering.html` asset을 읽는다. 이 경로는 실제 `index.html`/`root-app.js` 기본 launch가 아니며 진단 제목을 표시한다.
- 일반 `spinon_runtime_gpu_host_new_with_root_html()`, `spinon_runtime_gpu_host_eval()` 및 environment C ABI는 존재한다. Android/iOS 호스트의 root HTML 기본 launch에는 이를 호출하는 일반 eval wrapper와 기본 route 연결이 아직 없다.
- Android Gradle과 iOS Xcode project의 기본 앱 설정은 `SPINON_ENABLE_C04_RUNTIME_GPU=0`이다. 기본 화면이 일반 runtime GPU를 쓰도록 product feature를 켜되, 모든 fixture-test flag는 계속 꺼 둔다.

## 구현 진행

- Android/iOS 기본 route가 실제 bundle의 `examples/bootstrap/index.html`과 `root-app.js`를 일반 root HTML GPU host에 전달한다. asset byte 수, host 생성, environment, script marker와 첫 layout/render 상태가 런타임 로그에 남는다.
- 첫 화면 검증 중 body box 높이 아래의 표면이 검정으로 남는 문제를 발견했다. 기존 `RuntimeSession::resolve_root_html_canvas`가 계산한 색이 GPU surface clear에 연결되지 않은 것이 원인이었다. 이미 설정된 environment snapshot에서 Canvas 색을 계산하는 API를 분리하고, root HTML default host의 GPU clear color에 연결했다. 일반 runtime GPU scene은 이전 검정 clear를 사용한다.
- Android/iOS 제품 Simulator 빌드는 `c04_runtime_gpu=1`, fixture-test flags `0`으로 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 OS appearance light→dark→light 전환을 확인했고, 각 변경에서 `dark`와 `environment_revision`이 갱신됐다. 두 모드 모두 앱 content surface 경계 안쪽의 네 모서리와 중앙이 Chrome 155 관찰값과 같은 `(30,140,70)`이었다.
- 화면·빌드·런타임 로그와 artifact SHA-256은 [실행 근거](../spec/internal/evidence/c04-12-default-app-launch-2026-10-19/README.md)에 보존했다. Android PixelCopy, iOS compositor 이후 픽셀, 물리 기기와 hardware GPU는 검사하지 않았다.

## 구현 경계와 판정 기준

1. Android는 활성 `spinon_*` launch extra가 없을 때, iOS는 `--spinon-*` 실행 인수가 없을 때 새 기본 route를 택한다. Android의 일반 시스템 extra와 false/null `spinon_*` 값은 기본 앱 진입을 막지 않는다. 기존 활성 fixture·실험 인수는 현재 route와 충돌 거부 동작을 유지한다.
2. 기본 route는 bundle에서 실제 `index.html`/`root-app.js`를 읽고 두 파일을 일반 root HTML GPU host에 전달한다. native wrapper는 generic host create/eval/environment API를 호출하며 fixture 전용 C ABI, C12.4/C0412 fixture selector나 고정 HTML을 호출하지 않는다.
3. surface bounds에서 CSS viewport `width/height`, device scale, 현재 OS color scheme을 계산한다. 첫 `eval` 전에 최초 환경을 적용한다. Android AVD 표면 크기 변경과 iPhone/iPad의 서로 다른 surface size 및 light/dark 전환에서 최신 revision을 다시 계산하며 이전 revision 장면을 게시하지 않는지 확인한다. CSS viewport는 실제 native content surface bounds를 기준으로 하고 OS 상태 막대·홈 표시 영역을 임의로 늘리지 않는다.
4. root host에서 `root-app.js`를 실행하고 `data-runtime=ready`를 확인한 뒤에만 첫 render를 게시한다. 기존 DOM-only root document session은 같은 기본 route에서 중복 생성하지 않는다. `app.js` smoke session은 기존 의미와 로그를 유지한다.
5. root HTML asset 누락, 크기/UTF-8 거부, host/environment/eval/style/layout/render 실패에서는 부분 장면이나 이전 scheme 장면을 보여 주지 않는다. 일반 시작 오류를 플랫폼 로그에 남기고 surface는 결정된 단색 실패 상태로 유지한다.
6. 기본 앱 빌드는 C04 runtime GPU product feature를 포함하고 fixture features/payload는 포함하지 않는다. default build와 fixture-positive-control artifact를 대조해 제품 archive에서 C04/C15 fixture symbol과 Ahem 고정 payload가 빠진 것을 확인한다.
7. 기본 앱 화면에는 C0410 제목·fixture control이 없어야 한다. app content surface의 네 모서리와 가장자리까지 simulator screenshot으로 확인하고, Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 launch log에서 asset byte 수·`data-runtime` 성공·CSS viewport·DPR·scheme revision·GPU scene 게시를 기록한다. 물리 기기는 연결하지 않는다.
8. Agent Browser Chrome 155 기준은 실제 `index.html`을 320×240 CSS px, DPR 1, dark preference에서 연 뒤 `root-app.js`를 같은 page에서 평가한 결과다. 기준값은 title `Spinon`, lang `ko`, viewport meta `width=device-width, initial-scale=1`, color-scheme meta `light dark`, root computed `color-scheme: light dark`, body background `rgb(30, 140, 70)`, main text `Spinon 루트 HTML`, `data-runtime=ready`다. 이 기준은 HTML 입력의 독립 관찰값이며 모바일 OS 렌더 적합성 기준은 아니다.

### 제외 범위

일반 웹 전체 호환성, 임의 HTML/CSS·JavaScript 실행, 이벤트/입력 연결, network/font/image resource loading, 전체 viewport meta 문법, full safe-area CSS, 실기기·hardware GPU·iOS compositor 이후 픽셀, 전체 C04.12/C15/C16/C19 완료 판정은 포함하지 않는다. 현재 `index.html`에 필요한 제한 subset만 기본 시작 경로에서 확인한다.

## 검증 순서

1. Agent Browser Chrome oracle 입력과 root script 성공 조건을 고정한다.
2. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 인수 없는 기본 launch를 각각 실행한다. 기존 root HTML debug argument는 전달하지 않는다.
3. light/dark 전환과 surface resize를 각각 확인하고 log의 CSS viewport, DPR, environment revision 및 latest-scene 관계를 비교한다. 앱 캡처는 전체 content surface 가장자리까지 남긴다.
4. Rust focused tests, formatter, product/default 및 fixture-positive artifact 경계, Android/iOS app build를 실행하고 원본 로그·해시·화면을 근거 폴더에 기록한다. 실제 실행하지 않은 검증은 통과로 표시하지 않는다.
5. 별도 새 실패 관점 20개로 구현을 검토하고 PR 변경 diff는 30개 관점으로 검토한다. STATUS, 내부 계약, README, evidence, Tailnet roadmap 사본을 함께 갱신하고 PR을 Rebase and merge한다.
