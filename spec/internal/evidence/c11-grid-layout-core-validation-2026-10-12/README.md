# C11.1 layout-core 실행 검증 · 2026-10-12

**기록 시점:** layout-core 단계 직후의 실행 snapshot이다. 이후 Stylo typed cascade와 Rust style→layout adapter가 추가되었다. 현재 범위와 재실행 결과는 [후속 C11.1 실행 검증](../c11-style-grid-projection-2026-10-12/README.md)과 [후속 adapter 검토](../c11-style-grid-projection-implementation-review-2026-10-12.md)를 기준으로 본다. 아래의 “미구현”은 당시 코드 상태 기록이다.

**현재 소스의 깊이 시험 경계:** 아래 기록은 당시 Debug workspace에서 8 MiB worker·1,024-depth 시험이 실제 실행되어 통과한 역사적 snapshot이다. 이후 현재 소스는 해당 test를 Debug assertion 빌드에서 의도적으로 `ignored` 처리하고 optimized profile에서만 실행한다. 최신 run-08 Debug suite의 Rust 618 pass·4 ignored와 Release depth 1/1 결과는 [후속 실행 기록](../c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-08/README.md) 및 [전체 로그](../c12-4-follow-up-verification-2026-10-12/current-source-rerun-20261012-08/full-test.log)에 있다. Debug suite 통과를 현재 1,024-depth worker 검증으로 해석하지 않는다.

**범위:** Taffy Grid typed input·Block/Flex/Grid 계산기 연결과 compact style projection의 Rust/CSS fixture 회귀다. Stylo cascade, V8 document profile, Android/iOS Grid 화면 검증은 이 layout-core 단계에 연결되어 있지 않으므로 여기서 실행·완료했다고 주장하지 않는다.

## 결과

| 명령 | 결과 | 원본 |
| --- | --- | --- |
| `mise exec -- cargo test --locked --workspace` | 575 passed, 0 failed, 3 ignored | [전체 로그](cargo-test-workspace.log) |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 | [Clippy 로그](clippy.log) |
| `mise exec -- cargo fmt --all -- --check` | 통과 | [rustfmt 로그](rustfmt.log) |
| `mise exec -- bun run test:css-reference` | 193 passed, 0 failed | [CSS reference 로그](css-reference.log) |
| `mise exec -- bun run test:css-c11-grid-precomparison` | 4 passed, 0 failed | [C11 사전 비교 로그](c11-precomparison.log) |
| `mise exec -- bun run test:js` | 2 passed, 0 failed | [JS 로그](js-tests.log) |
| `git diff --check` | 통과 | [diff 검사](diff-check.log) |

## 8 MiB worker 깊이 재현과 수정

Grid feature를 추가한 직후 기본 debug workspace 시험에서 `spinon-css-cascade` worker가 depth 1,024 계산 중 stack overflow를 일으켰다. 해당 시험만 분리해도 재현됐다. 진단 과정에서 Taffy `Style` 크기는 grid feature 비활성 artifact 240 bytes, 활성 artifact 552 bytes였고, worker stack을 16 MiB로 잠시 키운 진단 실행은 통과했다. 이 변경은 복구해 저장소의 8 MiB 설정을 유지했다.

계산 트리에서 Taffy 전체 `Style`을 보유하지 않고 Grid named-line·area 필드 등을 제외한 `CompactTaffyStyle`을 저장하게 했다. compact projection 크기가 Taffy `Style`보다 작다는 회귀와 기본 debug 설정의 8 MiB·depth 1,024 worker 실행을 다시 통과했다. 이에 따라 전체 workspace는 575 passed, 0 failed, 3 ignored다. 이 호스트 실행은 Android/iOS simulator나 실기기 결과를 대신하지 않는다.

## 남은 검증 경계

- Grid 계산은 현재 `spinon-layout` typed input에 한정된다. Stylo computed value projection, 신규 runtime profile, document/style/environment revision publication은 미구현이다.
- Android API 37 AVD와 iOS 26.2 Simulator에서 C11 Grid를 V8→Stylo→Taffy→WGPU 경로로 실행하거나 surface pixel을 비교하지 않았다.
- 전체 Grid WPT suite 또는 Spinon WPT runner를 실행하지 않았다. Chrome 고정 reference는 별도 구현 전 기준이다.
- `Vec::try_reserve` fault injection, malicious allocation 경계, GPU hardware 성능은 확인하지 않았다.

이 단계는 C11.1, C11 전체, S02 또는 공개 CSS 지원 완료를 뜻하지 않는다.
