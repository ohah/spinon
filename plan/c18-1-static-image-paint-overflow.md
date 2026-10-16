# C18.1 · decoded PNG 이미지 페인트와 부모 overflow pixel

## 목표

현재 `spinon-image`가 디코드한 RGBA8 PNG와 C14.1–C14.3 layout frame을 `spinon-style-to-render`와 WGPU renderer까지 연결한다. HTML `img`가 실제 Android/iOS 앱 표면에 그려지는지, 부모 `overflow:visible`에서 border box 바깥 pixel이 보이고 `overflow:hidden`에서 padding edge 뒤가 잘리는지 고정 Chrome pixel oracle로 판정한다.

이 단계의 fixture는 기존 고정 24×16 PNG를 caller-owned bytes로 공급한다. URL resolution/network loader/cache, 비동기 source replacement, 이미지 alpha blending, 크기 조정 sampling, `object-fit`·`object-position`, background image는 별도 미완료다. C18.1은 실제 WGPU image paint를 추가하지만 U05, C14, C18 parent 또는 공개 CSS 지원을 완료로 표시하지 않는다.

## 기존 구현 경계

- `spinon-image`는 caller-owned PNG byte slice를 `DecodedImage`와 `ImageResourceSnapshot`으로 바꾸지만 앱에서 URL을 읽거나 GPU texture를 만들지 않는다. 기준은 [0061 U05.1](../spec/internal/0061-u05-png-image-decode.md)이다.
- `spinon-style-to-render`는 ancestor clip과 border를 가진 `RuntimeDrawBox`를 만든다. C13.2는 일반 descendant box가 부모 border 밖에 그려지는 것을 AVD PixelCopy/iOS Metal 표본으로 확인했지만 image texture는 다루지 않았다.
- C14.3의 `fit-content-small-basis`는 natural-size image frame이 부모 오른쪽 border 밖으로 7 CSS px 나오는 것을 측정했다. 기존 evidence는 geometry만 검사하고 image pixel paint는 검사하지 않았다.
- 이 단계는 현재 live V8→HostDocument→Stylo→Taffy→WGPU 경로에 opt-in image fixture를 더한다. fixture-owned PNG bytes를 넣는 것은 제품 URL loader를 검증한 것으로 보지 않는다.

## 규범 기준

