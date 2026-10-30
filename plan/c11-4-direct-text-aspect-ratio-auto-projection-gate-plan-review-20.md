# C11.4 direct-text `auto <ratio>` projection gate 계획 재검토 20개

**대상:** [계획](./c11-4-direct-text-aspect-ratio-auto.md)의 ratio-owner `NodeId` projection gate 보완

**근거:** `RuntimeGridProjectionOptions`, `StyleLayoutProjectionOptions`, `C11_4TextInput`, `runtime_layout.rs`의 현재 route 경계와 Chrome 사전 비교

**판정:** 서로 다른 계획 실패 경로 20/20 PASS. 구현을 평가하거나 feature 동작을 주장하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11의 공유 `RuntimeBlockFlexGridTrackSizingV1` 프로필이 곧 auto-ratio admission이라고 해석 | PASS — 프로필 자체로는 투영하지 않고 exact-node opt-in marker를 요구한다. |
| 2 | C11.4의 unrelated generic/nested/image scene이 content-box route를 선택 | PASS — `C11_4TextInput`에 특정 marker가 있을 때만 전용 projection entry point를 호출한다. |
| 3 | style snapshot의 generation/revision과 marker NodeId가 다른 문서에 적용 | PASS — existing snapshot identity validation 뒤 같은 snapshot에서 owner를 전달하고 stale revision을 거부한다. |
| 4 | `AAAA`가 아닌 short `A` item이 ratio owner로 선택 | PASS — collector는 유일한 ratio item이 `AAAA`인지 검증한 뒤 그 NodeId만 반환한다. |
| 5 | Grid root나 item 외부 DOM node가 Taffy basis override를 받음 | PASS — projection option은 단일 NodeId이고 collector가 direct child owner만 전달한다. |
| 6 | marker가 누락돼도 typed `ContentBox` ratio가 기본 projection으로 통과 | PASS — 기본 entry point는 marker 없이 호출하며 projection은 ContentBox typed ratio를 fail-closed한다. |
| 7 | marker NodeId가 snapshot에 없거나 ratio typed value가 없음 | PASS — exact-node 존재와 `ContentBox` typed ratio 일치가 전제다. |
| 8 | marker는 맞지만 computed serialization이 숫자 ratio임 | PASS — `auto <ratio>` serialization·typed basis를 함께 검사한다. |
| 9 | `auto <ratio>` serialization인데 typed basis가 `BoxSizing`임 | PASS — 반대 방향 basis mismatch도 layout 전에 실패한다. |
| 10 | 두 content-box ratio owner 중 하나만 marker로 숨겨 partial 성공 | PASS — collector는 한 ratio owner 제한을 유지하고 projection도 snapshot의 다른 ContentBox value를 거부한다. |
| 11 | numeric ratio가 기존 border-box/`box-sizing` semantics에서 이탈 | PASS — 숫자 basis는 `BoxSizing`이고 marker 기반 override 대상이 아니다. |
| 12 | no-ratio control이 padding/border 또는 align 경로를 바꿈 | PASS — `layout_aspect_ratio == None`은 `auto` control이며 ContentBox override를 받지 않는다. |
| 13 | ratio marker가 scene ID나 FFI label에서 추측됨 | PASS — HostDocument direct-text collector와 same-revision style이 source of truth다. |
| 14 | `NodeId` 재사용/세대 변경 뒤 marker가 재사용 | PASS — generation·document/render-tree/style revision mismatch를 layout 전 거부한다. |
| 15 | C07.3, Flex, Block profile이 auto ratio를 갑자기 수용 | PASS — cascade admission은 C11 track-sizing profile 안으로 제한되고 다른 profile rejection을 유지한다. |
| 16 | broad C11 projection API에 숨은 ContentBox enablement 기본값이 생김 | PASS — 기본값은 `None`; dedicated fixture wrapper만 exact `NodeId`를 넘긴다. |
| 17 | computed CSS `box-sizing:border-box`가 스냅샷에서 content-box로 덮어씀 | PASS — property snapshot은 원값을 보존하고 Taffy `LayoutStyle`의 정확한 item만 변환한다. |
| 18 | width/height/min/max의 explicit size까지 ContentBox remapping 영향 | PASS — candidate item은 auto dimensions/no min-max 조건에서만 통과한다. |
| 19 | 화면 fixture에서 track/item/root overflow 차이가 사라짐 | PASS — 같은 Chrome oracle의 32⅔/33⅓ row 및 91/90 column 구분을 계속 요구한다. |
| 20 | projection gate 계획 변경을 새 기능 완료나 C11 집계로 게시 | PASS — 이 amendment도 계획-only이고 C11/C11.4 checkbox 및 `72/213`을 그대로 둔다. |

**발견 및 조정:** 기존 style projection API는 여러 C11 fixture가 공유한다. 이 때문에 profile만으로 sizing basis를 변환하는 계획은 호출자 범위가 충분히 좁지 않았다. collector의 정확한 ratio-owner `NodeId`를 전용 runtime/style-to-layout fixture entry point까지 전달하고 일치할 때만 해당 Taffy item을 변환하도록 수정했다.
