# 내부 인터페이스 명세

## R06 큐 포화·복구

- [구현 전 비교 모델](./evidence/r06-queue-saturation-precomparison-2026-10-08.md) · [작업 계획·구현 점검](../../plan/r06-queue-saturation.md) · [실제 V8 실행 근거](./evidence/r06-queue-saturation-2026-10-08.md)

이 폴더에는 앱 작성자에게 공개하지 않는 Rust·C++·플랫폼 사이 호출 계약을 둡니다. 내부 ABI라도 호출자, 입력·출력, 소유권, 오류와 현재 한계를 기록합니다. 이 문서는 제품 API 지원 완료로 연결하지 않으며 상태 표시는 [공식 대장](../STATUS.md)을 따릅니다.

첫 공식 릴리스 전 내부 인터페이스 계약의 숫자 버전은 `0.1.0`으로 유지합니다. 구현·검증·문서 개정은 숫자 버전을 올리는 사유가 아닙니다. `-draft`는 성숙도 상태를 나타내며, 출시·호환성 버전은 사용자가 별도로 확정한 정책을 따릅니다. 의존성·도구의 버전이나 OTA 그래프 형식 버전과 혼동하지 않습니다.

| 문서 ID | 범위 | 상태 |
| --- | --- | --- |
| [0001 · V8 부팅 실험](0001-v8-bootstrap.md) | Bun 번들, Rust FFI, V8 C++ 어댑터와 Android/iOS 빌드 smoke | 실험 전용 |
| [0002 · Rust 트리 코어](0002-rust-tree-core.md) | 노드 ID, 트리 구조, 원자적 변경 묶음과 revision | 실험 전용 |
| [0003 · 공통 문서·호스트 계약](0003-shared-host-contract.md) | DOM 호환 계층과 프레임워크 어댑터의 문서 모델, 소유권, 동기 변경과 이벤트 경계 | 코어·제한 façade 일부 내부 구현 · 나머지 제안 초안 |
| [0004 · R06 스레드·소유권 위험 분석](0004-thread-ownership-risks.md) | Isolate·문서·콜백·revision·비동기 완료·종료 경계의 위험과 검증 후보 | 검토 초안 · R06 미완료 |
| [0005 · V8 런타임 세션 실험](0005-v8-runtime-session.md) | 세션별 Isolate 소유 스레드, 용량 제한 우선순위 큐, 취소·종료 경계 | 실험 전용 · R06 미완료 · 우선순위와 제한된 종료 경합의 실제 V8 시뮬레이터 검증 통과 |
| [0006 · JavaScript 작업 스케줄러](0006-js-task-scheduler.md) | Chromium 참고 우선순위 선택과 앱 작업 출처·프레임·취소 경계 | Android·iOS Simulator 실제 V8에서 1,087개 유입 및 큐 포화·거부·복구를 각 5회 통과 · 무한 유입 보장과 제품 역압력 정책은 미결정 |
| [0007 · 내장 UA stylesheet 자원](0007-ua-stylesheet-resource.md) | 지원 HTML 기본 CSS 자원과 읽기 전용 FFI 인터페이스 | 내부 초안 · C04.5 제한 computed-style API 연결됨 · 제품 runtime은 미연결 |
| [0008 · C02 Vite·Rspack CSS 비교 모델](0008-css-bundler-c02.md) | CSS 산출·자원·청크·오류 위치·resolver 비교 조건 | 내부 실험 계약 · C02 미완료 |
| [0009 · 레이아웃 엔진](0009-layout-engine.md) | S01 Tree 및 HostDocument 요소 snapshot, source·style·environment revision stamp, Taffy 프레임과 fixture stale-admission 경계 | 구현 초안 `0.1.0` · 제한된 Flex subset · 제품 revision 관리자와 GPU queue stale 검사는 미구현 |
| [0010 · C03 Stylo DOM adapter](0010-stylo-dom-adapter-c03.md) | HostDocument snapshot에서 Stylo 문서·노드·요소·선택자 DOM으로의 변환 계약 | 내부 구현 계약 초안 `0.1.0-draft` · C03 구현 완료, 계산 스타일 제외 |
| [0011 · C02 CSS 자원 어댑터](0011-css-resource-adapter-c02.md) | Vite·Rspack 산출을 빌드 단위 공통 CSS 자원 snapshot으로 정규화 | 내부 계약 후보 `0.1.0-draft` · fixture 스파이크 전용 · 제품 API 아님 |
| [0012 · C04 stylesheet 입력 목록](0012-stylesheet-registry-c04.md) | Stylo stylesheet 파싱, CSS 출처·등록 순서와 parser 진단 보존 | 내부 구현 계약 초안 `0.1.0-draft` · cascade 계산 미연결 |
| [0013 · R15 청크 OTA 호환 모델](0013-r15-ota-chunk-compatibility.md) | 바이너리 호환 ID, 단일 target ESM specifier 매핑이 있는 기능·청크·자원 그래프, typed edge diff·영향 scope·최초/불확실 기준 fallback, 객체 차등 전달·전체 그래프 사전 확보·target별 stale/CAS publish 경계, 로컬 영속 저장·객체 재검증·용량 보호, 서명·압축 제한·상향 sequence rollback 및 offline authorization 미결정 | 내부 설계 제안 `0.1.0-draft` · 배포/로더/API 미구현 |
| [0014 · C02 번들러 모듈 그래프 adapter](0014-c02-bundler-module-graph.md) | 입력 resolver graph와 최종 emitted ESM graph를 분리해 기능·청크·specifier 대응 및 추출 실패를 기록 | 내부 실험 계약 `0.1.0-draft` · 구현·제품 API 미완료 |
| [0015 · C02 JavaScript·CSS 자원 그래프 결합](0015-c02-resource-graph-join.md) | 같은 production build에서 0011 CSS 자원과 0014 JavaScript feature·chunk graph를 결합 | 내부 구현 계약 `0.1.0-draft` · 고정 fixture 구현 완료 · 제품 API 아님 |
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
| [0026 · C04.5 지원 HTML UA 계산 스냅샷](0026-c04-ua-baseline-snapshot.md) | 내장 UA CSS를 Stylo UA origin으로 계산하고 지원 요소 7개 property의 제한 snapshot을 반환 | 내부 Rust API `0.1.0-draft` · C01 Chromium 19개 값 비교 · runtime/layout/GPU 미연결 |
| [0027 · C04.6 Flex margin → Taffy 입력](0027-c04-flex-margin-layout.md) | 별도 computed-style profile의 physical/logical margin 값을 제한 Taffy Flex 입력으로 전달 | 내부 Rust API `0.1.0` · 고정 Chromium fixture 통과 · 제품 runtime/API 미연결 |
| [0028 · C04.7 CSS media 환경 입력](0028-c04-media-environment.md) | viewport와 revision에 묶은 scheme·primary/all pointer 입력 및 제한 `@media` cascade | 내부 Rust API `0.1.0` · Chromium 고정 4 case 비교 · OS/runtime 연결 미구현 |
| [0029 · C04.8 Runtime UA cascade 재계산](0029-c04-runtime-ua-cascade.md) | HostDocument 전체 revision snapshot·기존 inline `style` 속성·명시 viewport/media 환경에서 세션 초기화 중 준비한 CSS worker가 내장 UA cascade와 diagnostics를 revision JSON으로 반환 | 내부 runtime 계약 초안 `0.1.0-draft` · Android·iOS Simulator 실제 V8 실행 확인 · 제품 layout/GPU 연결 미완료 |
| [0030 · C04.9 Runtime CSS→Taffy layout snapshot](0030-c04-runtime-css-to-taffy.md) | 같은 RuntimeSession HostDocument revision에서 UA cascade와 제한 inline CSS profile을 계산하고 Taffy CSS px frame을 비동기 snapshot으로 복사 | 미출시 내부 계약 0.1.0 고정 · Android·iOS Simulator 실제 V8 실행 확인 · GPU scene/제품 CSS 미완료 |
| [0031 · C04.10 Runtime CSS→WGPU 장면](0031-c04-runtime-css-to-gpu.md) | 같은 HostDocument revision의 CSS/Taffy 계산을 불변 paint scene과 Android·iOS WGPU surface로 연결하는 내부 C ABI·수명 경계 | 미출시 내부 계약 0.1.0 고정 · 구현·시뮬레이터 검증 및 PR 변경 검토 완료 · 공개 API 아님 |
| [0032 · C05.1 Runtime inline 사용자 지정 속성](0032-c05-runtime-custom-properties.md) | runtime inline `--*`·`var()` cascade, 제한 Flex layout·paint 입력, revision별 전체 재계산 | 미출시 내부 계약 `0.1.0` 고정 · Rust, Android·iOS Simulator 검증 완료 · 공개 API 아님 |
| [0033 · C04.11 런타임 문서 author stylesheet](0033-c04-runtime-author-stylesheets.md) | 연결된 HTML `<style>` source 수집, Stylo origin/source order, C05.1 runtime custom properties·WGPU cascade 및 숨긴 style text 처리 | 미출시 내부 계약 `0.1.0` 고정 · Chromium 비교, Rust workspace, Android API 37·iOS 26.2 Simulator 검증 완료 · 공개 CSS 지원 아님 |
| [0034 · C05.2 Runtime CSS `@property` 등록](0034-c05-runtime-registered-properties.md) | 연결 HTML `<style>`의 registered custom properties, Stylo profile 경계, revision별 수명과 제한 layout·paint 연결 | 미출시 내부 계약 `0.1.0` 고정 · Android API 37/iOS 26.2 Simulator 검증 완료 · 공개 CSS 지원 아님 |
| [0035 · C05.3 연결 장면 불변 결과 재사용](0035-c05-runtime-result-cache.md) | worker-local 단일 계산 결과 cache, DocumentRevision 재발행, viewport/media 무효화 및 immutable payload 공유 | 미출시 내부 계약 `0.1.0` 고정 · Android API 37/iOS 26.2 Simulator, Rust release 반복 측정 완료 · 공개 API 아님 |
| [0036 · C05.4 runtime 하위 트리 cascade 재계산](0036-c05-runtime-incremental-restyle.md) | author stylesheet가 없는 단일 연결 root에서 namespace 없는 HTML inline `style` 변경만 dirty subtree Stylo cascade로 처리하고 나머지 직렬화 출력 재사용 | 미출시 내부 계약 `0.1.0` 고정 · Android API 37 emulator/iOS 26.2 Simulator, Chromium fixture 및 Rust release worker 비교 · 공개 API 아님 |
| [0037 · C06.1 백분율 크기](0037-c06-percentage-dimensions.md) | Stylo typed `width`·`height`·`flex-basis` percentage를 Taffy layout 입력까지 보존 | 내부 계약 숫자 버전 `0.1.0` 고정 · 별도 계획·Chromium/Rust fixture·Android/iOS Simulator 근거 · C06.1만 완료 |
| [0038 · C06.2 백분율 margin·padding·gap](0038-c06-spacing-percentages.md) | definite basis의 typed spacing percentage, Taffy 전달, cyclic/indefinite basis 오류 계약 | 내부 계약 숫자 버전 `0.1.0` 고정 · 현재 브랜치 검증 완료·미병합 · 공개 API 아님 |
| [0039 · C06.3 절대 길이 단위](0039-c06-absolute-lengths.md) | Stylo typed 절대 길이의 CSS px 정규화와 runtime 크기·spacing 입력 | 내부 계약 숫자 버전 `0.1.0` 고정 · 현재 브랜치 구현·검증 완료·미병합 · 공개 CSS 지원 아님 |
| [0040 · C06.4 글꼴 상대 길이 단위](0040-c06-font-relative-units.md) | 합성 HTML 문서 루트의 em/rem·font-size cascade, typed CSS px layout 전달, metric unit fail-closed 경계 | 내부 계약 숫자 버전 `0.1.0` 고정 · 현재 작업 브랜치 구현·Chromium/Rust 비교·Android/iOS Simulator 검증 완료·미병합 · 공개 API 아님 |
| [0041 · C06.5 Typed CSS math](0041-c06-typed-css-math.md) | 제한된 typed `calc()`·`min()`·`max()`·`clamp()` AST, Taffy resolver, 속성별 final-value censor와 failure-atomic layout | 내부 계약 숫자 버전 `0.1.0` 고정 · 현재 작업 브랜치 구현·검증 완료·미병합 · 공개 API 아님 |
| [0042 · C06.6a 네이티브 viewport 길이 단위](0042-c06-viewport-units.md) | `vw`/`vh`·`vi`/`vb`·`vmin`/`vmax`와 small/large/dynamic viewport 변형의 cascade·layout·resize 경계 | 내부 계약 숫자 버전 `0.1.0` 고정 · 현재 작업 브랜치 구현·Chromium/Rust·Android/iOS Simulator 검증 완료·미병합 · 제품 API 아님 |
| [0043 · C07.1 물리 축 최소·최대 크기](0043-c07-1-min-max-sizing.md) | `min-width`·`max-width`·`min-height`·`max-height`의 typed 값, box-sizing·percentage·CSS math·Flex clamp 및 fail-closed 경계 | 내부 계약 숫자 버전 `0.1.0` 고정 · Chromium/Rust 35-node 비교, Android API 37·iOS 26.2 Simulator runtime 확인 완료 · 공개 API 아님 |
| [0044 · C07.2 테두리 폭 레이아웃](0044-c07-2-border-width-layout.md) | 물리 네 면 border width·style gate, shorthand/cascade, box-sizing·min/max·flex 기하 및 오류 경계 | PR #103 리베이스 병합 · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0045 · C07.3 종횡비](0045-c07-3-aspect-ratio.md) | 제한 runtime Block/Flex의 typed aspect-ratio, Taffy 전달, definite-size leaf 보정, min/max fail-closed 경계 | [PR #104 리베이스 병합](https://github.com/ohah/spinon/pull/104) · Chrome 35-node 기준 중 30 supported geometry 비교 · Android API 37/iOS 26.2 Simulator V8→WGPU 화면·로그 확인 · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0046 · C08 Block·기본 페인트](0046-c08-block-paint.md) | root/child Block 흐름, display:none 제거, 제한 background paint, foreground/font computed style 경계 | 구현·Rust/Chromium/Android API 37·iOS 26.2 Simulator 검증 완료 · [PR #105](https://github.com/ohah/spinon/pull/105) 리베이스 병합 · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0047 · C09 Block formatting](0047-c09-block-formatting.md) | normal Block 흐름·containing block, signed margin collapse, `flow-root` BFC 경계와 intrinsic-width 의존 shrink-to-fit 계약 | C09.1 [PR #107 리베이스 병합](https://github.com/ohah/spinon/pull/107): Chrome 기준 10개/30개 node, DPR 1·2, Android/iOS Simulator 실행 · C09.2 [PR #109 리베이스 병합](https://github.com/ohah/spinon/pull/109): Chrome 기준 16개/57개 node, DPR 1·2, Android API 37 emulator/iOS 26.2 Simulator V8→WGPU 실행 · C09.3 [PR #111 리베이스 병합](https://github.com/ohah/spinon/pull/111): Chrome 기준 4개/16개 node, DPR 1·2, Android API 37 emulator/iOS 26.2 Simulator V8→WGPU 실행 · C09.4 미구현 · [계획 검토](./evidence/c09-block-formatting-plan-review-2026-10-10.md) · [fixture 통합 재검토](./evidence/c09-block-formatting-plan-review-precomparison-followup-2026-10-10.md) · [사전 비교](./evidence/c09-block-formatting-precomparison-2026-10-10.md) · [C09.1 검토](./evidence/c09-block-formatting-implementation-review-2026-10-10.md) · [C09.2 검토·실행](./evidence/c09-2-margin-collapse-implementation-review-2026-10-10.md) · [C09.3 검토·실행](./evidence/c09-3-flow-root-implementation-review-2026-10-10.md) · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0048 · C10.1 Flex 줄바꿈](0048-c10-flex-wrap.md) | 여섯 runtime Flex profile의 `nowrap|wrap` typed 값, Stylo cascade·incremental 재사용, Taffy line/gap 기하, Block/static fail-closed 경계 | PR #114 리베이스 병합 완료 · Chromium 12 case/44 node, DPR 1·2 · Android API 37 emulator/iOS 26.2 Simulator 실제 V8 frame 7/7 비교·WGPU 제출 · [구현 실패 관점·실행 근거](./evidence/c10-1-flex-wrap-implementation-review-2026-10-10.md) · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0049 · C10.2 Flex 크기 배분](0049-c10-flex-distribution.md) | definite main-size line의 basis·grow·scaled shrink, explicit min/max freeze·재분배와 Android/iOS runtime report 경계 | [PR #116 리베이스 병합](https://github.com/ohah/spinon/pull/116) · Chromium 27 case/92 node, DPR 1·2 · Android API 37 emulator/iOS 26.2 Simulator 실제 V8 frame 5/5 대조·WGPU 제출 · [구현 검토](./evidence/c10-2-flex-distribution-implementation-review-2026-10-10.md) · [병합 후 문서 동기화 검토](./evidence/c10-2-postmerge-doc-sync-review-2026-10-10.md) · 내부 계약 숫자 버전 `0.1.0` 고정 · 공개 API 아님 |
| [0050 · C10.3.1 Flex 축 역방향·줄 역방향](0050-c10-3-1-flex-reverse.md) | runtime Flex의 `row-reverse`, `column-reverse`, `wrap-reverse`, `flex-flow`; reverse+RTL·legacy profile fail-closed | 구현 브랜치 검증 완료·PR 미병합 · Chromium 17 case/64 node, DPR 1·2 · Android API 37 emulator/iOS 26.2 Simulator 실제 V8 frame 7/7 대조·WGPU 제출 · [구현 실패 관점 검토](./evidence/c10-3-1-flex-reverse-implementation-review-2026-10-10.md) · 내부 계약 숫자 버전 `0.1.0` 고정 · 전체 Flexbox 완료 아님 |
| [R13 · 플랫폼 생명주기·GPU 복구](r13-platform-gpu-recovery.md) | wgpu 실험 ABI, 플랫폼 표면 수명과 복구 경계 | 실험 전용 |

