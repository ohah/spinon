# R07 · Vue·Svelte 공통 호스트 어댑터 가능성 실험

**상태:** 구현 전 비교 기준 확정 · **대상:** R07 조사 항목만 · **공개 API:** 없음
**기준 계약:** [내부 인터페이스 0003](../spec/internal/0003-shared-host-contract.md) `0.1.0-draft`
**공식 상태:** [구현 상태 대장](../spec/STATUS.md#위험-검증)

## 질문과 범위

Vue와 Svelte가 프레임워크별 상태·diff 책임을 유지하면서, React에 종속되지 않은 동일한 Element/Text 호스트 노드 계약을 사용할 수 있는지 확인한다. Vue는 공식 custom renderer 경로를 사용하고, Svelte는 고정 compiler의 client output이 호출하는 DOM 작업만 받는 제한 facade로 실험한다. 실제 FFI·Rust `HostDocument` 연결이나 제품 어댑터는 구현하지 않는다.

## 구현 전 비교 기준

- **기준 계약:** `0003-shared-host-contract.md`의 Element/Text, 섞인 자식 순서, `OwnerId` 경계, 생성·속성·텍스트·삽입·삭제 동작. React 전용 타입·명령·패키지를 공통 계약에 추가하지 않는다.
- **독립 oracle:** 두 fixture의 상태 전이 표와 이 계획의 정규 HostDocument projection. 프레임워크가 만든 최종 트리와 관찰 가능한 click callback은 oracle에 비교한다. 구현기가 자기 출력을 기대값으로 만들지 않는다.
- **입력:** Vue와 Svelte 각각 하나의 counter fixture를 같은 플랫폼 무관 시나리오로 실행한다: 빈 root mount → `<main><button aria-label="count">count: 0</button></main>` 생성 → click 두 번을 차례로 전달 → 버튼 텍스트가 `count: 1`, `count: 2`로 한 단계씩 갱신 → unmount 시 root 자식 제거.
- **기록 필드:** framework/compiler version, adapter에 전달된 명시적 `OwnerId`, host operation과 callback 등록, 노드 종류·이름·순서, 속성 key/value, 이전/새 텍스트, synthetic click 횟수, mount/unmount 최종 트리.
- **입력 이벤트:** OS나 브라우저 click은 사용하지 않는다. 각 adapter가 등록한 click callback을 host harness가 두 번 순서대로 호출해 매번 한 번씩 상태가 증가하는지 확인한다.
- **통과 기준:** 두 실행의 초기·각 click 뒤·종료 projection이 oracle과 정확히 일치하고, 각 click이 상태를 한 번만 갱신한다. unmount는 제품 노드와 Svelte의 root/document delegated listener를 제거하며, host가 detached target click을 거부한다. detached object에 남을 수 있는 framework callback closure의 회수·GC는 판정하지 않는다. 지원하지 않는 DOM/host operation은 명시 오류로 끝나며 조용히 무시하지 않는다. 각 host operation은 전달받은 `OwnerId`를 유지한다. 실제 Rust 소유권 거부 동작은 이 adapter spike로 검증했다고 주장하지 않는다.
- **의존성 고정:** spike 전용 `package.json` 및 lockfile에서 Vue와 Svelte의 정확한 버전을 고정한다. 네트워크에서 내려받은 버전과 package integrity를 근거에 남긴다.

## 예상 어댑터 차이와 해석

- Vue `createRenderer`는 create/insert/remove/patch 등의 host operation을 받는 custom renderer 경로로 대응한다.
- Svelte client compiler output은 DOM 작업을 요구하므로 DOM-shaped facade가 해당 호출을 좁게 지원하고 같은 projection으로 내린다. facade 범위가 커지거나 요소·Text 바깥의 플랫폼 DOM이 필요하면 그 호출을 근거와 함께 실패시킨다.
- 서로 다른 adapter operation trace는 같을 필요가 없다. 각 명령을 보존해 감사 가능하게 기록하되 정규화된 node tree·속성·text·synthetic click 횟수를 oracle과 비교한다. 프레임워크 내부의 comment/fragment bookkeeping은 adapter-local로 보관하며 제품 노드로 승격하지 않는다.
- fixture는 의도적으로 한 루트·한 버튼·문자열 aria 속성·한 click callback만 사용한다. 다중 루트, fragment/list reconciliation, 임의 DOM property, 접근성 tree, CSS, 키보드/포인터 전파는 범위 밖이다.

## 결과 판정과 한계

이 실험만 통과하면 R07의 **어댑터 가능성 조사**를 완료할 수 있다. 결과는 JS recorder/harness가 `HostDocument` 내부 계약에 투영될 수 있음을 보일 뿐, Rust FFI 소유권 집행이나 React 대 Vue/Svelte 동작 동등성의 증거는 아니다. React·Vue·Svelte 제품 호환, FFI 수명, JS 오류 변환, 실제 화면 표시, 접근성, CSS 적합성, 여러 프레임워크 동시 사용, 공개 DOM API, R03 전체 구현은 완료 처리하지 않는다. 실패하면 어떤 framework host 요구가 공통 노드 계약과 어긋났는지 기록하고 R07은 미완료로 둔다.

## 검증 순서

1. 이 계획을 서로 다른 20개 실패 관점으로 검토하고 해결한다.
2. lockfile 및 fixture를 고정한 뒤 Vue/Svelte 실행과 정규화 projection을 구현한다.
3. 실제 관찰 trace를 oracle과 비교하고, unsupported operation·중복 click·stale root·교차 OwnerId 거부 경로를 확인한다.
4. 구현과 실행 근거를 새 20개 관점으로 검토하고 발견을 해결한다.
5. PR delta는 현재 프로젝트 규칙에 따라 서로 다른 30개 관점으로 검토한다.
