# C11.4 intrinsic measurement trace PR 변경 검토

**날짜:** 2026-10-19
**검토 범위:** `feat(c11-4-intrinsic-contributions-20261019` 브랜치의 layout trace, 오류 경계, 테스트와 상태 문서 변경

| 번호 | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | callback이 없는 item에도 존재하지 않는 측정값을 합성하는가 | 기록은 실제 text/replaced measure callback 성공 경로에서만 발생한다. output trace를 contribution 목록으로 취급하지 않도록 문서화했다. 통과. |
| 2 | Grid가 아닌 Flex/Block leaf 측정도 결과에 섞이는가 | direct Grid item membership을 통과한 callback만 기록하고 non-Grid text 회귀 테스트에서 빈 trace를 확인한다. 통과. |
| 3 | absolute Grid child가 Grid item trace에 들어가는가 | membership 구성 시 `Absolute` positioning을 제외한다. 통과. |
| 4 | fixed Grid child가 Grid item trace에 들어가는가 | membership 구성 시 `Fixed` positioning을 제외한다. 통과. |
| 5 | nested non-leaf item의 descendant callback을 item aggregate라고 오해하는가 | leaf callback만 관찰하며 nested aggregate/provenance를 지원하지 않는다고 plan/status에 명시했다. 미지원 범위 고정. |
| 6 | anonymous Grid text를 안정된 외부 NodeId로 잘못 기록하는가 | anonymous item은 trace 대상이 아니며 후속 synthetic identity 작업으로 남겼다. 미지원 범위 고정. |
| 7 | callback 중복을 한 node 값으로 덮어쓰는가 | Vec에 callback마다 append하므로 중복 요청과 순서를 보존한다. 통과. |
| 8 | Taffy cache hit로 생략된 callback을 완전한 intrinsic 결과라고 표시하는가 | 계약은 callback 관찰값이며 cache가 누락시킬 수 있다고 명시한다. 통과. |
| 9 | static-position flow 계산이 trace를 중복 생성하는가 | `flow_only` tree는 기록을 끈다. visual tree만 output trace를 소유한다. 통과. |
| 10 | callback이 재배치된 synthetic node를 외부 item으로 오인하는가 | Taffy node의 optional Spinon node identity를 통해서만 item membership을 조회하며 synthetic viewport node는 identity가 없다. 통과. |
| 11 | text font registry 변경이 source revision에서 빠지는가 | text source에 `FontRegistryRevision`을 보존하며 Ahem 회귀가 등록 revision을 확인한다. 통과. |
| 12 | replaced resource의 자연 크기가 source 추적에서 사라지는가 | `Replaced { natural }`에 snapshot 자연 크기를 복사하고 image 테스트에서 확인한다. 통과. |
| 13 | image snapshot revision이 output revision과 달라지는가 | snapshot revision을 output revision에 반영한 다음 trace 전체를 같은 revision으로 찍는다. 회귀 테스트가 둘을 비교한다. 통과. |
| 14 | document/style/environment revision이 callback trace에서 분리되는가 | 전체 `LayoutInputRevision`을 callback별 기록에 담고 output 최종 revision으로 제자리 갱신한다. 통과. |
| 15 | width와 height의 available space가 뒤바뀌는가 | Taffy `Size` 각 축을 독립 변환하며 text fixture가 두 definite 축을 확인한다. 통과. |
| 16 | MinContent/MaxContent 제약이 숫자나 서로 같은 variant로 축약되는가 | `MinContent`, `MaxContent`, `Definite(px)`를 공개 enum variant로 구분했다. 통과. |
| 17 | known dimensions를 available space 또는 callback 결과에서 역산하는가 | callback 입력의 두 `Option<f32>`를 그대로 복사한다. 테스트에서 기본 입력을 비교한다. 통과. |
| 18 | parent 크기가 child 결과에서 역산되는가 | callback의 parent size 입력을 두 축 그대로 보존한다. text fixture에서 `100×10px`를 확인한다. 통과. |
| 19 | callback content-box 결과가 border-box 결과로 오해되는가 | text 결과와 replaced `content_width/content_height`를 `content_size`로 저장하고 padding/border 비포함 계약을 문서화했다. 통과. |
| 20 | item style의 box-sizing/padding/border 정보가 contribution 전 단계에서 사라지는가 | direct item의 `LayoutStyle` 전체를 보존하고 text/image 테스트에서 입력 style equality를 확인한다. 통과. |
| 21 | Grid item identity가 없는 node를 map index panic으로 접근하는가 | identity 부재와 membership 미포함을 먼저 반환하고, 입력 검증에서 style/node 관계를 검사한다. 통과. |
| 22 | 예약 실패가 기본 크기나 성공 trace로 위장되는가 | `try_reserve(1)` 실패를 node가 담긴 구체 `LayoutError`로 기록하고 calc 단계가 그 오류를 반환한다. failpoint 회귀 통과. |
| 23 | 예약 실패 후 부분 LayoutOutput이 게시되는가 | calc 오류가 output 생성 경계까지 전파되어 `compute`는 `Err`를 반환한다. 회귀가 오류 variant를 확인한다. 통과. |
| 24 | 실패 후 같은 engine/tree 상태가 다음 계산을 오염시키는가 | 같은 engine 인스턴스에서 연속 성공을 검증하는 회복 테스트가 있다. 통과. |
| 25 | 최종 revision stamp가 pending 항목을 새 Vec로 복사해 OOM 경계를 우회하는가 | trace는 public record Vec 하나에 저장하고 최종 revision을 각 항목에 제자리 대입한다. 추가 변환 할당이 없다. 통과. |
| 26 | flow-only pass나 실패한 시도에서 allocator failpoint가 다른 테스트로 누출되는가 | thread-local guard가 drop 때 armed flag와 allocator failpoint를 초기화한다. 같은 worker 재계산 회귀가 확인한다. 통과. |
| 27 | trace allocation 실패가 item NodeId를 잃어 오류 원인을 찾지 못하게 하는가 | 실패 오류가 현재 callback의 외부 `NodeId`를 포함한다. 회귀가 image item id를 비교한다. 통과. |
| 28 | HostDocument 생성자의 인자 축약이 C15 text profile을 뒤바꾸는가 | `HostTextStyleProfile`이 paragraph와 inline run map을 이름 있는 필드로 묶고 유일한 projection 호출자가 명시적으로 채운다. workspace compile/test 통과. |
| 29 | 공개 추적 구조체가 완성된 CSS 동작이나 FFI/API를 암시하는가 | 0059/plan/conformance/status는 내부 관찰값, 앱 runtime 미연결, automatic minimum/track sizing 미완료를 분명히 한다. C11.4 완료 표시는 그대로 미체크다. 통과. |
| 30 | 테스트·CI·플랫폼 근거를 실제보다 넓게 주장하는가 | 현재 Rust workspace tests, 변경 crate Clippy, fmt만 통과로 기록한다. full workspace Clippy의 기존 오류, Bun CSS reference 4건, 앱 simulator 미실행을 별도 한계로 둔다. 근거 경계 통과. |

## 결과와 남은 범위

검토에서 최종 trace 변환의 불필요한 두 번째 Vec 할당을 발견해 in-place revision stamp로 수정했다. 변경 후 focused intrinsic measurement 5개, locked Rust workspace 전체, 변경 crate Clippy와 rustfmt 검사를 다시 통과했다.

이 PR은 Grid callback 관찰 trace만 추가한다. CSS contribution 산출, automatic minimum, track sizing, anonymous/nested item provenance, auto-repeat과 Android/iOS runtime 검증은 후속 범위다. full workspace Clippy의 기존 `spinon-style/...compute.rs:43` 오류와 CSS reference 4개 source/test mismatch는 변경과 무관한 기존 실패로 별도 증거에 기록되어 있다.
