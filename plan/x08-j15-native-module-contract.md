# X08/J15 네이티브 모듈 계약 계획

- **상태:** 계획 초안 · 구현·적합성 검증 미시작 · X08/J15 미완료 유지
- **공식 항목:** [X08 및 J15](../spec/STATUS.md)
- **제안 계약:** [0003 웹 표면·플랫폼 모듈 경계](../spec/0003-web-surface.md) · [0004 실행·빌드·배포](../spec/0004-runtime-build.md)
- **범위:** 기존 제안을 바탕으로 X08/J15의 결정 항목·소유자·차단 의존성과 acceptance case를 계획한다. 버전 있는 API, 등록 방식, 권한 정책을 이 문서에서 확정하지 않는다.

## 목적과 계획 범위

X08은 앱 개발자가 Kotlin·Swift·Rust·C++로 작성한 플랫폼 기능을 버전 있는 TypeScript/JavaScript 표면에서 호출하도록 연결하는 계약이다. J15는 사용자 정의 모듈의 선언·바인딩 생성·Android/iOS 등록, 웹 대체 또는 미지원 진단, 입력/출력·비동기 오류·취소·thread affinity·권한·자원 해제·host API 버전 호환을 이 상위 항목 안에서 다룬다. 계획 작업은 이 경계를 acceptance 질문과 결정 소유자로 정리하며, 모듈 runtime이나 생성기를 구현하지 않는다.

기존 [0003 제안](../spec/0003-web-surface.md)은 V8 handle·Rust pointer·내부 C ABI를 앱 코드에 공개하지 않고 JS 타입·빌드 바인딩·플랫폼 등록·오류·권한·취소·thread affinity·Isolate 수명·바이너리 호환성을 계약할 방향을 제안한다. 이 계획은 그 원칙을 새로 정의하지 않고, 이미 정해진 경계와 아직 결정을 요구하는 경계를 나눈다.

## 선행 의존과 소유 경계

