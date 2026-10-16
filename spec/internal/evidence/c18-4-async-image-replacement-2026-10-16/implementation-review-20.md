# C18.4 구현 적대 검토 20개

**대상:** C18.4 request/result·resource snapshot·intrinsic layout, opt-in live V8 fixture, Android AVD와 iOS Simulator 실행
**검토 시점:** 2026-10-16 · 계획의 사전 검토 20개와 PR diff 검토는 별도 기록
**판정:** 독립 실패 관점 20개 확인 완료. 같은 `src`를 유지한 문서 revision 변경에 대한 단위 검사가 빠져 있어 추가했고, guard mutation이 이 검사를 깨뜨리는 것을 확인했다.

| # | 독립 실패 관점 | 코드·실행 대조 | 판정 |
| ---: | --- | --- | --- |
| 1 | completion이 decode 요청의 node·src identity를 잃는가 | `ImageResourceOutcome`은 request와 성공/오류를 함께 소유한다. late failure test가 원래 src를 확인한다. | 통과 |
| 2 | 같은 `src`라도 document/tree revision이 달라진 completion을 받는가 | 동일 src 상태에서 `alt`를 바꾼 새 test를 추가했다. `request.source()` guard를 임시 제거하자 test가 실패했고 원복 후 통과했다. | 통과 · negative control |
| 3 | src 교체 뒤 이전 성공 completion이 새 PNG를 덮는가 | A→B gated completion 순서 B,A와 `SourceChanged` 분류를 단위·앱 fixture에서 확인했다. | 통과 |
| 4 | src 교체 뒤 늦은 decode 실패가 현재 image를 비우는가 | late failure는 stale로 분류되어 현재 success와 failure map을 바꾸지 않는다. request-bound outcome test가 확인한다. | 통과 |
| 5 | 문서 generation이 바뀐 요청을 같은 node ID로 재사용하는가 | old generation의 성공·실패 outcome 둘 다 거부하는 test가 있다. | 통과 |
| 6 | 분리된 node의 손상 PNG가 현재 오류로 잘못 기록되는가 | 연결 여부와 HTML `img` namespace를 다시 확인하고 detached result를 `InvalidRequest`로 돌린다. | 통과 |
| 7 | 중복 성공/실패 completion이 같은 node에 적용되는가 | duplicate node error test가 있다. guard를 임시 제거하면 해당 test가 예상 오류 대신 정상 snapshot을 받아 실패했다. | 통과 · negative control |
| 8 | decode 오류 bytes가 투명 성공이나 빈 이미지로 조용히 바뀌는가 | actual `PngDecoder` 오류가 같은 request의 error outcome에 남고 실패 map에 기록된다. | 통과 |
| 9 | 현재 source 실패 때 이전 image count가 남는가 | replacement snapshot에는 image가 없고 node 오류만 존재한다. failure insertion mutation에서 recovery test가 실패했다. | 통과 · negative control |
| 10 | 실패한 node가 같은 batch의 다른 node 성공을 막는가 | 한 image 실패와 다른 node success를 함께 적용하는 단위 test가 있다. | 통과 |
| 11 | 실패 뒤 정상 source가 새 revision에서 복구되는가 | decode failure 뒤 D success의 image·error map·dimension을 단위 및 app fixture에서 확인한다. | 통과 |
| 12 | 실패 결과가 revision 예산 제한을 우회하는가 | 최대 결과 수 계산은 success와 failure 모두 세며 boundary test에서 다음 항목을 거부한다. | 통과 |
| 13 | 성공 PNG 누적 RGBA byte 계산이 overflow 또는 budget 초과를 허용하는가 | checked addition과 64 MiB 경계·`usize::MAX` test가 있다. | 통과 |
| 14 | 첫 pending auto-size 0×0이 decode 전에 오류가 되는가 | 0×0 pending frame test가 통과한다. natural ratio guard를 임시 완화해 0/0 ratio를 만들면 test가 실패했다. | 통과 · negative control |
| 15 | 새 document generation에서 이전 natural dimensions를 이어받는가 | 새 generation pending은 0×0이며 이전 성공 측정 크기를 복사하지 않는 단위 test가 있다. | 통과 |
| 16 | 교체 pending이 이전 RGBA/cache key를 새 source에 붙이는가 | `to_pending_layout_intrinsic_size_snapshot`은 dimensions만 복사한다. C pending report의 image count는 0이고 source stamp는 새 document다. | 통과 |
| 17 | 새 decode natural dimensions가 Taffy auto-size frame에 도달하는가 | 첫 성공 A는 2×2, replacement C는 4×2 frame이며 Rust report와 플랫폼 report가 일치한다. | 통과 |
| 18 | out-of-order 재현 순서가 thread scheduling/sleep에 우연히 의존하는가 | worker gate를 B completion 이후에 열며 timeout은 교착 실패를 제한한다. C18.4 fixture test는 completion 순서 B,A를 기록한다. | 통과 |
| 19 | worker cancellation/panic이 pending snapshot을 성공으로 publish하거나 join 없이 남는가 | worker `started` acknowledgment·cancel gate·join·disconnect를 검사하는 opt-in test가 3/3 suite에서 통과한다. | 통과 |
| 20 | 실제 앱 화면이 report 문자열만 내고 잘못된 scene/style/pixel을 통과 처리하는가 | live V8 DOM·computed UA style·Taffy·renderer path를 실행했다. Android API 37 ARM64 AVD PixelCopy와 iOS 26.2 Simulator Metal drawable은 각각 6/6 exact이며 readback 시점을 제한해 기록했다. | 통과 |

## 발견 및 재검증

- 기존 stale tests는 src 변경, removed node, generation 변경을 다뤘지만 src가 그대로인 채 document revision만 바뀌는 경우를 직접 검사하지 않았다. 이 누락은 제품 guard 오류가 아니라 회귀 coverage 공백이었다. 동일-src stale test를 추가하고 source-stamp guard를 임시 제거해 실패를 확인한 뒤 원복 후 통과시켰다.
- 추가 negative control은 duplicate node guard, current decode failure 기록, 0×0 pending ratio guard를 각각 임시로 약화했다. 각 대응 test가 실패했으며 실제 소스는 모두 원상복구했다. 원본 실패 로그는 이 디렉터리의 `negative-*.log`에 있다.
- 최종 재검증: `cargo test --locked --workspace` 900 passed·4 ignored, C18.4 opt-in fixture test 3/3, workspace Clippy와 C18.4 feature-enabled Clippy, `cargo fmt --all -- --check`, fixture module-boundary 5/5, iOS build-flag 13/13.

이 검토는 고정 PNG fixture profile을 대상으로 한다. 제품 URL/network loader, HTML events, cache policy, UA broken-image fallback, 실기기/hardware GPU와 compositor 이후 iOS surface는 검증 범위가 아니다.
