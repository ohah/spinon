# 테스트 구성

Rust 단위·통합 테스트는 각 크레이트에, JavaScript 테스트는 각 패키지에 둡니다. 공통 플랫폼 fixture와 교차 패키지 시나리오는 이 디렉터리 아래에 둡니다. Android instrumentation과 iOS XCTest는 각 플랫폼 호스트에 둡니다.

호스트 작업·이벤트 공통 fixture 계약은 아직 확정되지 않았습니다. CSS 기준 입력은 `tests/fixtures/css/c01/`에서 별도 추적하며, Chromium 참조값은 브라우저 버전·revision과 환경 정보를 같이 저장합니다. 테스트가 명세의 지원 범위를 대신하거나 미구현 기능을 통과 처리하지 않게 합니다. 완료 여부는 [공식 상태 대장](../spec/STATUS.md), 관찰 가능한 동작은 [명세 인덱스](../spec/README.md)에서 확인합니다.

C01 초기 fixture는 Chromium 기본 UA 계산값과 내장 CSS 프로필 주입 결과만 대조하며 Stylo·제품 렌더 경로를 검증하지 않습니다. 실제 제품 테스트가 없는 동안에는 빈 검색 결과를 통과로 보고하는 통합 테스트 명령을 만들지 않습니다. 실행 명령은 테스트가 추가되는 크레이트·패키지와 함께 연결합니다.

S03.3 DOM 노드 수명 계약의 test-only Rust reference fixture는 `fixtures/dom/s03/node-lifecycle-v1.rs`와 `fixtures/dom/s03/node-lifecycle-limits-v1.rs`에 두며, `spinon-runtime` integration test가 이를 읽습니다. 유효 root graph 외에 malformed parent·child 관계, stale wrapper, foreign session/document generation·missing node 외부 root, 외부 root lease 한도, ID 소진, 동적 node registry, 생성·회수 실패 원자성을 확인합니다. fixture는 계약 oracle의 결정성만 확인하고 제품 collector, 실제 V8 weak handle reset 또는 모바일 실행을 검증하지 않습니다.

`spinon-runtime` 제품 단위 테스트는 16,385개 노드를 만들고 회수한 뒤 새 ID로 다시 생성하는 경로와 FFI collector의 16,385개 root·동적 출력 buffer 재시도를 확인합니다. buffer 부족 응답에서 문서가 바뀌지 않고 재호출 뒤 root 보존 또는 전체 회수가 되는지도 검사합니다. Android·iOS 테스트 전용 실제 V8 fixture도 16,385개 wrapper를 생성·보유·분리해 전체 registry scan, 동적 C++ vector 확장, weak wrapper sweep과 Rust 자원 기준선 복귀를 확인합니다. fixture는 같은 Isolate에서 16,394개 live wrapper scan을 추가로 10회 기록합니다. 재시작 7회와 동일 Isolate scan 10회의 wall-time 분포와 CFI 조건은 [반복 scan 근거](../spec/internal/evidence/s03-dynamic-registry-repeat-measurements-2026-10-05.md)에 있고, 대표적인 simulator 성능 수치로 보기에는 변동 폭이 큽니다. 실기기, release 성능 특성, CPU time 분리, 할당 실패 주입은 별도 검증 대상입니다.
