# C11.4 · WPT `grid-auto-fit-columns-001.html` 전체 고정 장면 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태 · 2026-10-27:** 고정 WPT 원문의 Chrome 156 harness 35/35 결과와 root/item 관측, WPT-authored frame 기대값을 [사전 비교 근거](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/README.md)에 보존했다. 이는 Chrome 결과이며 Spinon 구현 결과가 아니다. C11.4 상위 체크와 공식 `72/213`은 그대로 미완료로 둔다.

## 확인한 결손

상위 WPT inventory에는 [`grid-auto-fit-columns-001.html`](../spec/internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.md)이 auto-fit 후보로 남아 있다. 기존 고정 fixture들은 repeat count, named line, missing-name fallback, fixed-repeat group, percentage maximum, flex track, nonzero gap을 분리된 사례로 검증했다. 이 개별 사례 통과만으로는 WPT 원문에서 함께 나타나는 fixed prefix/suffix, 반복 line name, negative/named placement, implicit column과 gap/collapse 조합을 검증하지 않는다.

이번 구현은 간단한 일부 사례가 아니라 pinned WPT 문서의 **35개 `.grid` 검사와 37개 direct item 전부**를 같은 opt-in runtime 경로에 연결한다. Chrome 35/35는 기준 브라우저 결과일 뿐 Spinon pass가 아니다. 일반 author CSS admission, WPT runner 전체 통과, C11.4 완료는 범위 밖이다.

## 입력과 oracle

- 기준 파일은 WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/grid-auto-fit-columns-001.html`이다. 원본 및 helper의 SHA-256은 [source manifest](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/source-manifest.json)에 고정했다.
- [Chrome observation](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/chrome-observation.json)은 viewport `1280×633` CSS px, DPR 1, Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`에서 35개 root와 37개 item을 관찰한 자료다. 전체 화면은 [Chrome WPT 결과](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/chrome-wpt-full-35-results.png)에 보존했다.
- 독립 geometry 입력은 [expected.json](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/expected.json)이다. WPT source의 `data-offset-*`, `data-expected-*`를 추출하며 Chrome 결과에서 역산하지 않는다. [재생성 스크립트](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/derive_expected.py)는 정확히 35 root·37 item이 아니면 실패한다.
- Repeat count oracle은 [case별 독립 기대값](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/repeat-count-expected.json)이다. [재생성 스크립트](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/derive_repeat_expectations.py)는 pinned WPT의 track-list declarations, root width, padding, gap 및 helper CSS를 hash/내용으로 대조한 뒤 CSS Grid §7.2.3.2의 max/min floor 규칙, fixed prefix/suffix, pattern 내부·반복 사이 gutter를 분수 산술로 계산한다. 반복 수는 WPT index 1–35 순서로 `[6, 3, 6, 6, 6, 6, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 2, 1, 1, 1, 5, 5, 5, 1, 1, 3, 3, 3, 1, 1, 1, 2, 1, 3, 3]`이다.
- WPT helper는 item의 offset/size 오차가 1 CSS px 이상이면 실패시킨다. 구현 검증은 정수 expected frame에 더 엄격한 `0.02 CSS px` 허용 오차를 적용하고, repeat count·line occurrence·collapse·scene 수·surface RGBA는 정확히 대조한다.
- Chrome CSSOM의 used `gridTemplateColumns`는 browser observation으로 보존하며 규범 기대값과 혼동하지 않는다. case 7, 8, 11, 14, 19의 CSSOM에는 overflow placement가 만든 implicit `157px` column이 들어가므로 explicit repeated-track count와 분리한다. 앱 matcher는 WPT frames와 별도로 used tracks, 독립 repeat count, expanded line-name occurrence, stage arrays와 gutter/collapse flags를 검사한다.
- Root CSS `width`는 200px content box이며 border 2px는 바깥에 그린다. case 2의 좌우 padding 13px은 content box의 원점을 이동시키고 외곽 border box를 230px로 넓히지만 200px content 폭을 174px로 줄이지 않는다. `30%` track basis와 repeat count는 200px이고 item offset 13px·폭 180px은 Chrome 관측 및 WPT 기대값으로 따로 고정한다. gap scenes는 20px column gap을 쓰며, 모든 case에서 `grid-auto-columns:157px`, `grid-auto-rows:25px`, `justify-content:start`, root/item defaults를 고정한다. case 32의 `2em`은 이 WPT browser 환경의 16px inherited font basis에 대해 used 32px로 관측했다.
- 규범 출처는 CSS Grid 2025 CRD §§7.2.3.1–7.2.4, §§8.3–8.5, §11.7이다. definite repeat count와 gap, max/min floor, item placement 뒤 auto-fit empty 판정, occupied span, 인접 gutter collapse, repeated line-name merge, numeric/named placement, flex leftover distribution을 별도 invariant로 기록한다.

## 전체 35-scene 범위

