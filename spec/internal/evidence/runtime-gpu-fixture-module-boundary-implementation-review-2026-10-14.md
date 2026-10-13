# Runtime GPU 픽스처 모듈·빌드 경계 구현 검토 · 2026-10-14

## 실패 경로 검토

1. 고정 fixture scene adapter가 제품 runtime 모듈에 남는가 — `runtime_gpu/fixtures/` 아래의 작업 ID 모듈만 adapter를 소유한다.
2. C04–C12 FFI fixture 진입점이 제품 FFI 모듈에 남는가 — 작업별 `runtime_gpu/ffi/fixtures/` 모듈로 이동했다.
3. runtime GPU의 fixture 모듈이 기본 feature에 연결되는가 — 부모 `fixtures` 모듈과 re-export 모두 `c04-runtime-gpu-fixture-test` 아래에 둔다.
4. C11.2.3 fixture가 다른 Grid feature와 함께 기본 활성화되는가 — 전용 `c11-2-3-grid-fixture-test` gate를 유지한다.
5. UA cascade/layout probe가 제품 API에 남는가 — 별도 `runtime_ua_cascade/fixtures/`와 `c04-runtime-ua-cascade-fixture-test` gate를 사용한다.
6. JSON·JavaScript `include_str!`가 core 경로에서 초기화되는가 — 고정 payload와 include 지점을 opt-in fixture module로 옮겼다.
7. 모듈 경로만 옮기고 부모 선언은 기본 빌드에 남기는가 — `mod fixtures` 선언 자체에 feature 조건이 있다.
8. fixture module이 GPU core 없이 빌드되는가 — GPU 앱 fixture feature가 `c04-runtime-gpu`를 의존한다.
9. C11.2.3 전용 feature가 GPU fixture layer를 우회하는가 — 해당 feature는 `c04-runtime-gpu-fixture-test`를 의존한다.
10. 공용 host 생성·임의 source 평가가 fixture gate에 갇히는가 — 일반 runtime host와 공용 C ABI는 core 모듈에 남긴다.
11. C11.2.3 pixel readback이 일반 surface ABI로 노출되는가 — 전용 fixture FFI와 별도 pixel-test feature를 모두 요구한다.
12. 기존 C12.4 readback·stack test hooks가 기본 ABI에 남는가 — 관련 심볼과 Rust 코드를 각각 opt-in feature로 묶는다.
13. Android의 Cargo feature와 C/C++ header 조건이 갈라지는가 — 빌드 스크립트가 같은 검증 모드에서 feature와 define을 함께 전달한다.
14. iOS Simulator/native의 Cargo feature와 Xcode header 조건이 갈라지는가 — 두 iOS 빌더와 project definition이 같은 feature/define 값을 전달한다.
15. C header가 feature off에서도 fixture 심볼을 선언하는가 — 전처리 테스트에서 기본 GPU core와 fixture ABI를 분리해 확인한다.
16. 잘못된 feature 값이 조용히 무시되는가 — Android/iOS 빌더가 0·1 밖의 값을 거부한다.
17. 이동된 include 상대경로가 잘못되는가 — core, fixture-test 조합과 test suite를 실제 컴파일·실행한다.
18. core archive가 fixture 심볼만 제거하고 payload 문자열을 남기는가 — 기본 artifact와 positive control을 심볼·문자열로 각각 검사한다.
19. positive control도 잘못 설정되어 fixture 포함을 검출하지 못하는가 — fixture-test artifact에서 C11.2.3·UA probe marker가 나타나는 것을 확인한다.
20. 파일 이름 경계만 만족시키고 테스트/모듈 책임을 섞는가 — `foo.rs`/`foo/` 구조, `mod.rs` 부재, 순수 Rust `#[cfg(test)]`와 앱 fixture feature 분리를 확인했다.

## 검증 결과

