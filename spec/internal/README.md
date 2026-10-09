# 내부 인터페이스 명세

## R06 큐 포화·복구

- [구현 전 비교 모델](./evidence/r06-queue-saturation-precomparison-2026-10-08.md) · [작업 계획·구현 점검](../../plan/r06-queue-saturation.md) · [실제 V8 실행 근거](./evidence/r06-queue-saturation-2026-10-08.md)

이 폴더에는 앱 작성자에게 공개하지 않는 Rust·C++·플랫폼 사이 호출 계약을 둡니다. 내부 ABI라도 호출자, 입력·출력, 소유권, 오류와 현재 한계를 기록합니다. 이 문서는 제품 API 지원 완료로 연결하지 않으며 상태 표시는 [공식 대장](../STATUS.md)을 따릅니다.

| 문서 | 범위 | 상태 |
| --- | --- | --- |
| [0001 · V8 부팅 실험](0001-v8-bootstrap.md) | Bun 번들, Rust FFI, V8 C++ 어댑터와 Android/iOS 빌드 smoke | 실험 전용 |
| [0002 · Rust 트리 코어](0002-rust-tree-core.md) | 노드 ID, 트리 구조, 원자적 변경 묶음과 revision | 실험 전용 |
| [0003 · 공통 문서·호스트 계약](0003-shared-host-contract.md) | DOM 호환 계층과 프레임워크 어댑터의 문서 모델, 소유권, 동기 변경과 이벤트 경계 | 코어·제한 façade 일부 내부 구현 · 나머지 제안 초안 |
| [0004 · R06 스레드·소유권 위험 분석](0004-thread-ownership-risks.md) | Isolate·문서·콜백·revision·비동기 완료·종료 경계의 위험과 검증 후보 | 검토 초안 · R06 미완료 |
| [0005 · V8 런타임 세션 실험](0005-v8-runtime-session.md) | 세션별 Isolate 소유 스레드, 용량 제한 우선순위 큐, 취소·종료 경계 | 실험 전용 · R06 미완료 · 우선순위와 제한된 종료 경합의 실제 V8 시뮬레이터 검증 통과 |
| [0006 · JavaScript 작업 스케줄러](0006-js-task-scheduler.md) | Chromium 참고 우선순위 선택과 앱 작업 출처·프레임·취소 경계 | Android·iOS Simulator 실제 V8에서 1,087개 유입 및 큐 포화·거부·복구를 각 5회 통과 · 무한 유입 보장과 제품 역압력 정책은 미결정 |
| [0007 · 내장 UA stylesheet 자원](0007-ua-stylesheet-resource.md) | 지원 HTML 기본 CSS 자원과 읽기 전용 FFI 인터페이스 | 내부 초안 · fixture cascade 연결됨 · 제품 runtime은 미연결 |
| [0008 · C02 Vite·Rspack CSS 비교 모델](0008-css-bundler-c02.md) | CSS 산출·자원·청크·오류 위치·resolver 비교 조건 | 내부 실험 계약 · C02 미완료 |
| [0009 · 레이아웃 엔진](0009-layout-engine.md) | S01 Tree 및 HostDocument 요소 snapshot, source·style·environment revision stamp, Taffy 프레임과 fixture stale-admission 경계 | 구현 초안 `0.3.0-draft` · 제한된 Flex subset · 제품 revision 관리자와 GPU queue stale 검사는 미구현 |
| [0010 · C03 Stylo DOM adapter](0010-stylo-dom-adapter-c03.md) | HostDocument snapshot에서 Stylo 문서·노드·요소·선택자 DOM으로의 변환 계약 | 내부 구현 계약 초안 `0.1.0-draft` · C03 구현 완료, 계산 스타일 제외 |
| [0011 · C02 CSS 자원 어댑터](0011-css-resource-adapter-c02.md) | Vite·Rspack 산출을 빌드 단위 공통 CSS 자원 snapshot으로 정규화 | 내부 계약 후보 `0.1.0-draft` · fixture 스파이크 전용 · 제품 API 아님 |
| [0012 · C04 stylesheet 입력 목록](0012-stylesheet-registry-c04.md) | Stylo stylesheet 파싱, CSS 출처·등록 순서와 parser 진단 보존 | 내부 구현 계약 초안 `0.1.0-draft` · cascade 계산 미연결 |
| [0016 · C04 기본 stylesheet cascade](0016-c04-basic-cascade.md) | 불변 HostDocument revision의 UA·author·inline cascade와 whitelist computed-style snapshot | fixture 전용 내부 구현 계약 `0.1.0-draft` · Chromium 비교 통과 · 제품/API 연결 미완료 |
| [0017 · C04 computed style→Taffy 입력 adapter](0017-c04-style-layout-bridge.md) | revision이 일치하는 computed-style snapshot을 제한 Taffy 입력으로 변환하고 진단·미지원 값을 전체 실패 처리 | 내부 구현 계약 `0.1.0` · 고정 fixture 검증 완료 · 제품 runtime/API 미연결 |
| [0018 · S03.1 V8 HostDocument 변경 묶음](0018-s03-v8-hostdocument-bridge.md) | V8 내부 JS 배열을 UTF-16 C ABI로 복사하고 세션별 HostDocument에 원자 변경을 적용해 BigInt 영수증을 반환 | 내부 구현 계약 `0.1.0` · Android/iOS 시뮬레이터 실제 V8 검증 완료 · 공개 DOM API 아님 |
| [0019 · S04 첫 CSS·레이아웃·GPU 연결 슬라이스](0019-s04-css-layout-gpu-slice.md) | 고정 Flex·CSS 배경색 fixture에서 플랫폼 중립 snapshot을 거쳐 Android/iOS wgpu 표면까지 잇는 내부 계약 | 계약 `0.1.0-draft` · S04.1~S04.6 시뮬레이터 검증 · S04.7 후속 작업 소유 경계 연결 완료, 기능 구현 미완료 |
| [0020 · S03.2 제한 DOM façade](0020-s03-dom-facade.md) | 전역 `document`의 생성·트리 변경·관계/텍스트/속성 조회를 Rust `HostDocument`에 동기 연결 | 내부 구현 계약 `0.1.0` · Bun/Rust 및 Android/iOS 시뮬레이터 실제 V8 검증 완료 · 공개 DOM 지원 아님 |
| [0021 · S03.3 DOM 노드·wrapper 수명](0021-s03-dom-node-lifecycle.md) | Rust 소유 노드 회수, V8 weak `Global` 자동 reset과 safe point 전체 scan, quota·shutdown·실패 원자성 | 내부 계약 `0.1.0-draft` · Android·iOS callback closure, 반복 자원 기준선, 제한된 종료 경합 검증 · 최대 scan·quota·listener/external root 검증 진행 중 |
| [0022 · R05 Android 프레임 지연 귀속·계측](0022-r05-benchmark-attribution.md) | Android View 대조군, 동일 MainActivity UI-only/runtime 쌍, queue·V8·actor 응답 전 계측, FrameTimeline·caller/owner 스케줄링 수집과 해석 경계 | 내부 계측 계약 `0.1.0-draft` · Android emulator 및 iOS Simulator 진단 · 실기기 성능 근거 아님 · R05 미완료 |
| [0023 · S05 입력 이벤트 전달](0023-s05-event-delivery.md) | FrameId·revision stamp, 표시 확인과 입력 순서, stale target, callback owner와 실패 처리 | 내부 계약 `0.1.0-draft` · 설계 산출물만 작성 · 런타임·public API 미구현 |
| [0024 · C04.3 Flex 정렬 전달](0024-c04-flex-alignment.md) | 별도 computed-style profile에서 제한 align-items·justify-content 값을 Taffy 입력으로 전달 | 내부 계약 `0.1.0` · 고정 Chromium fixture 통과 · 제품 runtime/API 미연결 |
| [0025 · C04.4 Cascade Layers](0025-c04-cascade-layers.md) | 새 Flex alignment profile에서 Stylo CSS layer ordering·declaration allowlist를 확인하고 Taffy에 전달 | 내부 계약 `0.1.0` · 고정 Chromium 16-case fixture 통과 · 제품 runtime/API 미연결 |
| [0013 · R15 청크 OTA 호환 모델](0013-r15-ota-chunk-compatibility.md) | 바이너리 호환 ID, 단일 target ESM specifier 매핑이 있는 기능·청크·자원 그래프, typed edge diff·영향 scope·최초/불확실 기준 fallback, 객체 차등 전달·전체 그래프 사전 확보·target별 stale/CAS publish 경계, 로컬 영속 저장·객체 재검증·용량 보호, 서명·압축 제한·상향 sequence rollback 및 offline authorization 미결정 | 내부 설계 제안 `0.1.0-draft` · 배포/로더/API 미구현 |
| [0014 · C02 번들러 모듈 그래프 adapter](0014-c02-bundler-module-graph.md) | 입력 resolver graph와 최종 emitted ESM graph를 분리해 기능·청크·specifier 대응 및 추출 실패를 기록 | 내부 실험 계약 `0.1.0-draft` · 구현·제품 API 미완료 |
| [0015 · C02 JavaScript·CSS 자원 그래프 결합](0015-c02-resource-graph-join.md) | 같은 production build에서 0011 CSS 자원과 0014 JavaScript feature·chunk 그래프를 결합 | 내부 구현 계약 `0.1.0-draft` · 고정 fixture 구현 완료 · 제품 API 아님 |
| [R13 · 플랫폼 생명주기·GPU 복구](r13-platform-gpu-recovery.md) | wgpu 실험 ABI, 플랫폼 표면 수명과 복구 경계 | 실험 전용 |

