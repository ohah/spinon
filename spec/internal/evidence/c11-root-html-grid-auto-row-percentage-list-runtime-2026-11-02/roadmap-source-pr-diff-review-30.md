# C11.1 percentage `grid-auto-rows` public roadmap 변경 검토 · 30개 관점

**검토 대상:** `spec/public/roadmap.html`에 추가한 PR #1047 제한 구현 card와 링크. 제품 코드와 테스트는 변경하지 않는다.

| # | PR diff 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | PR 번호 누락 또는 다른 slice 번호 사용 | 구현 PR #1047을 연결한다. |
| 2 | PR merge 시각 혼동 | GitHub merge 시각 `2026-11-02T09:10:09Z`와 일치한다. |
| 3 | merge SHA를 source head로 바꿈 | GitHub merge SHA `bc3430ac664e76a2f246bcbfc387f61121017bb6`를 사용한다. |
| 4 | 제한 기능을 모든 Root HTML/Grid profile에 적용했다고 표현 | 문구는 기본 parsed Root HTML route만 가리킨다. |
| 5 | implicit rows 대신 explicit rows를 포함했다고 오해 | implicit-row 목록이라고 명시하고 전용 계획에 연결한다. |
| 6 | 지원되는 track type을 확대 | direct px/percentage scalar·list만 언급한다. |
| 7 | 음수 또는 비유한 값을 지원한다고 암시 | 유한한 비음수 값으로 제한한다. |
| 8 | percentage basis 조건을 생략 | definite height 조건을 문장에 포함한다. |
| 9 | 목록 순환 규칙을 누락 | 첫 implicit row부터 순환한다고 적는다. |
| 10 | Cascade test 수 오기 | evidence와 일치하는 7/7이다. |
| 11 | Layout test 수 오기 | evidence와 일치하는 14/14이다. |
| 12 | Style-to-layout suite 이름·수 오기 | `style-to-layout Root HTML 26/26`으로 기록한다. |
| 13 | Clippy/rustfmt 검증을 넓게 주장 | scoped all-target Clippy와 rustfmt로 한정한다. |
| 14 | Android 기기 종류 혼동 | Android API 37 ARM64 AVD로 적었다. |
| 15 | iOS Simulator를 실기기로 표현 | iPhone 17 Pro/iOS 26.2 Simulator라고 표시한다. |
| 16 | runtime 단계 과장 | 각 플랫폼의 layout-ready와 13-box draw/present만 주장한다. |
| 17 | screenshot을 pixel parity라 부름 | 시각 자료이며 surface readback이 아니라고 명시한다. |
| 18 | Android PixelCopy를 했다고 암시 | PixelCopy readback을 하지 않았다고 명시한다. |
| 19 | iOS Metal surface readback을 했다고 암시 | Metal drawable readback을 하지 않았다고 명시한다. |
| 20 | CI 미보고를 green으로 바꿈 | CI check이 보고되지 않았다고 밝힌다. |
| 21 | C11.1를 완료 처리 | C11.1 미완료를 명시한다. |
| 22 | C11 parent를 완료 처리 | C11 미완료를 명시한다. |
| 23 | 공식 완료 수를 증가 | `72/213`을 유지한다고 적는다. |
| 24 | 상태 URL을 오래된 위치로 연결 | Tailnet `/spinon/STATUS.html` anchor를 사용한다. |
| 25 | plan 링크가 빠지거나 다른 plan을 가리킴 | percentage auto-row dedicated plan으로 연결한다. |
| 26 | runtime evidence 링크가 wrong slug를 가리킴 | PR #1047 evidence directory와 `README.html`로 연결한다. |
| 27 | merge/review evidence 링크가 빠짐 | merge record와 PR diff review를 각 경로에 연결한다. |
| 28 | screenshot 링크의 platform 파일명을 혼동 | Android AVD 및 iOS Simulator pass03 파일을 각각 연결한다. |
| 29 | HTML 요소/class를 새 구조로 바꿔 layout 회귀 | 기존 `kanban-note hero-progress` paragraph 패턴을 유지한다. |
| 30 | 최신 카드가 overview section 밖에 삽입됨 | 기존 hero section 종료 직전에 추가해 같은 progress 영역에 둔다. |

## 최종 판정

카드의 구현 범위, 검증 수치, merge metadata, 지원 한계와 링크는 main 문서 및 PR #1047 evidence와 일치한다. 공식 C11 상태는 승격하지 않는다.
