# C11.4 · cyclic <code>minmax(calc())</code> mixed cross-axis span groups under max-content

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 mixed cross-axis span-groups 계획](./c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 Agent Browser 사전 관측과 CSS Grid 2025 CRD 독립 계산을 네 fixed scene에서 기록했고, opt-in runtime fixture와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 검증을 마쳤다. 각 플랫폼 4/4 scene, PixelCopy 및 compositor 제출 전 Metal 표본 8/8 exact다. 구현·artifact·검증 범위는 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-runtime-2026-10-22/README.md)에 기록했다. C11.4 checkbox와 공식 완료 수 <code>77/213</code>은 그대로 둔다.

## 목적

직전 [mixed cross-axis slice](./c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md)는 세 direct Ahem text item의 2/2/3 column-span group에서 min-content와 definite 40px width를 확인했다. 후속인 이 slice는 같은 입력을 max-content width로 측정해 cyclic percentage의 두 단계 처리와 span-group sizing을 확인한다. 80px definite width는 max-content contribution이 아니라 사용 가능한 고정 공간에서 track maximize와 text wrapping을 대조한다.

CSS Grid 2025 CRD §7.2.1은 track 크기의 cyclic percentage를 container intrinsic size 계산에서 auto처럼 취급한 뒤, 얻은 container size를 기준으로 layout 때 해석한다. §11.5–§11.5.1은 spanning contribution을 span 길이 순서대로 처리하고 track별 planned increase 최대값을 적용한다. 독립 계산은 Chrome 값에서 역산하지 않고 [spec-derived-expectations.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/spec-derived-expectations.json)에 기록했다.

## 고정 입력과 장면

재현 파일은 [CSS oracle source](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-cyclic-minmax-calc-cross-axis-max-content-span-groups-c114-precomparison.html)이고, Agent Browser가 연 동일 입력은 [evidence oracle](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/oracle.html)이다. Font는 저장소의 <code>tests/fixtures/resources/c15/Ahem.ttf</code>, 400 weight, 10px/10px이며 SHA-256은 <code>b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448</code>다.

모든 scene은 3열 × 2행 Grid, 모든 track의 <code>minmax(calc(10% + 2px), max-content)</code>, 양 축 2px gap, 2px border, 0 padding, content-box root을 쓴다. A는 <code>AA A A</code> auto-size text로 columns <code>1 / 3</code>과 rows <code>1 / 3</code>을 span한다. B는 <code>XXXX</code>, 40×10px fixed item으로 columns <code>2 / 4</code>와 rows <code>1 / 3</code>을 span한다. C는 <code>AA AA AA AA</code> auto-size text로 columns <code>1 / 4</code>와 rows <code>1 / 3</code>을 span한다. 모든 item은 direct child다.

| Scene | Root width | DOM order | 독립 예상 columns | 독립 예상 rows | Root content / border box | A/B/C text lines |
| --- | --- | --- | --- | --- | --- | --- |
| <code>max-content-forward</code> | max-content | A, B, C | <code>[107/3, 125/3, 86/3]</code> ≈ <code>[35.6667, 41.6667, 28.6667]</code> | <code>[4, 4]</code> | <code>110×10 / 114×14px</code> | 1 / 1 / 1 |
| <code>max-content-reverse</code> | max-content | C, B, A | same | same | same | 1 / 1 / 1 |
| <code>definite-forward</code> | 80px | A, B, C | <code>[76/3, 76/3, 76/3]</code> ≈ <code>[25.3333, 25.3333, 25.3333]</code> | <code>[9, 9]</code> | <code>80×20 / 84×24px</code> | 2 / 1 / 2 |
| <code>definite-reverse</code> | 80px | C, B, A | same | same | same | 2 / 1 / 2 |

Rows are cyclic too. In the max-content scenes their intrinsic height is 10px, so the resolved row minimum is 3px. In the 80px controls the wrapped text makes the intrinsic height 20px, so the resolved row minimum is 4px. A and C then determine the final [9, 9] row tracks. Item frames, line widths, and all fractions are in the expectation JSON.

