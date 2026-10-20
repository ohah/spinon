# C11.4 auto-repeat typed implicit tracks 계획 PR diff 검토 30개

**검토 대상:** 계획·20개 계획 검토·Chrome 155 fixture/결과/화면·해시 매니페스트
**검토 기준:** PR diff의 목적, 명세 연결, 경계, 증거의 재현성, 문서 정확도. 기능 구현 diff가 아니다.
**결과:** 30/30 관점 확인. 차단 이슈 없음. 이 검토 기록은 자기 자신을 제외한 PR 산출물을 검토했고, 파일 생성 후 자체 경로·목차·문법도 다시 확인했다.

| # | 검토 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR scope | 계획과 사전 비교 문서만 변경하며 Rust implementation을 포함하지 않는다. |
| 2 | 상위 roadmap anchor | C11 status, 전체 C11.4 plan, internal contract 0059에 연결한다. |
| 3 | normative source | CSS Grid 2025 CRD §7.5–§7.6, §8.3, §11.3–§11.8을 primary oracle로 지정한다. |
| 4 | parent feature attribution | PR #385의 auto-repeat + signed placement 경계를 전제로 후속 차이를 설명한다. |
| 5 | current route description | positive implicit list와 auto-repeat adapter의 typed/fixed-px 차이를 특정한다. |
| 6 | profile boundary | `RuntimeBlockFlexGridTrackSizingV1`만 확장 대상으로 삼는다. |
| 7 | grammar boundary | repeat count, line names, areas, arbitrary grammar 확장을 계획 범위에서 제외한다. |
| 8 | row/column symmetry | 양 축의 auto track size, percentage basis, cycle 방향을 독립 검증한다. |
| 9 | line semantics | negative/positive line cases와 leading/trailing 개수를 구체적으로 고정한다. |
| 10 | auto-fill / auto-fit | both modes와 occupied/empty collapse를 구분한다. |
| 11 | zero-size validity | `0px` 유효성과 negative/non-finite fixed rejection을 구분한다. |
| 12 | track list validation | empty list와 10,000-track limit을 명시한다. |
| 13 | `minmax()` validation | invalid function combinations을 입력 validation 범위에 둔다. |
| 14 | calc conversion | axis math property reuse와 `calc_handles` 전달을 구체 코드 경로에 적는다. |
| 15 | calc reference validation | auto track list의 node/axis binding을 확인하도록 명시한다. |
| 16 | Taffy adapter | `to_taffy_grid_auto_tracks`와 common track converter의 책임을 구분한다. |
| 17 | prevalidation chokepoint | `validate_auto_repeat_auto_track_list`의 기존 fixed-px restriction을 직접 다룬다. |
| 18 | unchanged positive implicit route | no-auto-repeat expansion 경로 회귀를 별도로 요구한다. |
| 19 | profile regression | C11.2.2/C11.2.3/TemplateGrid error behavior가 유지돼야 한다. |
| 20 | partial output | errors on invalid inputs must not publish partial `LayoutOutput`; OOM claim is scoped. |
| 21 | independent expectation provenance | normative JSON과 browser observation JSON/README를 분리했다. |
| 22 | repeat count with gap | 132px/8px gap/20px repeat formula yields five; case fixture values agree. |
| 23 | gutter expectation limits | gutter case records collapse/count behavior without hard-coding text metric as normative. |
| 24 | fractional tolerance | fractional CSS px comparison allows 1/64px and integer values remain exact. |
| 25 | screenshot completeness | image was captured with all 14 cases in a 1280×1800 viewport and visually inspected. |
| 26 | screenshot content | documented image path is the retained `chrome155-cases-agent-browser.png`; unused full-page image removed. |
| 27 | browser provenance | HeadlessChrome 155.0.8059.85, viewport, DPR and empty console errors are recorded in source JSON/README. |
| 28 | browser cleanup | named Agent Browser session was closed; active session listing was empty and no browser process remained. |
| 29 | runtime evidence promise | future Android/iOS simulator matrix is 124 existing +14 new=138 scenes, with hardware/compositor limits stated. |
| 30 | PR readability and integrity | plan review has 20 distinct perspectives; JSON parses, expected/observed IDs match 14, and the retained evidence files are hash-manifested. |

## Follow-up required before implementation

This plan PR must merge before the implementation branch starts. Implementation and runtime review use 20 new failure perspectives; final implementation PR diff review uses a new 30-point record.
