# C13.3 snapshot delta-router PR 변경 검토 30개

검토 대상은 현재 PR 후보의 render API, 정책 저장, snapshot 재발행, 테스트, 상태 문서다. 각 항목을 서로 다른 계약 경계나 실패 조건에서 확인했다. 이 결과는 typed CSS, gesture/session 또는 mobile integration의 검토를 대신하지 않는다.

| # | PR 변경에서 따로 확인한 관점 | 결과 |
| --- | --- | --- |
| 1 | 새 공개 타입이 `spinon-render` crate root에서 외부에 도달하는가 | `RuntimeOverscrollBehavior`, `RuntimeScrollInputPolicy`, outcome/change가 crate root에 re-export된다. |
| 2 | 기존 C13.2 constructor가 새로 사용자 입력을 허용해 회귀하는가 | 생성 시 두 user-scroll 축 모두 disabled다. dedicated default-policy test가 있다. |
| 3 | scroll offset 재발행 후 입력 정책이 사라지는가 | `with_offsets`가 기존 policy와 parent를 복사하며 routing test가 확인한다. |
| 4 | 정책 전용 변경이 scroll geometry 비교에 섞여 revision을 부당하게 올리는가 | C13.2 geometry equality는 owner/parent/scrollport/extent만 비교해 입력 style을 geometry로 취급하지 않는다. |
| 5 | 새 오류 variant가 진단 문자열 없이 누락되는가 | stale target, invalid point, invalid delta를 모두 `Display`에 매핑한다. |
| 6 | hit-test 결과와 owner metadata가 서로 다른 box를 가리키는가 | snapshot 생성 시 box/metadata 길이와 base frame 대응을 검증하고 node index로 metadata를 찾는다. |
| 7 | wheel 위치 입력이 현재 snapshot이 아니라 임의 target을 택하는가 | hit-test 경로는 현재 snapshot의 viewport와 clip-aware boxes를 사용한다. |
| 8 | clip 경계 바깥의 고순위 child가 target이 되는가 | hit clip을 통과하지 못하면 후보에서 빠진다. |
| 9 | viewport 오른쪽·아래 반열린 경계를 잘못 포함하는가 | 기존 hit-test의 half-open 경계 규칙을 그대로 따른다. |
| 10 | 무한 point가 hit miss로 조용히 위장되는가 | API 경계에서 `InvalidScrollPoint`로 구분해 반환한다. |
| 11 | 휠 delta의 NaN이 x만 적용된 뒤 y에서 실패하는가 | 두 delta를 함께 검사하고 계산에 들어가기 전에 거부한다. |
| 12 | old scroll revision이 현재 offset에 덮어쓰기를 하는가 | expected scroll revision 일치가 모든 계산의 선행조건이다. |
| 13 | stale document/tree/environment key를 router 단독으로 알아낼 수 있는가 | 이 lower-level API에는 외부 current-key 인자가 없다. C13.3 session integration에서 비교해야 하며 문서에 미구현으로 남겼다. |
| 14 | direct target이 container일 때 self owner가 ancestor에서 빠지는가 | target owner를 먼저 추가하고 metadata ancestor를 뒤에 붙인다. |
| 15 | target owner가 metadata ancestor에 중복되면 두 번 소비되는가 | `BTreeSet` 중복 검사가 `InvalidScrollState`로 종료시킨다. |
| 16 | ancestor owner 중 snapshot container가 누락돼도 일부 offset이 게시되는가 | 누락 lookup은 immutable candidate 반환 전에 오류가 된다. |
| 17 | hidden overflow를 auto처럼 직접 scroll해 기존 C13.2 계약을 어기는가 | 입력 가능 여부가 false면 offset을 소비하지 않는다. |
| 18 | hidden owner의 `overscroll-behavior: contain`이 chain을 막지 않는가 | 소비 없이도 정책을 검사하므로 해당 축 remainder가 유지된다. |
| 19 | `overflow:clip`/`visible`이 없는 owner 자리를 만들 필요가 있는가 | 이 router는 snapshot에 실제 owner로 전달된 chain만 방문한다. C13.1 builder가 ownerless box를 제외하는 범위는 그대로다. |
| 20 | x user-scroll 설정이 y 이동을 부당하게 제어하는가 | 정책과 stopped state가 물리 축별로 독립적이며 diagonal test를 통과한다. |
| 21 | zero range axis가 remainder를 소비한 것으로 기록되는가 | consumed는 실제 offset 차이로 계산하며 `auto` remainder를 다음 owner에 전달한다. |
| 22 | 두 owner가 연속 소비하면서 중간 반올림으로 CSS fraction을 정수화하는가 | offset/delta는 f32로 유지하고 중간 정수 변환이 없다. 1.25 px test를 둔다. |
| 23 | min/max에서 부호가 반전되거나 offset이 range를 벗어나는가 | 각 owner에서 `[0,max]` clamp하고 negative lower-bound 및 saturated upper-bound controls를 둔다. |
| 24 | 유한하지만 큰 f32 delta가 offset 합계 계산을 곧바로 f32 infinity로 만드는가 | sum/clamp는 f64에서 계산하고 offset range 안으로 변환한다. 극단 magnitude의 전체 CSS pixel conservation은 별도 한계로 취급한다. |
| 25 | 한 축 contain이 반대 축 ancestor remainder도 잃게 하는가 | 축별 stopped flag를 사용한다. X remain/Y consumed diagonal case가 검증된다. |
| 26 | `none`에 local overscroll affordance까지 이미 구현됐다고 오해할 수 있는가 | API는 ancestor chain만 차단하며 local visual distinction을 제공하지 않는다고 코드/리뷰에 적었다. |
| 27 | target이 없어도 no-op offset map을 새로 만들어 revision을 올리는가 | hit miss는 `None`으로 반환하고 immutable source snapshot에 손대지 않는다. |
| 28 | 여러 owner 변경이 owner별로 여러 scroll revision을 publish하는가 | 모든 update를 모아 `with_scroll_offsets`를 한 번 호출한다. 두 owner 변경 결과는 revision 1회다. |
| 29 | offset 변화가 없는 saturated input이 변경 record나 revision을 만드는가 | consume 값이 0이면 change/update 목록이 비고 current revision과 remainder를 보존한다. |
| 30 | 이번 PR만으로 Android/iOS gesture 또는 화면 검증을 통과했다고 주장하는가 | UI route가 없으므로 platform build/Simulator를 실행하지 않았고, 상태·계약·근거 문서에 남은 범위와 미완료를 명시했다. |

## 발견 및 처리

- 새 router만으로 stale document/tree generation을 판정할 수 없다는 경계를 발견했다. session integration 책임으로 남기고 `STATUS`, contract, evidence README에서 미완료라고 분리했다.
- 이 PR은 UI를 생성하거나 바꾸지 않아 app screenshot을 첨부하지 않는다. PR 본문은 동일 제한 범위와 테스트 결과를 적고, Android/iOS Simulator 검증이 아직 없다고 밝힌다.

이 좁은 core-router PR 범위에서는 수정되지 않은 P1/P2 결함을 찾지 못했다. 전체 C13.3의 통과 판정은 보류한다.
