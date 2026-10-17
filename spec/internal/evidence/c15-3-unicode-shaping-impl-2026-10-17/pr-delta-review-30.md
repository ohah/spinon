# C15.3 pure Rust shaping PR 변경 적대 검토 30개

현재 branch의 전체 tracked/untracked diff를 대상으로 review했다. 서로 다른 API·코드·근거·상태 경계를 확인했으며, 이 PR의 범위를 넘어서는 runtime 완료 주장을 금지한다.

| # | 변경 공격 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | `ParagraphStyle` 필드 추가로 workspace struct literal이 빠지는 경우 | 전체 workspace `cargo clippy --all-targets`가 모든 target을 컴파일한다. 통과. |
| 2 | 새 mode가 crate root에서 re-export되지 않아 호출자가 이름을 쓸 수 없는 경우 | `TextDirection`, `LineBreakMode`, `LigatureMode` re-export를 확인했다. |
| 3 | mode 기본값이 이전 Parley 0.12 strict line-break 기본과 달라지는 경우 | `LineBreakMode::Strict` 기본과 문서 근거를 확인했다. |
| 4 | Auto/LTR/RTL 중 구현되지 않은 방향이 silently default되는 경우 | 세 enum 분기와 Arabic Auto/LTR/RTL 배치 assertion을 확인했다. |
| 5 | 빈 paragraph early return이 mode를 기본값으로 덮는 경우 | empty snapshot regression에서 세 mode를 비기본값으로 지정해 검사한다. |
| 6 | Snapshot이 paragraph shaping policy를 반환하지 않아 cache/diagnostic 경계가 소실되는 경우 | nonempty와 empty snapshot 모두 요청 mode를 가진다. |
| 7 | `TextGlyph` 변경이 glyph identity/position/advance 계약을 제거하는 경우 | 기존 field를 유지하고 cluster UTF-8 range와 source vector를 추가했다. |
| 8 | RTL 정보가 visual order를 복원할 수 없게 run에서 빠지는 경우 | `TextRun.is_rtl`은 Parley run에서 채우고 mixed Arabic/digit test가 구별한다. |
| 9 | 여러 glyph를 source cluster와 대응할 때 길이 불일치를 zip truncation으로 숨기는 경우 | cluster-glyph와 positioned-glyph 개수를 먼저 비교한다. |
| 10 | 같은 glyph ID가 반복될 때 order mismatch를 잘못 허용하는 경우 | visual cluster 순서별 glyph ID와 positioned sequence의 ID를 각각 비교한다. |
| 11 | 중복 cluster start가 map에서 앞 범위를 덮어쓰는 경우 | BTreeMap 삽입 중 중복 start를 거부한다. |
| 12 | RTL 시각 순서를 logical cluster vector 순서로 잘못 처리하는 경우 | range start index lookup 후 `visual_clusters()` 순으로 기대 stream을 만든다. Arabic test가 거꾸로 된 source order를 확인한다. |
| 13 | ligature continuation가 비연속 source atom을 포함하는 경우 | 이어진 byte range인지 확인하고 불연속이면 닫는다. |
| 14 | line/run이 ligature atom 중간에서 시작해 source span을 과대/과소 지정하는 경우 | continuation-only run 시작을 거부하고 start/continuation group을 확장한다. |
| 15 | 한 cluster가 인접 DOM text node 경계를 넘어 owner 정보를 잃는 경우 | source map은 cluster range로 각 source를 조회하며 전용 2-node combining test가 있다. |
| 16 | 원본 UTF-16 범위와 normalized UTF-8 범위가 바뀌어 호출자에게 노출되는 경우 | `TextSourceRange` 기존 단위를 유지하고 C15.3 test가 양 단위를 함께 assertion한다. |
| 17 | glyph ID 0을 일반 glyph로 반환해 fallback 실패를 숨기는 경우 | `MissingGlyphCoverage { paragraph, text_range }`로 구체적으로 거부한다. |
| 18 | 새 오류 variant가 `Display`나 상위 error propagation 없이 빠지는 경우 | paragraph와 UTF-8 범위를 표시하고 기존 layout error wrapping을 재사용한다. |
| 19 | `LigatureMode::None`이 CSS ligature feature 하나만 끄는 경우 | liga/clig/dlig/hlig/calt를 모두 끄며 실제 width difference와 source range를 검사한다. |
| 20 | 고정 font와 system fallback이 섞여 C15.3 결과가 머신별로 변하는 경우 | fixture마다 `new_without_system_fonts()`와 단일 등록 family를 사용한다. |
| 21 | Noto test fonts가 기본 crate의 runtime resource로 들어가는 경우 | test module 아래 include이며 default `.rlib` negative control에서 원본 bytes가 없다. |
| 22 | negative control만 통과하고 scan 방법이 잘못된 경우 | 같은 원본 font bytes가 test executable positive control에 모두 존재한다. |
| 23 | pinned font README의 hash가 실제 artifact와 다르는 경우 | 세 SHA-256을 다시 계산해 provenance README 값과 일치시켰다. |
| 24 | font licensing 정보가 누락되거나 implementation PR에서 라이선스를 제거하는 경우 | fixture README에 upstream commit, SIL OFL 1.1과 license hash를 보존했다. |
| 25 | CJK test가 통과했으므로 general Japanese/Chinese segmentation 지원을 주장하는 경우 | spec은 ICU4X model 부재와 lexical/general wrapping 미검증을 명시한다. |
| 26 | Hangul Canvas advance를 DOM inline-block rect와 같은 값으로 처리하는 경우 | test comment와 C15 oracle 문서에서 `96.470169` advance와 `97.609375` DOM rect를 구분한다. |
| 27 | Arabic/Digit/emoji/combining fixture 결과를 actual browser glyph identity로 과장하는 경우 | 테스트는 fixed Parley/shaper와 pinned fonts의 결과만 주장하며 Chrome oracle를 separate evidence로 둔다. |
| 28 | runtime style projection·renderer가 text를 이미 paint한다고 로드맵에서 보이게 만드는 경우 | 문서와 STATUS는 text admission, WGPU glyph paint 및 Simulator ink를 미완료로 유지한다. |
| 29 | Rust code size, formatting 또는 lint 문제를 숨기는 경우 | `engine.rs`는 500줄 미만, glyph mapping은 module로 분리, fmt와 최종 workspace Clippy가 통과한다. 최초 lint 결과와 수정 사실도 보존했다. |
| 30 | evidence에 테스트 로그는 있어도 내부·공개 roadmap/status 링크나 결과 개수가 stale인 경우 | 이 PR의 최종 변경에서 `STATUS.md`, 내부 evidence index, 공개 `roadmap.html`, 구현 README, 20/30 review를 서로 연결하고 9/26/154 결과를 고정한다. Rspress 전체 빌드 실패는 저장소 기존 dead link 473개로 분리 기록했고 새 C15.3 README는 실패 목록에 없다. |

이 PR은 화면을 그리는 runtime path를 수정하지 않는다. Screenshot을 Spinon 동작 화면이라고 첨부하지 않으며 Android/iOS Simulator 결과도 통과로 표시하지 않는다.
