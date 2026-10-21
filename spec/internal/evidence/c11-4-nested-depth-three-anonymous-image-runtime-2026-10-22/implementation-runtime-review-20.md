# C11.4 depth-three anonymous text + image 구현/runtime 적대 검토 · 20개 관점

**검토 대상:** `3346d9b3`의 fixture 전용 depth-three Grid projection, runtime resource preparation, Android/iOS scene adapter와 최종 14-scene Simulator 실행. 계획 PR의 20개 및 계획 PR diff의 30개 관점은 재사용하지 않았다.

| # | 독립 실패 관점 | 확인·근거 | 결과 |
| ---: | --- | --- | --- |
| 1 | fixture-only nested-grid projection이 기본 style/runtime API에서 보이는가 | projection 함수와 re-export가 `c11-4-grid-track-sizing-fixture-test`로 cfg-gated다. 기본 FFI archive에서 신규 scene ID가 빠진다. | PASS |
| 2 | 신규 scene이 기존 direct-image/text adapter로 중복 처리되는가 | runtime adapter 준비 순서와 상호 배제 조건이 새 mixed input을 먼저 선택하며, fallback input은 모두 `None`으로 둔다. | PASS |
| 3 | 유사 prefix의 미등록 depth-three scene을 승인하는가 | 준비 함수는 고정 `SCENES`만 승인하고 같은 prefix의 알 수 없는 root ID는 Unsupported로 거부한다. | PASS |
| 4 | 문서 root 밖 또는 잘못 표시된 root를 허용하는가 | HTML `div`, document-root ownership, 빈 `data-node`, runtime profile을 검증한다. | PASS |
| 5 | root→middle→branch→inner 중간 노드가 다른 parent나 tag를 갖는가 | 네 고정 ID, `div` tag, 각 parent-child ownership 및 outer Grid의 child 수를 순서대로 확인한다. | PASS |
| 6 | 내부 Grid에 세 번째 child나 누락 child가 들어오는가 | inner direct child가 정확히 둘인지 확인하고 예상한 source order에 맞춰 text/image를 구분한다. | PASS |
| 7 | anonymous item이 다른 text, 빈 text, element wrapper로 바뀌는가 | 직접 `HostNodeKind::Text` 값이 정확히 `XX XXXXXXX`인지와 inner parent를 검사한다. | PASS |
| 8 | image가 다른 element, URL, descendant를 사용하는가 | direct `img`, 고정 ID/URL, 빈 `data-node`, 빈 자식 목록을 검증한다. | PASS |
| 9 | image fetch/decode 실패나 PNG 치수 변경이 부분 layout을 남기는가 | pinned PNG bytes를 decode하고 natural `24×16`을 확인한다. 오류는 `RuntimeLayoutFailure`로 전파한다. | PASS |
| 10 | decoded image intrinsic size revision이 layout stamp에서 누락되는가 | `ImageResourceSnapshot` revision을 만들고 mixed projection 결과의 `intrinsic_size_revision`에 복사한다. | PASS |
| 11 | 시스템 폰트 환경이 Ahem intrinsic 측정에 섞이는가 | 시스템 폰트 없이 `Ahem` fixture bytes를 등록하고 고정 paragraph style을 사용한다. | PASS |
| 12 | 변형된 inherited text style이 예상 intrinsic width/line count를 바꾸는가 | font family/size/weight/style, line-height, whitespace/wrap, break, direction, alignment, transform 및 spacing을 검증한다. | PASS |
| 13 | 외곽/inner Grid에서 min/max-content 축이 서로 다르게 전달되는가 | 모든 chain Grid의 width/height와 inner 2-column/1-row sizing keyword를 scene constraint와 대조한다. | PASS |
| 14 | margin, padding, border 또는 gap이 `94×20`/`124×16`을 우연히 맞추는가 | 모든 Grid edge·gap 및 image edge가 0인지 확인하고 frame/track 기대값도 별도로 검사한다. | PASS |
| 15 | named line, template area, auto-repeat 또는 implicit track이 input에 섞이는가 | 고정 track 수·빈 line names·area 부재·auto-repeat 부재·row flow 및 auto track list를 확인한다. | PASS |
| 16 | placement가 span/multiple cell이거나 source order와 어긋나는가 | child placement의 row/column 시작·끝이 모두 Auto인지 확인하고 scene별 직접 item order를 검사한다. | PASS |
| 17 | image authored ratio/크기 또는 self-alignment가 natural frame을 바꾸는가 | width/height auto, aspect ratio 없음, 정적 position, zero edges, 제한 `align-self`/`justify-self`를 검증한다. 이 누락 guard는 review에서 발견해 최종 재실행 전 추가했다. | PASS |
| 18 | 두 contribution 중 하나가 inner track 또는 부모 chain에서 누락되는가 | capture verifier는 inner의 각 text/image node에 대해 양 축 intrinsic trace와 각 바깥 Grid의 child contribution을 대조한다. | PASS |
| 19 | order 반전이 전체 intrinsic size까지 바꾸거나 item frame을 잘못 둔다 | 네 scene의 inner columns와 item x를 각각 확인한다. 두 min scene은 `94×20`, 두 max scene은 `124×16`이다. | PASS |
| 20 | scene matrix test가 네 조합 중 일부를 빠뜨려도 통과하는가 | 기존 개수 기반 검사의 약점을 발견해 `(scene ID, min/max, image order)` 정확한 네 tuple 비교로 바꿨고 1/1 통과했다. | PASS |

**최종 실행:** [Android API 37 ARM64 AVD](./README.md#최종-android-실행)와 [iPhone 17 Pro/iOS 26.2 Simulator](./README.md#최종-ios-실행)에서 신규 네 scene+기존 회귀 열 scene이 각각 14/14다. 실기기, WPT, glyph/image pixels 또는 iOS compositor-after 결과로 확대 해석하지 않는다.

**발견·수정:** matrix 검사가 constraint/order의 모든 조합을 확정하지 못했다. exact tuple 검사를 추가했다. image style guard에 authored aspect ratio 및 self-alignment 확인이 빠져 있어 fail-closed 검사를 보강했다. 둘 다 영향받는 focused test와 최종 Simulator matrix를 재실행했다.
