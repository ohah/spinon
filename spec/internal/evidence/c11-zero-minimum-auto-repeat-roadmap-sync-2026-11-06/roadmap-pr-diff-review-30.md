# C11.1 zero-minimum auto-repeat · roadmap PR diff 적대 검토 30개

**대상:** PR diff의 `spec/STATUS.md`, 내부 계약 0059, `plan/c11-css-grid.md`, `spec/public/roadmap.html`과 이 실행 근거. Tailnet copy는 merge 뒤 별도 안전 preflight·backup 절차로 진행한다.

| # | 독립 실패 관점 | 실제 변경 대조 및 판정 |
|---:|---|---|
| 1 | 공식 C11 checkbox가 실수로 완료되는가 | `spec/STATUS.md`의 C11 상위 행은 계속 `[ ]`다. 통과. |
| 2 | C11.1/C11.4가 과대 승격되는가 | 변경 요약은 제한 slice만 기록하고 두 범위를 미완료로 명시한다. 통과. |
| 3 | 완료 수 분모가 바뀌는가 | 변경에는 상태 checkbox가 추가되지 않는다. 후보 public page는 `78/214`를 표시했다. 통과. |
| 4 | 제품 지원 완료 문구가 추가되는가 | 일반 CSS admission, WPT, 공개 API가 미검증/미선언으로 남는다. 통과. |
| 5 | PR merge 식별자가 틀리는가 | GitHub PR #1216의 Rebase merge SHA `ec97003197569ca646563fc36afb386708d210a9`를 상태와 계획에서 대조했다. 통과. |
| 6 | 날짜가 잘못된 시간대를 포함하는가 | 기록된 `2026-11-06T09:38:25Z`는 GitHub `mergedAt`과 일치한다. 통과. |
| 7 | `RootHtmlAutoRepeatV1`이 다른 profile로 바뀌어 보이는가 | 모든 문서는 기존 parsed Root HTML 제한 profile로 표현하며 신규 runtime profile을 주장하지 않는다. 통과. |
| 8 | auto-fill과 auto-fit 범위를 혼동하는가 | 두 mode를 명시하되 단일 auto-repeat 축으로 한정한다. 통과. |
| 9 | 반복 pattern이 일반 track 목록으로 확대 해석되는가 | 한 pattern의 1–2개 track 경계를 모든 요약에 유지한다. 통과. |
| 10 | 최소값/최대값 grammar가 확대되는가 | finite non-negative fixed px minimum과 `1fr` maximum만 기록한다. 통과. |
| 11 | 컨테이너 및 gutter 조건이 빠지는가 | definite positive axis size와 finite non-negative px gap을 internal contract에 기록한다. 통과. |
| 12 | repeat-count 식에서 gutter가 누락되는가 | status·contract·roadmap 수식이 `(A + G)/(S + P×G)`로 일치한다. 통과. |
| 13 | 1px floor가 actual track minimum에 전파되는가 | status·contract·roadmap 모두 count 전용 floor와 actual 0px를 구분한다. 통과. |
| 14 | repeat count가 0이 될 수 있는가 | 수식과 contract는 최소 1회를 유지한다. 통과. |
| 15 | 10,000 track quota가 clamp로 오해되는가 | over-limit 입력은 실패하며 clamp하지 않는다고 명시한다. 통과. |
| 16 | undocumented unsupported syntax가 지원처럼 보이는가 | named line, general `minmax()`/intrinsic sizing, implicit columns, nested Grid 등의 기존 제외를 유지한다. 통과. |
| 17 | auto-repeat 전체 거부라는 계약 문구가 구현과 모순되는가 | 0059의 기존 문장을 bounded subset 예외를 가리키도록 수정하고 별도 섹션에서 범위를 정의했다. 통과. |
| 18 | internal contract numeric version이 오르는가 | `0.1.0`을 유지하고 신규 public contract/version을 추가하지 않는다. 통과. |
| 19 | public CSS API가 생겼다고 표현되는가 | 신규 parser, FFI 및 public API 없음으로 설명한다. 통과. |
| 20 | Android evidence가 hardware GPU로 일반화되는가 | Android API 37 AVD/ANGLE/SwiftShader 제한을 STATUS·contract·roadmap에서 표시한다. 통과. |
| 21 | iOS evidence가 compositor pixel proof로 일반화되는가 | Simulator Metal drawable이 compositor 제출 전임을 표시한다. 통과. |
| 22 | Chrome oracle가 WPT 통과로 오인되는가 | Chrome 157 10/10은 fixed reference 비교로만 표기하고 WPT를 별도로 미검증 처리한다. 통과. |
| 23 | broad suite가 통과했다고 오해하는가 | focused suites와 Clippy 결과로 한정하고 전체 CSS 적합성을 주장하지 않는다. 통과. |
| 24 | CI가 통과했다고 표시되는가 | PR check rollup 없음과 main required check 미설정을 기록한다. 통과. |
| 25 | `STATUS.md` 링크 상대경로가 깨지는가 | 계획과 evidence 링크가 저장소 경로 기준으로 맞고 PR artifact가 main에 존재한다. 통과. |
| 26 | public HTML이 과거 “implementation not started” 상태를 유지하는가 | C11 plan의 기존 후속 요약을 merged PR #1216 상태로 갱신했다. 통과. |
| 27 | 계획의 auto-repeat 제외 문장이 구현과 충돌하는가 | blanket exclusion을 bounded-slice 설명으로 교체했다. 통과. |
| 28 | Desktop 화면에 최신 note가 빠지는가 | Agent Browser에서 첫 progress note로 PR #1216과 merge SHA를 확인했다. 통과. |
| 29 | Mobile overflow 또는 count drift가 생기는가 | 390px viewport에서 document/body 폭 390px, counter `78/214`를 확인했다. 통과. |
| 30 | 증거 캡처와 브라우저 lifecycle이 부정확한가 | Desktop/mobile 캡처 hash를 기록했고 Agent Browser 프로세스와 임시 server가 종료됐다. Tailnet copy는 merge 뒤로 분리했다. 통과. |

**결론:** 30개 서로 다른 문서·로드맵 실패 관점을 확인했다. 기존 전체 auto-repeat 거부 문구와 C11.1 지원 목록의 과도한 exclusion만 이번 bounded implementation에 맞게 정정했다. 이 PR은 공식 완료 수를 바꾸지 않으며, Tailnet preview는 병합 후 별도 hash·backup 검증으로 갱신한다.