## 검증 기록

- [S04.10 · 플랫폼 presentation 신호 API·기기 조사](./evidence/s04-10-presentation-signal-audit-2026-10-07.md) — wgpu 30.0.1·Metal·Android/Vulkan API 경계, emulator와 Xclipse 940 실기기 capability 차이, 잠금 해제 후 실기기 화면 제출·surface 재생성·36개 오프스크린 표본 readback, 미실행 timing correlation 경계.
- [C01 · Chromium HTML UA 스타일 초기 비교](./evidence/css-c01-chromium-ua-2026-10-01.md) — macOS Chromium oracle와 고정 author baseline을 덮는 19개 computed value 비교 및 한계.
- [C01.2 · Chromium 단위·Flexbox·Grid 기준](./evidence/css-c01-layout-2026-10-02.md) — Chrome 154.0.8037.95의 `rem`·`em`·퍼센트·분수 Flexbox/Grid 기준값 41개와 CSS px 좌표 오차 계약. Spinon/Taffy 비교는 포함하지 않음.
- [C01.3 · Chromium CSSOM 속성 이름 표면](./evidence/css-c01-cssom-property-surface-2026-10-09.md) — Chrome 154.0.8037.98에서 HTML `div` 하나의 계산 스타일 속성 이름 478개를 관찰한 내부 기준. 전체 속성 지원 목록이나 제품 CSS 지원 판정은 아님.
- [C02 · Vite·Rspack CSS 산출 비교](./evidence/css-c02-bundler-2026-10-01.md) — production fixture의 CSS Modules·자원·청크, 기본 진단 차이와 공통 snapshot 원본 위치 근거.
- [C02.1 · Vite·Rspack CSS resolver 비교](./evidence/css-c02-resolver-2026-10-02.md) — fixture alias·package `exports`로 선택한 CSS 및 내부 `@import`의 production graph·snapshot 연결 근거.
- [C02.2 · 공통 모듈 그래프 계약 검증기](./evidence/css-c02-module-graph-contract-2026-10-02.md) — 0014 snapshot 검증·digest·최종 ESM AST parser와 통합 suite 결과.
- [C02.2 · Vite module graph adapter](./evidence/css-c02-vite-module-graph-2026-10-02.md) — Vite 8.3.1/Rolldown 1.2.12 fixture의 입력 graph·출력 ESM·resource digest 근거.
- [C02.2 · Rspack module graph adapter](./evidence/css-c02-rspack-module-graph-2026-10-02.md) — Rspack 2.2.7 fixture의 입력 graph·ESM profile 판정·출력 bytes 근거.
- [C02.3 · JavaScript·CSS 자원 그래프 결합](./evidence/css-c02-resource-graph-join-2026-10-02.md) — Vite·Rspack 고정 production fixture의 동시 capture·JS bytes 대조·feature resource closure와 원본 resource inventory.
- [C03 · HostDocument Stylo DOM adapter](./evidence/css-c03-stylo-dom-adapter-2026-10-01.md) — Stylo DOM/selector trait 구현, 고정 snapshot fixture, 실제 selector matcher 결과와 대상별 컴파일 결과.
- [C04 · stylesheet 입력 목록](./evidence/css-c04-stylesheet-registry-2026-10-01.md) — Stylo 출처·등록 순서·진단 보존과 처리기가 없는 `@import`의 경계 검증.
- [C04.1 · 기본 cascade slice](./evidence/css-c04-basic-cascade-2026-10-03.md) — fixed Chromium reference의 computed value 80개와 Stylo 내부 cascade 비교 및 제한.
- [C04.3 · Flex 정렬 전달](./evidence/css-c04-flex-alignment-2026-10-09.md) — 별도 computed-style profile의 16개 Chromium case·64개 frame 비교 및 미지원 경계.
- [C04.4 · Cascade Layers](./evidence/css-c04-cascade-layers-2026-10-09.md) — 16개 Chromium case·64개 frame에서 layer ordering과 오류 거부를 대조한 내부 profile 근거.
- [R15 · 청크 OTA 호환 모델](./evidence/r15-ota-chunk-model-2026-10-02.md) — 기존 OTA·C02 문서 사이의 그래프·기능 rollout·runtime 호환·서명/rollback 경계를 정리한 설계 추적 근거. 실행 기능 검증은 포함하지 않음.

