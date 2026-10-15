# R07 Vue·Svelte 공통 호스트 어댑터 가능성

## 판정

고정한 counter 사례에서 Vue custom renderer와 Svelte client compiler output이 모두 동일한 제한 Element/Text host projection을 만들고 갱신했다. 따라서 React 전용 타입 없이 Vue와 Svelte가 같은 호스트 노드 계약에 연결될 **가능성 조사**는 통과했다. 이 결과는 제품 어댑터 지원이나 실제 Rust FFI 통합을 뜻하지 않는다.

## 비교 기준과 실행 환경

- 사전 고정 oracle: [`oracle.json`](../r07-framework-adapter-precomparison-2026-10-16/oracle.json) — mount, click 1회, click 2회, unmount의 정확한 tree와 횟수.
- fixture: [`vue-counter.mjs`](../../../../spikes/framework-adapters/fixtures/vue-counter.mjs), [`svelte-counter.svelte`](../../../../spikes/framework-adapters/fixtures/svelte-counter.svelte).
- 실행 결과 원본: [`observation.json`](observation.json). 각 단계 tree와 operation trace, owner token, framework 버전을 담는다.
- 소스: Vue `@vue/runtime-core` `3.5.44` custom renderer; Svelte `5.57.2`의 `generate: "client"` output; static template parsing은 `parse5` `8.0.1`.
- 의존성은 [`package.json`](../../../../spikes/framework-adapters/package.json)과 [`bun.lock`](../../../../spikes/framework-adapters/bun.lock)에 정확히 고정됐다. Bun `1.4.0`, Node `v26.7.0`, macOS `26.5.1` arm64에서 실행했다.
- 기준 API 근거: [Vue Custom Renderer API](https://vuejs.org/api/custom-renderer), [Svelte compiler](https://svelte.dev/docs/svelte/svelte-compiler).

## 실행

```sh
cd spikes/framework-adapters
bun install --frozen-lockfile
bun run test
bun run capture > ../../spec/internal/evidence/r07-framework-adapter-2026-10-16/observation.json
```

`bun run test` 결과는 2 passed, 0 failed다. `bun run capture`를 연속 실행해 `observation.json`과 byte-for-byte 동일한 결과도 확인했다. 첫 검증에서 발견한 실행 하니스 오류는 다음과 같이 수정했다.

- Node 기본 export가 Svelte server entry를 골라 `lifecycle_function_unavailable`로 끝나던 문제를 `--conditions=browser`로 고정했다.
- 임시 compiler module을 OS 임시 폴더에 두자 spike 의존성을 찾지 못해 spike 디렉터리 안에서 생성 후 제거하도록 바꿨다.
- Vue harness에 문서 객체가 없는 경로에서 event path를 순회하던 null 접근을 고쳤다.
- operation label `SyntheticEventDispatch`가 React의 `SyntheticEvent` 타입으로 오해되지 않도록 `HarnessClickDispatch`로 바꿨다.

별도로 실제 Rust `HostDocument`의 소유권 거부 경계 두 건도 실행했다.

- `cargo test -p spinon-core owner_conflict_and_cycles_reject_the_whole_batch` — 1 passed.
- `cargo test -p spinon-core cross_owner_edges_and_non_element_parents_fail_closed` — 1 passed.

## 관찰값

두 framework 모두 사전 oracle과 정확히 일치했다.

- mount: `<main><button aria-label="count">count: 0</button></main>`, click 0회.
- 첫 click 뒤: `count: 1`, click 1회.
- 두 번째 click 뒤: `count: 2`, click 2회. Vue는 새 callback을 등록하고 이전 callback을 제거해 한 번씩만 증가했다. Svelte delegated root/document listener도 같은 상태를 만들었다.
- unmount: HostRoot 자식 0개. detached target click은 `DetachedTarget`으로 거부했다.
- 두 실행의 operation 17개 전부 명시된 owner token을 보존했다. Vue와 Svelte의 개별 operation 순서는 달랐고 정규화 tree는 같았다.
- Svelte root/document delegated listener는 unmount 뒤 각각 0개다. 분리된 객체에 framework callback closure가 남는지 또는 GC되는지는 확인하지 않았다.
- Svelte compiler warning은 0개, client output은 852 bytes, runes mode였다.

`observation.json`의 `CreateElement`, `CreateText`, `InsertBefore`, `RemoveChild`, `SetTextData`, `SetAttribute` 이름은 Rust `DocumentOperation` 표면과 대응한다. `AdapterListenerAdd`, `AdapterListenerRemove`, `HarnessClickDispatch`는 이 spike의 계측용 항목이며 Rust `HostDocument` operation이라는 뜻이 아니다. JS recorder의 string owner token은 Rust `OwnerId` ABI 연결이 아니다. 기존 Rust ownership tests는 실제 코어 경계를 별도로 확인한다.

## 범위와 남은 계약

실험은 한 루트, `<main>`, `<button>`, 문자열 `aria-label`, synthetic click 두 번만 포함한다. harness는 한 owner의 변경, 지원 속성, Element/Text 투영을 제한적으로 흉내 낸다. core `HostDocument` 파일에서 React 전용 이름은 검색되지 않았고 spike에도 React dependency가 없다.

Svelte는 custom renderer API가 아니라 client output이 요구하는 일부 DOM-shaped facade로 연결했다. 다음은 검증하지 않았다: 실제 FFI/HostDocument commit 연결, 공개 `OwnerId` 수명, React adapter와의 동등성, 다중 루트·fragment·list, DOM property와 event propagation 전 범위, CSS/layout/GPU 화면, 접근성, Android/iOS Simulator·실기기, framework 오류와 callback closure 회수. 따라서 R03 및 제품 Vue/Svelte 지원은 미완료다.

## 재현성 checksum

| 파일 | SHA-256 |
|---|---|
| `spikes/framework-adapters/bun.lock` | `afc7696902e427c094f01e70e5466dcfe07899ae234246dfca2ae323b0d9e2f4` |
| `spikes/framework-adapters/fixtures/vue-counter.mjs` | `fa1a35382b409d8ab1bde62f4c6c4ece8e82920b38db7f7795d562d41ab8ca8d` |
| `spikes/framework-adapters/fixtures/svelte-counter.svelte` | `3f0dd936f90f7c09cdce6297d65bc3e0b9d4e4a060c1baa95c655315aa714fb2` |
| `spec/internal/evidence/r07-framework-adapter-precomparison-2026-10-16/oracle.json` | `287f05410e6b57cec6effa0498594dacde0e070a88e39e0c66a171512f52014d` |
| `spec/internal/evidence/r07-framework-adapter-2026-10-16/observation.json` | `ec3eaea806e0c2837f7d555bd2cb544a46081dfde3470dd946dff027392465bc` |

## 검토 기록

- [구현·실행 적대 검토 20개](implementation-review-20.md)
- [계획 적대 검토 20개](../r07-framework-adapter-plan-review-20-2026-10-16.md)
- [PR 변경 적대 검토 30개](pr-delta-review-30.md)
