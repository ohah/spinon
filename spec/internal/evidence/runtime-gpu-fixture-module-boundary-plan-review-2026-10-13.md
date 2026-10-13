# runtime GPU 픽스처 모듈 경계 계획 검토

계획을 서로 다른 실패 관점에서 공격하고, 대응 범위를 계획에 반영했다.

1. fixture 분류 범위가 C11.2.3 하나에만 머물 위험 — C04–C12 전용 FFI와 C11 matrix adapter를 각각 열거한다.
2. 제품 runtime API까지 픽스처 디렉터리로 이동할 위험 — generic host 생성·arbitrary-source eval·환경 설정·presentation·공용 surface API는 보존한다.
3. C11.2.1 adapter를 놓칠 위험 — C11.1과 C11.2.1이 함께 들어 있는 adapter 파일도 fixtures 모듈로 이동한다.
4. C06–C12 개별 FFI 파일만 옮기고 부모 `ffi.rs`의 fixture export가 남을 위험 — 부모에 인라인된 C04·C05·C08·C09·C11·C12 진입점도 각 작업 파일로 분배한다.
5. C10.3.5 fixture가 별도 작업 모듈 위치를 잃을 위험 — 기존 `c10_3_5.rs` 모듈과 symbol을 유지한 채 fixtures 디렉터리로 옮긴다.
6. C11.3.1 feature 코드가 refactor PR에 섞일 위험 — 현재 main에 없는 feature 구현은 PR 대상에서 빼고, 별도 feature 작업 파일도 fixtures 경로를 사용한다.
7. C11.2.3을 `#[cfg(test)]`로만 gate해 모바일 검증 앱에서 사라질 위험 — 일반 앱 빌드에서 명시적으로 켤 수 있는 Cargo feature를 둔다.
8. C11.2.3 feature가 GPU dependency 없이 켜질 위험 — `c11-2-3-grid-fixture-test`가 `c04-runtime-gpu`를 의존하게 한다.
9. 기본 C04 runtime host가 fixture gate와 함께 사라질 위험 — generic `spinon_runtime_gpu_host_new`와 `ffi.rs` core API는 옮기거나 gate하지 않는다.
10. C11.2.3 pixel readback이 원래 별도 실험 feature를 잃을 위험 — 기존 `c11-grid-metal-surface-pixels-test` 조건을 보존한다.
11. Android 앱이 새 Cargo feature를 빠뜨릴 위험 — 같은 C04 build-script 분기에서 `c04-runtime-gpu`와 fixture feature를 선택한다.
12. iOS native 빌드가 새 Cargo feature를 빠뜨릴 위험 — 같은 C04 build-script 분기에서 두 feature를 선택한다.
13. Android/iOS C++ compile condition과 Cargo feature가 어긋날 위험 — 앱에서 이미 쓰는 C04 매크로와 Cargo feature 연결을 유지한다.
14. C 헤더가 존재하지 않는 C11.2.3 symbol을 선언할 위험 — host/eval 선언을 C04 compile condition에 둔다.
15. 더 깊은 경로에서 `include_str!`가 fixture JSON을 못 읽을 위험 — 이동한 파일 깊이만큼 상대 경로를 조정하고 feature 조합 컴파일로 확인한다.
16. 중첩 모듈 import가 다른 부모를 참조할 위험 — runtime GPU와 FFI helper 경계를 각각 올바른 부모 경로로 수정하고 Clippy까지 실행한다.
17. 모듈 경로 이동으로 C ABI가 바뀔 위험 — `no_mangle` 이름·인자·반환·오류 계약을 유지하고 컴파일 결과를 비교한다.
18. feature-off에도 C11.2.3 host/eval export가 포함될 위험 — C04-only와 fixture feature-on LLVM IR을 분리해 생성 여부를 확인한다.
19. module/test 이동으로 장면 선택과 인자 거부가 사라질 위험 — 기존 Rust FFI 테스트와 build-script 검사를 feature 조합별로 실행한다.
20. `fixtures/` 소스 경계를 바이너리 제외와 혼동할 위험 — 기본 빌드에 들어가던 UA cascade/layout fixture probe는 별도 feature로 제외하고, C04 모바일 검증 빌드가 GPU fixture harness와 `include_str!` payload를 포함한다는 점을 계획에 남긴다. 제품 빌드용 GPU fixture feature/platform 분리는 별도 완료 조건으로 기록하고 CSS 지원 상태·모바일 재검증 여부와 구분한다.

계획을 이 검토에 맞춰 보완했다. implementation review와 실행 결과는 별도 증거 문서에 기록한다.
