# 구현/runtime 적대 검토 20개

계획 검토 20개와 별도로 최종 구현, focused trace, Android API 37 AVD와 iOS 26.2 Simulator 실행을 실패 경로별로 다시 확인했다. 정적 검토인 항목은 그 범위를 표시했다. 처음 확인 중 발견한 “replaced image의 `align-self:start` 허용이 기존 Grid item에 번질 수 있음”은 projection option을 새 fixed fixture helper에만 켜고 일반 helper의 음성 검증을 추가해 수정한 뒤 영향 항목을 재확인했다. 마지막 정리에서 공통 assertion helper를 cases 모듈로 옮겨 Rust 파일을 각각 479줄/309줄로 나눴고 style focused test와 feature Clippy를 다시 통과시켰다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 등록하지 않은 scene ID가 새 CSS/runtime 경로로 진입 | FFI dispatcher가 네 exact ID만 소스 생성, unknown ID는 오류. FFI validator 통과. PASS |
| 2 | 올바른 ID를 다른 profile, root tag 또는 owner에 재사용 | runtime adapter가 profile, direct root `div`, root parent와 빈 `data-node`를 검사한다. 코드 검토. PASS |
| 3 | root에 item 추가·누락·중복 | direct child가 정확히 3개가 아니면 거부한다. 코드 검토. PASS |
| 4 | DOM child order와 scene source order 불일치 | item id, parent와 expected order를 함께 비교한다. forward/reverse adapter 테스트 통과. PASS |
| 5 | text item을 익명 text 또는 복수 text node로 바꿈 | 각 `div`에는 direct text leaf 한 개와 exact Ahem text만 허용한다. 코드 검토 및 runtime text intrinsic trace. PASS |
| 6 | text를 `img`/다른 namespace element로 바꿈 | `(ItemKind, tag)` 조합을 제한하고 다른 source kind는 거부한다. 코드 검토. PASS |
| 7 | image children, 다른 `src`, 대체 URL 사용 | leaf `<img>`와 고정 `.invalid` URL만 허용한다. FFI source test와 runtime adapter 검토. PASS |
| 8 | decode 실패나 빈/다른 PNG로 재현 입력 변경 | 고정 PNG bytes decode를 요청에 결합하고 오류를 반환한다. 코드 검토. PASS |
| 9 | natural size/aspect metadata가 CSS width와 다르게 전달 | `24×16` natural dimensions와 `3:2` ratio를 확인한다. Style-to-layout intrinsic source test 통과. PASS |
| 10 | Ahem font 등록 실패 또는 system font fallback | system font 없이 새 engine을 만들고 고정 Ahem bytes 등록 실패는 layout failure로 전파한다. runtime source 및 line metrics 확인. PASS |
| 11 | intrinsic/definite width scene 교차 사용 | Scene ID별 `min-content` 또는 `40px`와 root height/box style를 검사한다. 네 scene 실행 통과. PASS |
| 12 | `calc(10% + 2px)`를 다른 axis/track count/order로 주입 | track count, typed percentage + length terms, max-content, line-name·auto-repeat 부재를 검사한다. Trace/track assertions 통과. PASS |
| 13 | gap, border, margin, padding, content-box 변경 | root 2px gap/border, content-box, zero margin/padding 조건을 exact style 검증한다. 코드 검토 및 root frame 확인. PASS |
| 14 | row/column placement 바뀌어 다른 span topology 생성 | item별 `1/3`, `2/4`, `1/4` columns와 `1/3` rows를 검사한다. Source test·trace 검증 통과. PASS |
| 15 | image width/min-width/ratio 조건을 제거해 다른 row input 생성 | image used width/min-width `40px`, height auto, no authored ratio를 요구한다. intrinsic contribution `80/3px` trace 통과. PASS |
| 16 | text/grid item까지 `align-self:start` 허용 | replaced HTML image metadata와 별도 option이 모두 있어야 허용된다. 신규 helper만 option을 켠다. PASS |
| 17 | 새 정렬 예외가 기존 public fixture helper에 번짐 | 일반 text + intrinsic-size helper에서 같은 image start alignment가 오류인지 negative test로 확인. PASS |
| 18 | arbitrary helper가 잘못된 profile/viewport를 두 축 fixture로 처리 | 신규 helper가 profile과 viewport equality를 먼저 검사한다. Style-to-layout entry 검토. PASS |
| 19 | text/image intrinsic input이 다른 revision이거나 item이 누락 | 하나의 image resource snapshot revision을 만들고 runtime layout에 완전한 text map과 image snapshot을 넘긴다. three-item intrinsic source assertion 및 scene trace 통과. PASS |
| 20 | order-dependent group sizing, stale fallback 또는 미제출 draw를 성공으로 표시 | span-2/3 staged bases, item-keyed frames, reverse order, `drawPresented`를 검사한다. Rust trace와 양 simulator 4/4 통과. PASS |

이 검토는 네 fixed scene의 구현 경계와 실행을 확인한다. 일반 CSS parser admission, WPT, glyph paint, 실제 Android GPU와 iOS compositor 이후 화면은 검토/입증 범위가 아니다.
