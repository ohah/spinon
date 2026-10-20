# C11.4 direct image multi-span 구현·runtime 리뷰 20개

**대상:** 제한 fixture adapter, FFI scene source, Android/iOS 검증, capture runner

**검토 결과:** 20개 관점을 확인했다. 범위 밖의 border paint와 PNG texel paint 제한은 evidence README에 명시했다. 공식 C11.4 완료 상태는 바꾸지 않는다.

| # | 실패 관점 | 확인 내용과 결과 |
| ---: | --- | --- |
| 1 | 허용되지 않은 scene ID가 fixture에 들어옴 | runtime/FFI/native runner 모두 네 개의 고정 ID만 image multi-span으로 취급한다. unknown ID 테스트도 유지된다. |
| 2 | 새 adapter가 기존 image fixture를 가로채거나 제품 바이너리에 노출됨 | root ID에서 네 allowlist와 정확히 맞을 때만 `Some(input)`을 반환하며, generic 단일 image 경로 앞에서 제한 호출한다. 기본 `spinon-ffi` rlib/staticlib 빌드에서 네 scene ID 문자열은 나오지 않았다. |
| 3 | 다른 namespace/tag 또는 root 위치를 fixture로 받아들임 | HTML `div`, 문서 root의 직접 자식 조건을 확인하고 아니면 fail-closed한다. |
| 4 | 누락/추가 자식으로 기대와 다른 입력이 만들어짐 | root direct child 수를 scene별 2/3개와 비교한다. |
| 5 | 중첩 또는 non-leaf image가 직접 image로 오인됨 | 각 child가 HTML `img`, fixture ID, root의 direct child이며 자식이 없는지 확인한다. |
| 6 | DOM 순서와 노드 배치 테이블이 어긋남 | 순서별 고정 ID/placement 테이블을 child iteration과 zip으로 검증하고 forward/reverse scene을 분리했다. |
| 7 | 임의 URL 또는 decode 실패가 intrinsic 입력에 사용됨 | source URL을 fixture 상수로 제한하고 `ImageRequest` decode 및 `ImageResourceSnapshot` 생성을 확인한다. 디코드된 natural size는 24×16이다. |
| 8 | 이미지마다 revision이 다르거나 stale 입력 사용 | snapshot 전체에 `INITIAL.checked_next()` 한 revision을 사용하며 세 플랫폼 로그에서 intrinsic revision 1을 확인한다. |
| 9 | root border-box/content-box 크기 산정 오류 | root는 40×60 border-box, 1px padding으로 38×58 content box를 갖는다. Chrome·Android·iOS item origin은 (1,1)로 일치한다. |
| 10 | 예상하지 않은 track 함수/gutter/flow가 허용됨 | 4개 `min-content` column, 3개 20px row, row auto-flow, zero gap만 허용하고 auto-repeat/line names/areas는 거부한다. |
| 11 | item intrinsic width가 cell stretch에 묻힘 | item `width:auto`, 높이 16/32/48px와 replaced `justify-self:auto`/`align-self:auto`를 고정했다. Chrome 및 앱에서 24/48/72px width를 확인했다. |
| 12 | 배치 선/span 파싱이 다른 조합을 허용함 | 각 item의 row start, column start와 numeric span을 고정 비교하며 end line은 `auto`만 허용한다. |
| 13 | 앱이 거부하는 alignment 키워드 때문에 장면 생성이 실패함 | `start` 선언 시 실제 `unsupported_computed_value`를 재현했다. 두 item 축의 `auto`를 사용하고 Chrome·모든 앱 runtime frame이 예상과 일치한다. |
| 14 | Minimum과 MinContent 기여를 혼동함 | trace에서 각 span group의 `Minimum=0`, `MinContent=24/48/72`를 입력 순서 및 반복 횟수별로 대조한다. |
| 15 | span-2와 span-3 기여 그룹 순서가 잘못됨 | trace stage span 배열 `[2]` 및 `[2,3]`을 확인하고 20px row pass도 별도로 검사한다. |
| 16 | DOM 순서 역전에서 track 또는 frame이 달라짐 | forward/reversed 두 쌍이 동일한 track 배열과 의미상 image frame을 산출했다. 세 플랫폼의 결과 JSON이 동일하다. |
| 17 | 잘못된 track만 통과하고 overflow/frame은 틀림 | runner가 root 및 각 node의 order/ID/x/y/width/height를 `1/64px` 허용 오차로 비교하고 final track arrays도 별도로 확인한다. |
| 18 | root 밖 또는 아래 overflow가 layout 로그만으로 잘못 판정됨 | Android PixelCopy와 iOS Metal surface의 root interior/exterior, item overflow, 아래 overflow, item 뒤 빈 표본이 모두 정확히 일치했다. |
| 19 | 빌드 산출물과 설치 산출물이 다르거나 target 환경을 잘못 기록함 | 실기기·AVD installed APK SHA-256이 local APK와 모두 일치한다. device/API/GPU 및 iOS simulator/runtime/app hash도 기록했다. |
| 20 | 제한 fixture 결과가 일반 image/Grid/WPT 지원 완료로 과장됨 | evidence는 intrinsic layout, CSS box background overflow에 한정한다. Android 녹색 border paint 미지원과 PNG texel paint 부재를 기록하고 공식 C11.4 checkbox/count를 유지한다. |

## 검증 증거

- Android 실기기·AVD와 iPhone 17 Pro/iOS 26.2 네 장면은 [runtime evidence README](./README.md)와 각 `result.json`에 기록했다.
- `mise exec -- cargo fmt --all -- --check` 통과.
- Runtime focused test 1/1 통과: `c11_4_image::multispan::tests::exposes_four_order_independent_direct_image_multispan_scenes`.
- FFI focused test 1/1 통과: `runtime_gpu::fixtures::c11_4::tests::exposes_only_bounded_intrinsic_track_sizing_scenes`.
- 기본 `mise exec -- cargo build --locked -p spinon-ffi --lib` 산출 rlib/staticlib에 `image-span2-forward`·`image-span3-before-span2` scene ID가 없음을 확인했다.
- Android fixture APK build 성공. iOS Simulator fixture build는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`을 켠 상태에서 성공.
- Capture runner JavaScript syntax 검증 통과.
