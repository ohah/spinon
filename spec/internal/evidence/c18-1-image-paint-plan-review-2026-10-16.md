# C18.1 이미지 페인트 계획 적대 검토 — 2026-10-16

이 문서는 구현 전 계획을 코드·고정 의존성·Chrome 사전 비교 근거에 대조한 20개 실패 관점 기록이다. 각 관점은 서로 다른 입력 경계 또는 화면 제출 경로를 다룬다. 계획 변경 뒤 같은 관점을 다시 확인했다. 구현 결과의 검토 근거로 재사용하지 않는다.

## 검토 기준

- 대상 계획: [C18.1 image paint/overflow 계획](../../../plan/c18-1-static-image-paint-overflow.md)
- 코드 기준: `ImageResourceSnapshot`은 HostDocument generation/document/render-tree source와 intrinsic-size revision을 소유한다. `RuntimeRenderSnapshot::with_document_snapshot_revision`은 기존 snapshot의 boxes·scroll state를 복사하면서 document revision을 바꾼다. WGPU renderer는 현재 색상 vertex buffer 한 개와 `RuntimePaint`만 그리며 image resource/pipeline은 없다.
- 고정 의존성 기준: `Cargo.lock`은 `wgpu 30.0.1`이다. 로컬 고정 소스의 `Queue::write_texture` 문서와 `wgpu-types 30.0.1`은 256-byte 행 정렬 요구가 `copy_buffer_to_texture`에는 있지만 `Queue::write_texture`에는 없다고 명시한다.
- 사전 oracle: Chrome 155 DPR 1의 fixture·9개 RGBA 표본·PNG/fixture/screenshot hash를 [사전 비교 evidence](./c18-1-image-paint-precomparison/README.md)에 고정했다.

## 적대 관점 20개

