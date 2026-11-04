# Public roadmap PR #1144 diff 적대 검토 30개

검토 대상은 spec/public/roadmap.html의 C11.1 one-axis placement 카드, spec/STATUS.md의 C11 fragment anchor, 이 review 기록이다. 병합된 PR #1142, main 상태 대장, 보존 실행 evidence와 비교했다. 페이지 전용 문서 변경이며 구현/test/runtime은 다시 실행하지 않았다.

| # | 적대 관점 | 확인 근거와 판정 |
| --- | --- | --- |
| 1 | 카드 변경에 무관한 HTML이나 roadmap 영역이 포함되었는가 | diff는 C11 section 끝의 새 paragraph, 상태 대장의 C11 항목에 fragment span 하나, 별도 검토 파일을 추가한다. 다른 roadmap section이나 status 내용은 바꾸지 않는다 — 통과. |
| 2 | 카드가 존재하지 않는 구현 PR을 병합 완료로 표시하는가 | GitHub PR #1142 조회는 MERGED다 — 통과. |
| 3 | merge SHA가 실제 merge commit과 다른가 | PR #1142 merge SHA는 0f5506423a1150c80e5684fd087b948923fdbcb7이며 main tip ancestry와 일치한다 — 통과. |
| 4 | implementation head와 merge SHA를 혼동하는가 | 카드에는 merge SHA만 표기하고 구현 head는 링크된 status/evidence에서 별도로 식별한다 — 통과. |
| 5 | UTC timestamp가 GitHub merge 시각과 일치하는가 | PR #1142 mergedAt은 2026-11-04T02:24:11Z다 — 통과. |
| 6 | bounded slice를 전체 C11.1 지원으로 표현하는가 | 카드 제목에 bounded implementation을 표시하고 기본 parsed Root HTML 경로의 제한 조건만 설명한다 — 통과. |
| 7 | 지원 admission 조건이 느슨하게 요약되는가 | 모든 visible direct item의 definite numeric line/span 축이 같고 반대 축이 auto일 때만 해당한다 — 통과. |
| 8 | mixed/full placement나 implicit sizing이 성공한다고 암시하는가 | 범위 밖 line, mixed/full definite, named, dense/column, content item 및 implicit extent는 거부한다고 명시한다 — 통과. |
| 9 | Chrome comparison 결과가 제품 runtime이나 WPT 통과로 혼동되는가 | 14 scenes, 41 records, 40 frames, 8 accepted, 5 rejection, all-auto control 1, 0 CSS px를 사전 비교로만 표현한다 — 통과. |
| 10 | Rust focused test 수가 저장된 기록과 다른가 | linked evidence는 layout 5/5, style-to-layout 6/6, RuntimeSession 2/2와 sparse/dense regression 77/77을 기록한다 — 통과. |
| 11 | Android PixelCopy 수치를 달리 말하는가 | 실행 로그와 adapter evidence는 PixelCopy 6/6 exact를 기록한다 — 통과. |
| 12 | AVD 결과를 실기기나 hardware GPU 성공으로 과장하는가 | 카드에 API 37 ANGLE/Vulkan/SwiftShader AVD라고 적고 hardware 경계를 보존한다 — 통과. |
| 13 | iOS drawable sample 수를 잘못 말하는가 | simulator runtime log의 Metal 표본 6/6과 일치한다 — 통과. |
| 14 | iOS 표면 readback을 compositor 뒤 화면으로 표시하는가 | compositor 제출 전 drawable이라는 제한을 카드에 명시한다 — 통과. |
| 15 | 빈 GitHub checks를 CI 성공으로 주장하는가 | PR #1142 checks가 보고되지 않았고 카드가 CI 성공으로 세지 않는다고 적는다 — 통과. |
| 16 | 공식 완료 수를 C11 slice 때문에 올리는가 | 카드와 공식 상태는 72/213을 그대로 유지한다 — 통과. |
| 17 | C11.1/C11 상위 체크 상태가 완료로 바뀌는가 | card와 linked STATUS는 상위 완료 checkbox를 열어 둔다 — 통과. |
| 18 | 계획·계약·실행 근거·review 링크가 잘못되거나 누락되는가 | 카드의 10개 local route에 해당하는 계획, 계약, evidence, PNG, review, STATUS source가 존재한다. 새 30개 문서 PR 검토 기록도 연결된다 — 통과. |
| 19 | 증거 화면 링크가 다른 commit/scene을 가리키는가 | Android/iOS screenshot은 PR #1142 evidence 디렉터리의 같은 one-axis scene PNG다. #1143 link는 merge-status 문서 PR을 가리킨다 — 통과. |
| 20 | 카드가 중복·잘못 중첩되거나 HTML paragraph 경계가 깨지는가 | 고유 merge SHA marker가 한 번만 있고 추가 p 요소는 C11 section 닫기 바로 앞에 있다. 전체 diff whitespace check가 통과한다 — 통과. |
| 21 | 계획 문서의 `/spinon/` base path와 source 경로가 일치하는가 | `plan/c11-root-html-grid-one-axis-auto-placement.html`은 `plan/c11-root-html-grid-one-axis-auto-placement.md`를 가리키며 파일이 존재한다 — 통과. |
| 22 | 내부 계약 route가 spec 경로를 잘못 가리키는가 | `internal/0059-c11-grid-layout.html`에 대응하는 `spec/internal/0059-c11-grid-layout.md`가 존재한다 — 통과. |
| 23 | 중첩 evidence route가 다른 디렉터리의 동명 파일과 충돌하는가 | README·runtime reviews·PNG의 route별 source 경로가 `spec/internal/evidence/` 아래에 존재한다 — 통과. |
| 24 | HTML fragment link가 잘못된 C11 위치로 이동하는가 | 명시적 C11 fragment span을 추가했다. local Agent Browser에서 `#c11-css-grid` 대상과 C11 문구, scrollY 52274를 확인했다 — 통과. |
| 25 | 구현 PR의 기준 link가 임시 브랜치로 바뀌는가 | implementation/status link는 고정 번호 PR URL #1142/#1143을 사용한다 — 통과. |
| 26 | PR 본문의 이미지 원본 URL이 404 또는 잘못된 형식인가 | Android/iOS pinned raw GitHub URLs는 HTTP 200, `image/png`로 확인했고 각각 1080×2400, 1206×2622다 — 통과. |
| 27 | 카드가 캡처 이미지를 surface readback의 증명으로 과장하는가 | 카드의 image links는 실행 근거의 Android PixelCopy/iOS Metal 표본 설명과 함께 연결하고 compositor 전 한계를 유지한다 — 통과. |
| 28 | 현재 roadmap에 C11 관련 카드가 중복 추가되는가 | 새 one-axis marker는 한 번만 나오며 기존 C11.1 카드 순서와 본문은 수정하지 않는다 — 통과. |
| 29 | 새 검토 evidence route에 잘못된 문서 수가 표시되는가 | 신규 linked review source는 실제 30개의 서로 다른 실패 관점 표를 포함한다 — 통과. |
| 30 | PR 처리 절차와 본문이 프로젝트 규칙을 따르는가 | 한글 `docs(c11)` 제목과 기존 type/area label을 쓰고, 템플릿의 상태·명세·근거·잔여 범위를 채운다. 프로젝트의 Rebase and merge 규칙을 따른다 — 통과. |

## 판정

카드의 merge state, support 범위, 검증 숫자와 한계가 PR #1142 및 연결 evidence와 일치한다. 일반 CSS/WPT 또는 C11 완료를 주장하지 않는다. 이 검토는 HTML/CSS 전체 roadmap의 적합성 검사나 제품 runtime test가 아니다.
