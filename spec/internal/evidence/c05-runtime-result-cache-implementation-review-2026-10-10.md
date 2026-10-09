# C05.3 runtime 결과 cache 구현 검토와 실행 근거

- **대상:** worker-local runtime cascade/layout 결과 재사용
- **기준 코드:** C05.2 병합 이후 `origin/main`의 `693be438d2b537106a5b14fd41dacb52d89f0a39`
- **계약 버전:** `0.1.0` 유지. 문서 번호 `0035`는 식별자이며 제품·crate 버전이 아니다.
- **비교 기준:** [고정 Chromium detached-node 비교](c05-runtime-result-cache-precomparison-2026-10-10.md)
- **V8:** 저장소가 고정한 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- **계획 검토:** [구현 전 실패 관점 검토](c05-runtime-result-cache-plan-review-2026-10-10.md)
- **판정:** 검토 중 발견한 임의 revision 재표기 경계를 snapshot 증명 방식으로 제한했다. key 차원, 실패·stale 경로, 빈 장면, 두 모바일 시뮬레이터와 release 반복 측정을 보완한 뒤 실행했다.

## 구현 변경과 발견사항

1. cache는 CSS worker마다 성공 결과 한 건만 보유한다. generation·connected render-tree revision·style/environment revision·정확한 viewport bit pattern·media 입력 중 하나라도 다르면 기존 값을 지우고 전체 계산한다.
2. `DocumentRevision`만 달라진 hit은 computed style, layout, render key의 metadata를 현재 `HostDocumentSnapshot`에 맞춰 다시 발행한다. style element, layout frame, renderer box payload는 immutable `Arc` backing을 공유한다.
3. 처음 검토에서 `RuntimeRenderSnapshot::with_document_revision(DocumentRevision)`가 호출자가 임의의 실제 revision을 장면에 붙일 수 있음을 발견했다. 이를 `with_document_snapshot_revision(&HostDocumentSnapshot) -> Option<_>`로 바꿔 generation과 connected render-tree revision이 같은 snapshot만 수용한다. 다른 generation 또는 connected projection이면 실패한다.
4. 빈 HostRoot 장면은 roots 검증이 vacuous하게 통과할 수 있다. 전용 테스트를 추가해 detached-only 변경만 재사용하고 cascade/layout 상태가 계속 `Empty`인지 확인한다.
5. media key 검사는 scheme뿐 아니라 primary pointer·primary hover·all-pointer coarse/fine/hover 입력 각각을 바꾸어 miss가 되는지 확인한다. viewport width·height·device scale의 정확한 float bit 변화도 각각 검증한다.
6. 실패한 cascade, 실패한 layout, stale 완료는 cache에 저장하지 않는다. FFI 완료 JSON과 플랫폼 상태에는 내부 진단 `cacheHit`을 표시한다.

