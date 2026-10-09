# C04.10 대기열·종료·실패 보완 계획 검토

- 검토 대상: [`C04.10 구현 계획`](../../../plan/c04-runtime-css-to-gpu.md)의 bounded admission, 종료, draw 오류 주입 보완.
- 검토 대상 SHA-256: `19c80840b78499b7565df47ce33f874eef5fba5e89463ecd894d3ff7e20ea90f`
- 구분: 구현 전 계획 검토다. 코드·시뮬레이터 통과를 뜻하지 않는다.

## 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획 판정 |
|---:|---|---|
| 1 | 렌더 요청 병합 정책이 JavaScript 평가나 DOM 변경까지 버려도 된다고 오해되는가 | 통과. JS 평가와 `HostDocument` 변경은 순서 보존 lane이며 무손실이라고 분리했다. |
| 2 | 환경 입력의 viewport와 scheme은 따로 최신화되어 서로 다른 시점의 값이 합쳐지는가 | 통과. platform이 소유하는 최신 canonical environment state를 함께 읽도록 했다. |
| 3 | lane이 사실상 무제한 closure/snapshot을 queue에 계속 쌓는가 | 통과. lane당 실행 하나와 executor 대기 drain 하나, 최신 상태 하나로 제한했다. |
| 4 | 부하가 계속될 때 이전 입력을 모두 처리하느라 최신 입력이 끝없이 밀리는가 | 통과. 이전 environment revision 계산을 생략하고 executor queue tail에서 최신 상태를 처리한다. 지연 상한을 약속하지 않으며 유입이 멈춘 뒤 최종값 수렴을 검증한다. |
| 5 | 실행 중 새 presentation이 오면 dirty marker 설정과 drain 예약 사이에서 요청이 유실되는가 | 통과. coalescer의 동기화·단일 재예약 불변 조건을 코드 계획 및 경쟁 테스트로 검증한다. |
| 6 | surface가 크기는 같고 format/lifetime만 바뀌었는데 renderer generation이 유지되는가 | 통과. generation 입력에 surface 수명·format·크기를 포함하도록 확장했다. |
| 7 | 이전 generation의 renderer 생성이 완료된 뒤 현재 renderer로 덮어쓰는가 | 통과. 생성 전후 generation을 비교하고 stale 결과는 파괴한 뒤 canonical 최신 상태를 재조정한다. |
| 8 | 현재 generation의 생성 실패를 즉시 자동 재시도해 오류 loop나 queue 폭주를 만드는가 | 통과. 현재 generation 실패는 오류 상태로 멈추고 새 surface 입력 또는 명시적 retry에서만 다시 시도한다. |
| 9 | Android surface resize callback마다 destroy/create closure가 대기열에 누적되는가 | 통과. callback payload 대신 현재 surface canonical state를 읽는 하나의 idempotent reconciler를 둔다. |
| 10 | `surfaceDestroyed`가 render가 surface를 쓰는 중에 반환하거나 timeout 뒤 위험한 상태를 방치하는가 | 통과. generation과 draw admission을 먼저 무효화하고 renderer drain/destroy 전 반환하지 않는다. timeout 안전성은 약속하지 않고 대기 시간을 기록한다. |
| 11 | Android 대기 latch가 생성·destroy 예외에서 풀리지 않아 UI callback이 영구 대기하는가 | 통과. renderer 종료 조정의 `finally`에서 barrier를 해제하고 별도 실패 주입으로 확인한다. |
| 12 | Activity 종료와 surface 종료가 겹쳐 같은 renderer/host를 두 번 해제하는가 | 통과. close admission, runtime drain, render destroy, host free의 단일 소유 순서를 명시했다. |
| 13 | 실행 중 V8/FFI/GPU 작업이 끝나기 전에 host나 native window가 해제되는가 | 통과. 새 작업을 닫고 실행 중 호출이 끝난 뒤 queue barrier에서 host/surface를 해제한다. 강제 취소는 하지 않는다. |
| 14 | iOS layout callback이 renderer configure를 무제한 main queue에 적재하는가 | 통과. pending UIKit configure는 하나만 두고 그동안의 layout 변경은 최신 canonical state로 합친다. |
| 15 | UIKit 또는 `CAMetalLayer` 접근이 render/background thread에서 발생하는가 | 통과. 해당 접근은 main thread 전용이며 draw/adapter 작업과 경계를 분리했다. |
| 16 | iOS main thread가 render drain을 동기 대기해 화면 정지·교착을 만드는가 | 통과. main thread의 동기 queue wait를 금지하고 종료 drain은 비동기로 수행한다. |
| 17 | 종료 뒤 pending draw가 stale surface로 실행되거나 active draw가 강제 취소되는가 | 통과. pending draw는 close 시 버리고 active draw는 완료시킨 뒤 renderer를 해제한다. |
| 18 | draw 오류 주입이 정식 앱에 노출되거나 성공 결과를 가짜로 만드는가 | 통과. 내부 검증 빌드만 hook을 포함하고 오류 결과는 nonzero로 보존한다. 기본 제품 빌드에서 심볼·경로가 빠지는지 확인한다. |
| 19 | 합성 오류 검증을 실제 GPU/driver 장애 복구 증거로 과장하는가 | 통과. hook은 draw 경계에서 한 번 실패시켜 전달·snapshot 불변·다음 정상 draw만 검증한다고 한계를 명시했다. |
| 20 | 시험이 단순 호출 횟수만 보고 최종 상태, 예외 복구, queue 상한을 놓치는가 | 통과. latch 중 10,000개 이상 유입, 대기 drain 수, 마지막 viewport/scheme/generation, consumer/executor 실패, pending 종료, 후속 draw를 분리해 확인한다. |

## 계획 보정 사항

첫 검토 초안에서 확인한 모호성을 계획에 반영했다. 최신값 유입이 계속될 때의 지연 상한을 보장하지 않고 quiescence 뒤 최종값 수렴으로 정의했다. 현재 generation의 surface 생성 실패는 자동 재시도하지 않도록 했다. UIKit configure 예약량, 종료 중 Android 무기한 대기 가능성, lane 작업 예외 시 scheduled 상태 복구와 상태 잠금 금지도 완료 조건에 넣었다.

## 구현 전 판정

보완 계약은 현재 20개 실패 관점에서 모순 없이 정리됐다. 다음 단계는 Android·iOS coalescing lane과 Android surface reconciler를 구현하고, 각 플랫폼의 실제 queue stress 및 종료·draw 오류 주입을 실행하는 것이다. 이 계획 검토는 해당 구현이나 검증을 대신하지 않는다.
