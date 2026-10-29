# C11.3.3 leading named implicit lines PR 변경 실패 관점 검토 30개

이번 PR의 실제 base-to-head diff를 대상으로 구현 검토 20개와 다른 실패 경로를 확인한다. 구현 PR 생성 후 PR 번호, base/head SHA, GitHub diff 대조를 이 문서에 확정한다.

1. **변경 범위** — C11.3.3 resolver, signed track normalization, 제한 cascade, fixture route, 검증 도구와 evidence만 포함한다. unrelated worktree entries는 stage 대상에서 제외한다.
2. **공식 완료 상태** — `spec/STATUS.md`의 C11 checkbox 및 완료 집계를 변경하지 않는다. C11과 `72/213`은 미완료로 남는다.
3. **내부 계약 버전** — `0059`의 `0.1.0`을 유지하고 새 제한 조건을 별도 C11.3.3 section으로 덧붙인다.
4. **계획 연결** — 구현 범위가 병합된 C11.3.3 leading named implicit lines 계획과 고정 precomparison에 연결된다.
5. **규범·oracle 표현** — CRD §8.3을 기준 권위로 두고 Chrome 156 observation의 binary hash 미고정을 명시한다.
6. **file size 정책** — resolver 및 layout helper를 하위 모듈로 분리해 새 Rust production file이 500줄을 넘지 않는다.
7. **Rust module ownership** — 새로운 Rust implementation은 기존 crate 책임 아래 모듈화되고 `mod.rs`나 중복 old path를 만들지 않는다.
8. **FFI fixture 위치** — scene catalog와 source는 `runtime_gpu/fixtures/c11_3_3.rs`에 유지되고 FFI adapter는 `runtime_gpu/ffi/fixtures/c11_3_3.rs` 경계를 따른다.
9. **feature gate** — 앱 fixture FFI는 `c04-runtime-gpu-fixture-test`에 묶이며 기본/product feature에 노출되지 않는다.
10. **artifact positive control** — 기본 static archive에서 C11.3.3 symbol/payload가 없고 opt-in archive에는 symbol 2개/payload가 있다.
11. **scene ID 일관성** — Rust source allowlist, Android/iOS route, capture scripts, pixel fixture와 Chrome inventory의 scene ID 일곱 개가 서로 일치한다.
12. **untrusted scene 입력** — 추가 scene ID는 allowlist로 resolve하고 임의 source injection 문자열은 거부한다.
13. **root width/profile boundary** — 모바일 runtime source는 각 expected track sum과 동일한 fixed width를 쓰며 unsupported intrinsic root sizing을 profile에 넣지 않는다.
14. **expected track integrity** — root width가 JSON `columnsPx` 합과 같고 seven scene inventory/precomparison vectors가 순서대로 매핑된다.
15. **pixel sample containment** — 각 pixel 좌표는 해당 item rect 내부에 있으며 target node ID가 scene의 item ID와 일치한다.
16. **Android RGB mapping** — 새 `named-implicit-item` sample을 expected blue로 지정하고 default background fallback을 막는 structural assertion이 있다.
17. **Android run ID length** — scene suffix를 포함한 run ID가 `[A-Za-z0-9_-]{1,80}`을 만족하며 scene 이름은 별도 field로 보존한다.
18. **Android target identity** — AVD route는 명시 serial, QEMU state 및 API 37 확인으로 physical device를 잘못 기록하지 않는다.
19. **Android renderer claim** — 결과 JSON은 ANGLE/Vulkan/SwiftShader software 경로를 표시하고 hardware GPU라고 확대하지 않는다.
20. **Android PixelCopy source** — item exact 판정은 full-screen screenshot이 아니라 앱 cache의 SurfaceView PixelCopy PNG/RGBA 결과를 사용한다.
21. **iOS simulator identity** — iOS runner는 지정 UDID, iOS 26.2 runtime과 iPhone 17 Pro identity를 확인한다.
22. **iOS sample transform** — CSS sample, 3x drawable coordinates, RGBA 및 fixture sample count를 비교한다.
23. **iOS compositor boundary** — docs/JSON/PR claim은 drawable readback까지만 말하며 compositor 이후 proof로 쓰지 않는다.
24. **old C11.3.3 regression** — 26 scene, 65 node의 success/fallback/rejection counts와 geometry를 새 scene 결과와 따로 남긴다.
25. **unknown-area regression** — 2026-10-21 fixed implicit track overflow scene 및 3 pixel samples가 각 플랫폼에서 별도로 대조된다.
26. **frame/paint identity** — 새 7 scene은 root/item 14 frames와 draw/present state를 검증하고 DOM node identity를 보존한다.
27. **expected rejection control** — 기존 fail-closed route-negative scene을 유지하며 새 seven scene admission이 unknown scene을 허용하지 않는다.
28. **failure provenance** — run ID 길이, unsupported max-content, RGB expectation 실패와 corrective rerun 결과를 별개 run directories에 보존한다.
29. **evidence link integrity** — README의 JSON, screenshots, fixture hash, app hash 및 precomparison links는 실제 tracked path를 가리킨다.
30. **PR description completeness** — PR template에 status ID, spec, Android/iOS 결과, screenshots, 초기 실패 수정, known suite failure 및 hardware/compositor/WPT 한계를 기록한다.

## PR 원격 diff 판정

PR 생성 뒤 base/head와 changed path 목록을 GitHub에서 다시 받아 이 항목들을 실제 remote diff에 대조하고, 발견 사항과 최종 판정을 기록한다. 리뷰가 끝나기 전에는 PR을 병합하지 않는다.
