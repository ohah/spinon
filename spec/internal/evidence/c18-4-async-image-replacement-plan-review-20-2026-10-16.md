# C18.4 계획 적대 검토 20개 관점

**대상:** [C18.4 계획](https://github.com/ohah/spinon/blob/main/plan/c18-4-async-image-replacement.md)
**검토 시점:** 구현 전 · 2026-10-16
**판정:** 20개 관점 검토 완료 · 확인된 계획 공백 3개를 수정했다.

| # | 별도 실패 관점 | 확인과 조치 | 판정 |
|---:|---|---|---|
| 1 | 비동기 순서가 단순 sleep 편차에 따라 뒤집히는가 | Chrome oracle의 HTTP delay는 관찰 입력이다. 제품 fixture는 A/B 벽시계 경쟁에 의존하지 않고 barrier/channel로 B 완료 뒤 A 결과를 공개하도록 계획에 고정했다. | PASS |
| 2 | 빠른 B 뒤 늦은 A 성공이 현재 이미지를 되돌리는가 | Chrome observation은 B만 load되고 A 응답 후에도 B src·1×4 natural size·blue pixel을 기록한다. 현재 image key에 A completion이 섞이지 않는지 단계별 report로 검사한다. | PASS |
| 3 | 빠른 B 뒤 늦은 A 실패가 현재 리소스를 지우는가 | success뿐 아니라 stale failure도 current source/document stamp 불일치로 무시해야 한다고 계약에 명시했다. | PASS |
| 4 | source가 A→B→A로 돌아오는 ABA 경우 오래된 A가 수용되는가 | 동일 문자열 src만 비교하지 않고 generation/document/render-tree stamp와 request identity를 함께 사용한다. 오래된 A request는 값이 같아도 이전 document stamp라 stale이다. | PASS |
| 5 | 요소가 분리되거나 node id가 재사용된 뒤 completion이 붙는가 | generation 및 연결된 HTML img preflight를 유지하고 stale generation/tree를 검사 항목에 둔다. | PASS |
| 6 | 새 src pending 동안 이전 GPU texture가 새 src로 잘못 표시되는가 | Chrome pending screenshot에서 이전 green pixel 대신 빈 표면이 보인다. 계획은 draw 0·texture 제거·고정 background 유지로 고정했다. | PASS |
| 7 | pending 동안 intrinsic geometry가 갑자기 0으로 줄어드는가 | auto-size Chrome 결과에서 첫 pending은 0×0, 이전 성공 뒤 pending은 직전 2×2 frame을 유지했다. 두 분기를 판정 기준에 추가했다. | PASS |
| 8 | 이전 intrinsic measurement를 새 document stamp로 옮길 때 이전 bytes까지 새 source로 재바인딩되는가 | 계획을 수정해 pending은 size-only retained measurement만 현재 stamp로 만들고 이전 RGBA/request/GPU key는 옮기지 않도록 분리했다. 구현 검토에서 snapshot 타입 경계와 positive control을 확인한다. | PASS |
| 9 | decode 성공 natural size가 새 layout에 전파되지 않는가 | Chrome auto-size A는 2×2, C는 4×2 frame을 만들었다. current resource revision 증가, C14 intrinsic snapshot과 Taffy frame 전파를 요구한다. | PASS |
| 10 | decode failure가 이전 source texture를 고정 frame 아래 남기는가 | 실패 fixture는 고정 48×32 frame에서 배경 표본·0 draw로 판정한다. 이전 cache entry를 남기는 negative control이 실패해야 한다. | PASS |
| 11 | 실패가 다음 정상 src 복구를 막는가 | 관찰된 corrupted PNG 이후 D load·purple 표본을 독립 recovery 단계로 고정했다. | PASS |
| 12 | 실패 결과에서 어떤 request가 실패했는지 잃는가 | 현재 `ImageRequest::decode(self)`는 error 때 request를 반환하지 않는다. 새 result 모델이 성공·실패 모두 원래 request identity를 보존하도록 구현 관문에 명시했다. | PASS |
| 13 | 손상 bytes가 성공·빈 이미지로 조용히 바뀌는가 | Chrome `decode()`의 `EncodingError`와 `error` event를 oracle에 보존하고 Rust failure 종류·node/source/revision을 report에 남긴다. | PASS |
| 14 | Chrome UA broken-image icon을 renderer 색상 계약으로 오인하는가 | 실패 screenshot에 보이는 UA icon을 명시적으로 제외했다. 내부 fixture pixel 계약은 decoded image 0과 background만 대상으로 한다. | PASS |
| 15 | auto-size 실패 시 브라우저 fallback 크기를 제품 보장으로 확대하는가 | 추가 oracle는 UA auto-size failure frame이 viewport에 따라 달라질 수 있음을 보였다. 계획에서 그 fallback을 제외하고 고정 frame failure와 분리했다. | PASS |
| 16 | 비동기 결과가 C18.1 opaque·C18.2 alpha 또는 C18.3 fit를 회귀시키는가 | 세 profile의 기존 픽셀 행렬을 Android/iOS에서 다시 실행하는 완료 관문이 있다. | PASS |
| 17 | 고정 frame 검증이 C14 intrinsic re-layout을 대신했다고 과장하는가 | paint track과 별도 auto-size geometry track을 분리해 두 결과를 각각 보존한다. 상위 C14는 여전히 미완료다. | PASS |
| 18 | simulator 결과가 실기기·hardware GPU 또는 compositor 이후 결과로 읽히는가 | Android AVD software renderer와 iOS Metal drawable의 readback 시점을 결과마다 따로 제한한다. | PASS |
| 19 | fixture 코드와 정적 PNG가 기본 제품 artifact에 들어가는가 | 새 FFI route/payload를 opt-in `c18-4-image-fixture-test`에 격리하고 default/positive control archive·APK·app을 검사한다. | PASS |
| 20 | 전체 scope가 URL loader·public event·C18 완성으로 번지는가 | 네 개 고정 PNG와 한 malformed payload만 다룬다. URL/network loader, DOM load/error events, icon, public API와 C18 전체는 명시적으로 제외했다. | PASS |

## 수정 기록

- 첫 계획 초안은 fixed CSS frame만 포함해 C18 상위가 요구하는 decode-success 후 natural-size re-layout을 빠뜨렸다. Chrome 155 auto-size oracle 및 pending screenshot을 추가하고 C14 intrinsic geometry pass criteria를 별도 경로로 명시했다.
- pending geometry의 retained measurement가 이전 resource bytes를 새 source stamp에 붙이는 것으로 해석될 수 있었다. size-only measurement와 source-bound decoded bytes/GPU key를 분리했다.
- fixture worker의 completion 순서가 wall-clock delay에 의존할 수 있었다. 제품 검증 순서는 barrier/channel gate로 확정했다.

이 검토는 계획 문서에 대한 사전 검토이며 구현 코드 검토를 대신하지 않는다.
