# R07 Vue·Svelte 어댑터 실험 계획 적대 검토 20회

검토 기준: `plan/r07-framework-adapter-feasibility.md` 및 사전 oracle `r07-framework-adapter-precomparison-2026-10-16/oracle.json`. 계획 단계 검토이며 구현 검토나 PR 검토를 대신하지 않는다.

| # | 실패 관점 | 확인 결과 및 반영 |
|---:|---|---|
| 1 | 기대값이 실험 출력에서 역산되는가 | 고정된 counter 상태 전이 JSON을 oracle로 두었다. 실행 출력으로 oracle을 갱신하지 않는다. |
| 2 | 프레임워크 버전이 실행마다 바뀌는가 | spike 전용 lockfile과 integrity를 사전 조건으로 명시했다. |
| 3 | Vue가 실제 custom renderer 경로를 쓰지 않는가 | Vue custom renderer로 실행하고 host API 호출을 기록한다. |
| 4 | Svelte가 SSR 결과만 비교해 client host path를 건너뛰는가 | Svelte compiler의 client output을 실행하는 제한 DOM facade를 요구한다. |
| 5 | callback 직접 호출을 OS click 검증으로 과장하는가 | 입력을 synthetic callback 1회로 제한하고 실제 OS/browser 입력을 명시적으로 제외했다. |
| 6 | 등록되지 않은 callback도 상태를 바꿀 수 있는가 | callback 등록을 기록하고 harness에서 adapter가 연결한 event route만 두 번 호출해 매번 한 단계만 바뀌는지 확인하도록 확장했다. |
| 7 | unmount 후 listener가 남아 다음 시나리오에 영향을 주는가 | 계획을 정정해 제품 노드와 Svelte root/document delegated listener 제거, detached target 거부만 판정한다. framework closure 회수·GC는 확인 불가 한계로 명시했다. |
| 8 | Vue comment/fragment node가 Element/Text 계약에 섞이는가 | framework bookkeeping은 adapter-local로 두고 제품 projection에는 승격하지 않는다. |
| 9 | Svelte가 createDocumentFragment 등 범위 밖 DOM을 몰래 쓰는가 | 허용 목록 밖 host operation은 명시 오류로 실패하도록 계획을 보강했다. |
| 10 | 서로 다른 operation 순서를 byte-equal로 강요하는가 | 원본 trace는 보존하되 oracle 비교는 정규화된 tree와 callback으로 제한했다. |
| 11 | `aria-label`을 일반 DOM/API 지원으로 과장하는가 | 단일 문자열 속성만 fixture에 두고 임의 property·접근성 검증을 제외했다. |
| 12 | 잘못된 속성이나 노드가 조용히 사라지는가 | 지원하지 않는 동작은 무시 대신 식별 가능한 오류로 종료한다. |
| 13 | `OwnerId` 검증을 recorder만으로 실제 Rust 집행이라 주장하는가 | 명시 OwnerId 전달만 판정하고 실제 Rust 소유권 거부 검증은 한계로 고정했다. |
| 14 | document revision·generation을 adapter spike가 입증했다고 보는가 | revision·generation 및 FFI 수명 주장은 완료 범위에서 제외했다. |
| 15 | 여러 root·framework 공유 트리의 충돌을 놓치는가 | 단일 루트 실험이며 다중 루트는 R03 후속 범위로 명시했다. |
| 16 | click 수가 초기 렌더 수와 뒤섞이는가 | mount 0, 첫 synthetic click 뒤 1, 두 번째 뒤 2를 단계별 oracle에 고정했다. |
| 17 | framework 컴파일 오류를 제품 호환 실패/성공으로 모호하게 기록하는가 | exact version·compile/runtime 오류를 원본 근거로 남기고 실패 시 R07 미완료로 둔다. |
| 18 | Vite·모바일 번들·렌더러를 실험에서 검증했다고 보는가 | 번들러와 실제 surface 검증은 비목표로 분리했다. |
| 19 | React 전용 가정 부재를 framework 동등성으로 과장하는가 | 공통 계약 표면 점검과 두 어댑터 가능성만 조사하며 React 상호운용/동등성은 주장하지 않는다. |
| 20 | 성공 결과가 R03 전체 또는 공개 지원으로 확대되는가 | STATUS 결과 문구에 조사 산출물·미완료 계약 경계를 남기도록 계획했다. |

## 판정

20개 관점에서 계획을 검토하고 발견한 입력·수명·소유권·지원 범위 모호성을 계획에 반영했다. 구현 진행 가능. 구현 결과를 대상으로 한 새 20개 관점 검토는 별도로 작성한다.