## 독립 계산과 관측

Intrinsic max-content sizing treats the cyclic percentage component as auto while retaining the absolute 2px term. The min-content span-2 group proposes per-track increases 9 for A and 19 for B; the §11.5.1 maximum gives track bases [9, 19, 19]. For max-content contributions, A sets the first two growth limits to [24, 34]. C's 110px contribution leaves 29px after those limits, the third base, and the two 2px gutters; the 29px is split across three tracks. The intrinsic container width is therefore 110px. After resolving each cyclic column minimum to 13px, the final max-content pass gives [107/3, 125/3, 86/3].

For the 80px control the column minimum resolves to 10px. B's 40px min-content contribution produces growth limits [infinity, 19, 19]; A's 60px max-content contribution changes these to [24.5, 33.5, 19], and C's 110px contribution changes them to [205/6, 259/6, 86/3]. The available 76px of track space is then maximized equally from 10px bases, resulting in three 76/3px tracks. A and C wrap to two lines, establishing a 20px max-content height; the cyclic row minimum resolves to 4px and final Rows are [9, 9]. Full arithmetic and tolerances are in the JSON.

Chrome 156's computed tracks are rounded to 1/64 CSS px and differ from the rational expectations by at most 1/64 CSS px. A same-page diagnostic with an explicit <code>auto</code> minimum distinguishes the intrinsic pass; it is labeled Chromium-only and is not used as the independent expectation. See the [Agent Browser observation](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/chrome156-observation.json), [diagnostic variants](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/chrome156-diagnostic-variants.json), and [screenshot](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/chrome156-max-content-span-groups.png).

## 구현 관문

1. 기존 C11.4 opt-in fixture-test 경계에 새 scene 네 개만 추가한다. item 수·identity·source-order variant·placement·text·track expression·font hash를 exact allowlist로 고정하고 다른 input은 fail-closed한다.
2. Columns trace에서 intrinsic pass의 cyclic-auto basis와 resolved 13px/10px minimum pass를 분리한다. span-2 A/B contributions는 한 group에서 track별 maximum planned increase를 만들고, span-3 C는 뒤에 처리되어야 한다. max-content 110px과 80px definite control의 trace를 별도 assert한다.
3. Rows trace는 Columns layout 뒤 측정된 text 높이를 사용한다. max-content 장면은 A/B/C 한 줄과 Rows [4,4]; 80px control은 A/C 두 줄, B 한 줄과 Rows [9,9]여야 한다. Definite control의 cyclic row basis 20px과 resolved 4px minimum을 기록한다.
4. 네 scene의 track sizes, root content/border boxes, item frames와 Range line widths를 Chrome rational expectation 대비 1/64 CSS px 이내로 비교한다. Reverse DOM order는 track list와 item-keyed geometry를 바꾸지 않아야 한다.
5. 기존 fixture feature를 사용한다. 제품 CSS admission이나 새 공개 API를 추가하지 않는다. 기본 release artifact와 opt-in fixture-test artifact의 경계를 검증하고, C11.4 cyclic/min-content 및 mixed-span 회귀를 실행한다.
6. 고정 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 앱 runtime geometry와 draw/present를 검사한다. 실기기 연결은 요구하지 않는다. AVD screenshot/PixelCopy와 iOS Metal drawable은 제출 전 표본으로 분리하고, glyph raster·iOS compositor 이후 pixels를 주장하지 않는다.
7. 구현 뒤 서로 다른 새 runtime 실패 관점 20개를 검토하고, PR 변경에는 별도 20개 관점을 기록한다. 결함을 찾으면 영향받는 scene·trace·Simulator 표본을 다시 확인한다.
8. 내부 계약 0059, C11.4 parent, <code>spec/STATUS.md</code>, 공개 roadmap과 runtime README의 표현을 일치시킨다. C11.4 checkbox, 77/213, 공개 CSS 지원 상태를 올리지 않는다.

## 성공·실패 판정