C04.10 검토 기록: [PR 변경 검토](evidence/c04-runtime-css-to-gpu-pr-review-2026-10-10.md) · [계획 검토](evidence/c04-runtime-css-to-gpu-plan-review-revised-2026-10-09.md) · [UIKit surface 스레드 분리 계획 재검토](evidence/c04-runtime-css-to-gpu-plan-surface-thread-review-2026-10-09.md) · [resize 계획 재검토](evidence/c04-runtime-css-to-gpu-resize-plan-review-revised-2026-10-09.md) · [대기열·종료·실패 보완 계획 검토](evidence/c04-runtime-css-to-gpu-queue-plan-review-2026-10-09.md) · [구현 전 Chromium 비교 기준](evidence/c04-runtime-css-to-gpu-precomparison-2026-10-09.md) · [resize 사전 기준](evidence/c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md).

C05.1 검토와 실행 기록: [구현 전 계획 검토](evidence/c05-runtime-custom-properties-plan-review-2026-10-10.md) · [구현 후 실패 관점 검토와 시뮬레이터 근거](evidence/c05-runtime-custom-properties-2026-10-10.md).

C04.11 계획·구현 검토와 실행 기록: [계획 20개 실패 관점 검토](evidence/c04-runtime-author-stylesheets-plan-review-2026-10-10.md) · [구현 전 Chromium 비교 기준](evidence/c04-runtime-author-stylesheets-precomparison-2026-10-10.md) · [구현 20개 실패 관점 검토와 Simulator 근거](evidence/c04-runtime-author-stylesheets-2026-10-10.md).

