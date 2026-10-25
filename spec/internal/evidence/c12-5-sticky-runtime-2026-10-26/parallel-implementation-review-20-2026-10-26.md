# C12.5 sticky 구현·PixelReadback 독립 검토 20관점 · 2026-10-26

## 검토 범위와 fingerprint

기준 커밋은 `d94f645843fca7ca431a3c51fc3519cced9109c0`이며, 현재 미커밋 source/config/fixture 파일 34개를 검토했다. fingerprint는 build output, `DerivedData/`, `spec/internal/evidence/`를 제외하고 변경 source/config/fixture 파일을 경로 정렬한 뒤 각 파일의 `경로 + NUL + 길이(8-byte big-endian) + 바이트`를 이어 SHA-256을 계산했다.

```text
source-only file count: 34
source-only SHA-256: 540f6fc2ebec69077ac084f1cc6144503dc87b0ed9d36d66db9ee6e12c7e3360
```

검토 중 제품 소스와 source fixture는 바뀌지 않았다. Chrome 확인용 parent-edge HTML/JSON 및 evidence README는 검토 기록에만 추가됐고 fingerprint에 들어가지 않는다. 리뷰를 시작한 코드 snapshot 시점에는 Android PixelCopy가 pending이었다. 종료 시점에는 별도 Android API 37 AVD evidence가 도착해 두 표본이 2/2 exact로 일치했다([Android PixelCopy 기록](./android-pixelcopy-parallel-20261026/README.md)). 이 후속 결과는 source-only fingerprint를 바꾸지 않았다.

계획 리뷰의 관점 목록을 재사용하지 않았다. 다음 20개는 실제 변경 구현·readback 경로를 대상으로 한 별도 실패 경로 점검이다. 빌드·테스트·시뮬레이터는 실행하지 않았다.

## 관점별 결과

