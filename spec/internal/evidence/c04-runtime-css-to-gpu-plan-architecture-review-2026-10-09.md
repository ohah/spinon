# C04.10 스레드·생명주기 계약 재검토

- 검토 대상: `plan/c04-runtime-css-to-gpu.md`
- SHA-256: `986f695e025259b08ea68c1b0b43251cb69a91edf8074d4dfd6d7a9e314a927f`
- 기존 계획 검토 `c04-runtime-css-to-gpu-plan-review-2026-10-09.md` 이후, 기존 RuntimeSession 실행기와 계획의 thread ownership을 실제 코드에서 대조하고 계획을 수정했습니다.
- 확인한 구현: `crates/spinon-runtime/src/session/actor.rs`, `crates/spinon-runtime/src/session/ua_cascade.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs`, `platforms/android/app/src/main/java/dev/spinon/bootstrap/R08GpuDemo.java`, `platforms/ios/Sources/R08GpuDemo.swift`.

## 독립 실패 관점 20개

1. **V8 isolate thread affinity** — RuntimeSession actor가 V8을 소유합니다. platform renderer queue로 옮기지 않고 기존 actor thread를 유지하도록 고쳤습니다.
2. **cascade/layout 소유권** — CSS cascade와 layout은 별도 worker에서 계산됩니다. 단일 platform queue라고 기술하지 않고 기존 worker 경계를 유지하도록 고쳤습니다.
3. **호출자 대기 전파** — C ABI eval 호출은 작업 완료를 동기 대기합니다. Android/iOS UI에서 직접 호출하지 않고 background executor에서 직렬 수행하도록 정했습니다.
4. **renderer queue의 교착** — GPU thread가 V8/CSS 결과를 기다리면 resize/destroy가 막힙니다. 계산 대기는 background executor에서 끝낸 뒤 장면만 전달하도록 고쳤습니다.
5. **서로 다른 큐의 resize 원자성** — environment 갱신과 surface 교체가 다른 queue에 있어 중간 상태 제출 위험이 있습니다. 하나의 host presentation sequence로 묶고 두 변경이 모두 적용될 때까지 제출을 막도록 정했습니다.
6. **빠른 연속 resize** — 과거 resize 작업이 새 크기를 덮을 수 있습니다. 단조 증가 sequence와 최신 surface generation만 허용하도록 정했습니다.
7. **문서 generation stale 장면** — 직전 HostDocument 결과가 남을 수 있습니다. 전체 scene key와 현재 requested key를 비교해 오래된 장면을 거부하도록 유지했습니다.
8. **render-tree revision 누락** — 같은 document revision에서 구조 revision이 달라질 수 있습니다. key에 render-tree revision을 포함하도록 유지했습니다.
9. **style revision 누락** — 같은 트리에서 inline style 수정 결과가 늦게 도착할 수 있습니다. style revision을 포함하도록 유지했습니다.
10. **environment update 미완료** — CSS worker의 environment revision이 바뀌기 전 이전 스타일이 제출될 수 있습니다. background executor가 새 revision을 확인한 sequence만 허용하도록 정했습니다.
11. **surface generation mismatch** — 표면 파괴 뒤 이전 drawable을 사용할 수 있습니다. render queue가 보유한 현재 generation과 scene 요청 generation을 제출 직전에 비교하도록 유지했습니다.
12. **surface 파괴 중 pending draw** — view/layer 해제와 대기 draw가 경합할 수 있습니다. render queue drain 뒤 표면을 파괴하도록 정했습니다.
13. **종료 중 새 작업 유입** — teardown 도중 eval/resize 요청이 들어올 수 있습니다. 요청 차단과 sequence 무효화를 renderer 해제보다 먼저 하도록 순서를 명시했습니다.
14. **session worker join 순서** — RuntimeSession worker가 callback 중인데 문서를 해제할 수 있습니다. background 요청을 멈추고 session 종료를 기다린 뒤 renderer를 해제하도록 정했습니다.
15. **문서 callback 수명** — V8 callback이 renderer나 platform 객체를 직접 참조하면 수명이 교차합니다. 장면은 소유권이 분리된 immutable 값으로 전달하도록 유지했습니다.
16. **플랫폼 경계 배열 왕복** — Node/paint 배열을 Kotlin/Swift로 복사했다가 Rust에 다시 넘길 수 있습니다. FFI 내부 Rust 경로로 직접 장면을 전달하도록 유지했습니다.
17. **기존 raw session ABI 호환** — 기존 opaque pointer를 새 host 형식으로 오해하면 UB가 납니다. 기존 session API의 포인터 의미를 보존하고 새 host entrypoint는 분리하도록 유지했습니다.
18. **main-thread surface work** — SurfaceHolder/UIKit callback에서 renderer를 만들면 UI를 막을 수 있습니다. callback은 변경을 전달하고 renderer 생성·draw·resize·destroy는 전용 queue에서 처리하도록 유지했습니다.
19. **오류를 성공 revision으로 오인** — draw 실패가 CSS 계산 성공으로 되돌아가거나 장면을 제출된 것으로 기록할 수 있습니다. 계산 key와 GPU 결과/오류를 독립 상태로 보존하도록 유지했습니다.
20. **출시 버전 과상승** — 내부 기능 추가가 package/contract release version을 올릴 수 있습니다. 계획에 숫자 버전 `0.1.0` 고정과 출시 결정 전 변경 금지를 명시했고 Cargo package 버전은 바꾸지 않습니다.

계획을 기존 RuntimeSession thread ownership에 맞춰 수정한 뒤 이 검토를 수행했습니다. 추가 미해결 충돌은 발견하지 못했습니다. 이 검토는 Android/iOS simulator 실행이나 기능 구현 검증이 아닙니다.
