# C11.2.1 Grid 배치 구현 실패 관점 검토 · 2026-10-12

**판정:** explicit numeric line/span 제한 profile의 구현·테스트·모바일 실행 경계를 20가지 실패 상황으로 다시 확인했다. 검토 중 nested Grid의 container별 배치 모드, `display:none` 자식의 잘못된 placement, 겹치는 항목의 paint snapshot 순서를 직접 고정하는 회귀 검사가 빠져 있어 추가했다. 이 검토는 C11.2.1 전체 완료, CSS 전체 지원, Android 실기기 또는 pixel parity를 뜻하지 않는다.

| # | 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | positive line `1..track_count+1` 경계와 row/column 축이 뒤바뀜 | 통과. Chromium 고정 fixture와 layout 정규화 검사에서 두 축의 line 입력을 구분한다. [`grid_placement.rs`](../../../crates/spinon-layout/src/grid_placement.rs) · [C11.2.1 비교 fixture](../../../tests/fixtures/css/references/c11-grid-placement-precomparison-v2.json) |
| 2 | negative line이 마지막 track의 끝이 아닌 다른 line을 기준으로 계산됨 | 통과. `-1`·`-2` 및 양 끝 음수 line fixture를 확인했다. `explicit_line`은 explicit track의 끝 line에서 역산한다. |
| 3 | CSS line `0`이 Taffy의 auto 값으로 조용히 변환됨 | 통과. zero line은 adapter에서 `InvalidGridPlacement`로 거부하고 스타일 통합 회귀도 있다. |
| 4 | `line / span N`의 덧셈 overflow 또는 off-by-one | 통과. checked addition 후 명시 track 경계를 확인한다. positive span fixture가 Chrome frame과 비교된다. |
| 5 | `span N / line`의 뺄셈 방향 또는 end 기준이 뒤집힘 | 통과. checked subtraction과 span-from-end Chromium fixture를 확인했다. |
| 6 | span 0, 너무 큰 span, 음수 span이 정상 배치로 오인됨 | 통과. zero와 `u32::MAX`는 layout 단위에서 거부하며, 부호 없는 변환 실패는 style projection 경계에서 오류가 된다. |
| 7 | 역순 line pair를 잘못 처리하거나 동일 line을 0-size track으로 통과시킴 | 통과. 역순은 교환하고 동일 line은 auto-placement 요구로 거부한다. 양쪽 동작 모두 fixture가 있다. |
| 8 | 같은 container에서 auto item과 explicit item이 섞여 부분 계산됨 | 통과. 전체 요청을 `invalid_grid_placement`로 실패시키는 runtime 회귀가 있다. |
| 9 | 바깥·중첩 Grid의 placement mode가 서로 간섭함 | 통과. 바깥 all-auto와 안쪽 explicit 배치를 함께 계산하는 회귀 테스트를 추가했다. |
| 10 | `display:none` subtree의 잘못된 배치값이 보이는 Grid를 실패시킴 | 통과. 숨겨진 자식의 line 0을 무시하고 0×0 frame으로 유지하는 회귀 테스트를 추가했다. |
| 11 | explicit track 밖 line/span을 Taffy가 implicit track 또는 clamp로 수용함 | 통과. `1..=track_count+1` 밖 정규화 결과를 계산 전에 거부한다. |
| 12 | 두 endpoint가 모두 span이거나 한 endpoint만 definite인데 성공 처리됨 | 통과. layout normalizer가 해당 조합을 거부하고 스타일 fixture의 mixed-auto 실패도 통과한다. |
| 13 | named line/area 또는 이 단계 밖 auto-placement 값이 numeric line으로 잘못 읽힘 | 통과. unsupported computed value는 node·property 문맥을 보존해 거부한다. named line, implicit placement와 mixed auto/explicit 회귀가 있다. |
| 14 | shorthand 해석이 직접 재구현되어 cascade 우선순위를 덮어씀 | 통과. Stylo computed longhand를 typed snapshot으로 전달하고 `grid-area` 및 longhand override를 고정 fixture와 비교한다. |
| 15 | typed computed value를 다시 CSS 문자열로 파싱하며 의미가 달라짐 | 통과. line/span typed value가 문자열 재파싱 없이 layout input으로 전달되는 통합 테스트가 있다. |
| 16 | explicit 배치의 auto item size 지원이 C11.1 all-auto profile을 바꿈 | 통과. explicit 배치에는 auto/fixed size를 허용하고 기존 profile은 별도 유지한다. 관련 gate는 [`grid.rs`](../../../crates/spinon-layout/src/grid.rs)에 있으며 auto-sized explicit fixture가 실행된다. |
| 17 | 겹치는 explicit item의 geometry가 pinned Chrome과 다름 | 통과. Chrome overlap fixture와 같은 두 item을 비교하도록 중복 세 번째 node를 제거하고 실제 대상 두 node frame을 확인했다. |
| 18 | 겹치는 item이 layout 후 paint snapshot에서 DOM 순서를 잃음 | 통과. 동일 위치 두 item의 geometry overlap과 snapshot node 순서를 확인하는 runtime 회귀를 추가했다. GPU pixel readback은 하지 않았다. |
| 19 | 뒤쪽 invalid item이나 잘못된 FFI 인수가 부분 layout을 노출함 | 통과. invalid item은 성공 결과 없이 오류로 반환하고 FFI null host/output 경계를 별도 검사한다. 이는 앱 전체 메모리 실패 복구 보장을 뜻하지 않는다. |
| 20 | Android/iOS 진입 인자·fixture 선택·화면 라벨이 다른 실행을 가리킴 | 제한 실행 통과. Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 V8 scene의 4 frame과 WGPU present를 기록했다. iOS 인자 전달 및 Android C11.2.1 라벨을 고쳤다. Android 실기기, 해당 장면의 pixel readback, 전체 mobile matrix와 Spinon WPT 적합성은 미확정이다. [실행 근거](./c11-2-1-grid-mobile-runtime-2026-10-12/README.md) |

## 남은 완료 관문

- 현재 ADB에 `emulator-5554`만 연결되어 있어 C11.2.1 Android 실기기 실행은 하지 않았다.
- 이 단일 scene의 Android PixelCopy 및 iOS Metal drawable pixel readback, 10-scene 전체 mobile matrix는 별도 실행이 필요하다. screenshot과 present marker는 pixel parity가 아니다.
- pinned WPT 후보의 Chrome 결과는 oracle 근거이며 Spinon WPT 적합성 판정이 아니다. 전체 CSS Grid, named line/area, implicit track, mixed auto-placement, dense/column flow와 intrinsic sizing은 미완료다.
- 따라서 C11.2.1, C11 상위와 공개 CSS 지원은 미완료로 유지한다.
