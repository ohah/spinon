# C18.4 구현 PR diff 적대 검토 30개

**대상:** PR [#227](https://github.com/ohah/spinon/pull/227), 기준 `e8bec277a3241a2b0beae52679d6b8dcb947438d`, 초기 diff `e4b9f8c59128a8c6ab3691033f83dc5c63a9d84d`
**검토 시점:** 2026-10-16 · 계획 검토 20개와 구현 검토 20개는 재사용하지 않음
**판정:** 아래 30개 변경 경계 확인. Agent Browser에서 PR 본문의 Android/iOS 이미지가 실제 원본 크기로 모두 로드됨을 확인했다. GitHub는 check run 0개를 보고했으며 현재 branch protection에는 required status checks/approval 개수가 설정되어 있지 않다. 로컬 gate 결과는 실행 근거에 별도 기록했다.

| # | PR diff에서 확인한 독립 실패 조건 | 확인한 변경·근거 | 판정 |
| ---: | --- | --- | --- |
| 1 | 세부 C18.4를 체크하며 C18/C14/U05 상위를 완료로 오인하는가 | `spec/STATUS.md`에서 C18.4만 체크했고 parent C18과 C14/U05는 unchecked다. | 통과 |
| 2 | 내부 계약의 출시 전 숫자 버전이 문서 구현마다 올라가는가 | `0067`은 프로젝트 규칙에 맞춰 `0.1.0`으로 고정했다. | 통과 |
| 3 | 관찰한 Chrome behavior와 Spinon 제품 보장을 섞는가 | 계획과 계약이 고정 fixture scope 및 network/API/HTML event/icon 미지원 범위를 분리한다. | 통과 |
| 4 | decode 결과가 다른 node나 src 요청에 연결되는가 | `ImageResourceOutcome`에 요청 identity와 decode result를 함께 소유시킨다. | 통과 |
| 5 | 같은 src의 이전 document/tree revision completion을 수용하는가 | generation/document/render-tree tuple를 확인하며 같은-src revision 회귀 test와 stale mutation 기록이 있다. | 통과 |
| 6 | src 교체 후 늦은 성공이 최신 source를 덮는가 | B 완료 뒤 A 완료를 넘겨 `SourceChanged`로 분류하고 B resource만 남기는 단위·fixture report가 있다. | 통과 |
| 7 | src 교체 후 늦은 failure가 최신 image를 지우는가 | 같은 request-bound stale 검사가 성공·실패 outcome 모두에 적용되고 late-failure test가 있다. | 통과 |
| 8 | detached, SVG 또는 `src` 없는 node가 HTML image resource로 들어가는가 | request 생성 시 connected HTML namespace `img`, nonempty bounded src를 검증한다. | 통과 |
| 9 | 같은 node의 성공과 실패가 같은 snapshot에 두 번 적용되는가 | duplicate node를 오류 처리하며 guard 제거 negative control에서 test가 실패했다. | 통과 |
| 10 | failure outcome을 세지 않아 snapshot item 제한이 우회되는가 | success/failure를 공통 accepted outcome 수로 제한하고 boundary test가 있다. | 통과 |
| 11 | 이미지 byte 합계 계산이 integer overflow 또는 64 MiB 초과를 허용하는가 | checked addition과 최대치·overflow 단위 test를 확인했다. | 통과 |
| 12 | malformed PNG가 빈 성공이나 이전 bytes로 위장되는가 | 실제 decoder error를 request identity에 붙이고 오류 snapshot entry로 유지한다. | 통과 |
| 13 | 현재 source decode failure 뒤 과거 image draw가 다시 제출되는가 | 새 resource snapshot은 해당 node image를 비우고 renderer의 빈 map publish path가 이전 texture cache를 제거한다. | 통과 |
| 14 | failure state가 다음 정상 source의 성공을 막는가 | 실패 이후 D completion으로 오류 map을 비우고 새 PNG를 복구하는 test/report가 있다. | 통과 |
| 15 | image 하나의 실패가 독립된 다른 node를 중단시키는가 | same snapshot의 한-node failure·다른-node success test가 있다. | 통과 |
| 16 | 고정 frame pending 동안 이전 source pixel이 보이는가 | C pending snapshot의 image count와 draw는 0이고 배경 표본만 남는다. | 통과 |
| 17 | auto-size pending에서 이전 source의 RGBA/cache key가 새 source에 붙는가 | pending layout snapshot은 dimensions만 현재 stamp에 다시 묶으며 image map은 비어 있다. | 통과 |
| 18 | decode 완료 후 natural size 변경이 Taffy frame까지 전달되지 않는가 | A 2×2 뒤 C 4×2 auto-size frame을 Rust와 양 플랫폼 report에서 확인했다. | 통과 |
| 19 | 첫 pending 0×0이 NaN ratio나 잘못된 layout error를 만드는가 | 0 natural dimension은 ratio 계산에서 제외된다. guard 완화 negative control에서 layout test가 실패했다. | 통과 |
| 20 | worker 취소/panic 때 pending state 이후 completion이 publish되거나 thread가 남는가 | started acknowledgment, cancellation gate, join과 결과 disconnect를 opt-in fixture test로 확인했다. | 통과 |
| 21 | renderer resource key가 src/revision 교체 때 이전 texture를 alias하는가 | snapshot resource revision이 증가하고 image 없는 publish가 기존 draw entry 제거를 수행한다. C18.4/C18.1–3 표면 회귀가 확인한다. | 통과 |
| 22 | FFI 호출의 output buffer 또는 host 포인터 오류가 Rust dereference로 이어지는가 | C18.4 FFI는 null/zero output과 null host를 dereference 전에 거부하고 header에 buffer capacity·host lifetime 계약이 있다. | 통과 |
| 23 | readback sample 수·null 좌표·중복 좌표가 고정 길이 slice를 오용하는가 | 정확히 6쌍을 요구하고 null/duplicate를 검사하며 unsafe slice 길이를 12 `u32`로 제한한다. | 통과 |
| 24 | FFI header 선언만 기본 제품 ABI에 노출되는가 | 선언과 Rust module이 `c18-4-image-fixture-test` cfg 아래에서만 연결된다. | 통과 |
| 25 | fixture 경로 이름만 믿고 기본 artifact payload 부재를 주장하는가 | 기본/opt-in Cargo staticlib, Android APK native·DEX, iOS executable을 직접 검사했고 opt-in positive control을 함께 보존했다. | 통과 |
| 26 | Android/iOS/ Cargo feature flag의 오타·C04 runtime 의존성 불일치가 조용히 통과하는가 | Android Gradle 및 native/iOS build script가 0/1과 C04=1 선행 조건을 검사한다. | 통과 |
| 27 | Android opt-in adapter class가 default DEX에 포함되는가 | dedicated source set만 flag=1에서 포함되며 default/opt-in DEX class 정의를 비교했다. | 통과 |
| 28 | Android route가 다른 C18 route로 흐르거나 V8 style과 다른 frame을 사용해도 sample이 통과하는가 | C18.4 intent, canvas 224×88 CSS px, revision-matched computed root style 및 fresh PixelCopy report를 대조했다. | 통과 |
| 29 | iOS route가 AppDelegate에서 누락되거나 Swift/ObjC++/C ABI flag가 어긋나는가 | AppDelegate route, Xcode define, build script, header와 `SpinonRunner` bridge를 함께 확인하고 iOS Simulator에서 실행했다. | 통과 |
| 30 | PR 화면 링크가 markdown text만 있고 이미지가 실제로 깨져 있는가 | Agent Browser 0.31.1 PR 화면에서 C18.4 Android/iOS와 C18.1 visible-overflow Android/iOS 총 4개 이미지 모두 `naturalWidth>0`·`complete=true`를 확인하고 [`PR 화면 캡처`](./pr-227-agent-browser.png)를 저장했다. | 통과 |

## 병합 전 현재 상태

- GitHub PR `#227`은 2026-10-16 확인 시 `MERGEABLE`, check runs 0, review approval required 0이다. 저장소 branch protection은 linear history를 요구한다. PR은 `Rebase and merge` 방식으로 처리한다.
- 로컬 `cargo test --locked --workspace`는 900 passed·4 ignored, C18.4 opt-in fixture tests 3/3, workspace/feature Clippy, rustfmt, fixture-module 5/5, iOS build-flag 13/13으로 검증했다.
- 전체 CSS reference는 494/498이며 C11/C13 네 assertion 실패가 같은 `origin/main` baseline에도 있다. 변경분 관련 fixture module-boundary와 iOS build-flag gate는 별도 통과다.
- Android 결과는 API 37 ARM64 AVD + SwiftShader PixelCopy, iOS 결과는 iPhone 17 Pro/iOS 26.2 Simulator compositor 전 Metal drawable이다. 실기기·hardware GPU·iOS compositor 이후 결과로 확대하지 않는다.

이 표는 PR diff의 코드·빌드·문서·실행 boundary를 확인한 기록이며 GitHub CI를 실행했다는 뜻은 아니다. 현재 PR에는 GitHub check run이 없다.
