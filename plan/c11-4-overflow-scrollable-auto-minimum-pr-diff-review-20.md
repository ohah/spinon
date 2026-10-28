# C11.4 Grid auto minimum · 계획 PR diff 적대적 검토 20개

**대상:** `plan/c11-4-overflow-scrollable-auto-minimum` 계획·계획 검토·parent-plan link, 14-case HTML oracle fixture 및 연결된 precomparison 자료 전체 staged diff.

**검토 경계:** 계획 PR의 문서와 기준 입력만 검토했다. Rust implementation, build/test, 앱 runtime, Android/iOS simulator, GitHub CI는 검증하지 않았다.

| # | PR diff의 실패 경로 | 변경분에서 확인한 방지 조건 | 판정 |
| ---: | --- | --- | --- |
| 1 | 좁은 자동 최소값 회귀를 공식 C11.4 완료로 잘못 승격 | `spec/STATUS.md`와 `72/213`을 변경하지 않고 제한 unit-test 후보로 설명한다. | 통과 |
| 2 | 남은 overflow control을 추적하는 parent plan이 새 plan을 찾지 못함 | parent C11.4의 unsupported-overflow control 항목에서 focused plan으로 링크한다. | 통과 |
| 3 | Grid auto minimum 조건 일부를 빠뜨려 잘못된 content-based 기대값 설정 | §6.6을 인용하고 단일 `minmax(auto,1fr)` span으로 span 예외를 제거한다. | 통과 |
| 4 | `overflow:clip`을 scrollable로 취급해 content-based 최소 크기를 0으로 설정 | W3C Overflow 기준과 Chrome oracle에서 `visible`·`clip`을 같은 100px/10px 분기로 둔다. | 통과 |
| 5 | `hidden`만 검사하고 나머지 scrollable 값의 분기를 누락 | Columns/Rows 각각 `hidden`, `auto`, `scroll` 세 값을 독립 case로 둔다. | 통과 |
| 6 | `overflow:auto`를 scrollbar geometry까지 완성된 제품 지원으로 오해 | 계획은 기존 `Auto -> Hidden` adapter mapping만 다루고 C13.4 scrollbar geometry를 제외한다. | 통과 |
| 7 | column text가 wrapping되어 100px min-content 조건을 만들지 못함 | 공백 없는 Ahem 10-glyph line과 60px definite Grid를 고정한다. | 통과 |
| 8 | row-axis 입력이 column contribution을 재사용해 수직 최소 크기를 잘못 판정 | 단일 Ahem glyph의 10px line-height와 5px definite Grid row를 별도로 둔다. | 통과 |
| 9 | Columns만 구현/test되고 row overflow mapping이 빠짐 | HTML, expected JSON, 계획 표에서 양 axis의 5가지 overflow 값과 controls를 각각 적는다. | 통과 |
| 10 | 명시적 item min 0이 overflow-zero 분기의 성공으로 위장 | visible overflow의 `min-width:0`·`min-height:0`을 별도 specified-minimum control로 둔다. | 통과 |
| 11 | zero-min track control이 item min-size control과 섞임 | `minmax(0,1fr)` controls를 item min 0과 별도 입력·trace 분기로 둔다. | 통과 |
| 12 | 최종 geometry만 비교해 잘못된 contribution source가 우연히 통과 | 계획은 track contribution trace source와 value를 함께 assert하도록 요구한다. | 통과 |
| 13 | item frame만 맞고 Grid track base가 틀린 상태가 숨겨짐 | Grid frame, used tracks, item frame을 각각 독립적으로 판정한다. | 통과 |
| 14 | browser fixture·expected·observation에서 case가 빠지거나 다른 ID에 연결 | 독립 확인 script가 fixture 14 IDs, expected 14 cases, Chrome 14 entries의 frame/track/overflow를 대조했다. | 통과 |
| 15 | Ahem 누락 또는 fallback font로 임계값이 바뀜 | fixture 상대 경로, loaded status, `fonts.check`, font digest를 함께 기록한다. | 통과 |
| 16 | viewport screenshot을 전체 문서 화면이나 simulator evidence로 오인 | full-page PNG와 viewport metadata를 구분하고 HTML/Chrome 관측임을 명시한다. | 통과 |
| 17 | 단일 Chrome build 결과를 보편 browser conformance로 확대 | metadata는 `HeadlessChrome/156`으로 한정하고 WPT/다른 browser를 주장하지 않는다. | 통과 |
| 18 | `scrollWidth`/`scrollHeight`를 실제 scrolling, clipping 또는 scrollbar 동작 증명으로 사용 | README는 scroll extent를 observation으로만 두고 UI/state/pixel 검증에서 제외한다. | 통과 |
| 19 | Agent Browser process가 남거나 Terminal Browser evidence가 섞임 | 전용 Agent Browser sessions를 닫고 빈 session 목록·비활성 process 결과를 기록했다. | 통과 |
| 20 | 계획 PR을 제품 구현·시뮬레이터·CI 통과로 보고 | 계획 문서와 PR 본문은 실제 Rust implementation, tests, app runtime, simulator, CI를 별도 미실행 상태로 둔다. | 통과 |

**판정:** PASS 20/20, 계획 PR diff blocker 0개. 계획 검토 20개와는 다른 PR-level failure paths를 확인했다. 실제 implementation/runtime의 새 20관점 검토와 implementation PR diff 20관점 검토는 후속 구현에서 다시 수행한다.

## 구현 피드백 추가 기록 · 2026-10-28

계획 PR #749 병합 후 focused Rust test에서 `minmax(0,1fr)` control에는 item `Minimum` contribution이 질의되지 않는 것을 확인했다. 이 PR diff 검토는 그 대조군이 `AutomaticMinimum { content_based: false }`를 내야 한다는 잘못된 기대를 발견하지 못했다. 원래 PASS는 #749의 당시 diff 검토 기록으로 보존하며, 구현 PR에서 expected JSON과 계획을 `Minimum query 없음`으로 수정한다. 이 후속 발견을 새 구현/runtime 검토와 implementation PR diff 검토에서 별도로 다룬다.

Rows text case도 planned Taffy path에서 `TextLayoutHeightMismatch`를 만들었다. 이 failure boundary는 PR diff 검토에서 발견하지 못했고, C15 text contract를 느슨하게 하지 않도록 Rows oracle/test를 natural replaced image로 변경했다.

구현을 연결한 첫 실행에서는 natural 10×10 image의 aspect ratio가 row track 축소에 따라 auto width까지 5px로 줄어드는 것을 추가 발견했다. 앞선 PR diff 검토는 fixed 10px column track만으로 image width가 고정된다고 가정했다. fixture와 test 양쪽에서 image CSS width를 10px로 고정하고, 구현 PR 검토에서 축 간 coupling을 별도 관점으로 확인한다.

이후 Agent Browser 재관측에서는 replaced `<img>`의 UA 기본 computed overflow가 `clip`인 것을 발견했다. 앞선 PR diff 검토는 HTML fixture의 `overflow:visible` row baseline을 확인하지 못했다. fixture에서 visible을 명시하고 Chrome computed style을 재수집해 visible/clip cases가 구별되게 했다.