C06.3 계획·구현 검토와 실행 기록: [계획 실패 경로 20개](evidence/c06-3-absolute-lengths-plan-review-2026-10-10.md) · [구현 실패 경로 20개와 Android/iOS Simulator 근거](evidence/c06-absolute-lengths-implementation-2026-10-10.md).

C05.2 계획·구현 검토와 실행 기록: [계획 실패 관점 검토](evidence/c05-runtime-registered-properties-plan-review-2026-10-10.md) · [구현 전 Chromium 기준](evidence/c05-runtime-registered-properties-precomparison-2026-10-10.md) · [구현 실패 관점 검토와 Android·iOS Simulator 근거](evidence/c05-runtime-registered-properties-implementation-review-2026-10-10.md).

C05.3 계획·구현 검토와 실행 기록: [계획 실패 관점 검토](evidence/c05-runtime-result-cache-plan-review-2026-10-10.md) · [구현 전 Chromium 기준](evidence/c05-runtime-result-cache-precomparison-2026-10-10.md) · [구현 실패 관점 검토·release 측정·Android/iOS Simulator 근거](evidence/c05-runtime-result-cache-implementation-review-2026-10-10.md).

C05.4 계획·구현 검토와 실행 기록: [계획 실패 관점 검토](evidence/c05-runtime-incremental-restyle-plan-review-2026-10-10.md) · [구현 전 Chromium 기준](evidence/c05-runtime-incremental-restyle-precomparison-2026-10-10.md) · [구현 실패 관점 검토·release 측정·Android/iOS Simulator 근거](evidence/c05-runtime-incremental-restyle-implementation-review-2026-10-10.md).

