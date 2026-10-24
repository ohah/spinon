# C11.4 auto-fit gutter 규범·Chromium·Taffy 재대조 · 2026-10-24

## 정정 요약

앞선 판정은 빈 `auto-fit` track의 양쪽 gutter를 각각 0px로 없앤다고 계산해 `col-fit-first`/`row-fit-first`를 200px로 기록했다. 이 해석은 CSS Grid의 gutter overlap 규칙을 빠뜨렸다. CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2는 빈 반복 track을 collapse하고 양쪽 gutter를 collapse하도록 하며, §11.1은 collapsed track의 두 gutter가 정확히 겹쳐 start/end edge가 일치한다고 명시한다. 연속 빈 track들의 gutter는 하나의 shared gutter로 합쳐진다.

고정 input은 content axis 240px, percentage prefix 10%=24px, suffix 5%=12px, gap 4px, `minmax(40px, 1fr)` 네 회다. 첫 repeat만 점유되므로 뒤 세 track은 0px이다. Prefix와 첫 repeat 사이 일반 gutter 한 개와 연속 빈 repeat run을 대표하는 shared gutter 한 개가 남는다. 따라서 첫 repeat used size는 `240 - 24 - 12 - 2×4 = 196px`이다.

## Chrome 156 관측과 geometry

Agent Browser가 저장한 Chrome 156.0.8078.48 CSSOM은 columns와 rows에서 모두 `[24px, 196px, 0px, 0px, 0px, 12px]`를 반환한다. 시작 정렬 item frame은 columns `(38,10,2,2)`, rows `(10,38,2,2)`다. 브라우저 DOM에서만 적용한 end-alignment 진단은 item origin x/y=232px로 관측되며, 보정된 규범 계산도 232px을 낸다.

Suffix item probe는 x/y=238px이다. 계산식 `10 + 24 + 4 + 196 + 4`와 일치한다. 이는 두 gutter의 각 위치를 직접 측정한 별도 증거로 사용하지 않으며, CSSOM track list와 규격의 overlapping-gutter 규칙을 함께 해석한다. raw Chrome capture는 [observation JSON](./chrome156-observation.json), end-alignment는 [probe JSON](./auto-fit-end-alignment-probe-2026-10-24.json)에 보존했다.

동일 Agent Browser / Chrome 156에서 기존 고정 길이 auto-fit regression matrix도 다시 조회했다. 200×60 columns 장면과 50×200 rows 장면 모두 occupied repeat track은 154px이며, occupied track이 가운데나 끝에 있어도 used-size는 같다. 빈 repeat run 앞에 남는 shared 8px gutter 때문에 repeat item과 suffix item은 각각 28px와 190px에서 시작한다. span-two 장면은 `[73px, 73px]`이고 suffix-only 장면도 item origin 28px이다. 기존 Rust oracle의 162px·170px·77px과 20px 기대값은 shared gutter를 누락했으므로 [고정 neighbor 재대조 JSON](./auto-fit-fixed-neighbor-chrome156-agent-browser-2026-10-24.json)으로 교체했다.

## Taffy 구현 대조

- 기존 vendored Taffy는 빈 auto-fit track을 초기화할 때 그 track의 앞·뒤 gutter를 모두 0 sizing function으로 바꿨다. 이는 연속 gutter가 한 위치로 겹친다는 규칙을 표현하지 못해 shared gutter 하나를 누락했다.
- 보정 구현은 빈 repeat track과 그 다음 gutter를 collapse하고, collapsed run 직전 gutter를 shared gap으로 남긴다. 연속 빈 track에서는 뒤이어 생성되는 gutter만 0이 되므로 하나의 collapsed-run boundary gap이 남는다. auto-repeat 자체가 grid 끝에서 완전히 비는 경우에는 기존 끝 경계 처리를 유지한다.
- 보정 뒤 기존 percentage-neighbor 13-scene matrix를 Android API 37 ARM64 AVD에서 다시 실행해 axis track·node frame·trace·draw/present 13/13을 확인했다. [후속 Android evidence](../c11-4-auto-repeat-percentage-neighbors-post-gutter-fix-pass03-2026-10-24/README.md). 이 기존 percentage-only matrix의 post-fix iOS 실행과 PixelCopy 표본은 없다. 신규 typed `calc()` candidate의 Android/iOS auto-fit empty/single occupancy 및 columns/rows 회귀는 [별도 14-scene evidence](../c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md)에 기록했다. 과거 PR #580 runtime logs는 이전 Taffy source 결과다.

## 범위와 한계

이 정정은 fixed fixture의 단일 auto-repeat와 해당 gutter sizing만 다룬다. 다른 engine, Chrome 156 바이너리의 내부 source revision, 모든 auto-placement 조합 또는 WPT 적합성을 추론하지 않는다. 규격 원문: [CSS Grid 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) 및 [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters).
