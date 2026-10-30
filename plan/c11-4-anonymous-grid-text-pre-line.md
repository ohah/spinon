# C11.4 anonymous Grid text `white-space:pre-line` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** Chrome 156 Agent Browser 사전 관찰과 CSS Text/Grid/Box Sizing 별도 기대값 계산을 기록했다. 계획 실패 관점 20개 검토 후 계획 PR [#866](https://github.com/ohah/spinon/pull/866)은 2026-10-30에 병합됐다. 구현 후보와 Android/iOS 시뮬레이터 근거를 담은 구현 PR [#867](https://github.com/ohah/spinon/pull/867)은 2026-10-31에 열었고, 별도 PR diff 실패 관점 30개 검토를 마쳤다. required checks와 병합 상태는 아직 확인·완료하지 않았다. 이 계획은 기존 opt-in C11.4 anonymous Grid text fixture 경로에서 `white-space:pre-line` 한 값을 추가한다. C11.4/C11 checkbox와 공식 완료 수 `72/213`은 바꾸지 않는다. [사전 비교와 입력 hash](../spec/internal/evidence/c11-4-anonymous-text-pre-line-precomparison-2026-10-31/README.md) · [구현·시뮬레이터 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-line-runtime-2026-10-31/README.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-line-runtime-2026-10-31/pr-diff-review-30.md)

## 목적과 경계

CSS Text Level 3 §3에 따라 `white-space:pre-line`은 공백을 접으면서 source segment break를 forced line break로 보존한다. `Normal | NoWrap`만 받던 C11.4 anonymous Grid text fixture allowlist에 `PreLine`을 추가하고, 그 줄바꿈이 text intrinsic contribution과 Grid auto row에 전달되는지 고정한다. text engine은 `PreLine`을 이미 지원하며 root HTML default-render adapter도 `white-space-collapse:preserve-breaks;text-wrap-mode:wrap`을 `WhiteSpaceMode::PreLine`으로 변환한다. 이번 변경은 anonymous Grid fixture의 명시 거부 경계와 CSS profile validator가 막고 있는 인접 연결을 다룬다.

이번 slice는 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture 경로만 수정한다. 일반 CSS author style-to-text admission, arbitrary source document, 공개 API, inline spans, `white-space:pre`, `pre-wrap`, `break-spaces`, 새 CSS parsing, nested Grid, placement/`order` 의미는 포함하지 않는다. HTML/CSS computed style을 일반적으로 자동 text shaping에 연결한다고 주장하지 않는다.

## 사전 비교와 독립 기대값

[Chrome 156 Agent Browser 관찰](../spec/internal/evidence/c11-4-anonymous-text-pre-line-precomparison-2026-10-31/README.md)은 Ahem 10px/10px로 같은 `XX\nXX` source run을 측정했다. Chrome 관찰과 별도로 CSS Text Level 3 §3의 white-space 처리, CSS Box Sizing Level 3 §2의 max-content 크기, CSS Grid Level 1 §6·§7.2.1·§11.5.1의 anonymous item과 max-content track 계산에서 기대값을 만들었다.

| 장면 | 고정 입력 | 독립 기대값 | Chrome 156 |
| --- | --- | --- | --- |
| `anonymous-normal-newline-max-content` | `white-space-collapse:collapse;text-wrap-mode:wrap`; root width 20px; `max-content` column; `XX\nXX` | LF가 collapsible space가 되어 max-content 한 줄 `20 + 10 + 20 = 50px`; auto row 10px; track/item이 root 밖으로 30px 넘침 | columns 50px, rows 10px, root 20×10px, Range 50×10px |
| `anonymous-pre-line-max-content` | `white-space-collapse:preserve-breaks;text-wrap-mode:wrap`; 나머지는 normal 장면과 동일 | LF가 forced line break; 각 줄 20px; max-content column 20px, auto row 20px, root 20×20px | columns 20px, rows 20px, root 20×20px, 두 text line fragment 각 20×10px와 LF의 0×10px Range rect |