- Cyclic intrinsic first pass와 resolved percentage layout pass를 서로 다른 trace stage로 남기고, max-content root width는 110px이어야 한다.
- Final max-content columns는 [107/3, 125/3, 86/3]px, rows는 [4,4], root border box는 114×14px이다. 80px control columns는 [76/3,76/3,76/3]px, rows는 [9,9], root border box는 84×24px이다. Track tolerance는 독립 산술값 기준 1/64 CSS px다.
- Max-content에서 A/B/C는 각 한 줄이며 item frame widths는 238/3px, 40px, 110px이다. 80px control의 A/C는 각각 [40,10]px 및 [80,20]px line widths로 두 줄, B는 한 줄이다.
- Reverse DOM order에서 identity별 contribution, tracks, text lines 및 frame이 같아야 한다. B는 max-content에서 고정 40px을 유지하며 definite control에서 grid area 밖으로 넘는 overflow를 숨기지 않는다.
- Android/iOS simulator·artifact evidence는 platform과 capture boundary를 따로 기록한다. Chrome oracle pass를 앱 결과로 세거나 simulator 결과를 실기기/WPT 증거로 확대하면 실패다.
- 등록되지 않은 item/tree/style/source order의 일부만 결과에 남거나 C11.4 전체를 완료로 게시하면 실패다.

## 제외 범위

네 fixed scene 밖의 임의 item/span count, 다른 topology, nested/anonymous/replaced item, arbitrary CSS input, width 외 definite height control, min-content 재구현, WPT, 일반 CSS admission, 제품 API, Android 실기기/hardware GPU, iOS compositor 이후 pixel 및 전체 C11.4 완료는 제외한다.

## 검토와 근거

- [Chrome 156 사전 비교·독립 계산·해시](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/README.md)
- [계획 실패 관점 20개](./c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-plan-review-20.md)
- [계획 PR 변경 실패 관점 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups-precomparison-2026-10-22/plan-pr-diff-review-20.md)
- 기존 [mixed-span 계획](./c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups.md), [실행 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-cross-axis-mixed-span-groups-runtime-2026-10-22/README.md) 및 [PR #515 회귀 근거](../spec/internal/evidence/c11-4-pr515-prereq-regression-2026-10-22/README.md)

## 구현 결과 (2026-10-22)

기존 `c11-4-grid-track-sizing-fixture-test`에 MaxContent width scene 두 개와 80px definite control 두 개를 추가했다. Rust trace의 cyclic-auto intrinsic basis와 resolved percentage pass를 분리하고 span-group 순서, Columns→Rows 의존성, text line widths, reverse-order geometry를 검사한다. Android runtime pixel expectation에 신규 고정 sample ID를 추가했고 iOS fixture build에서는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`을 켰다.

시뮬레이터 캡처 도구는 이미 빌드된 앱을 설치한다. 첫 Android 시도는 오래된 APK여서 새 scene ID를 거부했다. 최신 fixture APK로 다시 빌드한 뒤 네 scene이 통과했다. 다음 PixelCopy 시도에서 새 sample ID의 Android 기대색이 미등록이라 `-1` mismatch가 드러나 border/clear mapping을 수정하고 재실행했다. 첫 iOS capture도 Metal surface-pixel build flag가 꺼져 `status=-90`으로 거부됐으며 해당 fixture flag를 켜고 재빌드한 뒤 네 scene을 통과했다. 실패 및 재실행 이유를 숨기지 않고 최종 성공 로그·hash를 runtime evidence에 보존한다.

기본 release FFI archive에는 새 scene 문자열과 fixture prepare symbol이 없고 fixture-test positive control에는 있다. 이 검사는 정적 archive 경계이며 최종 APK/IPA 증명이 아니다. 구현/runtime 실패 관점 20개 및 PR 변경 실패 관점 30개는 runtime evidence에 별도로 기록한다. 일반 CSS admission, WPT, 실기기/hardware GPU, glyph raster equality, iOS compositor 이후 pixel과 전체 C11.4는 범위 밖이며 C11.4 checkbox와 공식 `77/213`은 유지한다.