1. **문서 source revision이 서로 다름** — `ImageResourceSnapshot.source()`와 `RuntimeRenderKey`의 generation/document/render-tree가 하나라도 다르면 image가 이전 layout에 붙을 수 있다. 계획이 세 revision 모두 일치하고 intrinsic-size revision도 일치해야 하도록 명시했다. 통과 기준: 각 revision 단독 변조는 draw input 생성을 거부한다.
2. **요청 후 `src` 변경** — `ImageResourceSnapshot::new`은 요청을 만든 document의 현재 `src`를 확인하지만, renderer에 전달될 때의 document는 더 최신일 수 있다. 계획은 현재 src와 요청 src 재검증, 변경 시 scene 폐기를 요구한다. 통과 기준: src만 바뀌어도 이전 bytes가 제출되지 않는다.
3. **부분·중복 resource snapshot** — snapshot 생성자는 중복 NodeId는 거부하지만 임의의 일부 image만 가진 snapshot도 허용한다. 계획은 fixture의 예상 두 NodeId 집합과 resource key 집합이 정확히 같도록 요구했다. 통과 기준: 누락·중복·추가 NodeId가 모두 실패한다.
4. **디코더 한도와 GPU 한도 차이** — PNG decoder의 허용 최대치가 모든 adapter의 `max_texture_dimension_2d`를 보장하지 않는다. 계획에 device limit, checked byte length, WGPU allocation/validation 실패를 추가했다. 통과 기준: 최대 texture dimension 초과와 불일치 byte length는 frame submit 전에 거부된다.
5. **알파가 있는 pixel의 blend 누락** — `DecodedImage`는 straight-alpha RGBA8을 보존하지만 기존 pipeline은 blend를 설정하지 않는다. 계획은 이 단계에서 모든 alpha가 255임을 확인하고 한 pixel이라도 투명하면 거부한다. 통과 기준: alpha 변경 negative control이 surface를 갱신하지 않는다.
6. **PNG profile에 따른 색 변환** — decoder는 ICC/gAMA/cHRM 등의 color metadata를 거부한다. 계획은 해당 고정 PNG의 bytes를 sRGB sample로 보존하고 texture 및 surface format별 변환을 명시했다. 통과 기준: sRGB/non-sRGB surface 분기가 각각 Chrome RGBA와 일치한다.
7. **texture row layout 오해** — 24px RGBA row는 96 bytes라 256-byte 경계를 만족하지 않지만 `Queue::write_texture`는 그 alignment를 요구하지 않는다. 계획은 고정 WGPU 30.0.1 `write_texture`를 사용하며 tightly-packed `width × 4`를 공급한다고 명시했다. 통과 기준: 행마다 padding하지 않아도 모든 행의 색이 맞고 크기 계산 overflow가 거부된다.
8. **PNG의 세로 방향 뒤집힘** — texture 좌표 원점과 UI의 위쪽 원점이 혼동되면 위쪽 행이 아래에 그려진다. 계획에 첫 decoded row→CSS frame top 및 `[0,1]` UV 범위를 넣었다. 통과 기준: 비대칭 위·아래 row 표본이 oracle과 일치한다.
9. **잘못된 filter 또는 edge sampling** — linear filtering은 물리 확대에서 인접 texel을 혼합하고 wrap은 가장자리 색을 반복한다. 계획은 nearest/clamp를 지정하고 texel 내부 표본을 고정했다. 통과 기준: 확대된 동일 texel 영역에서 sample RGBA가 일정하다.
10. **이미지가 border 위에 그려지지 않음** — 기존 `build_vertices`는 node마다 배경과 네 border를 하나의 평면 vertex stream으로 만든다. 이미지를 마지막에 한 번에 그리면 parent border와 stacking order가 깨진다. 계획에 각 box의 background→image→border와 flattened document paint order를 명시했다. 통과 기준: 부모 border overpaint 표본이 Chrome과 같다.
11. **clip 적용 때 texture가 재축소됨** — `CssClip`으로 image destination만 자르고 원래 UV를 두면 남은 image 영역이 늘어난다. 계획은 clip에 비례한 destination/UV 동시 절단을 요구했다. 통과 기준: hidden padding 안쪽·border·바깥 표본 3개가 고정 색과 일치한다.
12. **비어 있거나 축별로 교차된 clip** — clip이 frame과 만나지 않거나 한 축만 제한될 때 NaN/역방향 quad가 생길 수 있다. 계획에 반열린 max, 빈 교차, 화면 밖·비유한 frame/UV 및 paint clip/hit-test clip 분리를 추가했다. 통과 기준: empty clip은 아무 pixel도 쓰지 않고 invalid 수치는 error가 난다.
13. **CSS 크기와 physical surface 크기 혼용** — `build_vertices`는 CSS viewport와 surface width/height로 좌표를 변환하며 기기 density는 알지 않는다. 계획은 image content rect를 natural dimensions와 exact 일치시키고 screenshot 좌표에는 측정된 viewport/surface 비율을 사용하도록 고쳤다. 통과 기준: density를 중복 적용한 좌표와 fractional scale 경계가 검출된다.
14. **document revision 재발행 뒤 이전 image 유지** — `with_document_snapshot_revision`은 generation/render-tree가 같으면 boxes를 복사해 document revision만 바꾸므로 src 속성 변경 뒤 stale image가 남을 수 있다. 계획은 image-bearing scene에서 이 재발행을 금지하거나 최신 resource와 다시 결합하지 못하면 거부하도록 명시했다. 통과 기준: document revision만 올린 scene은 old texture를 그리지 않는다.
15. **texture cache가 다른 device로 넘어감** — device가 바뀐 뒤 이전 texture/view/bind group을 쓰면 WGPU validation 오류나 잘못된 image가 난다. 계획은 자원을 `RuntimeRenderer` 소유로 한정하고 renderer/device generation 간 cache 공유를 금지했다. 통과 기준: 새 renderer는 새 texture를 만들며 이전 cache handle을 참조하지 않는다.
16. **resize와 device loss를 같은 세대로 취급** — resize는 새 surface texture를 받지만 device가 유지될 수 있고 device loss는 texture 전부를 폐기해야 한다. 계획은 surface generation과 renderer/device generation을 분리하고 resize 재획득·device 복구 경로를 따로 판정한다. 통과 기준: resize 뒤 재표본은 새 surface identity이며 새 renderer는 빈 cache로 시작한다.
17. **업로드 오류가 난 부분 frame의 stale 성공 표본** — 현재 `draw_if`는 build/submit/present 경계와 테스트 readback을 가진다. 실패 전 surface에 있던 pixels가 이번 성공 결과로 잘못 연결될 수 있다. 계획은 image 준비/geometry 검증 실패 시 submit/present를 하지 않고 이전 표본을 폐기하며 뒤이은 정상 draw를 요구한다. 통과 기준: 실패 run과 다음 성공 run의 frame identity가 다르다.
18. **CSS 표본과 GPU 표본의 좌표 변환 오류** — integer CSS 좌표를 `floor(x × density)`처럼 매핑하면 fractional surface scale에서 다른 pixel을 읽을 수 있다. 계획은 viewport/surface transform, CSS pixel 중심, surface pixel 중앙, 범위 검사를 report에 요구했다. 통과 기준: 좌표 ±1 pixel 변조는 comparator에서 실패한다.
19. **post-compositor screenshot의 잘못된 crop/origin** — Metal readback 또는 SurfaceView PixelCopy만으로 전체 앱 화면에 보인다는 사실은 확인할 수 없다. 계획은 Android AVD 및 iOS Simulator 전체 화면 capture, view 원점·크기, 네 anchor를 추가하고 anchor 실패 시 sample을 무효화했다. 통과 기준: 같은 run의 앱 화면 screenshot에 image/overflow가 보이고 9개 mapping sample이 oracle과 일치한다.
20. **fixture가 제품 artifact에 섞임** — fixture 파일 경로만 분리해도 `include_bytes!`, FFI symbol 또는 app route가 기본 artifact에 들어갈 수 있다. 계획은 dedicated `*-fixture-test` feature/build flag, default artifact/archive 부재, opt-in positive control 포함을 요구했다. 통과 기준: Cargo 및 두 플랫폼 app archive에서 심볼·문자열·PNG payload를 각각 비교한다.

## 계획 반영 후 판정

20개 관점을 모두 계획의 입력 검증, draw ordering, resource lifetime, 좌표 mapping, 캡처 identity 또는 artifact boundary로 반영했다. 남은 사항은 구현에서 API의 실제 WGPU validation/error-scope 동작, simulator screenshot mapping, sRGB 결과를 실행 근거로 확인해야 한다는 점이며, 계획 승인으로 구현 완료를 뜻하지 않는다.