- [CSS Overflow Module Level 3 §3.1](https://drafts.csswg.org/css-overflow-3/#overflow-control)은 `visible`에서 content clipping을 하지 않고 `hidden`에서 overflow clip edge에 자르는 기준이다. [§3.2](https://drafts.csswg.org/css-overflow-3/#overflow-clip-margin)는 별도 visual box가 없을 때 clip edge의 기본 origin을 padding box로 둔다.
- [CSS 2.2 Appendix E.2](https://www.w3.org/TR/CSS22/zindex.html#painting-order)는 block의 background/border 뒤에 in-flow descendants와 replaced content를 paint한다. 따라서 visible image가 부모 border 위에 그려지는 것이 고정 fixture의 기대 동작이다.
- 위 문서의 범위와 pinned Chrome pixel observation을 함께 판정 기준으로 쓴다. WGPU 구현의 표준 적합성을 이 단일 fixture로 확대하지 않는다.

## 고정 입력과 독립 oracle

- Oracle: Agent Browser의 Google Chrome `155.0.8059.40`, macOS, DPR 1.
- Fixture: [image-overflow-pixels.html](../tests/fixtures/css/c18/image-overflow-pixels.html), 80×40 CSS px. image는 `24×16` PNG `c14-1-png-basic.png`이며 source SHA-256은 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다.
- Parent 두 개는 각각 `12×16` content box, 사방 `4px` padding, `1px` solid border를 쓴다. `visible` parent는 `(4,4) 22×26`, image는 `(9,9) 24×16`이고 image가 border box 바깥 7px까지 보인다. `hidden` parent는 x=40에서 같은 크기로 시작하며 image는 `(45,9) 24×16`이다.
- Chrome 원본 pixel, image resource, fixture, viewport, 전체 관찰 및 SHA-256은 [사전 비교 근거](../spec/internal/evidence/c18-1-image-paint-precomparison/README.md)에 고정한다.

| sample | CSS pixel | 예상 RGBA | 판정 |
| --- | ---: | --- | --- |
| visible-content-positive | `(9,9)` | `(255,255,64,255)` | 원본 이미지 시작 pixel |
| visible-border-overpaint | `(25,10)` | `(0,255,64,255)` | visible child가 부모의 오른쪽 border 위에 페인트 |
| visible-overflow-first | `(26,10)` | `(0,255,64,255)` | 부모 border box 바깥 첫 pixel |
| visible-overflow-middle | `(29,10)` | `(0,255,64,255)` | overrun 내부 pixel |
| visible-overflow-last | `(32,10)` | `(0,255,64,255)` | border box 바깥 마지막 image pixel |
| visible-after-image | `(33,10)` | `(16,24,39,255)` | 이미지 다음 배경 control |
| hidden-padding-positive | `(60,10)` | `(0,255,64,255)` | hidden parent padding edge 안쪽 |
| hidden-border-positive | `(61,10)` | `(240,198,116,255)` | clip 뒤에도 남는 부모 border |
| hidden-outside | `(62,10)` | `(16,24,39,255)` | border 바깥 배경 control |

## 설계와 소유권

1. `ImageResourceSnapshot.source()`의 generation/document/render-tree tuple이 `HostDocumentSnapshot` 및 `RuntimeRenderKey`와 모두 같고 `ImageResourceSnapshot.revision()`이 intrinsic-size revision과 같을 때만 연결한다. 선택된 fixture의 두 `img` NodeId 집합도 resource snapshot의 image key 집합과 정확히 같아야 한다. 중복, 누락, 예상 외 NodeId, 분리 요소는 draw command 생성 전에 구체 오류로 끝난다.
2. 요청 때 저장된 `src`와 현재 document의 `src`, PNG encoded SHA-256, 디코드 RGBA SHA-256, dimensions, NodeId, source tuple을 하나의 입력 report로 결합한다. 문서 revision이 바뀌면 `with_document_snapshot_revision`처럼 render snapshot의 document revision만 다시 발행하는 경로가 이전 image를 보존하지 못하게 한다. 새 document의 요청·image snapshot을 다시 맞추지 못하면 image scene을 거부한다.
3. `DecodedImage::rgba8()`의 길이가 checked `width × height × 4`와 같고 device 최대 texture dimension 이내인지 검사한다. fixture 이미지 두 장만 업로드하며 크기·메모리 한계를 넘어가거나 WGPU validation/allocation이 실패하면 frame 제출 전 구체 오류로 끝난다. texture/view/sampler/bind group 생성과 upload descriptor는 중첩 `Validation`/`OutOfMemory` error scope로 감싸고 scope를 역순으로 회수해 오류를 surface acquire/submit 전에 반환한다. WGPU 30.0.1의 `Queue::write_texture`에 행당 `width × 4` bytes를 공급하고 256-byte row alignment 규칙을 적용하지 않는다. 이 API에는 해당 복사 alignment 요구가 없음을 고정 dependency source와 대조한다.
4. 이 fixture PNG는 color-profile chunk가 없는 RGBA8 sRGB sample이며, 디코드 바이트는 그대로 보존한다. GPU texture는 `Rgba8UnormSrgb`로 샘플링하고 nearest/clamp sampler를 쓴다. sRGB surface format에서는 sampled linear 값을 출력하며, sRGB color-space의 non-sRGB surface format에서는 기존 단색 경로처럼 linear-to-sRGB encode를 적용한다. 두 분기 모두 Chrome의 8-bit sample과 exact 비교한다.
5. texture, view, sampler, bind group은 `RuntimeRenderer` 인스턴스가 소유한다. 캐시는 그 인스턴스 안에서 source tuple·intrinsic resource revision·NodeId·RGBA hash·dimensions로 식별하고 다른 renderer/device에 공유하지 않는다. resize는 surface generation만 갱신하고 device loss 이후 복구는 기존 device cache를 폐기한 새 renderer에서 시작한다. report는 renderer/device generation과 surface generation을 구분한다.
6. image command는 기존 순서의 각 box background 다음, 자신의 border 이전에 삽입한다. image 노드가 아닌 조상 box의 border 뒤에 자식 image가 오도록 현재 flattened stacking order를 보존한다. 고정 C18.1 fixture는 image element 자체의 border/padding 및 transform을 적용하지 않는다. render snapshot은 칠해지는 image border를 연결 시 거부한다. generic render box에는 padding/transform 정보가 없으므로 이 fixture 밖의 해당 style 거부를 주장하지 않는다. image frame은 natural dimensions와 exact 일치해야 하며 alpha가 있는 pixel은 하나라도 있으면 unsupported 오류다.
7. image quad는 frame, paint clip, source UV를 함께 가진다. `CssClip` 경계에서 destination rect를 자를 때 같은 비율로 UV도 보정하여 이미지를 재축소·늘이지 않는다. clip의 반열린 max 경계, 비어 있는 교차, 화면 밖 좌표, 비유한 frame/UV, 음수 또는 초과 UV를 별도 검사한다. hit-test clip은 paint에 사용하지 않는다.
8. image source의 위쪽 행은 CSS frame의 위쪽 행에 대응하고 UV 범위는 `[0,1]`을 넘지 않는다. 1:1 CSS 크기에서도 기기 surface scale 때문에 다중 물리 pixel이 생길 수 있으므로 nearest filtering과 texel 내부의 sample point를 고정한다. 기본 artifact에 image 입력이 없는 장면의 정점·paint 순서·색상 결과가 이전과 같아야 한다.
9. image 준비·geometry 검증·bind group 검증에 실패하면 해당 draw는 `draw_if`의 surface submit/present 단계로 진행하지 않고 이전 표본을 성공 결과처럼 반환하지 않는다. 실패 후 같은 renderer/session의 정상 fixture draw가 새 revision으로 성공해야 한다. resize는 보유 texture 재사용을 허용하지만 새 surface frame을 다시 획득하고 sample을 새 좌표로 계산한다.
10. Android PixelCopy 표본과 iOS Metal surface readback은 WGPU surface 결과의 사전 compositor evidence로 따로 기록한다. 각 run은 app build hash, source tuple, image hashes, CSS viewport, physical surface size, viewport-to-surface 비율, renderer/device·surface generation, draw submission/frame identity를 한 report로 묶는다. 표본 좌표는 CSS pixel 중심을 surface scale로 변환하고 정수 pixel의 중앙에서 읽으며, 범위 밖 표본은 거부한다.
11. post-compositor 증거는 Android AVD 전체 화면 capture와 iOS Simulator 전체 화면 screenshot으로 따로 수집한다. app content view의 화면상 원점·크기와 CSS viewport transform을 report에 기록하고 fixture 밖의 서로 다른 색상 anchor 네 점으로 좌표 mapping을 검증한다. Anchor가 기대 위치·색과 맞지 않으면 screenshot 표본 판정은 무효다. PNG 전체 byte equality는 요구하지 않고 9개 대응 pixel만 exact 비교한다.
12. 기존 sRGB·vertex pipeline과 app foreground 상태를 유지한다. AVD density를 CSS 좌표에 중복 적용하지 않고 실제 runtime surface/viewport 비율을 쓴다. Android SurfaceView PixelCopy/Metal surface readback과 전체 화면 screenshot의 캡처 API·시점·색 공간을 report에 분리한다. 앱이 가려졌거나 다른 surface generation/frame이면 캡처를 폐기한다.
13. Android/iOS 고정 fixture ABI와 PNG payload는 `c18-1-image-paint-fixture-test` opt-in feature 및 양쪽 app build flag 아래 둔다. 기본 Cargo artifact 및 Android/iOS 기본 app archive에는 fixture symbol·PNG label/bytes·전용 route가 없어야 하고 opt-in positive control에는 포함되어야 한다.

## 실행 순서

1. 이 계획을 독립된 20개 실패 관점으로 먼저 검토하고 P1/P2 발견은 구현 전에 계획에 반영한다. 계획 검토표를 구현 검토에 재사용하지 않는다.
2. Chrome fixture/oracle의 source hash, viewport, geometry와 9개 RGBA sample을 machine-readable comparator로 고정한다. comparator는 빠진/중복 sample, 다른 run, stale generation, 잘못된 viewport, PNG hash 변경과 sample coordinate 변조를 거부한다.
3. `spinon-style-to-render`와 `RuntimeRenderSnapshot`에 source/revision-bound image draw input을 연결하고 `with_document_snapshot_revision`의 stale image 보존 경로를 막는다. HostDocument→Stylo→Taffy frame, image NodeId 일치 및 C13 clip/paint order를 검증한다.
4. WGPU 30.0.1 renderer에 RGBA8 sRGB texture upload, nearest/clamp sampler, texture/image pipeline과 같은 노드 paint command stream을 추가한다. background→image→border의 ordering, clipped UV, surface format별 color 변환을 pure geometry/command tests로 고정한다.
5. live V8 document가 예상 NodeId의 두 `img[src]` element를 만들고 동일 PNG를 요청하는 opt-in Android/iOS route를 추가한다. fixture provider는 고정 bytes만 전달한다. report에 encoded/decoded hash, source tuple, NodeId, CSS frame/content rect, UV, color format, renderer/device·surface generation과 frame identity를 기록한다.
6. negative control은 구현 가능한 입력 경계마다 고정한다. Runtime snapshot의 source revision·NodeId·RGBA byte length·natural frame·칠해지는 border 거부, fixture의 고정 img ID/src/cardinality, clip/UV geometry, FFI null·sample-count 입력, comparator의 image pixel/hash/좌표·anchor 검사를 검증한다. 고정 PNG 외 malformed/oversized decode, device 최대 dimension 초과, WGPU validation/OOM 주입, padding/transform 표현, renderer recreation, image 준비 오류 뒤 같은 renderer/session의 복구 draw는 실행하지 않은 경우 별도 미검증으로 기록한다. 실제 GPU OOM을 유도했다고 주장하지 않는다.
7. Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal surface readback에서 위 9개 표본을 Chrome과 RGBA exact 비교한다. 각 캡처를 같은 source/document/style/render/surface/frame identity로 묶는다. Android AVD 전체 화면 capture와 iOS Simulator 전체 화면 screenshot에서도 app content view 원점·크기 및 네 anchor 검증을 통과한 뒤 대응 sample을 exact 비교한다. Metal readback과 post-compositor 결과, software/hardware renderer 제한을 분리해 기록한다.
8. 기본/opt-in Cargo artifact 및 Android/iOS 기본/fixture-test app archive를 비교하고 fixture-boundary, workspace tests·Clippy·rustfmt와 변경된 build/run 명령을 검증한다. 실행하지 않은 실기기·hardware GPU 검증은 주장하지 않는다. 전체 화면 screenshot은 Simulator/AVD app foreground, 현재 surface generation, 화면상 view geometry를 증거로 남기며 페이지 사이트 배포는 하지 않는다.
9. 구현이 끝나면 계획과 중복되지 않는 새로운 20개 구현 실패 관점 검토를 수행한다. 수정한 관점을 재검증한 뒤 PR diff를 별도 30개 적대 관점으로 검토한다. PR 본문에는 Android·iOS 실제 app screenshot과 evidence 링크를 넣고 Agent Browser에서 GitHub 이미지 URL 로드를 확인한다.
10. `spec/internal/0064-c18-1-image-paint.md`, internal README, `spec/STATUS.md`, `docs/architecture.md`, CSS support summary와 Tailnet preview를 실제 결과에 맞춘다. C18.1만 완료 표시하고 C18/U05/C14 및 공개 CSS 지원은 미완료로 둔다. GitHub Pages는 배포하지 않는다.

## 완료 판정

- Android AVD와 iOS Simulator가 각각 같은 live V8 image nodes를 layout하고 실제 WGPU texture draw를 제출한다.
- 두 플랫폼에서 9개 CSS sample의 RGBA가 Chrome 155 oracle과 exact 일치한다. `visible` 7px overflow, border overpaint, `hidden` padding-edge clip이 모두 검증되고 각 sample의 document/style/render/surface identity가 일치한다.
- 두 플랫폼의 post-compositor screenshot에서도 sample들이 같은 기대 색을 보여 app UI에서 이미지·border-overflow 결과를 확인할 수 있다.
- 고정된 snapshot/fixture 경계와 clip/UV 및 pixel comparator negative controls가 거부 결과를 보인다. 실행하지 않은 decoder·device error 주입과 renderer recovery는 완료로 주장하지 않고 명세 한계에 적는다.
- 화면 캡처는 report text가 아니라 실제 이미지와 두 overflow 동작을 보여준다. 기본 artifact에는 fixture code/payload가 없다.

이 판정은 opaque, natural-size, static PNG 한 장의 Android/iOS Simulator paint/profile에 한정한다. 실제 URL loader/cache, network/auth/redirect policy, asynchronous replacement/re-layout, alpha blending, scaled filtering, `object-fit`/`object-position`, SVG/다른 image format, 실기기·hardware GPU·OS compositor 이후 pixels 및 전체 C18/C14/U05 지원은 완료가 아니다.

## 구현·검증 실행 결과 · 2026-10-16

- [구현 전 비교 모델](../spec/internal/evidence/c18-1-image-paint-precomparison/README.md)의 Chrome 155 fixture, 9개 RGBA oracle, PNG hash를 고정한 뒤 구현했다. live V8 report의 generation `1`, document revision `27`, render-tree revision `5`와 이미지 두 개의 encoded/decoded SHA 및 frame/clip 정보를 기록했다.
- Android API 37 ARM64 `emulator-5554` AVD의 OpenGL ES/ANGLE SwiftShader run `636ee4a82a20` (PID `28792`, opt-in APK SHA-256 `5ccbbf60c918a33c985b4a6ca7f68682c453d60bf5bdf590e72ec6f3d112e3cf`, embedded library SHA-256 `a99e17063d472ca1b423719e71f9f99809a3be32b6995a65f78434e7d3b11e7f`)에서 PixelCopy 9/9와 전체 화면 9/9, 바깥 anchor 4/4가 exact였다. [화면·원본 로그·geometry mapping](../spec/internal/evidence/c18-1-image-paint-android-api37-2026-10-16/README.md). 실기기에는 명령을 보내지 않았다.
- iPhone 17 Pro/iOS 26.2 Simulator (PID `66309`, app executable SHA-256 `a40b9d549fd9ec31c388311dc69e39b6e3b8fba5b22856c0092731b778377479`)에서 Metal drawable 9/9, 전체 화면 screenshot 9/9, 바깥 anchor 4/4가 exact였다. [화면·원본 로그·geometry mapping](../spec/internal/evidence/c18-1-image-paint-ios-simulator-2026-10-16/README.md). Metal readback은 compositor 이전이며 screenshot 결과와 분리했다.
- 기본/opt-in `spinon-ffi` `.a`·`.rlib`, Android APK 및 iOS app executable을 각각 비교했다. 기본 artifact에는 C18.1 전용 symbol·route·label·PNG bytes가 없고 opt-in positive control에는 포함됐다. [hash와 search 결과](../spec/internal/evidence/c18-1-image-paint-artifact-boundary-2026-10-16/README.md).
- Rust 검증: `spinon-render` 79/79, `spinon-render-wgpu` 22/22 unit + 3/3 integration, C18.1 opt-in `spinon-ffi` 44 pass + 2 ignored. 기본 workspace와 opt-in FFI Clippy (`--all-targets -D warnings`), `cargo fmt --all -- --check`, fixture boundary 및 compositor comparator 16/16도 통과했다. attachment 단계의 image painted-border 거부를 추가 확인했다. 구현 review에서 geometry 전용 코드가 500줄을 넘는 점을 발견해 image quad 구현을 `geometry/image.rs`로 분리했고, 분리 뒤 `spinon-render` 및 `spinon-render-wgpu` 테스트를 다시 통과시켰다.
- negative controls는 snapshot source/NodeId/bytes/frame/image border, fixture img ID/src/cardinality, clip UV·완전 clip, comparator 표본/hash/geometry/anchor 경계를 다룬다. malformed·oversized PNG 요청, 기기 texture limit 및 WGPU validation/OOM 주입, padding·transform 적용, image 준비 실패 뒤 같은 renderer 복구는 이 고정 fixture에서 실행하지 않았다. 이 항목은 완료 근거에서 제외한다.
- 기본 Android/iOS builds와 C18.1 opt-in Android/iOS builds가 모두 성공했다. C18.1 앱 route는 opt-in adapter·selector·native FFI로 격리했다. simulator screenshot sample은 fixture 안의 각색한 색상뿐 아니라 fixture 밖의 서로 다른 anchor 네 점으로 screen origin·scale도 검사했다.
- Android 빌더의 `_Unwind_Resume` 검사 false failure는 `pipefail` 아래 `llvm-nm | grep -q`의 조기 종료가 `llvm-nm` SIGPIPE를 만들던 문제다. `grep`이 입력을 끝까지 읽도록 바꾼 뒤 기본·opt-in Android build를 통과시켰다. iOS Simulator wrapper도 build phase의 `SPINON_V8_DIR` 확인에 필요한 값을 양쪽 `xcodebuild` 호출에 전달해 고정 V8 checkout의 직접 빌드 실패를 해결했다.
- 계획 20개·구현 20개·최종 PR diff 30개 관점 검토를 각각 별도 문서로 남겼다. 구현 review 후 geometry를 전용 모듈로 분리하고 최종 simulator run으로 캡처를 다시 만들었다. Rust·Clippy·fixture·pixel 비교 원본은 [validation evidence](../spec/internal/evidence/c18-1-image-paint-validation-2026-10-16/README.md), 최종 변경 검토는 [PR diff review](../spec/internal/evidence/c18-1-image-paint-pr-diff-review-30-2026-10-16.md)에 기록했다. 상태 대장에서는 C18.1 slice만 완료하고 C18, C14, U05 및 공개 image 지원은 미완료로 유지한다.
