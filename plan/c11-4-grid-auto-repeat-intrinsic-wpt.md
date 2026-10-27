# C11.4 · WPT `grid-auto-repeat-intrinsic-001.html` 제한 runtime slice

**상태:** 계획 PR [#690](https://github.com/ohah/spinon/pull/690)과 구현 PR [#691](https://github.com/ohah/spinon/pull/691)이 모두 병합됐다. 구현 PR은 2026-10-27T10:30:12Z `Rebase and merge`됐고 최종 merge SHA는 `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`다. 제한된 8-scene runtime 구현과 두 simulator 실행, 구현/runtime 실패 관점 검토 20개, 별도 PR diff 실패 관점 검토 30개를 마쳤다. Agent Browser에서 두 실행 화면의 원격 렌더링을 확인했다. PR check rollup은 비어 있고 main에는 required review/status check가 없어 CI 통과나 독립 승인을 주장하지 않는다. 이 문서는 WPT harness의 Spinon 통과나 C11.4 전체 완료를 주장하지 않는다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [Chrome 156·규범 기대값·고정 입력](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/README.md) · [독립 기대값 JSON](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/expected.json) · [WPT 원본 및 helper hash](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/source-manifest.json)

## 목적과 기준 입력

고정 Web Platform Tests revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/grid-auto-repeat-intrinsic-001.html`에 있는 8개 Grid를 모두 다룬다. `auto-fill`과 `auto-fit`, 두 trailing track 함수, `max-content`와 `min-content` 폭을 교차한다. 한 WPT 문서 안의 sibling Grid 각각을 같은 입력과 독립 기대값으로 검사한다. 앱에서는 동일 계약을 가진 여덟 개의 고정 scene ID로 나눠 실행한다. 이는 원본 문서의 나머지 기본 body flow 전체를 구현하거나 WPT harness를 Spinon에서 실행한다는 뜻은 아니다.

Chrome 관찰은 Agent Browser 0.31.1이 실행한 HeadlessChrome 156.0.0.0, DPR 1, viewport 1280×633에서 고정했다. 원본 testharness는 8개 `.grid` test 모두 PASS였고, root와 각 자식의 크기·offset을 비교했다. 원본 및 다섯 helper 파일의 SHA-256, 결과 DOM/geometry, 화면, W3C 발췌문은 사전 비교 폴더에 둔다. Chrome 결과는 기준 관찰이며 Spinon 실행이나 전체 WPT 통과 근거가 아니다.

원본 Grid는 `.grid`의 `display:grid`와 회색 배경, 상대 위치, 검은 2px 테두리, 상단 padding 10px, `justify-items:start`를 사용한다. 폭은 helper class의 `max-content` 또는 `min-content`다. 유일한 in-flow item은 `grid-column:1 / -1`, `width:100px`, 높이 20px이고 전체 explicit column 영역을 span한다. 네 absolute probe는 각각 `grid-column:1/2`부터 `4/5`까지 사용하며 높이는 5px, inset은 auto, 폭은 100%다. 원본 `grid-auto-columns`/`grid-auto-rows`는 초기값을 사용한다. 화면 fixture는 이 CSS, 색상 및 child 순서를 보존한다.

## 규범적 독립 계산

기대값은 Chrome의 computed style을 복사하지 않고 [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에서 계산한다.

- §5.2에서 intrinsic Grid width는 해당 제약으로 track size를 계산한 합이다. border는 양쪽 각 2px이므로 content width에 4px을 더해 root border-box width를 구한다.
- §7.2.3.2의 repeat-to-fill 규칙에서 auto-repeat count는 고정 repeated track과 이웃 track의 count용 definite sizing function을 적용한다. `minmax(min-content,40px)`의 count용 폭은 40px이고 repeated track은 20px이다. 100px content constraint에서 `(100−40)/20`이 3이므로 count 3이다.
- `minmax(60px,auto)`는 count에 고정 minimum 60px을 쓰고 repeated track은 10px이다. max-content content width 100px에서는 `(100−60)/10`이 4, min-content content width 70px에서는 `(70−60)/10`이 1이다.
- 100px fixed-width in-flow item이 전체 explicit track을 span하므로 fixed-tail max/min-content 두 case는 세 repeated tracks와 최대 40px suffix로 100px을 수용한다. 두 repeated tracks만 두면 합계 80px이고 네 개면 120px이라 각각 item contribution에 부족하거나 100px constraint를 넘는다. auto-tail max-content는 `4×10+60=100px`로 span contribution을 수용한다. auto-tail min-content는 fixed minimum 합 `10+60=70px`이고, 100px item은 이 좁은 content box에서 줄지 않고 overflow한다.
- §11.5의 track sizing에서 한 in-flow item이 모든 repeated track을 span한다. 따라서 `auto-fit`의 각 repeated track은 item이 놓이거나 span한 non-empty track이며 auto-fit collapse 대상이 아니다. 네 absolute probe는 track sizing이나 auto-repeat count에 참여하지 않는다.
- §9.1의 out-of-flow Grid placement는 기존 implicit track을 늘리지 않는다. 존재하지 않는 line을 참조하면 그 placement line은 auto로 처리된다. 특히 두 min-content/`minmax(60px,auto)` case의 열은 `[10,60]px`뿐이다. 세 번째 probe는 `x=70,width=0`; 네 번째 probe의 두 범위 밖 line은 padding-edge fallback을 만들어 `x=0,width=70`을 낸다.

| # | 고정 scene ID | Repeat | Root width keyword | 반복 수 | 최종 columns (CSS px) | Root border box |
|---:|---|---|---|---:|---|---:|
| 1 | `auto-repeat-intrinsic-fill-fixed-max` | auto-fill 20px + minmax(min-content,40px) | max-content | 3 | `[20,20,20,40]` | 104×34 |
| 2 | `auto-repeat-intrinsic-fill-fixed-min` | auto-fill 20px + minmax(min-content,40px) | min-content | 3 | `[20,20,20,40]` | 104×34 |
| 3 | `auto-repeat-intrinsic-fill-auto-max` | auto-fill 10px + minmax(60px,auto) | max-content | 4 | `[10,10,10,10,60]` | 104×34 |
| 4 | `auto-repeat-intrinsic-fill-auto-min` | auto-fill 10px + minmax(60px,auto) | min-content | 1 | `[10,60]` | 74×34 |
| 5 | `auto-repeat-intrinsic-fit-fixed-max` | auto-fit 20px + minmax(min-content,40px) | max-content | 3 | `[20,20,20,40]` | 104×34 |
| 6 | `auto-repeat-intrinsic-fit-fixed-min` | auto-fit 20px + minmax(min-content,40px) | min-content | 3 | `[20,20,20,40]` | 104×34 |
| 7 | `auto-repeat-intrinsic-fit-auto-max` | auto-fit 10px + minmax(60px,auto) | max-content | 4 | `[10,10,10,10,60]` | 104×34 |
| 8 | `auto-repeat-intrinsic-fit-auto-min` | auto-fit 10px + minmax(60px,auto) | min-content | 1 | `[10,60]` | 74×34 |

All eight roots have a 100×20 in-flow item at padding-edge-relative `(0,10)`. The first two/five/six cases have absolute probes `(x,width)` of `(0,20)`, `(20,20)`, `(40,20)`, `(60,40)`. Cases 3/7 use `(0,10)`, `(10,10)`, `(20,10)`, `(30,10)`. Cases 4/8 use `(0,10)`, `(10,60)`, `(70,0)`, `(0,70)`. Every probe has height 5px and padding-edge-relative y=0. The exact JSON records each expected width, height and offset separately.

Cases 4 and 8 deliberately place the 100px item in a 70px content box. Relative to the root border-box origin, the item starts at x=2 and ends at x=102; the root border box ends at x=74. The painted item therefore extends **28px beyond the right border edge**. Android PixelCopy and the iOS Simulator Metal drawable must sample a point beyond the border but inside the item, an adjacent point past the item, and the right-border control. Geometry alone or a screenshot does not substitute for those surface samples.

## 구현 범위와 거부 경계

1. `c11-4-grid-track-sizing-fixture-test`의 opt-in fixture 경로에 고정 scene table과 source adapter를 추가한다. CSS root, in-flow item, absolute probe의 tag/ID/순서/자식 수, computed typed style, root position, border/padding, two supported track shapes, repeat mode, width keyword와 네 placement만 exact-match한다. Unknown ID, 누락·추가·중복 child, 값이 다른 style, 다른 track grammar는 layout 전 거부한다.
2. 기존 `c114_absolute_auto_repeat_probes::supports_wpt_probe_matrix`의 auto-fill·fixed suffix 범위를 암묵적으로 넓히지 않는다. 새 독립 matcher 또는 공통 검증 helper에서 이 WPT의 정확한 8-case matrix를 feature gate 아래 추가하고, 실제 존재하지 않는 explicit line을 implicit track처럼 확장하지 않는 negative/control 검사를 둔다.
3. `spinon-style-to-layout`에서 두 auto-repeat mode와 두 `minmax()` neighbor shape가 computed track order·property owner·auto-repeat fragment로 손실 없이 전달되는지 검사한다. intrinsic axis와 explicit track input의 unsupported 조합은 `LayoutError`로 fail-closed한다.
4. Pure layout 검증은 root/item/every absolute frame, complete final column list, auto-repeat count, track sizing stages·contribution owner를 각 scene별 expected JSON과 대조한다. Absolute probes는 in-flow item count, contribution list, auto-repeat count에서 제외한다. 0-width third probe는 frame result에 보존하되 painted-box 수와 surface sample 수에서는 제외한다.
5. Runtime V8-authored `HostDocument` fixture를 고정 8개 scene ID로 연결한다. Scene lookup과 root/style/node topology가 정확히 맞는 경우만 처리한다. 다른 scene·CSS·placement는 계산 전에 거부하고 부분 layout 또는 trace를 성공 결과로 반환하지 않는다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source revision으로 8/8 scene을 각각 실행한다. 각 scene에서 installed app/build hash, root 및 child frames, columns trace, draw/present와 screenshot을 보존한다. Cases 4/8은 PixelCopy와 Metal drawable에서 border, border 밖 cyan item, item 바깥 background negative control을 각각 sample한다. App scene matrix는 WPT runner 통과나 iOS compositor 이후 화면을 증명하지 않는다.
7. 기본 product `spinon-layout`/`spinon-ffi` archive와 opt-in positive-control archive를 대조해 새로운 fixture module·symbol·marker·scene payload의 경계를 검사한다. 이 검사는 static archive 범위이며 APK/IPA 전체 fixture exclusion을 주장하지 않는다.
8. 구현 완료 뒤 새로운 실패 관점 20개, 구현 PR 변경분에 앞선 검토와 다른 적대 관점 30개를 기록한다. 결함을 발견하면 수정 후 해당 관점을 다시 확인한다. 계획 PR과 구현 PR은 각각 별도로 review한다. 변경 결과는 `Rebase and merge`로 반영하고, C11.4 checkbox와 공식 `72/213`은 그대로 둔다.

## 구현·실행 상태 (2026-10-27)

- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 소스 8/8 scene씩 통과했다. 양쪽 모두 final track, node frame, 두 축 trace, draw/present가 맞았다. overflow 두 장면의 Android PixelCopy와 iOS compositor 제출 전 Metal 표본은 각각 3/3 RGBA exact다. [최종 실행 근거](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-runtime-2026-10-27/README.md)
- 기본 `spinon-ffi` `.rlib`에서 새 fixture symbol이 빠지고 `c04-runtime-gpu-fixture-test` opt-in positive control에만 포함된다. APK/IPA 전체 fixture 배제는 검사하지 않았다. [artifact 경계 근거](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-runtime-2026-10-27/feature-boundary.md)
- 구현/runtime 실패 관점 검토 20개를 마쳤다. 검토 중 Android/iOS trace matcher가 stretch 전 track을 검사하던 점과 Android malformed stage null 처리 결함을 발견해 수정하고 두 플랫폼 전체 8 scene을 다시 실행했다. [구현/runtime 검토](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-runtime-2026-10-27/implementation-runtime-review-20.md)
- 변경 파일 간 계약·artifact·증거·로드맵을 대상으로 별도 PR diff 실패 관점 30개를 검토했고 차단 결함은 찾지 못했다. 원본 Android runtime log 두 행 끝의 공백은 로그 보존을 위해 유지했다. [PR diff 검토](../spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-runtime-2026-10-27/pr-diff-review-30.md)
- PR #691은 `Rebase and merge`됐다(2026-10-27T10:30:12Z, 최종 merge SHA `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`). check rollup은 비어 있고 required review/status check는 설정되지 않아 CI 통과나 독립 승인을 주장하지 않는다. 제한된 WPT fixture만 병합됐으며 C11.4 전체는 완료되지 않았다.

## 통과 기준과 미완료 범위

- Pinned WPT 및 helper hash, Chrome testharness 8/8, 독립 기대값 8행이 source와 관계를 추적할 수 있게 고정된다.
- Projection/layout/runtime 모두 8/8 expected tracks, root/item/probe geometry, contribution 제외 규칙을 통과하며, error·unknown route·잘못된 scene shape negative control도 통과한다.
- Android/iOS Simulator에서 각각 8/8 scene을 검사하고, cases 4/8의 border 바깥 paint와 item 경계 negative control을 PixelCopy/Metal sample로 판정한다.
- 새 fixture는 기본 product artifact에서 제외되고 opt-in positive control에서만 포함된다.
- 구현/runtime review 20개와 별도 PR diff review 30개에서 해결되지 않은 차단 결함이 없다.

이는 이 고정 test의 제한된 fixture route다. 일반 CSS author admission, WPT runner에서의 Spinon 통과, 모든 `auto-fit`/`auto-fill` 입력, 다른 intrinsic breadth, arbitrary absolute placement, 전체 CSS Grid 또는 C11.4 완료를 의미하지 않는다. 호출 가능한 public API를 추가하지 않으며 내부 계약 숫자 버전 `0.1.0`, C11.4 checkbox, 공식 `72/213`을 변경하지 않는다.
