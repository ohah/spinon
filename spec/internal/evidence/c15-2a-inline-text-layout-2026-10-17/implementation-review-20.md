# C15.2a 구현 실패 관점 검토 20개

검토 범위는 C15.2a HostDocument 입력 변환, `ParagraphInput` owner 경로와 inline marker 생성, Parley 결과 검증, Taffy text measure 통합이다. 이 검토는 앱 runtime renderer나 실제 글자 픽셀을 포함하지 않는다. 두 개의 음성 경로를 직접 입력해 결함을 찾고 수정한 뒤 전체 검증을 다시 실행했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 같은 owner가 source order에서 닫힌 뒤 다시 나타나 중간 text까지 하나의 min/max 범위로 장식되는가 | `[owner] → [] → [owner]` 입력을 추가해 재현했다. `InvalidInlineHierarchy(owner)`로 거부하도록 source path 연속성 검사를 추가했고 회귀 테스트가 통과했다. |
| 2 | 한 ancestry path 안에 같은 NodeId가 두 번 들어가 순환 owner를 만드는가 | 입력 단계에서 반복 NodeId를 검사하고 `InvalidInlineHierarchy`로 반환한다. 빈 text source도 같은 구조 검사를 거친다. |
| 3 | 같은 NodeId가 서로 다른 depth나 parent path에서 재사용되는가 | NodeId별 `InlineRange.depth`와 전체 prefix owner path를 대조한다. 불일치는 범위를 합치기 전에 거부한다. |
| 4 | 세 번째 inline ancestor가 지원 profile에 조용히 평탄화되는가 | 직접 입력은 `UnsupportedInlineDepth`; HostDocument의 세 단계 span은 paragraph 진단으로 거부한다. layout crate의 C15 depth 테스트가 통과했다. |
| 5 | `inline_styles` key가 어느 text ancestry에도 없는가 | owner range가 없으면 `InlineBoxWithoutText(node)`로 거부한다. |
| 6 | 장식된 span에 정규화 후 보이는 문자가 없는가 | 빈 문자열 fixture가 `InlineBoxWithoutText`로 종료한다. 선행·후행으로 제거되는 whitespace도 zero-length range여서 같은 guard를 거친다. |
| 7 | 음수·NaN·무한 edge가 geometry에 전파되는가 | style validation은 각 edge를 finite/nonnegative로 제한한다. 음수·NaN negative test가 통과했다. |
| 8 | 한 style 안에서 border와 padding 합계가 overflow하는가 | inline/block start/end 네 합계를 검사한다. `f32::MAX + f32::MAX` 입력은 `InvalidInlineBoxStyle`로 거부한다. |
| 9 | 개별 style은 유효하지만 중첩 start marker 합계가 Parley 내부 범위를 깨뜨리는가 | 각 nested owner에 유한 `f32::MAX` edge를 둔 입력에서 Parley가 역전된 `usize::MAX..0` line range 두 개를 반환하는 것을 재현했다. output text range 검증을 추가해 `InvalidGlyphLayout`로 닫았고 회귀 테스트가 통과했다. |
| 10 | Parley line/run/cluster가 UTF-8 중간, 역전, 문자열 밖 byte range를 반환하는가 | 세 결과 모두 순서·길이·UTF-8 경계를 검사한다. supplementary scalar의 원 UTF-16 및 정규화 UTF-8 매핑 테스트도 유지된다. |
| 11 | 여러 Text node 경계를 넘는 collapsed whitespace가 source code unit과 owner 정보를 잃는가 | C15.1 source-map 회귀 테스트에서 모든 원본 code unit과 ancestor path를 확인했다. 인라인 geometry는 보존된 source order에서 계산한다. |
| 12 | 줄 시작/끝에서 제거되는 whitespace가 가짜 glyph 또는 폭을 만드는가 | normalizer는 zero-length source mapping만 보존한다. 실제 line advance는 Parley가 배치한 glyph/marker만 사용한다. |
| 13 | `pre-line` CRLF가 두 원본 UTF-16 code unit을 잃거나 두 줄이 되는가 | CRLF의 두 source range가 하나의 `\n` 출력 범위에 연결되고 line 수는 2인 회귀 테스트가 통과했다. |
| 14 | 중첩 start/end marker가 반대 순서로 쌓여 edge width가 뒤바뀌는가 | 같은 byte 위치 start는 outer→inner, end는 inner→outer 정렬이다. `nested-short`의 outer/inner x·width oracle이 통과했다. |
| 15 | 인접 sibling 경계에서 end가 다음 start보다 뒤에 붙는가 | 동일 byte 위치에서 모든 End가 Start보다 먼저 정렬된다. source range별 폭을 덧셈하는 대신 각 owner의 marker 폭만 사용한다. |
| 16 | 줄바꿈 때 시작 edge가 모든 fragment에, 끝 edge가 첫 fragment에 중복되는가 | owner/line별 별도 rect를 만들고 marker의 실제 line 배치를 따른다. `nested-wrap` 두 줄 oracle이 outer·inner·text rect를 모두 통과했다. |
| 17 | 줄별 fragment가 하나의 union rect로 합쳐져 빈 줄 영역을 덮는가 | fragment map key가 `(NodeId, line_index)`이고 snapshot도 두 줄을 각각 보존한다. |
| 18 | vertical border/padding이 line box 높이를 바꾸거나 content text rect까지 확장되는가 | owner fragment에만 block edge를 더하고 `TextLayoutSnapshot.height`는 Parley line height 그대로 둔다. nested-short oracle에서 line `10px`, inline rect `16px/20px`를 확인했다. |
| 19 | HostDocument가 DOM source order를 바꾸거나 숨은 subtree를 visible inline content로 포함하는가 | adapter는 역순 stack으로 child order를 복원한다. `display:none` subtree를 text collection에서 제외하고 hidden node style 누락은 별도 오류로 거부한다. host adapter 테스트가 통과했다. |
| 20 | unsupported element/style이 default inline box처럼 조용히 처리되거나 measure snapshot과 final width가 달라 재사용되는가 | non-span, 비-inline display, margin, percentage padding, 음수/overflow border는 실패 진단을 낸다. Taffy는 exact available width와 revision을 확인하고 불일치 snapshot을 거부한다. 관련 host/layout 회귀 테스트가 통과했다. |

## 수정 후 검증

- `cargo test --locked -p spinon-text-layout`: 17 passed.
- `cargo test --locked -p spinon-layout c15`: 9 passed.
- `cargo test --locked --workspace`: 947 passed, 0 failed, 4 ignored.
- workspace Clippy, rustfmt, Android target compile, iOS Simulator target compile: 통과.
- 해당 검증은 Rust geometry snapshot과 target compile까지다. 앱 runtime text projection, 실제 glyph paint, overflow pixel, Android/iOS Simulator 화면은 연결되지 않았다.