C06 계획·구현 검토와 실행 기록: [전체 C06 계획 공격 검토](evidence/c06-value-unit-conversion-plan-review-2026-10-10.md) · [C06.2 계획 공격 검토](evidence/c06-2-spacing-plan-review-2026-10-10.md) · [C06.2 구현 공격 검토·Chromium 수치 비교·Android/iOS Simulator 실행](evidence/c06-spacing-percentages-implementation-2026-10-10.md) · [C06.4 Chromium 사전 비교](evidence/c06-4-font-relative-units-precomparison-2026-10-10.md) · [C06.4 계획 공격 검토](evidence/c06-4-font-relative-units-plan-review-2026-10-10.md) · [C06.4 구현 실패 경로 검토·Rust workspace·Android/iOS Simulator 실행](evidence/c06-font-relative-units-implementation-review-2026-10-10.md) · [C06.5 typed CSS math 계획 검토](evidence/c06-5-typed-css-math-plan-review-2026-10-10.md) · [C06.5 Chromium 사전 비교·Stylo DTO 근거](evidence/c06-5-typed-css-math-precomparison-2026-10-10.md) · [C06.5 구현 실패 경로·전체 Rust workspace·Android/iOS Simulator 실행](evidence/c06-typed-css-math-implementation-review-2026-10-10.md) · [C06.6 viewport 단위 계획 실패 경로 검토](evidence/c06-6-viewport-units-plan-review-2026-10-10.md) · [C06.6 viewport 단위 구현 실패 경로·Android/iOS Simulator 실행](evidence/c06-viewport-units-implementation-review-2026-10-10.md).

