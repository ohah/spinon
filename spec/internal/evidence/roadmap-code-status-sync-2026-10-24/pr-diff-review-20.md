# 코드·로드맵 상태 동기화 PR diff 검토 20개

이 문서는 문서·상태 동기화 diff만을 대상으로 한다. 각 항목은 다른 불일치 경로를 확인한다. 상태는 선택 RSPress build, source comparison, GitHub merge-state lookup 및 Agent Browser 검증을 마친 뒤 기록한다.

| # | 독립 실패 관점 | 판정 기준 | 결과·근거 |
|---:|---|---|
| 1 | raw Markdown checkbox 수를 공식 표시 수로 오해한다. | 카운터의 source-of-truth를 공식 상태 대장의 로드맵 ID 집계로 적는다. | 통과 — 상태 대장 ID 집계를 기준으로 공개 UI가 동적으로 세며, README에서 원시 checkbox 수와 구분했다. |
| 2 | 상위·세부·ID 없는 하위 작업 합계가 213과 다르다. | 78 + 133 + 2 = 213을 공개 UI에서 확인한다. | 통과 — Agent Browser 칸반 캡처와 DOM에서 78 + 133 + 2 = 213을 확인했다. |
| 3 | 현재 완료 수가 상태 대장과 공개 페이지에서 다르다. | 양쪽 모두 72/213이어야 한다. | 통과 — 공식 상태 대장, roadmap footer, 완료 칸 모두 72/213이다. |
| 4 | C13 unchecked 상태가 실수로 완료 처리된다. | 변경 전후 C13 상위 체크가 미완료다. | 통과 — diff는 체크박스를 바꾸지 않고 C13은 공식 상태 대장에서 미완료다. |
| 5 | 미완료와 현재 WIP가 같은 의미로 쓰인다. | C13은 미완료·비활성, C11.4는 현재 WIP라고 분리한다. | 통과 — 공식 상태·roadmap 본문·칸반이 C13 inactive, C11.4 active로 일치한다. |
| 6 | C13.1 fixture 결과가 전체 overflow/clip 지원으로 확대된다. | 제한 opt-in slice와 남은 범위를 함께 적는다. | 통과 — C13.1/2/3 구현은 제한 slice임을 명시했고 C13 전체를 완료 처리하지 않았다. |
| 7 | C13.2 box-shadow overflow가 완료된 것으로 읽힌다. | ink-only overflow의 fail-closed 경계를 그대로 적는다. | 통과 — C13.2 ink-only `box-shadow` overflow의 fail-closed 상태를 그대로 유지했다. |
| 8 | C13.2 scroll owner/offset fixture가 공개 CSSOM 지원으로 확대된다. | 내부 snapshot 결과와 공개 API 범위를 분리한다. | 통과 — 내부 scroll snapshot 근거와 public DOM `scroll` event/CSSOM 미구현을 구분했다. |
| 9 | C13.3 gesture router가 public DOM event 지원으로 과장된다. | 공개 DOM `scroll` event/CSSOM 미구현을 유지한다. | 통과 — roadmap과 상태 대장 모두 공개 DOM event/CSSOM 부재를 명시한다. |
| 10 | native wheel/trackpad 결과를 양 플랫폼 완전 검증으로 표현한다. | 플랫폼별 부분 검증으로 제한한다. | 통과 — OS-native wheel/trackpad은 부분 검증으로 기록했다. |
| 11 | C13.3 PR #185 merge 상태가 잘못 기록된다. | GitHub의 현재 PR 상태와 merge SHA를 대조한다. | 통과 — GitHub에서 `MERGED`, merge SHA `71a6e28bbcf6c8181a86f3e88a79b4c4e9cbf130`을 확인했다. |
| 12 | C13.4 scrollbar/gutter가 이미 끝난 것처럼 읽힌다. | 미완료 범위로 남긴다. | 통과 — C13.4 scrollbar/gutter를 미완료로 유지했다. |
| 13 | C13.5 scroll snap이 이미 끝난 것처럼 읽힌다. | 미완료 범위로 남긴다. | 통과 — C13.5 scroll snap을 미완료로 유지했다. |
| 14 | C13.6 logical axis가 이미 끝난 것처럼 읽힌다. | 미완료 범위로 남긴다. | 통과 — C13.6 logical axis를 미완료로 유지했다. |
| 15 | C11.4 제한 slice merge가 상위 C11.4를 완료 처리한다. | C11.4 unchecked 상태를 보존한다. | 통과 — C11.4 상위 체크는 미완료고, 제한 fixture/PR을 전체 Grid 지원으로 승격하지 않았다. |
| 16 | C11.4 live plan이 옛 `77/213`을 현재값처럼 남긴다. | live plan의 현재 집계를 72/213으로 맞춘다. | 통과 — 14개 live plan 각각 `77/213`을 `72/213`으로 바꿨으며, 각 diff는 그 치환만 포함한다. |
| 17 | 과거 계획·리뷰 기준값을 현재값으로 덮어쓴다. | parent-plan 이력과 plan-review 기록의 날짜별 snapshot은 보존한다. | 통과 — parent plan·날짜별 review와 과거 실행 snapshot은 수정하지 않았다. |
| 18 | 공개 roadmap HTML과 status markdown이 서로 다른 WIP를 말한다. | 두 문서 모두 C11.4 active, C13 inactive를 말한다. | 통과 — 공식 상태 대장과 공개 roadmap이 현재 active lane 및 C13 범위를 같은 문장으로 표현한다. |
| 19 | Tailnet이 로컬 소스와 다른 문서 build를 제공한다. | 선택한 각 route의 SHA-256이 build output과 HTTP 제공 응답에서 일치한다. | 통과 — 선택 30개 route 모두 HTTP 200, build/local preview SHA 일치, 삭제 없음. 상세 hash는 `remote-verification-pass01.json`. |
| 20 | 검증 종료 때 이외 Agent Browser 세션까지 닫는다. | 검증 세션만 종료하고 세션 목록·해당 프로세스를 확인한다. | 통과 — 해당 세션을 종료했고 세션 목록은 비었으며 Chrome profile 프로세스도 없다. Tailnet preview server는 유지했다. |
