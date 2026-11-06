# C11.4 parsed Root HTML normal-whitespace ASCII word intrinsic sizing

**상태:** 제한 구현 PR [#1208](https://github.com/ohah/spinon/pull/1208)이 2026-11-06T05:32:14Z Rebase and merge로 병합됐다(merge SHA `3f022b175dfb689cf721ef407a0d86b05ea96db1`). C11.4·C11 checkbox와 공식 완료 78/214는 변경하지 않는다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 normal-whitespace slice](./c11-root-html-intrinsic-normal-whitespace-multiple-text.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 156 관측·독립 기대값·fixture hash](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/README.md) · [runtime sample 지점의 Agent Browser Chrome 관측](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06/chrome/README.md)

**계획 실패 관점 검토:** [서로 다른 20개](./c11-root-html-intrinsic-letter-words-plan-review-20.md) · **계획 PR diff 검토:** [서로 다른 30개](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/plan-pr-diff-review-30.md)

## 결손과 목표

PR #1205는 기본 parsed Root HTML의 `grid-auto-rows:min-content` 경로에서 `white-space:normal` 직접 text source를 연결했지만, token을 단독 `A`로 제한한다. `AA`, `AB`, `BBB`와 같은 일반 ASCII 알파벳 단어는 fail-closed되므로 가장 긴 unbreakable word가 min-content track과 implicit row 높이에 미치는 영향을 기본 Root HTML 앱 경로로 확인할 수 없다.

이 slice는 기존 `RuntimeRootHtmlTextPaintV1`과 intrinsic implicit-row 경로의 text-source allowlist를 하나 이상의 ASCII alphabetic run으로 확장한다. token 사이 및 source 앞뒤에는 PR #1205와 같은 CSS `white-space:normal` collapsible ASCII whitespace를 허용한다. 원문 string과 Text node identity는 유지하고 기존 shaping, line breaking, track sizing, paint 경로를 재사용한다.

## 규범 기준과 독립 oracle

- [CSS Grid Layout Module Level 1, 2025 CRD §11.4–11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)은 min-content contribution과 track sizing 단계의 기준이다.
- [CSS Text Module Level 4 §4.1·§4.3](https://drafts.csswg.org/css-text-4/#white-space-processing)은 `white-space:normal`에서 collapsible whitespace와 normal wrapping의 기준이다.
- Pinned Chrome 156 관측은 [사전 비교 자료](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/README.md)에 둔다. 8개 장면의 used track/row는 독립 계산과 일치하지만 Chrome observation 자체를 규범 기대값으로 취급하지 않는다.
- 독립 min-content 계산은 normal wrapping 기회 사이의 ASCII letters token 중 가장 넓은 token의 advance다. `min-content` track은 그 contribution을 사용한다. `auto` track은 `max(min-content, min(40px, max-content))`를 사용한다. 네 `auto` scene의 max-content contribution은 모두 40px 이상이므로 40px content box를 채우고, unbreakable word의 automatic minimum이 더 크면 track이 바깥으로 확장한다. 예상 값은 token, generic monospace advance, 1/64 CSS px layout quantization으로 [`derive_expected.py`](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/derive_expected.py)가 계산한다.

## 제한 범위와 실패 동작

1. Text token은 `[A-Za-z]+`만 허용한다. token 사이는 U+0020 SPACE, U+0009 TAB, U+000A LF, U+000D CR만 허용하며 입력 source를 trim·normalize·rewrite하지 않는다.
2. 기존 visible direct-child HTML Grid, definite 40px container, `font-family:monospace`, `font-size:10px`, `line-height:10px`, `lang="en"`, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`, `grid-auto-rows:min-content` profile에만 적용한다.
3. Unicode, digits, punctuation, NBSP, U+000B VERTICAL TAB, U+000C FORM FEED, empty/whitespace-only text, 다른 font/style/owner, hidden/detached node, unsupported track grammar는 계속 fail-closed한다.
4. 허용 source는 원문 text와 node identity를 보존한다. 구조·revision·style·track 판정 실패 시 이전 revision의 frame이나 partial draw를 새 결과로 게시하지 않는다.
5. 일반 CSS admission, arbitrary font shaping, C15 text layout, WPT runner, named-area/placement 조합, 전체 C11.4는 범위 밖이다.

## 고정 장면과 acceptance

Precomparison의 10개 고정 장면을 전부 재생한다: single-letter control, `AA A A A`, `AB BBB A`, `aa bb ccc`를 각각 `min-content` 및 definite `auto` column에서 실행하고, `AB BBB A`의 두 column mode에서 short/long item의 reverse DOM order를 추가 확인한다. 별도로 default Root HTML 앱 fixture에서 `AAAAAAAA`를 실행해 40px container 안에서 unbreakable word의 48.171875px automatic minimum track이 확장되는지 확인한다.

- Style-to-layout와 RuntimeSession 검사는 source gate, exact token/source preservation, document/style/layout revision, node/tree identity, track sizing trace, forward/reverse item source order, item line count/frame과 rejection atomicity를 검사한다.
- Style-to-layout과 RuntimeSession의 고정 10-scene Rust matrix에서 독립 기대값에 대한 geometry·track trace·source identity를 검사한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서는 기본 Root HTML app route로 DOM source를 확인하고 layout ready, draw/present 및 두 text item을 확인한다. 모바일 화면 로그는 개별 item frame dump를 제공하지 않으므로 그 로그만으로 full geometry matrix 통과를 주장하지 않는다.
- Grid 40px border/content right edge 바깥의 item overflow를 CSS `(42px,19px)`에서 Android PixelCopy와 iOS Simulator Metal drawable로 표본화한다. Agent Browser Chrome 157에서 text row는 `y=10..20px`이고 `(42px,15px)`은 antialiased glyph와 겹쳐 item 배경색을 반환하지 않았다. `(42px,19px)`은 같은 row의 하단 내부 지점이며 40px Grid 바깥에서 text-item background `[91,168,245,255]`를 반환했다. 모바일 표본은 같은 RGBA를 반환했고, 매핑된 Android surface `(111,51)`, iOS drawable `(127,58)`에서 exact였다. 이는 해당 좌표의 overflow paint만 확인하며 전체 item width나 viewport clipping을 주장하지 않는다. 좌표 mapping과 app source/binary hash를 evidence에 기록한다.
- Rust targeted suites, `cargo fmt --all -- --check`, changed crates의 Clippy, Android/iOS Simulator builds와 default/fixture artifact boundary를 기록한다. 기존 unrelated baseline failure는 변경 유발 실패와 분리한다.
- 구현 뒤 새 [implementation/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06/implementation-runtime-review-20.md)를 기록했고, 실제 구현 PR diff 검토 30개는 PR 생성 후 별도로 수행한다. 계획 PR은 별도 계획 실패 관점 20개 및 실제 계획 PR diff 30개 검토를 통과해야 한다.

이 제한 slice를 C11.4/C11 완료로 표시하지 않는다. 숫자 내부 계약 버전 `0.1.0`을 유지한다.

## 실행 결과

[시뮬레이터·artifact 근거](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-runtime-2026-11-06/README.md)에서 Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 표본 각 1/1 exact를 확인했다. Android 전용 adapter는 c11-grid-metal-surface-pixels fixture source set에만 두었고, Android/iOS 기본 pixel-readback flag 0 산출물에서 작업별 adapter·scene marker가 빠지는지 feature-on positive control과 함께 검사했다. C11.4/C11 체크박스와 공식 완료 78/214는 유지한다.
