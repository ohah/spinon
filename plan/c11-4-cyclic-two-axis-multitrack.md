# C11.4 · 동시 cyclic percentage 다중 track sizing 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 네 fixed scene 구현과 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 확인을 마쳤다. 구현 PR은 이 slice와 근거를 연결한다. 이 작업은 전체 C11.4 지원이나 공식 완료 수 `72/213`을 바꾸지 않는다.

## 목적과 제한 범위

앞선 cyclic percentage 세 slice는 각각 단일 percentage track, `minmax(0px, 50%)`, `fit-content(50%)`를 한 축에서 확인했다. 이번 계획은 같은 Grid에서 columns와 rows 모두 cyclic percentage를 사용하고, 두 축에 각각 두 percentage track과 하나의 fixed-length track을 함께 둔다. 각 축만 cyclic인 control과 두 축 definite control을 포함해 동시 sizing, 다중 track 분배, definite percentage 경로를 구분한다.

허용 입력은 각 root Grid와 direct child 두 개(첫 cell의 Ahem text, 마지막 cell의 고정 크기 overflow box), explicit 3×3 tracks, zero gap, zero border/padding, 네 scene ID다. 신규 공개 CSS/API admission은 없고 기존 C11.4 opt-in fixture-test 경로만 확장한다. 각 축의 track list는 `50% 25% 20px` 또는 고정 길이 세 개로 고정한다. root의 `overflow` 초기값은 `visible`이며 실제 border stroke는 이번 범위가 아니다.

## 규범 기반 독립 기대값

[CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 track percentage를 해당 Grid container의 inner size에 연결한다. [§11 Grid Layout Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)은 cyclic percentage를 grid container의 intrinsic size를 구하는 동안 `auto`로 취급한 뒤, 그 결과 size를 기준으로 final track sizing에서 percentage를 다시 해석한다. [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)은 columns를 먼저 계산하고, 그 column sizes를 이용해 row contribution이 필요한 item을 측정하도록 한다.

Ahem 10px/10px에서 `aa`의 min-content와 max-content inline size는 모두 20px이고 한 line의 block size는 10px이다. border, padding, gap이 모두 0이므로 inner size와 root content size가 같다. intrinsic pass에서 두 percentage track은 각 축의 content contribution을 받을 수 있는 `auto`처럼 동작한다. 첫 cell의 text가 첫 track에 20×10px을 제공하고, 세 번째 track은 고정 20×10px이므로 `width/height:max-content` root basis는 40×20px이다. 세 번째 child의 고정 20×10px contribution은 그 fixed track을 넘기지 않는다.

최종 sizing에서 inline basis 40px의 50%와 25%는 20px과 10px이다. block basis 20px의 50%와 25%는 10px과 5px이다. 따라서 두 축이 cyclic인 장면의 used tracks 합은 50×25px이고 root는 40×20px로 유지된다. 마지막 child는 세 번째 track 시작점 `(30, 15)`에서 `20×10px`을 그려 root의 오른쪽으로 10px, 아래로 5px 넘친다. Root overflow는 기본 `visible`이다. 이 계산은 Chrome 관찰에서 역산하지 않았다.

| Scene | Root sizing | Columns | Rows | Root frame | Text item | Overflow item, root 기준 |
| --- | --- | --- | --- | --- | --- | --- |
| `cyclic-percent-two-axis-multitrack` | `max-content × max-content` | `[20, 10, 20]px` | `[10, 5, 10]px` | `40×20px` | `(0,0) 20×10px` | `(30,15) 20×10px` |
| `cyclic-percent-columns-multitrack` | `max-content × 20px` | `[20, 10, 20]px` | `[10, 5, 5]px` | `40×20px` | `(0,0) 20×10px` | `(30,15) 20×10px` |
| `cyclic-percent-rows-multitrack` | `40px × max-content` | `[20, 10, 10]px` | `[10, 5, 10]px` | `40×20px` | `(0,0) 20×10px` | `(30,15) 20×10px` |
| `definite-percent-two-axis-multitrack` | `40px × 20px` | `[20, 10, 20]px` | `[10, 5, 10]px` | `40×20px` | `(0,0) 20×10px` | `(30,15) 20×10px` |

`cyclic-percent-multitrack-columns` keeps only the inline axis intrinsic; rows are fixed. `cyclic-percent-multitrack-rows` keeps only the block axis intrinsic; columns are fixed. The final row sizing still uses the columns resolved earlier. The definite control resolves both percentages directly against fixed 40×20px inner sizes. The text child provides the intrinsic basis; the third-cell child makes track placement and visible overflow measurable. Root absolute page `y` is not compared; node positions are root-relative.

## 사전 관찰과 계획 검토

- 입력: [고정 HTML fixture](../tests/fixtures/css/c11/grid-cyclic-percent-two-axis-multitrack-c114-precomparison.html).
- 기준: Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.0.0`, viewport `1280×633`, DPR 1. `document.fonts.status=loaded`와 `document.fonts.check('10px Ahem')=true`를 확인했다.
- 독립 계산, 네 장면의 Chrome CSSOM/DOMRect 관찰, screenshot 및 입력 hash는 [사전 비교 evidence](../spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-precomparison-2026-10-22/README.md)에 있다. Chrome 156은 단일 보조 oracle이며 155 baseline, 규범 명세 또는 다른 UA parity로 확대하지 않는다.
- 계획 자체의 독립 실패 관점 20개 검토는 [계획 검토](./c11-4-cyclic-two-axis-multitrack-plan-review-20.md)에 둔다. 계획 검토는 구현 뒤 runtime 검토와 재사용하지 않는다.

## 구현 및 판정 절차

1. 기존 opt-in C11.4 fixture-test 경로에서 위 네 scene ID만 허용한다. runtime adapter는 DOM shape, source text, computed style과 typed track inputs가 고정 case와 맞지 않으면 fixture input을 만들지 않는다.
2. Rust projection/layout에서 네 장면의 Columns/Rows track 개수·used sizes·root/text/overflow node frame과 stage trace를 독립 기대값에 대조한다. 두 cyclic 축이 같은 root의 containing size를 서로 잘못 참조하지 않는지 확인한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 장면 전부 실행했다. 각 scene의 Columns/Rows trace, root/text/overflow frame, draw/present를 확인했다. Android PixelCopy와 iOS Metal drawable(제출 전)의 다섯 표본은 빨강 text-item 배경, grid 안 overflow, root의 오른쪽·아래·두 경계 바깥에 있는 초록 overflow를 검사한다. 표본 결과는 각 플랫폼 20/20 exact다. 이는 글리프 paint 증거가 아니며 Simulator 결과를 실기기로 확대하지 않는다.
4. 기본 `spinon-ffi` debug staticlib에서 새 scene ID가 없는 것과 `c04-runtime-gpu-fixture-test` positive-control staticlib에 포함된 것을 `strings`로 확인했다. 이는 release/app bundle 전반의 바이너리 크기 검증은 아니다.
5. 관련 Rust 단위 테스트 3개, JavaScript 구문 검사와 fixture module-boundary 검사, `cargo fmt --all -- --check`, fixture-test feature의 `spinon-ffi` all-target Clippy, Android/iOS Simulator app build가 통과했다. full-workspace test/Clippy는 실행하지 않았다. 실행 근거와 SHA-256은 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/README.md)에 기록한다.
6. 계획 검토와 다른 관점 20개로 구현/runtime을 검토했다. PR 변경분은 별도 관점 20개로 검토해 결과를 기록한다. 발견 사항은 수정 후 해당 검토를 반복한다.
7. PR은 fixed fixture, 두 축 동시 sizing, root 밖 item pixel evidence와 한계를 설명한다. C11.4 공식 check와 `72/213`은 그대로 둔다. 병합 직전 현재 required checks와 merge state를 조회하고 `Rebase and merge`를 사용한다.

## 범위 밖

Non-zero gap/border/padding, 다른 percentage 값이나 `calc()`, `minmax()`/`fit-content()`의 다중 track 조합, auto-repeat, implicit tracks, span contribution, nested/anonymous items, vertical writing mode, parent clipping variants, CSS author admission, WPT, Android 실기기, hardware GPU, iOS compositor 이후 pixel, glyph raster equality 및 C11.4 전체 완료는 포함하지 않는다. Chrome screenshot의 overflow는 precomparison 결과일 뿐 Spinon 결과가 아니다.

## 구현 결과 (2026-10-22)

네 scene 모두 AVD와 iOS Simulator에서 `40×20px` root, `(0,0) 20×10px` text item, `(30,15) 20×10px` overflow item을 관찰했다. scene별 최종 Columns/Rows는 위 표와 일치했고, Android draw/present와 PixelCopy 표본, iOS draw/present와 Metal drawable 표본이 모두 맞았다. Rust trace 테스트도 최종 세 track의 base size를 대조한다. 앱 screenshot은 box/background를 보여주며 Ahem glyph paint는 하지 않는다.

최종 screenshot, PixelCopy/Metal 표본, 앱 hash, 그리고 commit으로 고정된 source revision은 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-two-axis-multitrack-runtime-2026-10-22/README.md)에 기록한다.