## 구현 실패 관점 검토

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | `DocumentRevision`만 비교해 연결 변화 결과를 재사용 | key는 connected `RenderTreeRevision`도 요구한다. detached hit 및 attach/detach miss 테스트 통과. |
| 2 | 새 document의 같은 NodeId가 이전 계산을 오염 | `DocumentGeneration`을 key에 포함한다. 서로 다른 generation miss 테스트 통과. |
| 3 | 호출자가 임의 document revision으로 renderer scene을 재표기 | snapshot이 기존 generation·render revision과 일치해야 rebind를 허용하도록 수정했고 불일치 테스트 통과. |
| 4 | 연결 style/class/tag/namespace 변경 뒤 낡은 CSS 사용 | connected render projection 변경은 miss. 연결 style mutation 벤치와 attach/detach 테스트 통과. |
| 5 | 삽입·삭제·이동·형제 순서 변경 뒤 오래된 layout frame 사용 | render-tree revision mismatch가 miss를 만든다. attach/detach에서 box 수와 node frame 변화를 확인. |
| 6 | connected text 및 stylesheet source/order 변경이 key에 반영되지 않음 | core projection 비교와 C05.2 stylesheet 순서 테스트가 입력 경계를 보장하며 render revision은 key에 포함된다. |
| 7 | 분리 요소 style 변경으로 연결 selector/layout 결과가 잘못 갱신 | pinned Chrome은 연결 computed style·rect 불변을 보인다. HostDocument detached style mutation은 hit. |
| 8 | viewport width의 아주 작은 bit 차이를 동일 입력 취급 | `to_bits()` key 차이 테스트 통과. |
| 9 | viewport height 또는 device scale이 빠짐 | 두 값을 각각 한 bit 변경하는 key 테스트 통과. |
| 10 | color scheme 이외 pointer·hover media 차원이 빠짐 | primary pointer/hover와 all-pointer capability 각 차원 테스트 통과. |
| 11 | environment revision만 바뀌고 실제 입력이 같을 때 잘못된 hit | environment revision도 key에 포함되며 miss 테스트 통과. |
| 12 | style revision 변경이 key에 빠짐 | 변경한 revision key가 miss하는 단위 테스트 통과. |
| 13 | 계산이 error를 반환한 뒤 예전 성공 결과를 재사용 | mismatch 시 slot을 먼저 비우고 cascade error는 저장되지 않는다. 오류 후 재계산 테스트 통과. |
| 14 | cascade는 성공하고 layout projection만 실패했는데 결과를 cache | failed layout을 반복 계산하는 테스트 통과. |
| 15 | 계산 중 더 최신 요청이 생긴 뒤 늦은 결과가 cache를 덮음 | stale compute 차단 테스트에서 최신 결과만 남고 다음 detached 요청만 hit한다. |
| 16 | worker panic 또는 shutdown 뒤 cache entry가 계속 남음 | cache는 worker 지역 변수이며 panic return/shutdown join에서 drop된다. 기존 worker panic·shutdown 테스트 및 코드 경계를 확인. |
| 17 | roots가 0인 빈 장면에서 빈 iterator 검증 때문에 임의 세대를 재사용 | empty-scene 테스트는 generation/render key와 render snapshot을 검증하며 detached-only 변경만 hit한다. |
| 18 | outer cascade key만 고치고 style/layout/render metadata를 낡게 둠 | style document revision, layout full key, render key를 각각 검사하는 runtime 테스트 통과. |
| 19 | cache와 published result가 대형 배열을 별도로 복사 | style elements, layout frames, renderer boxes의 immutable `Arc` 공유 테스트 통과. |
| 20 | 플랫폼 구현이 Rust fixture와 다른 동작을 함 | Android API 37 emulator와 iPhone 17 Pro iOS 26.2 Simulator에서 실제 V8→WGPU 경로를 실행해 같은 hit/miss 전이를 확인. |

## Release worker latency

각 실행 프로세스에서 16·256·2048 connected node 장면별로 connected-root mutation miss 30회와 detached-only mutation hit 30회를 짝지어 측정했다. 시간은 snapshot 등록부터 cascade와 layout이 terminal 상태가 될 때까지의 wall-clock이며 HostDocument mutation/snapshot 작성 시간은 제외한다. 각 행은 독립 release process 하나의 p50/p95다.

| Connected node | 실행 | Miss p50 / p95 (µs) | Hit p50 / p95 (µs) | payload |
|---:|---:|---:|---:|---|
| 16 | 1 | 103.1 / 162.1 | 3.8 / 7.3 | style/frame `Arc` 공유 |
| 16 | 2 | 106.4 / 161.7 | 3.8 / 7.2 | style/frame `Arc` 공유 |
| 16 | 3 | 104.9 / 153.2 | 3.9 / 6.1 | style/frame `Arc` 공유 |
| 256 | 1 | 1407.2 / 1577.0 | 5.2 / 11.6 | style/frame `Arc` 공유 |
| 256 | 2 | 1422.0 / 1600.0 | 5.1 / 9.2 | style/frame `Arc` 공유 |
| 256 | 3 | 1477.5 / 1611.5 | 4.7 / 9.2 | style/frame `Arc` 공유 |
| 2048 | 1 | 12662.1 / 12980.9 | 9.1 / 15.2 | style/frame `Arc` 공유 |
| 2048 | 2 | 12665.3 / 12912.0 | 10.6 / 14.6 | style/frame `Arc` 공유 |
| 2048 | 3 | 12653.3 / 13216.8 | 9.8 / 16.9 | style/frame `Arc` 공유 |

