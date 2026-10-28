# 구현/runtime 적대적 검토 · 20개 관점

검토 대상은 PR #738에서 부모 모듈이 이미 참조했지만 Git에서 빠진 `visibility_hidden.rs`다. 기존 단위 테스트의 4개 axis/control 조합과 malformed visibility 및 track mutation 거부 검사를 대조했다. 관점별 소스 검토와 실행 증거를 구분한다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 허용하지 않은 root ID가 새 adapter로 오인된다 | `scene_for_root`는 column/row의 hidden 및 visible-control root 네 개만 선택한다. 알 수 없는 root는 `None`으로 돌아가 기존 adapter 라우팅에 맡긴다. |
| 2 | column/row scene이 서로 바뀐다 | 네 root ID가 각각 `Axis::Columns` 또는 `Axis::Rows`에 고정되어 있다. |
| 3 | 현재 snapshot과 computed style revision이 어긋난다 | scene 확인 후 `validate_snapshot_revision`를 호출해 stale revision을 거부한다. |
| 4 | 다른 namespace/tag/root 위치를 C11.4 root로 받는다 | HTML namespace, `div`, `HostParent::Root`를 검사한다. |
| 5 | 추가·누락 child가 고정 fixture를 통과한다 | 직계 child 수가 정확히 4인지 검사한다. |
| 6 | child 순서를 바꾸어도 같은 fixture로 받아들인다 | `[a, span, image, c]` 순서를 고정 destructuring으로 검증한다. |
| 7 | child ID나 부모 관계가 다른 트리를 허용한다 | 각 child의 HTML tag, scene별 ID, `HostParent::Node(root)`를 검사한다. |
| 8 | root `visibility:hidden`가 상속된 채 layout 입력으로 쓰인다 | root computed property가 `visible`인지 확인한다. |
| 9 | hidden scene이 visible span 또는 control scene이 hidden span을 허용한다 | root scene suffix로 기대값을 정하고 span의 computed `visibility`와 비교한다. 기존 mutation 테스트가 두 방향 변경을 거부한다. |
| 10 | A/image/C가 hidden이거나 span 이외 child가 hidden이다 | 세 non-span child의 `visibility:visible`을 각각 확인한다. |
| 11 | track axis·크기가 fixture contract에서 바뀐다 | `track_axis`와 `validate_root_style`가 요청 축 및 고정 root track style을 확인한다. 기존 root track mutation 테스트가 거부된다. |
| 12 | item placement 또는 item별 style이 변경된다 | A/span/image/C의 track 번호와 `validate_item_style`를 검사한다. |
| 13 | 다른 문자열이 intrinsic contribution 입력으로 섞인다 | A는 10개 X, span은 15개 X, C는 4개 X로 고정 비교한다. |
| 14 | image가 nested child를 갖거나 다른 URL을 가리킨다 | leaf 조건과 고정 `spinon.invalid/c14-1-png-basic.png` source를 검사한다. |
| 15 | 손상된 PNG가 정상 intrinsic metadata로 승격된다 | `ImageRequest` 생성과 decode 오류를 `RuntimeLayoutFailure`로 반환한다. |
| 16 | 이미지 resource snapshot이 빠지거나 자연 크기가 다르다 | snapshot 내 image entry를 요구하고 24×16 natural dimensions를 확인한다. |
| 17 | intrinsic-size revision 증가가 overflow한다 | `checked_next()` 실패를 명시적으로 반환한다. |
| 18 | layout에 전달할 intrinsic snapshot 생성이 실패한다 | `to_layout_intrinsic_size_snapshot()` 오류를 호출자에게 전파한다. |
| 19 | row 장면에서 text shaping 방향이 column과 혼동된다 | 모든 텍스트를 no-wrap으로 두고 row axis에서만 `VerticalRl` 및 `LineBreakMode::Normal`을 설정한다. |
| 20 | OS font에 의존하거나 extended projection이 켜진다 | system font 없는 engine에 Ahem을 등록하고 `use_extended_fixture_projection: false`를 반환한다. |

**실행 결과:** 기존 회귀 테스트 1/1 통과, `cargo fmt --all -- --check` 통과, package library Clippy 통과. 전체 workspace Clippy는 README에 적은 기존 FFI test initializer와 `map_identity` 문제로 중단됐다. 이 검토로 모든 malformed 입력의 런타임 거부를 새로 실행했다고 주장하지 않는다.