Browser page의 x/y는 section 위치이므로 acceptance에는 쓰지 않는다. Range rectangle은 inline text fragment 관찰이며 anonymous Grid item box나 glyph ink 픽셀을 증명하지 않는다. LF의 0폭 Range rect는 원본 관찰값으로 남기고 글자 fragment 수나 item 크기로 해석하지 않는다.

## 계획된 변경

1. `crates/spinon-layout/src/host_document/text_paragraphs.rs`의 opt-in anonymous Grid paragraph allowlist가 `Normal | NoWrap | PreLine`과 `order:0`만 받도록 조정한다. 오류 이유 문자열도 세 허용값을 정확히 말한다. 일반 Block, opt-in 비활성 Grid, nonzero `order` 경계는 유지한다.
2. `crates/spinon-layout/src/host_document_tests/c11_4_anonymous_text.rs`에서 두 고정 장면을 검증한다. normal LF control은 max-content 50px/10px 한 줄, pre-line은 20px/20px 두 줄이어야 한다. intrinsic width/height, `ParagraphStyle.white_space`, source node/anchor, Columns·Rows contribution trace, root/item frame 및 revision provenance를 함께 검사한다.
3. `RuntimeBlockFlexGridTrackSizingV1` anonymous text fixture validator에 두 scene ID와 정확한 direct text source `XX\nXX`를 추가한다. pre-line scene의 computed root style은 `white-space-collapse:preserve-breaks;text-wrap-mode:wrap`이어야 하고 Normal scene은 `collapse/wrap`이어야 한다. static scene adapter는 `PreLine`을 명시적으로 전달하며 inferred CSS/cascade mapping을 주장하지 않는다.
4. 두 scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator app fixture 경로에 연결한다. 기존 anonymous text controls와 함께 root/item frame, intrinsic values, Columns·Rows trace, source order, layout-ready/revision, draw/present를 확인한다. 실제 실기기는 사용하지 않는다. Frame·trace·draw/present 결과를 glyph pixel equality나 iOS compositor 이후 pixels로 확대 해석하지 않는다.
5. 기본 artifact와 fixture-test opt-in artifact 경계를 다시 검사한다. 새 scene ID/payload와 fixture FFI marker는 opt-in positive control에만 있어야 한다. 관련 focused Rust tests, workspace Clippy, formatting, Android/iOS builds를 실행한다.
6. 구현/runtime 변경 후 새로운 실패 관점 20개, PR diff에는 별도 실패 관점 30개를 적용한다. 발견된 결함은 고치고 영향 관점을 다시 확인한다. required checks가 있으면 통과와 최종 head 연결을 확인한 뒤 저장소 허용 방식으로 병합한다.

## 실패 기준과 남은 제한

- Normal LF control이 50px/한 줄이 아니거나 pre-line scene이 20px/두 줄이 아니면 실패다.
- `PreLine` paragraph style과 computed `preserve-breaks/wrap` root profile이 다르거나 다른 revision/source의 intrinsic result를 소비하면 실패다.
- `order != 0`, 일반 CSS admission, opt-in 비활성 경로에서 새 동작이 허용되거나 실패 전 partial layout/trace가 발행되면 실패다.
- Android AVD 또는 iOS Simulator에서 frame·track/contribution trace·intrinsic/source revision·draw/present 중 하나라도 Chrome 관찰과 독립 계산에 어긋나면 실패다.
- 이 slice는 일반 HTML/CSS 지원, arbitrary fonts/scripts, WPT 적합성, 실제 Android 기기, Android hardware GPU, iOS compositor 이후 pixels, glyph paint 또는 C11.4/C11 완료를 증명하지 않는다.

## 계획 실패 관점 검토

[독립 실패 관점 20개](./c11-4-anonymous-grid-text-pre-line-plan-review-20.md)를 계획 PR 전에 별도로 기록한다. 기존 nowrap 계획의 검토 기록은 재사용하지 않는다.
