# C11.4 fit-content span-2 · 구현/runtime 실패 관점 20개

계획 검토 표를 반복하지 않고 fixture adapter, typed Grid input, track trace, native result matcher와 실제 표면 표본을 대상으로 다시 확인했다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | 미등록 또는 suffix scene ID가 exact fixture source로 들어갈 수 있다. | FFI route와 runtime source는 `fit-content-spanning-wpt`만 허용한다. Unknown/suffixed ID test가 거부한다. 통과. |
| 2 | fixture가 HTML root가 아니거나 지원하지 않는 profile에서 실행될 수 있다. | Runtime adapter가 HTML namespace, `div`, root ownership, `RuntimeBlockFlexGridTrackSizingV1`을 검사한다. mutation 통과. |
| 3 | root에 item이 추가되거나 child parent가 바뀌어 같은 node frame을 위조할 수 있다. | Exact one direct item/one nested child, parent, ID, order 및 style node set을 검사한다. extra direct item negative control 거부. |
| 4 | root width/height 차이를 예상 frame으로 숨길 수 있다. | Computed style과 typed input에서 60×300px를 검사하고 width/height mutation을 각각 거부한다. 통과. |
| 5 | 두 implicit row가 explicit row나 다른 auto track으로 바뀔 수 있다. | `rows=[]`, `auto_rows=FitContent(40px)`, one 60px column을 computed style과 `GridContainerInput` 양쪽에서 확인한다. 통과. |
| 6 | row gap이 span 합계에 끼어든다. | Computed row/column gap 0을 exact 검사하고 1px row-gap mutation을 거부한다. 통과. |
| 7 | item span이 1 또는 다른 placement로 바뀐다. | `row_start=Span(2)`, 나머지 placement Auto를 검사한다. 별도 span mutation과 matrix가 거부한다. 통과. |
| 8 | `min-height:0` 대신 automatic minimum이 row base를 바꾼다. | Item minimum style과 `Minimum` contribution을 검사한다. `min-height:1px` mutation이 거부된다. 통과. |
| 9 | `overflow:visible`을 author declaration으로 추가해 fixture가 profile 밖으로 넓어진다. | CSS source에 선언을 넣지 않고 initial computed overflow만 검사한다. 명시적 선언 mutation은 layout projection에서 거부된다. 통과. |
| 10 | vertical writing mode가 물리 축을 바꿔 같은 숫자를 잘못 판정한다. | Root computed style gate와 vertical writing-mode mutation이 거부된다. 통과. |
| 11 | child 높이 또는 width가 달라져 max-content contribution 근거가 끊긴다. | Child의 fixed 60×200px style 및 frame을 확인한다. 201px mutation이 거부된다. 통과. |
| 12 | style snapshot과 layout output이 서로 다른 revision에서 온다. | Generation, document, render tree, style, environment revision을 대조한다. 두 simulator 모두 presentation identity 일치. 통과. |
| 13 | root/item/child 중복·누락 frame이 선택적으로 무시된다. | Exact 세 node ID 집합, 중복 없는 frame map, 세 기대 frame을 검사한다. 통과. |
| 14 | Columns pass가 Rows 결과를 대신하거나 stage가 누락·재정렬된다. | Root trace 한 개, Columns/Rows 각 한 pass, Rows 여섯 stage를 exact 순서로 검사한다. 통과. |
| 15 | gutter index, collapsed boundary 또는 row 순서가 바뀐다. | 다섯 internal entries의 index, gutter, collapsed state와 두 실제 row base를 각 stage에서 검사한다. 통과. |
| 16 | 성장 한계가 40px을 넘거나 fit-content track이 flexible로 오인된다. | Initialize의 미설정 row limit, 이후 40px limit, 0px gutters 및 non-flexible flag를 검사하고 growth-limit/flexible mutation controls를 추가했다. 통과. |
| 17 | span-2 contribution 중 Minimum/MinContent 값이 틀려도 MaxContent 하나만으로 통과한다. | Rust runtime trace, Android, iOS 및 capture verifier가 `[Minimum 0, MinContent 200, MaxContent 200]`을 item/span/source와 함께 exact 검사한다. 통과. |
| 18 | Rust enum debug 문구가 CSS computed overflow로 오인돼 platform 비교만 실패한다. | 첫 Android 실패에서 `Some(Visible)` 대 `visible`을 확인했다. report를 CSS keyword로 바꾸고 양쪽 app matcher와 최종 실행을 재검증했다. 통과. |
| 19 | layout trace는 맞아도 item 밖 child가 실제 표면에 그려지지 않는다. | Android PixelCopy 및 iOS Metal drawable 표본에서 item 내부/경계 밖 child, child 이후 root 배경, root 외부를 각 4/4 exact 확인했다. 통과. |
| 20 | emulator/simulator 증거를 실기기나 compositor 이후 표시로 과장한다. | Android는 API 37 ARM64 AVD PixelCopy, iOS는 iPhone 17 Pro Simulator의 compositor 제출 전 Metal readback으로 문서화했다. 실기기/hardware GPU/iOS compositor 이후는 미검증으로 남겼다. 통과. |

최종 source snapshot, scene 결과, 빌드 hash와 한계는 [실행 README](./README.md)에 기록했다.
