# C11.4 혼합 text/image 계획 로드맵 반영 PR diff 검토 20개

**검토 대상:** `spec/public/roadmap.html`에 추가한 PR #609 계획 기록 1개와 이 검토표. `spec/STATUS.md`의 공식 체크 상태나 기능 코드는 수정하지 않았다.

| # | 독립 실패 관점 | 판정과 확인 |
| ---: | --- | --- |
| 1 | 기준 PR이 아직 병합되지 않았거나 다른 변경을 가리키는가? | 통과. GitHub PR #609가 `MERGED`이고 merge SHA가 `9624792443f1ff0e3678cc8b4278a87e82d775d1`이다. |
| 2 | 기록한 병합 시각이 실제 값과 다른가? | 통과. `2026-10-24T19:58:54Z`는 PR의 `mergedAt`과 일치한다. |
| 3 | 2026-10-25 표시일과 UTC 병합 일자가 충돌하는가? | 통과. roadmap 표시일은 한국 시간 기준 날짜이고 괄호에는 GitHub UTC 시각을 썼다. |
| 4 | 계획 기록이 런타임 구현 완료처럼 읽히는가? | 통과. 본문이 계획·사전 비교만 포함하며 Spinon runtime 검증 전이라고 한정한다. |
| 5 | 4개 scene 결과를 전체 Grid 지원으로 확대하는가? | 통과. 단일 intrinsic column, non-spanning item 조합으로 범위를 제한한다. |
| 6 | 독립 기대값을 Chrome 관측에서 역산했다고 오해할 수 있는가? | 통과. 기대값과 Chrome 관측을 구분하고 CSS Grid 2025 CRD를 기준으로 기록한다. |
| 7 | `min-content`와 `max-content`의 기대 크기가 계획과 다른가? | 통과. roadmap의 50px/80px은 계획 및 사전 비교의 독립 계산과 같다. |
| 8 | 텍스트/이미지의 source order 역전 조건이 빠졌는가? | 통과. 본문에 markup source order를 뒤집어도 크기가 유지되는 검사를 썼다. |
| 9 | span-1 조건을 더 넓은 spanning 결과처럼 표현하는가? | 통과. `non-spanning`이라고 명시했다. |
| 10 | Android 앱이나 AVD 검증을 주장하는가? | 통과. Android/iOS 앱 검증은 아직 하지 않았다고 적었다. |
| 11 | iOS 앱 또는 compositor 결과를 주장하는가? | 통과. 플랫폼 앱 검증을 했다는 문구가 없다. |
| 12 | 기본/opt-in artifact 경계 검증을 주장하는가? | 통과. artifact 검증 전이라고 적었다. |
| 13 | WPT 또는 전체 conformance를 주장하는가? | 통과. WPT 미실행을 명시한다. |
| 14 | C11.4 공식 checkbox가 이 계획만으로 완료 처리되는가? | 통과. `spec/STATUS.md`를 변경하지 않았고 C11.4 미완료라고 남겼다. |
| 15 | 완료 수 `72/213`을 올리거나 다른 분모로 바꾸는가? | 통과. roadmap 메모는 당시 공식 `72/213`을 유지한다고 적는다. |
| 16 | 화면에 표시되는 공식 집계의 동적 fetch 로직을 망가뜨리는가? | 통과. `baseTasks`, status URL, 집계 스크립트는 변경하지 않았다. |
| 17 | 기존 #607의 병합 기록이나 그 앞의 note를 덮어쓰는가? | 통과. 새 문단을 hero section 끝에 추가했고 기존 문단은 보존했다. |
| 18 | HTML 문단/앵커 경계가 깨져 뒤 section까지 감싸는가? | 통과. 새 문단을 `</section>` 앞에 두고 시작·종료 태그를 확인했다. |
| 19 | 새 계획·사전 비교·PNG 경로가 잘못되었거나 누락되었는가? | 통과. 저장소에 있는 계획, README, Agent Browser PNG 경로와 일치한다. |
| 20 | 검토 링크 자체가 빠졌거나 잘못된 route를 가리키는가? | 통과. 이 검토표를 `/spinon/internal/evidence/c11-4-roadmap-mixed-text-image-plan-sync-2026-10-25/pr-diff-review-20.html`에 연결했다. |

## 남은 범위

이 변경은 로드맵 메모를 고치는 문서 변경이다. 해당 계획 slice의 runtime 구현, Android/iOS 앱 실행, fixture artifact 경계, WPT, 실기기 또는 전체 C11.4 완료를 증명하지 않는다.
