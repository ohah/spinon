# C11.4 visibility-hidden border overflow · runtime/구현 실패 관점 20개

이 검토는 계획 검토 표를 재사용하지 않고 새 runtime adapter, zero-height text 경로, FFI route, 양 플랫폼 실행을 대상으로 한다. 각 항목은 별도 입력 변형이나 runtime failure 경계를 확인한다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | 미등록 scene 접미사가 새 고정 route에 우연히 매칭될 수 있다. | FFI source generator와 runtime root ID map이 exact ID를 사용한다. `routes_only_the_eight_exact_visibility_mixed_track_scenes`는 unknown/suffixed route를 거부한다. 통과. |
| 2 | `row-*` ID가 Columns track topology에 연결될 수 있다. | Root ID map의 axis와 fixed/flexible track 위치를 대조했다. 네 scene runtime test가 Rows/Columns 각각을 계산한다. 통과. |
| 3 | 새 border-overflow ID가 예전 300px root/120px fixed cross track 스타일을 받아들일 수 있다. | 이 경로를 발견했다. root 크기를 style에서 추정하지 않고 exact scene ID에서 border 계약을 선택하도록 바꾸고, legacy geometry negative control을 추가했다. 최종 test와 matrix 통과. |
| 4 | 기존 visibility ID가 새 100px border-box geometry를 받아들일 수 있다. | 기존 scene은 border 계약이 false로 고정된다. 예전 ID에 border/116px track 스타일을 넣는 반대 방향 negative control이 거부됐다. 통과. |
| 5 | border-box 크기를 content-box track constraint로 오인할 수 있다. | Root frame은 Columns `100×120`, Rows `120×100`이고, sizing content는 `96×116` / `116×96`이다. runtime frame 및 manifest trace가 일치한다. 통과. |
| 6 | 한쪽 border만 2px이 아니거나 edge가 layout 크기에서 빠질 수 있다. | Computed layout border 네 변이 모두 2px인지 검사하고 frame이 `(2,2)`에서 시작하는지 확인했다. 폭 mutation이 fail-closed다. 통과. |
| 7 | border paint가 빠지거나 다른 색으로 계산돼 표본이 엉뚱한 표면을 읽을 수 있다. | Typed border paint `#f0c674`를 요구하고 4 scene/platform에서 border RGBA `[240,198,116,255]`를 확인했다. paint 누락 negative control이 거부됐다. 통과. |
| 8 | `overflow-x:hidden`이 새 장면에서 통과할 수 있다. | Runtime root 계약에서 `layout_overflow_x=Visible`을 검사한다. X hidden mutation이 거부됐다. 통과. |
| 9 | `overflow-y:hidden`이 새 장면에서 통과할 수 있다. | Runtime root 계약에서 `layout_overflow_y=Visible`을 검사한다. Y hidden mutation이 거부됐다. 통과. |
| 10 | 다른 style profile, writing mode, alignment 또는 box-sizing의 레이아웃이 같은 expected frame으로 보일 수 있다. | `RuntimeBlockFlexGridTrackSizingV1`, horizontal-tb, border-box, LTR, start/normal 정렬과 zero margin/padding을 exact check한다. 양 플랫폼 18-scene verifier가 root/frame/trace도 확인한다. 통과. |
| 11 | 오래된 document revision의 computed style이 새 scene처럼 라우팅될 수 있다. | Snapshot revision gate가 scene style 확인 전에 실행된다. 기존 revision fail-closed test와 C11.4 focused test가 통과했다. |
| 12 | direct child를 추가/삭제하거나 root 밖 parent를 사용해 고정 scene 검사를 우회할 수 있다. | Root parent 및 direct child 수 4를 확인한다. 기존 topology mutation suite와 새 runtime test가 거부 경로를 통과했다. |
| 13 | A/span/image/C의 순서, namespace, tag 또는 node ID를 바꿔 다른 content가 같은 frame을 차지할 수 있다. | 각 child의 HTML identity, exact ID, parent와 source-order cell을 검증한다. 기존 child order/identity mutation suite 통과. |
| 14 | hidden ID에 visible span을 연결하거나 visible control을 숨겨 positive control을 위조할 수 있다. | Span `visibility`를 scene ID와 대조하고 root/A/image/C는 visible이어야 한다. visibility mutation test 통과. |
| 15 | A/span/C의 글자를 바꿔 min-content 결과를 바꿀 수 있다. | Direct Ahem text를 각각 `XXXXXXXXXX`, 15자 span, `XXXX`로 exact 비교한다. runtime adapter gate 통과. |
| 16 | PNG URL/decode/natural size가 달라도 Grid 결과만 맞아 보일 수 있다. | 고정 image URL, leaf topology, decode 성공과 natural `24×16`을 검사한다. 18-scene intrinsic trace에는 같은 natural size가 기록된다. 통과. |
| 17 | Items가 다른 grid line에 놓이거나 span이 달라져도 expected frame만 강제로 낼 수 있다. | A/span track 1, image track 2, C track 3 및 axis placement를 style adapter에서 검사한다. FFI/runtime route와 frame matrix 통과. |
| 18 | Border가 반영된 fixed cross track가 계속 120px이면 root 안쪽 item 영역을 넘길 수 있다. | 신규 route는 116px을 exact 요구하고 120px mutation을 거부한다. 두 축 trace의 fixed pass와 5 node frames가 일치했다. |
| 19 | Hidden span의 150px min-content가 사라지거나 두 번 합산되고, zero-sized final track이 음수로 내려갈 수 있다. | Span contribution은 150px, target tracks `[150,40,0]`, remaining space `-114px`다. Row 축 C는 height 0 frame을 유지한다. zero-height text guard는 TrackSizing Grid item 조건에서만 적용하고 layout crate 308/308 및 runtime test를 통과했다. |
| 20 | Hidden paint가 표면에 새거나 visible control이 빠져도 frame만 보고 통과시킬 수 있다. | Hidden draw 3 / visible draw 4, presentation completion, border 표본, root 밖 표본을 각각 확인했다. Android PixelCopy와 iOS 제출 전 Metal readback에서 hidden 파랑과 visible 보라를 exact match했다. |

최종 근거: [Android/iOS 18-scene matrix](./matrix-verification-18-scenes.json), [runtime focused test 로그가 요약된 실행 README](./README.md). 이 검토는 실기기, 전체 surface parity 또는 iOS compositor 이후 결과를 입증하지 않는다.
