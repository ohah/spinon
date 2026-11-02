# C11.1 Root HTML `fr` implementation PR diff 실패 관점 검토 30개

**검토 범위:** Root HTML direct `fr` projection·axis gate, Taffy flex-factor accumulation, runtime positive/rejection matrix, 명세·상태 문서, Android/iOS simulator evidence. 구현/runtime 20개 검토는 [별도 문서](./implementation-runtime-review-20.md)에 기록했다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 일반 Grid fixture가 새 Root HTML admission 제한을 받을 수 있음 | **PASS** — fraction-axis gate와 track validator는 `RuntimeRootHtmlTextPaintV1` 분기에서만 호출된다. |
| 2 | 다른 Root HTML profile도 `fr` admission이 열릴 수 있음 | **PASS** — 다른 profile 경로와 dispatch 조건은 변경하지 않았다. |
| 3 | 새 공개 CSS 지원 또는 ABI로 오해될 수 있음 | **PASS** — 공개 CSS parser·FFI·header·platform bridge는 변경하지 않고 기존 default Root HTML route만 쓴다. |
| 4 | computed track 순서가 변환 중 바뀔 수 있음 | **PASS** — direct `LengthPx`/`Fraction` values를 순회 순서대로 projection하고 typed fraction을 유지하는 projection/runtime assertion이 있다. |
| 5 | 음수 `fr`가 flex sizing에 전달될 수 있음 | **PASS** — Root HTML validator가 non-negative factor만 허용한다. |
| 6 | NaN 또는 무한 factor가 denominator를 오염시킬 수 있음 | **PASS** — factor가 finite인지 먼저 검사하고 projection unit tests가 NaN·양/음 infinity를 확인한다. |
| 7 | direct `fr` 허용이 `minmax()`까지 넓어질 수 있음 | **PASS** — direct enum variant 외 track shape는 거부하고 runtime cascade/property diagnostic을 검사한다. |
| 8 | auto-repeat 또는 authored line names가 implicit metadata를 바꿀 수 있음 | **PASS** — 기존 제한 검사와 Root HTML repeat/line-name 회귀는 유지한다. |
| 9 | column factor가 indefinite 또는 percentage width를 기준으로 계산될 수 있음 | **PASS** — columns `fr`는 exact computed element의 finite non-negative `LengthPx` width를 요구한다. |
| 10 | row factor가 indefinite 또는 percentage height를 기준으로 계산될 수 있음 | **PASS** — rows `fr`는 같은 element의 finite non-negative `LengthPx` height를 요구하며 fixed row positive와 indefinite negative가 있다. |
| 11 | 다른 element나 revision의 width/height를 basis로 섞을 수 있음 | **PASS** — validator는 현재 computed grid element만 읽고 runtime positive는 cascade/layout key 동일성을 검사한다. |
| 12 | 계산으로 정규화된 `calc()` 크기를 ordinary definite px로 오인할 수 있음 | **PASS** — `layout_math_values`에 해당 size property가 있으면 computed length가 있어도 거부한다. |
| 13 | 해당 축 min-size가 flex 결과를 바꿀 수 있음 | **PASS** — computed min-size는 `auto`만 허용하며 negative control이 `min-width`를 검증한다. |
| 14 | 해당 축 max-size가 배분 결과를 clamp할 수 있음 | **PASS** — computed max-size는 `none`만 허용하며 negative control이 `max-width`를 검증한다. |
| 15 | aspect ratio가 반대 축에서 basis를 유도할 수 있음 | **PASS** — 어느 축이든 `fr`가 있을 때 `aspect-ratio`를 거부하고 runtime error 경계를 확인한다. |
| 16 | area-backed named placement가 fraction admission을 우회할 수 있음 | **PASS** — named-area 검증은 fixed px로 남기고 area + `fr` scene은 거부한다. |
| 17 | fixed track gap이 free space에서 누락되거나 중복 차감될 수 있음 | **PASS** — 기존 `LayoutGap` projection 경로를 재사용하고 two-axis gap 있는 scene에서 marker origins를 검사한다. |
| 18 | marker의 authored 크기가 `fr` 결과를 직접 강제할 수 있음 | **PASS** — positive controls의 고정 marker는 각 target track보다 작고 root bounds·gap·뒤 marker origin을 검사한다. |
| 19 | `1fr 2fr` ratio가 순서 또는 gap 계산에 따라 틀릴 수 있음 | **PASS** — 130px container와 10px gap에서 marker origins `0`, `50`을 runtime이 검사한다. |
| 20 | `2fr 1fr` reverse distribution이 틀릴 수 있음 | **PASS** — second marker origin `90`을 positive matrix가 검사한다. |
| 21 | fixed px와 flex tracks가 혼합될 때 offset이 어긋날 수 있음 | **PASS** — `30px 1fr 2fr` case가 세 marker origin을 확인한다. |
| 22 | row axis 구현이 column axis 조건을 잘못 재사용할 수 있음 | **PASS** — fixed-height row `1fr 2fr` case는 y origins `0`, `50`을 확인하고 indefinite row basis는 거부한다. |
| 23 | flex-factor sum이 1보다 작을 때 표준 하한을 놓칠 수 있음 | **PASS** — `0.25fr 0.25fr` Taffy unit/runtime case가 30px track 결과를 검사한다. |
| 24 | `0fr` item frame이 paint list에 없으면 검사가 거짓 통과할 수 있음 | **PASS** — computed background로 element를 찾고 layout snapshot의 node frame을 검사한다. |
| 25 | f32 factor sum overflow가 큰 유한 비율을 0으로 만들 수 있음 | **PASS** — factor sum과 used space를 f64로 누산하고 세/네 `1e38fr` 결과를 Taffy와 Root HTML runtime에서 검사한다. |
| 26 | 큰 값 전용 수정이 통상 비율·zero·subunit behavior를 바꿀 수 있음 | **PASS** — 해당 Taffy positive cases와 전체 vendored Taffy 176 tests가 통과했다. |
| 27 | node id·class 이름·child index를 사용해 source order를 감출 수 있음 | **PASS** — semantic class와 DOM order를 바꾼 positive control이 현재 order의 marker origin을 확인한다. |
| 28 | sim app이 Rust test와 다른 HTML source를 실행할 수 있음 | **PASS** — APK와 iOS bundle asset hash 및 두 runtime source 로그가 shared fixture SHA-256과 같다. 빌드 후 원래 platform assets도 복원해 hash를 비교했다. |
| 29 | platform app smoke를 geometry 또는 pixel parity로 과장할 수 있음 | **PASS** — 문서와 PR evidence는 layout-ready/draw 및 screenshot을 smoke evidence로만 분류하고 PixelCopy/Metal readback을 미실행으로 명시한다. |
| 30 | slice 결과가 C11/C11.1 또는 공식 완료 수를 잘못 올릴 수 있음 | **PASS** — `spec/STATUS.md`는 C11/C11.1 미완료와 `72/213`을 유지하고, `0059`는 bounded implementation candidate 및 비지원 범위를 기술한다. |

**남은 경계:** intrinsic flex sizing, `minmax()`의 flex maximum, named-area + `fr`, implicit layout, 일반 CSS admission, WPT·전체 C11, Android hardware renderer, PixelCopy/Metal drawable readback, iOS compositor 이후 parity는 검증하거나 완료로 주장하지 않는다. PR #1015의 GitHub status check rollup은 비어 있다. CI 통과로 주장하지 않는다.