| 항목 | 기존 책임 | 계획에서 남길 gate |
|---|---|---|
| S03 / 내부 V8 host binding | 엔진 adapter, isolate owner, JS↔native 호출 경계 | S03 내부 facade를 앱 공개 API로 재사용하지 않는다. 사용자 표면은 버전 있는 TypeScript/JavaScript 계약으로 제한하고, 내부 구현 binding 형식은 S03 internal host-binding boundary가 확정될 때까지 후보로 둔다. 공개 module API는 X08/J15가 소유한다. |
| S03.3 / [0021 DOM node lifecycle](../spec/internal/0021-s03-dom-node-lifecycle.md) | HostRoot, DOM wrapper, callback·external-root lease, collector와 종료 수명 | 일반 native module callback의 공개 등록·취소·session 계약은 J15, V8 handle 정리와 Isolate 종료 mechanics는 S03가 소유한다. DOM node/wrapper, HostNodeHandle 또는 S03.3 external-root lease를 붙잡는 closure의 추가 수명 규칙만 S03.3 경계로 위임한다. 기존 [수명 계획](./dom-node-lifecycle.md)과 겹치는 GC·node graph 모델은 복제하지 않는다. |
| D02 OTA compatibility | module/API version과 native binary compatibility를 릴리스 graph와 조정 | [0004 기존 제안](../spec/0004-runtime-build.md#ota)은 native code/V8/GPU renderer/host API/permission 변경에 새 앱 바이너리가 필요하다고 둔다. D02에는 manifest version 표현, 호환성 판정 시점과 정확한 OTA reject/error contract를 남긴다. |
| X16 conformance | 웹·Android·iOS 예제와 예상 결과를 반복 검증 | 세 플랫폼 지원 완료를 주장하지 않는다. X16이 정한 고정 입력과 플랫폼별 결과가 준비되기 전에는 acceptance plan만 둔다. |
| J14 built-in modules | 기본 safe-area, app state, back, keyboard, permission, haptics, URL 동작 | J14가 소유하는 기능별 permission prompt/denial behavior를 J15로 가져오지 않는다. J15는 사용자 모듈에서 공통으로 표현해야 할 capability/permission declaration 경계만 다룬다. |
| X08 / J15 | 사용자 모듈 API·빌드·플랫폼 등록의 상위/하위 상태 항목 | S03, D02, X16이 미완료인 점을 구현 선행 gate로 기록한다. 계획 작성 자체는 해당 구현 완료를 요구하지 않는다. |

## 계약 질문과 후보

| 경계 | 계획에서 확인할 질문 | 결정 전 금지할 주장 |
|---|---|---|
| 모듈 식별·버전 | 모듈 name, API version, binary/runtime requirement, duplicate registration, unknown/missing module을 빌드 또는 앱 시작 중 어디서 판정하는가? | registry key나 호환 범위를 임의로 고정하지 않는다. |
| 타입 선언·바인딩 생성 | TS declaration과 generated/native binding의 source of truth, 플랫폼별 구현 매핑, generated artifact provenance는 무엇인가? | 타입 파일만으로 runtime binding이나 지원 플랫폼이 존재한다고 표시하지 않는다. |
| 입력·결과·오류 | 값 직렬화/validation, sync·async result, native error category, rejection 전달, partial side effect를 어떻게 표현하는가? | V8 value/pointer 직접 노출이나 안정 error code를 제안된 것처럼 쓰지 않는다. |
| 취소·thread affinity | abort가 가능한 작업 종류, 취소 이후 결과/부작용, JS Isolate owner와 플랫폼 queue/thread의 허용 호출·반환 순서는 무엇인가? | 모든 API가 취소 가능하거나 UI/main thread에서 안전하다고 일반화하지 않는다. |
| 권한·기능 가용성 | 선언 권한, 사전 거부·미지원 OS, 사용자 거부·철회와 재요청, 웹 fallback의 책임이 J14·앱·모듈 중 누구에게 있는가? | native permission을 웹 권한과 같은 의미로 표시하거나 사용자 선택 없이 prompt를 띄우는 규칙을 만들지 않는다. |
| 모듈·callback 수명 | 등록, 앱 session, Isolate 종료, 모듈 교체와 native-held callback의 소유자·해제 시점은 무엇인가? | J15는 공개 callback register/cancel/session 계약을 정하고, S03은 V8 handle reset과 Isolate 폐기를 소유한다. DOM wrapper·HostNodeHandle·external-root lease를 캡처하는 closure에는 S03.3/0021의 추가 root 규칙을 적용한다. |
| 바이너리·OTA 호환 | host API version 불일치, OS별 capability 차이, OTA graph에서 요구 모듈이 없거나 version이 다를 때 어느 단계에서 거부하는가? | 원격 JS가 기존 바이너리의 native module을 호출하는 것과 OTA로 native implementation을 배포하는 것을 구분한다. 0004 제안상 native code/V8/GPU renderer/host API/permission 변경은 새 앱 바이너리가 필요하며, D02가 확정할 manifest 표현·reject point·오류를 미리 단정하지 않는다. |
| 대용량 데이터 | image/camera frame 전송이 필요할 때 일반 module call과 별도 ownership·release·backpressure 계약이 필요한가? | 기본 API를 zero-copy 또는 shared-buffer 경로로 설계하지 않는다. 별도 필요성·명세가 승인될 때까지 제외한다. |

정확한 TS API 이름, registry/schema key, generated binding tool, permission declaration format, error code 숫자, sync-call 허용 범위, cancellation guarantees, module lifecycle hooks, platform matrix는 아직 확정하지 않았다. 구현 PR 전 이 값들의 결정 소유자와 공개/internal 명세 위치를 먼저 합의한다.

## 계획 acceptance matrix

구현 후보를 정하기 전 아래 상태·오류 경로별 예상 관찰값과 owner를 고정한다. 이 표는 실제 API 예시나 구현 통과 결과가 아니다.

| 시나리오 | 계획에서 구분할 결과 |
|---|---|
| 선언·생성 binding은 있지만 platform registration이 없음 | build-time 또는 startup-time diagnostic owner와 unsupported platform 표기 결정. silent no-op으로 처리하지 않는다. |
| module이 빠졌거나 host/API version이 다름 | build, app startup, OTA activation 중 reject point와 오류 범위를 결정한다. D02가 확정되기 전 OTA 동작은 unresolved다. |
| 입력 타입·필수 필드가 잘못됨 | JS boundary에서 native side effect 전 실패하는지와 validation error owner를 정한다. |
| native operation이 동기/비동기 실패·취소·늦은 완료를 반환 | JS promise/error mapping, cancellation race, session 종료 뒤 callback 처리 owner를 정한다. |
| permission unavailable/denied/revoked | 앱 설정, 플랫폼 OS, J14 기능별 동작과 J15 module declaration의 책임을 분리한다. |
| 웹 대체 구현이 있거나 없음 | 같은 public call의 web fallback 제공자 또는 명시적 unsupported diagnostic을 정한다. 존재를 가정하지 않는다. |
| native module callback closure가 DOM node/wrapper/HostNodeHandle/external-root lease를 보유하려 함 | 해당 DOM root 보유 범위에 한해 S03.3/0021과 충돌 없는 소유·release 계약이 마련되기 전 지원하지 않는 것으로 둔다. 일반 callback의 공개 등록·취소·session 계약은 J15, V8 handle/Isolate 정리는 S03가 소유한다. |
| 플랫폼·ABI·host API 버전 조합이 지원표 밖임 | 성공 fallback으로 이어지지 않도록 fail-closed 조건과 diagnostic owner를 정한다. |

각 시나리오는 웹·Android·iOS에서 동일 의미를 강제하기 전에 platform capability의 실제 차이를 분리한다. permission prompt, app lifecycle, UI/main-thread 작업, JIT/Inspector 조건은 simulator나 브라우저로 일반화하지 않는다. 계획을 검토하는 동안은 browser·device·simulator를 열지 않는다.

## 계획 단계와 자원 경계

1. `spec/STATUS.md`, [0003](../spec/0003-web-surface.md), [0004](../spec/0004-runtime-build.md), [0021](../spec/internal/0021-s03-dom-node-lifecycle.md), [J14/J15](../spec/STATUS.md), D02 및 X16에서 결정된 내용과 미결 내용을 ownership map으로 정리한다.
2. 기존 [DOM lifecycle 계획](./dom-node-lifecycle.md)과 겹치는 node/wrapper/callback/lease 수명 세부는 S03.3로 위임하고, X08/J15에서는 module boundary와 그 의존 점만 기록한다.
3. 별도의 20개 실패 관점으로 계획 초안을 검토하고, 보정된 항목을 다시 확인한다([계획 검토 기록](./x08-j15-native-module-contract-plan-review-20.md)). 이 검토는 모듈 코드·API 지원·플랫폼 동작 통과가 아니다.
4. X08 구현 PR에는 승인된 버전 있는 계약·예제·Android/iOS/web 동작 evidence와 S03/D02/X16 dependency closure를 연결하고, 실제 코드·runtime 실패 관점 및 별도 PR diff 검토를 수행한다.

계획은 공유 중인 더티 checkout을 건드리지 않도록 별도 worktree에서 작성했다. X08/J15 작업은 공식 의존성인 S03·D02·X16이 준비되면 깨끗한 격리 worktree에서 진행할 수 있으며, C11 완료를 기다릴 필요는 없다. 다른 활성 작업과 Cargo/V8 빌드·테스트를 겹칠 경우에는 CPU/I/O 자원 경합을 별도로 확인한다.

## 중단 조건과 완료 경계

- S03 internal host-binding boundary, X08/J15 public API owner 또는 error/cancellation/lifecycle responsibilities를 정할 자료가 없으면 API 결정을 보류한다.
- S03.3/0021과 충돌하는 DOM wrapper, callback, external-root lease ownership이 발견되면 그 범위는 여기서 다시 정의하지 않고 해당 계약 owner에게 돌린다.
- J14 permission prompt/denial behavior와 J15 공통 declaration이 구분되지 않으면 기본 모듈 또는 사용자 모듈의 권한 동작을 지원한다고 쓰지 않는다.
- D02 binary/OTA compatibility, X16 platform acceptance 또는 host API version rule이 unresolved이면 그 조합을 허용하거나 실행하는 구현을 보류한다.
- high-volume data path의 ownership·release·backpressure가 별도 계약 없이 요구되면 기본 모듈 호출에 끼워 넣지 않고 별도 결정으로 분리한다.
- 공유 main checkout의 C11 WIP와 증거를 X08/J15 작업에서 수정·재사용하지 않는다. X08/J15 구현은 별도 clean worktree에서 공식 의존성 및 자체 검증 계획에 따라 진행한다. 동시에 자원 집약 검증을 실행하려면 별도 자원 확인이 필요하지만, C11은 X08/J15의 제품 기능 의존성이 아니다.

이 계획은 [상태 대장](../spec/STATUS.md)의 X08/J15 체크, 공식 완료 수, 공개 API 또는 내부 interface version을 변경하지 않는다. X08/J15는 계속 미완료이며, 이 문서만으로 네이티브 모듈이 빌드되거나 Android/iOS/web에서 동작한다고 주장할 수 없다.
