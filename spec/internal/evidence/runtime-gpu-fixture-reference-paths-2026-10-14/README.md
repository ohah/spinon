# Runtime GPU fixture 경로 참조 후속 검증 · 2026-10-14

## 범위

PR #148은 이동된 fixture Rust 파일의 이전 위치 복사본을 제거했다. 이후 전체 CSS reference 테스트에서 일부 테스트와 C11 캡처 입력 목록이 제거된 파일 경로를 계속 사용해, 파일 책임 경계와 테스트가 어긋난 것을 확인했다. 활성 참조를 정식 경로로 고쳤으며 Rust runtime, CSS 동작, C ABI, 플랫폼 코드는 변경하지 않았다.

주요 경로 대응은 다음과 같다.

| 이전 책임 경로 | 현재 정식 경로 |
| --- | --- |
| `runtime_gpu/ffi/c06.rs` 등 fixture별 FFI | `runtime_gpu/ffi/fixtures/c06.rs` 등 |
| `runtime_gpu/ffi.rs`의 fixture registry 기대 | `runtime_gpu/ffi/fixtures.rs` |
| `runtime_gpu.rs`에 있다고 가정한 고정 JS/JSON | `runtime_gpu/fixtures/data.rs` |
| C11 Metal 캡처의 C11 FFI source hash 입력 | `runtime_gpu/ffi/fixtures/c11.rs` |
| surface texture 보고 | `runtime_gpu/renderer.rs` |
| runtime viewport 보고와 orchestration | `runtime_gpu.rs` |

마지막 두 항목은 별도 책임을 유지한다. 테스트가 단순히 새 경로로 문자열만 바꾸지 않고 각 값의 실제 소유 모듈을 확인하도록 수정했다.

## 실행 근거

- `bun install --frozen-lockfile`: 성공 (`parse5@7.3.0`, 209 packages).
- `bun run test:css-reference`: **372 통과, 0 실패, 0 건너뜀**. 원본 출력은 [css-reference-tests.log](./css-reference-tests.log)에 보관했다.
- `node --test tools/css-reference/c04-runtime-css-to-gpu-resize.test.mjs tools/css-reference/c04-runtime-css-to-gpu.test.mjs tools/css-reference/c05-runtime-custom-properties.test.mjs`: 6 통과, 0 실패.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 전체 CSS reference 실행에서 3개 경계 검사가 통과했다.
- negative control: 임시 active `.mjs` 도구에 제거된 `runtime_gpu/ffi/c10.rs` 경로를 기록했을 때 stale-reference 검사가 실패함을 확인하고 임시 파일을 삭제했다. 그 뒤 전체 372개 suite를 다시 통과시켰다.
- 2026-10-14 이전 참조 전체 점검: 일반 CSS reference tool에서 삭제 경로가 남지 않았다. 경계 회귀 테스트의 명시적 이전 경로 목록만 예외이며 그 자체가 검증 대상이다.

## 해석 한계

이번 변경은 테스트와 캡처 도구의 소스 경로 참조를 고친 것이다. 모바일 앱을 새로 빌드하거나 runtime behavior를 재측정하지 않았다. 이 결과는 CSS Grid 지원, Android/iOS release artifact 구성, 전체 CSS 적합성을 판정하지 않는다. [서로 다른 20개 실패 경로 검토](./implementation-review.md)도 이 범위에 한정된다.
