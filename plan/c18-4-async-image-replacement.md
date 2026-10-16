# C18.4 · 비동기 PNG 교체와 decode 실패 제한 profile

**상태:** 제한 profile 구현·시뮬레이터 검증 완료 · PR [#227](https://github.com/ohah/spinon/pull/227)
**상위:** [C18 이미지·replaced paint 규칙](../spec/STATUS.md) · [내부 계약 0066](../spec/internal/0066-c18-3-object-fit-position.md) · [C18.4 내부 계약 0067](../spec/internal/0067-c18-4-async-image-replacement.md)
**선행 범위:** C14.1–C14.3 intrinsic sizing · C18.1 opaque PNG paint · C18.2 alpha composite · C18.3 `object-fit`
**기반 코드:** [U05.1 PNG decoder](../spec/internal/0061-u05-png-image-decode.md), `ImageRequest`, `ImageResourceSnapshot`, `RuntimeRenderSnapshot`
**규범·oracle:** HTML image request behavior를 관찰한 Agent Browser Chrome 155.0.8059.85 · 고정 PNG 입력

## 목표

현재 HostDocument의 HTML `img[src]` 요청을 비동기 decode 완료와 연결하고, 문서 또는 `src`가 바뀐 뒤 도착한 오래된 completion이 현재 paint를 덮지 않도록 한다. 현재 요청의 PNG decode가 실패하면 그 source의 이전 texture를 유지하지 않고 실패 상태를 기록한다. 다음 정상 source가 완료되면 이미지 paint를 복구한다.

이 단계는 정적 PNG를 앱 fixture 안에서 지연·역순으로 전달하는 내부 profile이다. 제품 URL/network loader, public image API, HTML load/error event, cache policy, alt text와 브라우저별 broken-image icon은 구현하지 않는다. paint 경로는 고정 CSS frame에서 확인하고, 별도 `width:auto; height:auto` 경로로 natural-size completion 뒤 C14 layout frame이 바뀌는지 확인한다. C14·C18·U05와 공개 image 지원은 계속 미완료로 둔다.

## 구현 전 비교 모델

- Chrome 155.0.8059.85를 Agent Browser로 제어해 로컬 threaded HTTP fixture에서 네 PNG 응답 순서와 손상 PNG를 관찰했다. viewport는 80×48 CSS px, DPR 1이며 `<img>` frame은 48×32 CSS px, `object-fit: fill`이다.
- 느린 요청 A(500 ms, 2×2 red)를 시작한 뒤 빠른 B(20 ms, 1×4 blue)로 교체했다. Chrome에는 B의 `load`만 발생했고, A 응답이 도착한 뒤에도 natural size `1×4`, 표본 `[24,64,240,255]`와 현재 src `/b-fast.png`가 유지됐다.
- B에서 느린 C(500 ms, 4×2 green)로 교체한 직후 `complete=false`, `naturalWidth=0`, `naturalHeight=0`이었다. 기존 B의 픽셀은 유지되지 않았다. C decode 후 natural size는 `4×2`, 표본은 `[32,220,80,255]`였다.
- 유효하지 않은 PNG bytes는 `HTMLImageElement.decode()`에서 `EncodingError`로 거부됐다. `error`만 발생하고 `load`는 발생하지 않았으며, 완료 상태에서 natural size는 `0×0`이었다. Chrome은 48×32 frame의 왼쪽 위에 UA broken-image icon을 표시한다. 이 아이콘은 플랫폼별 표현이므로 내부 profile의 pixel contract에는 포함하지 않는다.
- 이후 정상 D(20 ms, 2×1 purple)는 `load`되어 natural size `2×1`, 표본 `[200,40,220,255]`로 복구됐다.
- 별도 `width:auto; height:auto` 확인에서 첫 A request pending 동안 natural size와 frame은 `0×0`이었다. A decode 뒤 natural size와 frame은 `2×2`가 됐다. 이후 C request가 pending일 때 natural size는 `0×0`이지만 frame은 이전 `2×2`로 남았고, C decode 뒤 natural size와 frame은 `4×2`로 갱신됐다. 추가 pending screenshot에서는 C decode 뒤 A로 바꾼 상태에서 이전 C 픽셀 대신 빈 표면이 보였고 frame은 이전 `4×2`를 유지했다. 손상 요청의 UA auto-size fallback frame은 현재 profile 계약에 넣지 않는다.
- 원본 JSON, 실패 화면, threaded fixture와 SHA-256은 [Chrome 비교 근거](../spec/internal/evidence/c18-4-async-image-oracle-2026-10-16/README.md)에 고정했다.

이 oracle은 HTML request lifecycle 관찰이다. 앱 fixture는 같은 순서와 바이트를 사용해 request identity, decoded source, frame, rendered pixel을 검사하되, Chrome event/API 또는 broken-image icon의 복제를 주장하지 않는다.

## 제한 profile 계약

1. 각 decode 작업은 시작 시점의 `HostDocument` generation/document/render-tree stamp, `img` node와 `src`를 소유한다. bytes와 decode 결과는 그 요청에만 귀속한다.
2. 현재 document 또는 `src`가 달라진 뒤 도착한 성공·실패 결과는 stale로 분류하고 새 resource snapshot이나 render cache를 변경하지 않는다. stale 결과 거부는 새 요청 성공과 별도로 기록한다.
3. 현재 요청의 decode가 진행 중이면 그 `src`에 연결된 decoded image를 제출하지 않는다. 기존 source texture를 새 source의 대용으로 남겨두지 않는다. 고정 frame과 주변 paint는 유지한다. auto-size 경로는 첫 성공 전 intrinsic size 0×0을 사용하고, 직전 성공 뒤에는 그 natural-size 값만 새 document source stamp에 묶인 measurement로 보존한다. 이전 source의 RGBA bytes, request identity 또는 GPU texture는 새 stamp로 옮기지 않는다.
4. 현재 요청의 PNG decode 성공은 증가한 resource revision의 immutable image snapshot에 반영한다. RGBA bytes, dimensions와 source stamp는 해당 completion에서만 가져온다. auto-size 이미지의 새 natural size는 새 intrinsic snapshot과 layout frame으로 전파한다.
5. 현재 요청의 decode 실패는 node/source/revision에 바인딩된 구체 오류 결과로 기록하고 이미지 paint resource를 비운다. 실패가 document, 다른 node, renderer worker 또는 다음 source의 처리를 중단하지 않는다.
6. 새 source로 바뀐 뒤 성공하면 그 source의 decoded bytes만 그린다. 실패 뒤 새 source 성공도 같은 경로로 복구한다. 이전 source의 늦은 성공·실패는 새 성공·실패를 역전하지 못한다.
7. renderer resource key/cache revision이 교체·실패·복구 순서에서 일치해야 한다. 이미지가 없는 snapshot을 제출하면 이전 texture entry가 제거되고, 다른 source bytes가 기존 key로 재사용되지 않는다.
8. fixed-frame paint track은 CSS 48×32 px content box로 intrinsic sizing과 paint 결과를 분리한다. 별도 auto-size track은 C14 intrinsic geometry만 측정한다. C18.1 opaque, C18.2 alpha, C18.3 object-fit 회귀 입력도 같은 revision 경계에서 보존한다.
9. malformed PNG, 잘못된 dimensions, stale generation/document/tree/src, duplicate node completion, resource revision 역행·재사용, 채널/byte 길이 오류와 비동기 worker 종료 경합은 frame 제출 전에 거부하거나 기록한다. 제한을 조용히 기본 이미지로 바꾸지 않는다.

## 실행 입력과 판정 기준

고정 입력은 위 oracle의 opaque PNG 4종(A 2×2, B 1×4, C 4×2, D 2×1) 및 `image/png`로 전달되는 잘못된 byte sequence다. fixture runner는 A를 지연시키고 B를 먼저 완료한 뒤 A를 전달한다. 이후 B→C pending→C 완료, C→손상 decode 실패, 실패→D 회복을 순서대로 실행한다. A와 C의 auto-size subcase는 각각 pending 및 완료 단계의 intrinsic snapshot과 Taffy frame도 대조한다. fixture task는 시간 sleep에 결과 순서를 맡기지 않고 barrier/channel gate로 B 완료 후 A를 공개한다. 테스트용 지연과 실행 입력은 전용 opt-in fixture-test 경로에만 둔다.

판정은 단계별 document/source/resource revision, request→completion identity, stale/error 분류, image count/hash, CSS frame, GPU draw 수와 RGBA 표본으로 분리한다. 성공 PNG의 고유 단색 표본은 Android API 37 ARM64 AVD PixelCopy 및 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 채널당 오차 0이어야 한다. pending·fixed-frame failure에서는 고정 배경색이 보이고 image draw가 0이어야 한다. auto-size 첫 pending은 intrinsic frame 0×0, 첫 성공은 A natural size 2×2, 성공 후 교체 pending은 직전 natural-size frame 2×2, 새 성공은 C natural size 4×2여야 한다. pending measurement는 새 document stamp를 가지며 이전 image bytes와 draw key를 포함하지 않아야 한다. 오래된 A 결과와 손상 PNG는 현재 resource에 포함되지 않아야 한다. 마지막 D 단계는 새 bytes의 purple 표본으로 복구되어야 한다.

## 구현·검증 관문

1. `spinon-image` 요청/result 모델에서 성공과 decode 실패를 같은 request identity에 귀속하고, stale 결과의 처리 정책을 명시한다. 현재 `ImageResourceSnapshot`와 C14 layout snapshot의 source/revision 계약을 유지한다. pending auto-size의 retained dimensions는 source revision을 현재 문서로 바꾼 크기 전용 입력이며, 기존 request/source key의 이미지 RGBA를 재바인딩하지 않는다.
2. fixture 전용 runner에서 실제 PNG decoder와 비동기 task completion 순서를 사용한다. fixture 코드·정적 PNG·FFI symbol은 전용 `c18-4-image-fixture-test` feature 아래에 두며 기본/제품 artifact에는 포함하지 않는다.
3. Rust 단위 검사는 오래된 src 성공·실패, stale document/tree/generation, valid→pending→success, valid→decode failure, failure→recovery, 다른 node 독립성, 잘못된 PNG/크기, 중복 completion, task panic/cancel과 revision overflow를 다룬다. 각 핵심 guard를 제거하는 negative control은 해당 검사가 실패해야 한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source sequence를 최신 소스로 재빌드해 실행한다. 단계별 pixel/screenshot 및 event/report를 저장한다. AVD software renderer와 iOS compositor 전 Metal drawable을 실기기·hardware GPU·compositor-after 결과로 확대 해석하지 않는다.
5. C18.1 opaque, C18.2 alpha와 C18.3 fit/position 픽셀을 두 플랫폼에서 재실행한다. 기본·opt-in Cargo/static library와 실제 APK/iOS app executable을 비교해 새 fixture symbol·JS·PNG bytes의 기본 artifact 부재 및 opt-in 양성 대조를 확인한다.
6. `cargo fmt --all -- --check`, workspace tests, Clippy, 변경된 JS/CSS reference 및 fixture module-boundary 검사를 실행하고 원본 로그를 보존한다. 기존 C11/C13 source-boundary assertion 실패가 재발하면 별도 귀속하고 C18.4 통과로 숨기지 않는다.
7. 구현 전 계획 자체를 서로 다른 실패 관점 20개로 검토했다([계획 검토 20개](../spec/internal/evidence/c18-4-async-image-replacement-plan-review-20-2026-10-16.md)). 구현 뒤 새 20개 구현 검토와 [PR diff 독립 검토 30개](../spec/internal/evidence/c18-4-async-image-replacement-2026-10-16/pr-delta-review-30.md)를 기록하고 발견된 결함을 재실행으로 닫았다.
8. 내부 `spec/STATUS.md`에는 C18.4 제한 profile만 연결한다. URL/network loader, HTML events, broken-image icon, 실패 시 auto-size UA fallback, public API와 C18 전체 완료를 주장하지 않는다. Tailnet preview에 계획·구현 근거를 동기화하고 URL, HTTP status와 served hash를 확인한다. GitHub Pages는 배포하지 않는다.

## 구현 결과 · 2026-10-16

- `spinon-image`에서 decode 성공·실패를 원 요청 identity와 함께 보존하고, 현재 HostDocument/source와 맞지 않는 completion을 따로 거부한다. 새 immutable image snapshot은 decode 실패 node의 paint image를 비운다. pending auto-size 입력은 이전 natural dimensions만 현재 source stamp에 전달하며 RGBA·request/cache key를 복사하지 않는다.
- 앱 실행 fixture는 `c18-4-image-fixture-test` opt-in 아래에서 gated worker와 실제 PNG decoder를 사용한다. Android C18.4-only APK와 iOS Simulator C18.4-only executable에서 V8 route, 네 PNG payload와 adapter를 확인했고, default artifact에서 전용 symbol·adapter·JS/PNG payload가 빠지는 양성·음성 대조를 남겼다.
- Android API 37 ARM64 AVD의 PixelCopy 6/6, iPhone 17 Pro/iOS 26.2 Simulator의 compositor 제출 전 Metal drawable 6/6이 exact다. C18.1/2/3 runtime 회귀도 두 플랫폼에서 각 9/9, 10/10, 19/19다. [실행 근거·화면·로그](../spec/internal/evidence/c18-4-async-image-replacement-2026-10-16/README.md)와 [artifact 근거](../spec/internal/evidence/c18-4-async-image-replacement-2026-10-16/artifact-boundary/README.md)를 참조한다.
- 전체 CSS reference suite는 498개 중 494개 통과했고 네 C11/C13 source-boundary assertion이 실패했다. 같은 네 실패가 구현 전 `origin/main` snapshot에서 재현됐다. C18.4 module-boundary와 iOS build-flag 집중 검사는 각각 통과했다. 이 baseline 실패는 C18.4 회귀로 보고하지 않는다.
- 계획 검토 20개는 구현 전에 별도로 완료했다. 구현 검토 20개와 PR diff 검토 30개는 각자 다른 실패 관점으로 기록한다. 제한 profile 외의 URL/network loader, HTML events, browser broken-image icon, 실패 auto-size fallback, 공개 image API와 C14/C18/U05 전체는 구현하지 않았다.
