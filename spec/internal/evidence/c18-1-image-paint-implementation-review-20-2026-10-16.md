# C18.1 이미지 페인트 구현 적대 검토 — 2026-10-16

계획 검토표와 겹치지 않도록 구현 코드·실행 결과를 대상으로 입력 소유권, scene 결합, WGPU 제출, pixel mapping 및 플랫폼 경계의 실패 경로 20개를 따로 확인했다. 리뷰 중 찾은 두 가지 문제는 수정했다. `geometry.rs`가 500줄을 넘어 image quad 구현을 `geometry/image.rs`로 분리했고, 계획에 적힌 미실행 오류 주입을 완료한 것처럼 읽히지 않도록 완료 경계와 결과 문구를 줄였다. 모듈 분리 뒤 `spinon-render` 79개 및 `spinon-render-wgpu` 22개 unit + 3개 integration 테스트를 다시 실행했다.

## 구현 관점 20개

| # | 실패 관점 | 대조 근거 및 판정 |
| ---: | --- | --- |
| 1 | 다른 document generation에서 요청한 bytes가 현재 scene에 붙음 | `RuntimeImageResourceKey::matches`가 generation을 비교한다. 다른 HostDocument generation은 `runtime_image_attachment_validates_source_node_bytes_and_natural_frame`에서 거부한다. |
| 2 | 같은 generation에서 document revision만 바뀐 stale image scene을 다시 발행 | `with_document_snapshot_revision`은 image가 있으면 document revision 재발행을 거부한다. `image_scene_rejects_document_revision_rebinding_after_resource_request`가 확인한다. |
| 3 | source render-tree revision과 현재 frame tree가 다름 | resource key에 render-tree revision이 포함되고 `with_images`가 현재 RuntimeRenderKey와 비교한다. stale frame과의 결합은 `InvalidImageSource`로 거부한다. |
| 4 | intrinsic-size revision이 바뀌었지만 이전 bytes를 재사용 | intrinsic-size revision이 image resource key의 일부이며 cache key에도 포함된다. revision 변경을 cache identity test가 구분한다. |
| 5 | 한 NodeId에 image resource가 중복 연결 | `with_images`의 중복 NodeId 검사가 `DuplicateImage`를 반환한다. runtime attachment 단위 테스트에서 확인했다. |
| 6 | scene에 없는 NodeId 또는 연결되지 않은 image | snapshot 결합은 `ImageNodeMissing`으로 거부한다. 고정 fixture traversal도 예상된 DOM image 집합 외의 항목을 거부한다. |
| 7 | 0 크기·overflow·RGBA byte length 불일치 | `RuntimeRenderImage::new`은 checked `width × height × 4`를 적용한다. `u32::MAX × 2` byte 계산이 거부되는 단위 검사가 있다. PNG decoder의 malformed/oversized 입력은 이 paint fixture에서 다시 주입하지 않았다. |
| 8 | decoded natural size와 layout frame이 서로 다름 | `with_images`는 frame width/height를 decoded dimensions와 비교한다. 잘못된 frame을 `InvalidImageFrame`으로 거부하는 단위 검사가 있다. |
| 9 | image 본체가 자신의 painted border를 덮음 | image scene 결합은 실제로 칠해지는 side가 있으면 `UnsupportedImageBorder`로 거부한다. 해당 거부 단위 검사를 추가하고 실행했다. |
| 10 | fixture에서 빠짐·추가·중복 `<img>`가 예상 두 요청을 바꿈 | `image_nodes`는 정확한 두 HTML namespace `img`, 고유 고정 ID를 요구한다. missing, unexpected, duplicate 항목을 각각 호출하는 fixture 단위 검사가 있다. |
| 11 | 고정 ID가 맞아도 `src`가 변경됨 | `ImageRequest::from_document` 뒤 고정 URL을 비교한다. fixture 테스트가 변경된 `src` 요청을 거부한다. 일반 비동기 src replacement와 loader scheduling은 구현 범위 밖이다. |
| 12 | visible image가 부모 border 아래로 가거나 나중 형제에 덮임 | renderer가 scene box를 paint order대로 순회하며 각 box의 background/border 뒤에 그 box의 image를 제출한다. Chrome·Android PixelCopy·iOS Metal 표본의 border-overpaint 색이 일치한다. |
| 13 | ancestor clip에서 destination만 줄고 UV는 그대로 남음 | [`geometry/image.rs`](../../../crates/spinon-render-wgpu/src/geometry/image.rs)는 교차한 frame 비율로 UV를 함께 자른다. `image_clip_adjusts_uv_with_destination_and_preserves_top_row`가 UV와 위쪽 행을 검사한다. |
| 14 | clip 교차가 비어 있는데 잘못된 quad를 제출 | clip이 완전히 밖에 있으면 empty draw range를 만든다. `fully_clipped_image_has_an_empty_draw_range`가 확인한다. 임의 NaN UV mutation을 별도 주입하지 않았다. |
| 15 | alpha가 포함된 PNG가 blend 없는 opaque pipeline에 들어감 | `RuntimeImageRenderer::prepare`는 alpha가 255가 아닌 pixel을 거부한다. fixed opaque fixture와 두 simulator readback은 성공했다. 반투명 픽셀을 device에 넣는 별도 실패 실행은 하지 않았다. |
| 16 | cache key가 같은 node의 변경 bytes 또는 다른 renderer resource를 혼동 | cache key는 source tuple, NodeId, dimensions, decoded RGBA SHA-256을 포함하고 cache는 renderer 인스턴스 소유다. source/node/content/dimensions 분기를 unit test로 구분했다. renderer destruction/recreation을 simulator에서 강제하지 않았다. |
| 17 | texture 한도·upload descriptor·allocation 실패가 present까지 진행 | byte 수와 row byte overflow를 검사하고 texture/bind/upload 경계에 Validation·OutOfMemory scope가 있다. 지정 AVD/Simulator에서 성공 경로만 관찰했다. 기기 texture limit 초과, WGPU validation failure, 실제 OOM은 주입하지 않았다. |
| 18 | FFI가 null·잘못된 sample count·중복 좌표를 읽음 | FFI는 null/capacity/count를 검사하고 중복 좌표를 거부한다. 표본 좌표 범위는 readback boundary가 drawable 크기로 확인하며 renderer 단위 검사가 있다. FFI 자체의 null/count만 현재 별도 test로 고정했고 중복 FFI 좌표는 source-level 확인이다. |
| 19 | manifest hash가 유효해도 잘못된 sample 좌표·pixel·anchor를 통과 | comparator tests는 screenshot/surface pixel 변경, hash 불일치, log 좌표/exact flag, geometry origin, fixture 내부/중복색 anchor를 거부한다. 실제 Android·iOS manifest 모두 9 sample + 4 anchor exact였다. |
| 20 | 한 플랫폼 화면만 확인되거나 opt-in fixture가 기본 제품 artifact에 포함 | API 37 ARM64 AVD PixelCopy/full-screen과 iPhone 17 Pro iOS 26.2 Simulator Metal/full-screen을 같은 Chrome oracle로 비교했다. Cargo, APK, iOS executable default scan은 고정 fixture payload/route가 없고 opt-in positive control에는 있다. 물리 device·hardware GPU·실제 OS scanout은 검사하지 않았다. |