- [R10 · Taffy 적합성 실험](./evidence/taffy-r10-2026-09-28.md) — 시뮬레이터·에뮬레이터 로그, 화면 캡처, fixture 범위와 해석 한계.
- [R03 · 공통 호스트 계약 검토](./evidence/r03-host-contract-review-2026-09-29.md) — 제안 계약을 기존 DOM·Rust 트리·렌더러 계획과 대조한 문서 검토 기록.
- [R06 · 스레드·소유권 소스 감사](./evidence/r06-thread-ownership-source-audit-2026-09-29.md) — V8·Rust FFI·트리·GPU 코드를 대조한 정적 조사. 경합을 실행 검증하지 않음.
- [R06 · V8 런타임 세션 실험](./evidence/r06-v8-runtime-thread-2026-09-30.md) — Android 16 에뮬레이터와 iOS 26.2 시뮬레이터의 취소·입력·세션 재생성 근거와 한계.
- [R06 · Chromium 참고 우선순위 큐](./evidence/r06-task-scheduler-2026-09-30.md) — strict-priority/FIFO 선택기, fake V8 순서 테스트, 분리 후 Rust 33개 테스트와 초기 Android·iOS 결과.
- [R06 · 실제 V8 우선순위 시뮬레이터 검증](./evidence/r06-priority-simulators-2026-09-30.md) — Android 16 에뮬레이터와 iPhone 17 Pro / iOS 26.2 시뮬레이터에서 혼합 여섯 작업의 실제 V8 선택 순서와 등급별 FIFO를 확인한 후속 기록.
- [R06 · Android 우선순위 유한 유입 검증](./evidence/r06-priority-fairness-android-2026-10-08.md) — Android 16 에뮬레이터에서 64개 대기 용량과 뒤늦은 높은 등급 입력 96개가 낮은 등급 작업을 지연시키는 유한 사례, 재현 명령·로그·캡처와 한계를 기록.
- [R06 · Android·iOS Simulator 유한 유입 검증](./evidence/r06-priority-fairness-simulators-2026-10-08.md) — Android 16 ARM64 에뮬레이터와 iPhone 17 Pro / iOS 26.2 Simulator의 실제 V8에서 같은 유한 높은 등급 유입을 각각 5회 실행한 순서·FIFO·owner thread·로그·화면과 한계를 기록.
- [R06 · 공정성 정책 후보 비교](./evidence/r06-priority-policy-comparison-2026-10-08.md) — strict-priority·aging·가중 순환 동일 입력 모델, Android·iOS Simulator 실제 V8의 1,087개 높은 등급 유입 반복, 로그·화면과 정책 결정 한계를 기록.
- [R06 · 실제 V8 큐 포화·복구 검증](./evidence/r06-queue-saturation-2026-10-08.md) — Android·iOS Simulator 각 5회에서 64개 수락·65번째 `-5` 즉시 반환·거부 side effect 부재·취소 뒤 FIFO drain·재접수를 확인한 로그, 화면, 환경과 checksum.
- `evidence/r06-priority-android-emulator-2026-09-30.log` · `evidence/spinon-r06-priority-android-2026-09-30.png` · `evidence/r06-priority-ios-simulator-2026-09-30.log` · `evidence/spinon-r06-priority-ios-simulator-2026-09-30.png` — 원본 로그와 화면 캡처.
- `evidence/r06-ios-simulator-post-split-2026-09-30.log` · `evidence/spinon-r06-ios-post-split-2026-09-30.png` — 저장 공간 확보 뒤 현재 런타임 분리 코드로 수행한 iOS 26.2 시뮬레이터 검증 원본 로그와 화면.
- `evidence/r06-android-queue-pressure-2026-09-30.log` — 무한 JavaScript 중 주입한 UI 탭, 접수된 이벤트, 플랫폼 대기열의 명시적 거부와 세션 종료 원본 로그(저장소 파일).
- [R06 · iOS·Android 진단 화면 일치](./evidence/s03-r06-cross-platform-ui-parity-2026-10-05.md) — 같은 앱 화면·버튼 상태·취소와 대기 이벤트 순서 및 여섯 검증 결과, Android 로그 UI 묶음 갱신·다섯 프레임 표본을 양쪽 시뮬레이터에서 확인한 기록과 캡처.
- [R05 · Android 프레임 지연 귀속 실험](./evidence/r05-android-frame-attribution-2026-10-06.md) — 고유 surface/display token 집계와 caller TID로 연결한 worker·actor·V8 구간. 고정 순서 matrix의 비교 한계, 균형 순서 50/100 dispatch 재측정, queue·caller wait·V8 handler 장시간 사례와 에뮬레이터 한계를 기록.
- [R05 · iOS·Android callback·프레임 원인 분석](./evidence/r05-cross-platform-callback-root-cause-2026-10-07.md) — Android sync barrier 개입, 동기·비동기 반복, runtime dispatch 없는 UI-only RenderThread frame miss, iOS System Trace 측정 실패와 미측정 항목·후속 계측을 기록.
- [R05 · Perfetto `gfx` 설정 오류 분석](./evidence/r05-perfetto-gfx-setup-errors-2026-10-07.md) — 설정 한 줄 최소차이 캡처로 37개 오류가 AVD에 없는 vendor GPU/display ftrace event임을 분류하고 generic trace 데이터와 계측 한계를 구분.
- [R05.3 · Android API·16KB 호환성 실행](./evidence/r05-android-api-compatibility-2026-10-08.md) — API 35·37.0·37.1 AVD fence 결과와 API fallback, WGPU adapter 미제공 경로, 수정 linker flags 적용 전 기존 APK의 GNU_RELRO 미정렬을 기록.
- [R05.3 · Android 16KB 재빌드 및 후속 구현 공격 검토](./evidence/r05-android-16kb-rebuild-2026-10-08.md) — 고정 V8 checkout·Android ARM64 재빌드, ELF/ZIP 정렬, API 37.2 16KB AVD 로더 결과와 checkout 설정 보호 검토를 기록. R05.3 전체는 미완료.
- [R05.3 · Android callback 실패 주입](./evidence/r05-android-callback-failure-injection-2026-10-08.md) — API 37.2 실제 Handler timeout·취소·100회 경합·executor 포화·60초 idle, API 36 GLES fallback, Release compile 및 별도 구현 실패 관점 검토를 기록. Android API 전체·iOS device callback·실기기·제품 표시 지연은 미완료.
- [R05.3 · Android callback API matrix](./evidence/r05-android-callback-api-matrix-2026-10-08/README.md) — API 35/4KB, 37.0/4KB, 37.1/16KB ARM64 emulator의 동일 APK fixture 실행, API 37.0 부팅 시도별 환경, 원본 log·screenshot 및 구현 후 실패 관점 검토를 기록. R05.3/R05 제품 완료 근거는 아님.
- [R13 · 플랫폼 생명주기·GPU 복구](./evidence/r13-platform-gpu-recovery-2026-09-29.md) — 회전·백그라운드·오류 주입·입력 복구의 로그와 화면 캡처.
- [S02 · Taffy 레이아웃 연결](./evidence/s02-taffy-layout-2026-09-30.md) — Chromium 기준 fixture, Taffy·기존 행/열 엔진의 좌표 비교와 한계.
- [S04.4 · Android GPU surface](./evidence/s04-android-gpu-surface-2026-10-03.md) — Android API 36 ARM64 emulator의 Vulkan llvmpipe surface 제출, 회전별 generation, sRGB 표본 readback과 화면 캡처. 실제 hardware GPU·실기기는 미검증.
- [S04.5 · iOS GPU surface](./evidence/s04-ios-gpu-surface-2026-10-03.md) — iPhone 17 Pro / iOS 26.2 simulator의 Metal surface 제출, sRGB 표본 readback과 화면 캡처. iOS 실기기·회전별 generation·성능은 미검증.
- [S04.6 · Android·iOS 교차 플랫폼 대조](./evidence/s04-cross-platform-comparison-2026-10-03.md) — simulator 캡처의 색상 경계를 CSS px로 환산해 Chromium geometry·StaticRenderSnapshot과 비교. 각 플랫폼 최대 좌표 오차 0.167 CSS px; 전체 화면 동등성과 실기기 GPU는 미검증.
- [S04.9 · 정적 snapshot hit-test 사전 비교](./evidence/s04-hit-test-precomparison-2026-10-07.md) — Chromium `elementFromPoint()` 대상, half-open 경계, paint order, surface 좌표 역변환과 generation/frame 거부 조건.
- [S04.9 · Android·iOS 터치 실행](./evidence/s04-hit-test-platforms-2026-10-07.md) — emulator·simulator 터치가 고정 snapshot NodeId로 변환되는 로그와 캡처, DOM event·displayed-frame 한계.
- [S04 iOS feature-off 확인](./evidence/s04-ios-fixture-disabled-2026-10-03.log) — 기본 iOS 시뮬레이터 빌드에서 fixture 인자에 비활성 안내를 반환한 원본 로그.
- [S04 Android feature-off 확인](./evidence/s04-android-fixture-disabled-2026-10-03.log) — 기본 APK에서 fixture 전용 JNI 경로가 비활성 안내를 반환한 원본 Logcat.
- [S02.1 · HostDocument 레이아웃 입력](./evidence/s02-host-document-layout-input-2026-10-03.md) — 기존 Tree 입력과 HostDocument 요소 투영의 ID·순서·revision·Taffy 출력 비교와 실행 결과.
- [S02.2 · 레이아웃 revision 사전 고정 비교 기준](./evidence/s02-layout-revision-precomparison-2026-10-04.md) — source·style·environment 입력 변화와 stale snapshot admission에 대한 구현 전 기대 결과.
- [S02.2 · 레이아웃 revision gate 실행 근거](./evidence/s02-layout-revision-gate-2026-10-04.md) — 고정 S04 revision fixture, layout echo와 snapshot admission 검증 및 제품 runtime 한계.
- [S03.1 · V8 HostDocument 변경 묶음](./evidence/s03-v8-hostdocument-bridge-2026-10-03.md) — Rust 직접 기준 실행, 30개 적대 검증 관점, Android 16 에뮬레이터와 iOS 26.2 시뮬레이터 실제 V8 빌드·실행 결과.
- [S03.2 · 제한 DOM façade 구현 전 비교 기준](./evidence/s03-dom-facade-precomparison-2026-10-04.md) — WHATWG·Chromium 비교 사례, 앱 문서 루트 차이, wrapper·저장 한도와 미구현 경계.
- [S03.2 · 제한 DOM façade 실제 V8 실행](./evidence/s03-dom-facade-runtime-2026-10-04.md) — Android API 36 ARM64 및 iOS 26.2 시뮬레이터 빌드·실행 로그, fixture·언어 도구 검증과 미확인 경계.
- [S03.3 · 노드 수명 계약 Rust fixture](./evidence/s03-node-lifecycle-contract-fixture-2026-10-04.md) — 고정 root graph·weak handle scan·wrapper 재생성·quota·실패 atomicity reference model 검증. 제품 HostDocument 회수와 실제 V8 GC는 포함하지 않음.
- [S03.3 · HostDocument 회수 구현 전 비교 기준](./evidence/s03-hostdocument-collector-precomparison-2026-10-04.md) — 고정 lifecycle tree fixture에서 Rust 회수 전 동작과 pass/fail 기대값을 고정.
- [S03.3 · Rust HostDocument 회수 경로](./evidence/s03-hostdocument-collector-2026-10-04.md) — 실제 Rust core/bridge collector를 고정 tree fixture에 적용한 결과, ID·문자열 계수와 revision 검증. V8 weak handle 자동 scan 및 모바일 runtime 연결은 포함하지 않음.
- [S03.3 · V8 weak wrapper Android·iOS 실행](./evidence/s03-v8-weak-wrapper-2026-10-04.md) — Android 16/API 36 ARM64 에뮬레이터와 iPhone 17 Pro/iOS 26.2 시뮬레이터에서 실제 GC·safe-point scan·Rust sweep·wrapper 재생성을 확인한 고정 fixture 및 한계.
- [S03.3 · 반복 lifecycle Android·iOS 실행](./evidence/s03-repeat-lifecycle-2026-10-05.md) — native strong callback closure root 보존·해제와 6×32 element/text 반복 뒤 node·UTF-16 문자열·weak wrapper 기준선 복귀를 확인한 시뮬레이터 근거 및 한계.
- [S03.3 · 세션 종료 경합 Android·iOS 실행](./evidence/s03-shutdown-2026-10-05.md) — 활성 평가 취소, 이미 접수한 명령 거부, 닫힌 세션의 후속 호출 거부와 작업자 회수를 확인한 제한 probe.
- [S03.3 · 대량 wrapper scan 반복 관측](./evidence/s03-dynamic-registry-repeat-measurements-2026-10-05.md) — Android·iOS에서 프로세스 재시작 7회와 같은 Isolate 재scan 10회 wall-time 분포를 기록했다. 표본 변동이 커 비용 판정은 미완료다.
- `evidence/s03-shutdown-android-2026-10-05.log` · `evidence/s03-shutdown-android-2026-10-05.png` · `evidence/s03-shutdown-ios-2026-10-05.log` · `evidence/s03-shutdown-ios-2026-10-05.png` — 실제 V8 결과 로그와 화면.
- `evidence/s03-v8-weak-wrapper-android-2026-10-04.log` · `evidence/s03-v8-weak-wrapper-ios-2026-10-04.log` · `evidence/s03-v8-weak-wrapper-android-2026-10-04.png` · `evidence/s03-v8-weak-wrapper-ios-2026-10-04.png` — 원본 결과와 검증 화면.
- `evidence/s03-v8-hostdocument-bridge-android-2026-10-03.log` · `evidence/s03-v8-hostdocument-bridge-ios-2026-10-03.log` — 해당 시뮬레이터 실행의 원본 부팅 결과 로그.
- `evidence/s02-basic-flex-chrome-2026-09-30.png` — 같은 fixture의 Headless Chrome 캡처.
