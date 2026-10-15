# C13.3 style invalidation 구현 실패 관점 20개 · 2026-10-15

## 검토 범위

실제 inline style update가 cascade request·cache·render snapshot·active gesture에 전파되는 변경, offset restore와 geometry 비교 순서, fixture-only native barrier, Android/iOS 시나리오 8/9/10 및 pixel 검증을 검토했다. 계획의 20개 항목과 별도이며, PR 변경 관점 30개 검토도 별도로 수행한다.

## 적대 관점

| # | 실패 관점 | 검토 결과와 근거 |
|---:|---|---|
| 1 | 새 document generation에서 이전 문서의 style revision이 유출되는가 | generation 교체 때 `StyleRevision::INITIAL`로 reset한다. 다른 document lineage의 cache 재사용은 generation 비교에서 거부된다. |
| 2 | viewport/environment 변경을 author style mutation으로 잘못 세는가 | `register_document_snapshot`에서 document generation·subtree invalidation만 revision을 바꾼다. environment update는 현재 style revision을 유지한다. |
| 3 | 첫 snapshot 또는 full invalidation이 가짜 subtree revision을 만드는가 | `Full`은 현재 style revision으로 full calculation을 수행하며 subtree revision 증가와 cache reuse는 분리된다. 기존 invalidation runtime tests와 workspace suite 통과. |
| 4 | subtree style mutation이 revision 증가를 놓치거나 두 번 올리는가 | state가 분류 결과를 한 번 검사하고 `checked_next`를 한 번 수행한다. Android/iOS 모두 mode 8/9/10에서 style `0→1`을 기록했다. |
| 5 | style revision overflow에서 snapshot을 부분 교체하는가 | checked increment 결과를 document 교체보다 먼저 구한다. core revision tests는 `u64::MAX`에서 `None`을 확인한다. runtime state의 최대값 주입 integration test는 없다. |
| 6 | 오래된 worker 결과가 새 style revision 결과를 덮는가 | request key에 typed style revision을 포함하고 게시 전에 전체 key를 비교한다. runtime latest-style revision 회귀 테스트와 전체 workspace tests 통과. |
| 7 | style revision이 cache key 비교에서 빠져 이전 output을 재사용하는가 | cache entry는 revision 값을 저장하고 `RuntimeCalculationCacheKey`도 typed revision을 비교한다. cache key 회귀 테스트 통과. |
| 8 | subtree style cache가 한 revision 이상 앞서 있어도 재사용되는가 | `Subtrees` reuse는 source revision 다음 단일 style revision만 허용하고 `Unchanged`는 동일 revision만 허용한다. 누락 revision은 재계산 경로로 간다. |
| 9 | 새 revision에서 이전 cascade tree를 incremental base로 오인하는가 | incremental cascade는 같은 revision 또는 정확히 한 단계 다음 revision만 수락한다. 그 외는 full fallback이다. |
| 10 | 계산 profile 중 하나가 `StyleRevision::INITIAL`을 계속 사용해 새 key와 다른 output을 만드는가 | cascade calculation profile 호출 전부 request key revision을 전달한다. 최종 styles/key equality guard도 확인했다. |
| 11 | 빈 render scene이 style revision을 잃는가 | empty-scene path가 request revision을 snapshot에 기록하고 key/document/environment consistency를 검증한다. |
| 12 | 대기 중 여러 style mutation을 합치며 cache base의 revision 간격을 숨기는가 | revision 간격이 1보다 크면 cache 및 incremental adjacency 검사에서 reuse하지 않는다. worker tests는 latest-revision publication을 검사한다. |
| 13 | 새 snapshot보다 이전 정책 epoch를 가진 queued move가 재해석되는가 | fixture host는 sample expected style revision과 report epoch, current snapshot revision을 각각 비교하고 stale epoch를 reject/discard한다. |
| 14 | style update가 반영되지 않았는데 move가 새 policy로 승인되는가 | update barrier는 성공 report 확인 뒤 deferred move를 enqueue한다. 실패면 active gesture를 cancel하고 deferred move를 버린다. |
| 15 | active `touch-action` mutation이 기존 gesture의 허용 축을 바꾸는가 | active session은 Begin 때 허용 축을 동결하고 geometry-neutral style revision에서 expected revisions만 전진한다. mode 8의 active pan은 유지됐고 다음 gesture는 `pan_rejected`였다. |
| 16 | style revision이 올랐다는 이유만으로 unrelated document/tree mutation이 허용되는가 | document/tree가 바뀌고 style revision은 그대로인 경우 `DocumentOrTreeChanged`로 취소한다. tree structure·owner chain·clip도 geometry key에 포함된다. |
| 17 | frame은 같아도 scrollport, max range, hit clip이 변한 geometry update를 놓치는가 | geometry equality는 frame, paint/hit clip, hit order, ancestor chain, scrollport, owner parent 및 max offsets를 비교한다. mode 9는 range `86→66`과 `GeometryChanged`를 관찰했다. |
| 18 | 새로운 scene의 offset 0 clip을 이전 scroll-adjusted clip과 비교해 policy-only update를 geometry 변경으로 오인하는가 | offset을 먼저 복원한 다음 geometry key를 비교하도록 순서를 고쳤다. mode 8·10의 scroll revision은 유지되고 mode 9만 증가했다. |
| 19 | geometry cancellation 뒤 stale gesture가 재개되거나 새 geometry의 gesture가 막히는가 | mode 9에서 active pan terminal은 `GeometryChanged`; 두 번째 gesture는 새 geometry에서 시작했다. 양 플랫폼 scenario assertions가 이를 확인했다. |
| 20 | paint 변경이 scroll revision을 올리거나 content가 inner border 밖으로 새는가 | mode 10은 active pan/scroll revision을 유지했다. Android/iOS exact pixel은 inner `[23,37,58,255]→[14,86,80,255]`, border 바깥 표본은 `[27,52,65,255]`였다. leaked-color 변형은 Node validator test에서 거부한다. |

## 검증 요약

- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator: 최종 opt-in matrix 6/6 통과. 원본 로그·화면·build·matrix·hash는 [최신 실행 evidence](20261015T112615633Z-3abc0b5e/README.md)에 있다.
- Rust workspace: 866 passed, 0 failed, 4 ignored. `spinon-ffi` feature-on: 45 passed, 0 failed, 2 ignored.
- Workspace Clippy, `cargo fmt --all -- --check`, fixture module boundary test와 style-invalidation capture tests를 통과했다.
- Chrome active-touch policy 결과 차이, iOS multi-touch, public events/CSSOM, real device/hardware GPU, compositor 이후 presentation은 이 20개 검토의 pass로 간주하지 않는다. 기본 iOS Simulator `.app`·Android debug APK의 style fixture symbol/payload/marker 부재 및 opt-in positive control은 별도 [artifact boundary evidence](artifact-boundary/README.md)에서 확인했다.