C06.1~C06.6a 및 C07.1 누적 변경의 ABI·캐시·CSS 의미·Android/iOS runtime 경계를 교차 점검한 [통합 적대적 검토와 현재 브랜치 검증 결과](evidence/c06-c071-integrated-review-2026-10-10.md).

## 검증 기록

- [C09.1 · Block formatting 구현 변경 검토와 Simulator 근거](./evidence/c09-block-formatting-implementation-review-2026-10-10.md) — `display:grid`의 Stylo fallback 우회 수정, 20개 실패 관점, 고정 Chromium 10개 case·30개 node 비교, Android API 37·iOS 26.2 Simulator 실행과 화면.
- [C09.2 · margin collapse 구현 변경 검토와 Simulator 근거](./evidence/c09-2-margin-collapse-implementation-review-2026-10-10.md) — signed strut·부모/자식·빈 상자·used border 조건의 20개 실패 관점, 고정 Chromium 16개 case·57개 node 비교, Android API 37·iOS 26.2 Simulator 실행과 화면.
- [C09.3 · flow-root 구현 변경 검토와 Simulator 근거](./evidence/c09-3-flow-root-implementation-review-2026-10-10.md) — BFC 내부/외부 margin 경계·profile 거부 경로 등 20개 실패 관점, 고정 Chromium 4개 case·16개 node 비교, Android API 37·iOS 26.2 Simulator 실행과 화면.
- [C09.3 · PR #111 병합 뒤 상태·명세 동기화 검토](./evidence/c09-3-merge-status-doc-review-2026-10-10.md) — C09.3 완료와 C09/C09.4 미완료 경계, Chromium·DPR·계약 버전·플랫폼 근거·링크를 대조했다.
- [C09.2 · PR #109 병합 상태 문서 정합성 검토](./evidence/c09-2-merge-status-doc-review-2026-10-10.md) — 병합 뒤 상태 대장·계약·계획·인덱스의 상태와 범위를 대조했다.
- [C09.1 · 병합 후 상태·명세 동기화 검토](./evidence/c09-block-formatting-post-merge-doc-sync-review-2026-10-10.md) — PR #107 병합 뒤 상태 대장, 내부 계약, 계획, 구현 인덱스와 실행 근거를 대조.

- [C04.9 · Runtime CSS→Taffy 계획 적대 검토](./evidence/c04-runtime-style-layout-plan-review-2026-10-09.md) — 구현 계획의 불변 조건·미지원 경계와 보정 뒤 계획문서 hash를 대조.
- [C04.9 · Runtime CSS→Taffy 시뮬레이터 실행](./evidence/c04-runtime-css-taffy-simulators-2026-10-09.md) — 고정 Chromium oracle과 Android·iOS Simulator의 실제 V8 layout JSON·프레임 비교 및 검증 경계.
- [C04.9 · Runtime CSS→Taffy 구현 후 검토](./evidence/c04-runtime-css-taffy-implementation-review-2026-10-09.md) — C ABI·CSS recovery·revision·root/frame·worker 및 플랫폼 실패 관점을 확인하고 수정한 동작을 기록.
- [C04.9 · PR 변경 검토](./evidence/c04-runtime-css-taffy-pr-review-2026-10-09.md) — 구현·명세·상태 대장·실행 근거·버전 고정을 함께 대조한 merge 전 확인.
- [C04.10 · Runtime CSS→WGPU 수정 계획 검토](./evidence/c04-runtime-css-to-gpu-plan-review-revised-2026-10-09.md) — canonical V8 JavaScript 입력, Android `SurfaceView` render executor, iOS surface generation 및 종료·stale 제출 조건을 포함한 수정 계획을 재검토.
- [C04.10 · 현재 계획 적대 검토](./evidence/c04-runtime-css-to-gpu-plan-final-review-2026-10-09.md) — UIKit resize queue 경계와 Android `surfaceDestroyed` drain 조건을 포함한 현재 계획 hash를 20개 실패 관점으로 대조.
- [C04.10 · Android surface 수명주기 실행](./evidence/c04-runtime-css-to-gpu-android-lifecycle-2026-10-09.md) — API 37 emulator 회전 중 surface destroy/drain/recreate 순서와 화면 복구.
- [C04.10 · Android·iOS resize 시뮬레이터 실행](./evidence/c04-runtime-css-to-gpu-resize-simulators-2026-10-09.md) — Chromium의 두 viewport, Android native window·acquired texture·renderer recreation, iOS point·drawable·Metal resize와 실제 화면 범위를 대조.
- [C04.10 · 대기열·종료·draw 실패 시뮬레이터 실행](./evidence/c04-runtime-css-to-gpu-queue-simulators-2026-10-09.md) — Java·Swift latest-only lane의 단일·동시 생산자 한도, Android·iOS pending draw 종료 및 내부 draw 오류 뒤 같은 장면 복구 근거.
- [C04.10 · PR 변경 적대 검토](./evidence/c04-runtime-css-to-gpu-pr-review-2026-10-10.md) — 제출 커밋의 계약·플랫폼 통합·fixture·증거 범위 20개 관점을 대조.
- [C04.10 · 최종 구현 적대 검토](./evidence/c04-runtime-css-to-gpu-final-implementation-review-2026-10-09.md) — 버전·FFI·revision·publish 경합·surface lifecycle·queue 종료 경로와 전체 테스트·플랫폼 build 결과.
- [C04.10 · 초기 구현 적대 검토](./evidence/c04-runtime-css-to-gpu-implementation-review-2026-10-09.md) — resize 왕복 실행 전 장면·revision·WGPU·C ABI·플랫폼 수명 검토 기록.
- [C04.10 · resize 왕복 구현 검토](./evidence/c04-runtime-css-to-gpu-resize-implementation-review-2026-10-09.md) — resize 세대 경합·오류·화면 크기 불일치와 대기열 한계를 실제 simulator 로그·캡처에 대조.

- [S04.10 · 플랫폼 presentation 신호 API·기기 조사](./evidence/s04-10-presentation-signal-audit-2026-10-07.md) — wgpu 30.0.1·Metal·Android/Vulkan API 경계, emulator와 Xclipse 940 실기기 capability 차이, 잠금 해제 후 실기기 화면 제출·surface 재생성·36개 오프스크린 표본 readback, 미실행 timing correlation 경계.
- [C01 · Chromium HTML UA 스타일 초기 비교](./evidence/css-c01-chromium-ua-2026-10-01.md) — macOS Chromium oracle와 고정 author baseline을 덮는 19개 computed value 비교 및 한계.
- [C01.2 · Chromium 단위·Flexbox·Grid 기준](./evidence/css-c01-layout-2026-10-02.md) — Chrome 154.0.8037.95의 `rem`·`em`·퍼센트·분수 Flexbox/Grid 기준값 41개와 CSS px 좌표 오차 계약. Spinon/Taffy 비교는 포함하지 않음.
- [C01.3 · Chromium CSSOM 속성 이름 표면](./evidence/css-c01-cssom-property-surface-2026-10-09.md) — Chrome 154.0.8037.98에서 HTML `div` 하나의 계산 스타일 속성 이름 478개를 관찰한 내부 기준. 전체 속성 지원 목록이나 제품 CSS 지원 판정은 아님.
- [C06.2 · 백분율 spacing 계획 공격 검토](./evidence/c06-2-spacing-plan-review-2026-10-10.md) — 고정 Chrome 78-node reference로 속성별 기준 축·Flex cyclic gap·definite auto-size 및 지원 범위를 결정한 계획 검토.
- [C06.2 · 백분율 spacing 구현 공격 검토와 실행](./evidence/c06-spacing-percentages-implementation-2026-10-10.md) — 별도 코드 실패 경로 20개, Chrome 71-node geometry 최대 오차, Rust 전체 검사 및 Android/iOS Simulator 실제 V8→WGPU 화면·로그.
- [C06.3 · 절대 길이 단위 변환 계획 검토](./evidence/c06-3-absolute-lengths-plan-review-2026-10-10.md) — Stylo CSS px 정규화, 7개 절대 단위, 화면 배율 경계, 26-node Chrome fixture와 검증 범위 검토.
- [C07.1 · 최소·최대 크기 구현 전 비교 모델](./evidence/c07-1-min-max-sizing-precomparison-2026-10-10.md) — Chrome 154 기준 35개 노드의 min/max CSS 값·box-sizing·percentage·flex 관찰과 입력 hash.
- [C07.1 · 최소·최대 크기 계획 실패 경로 검토](./evidence/c07-1-min-max-sizing-plan-review-2026-10-10.md) — 계획 구현 전 oracle, CSS sizing 경계, flex clamp, typed AST, profile 및 시뮬레이터 주장 검토.
- [C07.1 · 최소·최대 크기 구현 검토와 실행 근거](./evidence/c07-1-min-max-sizing-implementation-review-2026-10-10.md) — 별도 구현 실패 경로, workspace·FFI 검사와 Android API 37/iOS 26.2 Simulator 실제 V8→WGPU 화면·로그.
- [C07.2 · 테두리 폭 구현 전 비교 모델](./evidence/c07-2-border-width-layout-precomparison-2026-10-10.md) — pinned Chrome 154의 50-node·DPR 1/2 기준, fixture hash와 box geometry.
- [C07.2 · 테두리 폭 계획 실패 경로 검토](./evidence/c07-2-border-width-layout-plan-review-2026-10-10.md) — 계획의 20개 독립 실패 관점, fixture에서 발견해 추가한 shorthand/style/min-max/math 경계.
- [C07.2 · 테두리 폭 Simulator 실행](./evidence/c07-2-border-width-layout-simulators-2026-10-10.md) — 50-node Chrome 기준, Android API 37·iOS 26.2 Simulator V8→WGPU 실행, 화면·로그와 측정 한계.
- [C07.2 · 테두리 폭 구현 실패 관점 검토](./evidence/c07-2-border-width-layout-implementation-review-2026-10-10.md) — 구현 후 코드·실행 경로의 실패 점검과 수정한 non-finite 값 경계.
- [C07.3 · 종횡비 수정 계획 재검토](./evidence/c07-3-aspect-ratio-plan-review-scope-followup-2026-10-10.md) — Taffy differential 이후 축소한 범위, computed/used oracle 분리, 시뮬레이터 주장 경계를 재대조.
- [C07.3 · Taffy 차이와 시뮬레이터 결과](./evidence/c07-3-taffy-differential-2026-10-10.md) — definite leaf 차이와 min/max fail-closed 근거, 전체 회귀검사, Android/iOS V8→WGPU 화면·로그.
- [C07.3 · 구현 실패 경로 검토](./evidence/c07-3-aspect-ratio-implementation-review-2026-10-10.md) — Stylo typed 값부터 플랫폼 fixture까지 실패 경계를 점검하고 테스트·실행 결과를 기록.
- [C07.2 · 테두리 폭 내부 계약](./0044-c07-2-border-width-layout.md) — 제한 runtime profile 내부 구현 계약. 문서 ID `0044`, 숫자 버전 `0.1.0` 고정.
- [C07.3 · 종횡비 구현 전 비교 모델](./evidence/c07-3-aspect-ratio-precomparison-2026-10-10.md) — Chrome 154의 35-node·DPR 1/2 결과와 Flex stretch, box sizing, degenerate ratio 관찰.
- [C07.3 · Taffy 차이와 적용 경계](./evidence/c07-3-taffy-differential-2026-10-10.md) — 두 definite leaf 크기 보정, Chrome과 다른 min/max 전이, computed value 비교 단계, spacer fixture와 현재 검증 한계를 기록.
- [C07.3 · 종횡비 계획 실패 경로 검토](./evidence/c07-3-aspect-ratio-plan-review-2026-10-10.md) — 계획과 실제 사전 기준을 20개 독립 실패 관점으로 대조.
- [C07.3 · 종횡비 내부 계약](./0045-c07-3-aspect-ratio.md) — 제한 runtime layout 내부 구현 계획 계약. 문서 ID `0045`, 숫자 버전 `0.1.0` 고정.
- [C08 · Block 흐름 구현 전 비교 모델](./evidence/c08-block-flow-precomparison-2026-10-10.md) — 고정 Chromium 154·DPR 1/2의 기본 Block 흐름, visibility, 배경·글꼴 값과 한계를 기록.
- [C08 · Block 흐름 계획 실패 경로 검토](./evidence/c08-block-flow-plan-review-2026-10-10.md) — 별도 계획의 20개 실패 관점 및 반영한 fixture/contract 경계.
- [C08 · Block 흐름 구현 실패 경로 검토](./evidence/c08-block-flow-implementation-review-2026-10-10.md) — 코드·실행 경로를 별도 20개 관점으로 확인하고 빠진 실패 테스트를 추가.
- [C08 · Android/iOS Simulator 실행 근거](./evidence/c08-block-flow-simulators-2026-10-10.md) — V8 fixture, layout·WGPU 로그, screenshot 색상 표본과 backend 한계.
- [C08 · Block·기본 페인트 내부 계약](./0046-c08-block-paint.md) — 제한 subset 구현·검증 계약. 문서 ID `0046`, 숫자 버전 `0.1.0` 고정.
- [C06·C07.1 · 통합 변경 적대적 검토](./evidence/c06-c071-integrated-review-2026-10-10.md) — 누적 diff의 단위·cache·revision·C ABI·Android/iOS 경계를 교차 확인하고 수정 및 현재 검증 상태를 기록.
- [C07.1 · 물리 축 최소·최대 크기 내부 계약](./0043-c07-1-min-max-sizing.md) — 출시 전 숫자 버전 `0.1.0`으로 고정한 Stylo typed DTO→Taffy 제약과 오류 경계.
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
- [C04.7 · CSS media 환경 입력](./evidence/css-c04-media-environment-2026-10-09.md) — Chromium desktop/mobile × light/dark의 12개 media query와 computed-style 비교, 혼합/무포인터 Stylo mapping 및 제한 경계.
- [C04.8 · Runtime UA cascade 계획 검토](./evidence/c04-runtime-ua-cascade-plan-review-2026-10-09.md) — V8 task 경계, 세션 초기화 CSS worker, latest-wins 요청, full revision 일치, startup/snapshot 복제 비용과 FFI readback 실패 경계.
- [C04.8 · fragment root 의미 계획 보정 검토](./evidence/c04-runtime-ua-root-semantics-plan-review-2026-10-09.md) — direct HostRoot element의 Stylo `:root` blockification이 Chromium UA computed value를 바꾸는 불일치를 보정하고 fragment semantics·기존 adapter 회귀 경계를 검토.
- [C04.8 · Runtime UA cascade 시뮬레이터 실행](./evidence/c04-runtime-ua-cascade-simulators-2026-10-09.md) — Android API 37.1·iPhone 17 Pro iOS 26.2 Simulator의 실제 V8 JSON 결과, 화면, 측정 경계와 한계.
- [C04.8 · Runtime UA cascade 구현 후 검토](./evidence/c04-runtime-ua-cascade-implementation-review-2026-10-09.md) — 구현 경계별 실패 검토, 수정한 계약·iOS 보고서 표시, 현재 남은 제품 범위.
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
- [C09 · Block formatting 계획](../../plan/c09-block-formatting.md) — C09.1–C09.4 범위, 의존성, Chromium oracle와 completion gate.
- [C09 · 계획 실패 관점 검토](./evidence/c09-block-formatting-plan-review-2026-10-10.md) — 계획 자체를 20개 독립 반례·실패 경로로 대조한 기록. 기능 구현 검토는 별도다.
- [C10.2 · Flex 크기 배분 구현 검토와 Simulator 근거](./evidence/c10-2-flex-distribution-implementation-review-2026-10-10.md) — Chromium 27 case·92 node 비교, cascade·직접 입력 오류 경계, Android/iOS 실제 V8 frame 5/5와 WGPU 제출을 기록한다. PR #116 병합 후 상태를 동기화했다.