| # | 독립 실패 관점 | 코드 위치 | 관찰 및 구체적 결과 |
| ---: | --- | --- | --- |
| 1 | 부모에 padding/border가 있을 때 sticky가 containing block의 content edge를 넘는가 | `crates/spinon-style-to-render/src/stacking/sticky.rs:213-244`; `crates/spinon-render/src/runtime/scroll.rs:396-400` | **확정 병합 차단 결함.** 구현은 부모 `LayoutFrame` 전체를 constraint로 저장하고 border-box bottom에서 sticky 높이를 뺀다. Chrome 156 Agent Browser repro에서 `#section`의 `padding-bottom:12px; border-bottom:2px`, inner offset 212일 때 section border box는 `[-94,82]`, content bottom은 `68`, sticky는 `[40,68]`이며 `(200,78)`은 section이다. 현재 공식은 top `54`를 예측해 sticky가 14px 아래까지 보인다. 재현 관측은 [JSON](./parent-content-edge-repro.json), 입력은 [HTML](./parent-content-edge-repro.html)이다. C12.5가 relative Block 부모를 허용하면서 그 부모의 edge를 제한하므로 계약 위반이며 수정 전 병합하면 안 된다. |
| 2 | 큰 유한 offset 합에서 f32 정밀도가 CSS 위치를 잃는가 | `crates/spinon-render/src/runtime/scroll.rs:429-449` | offset을 f32로 순차 합산한다. 예를 들어 `16,777,216px + 1px`는 f32에서 1px 증가가 표현되지 않을 수 있어 깊은 문서에서 sticky threshold가 실제 CSS 위치와 달라질 수 있다. 현재 고정 fixture 범위에서는 재현되지 않았고 별도 상한 계약도 확인하지 못해 비차단 위험으로 남긴다. |
| 3 | 유한 offset들 합계가 overflow할 때 잘못된 frame을 게시하는가 | `crates/spinon-render/src/runtime/scroll.rs:443-449` | 누적 결과가 비유한 값이면 `InvalidScrollState`로 거부한다. guard가 빠지면 무한대가 clamp 계산에 들어가 snapshot이 깨진다. 현재 구현에서는 fail-closed다. |
| 4 | 같은 ancestor가 offset 목록에 중복되어 두 번 적용되는가 | `crates/spinon-render/src/runtime/scroll.rs:435-442` | 중복 NodeId에서 바로 오류를 반환한다. 중복 경로가 허용되면 sticky 위치와 scrollport가 어긋날 수 있으나 현 경로에서는 중복 적용을 막는다. |
| 5 | ancestry가 참조하는 owner offset이 snapshot에서 누락됐을 때 0으로 묵인하는가 | `crates/spinon-render/src/runtime/scroll.rs:440-442` | 누락 owner를 `InvalidScrollState`로 거부한다. 0으로 대체하면 sticky 또는 clip이 이전 위치에 남을 수 있는데 현 구현은 부분 snapshot을 게시하지 않는다. |
| 6 | box와 metadata 수가 다를 때 `zip`이 일부 box를 조용히 버리는가 | `crates/spinon-render/src/runtime/snapshot.rs:117-125`; `crates/spinon-render/src/runtime/scroll.rs:357-358` | snapshot 생성 전에 개수 및 base frame 대응을 검사한다. 이 검증이 없으면 `zip`이 짧은 쪽까지만 적용해 일부 box의 frame을 빠뜨릴 수 있으나 생성 경계에서 거부된다. |
| 7 | metadata frame이 다른 box의 frame과 연결되는가 | `crates/spinon-render/src/runtime/snapshot.rs:120-125` | 같은 index의 base frame 일치 여부를 검사한다. 불일치면 `InvalidScrollState`를 반환해 sticky constraint가 다른 노드의 frame에 적용되는 경우를 막는다. |
| 8 | 중복 scroll owner가 offset map에서 덮어써지는가 | `crates/spinon-render/src/runtime/snapshot.rs:127-141` | owner 중복을 offset map 생성 전에 거부한다. 중복을 허용하면 BTreeMap의 뒤 값만 남아 sticky 위치가 순서 의존적으로 바뀔 수 있다. |
| 9 | FFI가 고정 2개가 아닌 표본 수나 null 좌표를 읽는가 | `crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c12_5_sticky.rs:74-100` | count 2와 non-null 좌표를 먼저 확인한다. 잘못된 길이로부터 4개 u32를 읽는 입력을 차단한다. 포인터가 실제로 4개 값을 가리키는지는 명시된 C caller Safety 계약에 남는다. |
| 10 | 동일 좌표를 두 표본으로 요청해 중복 픽셀을 독립 검증처럼 세는가 | `crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c12_5_sticky.rs:101-116` | 완전히 중복된 두 좌표는 거부된다. 같은 표본 두 개가 2/2 통과로 오인되는 경로를 차단한다. |
| 11 | drawable 밖 좌표를 readback에 넣어 잘못된 표본/메모리 접근을 유발하는가 | `crates/spinon-render-wgpu/src/renderer.rs:394-410` | renderer가 실제 surface 크기로 `validate_points`를 호출한 뒤 요청을 저장한다. iOS caller의 좌표 bounds guard가 회귀해도 renderer 경계에서 거부된다. |
| 12 | 이전 표본 요청이 미완료인 동안 새 요청이 이전 결과를 덮는가 | `crates/spinon-render-wgpu/src/renderer.rs:405-412` | pending readback이 있으면 새 요청을 거부하고, 수락할 때 이전 결과를 비운다. 서로 다른 실행의 표본이 섞이는 경로는 차단된다. |
| 13 | readback 준비 중 presentation이 바뀌었는데 이전 장면의 픽셀을 성공 처리하는가 | `crates/spinon-render-wgpu/src/renderer.rs:616-621` | encoder 제출 전에 `is_current()`가 false면 pending/completed sample을 버리고 `Ok(false)`로 끝낸다. stale 장면 픽셀을 보고하지 않는다. |
| 14 | GPU submit 뒤 present 전에 presentation이 바뀌었는데 표본을 보존하는가 | `crates/spinon-render-wgpu/src/renderer.rs:644-652` | submit 뒤 두 번째 current check에서 표면 표본을 폐기한다. 이전 scene의 GPU readback을 다음 화면의 결과처럼 반환하는 경로를 차단한다. |
| 15 | GPU mapping 실패 뒤 실패한 표본이 다음 draw에 남는가 | `crates/spinon-render-wgpu/src/renderer.rs:655-668` | map 실패는 pending 및 completed 표본을 폐기하고 오류를 반환한다. 성공 경로에서만 pending을 비우고 sample을 publish한다. |
| 16 | iOS에서 실제 drawable이 고정 fixture와 다르거나 x/y scale이 다를 때 잘못된 좌표를 읽는가 | `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:13223-13305` | viewport `360×260`, 양의 drawable, 정수 치수와 거의 동일한 두 scale을 확인한 뒤 CSS pixel center를 각 축 scale로 변환하고 bounds 검사한다. 다만 `abs(scaleX-scaleY) < 1e-6` 조건은 유효한 정수 drawable이 비정수 배율로 반올림된 환경에서 테스트를 거부할 수 있다. 현재 iPhone 17 Pro Simulator 3× drawable은 정확히 일치해 비차단 호환성 제한이다. |
| 17 | iOS readback이 다른 surface generation/presentation의 결과를 통과시키는가 | `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:13295-13317` | 요청 generation과 현재 generation·presentation sequence를 비교하고, drawable/format, box 수, viewport, scroll revision과 offset도 확인한다. stale 결과는 오류로 종료한다. |
| 18 | iOS JSON 수치 검증이 소수 값을 정수로 잘라 통과시키는가 | `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:13319-13343` | `NSNumber.uint32Value` 변환 후 기대값과 비교하므로 `x=601.9` 같은 malformed 수치가 601로 잘려 통과할 가능성이 있다. 현 native Rust report는 u32와 정수 RGBA만 직렬화하고 고정 경로에서 2/2 exact였으므로 현재 blocker는 아니지만 parser는 정수형을 명시 확인하지 않는다. |
| 19 | PixelCopy 도중 surface/presentation이 stale가 되면 Android evidence runner가 끝나지 않는가 | `platforms/android/app/src/c12_5_sticky_fixture_test/java/dev/spinon/bootstrap/C125StickyFixtureCaptureAdapter.java:72-76`; `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:12213-12220` | stale callback은 artifact를 지우고 오류 로그만 남긴 뒤 status callback 없이 반환한다. capture는 앞에서 one-shot flag를 세우므로 자동 재요청도 없다. 결과적으로 표면 변경 race 시 runner UI에 terminal status가 안 올 수 있다. PixelCopy 제품 렌더링에는 영향이 없고 evidence 자동화 신뢰성 결함으로 분류한다. |
| 20 | Android PixelCopy callback이 요청 당시 draw identity와 다른 runtime scroll snapshot을 읽는가 | `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:12224-12230`; `platforms/android/app/src/c12_5_sticky_fixture_test/java/dev/spinon/bootstrap/C125StickyFixtureCaptureAdapter.java:24-37` | `sourceIsCurrent`는 surface generation·크기·renderer·host presentation sequence를 확인하지만 scroll-state revision은 직접 확인하지 않는다. `drawReport`의 revision/offset은 요청 때 고정값인지 검사할 뿐 callback 시점의 현재 revision과 대조하지 않는다. capture 중 scroll snapshot만 교체되는 경로가 있다면 새 surface를 이전 draw identity로 귀속할 수 있다. C12.5 고정 AVD run은 입력을 종료한 상태로 실행했고 2/2 pass였으므로 이번 evidence에서 재현되진 않은 비차단 race 위험이다. |

## 결론

확정된 제품 동작 결함은 #1 content-edge clamp 하나이며 병합 차단이다. #19는 evidence runner의 stale terminal callback 결함이고 #20은 scroll revision을 PixelCopy callback identity에 묶지 않은 검증 race다. #2, #16, #18은 고정 target 외 환경 또는 malformed/internal report 경계 위험으로 기록했다. 나머지 항목은 실패하면 어떤 결과를 내는지 확인했고, 현재 코드에서 해당 오경로가 차단된다.

Android AVD와 iOS Simulator의 2/2 색상 결과는 해당 고정 장면·surface scope에 대한 증거다. 이 결과는 parent content edge 변형을 검증하지 않았고, #1의 source 기반 예측을 무효화하지 않는다. 구현 수정 후 해당 repro를 실제 Spinon runtime에 넣어 재검증하고, 변경된 source fingerprint로 영향을 받은 관점을 다시 수행해야 한다.
