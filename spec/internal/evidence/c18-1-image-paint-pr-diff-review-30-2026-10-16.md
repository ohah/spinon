# C18.1 PR diff 적대 검토 — 2026-10-16

구현 전 계획 20개 관점과 구현 코드 20개 관점 검토를 반복하지 않고, 최종 PR diff의 입력·소유권·렌더 제출·플랫폼 경계·실행 근거에서 서로 다른 실패 조건 30개를 확인했다. 표본 실행은 Android API 37 ARM64 AVD run `636ee4a82a20` (PID `28792`)과 iPhone 17 Pro/iOS 26.2 Simulator PID `66309`의 최신 opt-in 앱에서 수집했다. 최종 코드 테스트와 두 capture 비교 결과는 [validation evidence](./c18-1-image-paint-validation-2026-10-16/README.md)에 보존한다.

## 검토 항목

| # | 적대 관점 | 최종 diff 대조와 판정 |
| ---: | --- | --- |
| 1 | 호출자가 임시 RGBA 버퍼를 버린 뒤 renderer가 해제된 메모리를 참조 | `RuntimeRenderImage`는 `Arc<[u8]>`를 소유한다. decode completion에서 복사한 뒤 scene과 함께 살아 있어 소유권이 분명하다. |
| 2 | 다른 문서 generation의 이미지 bytes를 현재 장면에 붙임 | `RuntimeImageResourceKey::matches`가 generation을 포함해 비교한다. 다른 HostDocument generation은 runtime attachment 검사에서 거부된다. |
| 3 | document/render-tree/intrinsic-size revision 하나가 달라도 cache reuse | resource key와 cache key에 세 revision이 포함된다. intrinsic revision은 image cache identity 단위 검사에서 구분된다. |
| 4 | image를 요청한 뒤 같은 render tree에서 document revision만 바뀐 오래된 scene 재발행 | image가 붙은 snapshot은 `with_document_snapshot_revision`에서 revision 변경을 거부한다. 전용 회귀 테스트가 같은 tree revision의 변경 document를 확인한다. |
| 5 | scene에 없는 NodeId의 image를 renderer까지 전달 | `with_images`가 일치하는 render box를 찾지 못하면 `ImageNodeMissing`으로 반환한다. |
| 6 | 한 NodeId를 중복해 두 texture를 겹쳐 제출 | `with_images`가 중복을 `DuplicateImage`로 거부한다. |
| 7 | `width × height × 4`가 정수 범위를 넘어 실제 bytes와 비교가 뒤틀림 | checked multiplication과 `usize` 변환 뒤 길이를 비교한다. overflow 입력 단위 테스트가 있다. |
| 8 | fixture에 다른 namespace의 `img`, 빠진 요소, 추가 요소가 섞임 | fixture traversal은 HTML namespace와 `img`를 세고 고정 ID 두 개만 받아들인다. 빠짐·예상 밖 ID·중복 ID negative control이 있다. |
| 9 | 고정 ID는 맞지만 `src`가 바뀐 요청을 원래 PNG로 대체 | `ImageRequest::from_document` 결과의 UTF-16 `src`를 고정 값과 비교한다. 변경 `src` fixture negative control이 실패한다. |
| 10 | decode completion의 source가 scene document와 다른 상태로 report만 일치 | 같은 `HostDocumentSnapshot`에서 request와 `ImageResourceSnapshot`을 만들고 그 generation/document/tree/intrinsic revision을 runtime image key에 전달한다. |
| 11 | layout이 image natural size와 다른 크기로 확대·축소했는데 natural-size profile로 통과 | scene 결합과 WGPU geometry 양쪽에서 frame width/height가 decoded dimensions와 정확히 같은지 확인한다. |
| 12 | image 자체의 painted border가 texture에 의해 가려짐 | painted side가 하나라도 있으면 attachment를 `UnsupportedImageBorder`로 거부한다. 검사 기준은 nonzero width와 실제 paint이며 border 없는 fixture는 통과한다. |
| 13 | hidden 조상의 padding-edge clip이 자식 image에 적용되지 않음 | image geometry는 해당 render box에 투영된 `clip_css_px`와 frame의 교집합을 사용한다. API 37 PixelCopy와 iOS Metal의 hidden padding/border/outside 세 표본이 맞았다. |
| 14 | clip으로 destination만 잘려 source가 늘어나거나 UV가 다른 영역을 읽음 | 교집합을 frame 비율로 변환해 UV에도 적용한다. geometry unit test가 UV와 상단 행을 확인한다. |
| 15 | clip과 frame이 만나지 않는데 이전 image vertex 범위를 재사용 | 빈 교집합은 빈 draw range로 기록하고 `draw_image`가 제출하지 않는다. fully-clipped geometry 테스트가 있다. |
| 16 | NaN·역전된 rect·0폭 quad·UV 바깥 범위를 GPU에 전달 | quad builder가 finite rect/UV, 양의 너비·높이, `[0,1]` UV 순서를 검사한다. 실패는 renderer geometry error로 전파된다. |
| 17 | CSS 좌표의 위·아래가 GPU 좌표계와 바뀌어 PNG가 뒤집힘 | quad 변환에서 CSS top을 NDC top으로 바꾸고 vertex/UV 순서를 고정한다. GPU geometry와 두 플랫폼의 asymmetric color sample이 같은 방향이다. |
| 18 | texel 경계에서 선형 보간 또는 가장자리 반복이 다른 색을 생성 | sampler는 nearest와 clamp-to-edge를 사용한다. Chrome oracle과 두 backend 표본에서 픽셀 중심을 exact 비교했다. |
| 19 | PNG의 alpha channel이 opaque-only pipeline에서 잘못 덮임 | texture 준비가 모든 alpha byte가 255인지 검사하고, 그 외에는 명시적 오류를 낸다. 반투명 device negative 실행은 범위 밖으로 기록했다. |
| 20 | source sRGB, surface sRGB·UNORM 조합 중 색 공간이 이중 변환됨 | image shader와 output-encoding uniform이 surface format을 분기한다. Android UNORM 경로와 iOS sRGB 경로에서 9개 색이 각각 exact였다. |
| 21 | device가 허용하지 않는 큰 PNG texture를 error 없이 제출 | `device.limits().max_texture_dimension_2d`를 확인한다. 실제 texture-limit 초과 입력과 GPU OOM은 주입하지 않았다. |
| 22 | texture/bind/upload 또는 vertex buffer 준비 오류가 render pass/present까지 진행 | image prepare는 current surface texture를 얻기 전에 완료되고 오류는 조기 반환한다. renderer 경로는 pending surface sample도 버린다. WGPU failure injection 및 같은 renderer 후속 복구 draw는 실행하지 않았다. |
| 23 | image vertex 수 또는 byte 용량 변환이 `u32`/`u64`를 넘음 | vertex 개수·byte 길이·다음 2의 거듭제곱 용량에 checked conversion/calculation과 `try_reserve`를 사용한다. |
| 24 | 같은 node·dimensions지만 다른 bytes 또는 resource revision이 cache hit | `ImageCacheKey`가 source tuple, NodeId, dimensions, decoded RGBA SHA-256을 포함한다. 동일·변경 resource test가 key 구분을 확인한다. |
| 25 | renderer가 재생성됐는데 다른 device의 texture/bind group을 재사용 | GPU image map은 해당 `RuntimeImageRenderer` instance에 소유된다. 다른 renderer 간 공유 정적 cache는 없다. simulator에서 강제 재생성 검사는 하지 않았다. |
| 26 | 이미지가 자신의 background/border 또는 부모 border보다 먼저 그려져 stacking 결과가 바뀜 | surface와 headless readback이 기존 box 순서대로 각 box geometry를 그린 뒤 그 node image를 제출한다. fixture의 child image는 부모 border 뒤에 그려지고 visible border-overpaint 표본이 확인한다. |
| 27 | FFI가 null 포인터·틀린 sample count·불충분 capacity를 읽음 | fixture FFI는 host/output/coordinates를 검사하고 표본 9개, 좌표 18개 조건을 확인한다. C 선언은 pointer lifetime과 동시 호출 금지를 적는다. |
| 28 | 기본 제품 artifact에 fixture adapter·route·고정 payload가 섞임 | Cargo feature, Android Gradle source set/define, iOS build define과 selector가 opt-in으로 짝을 이룬다. 기본과 positive-control Cargo archive·APK·iOS app 8개를 byte scan해 전용 문자열·97-byte PNG의 부재/포함을 확인했다. 변경한 Android builder는 `llvm-nm | grep -q` 조기 종료가 `pipefail` 아래 `llvm-nm` SIGPIPE로 실패하던 검사를 output을 끝까지 읽는 `grep`으로 바꿨다. iOS Simulator wrapper는 build phase가 읽는 `SPINON_V8_DIR`를 두 `xcodebuild` 경로 모두에 전달한다. 기본·opt-in Android와 iOS Simulator 앱이 빌드되고 실행됐다. |
| 29 | Android Activity가 fixture flag 없이 진입하거나 PixelCopy를 다른 run/PID에 연결 | build flag와 intent extra가 모두 있어야 route가 활성화된다. 최신 API 37 AVD의 run ID/PID를 로그·surface PNG·screenshot manifest로 결합했고 PixelCopy/screenshot 각 9/9, 바깥 anchor 4/4가 일치했다. 연결된 실기기는 사용하지 않았다. |
| 30 | iOS route 또는 compositor 비교가 다른 실행/잘못된 화면 좌표를 성공 처리 | compile define과 `--spinon-c181-image-paint`가 모두 있어야 실행한다. PID-filtered Metal log와 latest executable hash, full-screen image를 묶었고 Metal/screenshot 각 9/9 및 anchor 4/4가 맞았다. comparator suite의 hash·좌표·geometry·anchor negative controls도 통과했다. |

## 판정

최종 diff에서 병합을 막는 결함은 발견되지 않았다. C18.1 상태 대장·내부 README의 새 링크는 모두 존재한다. 변경 파일 전체 Markdown 링크 검사에서 나온 네 개의 누락 경로는 기준 `origin/main`의 C11/C12 설명에도 이미 있던 항목이어서 이번 변경 범위 밖으로 남겼다. 실행 범위는 고정된 불투명 natural-size PNG를 두 simulator에서 그리는 내부 C18.1 slice다. URL/network loader, 비동기 교체, alpha blending, scaled sampling, `object-fit`/`object-position`, 실기기·hardware GPU·OS scanout은 완료로 주장하지 않는다. malformed/oversized PNG mutation, texture-limit/OOM injection, renderer 재생성·실패 후 복구 draw도 미검증으로 남긴다.