- 기본 FFI 테스트: `cargo test --locked -p spinon-ffi --no-default-features` 18/18 통과.
- 앱 fixture feature 조합: `cargo test --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu-fixture-test,c11-2-3-grid-fixture-test,c04-runtime-ua-cascade-fixture-test` 43 passed·2 ignored·0 failed.
- `cargo fmt --all -- --check`, 같은 feature 조합의 Clippy `--all-targets -- -D warnings`, iOS build-flag Node test 9/9가 통과했다.
- `c04-runtime-gpu` 기본 Debug static library와 fixture-test positive control의 SHA-256·크기·심볼·payload marker는 [산출물 근거](../runtime-gpu-fixture-module-boundary-2026-10-14/README.md)에 기록했다.
- Android/iOS release application artifact stripping과 최종 설치 bundle 크기는 검사 범위에 포함하지 않는다.

## runtime GPU 책임 분할 실패 경로 검토

1. presentation sequence가 기존 0 초기값과 달라지는가 — 기존 초기값을 유지한다.
2. sequence 증가가 overflow 후 값을 재사용하는가 — `checked_add` 실패를 오류로 반환한다.
3. 오래된 sequence로 scene을 publish할 수 있는가 — 현재 sequence와 불일치하면 거부한다.
4. publish와 invalidate가 경합할 때 낡은 frame이 살아남는가 — `invalidation_winning_a_publish_race_rejects_the_stale_scene`가 재현해 거부한다.
5. invalidate 뒤 현재 scene 조회가 이전 snapshot을 반환하는가 — sequence mismatch면 비우고 테스트로 확인한다.
6. draw 중 render snapshot 소유권이 사라지는가 — `Arc<RuntimeRenderSnapshot>` guard 수명을 draw 종료까지 유지한다.
7. draw가 snapshot mutex guard 없이 scene을 참조하는가 — renderer 호출 범위에서 guard를 유지한다.
8. draw 도중 새 요청이 들어오면 낡은 scene을 제출하는가 — 제출 closure에서 sequence를 다시 검사한다.
9. renderer가 없는 draw/resize가 성공으로 보고되는가 — 두 경로에서 명시 오류를 반환한다.
10. resize가 진행 중 scene을 무효화하지 않는가 — resize 전 presentation sequence를 증가시킨다.
11. Android surface renderer를 중복 생성할 수 있는가 — 이미 renderer가 있으면 거부한다.
12. UIKit surface 준비가 renderer 작업 중 무기한 대기하는가 — `try_lock`이 `WouldBlock`이면 즉시 명시 오류를 반환한다.
13. UIKit pending surface를 덮어쓸 수 있는가 — 이미 대기 중이면 거부한다.
14. pending surface 없이 UIKit renderer를 만들 수 있는가 — 준비된 surface가 없으면 오류를 반환한다.
15. UIKit renderer가 동일 pending surface를 다시 쓸 수 있는가 — 생성 때 `take()`로 한 번만 소비한다.
16. renderer 생성 전 surface configure가 성공하는가 — renderer 존재를 확인한 다음 configure한다.
17. pixel readback flag가 기본 build에 남는가 — 관련 코드를 pixel-test feature cfg 아래 둔다.
18. host teardown 뒤 renderer/pending UIKit surface가 남는가 — 두 slot을 꺼내 drop한다.
19. 추출된 presentation/renderer helper가 crate 외부 API가 되는가 — sibling 접근을 `pub(super)`에 제한한다.
20. 파일 분할로 fixture ABI 경계가 흐려지는가 — 위 fixture 경계 검증과 default/positive artifact 검사를 유지한다.

해당 동시성·scene publication 단위 검사는 앱 fixture feature suite에 포함되어 통과했다. Android API 37 AVD와 iOS Simulator 실행 증거는 앞선 혼합 트리 진단이며 이 후보의 독립 플랫폼 검증으로 세지 않는다.

## 판정 경계

소스 경로만으로 binary exclusion을 주장하지 않는다. 이 검토는 Rust module, Cargo feature, C/C++ compile define, C header 및 host Debug static archive의 선택 marker 경계만 다룬다. CSS 동작 적합성, 전체 workspace 회귀, Android APK/iOS release artifact, 앱 bundle 크기, CSS 기능 완료는 판정하지 않는다.
