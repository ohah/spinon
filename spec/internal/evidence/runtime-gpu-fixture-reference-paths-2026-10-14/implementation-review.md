# Runtime GPU fixture 경로 참조 후속 구현 검토 · 2026-10-14

## 검토 범위

PR #148에서 삭제한 fixture 소스 경로를 가리키는 CSS reference 테스트와 캡처 입력 목록을 정식 모듈 경로에 맞췄다. Rust 실행 코드·C ABI·fixture 내용은 변경하지 않았다. 아래 검토는 경로 정리만 대상으로 하며 Grid 기능의 정확성이나 제품용 앱 artifact 포함 여부를 판정하지 않는다.

## 서로 다른 실패 경로 20개

| # | 실패 경로 | 확인 결과 |
| --- | --- | --- |
| 1 | C04 fixture 데이터 테스트가 이전 `runtime_gpu.rs` 안에서 `include_str!`를 찾는다 | `runtime_gpu/fixtures/data.rs`를 읽도록 바꾸고 전체 CSS suite로 확인했다. |
| 2 | C04 viewport resize 테스트가 runtime orchestration과 renderer 출력을 한 파일로 가정한다 | runtime 보고는 `runtime_gpu.rs`, surface texture 보고는 `runtime_gpu/renderer.rs`, 입력 소스는 `fixtures/data.rs`에서 각각 확인한다. |
| 3 | C05 사용자 정의 속성 fixture의 include 경로가 이전 Rust 파일 기준을 쓴다 | `fixtures/data.rs`와 새 상대 경로를 확인한다. |
| 4 | C06 viewport fixture 진입점이 이전 `ffi/c06.rs`에만 연결된다 | `ffi/fixtures/c06.rs` 경로로 갱신했다. |
| 5 | C07 border-width와 aspect-ratio 검증이 이전 `ffi/c07.rs`를 읽는다 | 두 테스트가 `ffi/fixtures/c07.rs`를 읽는다. |
| 6 | C07 min/max 검증이 이전 파일 경로뿐 아니라 잘못된 registry를 읽는다 | 구현은 `ffi/fixtures/c07.rs`, fixture registry는 `ffi/fixtures.rs`로 구분했다. |
| 7 | C09 block-formatting 검사에서 include payload가 이전 core 파일에 있다고 가정한다 | 진입점과 payload를 각각 `ffi/fixtures/c09.rs`, `runtime_gpu/fixtures/data.rs`에서 읽는다. |
| 8 | C10 Flex 분배·순서·reverse 검사가 이전 `ffi/c10.rs`를 읽는다 | 세 테스트의 진입점 경로를 `ffi/fixtures/c10.rs`로 갱신했다. |
| 9 | C10 wrap 검사에서 fixture registry와 payload 소유 파일을 혼동한다 | `ffi/fixtures.rs`와 `runtime_gpu/fixtures/data.rs`를 별도로 확인한다. |
| 10 | C11 Metal 캡처 provenance가 삭제된 FFI fixture 파일을 해시한다 | 캡처 입력 목록은 `ffi/fixtures/c11.rs`를 가리키며 실제 경로 해석 테스트를 통과한다. |
| 11 | C11 캡처가 fixture 변경을 더 이상 해시 입력에 포함하지 않는다 | 기존 source-hash 입력 목록에서 파일명만 canonical 경로로 바꿨고, 캡처 검증 테스트를 통과했다. |
| 12 | C12 이전 FFI 경로가 CSS reference 코드에 남는다 | 16개 이전 경로 목록을 능동 도구 파일과 대조하는 회귀 검사를 추가했다. |
| 13 | UA cascade probe 이전 경로가 도구 코드에 남는다 | `probe.rs`와 `layout_probe.rs`의 삭제 경로도 검사 목록에 포함한다. |
| 14 | 새 fixture 경로가 오타 또는 누락으로 존재하지 않는다 | canonical 경로 7개를 양성 검사하며 경계 테스트가 통과했다. |
| 15 | CSS reference 도구 중 한 파일이 이전 경로를 계속 참조한다 | `tools/css-reference/*.mjs` 전체를 스캔하는 회귀 검사를 추가했다. |
| 16 | 회귀 검사가 legacy path를 실제로 탐지하지 못한다 | 임시 `.mjs` negative control에 이전 C10 경로를 넣었을 때 해당 검사만 실패하는 것을 확인한 뒤 파일을 제거했다. |
| 17 | CSS reference 명령에서 새 경계 테스트가 생략된다 | `bun run test:css-reference` 파일 패턴이 경계 테스트를 실행하며 372개 전체가 통과했다. |
| 18 | 테스트가 현재 shell 작업 디렉터리에서만 경로를 해석한다 | 파일 경로는 `import.meta.url` 기준 저장소 루트로 해석하며 저장소 루트에서 전체 suite를 실행했다. |
| 19 | 이전 경로 이름만 검사하고 Rust 파일 자체가 다시 남는 것을 놓친다 | 이전 경로 파일 부재 검사와 canonical 파일 존재 검사를 함께 유지한다. |
| 20 | 경로 정리로 Rust runtime·ABI·Grid 지원 범위가 바뀐 것처럼 기록된다 | 제품 구현 파일, C header, native ABI, support status는 수정하지 않았고 문서에서 기능 완료 주장을 명시적으로 배제했다. |

## 검증 근거

- `bun install --frozen-lockfile` 통과.
- `bun run test:css-reference`: 372 passed, 0 failed, 0 skipped.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 전체 suite에서 실행되어 통과.
- negative control: 임시 CSS reference 도구에 삭제된 C10 FFI 경로를 넣으면 stale-reference 검사가 실패했고, 임시 파일 제거 후 전체 suite가 다시 통과했다.
- 이전 C04/C05 고정 경로 참조 때문에 실패하던 세 테스트는 분리 실행에서 6/6 통과했다.
- 제품 runtime, CSS parser/layout, Rust source, C ABI, Android/iOS native 코드에는 변경이 없다. Rust build 결과를 이 정리의 근거로 주장하지 않는다.

## 판정

경로 참조 후속 정리는 완료됐다. CSS reference 도구에서 제거된 경로를 다시 참조하는 회귀는 자동으로 검출된다. 이 결과는 C11.3.1 또는 전체 CSS 지원 완료를 뜻하지 않는다.
