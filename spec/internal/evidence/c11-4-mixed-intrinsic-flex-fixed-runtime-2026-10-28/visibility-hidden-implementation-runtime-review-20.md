# C11.4 `visibility:hidden` mixed-track 구현·런타임 적대 검토 20개

검토 대상은 새 네 scene의 FFI/runtime adapter, intrinsic contribution, renderer paint filtering, Android/iOS matcher와 실제 시뮬레이터 실행이다. 아래 관점은 계획 검토와 분리했고 각기 다른 실패 조건을 확인했다.

| # | 별도 실패 가설 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 임의 scene ID가 visibility adapter에 들어간다 | FFI helper는 고정된 네 ID만 반환한다. exact-route Rust test가 네 장면을 받아들이고 unknown/collapse ID를 거부했다. |
| 2 | 새 adapter가 기존 mixed-track 경로를 가로채 기존 scene을 바꾼다 | 기존 scene 검색 뒤 visibility helper로 넘기며 fallback 순서를 유지한다. Android/iOS 전체 14-scene 회귀가 모두 통과했다. |
| 3 | 다른 root ID·tag·namespace·parent가 fixed fixture로 처리된다 | runtime adapter가 HTML namespace, `div` root, document-root parent와 exact root ID를 검사한다. 다른 root는 이 adapter의 scene map에 없다. |
| 4 | direct child 추가·누락·재배열이 intrinsic input 순서를 바꿔도 통과한다 | 정확히 A/span/image/C 네 child의 tag·ID·parent를 확인한다. unit fixture의 `NodeId`와 source order는 `(2,0),(4,1),(6,2),(7,3)`이다. |
| 5 | stale snapshot 또는 computed-style revision으로 layout을 계산한다 | revision validator를 root validation보다 먼저 호출한다. 기존 revision 검사 경계를 재사용한다. |
| 6 | hidden scene에서 root visibility가 상속되어 layout input을 숨기거나 잘못 paint한다 | root `visibility:visible`과 A/image/C visible 값을 요구한다. root hidden mutation을 runtime test에서 거부했다. |
| 7 | hidden ID가 `visible`이 되거나 control ID가 `hidden`이 되어도 받아들인다 | scene ID에 따라 span의 computed `visibility`를 정확히 요구한다. 양 방향의 값 교환 변조를 test가 거부했다. |
| 8 | `visibility:hidden`을 `display:none`과 같게 취급해 intrinsic input에서 뺀다 | 새 runtime input에는 span NodeId 4와 150px min-content가 있고 `[150,40,90]` tracks가 나온다. 기존 `display:none` 두 대조 장면은 동일 행렬에서 계속 통과했다. |
| 9 | hidden span은 남지만 visible A의 100px contribution이 빠진다 | layout test와 플랫폼 matcher가 A NodeId 2의 100px min-content와 span NodeId 4의 150px를 각각 확인했다. |
| 10 | visible control이 hidden scene과 다른 track 분배를 받는다 | hidden/control 네 scene 모두 양 축에서 `[150,40,90]px`, remaining space 0px을 출력했다. |
| 11 | Rows fixture에서 orthogonal text의 sizing axis를 뒤집는다 | Rows A/span/C는 `vertical-rl` paragraph이고 결과 frame은 root `120×300`, 첫 cell `120×150`이다. row hidden/control 두 장면이 앱에서 통과했다. |
| 12 | 겹치는 hidden span이 DOM source order나 NodeId를 바꿔 아래 A를 덮는다 | DOM은 A 다음 span이며 hidden frame NodeId 4가 NodeId 2와 겹친다. node frame 순서와 overlap sample의 파란 A 픽셀을 두 플랫폼에서 확인했다. |
| 13 | PNG가 hidden route에서 intrinsic input을 우회하거나 크기를 바꾼다 | 기존 URL·decode·resource snapshot 경계를 유지하고 natural size `24×16px`을 검사한다. 네 scene 모두 이미지 frame과 위치가 기대값이다. |
| 14 | trace에 visibility span의 `Minimum`/`MinContent` 단계 기여가 누락·중복된다 | 앱 matcher와 독립 verifier는 item 2/4의 네 contribution record를 종류·span·값·flex crossing 기준으로 집합 비교한다. runtime trace에서 span min-content 150px가 보였다. |
| 15 | flex-stage 결과가 올바른 trace와 달라진다 | Columns/Rows target pass가 `ExpandFlexibleTracks` 뒤 `[150,40,90]`이고 fixed neighbor는 40px, 두 gutter는 각각 10px이다. 양 플랫폼 manifest에서 전 scene 검증했다. |
| 16 | hidden element의 layout frame 부재를 정상 paint suppression으로 오인한다 | hidden item은 full-size `120×150` 또는 `150×120px` frame과 frame record를 유지한다. box/draw count는 4로 분리됐다. |
| 17 | hidden box 배경 또는 Ahem text가 실제 renderer box 목록에 남는다 | renderer는 frame 유효성 확인 뒤 computed `visibility:hidden` box를 snapshot에서 생략한다. Rust render test는 frame 유지와 root+visible sibling 두 painted box를 확인했다. |
| 18 | visible positive control을 잘못 설정해 숨김 표본 위치가 맞는지 알 수 없다 | visible control은 같은 overlap 좌표에서 purple `[142,68,173,255]`를 내고 hidden scene은 blue `[40,100,220,255]`를 낸다. Android PixelCopy와 iOS Metal readback이 exact match했다. |
| 19 | pixel 좌표 변환·fixed-gap 배경 표본이 잘못돼도 화면 검증이 통과한다 | independent verifier가 CSS 좌표, drawable scale과 device pixel coordinate를 재계산하며 blue/purple overlap 및 `[232,237,245,255]` gap sample을 확인한다. 좌표·색 mutation은 negative control에서 거부된다. |
| 20 | 새 route가 다른 플랫폼/profile로 fallback하거나 시뮬레이터 로그를 화면 근거로 과장한다 | unknown scene·visible-control reject·C11.2.1 profile conflict를 양 앱에서 기록했고 전체 14/14를 current Android API 37 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 재실행했다. 실기기·hardware GPU·global pixel parity는 확인하지 않았다. |

Focused Rust test와 renderer test, `cargo fmt`, fixture FFI/library Clippy가 통과했다. `spinon-runtime`/`spinon-style-to-render --all-targets` Clippy는 변경하지 않은 `generic_direct_text_image/fixture.rs:311`의 기존 `clippy::map_identity`로 실패했다. library-only Clippy는 통과했으며 archive 경계는 [artifact-boundary 기록](artifact-boundary.md)에 정리한다.