| WPT index | Scene family | 확인할 조합 |
| ---: | --- | --- |
| 1–8 | `gridOnlyAutoRepeat`, `gridPercentAutoRepeat`, fixed `minmax()` | definite px/percentage repeat, max-below-min floor, spanning item, named line, gap, numeric/named placement, empty-track collapse |
| 9–14 | `gridAutoRepeatAndFixedBefore` | px/percentage prefix, auto-fit named pattern, forward/backward span, missing name, implicit auto-column, gap 및 overflow |
| 15–20 | `gridAutoRepeatAndFixedAfter` | auto-fit prefix, fixed/intrinsic suffix, adjacent line-name join, named placement, gap, implicit auto-column 및 overflow |
| 21–25 | `gridAutoRepeatAndFixed` | integer fixed-repeat + auto-fit + percentage suffix, repeat 경계 이름, named placement, gap 및 collapse |
| 26–31 | `gridMultipleNames` | 중복 이름, percentage fixed neighbor, positive/negative named·numeric occurrence, span, gap 및 overflow |
| 32–33 | `gridMultipleTracks` | 두 track auto-repeat, `em`+percentage pattern, line-name join, fixed flex suffix, gap 및 collapse/span |
| 34–35 | `gridMinMaxFixedFlex` | auto-fit `minmax(px,1fr)`, gap, auto-placement된 한두 item, 빈 반복 track |

35개 root와 37개 item을 모두 실행한다. root ID/class/style과 DOM order가 scene ID에 일대일로 대응해야 하며 누락, 중복, 미등록 source는 layout 전에 거부한다. Item이 200px content edge를 넘는 WPT 장면은 7, 8, 11, 14, 19, 31이다. 여섯 장면 모두 layout matrix와 표면 검사에 포함한다.

## 구현 범위와 안전 경계

1. `c04-runtime-gpu-fixture-test` opt-in 아래 정확히 35개 scene만 받는 adapter를 추가한다. WPT root 선언, 자식 수/순서, computed typed track list, placement, line-name groups, item 색 및 `grid-auto-*` sizing을 보존한다. item/root ID를 CSS 해석 키로 쓰지 않는다.
2. C11.4 dispatcher와 두 native demo host는 이미 크므로 WPT scene table·source builder·matcher는 책임에 맞는 전용 fixture module/helper로 추가한다. FFI registry, Android/iOS opt-in allowlist와 strict matcher를 확장하고 기존 C11.4 scene/fallback을 회귀 대조군으로 둔다. 미등록 scene, 바뀐 topology/style, 미지원 selector 또는 다른 profile은 partial frame/trace/draw 전에 거부한다.
3. 전체 35 scene에서 revision, root와 모든 child frame, WPT offset/size, final used columns/rows, explicit/repeated/implicit track 수, repeat mode/count, expanded line-name occurrence, placement, 각 stage의 base/growth arrays와 gutter/collapse를 검사한다. `auto-fit` 빈 여부는 placement 후 판정하고 span으로 점유된 반복 track은 유지한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 source commit의 35/35를 각각 실행한다. 여섯 overflow scene 각각에서 Android PixelCopy와 iOS Simulator Metal drawable을 읽어 border, border 바깥 item 내부, item 끝 직후 background, 별도 control을 표본화한다. 좌표 변환·기대 RGBA·manifest를 남긴다. compositor 이후 표면 결과라고 확대 해석하지 않는다.
5. 실행 전 simulator/AVD boot 상태를 기록한다. 이미 boot된 장치는 유지하고 이번 실행에서 직접 시작한 장치와 앱 프로세스만 종료한다. 원본 log, source/build/install hash, scene별 frame/track/trace/draw/present, representative screenshots를 저장한다.
6. 기본 product와 opt-in static FFI archive를 비교해 fixture source/symbol/string의 제외와 positive control 포함을 확인한다. 이를 전체 APK/IPA fixture exclusion으로 일반화하지 않는다.
7. focused C11.4 Rust/FFI와 fixture-boundary checks, `cargo fmt --all -- --check`, workspace Clippy `-D warnings`, Android/iOS app build를 기록한다. baseline 실패와 변경 유발 실패를 분리한다.
8. 구현 후 새 20관점 implementation/runtime hostile review와 별도 20관점 final PR diff review를 수행한다. 발견 사항은 수정 후 영향받은 관점에서 다시 확인한다. 계획 PR도 독립 20관점 계획 검토와 20관점 계획 PR diff 검토를 거친다.
9. 구현 PR에는 Android/iOS screenshot을 첨부하고 Agent Browser로 이미지 로드를 확인한다. 확인 후 Agent Browser 세션과 프로세스를 종료한다. Terminal Browser는 사용하지 않는다. 최종 base·PR·check 상태를 확인한 뒤 Rebase and merge하고, merge source에 맞춰 STATUS·공개 roadmap·상위 계획·Tailnet preview를 동기화한다.

## 완료 판정과 제외

같은 final source commit에서 Android/iOS 모두 35 WPT assertion·37 item geometry, track/repeat/line/collapse matcher, 여섯 overflow scene의 두 플랫폼 surface matrix, rejection controls, artifact/build gates, screenshot links와 별도 20관점 검토를 통과해야 한다. WPT runner pass, Chrome 관측, 코드 검사 또는 geometry-only 결과로 필수 runtime/readback gate를 대신하지 않는다.

이 결과는 고정 WPT 입력의 opt-in app fixture다. public CSS admission, Spinon WPT suite 통과, 다른 auto-fit input, physical device/hardware GPU, iOS compositor 이후 결과, Stage B 전체 또는 C11.4/C11 완료를 뜻하지 않는다. 공식 C11.4 checkbox와 `72/213`은 유지한다.

## 검토 기록

- [계획 독립 실패 관점 20개](./c11-4-grid-auto-fit-columns-wpt-plan-review-20.md).
- [Chrome/WPT 사전 비교와 입력 hash](../spec/internal/evidence/c11-4-grid-auto-fit-columns-precomparison-2026-10-27/README.md).
