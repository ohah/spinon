# Runtime GPU fixture 이전 경로 정리 · 2026-10-14

## 발견과 수정

PR #147은 실행 중인 fixture 모듈을 `fixtures/`로 옮기고 feature gate를 적용했지만, 이전 위치의 복사본 16개를 삭제하지 않았다. 소스 검색으로 해당 파일들이 새 module declaration 또는 `#[path]`에서 참조되지 않는 것을 확인했다. 현재 빌드에는 들어가지 않더라도 두 벌의 유사한 코드가 저장소에 남아 이후 수정 위치를 혼동시키므로 이전 경로의 사본을 제거했다.

삭제한 경로:

- `crates/spinon-ffi/src/runtime_gpu/c11.rs`
- `crates/spinon-ffi/src/runtime_gpu/c11_2_2.rs`
- `crates/spinon-ffi/src/runtime_gpu/c11_2_3.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c06.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c07.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c08.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c09.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c10.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c10_3_5.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c11.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_1.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_2.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_3.rs`
- `crates/spinon-ffi/src/runtime_gpu/ffi/c12.rs`
- `crates/spinon-ffi/src/runtime_ua_cascade/probe.rs`
- `crates/spinon-ffi/src/runtime_ua_cascade/layout_probe.rs`

활성 fixture 구현은 `runtime_gpu/fixtures/`, `runtime_gpu/ffi/fixtures/`, `runtime_ua_cascade/fixtures/` 아래에서 유지한다. 기본 제품 feature와 앱 fixture feature 조합은 앞선 PR의 artifact evidence에 기록되어 있다. 이번 변경은 모듈 선언이나 앱 ABI를 변경하지 않는다.

## 회귀 방지

`tools/css-reference/fixture-module-boundary.test.mjs`는 이전 경로의 16개 파일이 다시 생기지 않는지 검사한다. Rust fixture module declarations는 새 디렉터리 구조를 계속 가리키고, 순수 Rust unit-test helper는 기존 `#[cfg(test)]` 경계에서 유지한다.

## 검증 결과

| 검사 | 결과 |
| --- | --- |
| `node --test tools/css-reference/fixture-module-boundary.test.mjs tools/css-reference/c11-ios-build-flags.test.mjs` | 11 통과, 0 실패 |
| `cargo fmt --all -- --check` | 통과 |
| `cargo test --locked -p spinon-ffi --no-default-features` | 18 통과, 0 실패 |
| fixture feature 조합 `cargo test` | 43 통과, 2 무시, 0 실패 |
| 기본 feature `cargo clippy --all-targets -- -D warnings` | 통과 |
| fixture feature 조합 `cargo clippy --all-targets -- -D warnings` | 통과 |

fixture feature 조합은 `c04-runtime-gpu-fixture-test,c11-2-3-grid-fixture-test,c04-runtime-ua-cascade-fixture-test`다. 테스트는 Rust 기본 모듈과 앱 fixture 모듈이 새 경로에서 각각 컴파일되는지 확인한다. 전체 workspace 및 Android/iOS 앱 빌드는 이번 파일 경로 정리에서 실행하지 않았다.

## 적대적 검토

1. 이전 위치 파일 중 실제 모듈로 선언된 파일이 있어 삭제 뒤 컴파일이 깨지는가 — `mod` 선언과 `#[path]` 참조를 검색해 삭제 대상이 참조되지 않음을 확인했다.
2. 삭제 대상에만 있던 동작이 정식 경로 사본에 빠졌는가 — 16개 대응 파일의 차이를 검토하고 정식 경로에 추가된 API와 구현을 확인했다.
3. `include_str!` 상대 경로가 삭제된 파일에 의존하는가 — 활성 자료 모듈과 adapter를 두 feature 조합에서 실제 컴파일했다.
4. FFI 내보내기 함수가 고아 파일에만 정의되어 있었는가 — fixture 기능 테스트에서 활성 FFI 진입점들을 컴파일하고 단위 검사를 실행했다.
5. 기본 제품 FFI에서 fixture 함수가 사라지는가 — fixture 기능을 끈 기본 테스트와 Clippy를 별도로 실행했다.
6. 앱 fixture 기능에서 필요한 harness가 누락되는가 — GPU·UA cascade fixture 기능을 켠 전체 FFI 테스트와 Clippy를 실행했다.
7. Android/iOS 빌드가 삭제된 Rust 파일 경로를 직접 참조하는가 — 빌드 스크립트와 헤더 검사를 살펴보고 구현 파일 경로 직접 참조가 없음을 확인했다.
8. C 헤더 선언과 Rust 심볼 이름이 바뀌는가 — 이번 변경에서 C 헤더와 ABI 선언은 수정하지 않았다.
9. `#[cfg(test)]` 전용 데이터도 삭제되는가 — 삭제 대상 16개는 앱 검증용 FFI 복사본이며, 순수 Rust 테스트 모듈·자료 경로는 변경하지 않았다.
10. 제품 surface 보조 모듈이 비슷한 이름 때문에 함께 삭제되는가 — `runtime_gpu/ffi/surface.rs`와 `runtime_gpu/ffi/fixtures/surface.rs`를 구별해 둘 다 유지했다.
11. C11.3.1 후보 기능이 fixture 정리 PR에 섞이는가 — 후보를 현재 `main`에서 추출했으며 C11.3.1 구현 변경은 포함되지 않았다.
12. 새 회귀 검사가 기본 Node 테스트 명령에서 빠지는가 — `test:css-reference`의 파일 일괄 패턴이 새 `*.test.mjs`를 실행함을 확인했다.
13. 검사 코드가 저장소 루트를 잘못 계산하는가 — 테스트 파일 위치에서 두 단계 위 경로를 계산하고 실제 두 테스트 파일을 실행했다.
14. 이전 경로만 부정 검사하고 정식 경로의 누락을 놓치는가 — 테스트에 과거 경로 부재와 정식 fixture 파일 존재의 양성 검사를 모두 두었다.
15. 테스트가 실행 당시 작업 디렉터리에 의존하는가 — 모든 파일 경로를 테스트 모듈의 URL에서 계산한다.
16. 구현 검토 기록이 기존 파일을 발견하지 못했다고 잘못 주장하는가 — 이전 점검의 누락을 기록하고 후속 정리 근거를 연결했다.
17. 소스를 옮긴 뒤 이전 사본을 남겨도 된다고 규칙이 해석되는가 — 프로젝트 규칙에 이전 복사본 삭제와 회귀 검사 요구를 추가했다.
18. 파일 삭제가 공개 동작이나 ABI를 바꾸는가 — 선언·심볼·빌드 스크립트·플랫폼 파일은 수정하지 않았고 기본·fixture 기능 컴파일도 통과했다.
19. 목록에 없는 고아 fixture 파일이 더 남아 있는가 — 전체 Rust 경로 목록을 다시 검색하고 회귀 검사 대상 16개 경로를 대조했다.
20. 임시 후보 작업이 사용자의 미커밋 브랜치를 변경하는가 — 모든 수정과 검사는 `main`에서 추출한 임시 디렉터리에서 실행했다.

검토에서 수정한 항목은 이전 경로 사본 삭제, 회귀 테스트 추가, 규칙·계획·이전 검토 근거의 정정이다. 위 결과는 이 정리 후보에서 확인했다.
