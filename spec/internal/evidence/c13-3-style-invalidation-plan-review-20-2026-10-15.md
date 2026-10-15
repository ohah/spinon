# C13.3 style invalidation 계획 적대 검토 20개

검토 대상은 [구현 전 계획](../../../plan/c13-3-style-invalidation.md)과 [Chrome 155 사전 비교](./c13-3-style-invalidation-precomparison-2026-10-15/README.md)다. 코드 구현 전 별도 관점으로 판정했다.

| # | 실패 관점 | 실패 조건 | 계획의 방어·관찰값 | 판정 |
|---:|---|---|---|---|
| 1 | document revision을 모두 fatal로 취급 | inline policy style이 바뀌어도 active gesture가 매번 취소됨 | document/style revision을 함께 추적하고 같은 geometry의 style-only transition만 허용 | 반영 |
| 2 | style attribute도 render-tree revision을 올리는 사실 누락 | inline style 변경에서 render-tree revision이 달라져 활성 pan이 모두 취소됨 | HostDocument에서 style attribute 변경이 document/render-tree revision을 함께 올리는지 실제로 재현하고, style revision 및 geometry와 함께 분류 | 반영 |
| 3 | 임의 document mutation을 style-only로 오인 | text/attribute 변경이 style epoch 없이 geometry-neutral이면 session 유지 | style revision이 바뀌지 않은 document/render-projection mutation은 계속 취소 | 반영 |
| 4 | Begin의 touch-action을 업데이트된 값으로 교체 | `pan-x`로 시작한 gesture가 중간 `none` 변경으로 갑자기 멈춤 | active action/axis를 frozen 상태로 유지하고 fresh Begin에서 새 값 계산 | 반영 |
| 5 | overscroll policy를 gesture 전체에서 고정 | style update 뒤 boundary chaining이 계속 허용됨 | sample별 policy epoch을 기록하고 새 epoch 이후 sample에 새 snapshot을 사용 | 반영 |
| 6 | update 전 queued sample을 새 policy로 재해석 | 이전 auto sample이 contain으로 소급되거나 delta가 유실 | style update barrier 전 old sequence 처리, 뒤 sample에 새 key 연결 | 반영 |
| 7 | queue coalescing이 epoch 경계를 넘음 | old/new style sample이 합쳐져 정책 적용 시점 손실 | mutation 동안 native Move admission 보류, cumulative sample 재개 및 epoch trace | 반영 |
| 8 | 보류한 cumulative translation을 두 번 적용 | iOS/Android에서 update 뒤 첫 sample이 중복 delta 생성 | 마지막 accepted cumulative과 다음 sample의 차이만 적용, 플랫폼별 no-double-apply control | 반영 |
| 9 | snapshot recompute가 offsets를 0으로 초기화 | paint/policy mutation만으로 화면이 top-left로 점프 | owner ID 기준 offset 재적용 및 geometry-neutral scroll revision 보존 | 반영 |
| 10 | geometry shrink 뒤 offset clamp 누락 | offset이 새 max range를 초과하거나 viewport 밖 marker가 노출 | 새 range에 clamp, geometry transition과 revision 변경을 분리 기록 | 반영 |
| 11 | owner 삭제·reparent를 style-only로 처리 | frozen owner chain이 없는 node를 계속 scroll | owner ID/parent/scrollport/range/clip 비교, 관계 변화에서 cancel | 반영 |
| 12 | surface generation 변화와 style update 경합 | 닫힌 surface의 snapshot이나 late Move가 다시 publish | update 실패·surface 종료는 cancel; generation 검증과 stale rejection 기록 | 반영 |
| 13 | style eval 실패가 일부 revision을 게시 | 새 style report와 옛 snapshot/offset이 섞임 | 새 snapshot 완성/검증 뒤 publish; 실패 negative control에서 이전 상태 원자성 확인 | 반영 |
| 14 | invalid mutation ID가 기본 정책으로 fallback | 잘못된 route가 `auto`로 조용히 scroll 허용 | 고정 enum만 받고 invalid mode 명시 거부 | 반영 |
| 15 | 새 `touch-action:none`이 native recognizer를 영구 끔 | 후속 gesture가 OS에서 못 오거나 기존 tap 경로 변화 | native adapter는 active stream을 유지하고 core Begin 결과 및 tap gate를 분리 관찰 | 반영 |
| 16 | style-pause가 Android tap activation을 새로 발생 | pan 중 눌림 상태가 유지되어 up에서 click 발생 | 기존 pan-start pressed/click 취소 invariant를 유지하고 tap count를 확인 | 반영 |
| 17 | iOS pan recognizer의 cumulative 값과 CSS px 변환 혼동 | surface scale 때문에 axis delta·range 다르게 처리 | CSS viewport point, UIKit points, surface scale, cumulative/delta 로그를 기록 | 반영 |
| 18 | 정책 결과만 보고 border clip 주장 | marker가 inner scrollport 바깥에 그려짐 | 양 플랫폼 screenshot과 exact-color inside/outside clip sample을 합격 조건에 포함 | 반영 |
| 19 | Chrome active policy mutation을 Spinon expected로 잘못 승격 | Chrome에서 새 contain 값이 동일 gesture에 즉시 반영된다고 오기록 | JSON은 Chrome 관찰값 그대로 보존; local next-sample 계약 차이를 별도 표시 | 반영 |
| 20 | opt-in fixture ABI가 기본 artifact에 유입 | 제품 빌드에 style mutation FFI/payload 포함 | Android/iOS default 0, opt-in 1 symbol/payload 대조를 별도 gate로 고정 | 반영 |

**판정:** 구현 진입 가능. 실제 구현에서는 새 20개 실패 관점을 별도로 작성하고 각 미검증 사항을 합격으로 간주하지 않는다.
