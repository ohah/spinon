# C05.3 구현 전 비교 기준: detached-only document revision

## 기준 실행

- 저장소 기준: C05.2 병합 직후 `693be438d2b537106a5b14fd41dacb52d89f0a39`.
- 독립 browser oracle: Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- HTML: [`runtime-registered-properties.html`](../../../tests/fixtures/css/c05/runtime-registered-properties.html), viewport `301×100` CSS px, scale `1`, locale `en-US`, timezone `UTC`.
- 관찰 대상: `app`, `declared`, `inherited`의 computed width와 `getBoundingClientRect()`.

## Chromium 관찰

fixture를 읽은 뒤 문서에 연결하지 않은 `div`를 만들고 inline `width`, `height`, `--probe`를 두 번 변경했다. 그 전후 연결 대상 값은 변하지 않았다.

| node | computed width | x | y | width | height |
| --- | --- | ---: | ---: | ---: | ---: |
| `app` | `301px` | 0 | 0 | 301 | 100 |
| `declared` | `41px` | 0 | 0 | 41 | 14 |
| `inherited` | `19px` | 102 | 16 | 19 | 14 |

detached 요소의 `isConnected`는 `false`, rectangle은 `{x:0,y:0,width:0,height:0}`였다. 이는 연결 장면의 cascade·layout·paint 입력이 detached style 변경으로 달라지지 않는다는 기준이다. 실행기는 [`verify-c05-runtime-result-cache-precomparison.mjs`](../../../tools/css-reference/verify-c05-runtime-result-cache-precomparison.mjs)이며, 고정 Chrome·fixture 조건에서 반복할 수 있다.

## 현재 runtime 동작과 판정

`HostDocument::commit`은 detached subtree 수정에도 `DocumentRevision`을 올리지만, `same_render_projection`은 HostRoot 연결 노드만 비교하므로 연결 내용이 그대로면 `RenderTreeRevision`을 보존한다. 현재 runtime의 `register_document_snapshot → request_latest → worker_loop → compute_request` 경로는 render revision이 그대로인 경우도 새 cascade/layout 계산으로 전달한다.

C05.3의 사전 판정은 cache miss 기준 결과와 detached-only cache hit 결과의 연결 style·frame·paint 값이 같고, 모든 published revision이 새 document key로 재발행되는 것이다. baseline은 각 snapshot 등록당 계산 함수 호출 1회다. 성능은 기능을 구현한 뒤 동일 입력 반복으로 p50/p95를 측정하며, 이 사전 비교 자체로 속도 개선을 주장하지 않는다.