## 결과

- 이 검토에서 수정한 구현 구조 문제는 이미지 geometry 분리 뒤 재실행한 테스트에서 통과했다.
- Android API 37 ARM64 AVD run `636ee4a82a20` (PID `28792`, opt-in APK SHA-256 `5ccbbf60c918a33c985b4a6ca7f68682c453d60bf5bdf590e72ec6f3d112e3cf`, embedded library SHA-256 `a99e17063d472ca1b423719e71f9f99809a3be32b6995a65f78434e7d3b11e7f`)의 PixelCopy·surface PNG·screenshot comparator는 각각 `9/9`, screenshot anchor `4/4`였다. iPhone 17 Pro/iOS 26.2 Simulator run PID `66309` (executable SHA-256 `a40b9d549fd9ec31c388311dc69e39b6e3b8fba5b22856c0092731b778377479`)은 Metal readback/screenshot `9/9`, anchor `4/4`였다.
- 검증하지 않은 GPU failure injection, PNG decoder malformed/oversized mutation, renderer recovery after image preparation failure는 이 slice의 완료 근거로 사용하지 않는다.

계획의 독립 20개 검토는 [C18.1 계획 적대 검토](./c18-1-image-paint-plan-review-2026-10-16.md)다. 이 구현 검토표는 계획 검토 결과를 복사하지 않았다.
