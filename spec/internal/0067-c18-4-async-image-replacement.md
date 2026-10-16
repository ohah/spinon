# 0067 · C18.4 비동기 PNG 교체·decode 실패 제한 profile

**계약 버전:** `0.1.0` · **구현:** `spinon-image`, `spinon-layout`, `spinon-ffi`, Android/iOS fixture adapters · **앱 작성자용 공개 API 아님**

## 책임과 제한

이 계약은 한 HostDocument의 HTML `img[src]`에서 시작한 PNG decode 결과를 요청의 source stamp에 묶고, 현재 문서·tree·source와 맞는 결과만 새 immutable image resource snapshot에 반영하는 내부 제한 profile이다. 고정 fixture는 늦게 도착한 성공을 거부하고, 현재 source의 pending·decode 실패에서 이전 image bytes를 제출하지 않으며, 다음 정상 source로 paint를 복구한다. 별도 auto-size fixture는 decoded natural size를 C14.1 intrinsic snapshot과 Taffy frame으로 전달한다.

URL/network loader, 일반 URL 해석, fetch·cache 정책, HTML `load`/`error` event, public `HTMLImageElement` API, alt text, UA broken-image icon, SVG·다른 이미지 형식, 전체 CSS Images·HTML 적합성은 제공하지 않는다. C14, U05, C18 상위 항목과 공개 image 지원은 미완료다.

## 요청·완료 identity

`ImageRequest::from_document`은 연결된 HTML namespace의 `img`와 non-empty `src`에서 다음 값을 캡처한다.

- `LayoutSourceRevision::HostDocument { generation, document, render_tree }`
- `HostNodeHandle` 및 요소 `NodeId`
- UTF-16 길이 1,048,576 이하인 `src`

`decode_outcome`은 이 요청과 PNG decoder의 성공 또는 `ImageDecodeError`를 같은 `ImageResourceOutcome` 안에 보존한다. 호출자가 넘기는 encoded bytes는 해당 `src`에서 읽었다는 전제를 가진다. 이 제한 profile의 fixture bytes는 고정 `spinon-fixture:` 입력이며 network fetch를 시작하지 않는다.

## Snapshot 갱신·오류

`ImageResourceSnapshot::new_with_outcomes`는 outcome마다 다음 순서로 현재 HostDocument와 대조한다.

1. generation 불일치는 `StaleRequest`로 반환한다.
2. 요소가 분리·삭제되었거나 현재 `src`를 읽을 수 없으면 `InvalidRequest`; `src` 값이 다르면 `SourceChanged`로 반환한다.
3. generation은 같아도 document/render-tree stamp가 다르면 `StaleRequest`로 반환한다.
4. 현재 node/source의 성공은 decoded RGBA와 natural dimensions를 저장한다. 현재 source의 decode 실패는 구체 `ImageDecodeError`를 저장하고 image map에는 넣지 않는다.
5. 현재 node에 성공·실패 outcome이 중복되면 snapshot 전체를 `DuplicateNode`로 거부한다. 오류 outcome도 image-count 한도 4,096개에 포함한다.

단일 snapshot의 합산 decoded RGBA budget은 64 MiB다. `replace_with_outcomes`는 `IntrinsicSizeRevision::checked_next`로 새 revision을 만들며 overflow 시 `RevisionExhausted`를 반환한다. replacement는 전달된 현재 결과 집합으로 새 snapshot을 만들기 때문에 실패·pending 결과가 이전 image resource를 계속 그리지 않는다. 오래된 completion은 rejection으로 별도 반환하며 현재 image·failure map을 바꾸지 않는다.

`ImageResourceSnapshot::new`는 stale/invalid/source-changed outcome을 오류로 승격하는 편의 경로다. 비동기 대체는 오류 outcome identity와 stale rejection을 함께 관찰할 수 있는 `new_with_outcomes` 경로를 사용한다.

## Pending intrinsic size

`to_pending_layout_intrinsic_size_snapshot`은 현재 연결된 pending node를 새 document stamp와 전달받은 intrinsic-size revision에 연결한다. 같은 document generation의 이전 snapshot에 성공 image가 있으면 그 **natural width/height만** 보존한다. RGBA bytes, request identity, source key, render cache는 복사하지 않는다. 이전 성공 측정이 없거나 generation이 바뀌면 natural size를 `0×0`으로 둔다.

`spinon-layout`은 intrinsic ratio가 없는 정확한 `0×0` pending marker를 허용하고 이를 자연 비율 계산에 쓰지 않는다. 그 외 0 이하·비유한 크기와 비일치하는 natural ratio는 거부한다. 첫 decode 성공은 새 image snapshot과 intrinsic revision을 통해 자연 크기를 다시 계산한다.

## Fixture-only runtime·ABI

Rust fixture와 C ABI는 `c18-4-image-fixture-test` opt-in 아래에 둔다. Android/iOS adapter, fixture JavaScript와 PNG bytes도 같은 opt-in build switch로 연결한다. C18.4 FFI는 host 생성, 고정 fixture 평가와 6개 표면 표본 예약을 제공한다. host 수명·호출 직렬화·caller-owned output buffer 규칙은 기존 C04.10 fixture ABI 계약을 따른다.

고정 runtime route는 5개 image node를 사용한다. A의 지연 성공보다 B가 먼저 완료되고, 늦은 A는 현재 B snapshot에서 거부된다. 이어지는 C pending node와 손상 source는 image draw에서 빠지고, 다른 C 성공과 D recovery source는 각자의 decoded PNG만 그린다. 실제 PNG decoder를 worker thread에서 사용하며 fixture completion 순서는 channel gate로 고정한다. Android PixelCopy와 iOS Metal drawable의 표본 위치·RGBA는 내부 fixture가 검사한다.

현재 scene의 `ComputedStyleSnapshot`은 document, tree, generation, style revision이 render scene key와 같아야 한다. `RuntimeBlockImageV1`에서 각 fixture image의 `object-fit: fill`, `object-position: 50% 50%`, `image-rendering: auto`가 확인될 때만 resource를 scene에 붙인다. fixed frame paint는 image intrinsic size와 분리한다.

## 거부 동작과 확인된 한계

| 조건 | 동작 |
| --- | --- |
| unknown·분리·비-HTML-img 요소, 빈/과도한 `src` | `ImageRequestError`로 요청 생성을 거부한다. |
| stale generation/document/tree, 변경된 source, 삭제 node | outcome을 분류해 snapshot에서 제외한다. |
| 현재 node의 중복 outcome | `DuplicateNode` 오류로 snapshot 전체를 거부한다. |
| count/decoded-byte budget 초과, revision overflow | 구체 `ImageSnapshotError`로 replacement를 거부한다. |
| PNG decode 실패 | 요청 identity에 묶인 오류 map entry만 기록하고 image resource는 비운다. 다른 node와 다음 정상 source는 계속 처리한다. |
| pending auto-size와 이전 성공 측정 부재 | 현재 stamp에 natural `0×0`만 기록한다. |
| invalid zero/negative/non-finite intrinsic metadata | layout snapshot 또는 Taffy 계산에서 거부한다. |
| fixture computed style·scene revision mismatch | paint scene 결합을 실패 처리하고 표본을 성공으로 갱신하지 않는다. |

Android 결과는 API 37 ARM64 AVD의 PixelCopy 경로다. iOS 결과는 iPhone 17 Pro / iOS 26.2 Simulator의 `Bgra8Unorm` Metal drawable, compositor 제출 전 readback이다. 실기기, hardware GPU, iOS compositor 이후 화면, 다양한 color space/density 및 외부 URL loader는 검증 범위가 아니다.