세 실행 모두 모든 scene 크기에서 hit p50·p95가 miss보다 낮았다. 이 결과는 현재 Mac host의 Rust release worker latency이며 Android/iOS 또는 실기기 성능으로 일반화하지 않는다. `/usr/bin/time -l`이 측정한 benchmark process 최대 RSS는 36,814,848 bytes(약 35.1 MiB)였다. 이는 test binary 기준선과 benchmark 실행 전체를 포함하므로 cache만의 할당량으로 해석할 수 없다. swap 사용은 0이었다.

## Android·iOS 실제 V8 경로

초기 상태 다음에 버튼을 네 번 눌렀다. 각 실행은 실제 V8 fixture를 HostDocument에 적용하고 Stylo/Taffy 결과를 worker에 전달한 뒤 WGPU scene을 다시 그린다.

| 상태 | 동작 | Android API 37 emulator | iOS 26.2 Simulator |
|---|---|---|---|
| 초기 | 기존 connected scene | `cacheHit=false`, 11 boxes, doc/render `57/14` | `cacheHit=false`, 11 boxes, doc/render `57/14` |
| 1 | detached node style 변경 | `true`, 11 boxes, `60/14` | `true`, 11 boxes, `60/14` |
| 2 | detached node attach | `false`, 12 boxes, `61/15` | `false`, 12 boxes, `61/15` |
| 3 | node detach | `false`, 11 boxes, `62/16` | `false`, 11 boxes, `62/16` |
| 4 | detached style 재변경 | `true`, 11 boxes, `63/16` | `true`, 11 boxes, `63/16` |

Android는 API 37 emulator의 WGPU software renderer(ANGLE/SwiftShader)에서 확인했다. iOS는 iPhone 17 Pro / iOS 26.2 Simulator에서 Metal surface와 함께 확인했다. 제품 성능용 hardware 가속·실기기 검증은 하지 않았다.

### 캡처

- Android: [초기](c05-runtime-result-cache-android-api37-initial.png), [1](c05-runtime-result-cache-android-api37-step-1.png), [2](c05-runtime-result-cache-android-api37-step-2.png), [3](c05-runtime-result-cache-android-api37-step-3.png), [4](c05-runtime-result-cache-android-api37-step-4.png)
- iOS: [초기](c05-runtime-result-cache-ios-26.2-initial.png), [1](c05-runtime-result-cache-ios-26.2-step-1.png), [2](c05-runtime-result-cache-ios-26.2-step-2.png), [3](c05-runtime-result-cache-ios-26.2-step-3.png), [4](c05-runtime-result-cache-ios-26.2-step-4.png)

## 자동 검증

| 명령/확인 | 결과 |
|---|---|
| `mise exec -- cargo test --locked --workspace --no-fail-fast --quiet` | 통과: workspace 단위·통합 테스트, runtime 98 passed / benchmark 1 ignored |
| `mise exec -- cargo clippy --workspace --all-targets --locked -- -D warnings` | 통과 |
| `mise exec -- cargo check --locked -p spinon-ffi --all-features` | 통과 |
| `mise exec -- cargo fmt --all --check` 및 `git diff --check` | 통과 |
| `mise exec -- bun run css:verify:c05-result-cache-precomparison` | 통과: detached style 변경 전후 연결 computed style·rectangle 일치 |
| Android API 37 `build:android`, 설치·C05.3 실행 | 통과, 네 전이 모두 확인 |
| iOS Simulator `build:ios-sim`, 설치·C05.3 실행 | 통과, 네 전이 모두 확인 |
| `cargo test --release -p spinon-runtime ... -- --ignored --nocapture --test-threads=1` | 독립 프로세스 3회 모두 통과, 각 scene당 hit/miss 30쌍 |

실기기 성능, CSS 전체 적합성, dirty-subtree 계산, 캐시 메모리 압박 정책은 이 변경에서 검증하거나 지원하지 않는다.
