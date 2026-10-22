# C11.4 · cyclic `minmax(calc())` max-content 혼합 text/image span 그룹

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [cyclic max-content text span 그룹](./c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.md) · [cyclic min-content mixed text/image](./c11-4-cyclic-minmax-mixed-text-image-span.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 네 고정 장면 입력, CSS Grid 2025 CRD에서 도출한 유리수 기대값, Chrome 156 Agent Browser 관측, 서로 다른 계획 실패 관점 20개를 기록했다. runtime 구현은 시작하지 않았다. 계획 PR은 아직 병합되지 않았다. C11.4 checkbox와 공식 완료 수 `77/213`은 유지한다.

## 목적

기존 cyclic max-content span-group fixture는 Ahem text와 고정 크기 non-replaced item을 결합한다. 기존 cyclic min-content mixed fixture는 Ahem text와 replaced PNG image를 결합한다. 이 slice는 max-content sizing에 겹치는 text/image span group을 결합하고, image의 transferred block contribution이 cyclic Rows를 결정하는 교차 조합을 다룬다. definite 40px 대조 장면에서는 이미지가 Grid 테두리 오른쪽으로 12px 넘는 화면도 확인한다.

구현은 `c04-runtime-gpu-fixture-test` profile 안에서만 수행하며 일반 CSS admission이나 public API는 추가하지 않는다.

## 고정 입력

[Chrome oracle HTML](../tests/fixtures/css/c11/grid-cyclic-minmax-max-content-mixed-text-image-span-c114-precomparison.html)은 3-column × 2-row Grid를 쓴다. 두 축의 각 track은 `minmax(calc(10% + 2px), max-content)`, 각 gap은 2px, 각 border는 2px, padding은 0이고 root height는 `max-content`다.

- A는 Ahem direct text `AA A A`이며 columns 1/3과 rows 1/3을 span한다. min-/max-content inline contribution은 20/60px이다.
- B는 저장소의 24×16px, 3:2 PNG인 direct `<img>`다. CSS width/min-width는 40px, height는 auto이고 columns 2/4와 rows 1/3을 span한다. inline contribution 40px, block contribution은 `80/3px`이다.
- C는 Ahem direct text `AA AA AA AA`이며 columns 1/4와 rows 1/3을 span한다. min-/max-content inline contribution은 20/110px이다.
- 허용되는 장면 ID는 `max-content-forward`, `max-content-reverse`, `definite-forward`, `definite-reverse` 네 개뿐이다. 역순 장면은 DOM child 순서만 바꾼다.
- max-content 장면의 root width는 `max-content`다. definite 대조 장면은 40px content width다. 모든 장면의 height는 `max-content`다.

이미지/font hash, Chrome 156 관측, screenshot과 browser 환경은 [사전 비교 기록](../spec/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/README.md)에 있다.

## 독립 기대값과 oracle

CSS Grid 2025 CRD §7.2.1은 cyclic percentage를 intrinsic container sizing 중 auto처럼 취급하고 그 결과 inner size를 얻은 뒤 percentage를 resolve한다. §11.3–§11.5.1은 item contribution을 span 길이가 짧은 순서로 처리하고 같은 span group 안에서는 track별 planned increase 최댓값을 적용한다. max-content column 기대값은 기존 세 text max-content fixture의 독립 계산을 재사용한다. B image의 inline min/max contribution이 둘 다 고정 40px이므로 이 축의 입력은 같은 크기다.

- span-2 min-content contributions 뒤 intrinsic bases는 `[9,19,19]`이다. A/B의 max-content inline contributions 60/40px과 뒤이어 계산되는 span-3 C의 110px contribution으로 inner width는 110px이다.
- cyclic column minimum은 `10% × 110 + 2 = 13px`으로 resolve되고 최종 Columns는 `[107/3,125/3,86/3]px`이다.
- image의 block contribution은 `40 × 16/24 = 80/3px`이다. 두 row span에서 2px gutter와 각 2px initial base를 빼고 증가분을 나누면 Rows는 `[37/3,37/3]px`다. Intrinsic content size는 `110×80/3px`, border box는 `114×92/3px`이다.
- cyclic row minimum은 `10% × 80/3 + 2 = 14/3px`로 resolve되며 row bases보다 작다.
- definite 40px 대조 장면의 독립 Columns는 `[12,12,12]`, Rows는 `[19,19]`다. content는 40×40px, border box는 44×44px이다. A의 text line widths는 `[20,10,10]`, C는 네 개의 20px line이다. Image B는 x=16px에서 시작해 오른쪽 테두리를 12px 넘는다.

Chrome 156 Agent Browser 관측은 독립 값과 분리해 기록했다. Fractional track/frame에는 1/64 CSS px까지 허용한다. 전체 [규범 산출값](../spec/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/spec-derived-expectations.json)과 [브라우저 관측](../spec/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/chrome156-observation.json)을 참고한다.

## 구현 관문

1. adapter allowlist를 네 scene ID, 정확한 DOM order, A/B/C identity, text/image bytes와 hash, natural dimensions, CSS size, placement, gap, border, track expression으로 제한한다. 미등록 또는 일부만 들어온 입력은 fail-closed해야 한다.
2. text intrinsic source와 replaced-image natural size/ratio를 같은 layout revision에 전달한다. Trace는 item identity, source kind, intrinsic revision, 축과 contribution을 구별해야 한다.
3. max-content Columns, image block contribution, 양 축 cyclic resolve, 유리수 Rows와 item frame을 독립값에 대조한다. 정방향/역방향 장면의 identity별 geometry와 tracks는 같아야 한다.
4. definite 40px control의 text line geometry와 이미지의 12px 오른쪽 border overflow를 검사한다. Android API 37 ARM64 AVD의 실제 Surface PixelCopy에서 설정된 root border 지점, border 바깥의 녹색 PNG texel, image 뒤의 clear surface를 표본 추출한다. iPhone 17 Pro/iOS 26.2 Simulator에서는 같은 점을 compositor 제출 전 Metal drawable에서 읽는다. 이 표본은 전체 border-stroke 적합성을 증명하지 않는다.
5. 기본 제품 archive에는 새 fixture ID/symbol/payload가 없고 opt-in fixture-test archive에는 있는지 대조한다. Static archive 검사는 APK/IPA release 전체를 증명하지 않는다고 기록한다.
6. focused Rust/style/layout/FFI checks, fixture boundary, formatter, 관련 workspace/fixture Clippy와 Android/iOS Simulator build를 실행한다. 수정한 항목과 영향받은 행렬을 다시 실행한다.
7. 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개를 서로 분리해 기록한다. 이 계획 검토 행은 재사용하지 않는다.
8. 구현 PR에 Android/iOS simulator screenshot을 넣는다. live required checks와 merge 상태를 확인하고 Rebase and merge한다. C11.4와 `77/213`은 미완료로 유지한다.
9. 병합 뒤 Tailnet preview를 갱신하고 제공 파일의 hash와 화면을 Agent Browser로 확인한 뒤 사용한 session만 종료한다.

## 실패 조건과 제외 범위

같은 span group의 결과가 source order에 따라 바뀌거나, max-content text contribution이 min-content로 처리되거나, image natural dimensions/3:2 transferred block size가 사라지거나, Rows가 Columns 뒤 text wrapping을 받지 못하거나, cyclic percentage가 border-box 또는 잘못된 intrinsic axis를 기준으로 resolve되면 실패다. Rows/root/frame이 1/64 CSS px를 넘거나, 실제 surface에서 overflow 표본이 잘리거나 test overlay로 대체되거나, 미등록 입력에서 일부 layout이 나오거나, 기본 artifact에 opt-in fixture marker가 포함되는 경우도 실패다.

네 고정 장면 밖의 임의 CSS, 다른 image format/ratio, 동적 image 교체, 여러 image, nested/anonymous/out-of-flow item, auto-repeat, WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixels, 전체 CSS border-stroke 적합성과 C11.4 완료는 범위 밖이다.

## 근거와 검토

- [Chrome 156 사전 비교와 독립 계산](../spec/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/README.md)
- [계획 실패 관점 검토 20개](./c11-4-cyclic-minmax-max-content-mixed-text-image-span-plan-review-20.md)
- [계획 PR diff 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/plan-pr-diff-review-30.md)
- [max-content text span-group 선행](./c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.md)
- [cyclic min-content mixed text/image 선행](./c11-4-cyclic-minmax-mixed-text-image-span.md)

## 판정

구현 후보는 등록된 네 scene과 그 고정 입력에만 적용한다. 이 통과가 일반 CSS Grid 지원, WPT 적합성 또는 C11.4 완료를 뜻하지 않으며 상위 checkbox와 `77/213`은 바꾸지 않는다.
