# C11.5 사전 비교 링크 보정 PR 변경분 실패 관점 검토 20개

검토 대상은 precomparison README의 Rspress-safe link 보정과 이 검토 기록이다. 구현 계획 검토 및 PR #1130의 diff 검토와 별개다.

| # | 실패 관점 | 결과 및 방어 |
|---:|---|---|
| 1 | repo 밖 fixture 링크가 Rspress route처럼 해석됨 | **통과.** fixture는 GitHub source 링크로 바꿨다. |
| 2 | JSON 파일이 존재하지 않는 `.json.html` route가 됨 | **통과.** 관측 JSON은 GitHub source 링크다. |
| 3 | expected JSON 링크가 Chrome 관측과 뒤섞임 | **통과.** expected JSON을 별도 GitHub source 링크로 유지한다. |
| 4 | screenshot normal link가 `.png.html` route를 만듦 | **통과.** screenshot은 Markdown image asset으로 임베드한다. |
| 5 | Markdown image link가 상대 evidence 디렉터리를 벗어남 | **통과.** `./chrome156.png`는 같은 evidence 디렉터리를 가리킨다. |
| 6 | screenshot hash를 HTML 수정 후 갱신하지 않음 | **통과.** 캡처 파일은 그대로이며 manifest와 README의 SHA-256이 유지된다. |
| 7 | fixture hash가 fixture source와 어긋남 | **통과.** 링크만 바꿨고 fixture hash는 변경하지 않았다. |
| 8 | GitHub 링크가 존재하지 않을 경로/파일명을 사용 | **통과.** 링크 경로는 저장소의 tracked fixture와 evidence JSON 이름에 맞춘다. |
| 9 | `main` 링크가 merge 전에는 유효하지 않음 | **통과.** 대상 파일은 이미 PR #1130으로 main에 병합됐다. |
| 10 | image alt가 Spinon runtime screenshot으로 오인 | **통과.** Alt는 Agent Browser HeadlessChrome 사전 비교임을 말한다. |
| 11 | 이미지 링크에서 파일 hash/해상도 정보가 사라짐 | **통과.** 바로 앞 문장에서 SHA-256과 1280×633 크기를 유지한다. |
| 12 | raw GitHub 링크가 Markdown sanitizer에 의해 HTML로 해석 | **통과.** 표준 HTTPS Markdown links를 사용한다. |
| 13 | README 설명이 관측과 독립 기대값 분리를 잃음 | **통과.** Chrome observation과 independent expected JSON 문장을 분리한다. |
| 14 | 변경이 public CSS support claim을 추가 | **통과.** scope/limits 문구는 수정하지 않았다. |
| 15 | 변경이 공식 완료 집계에 영향을 줌 | **통과.** `STATUS.md`와 checkbox는 이 PR에서 변경하지 않는다. |
| 16 | Tailnet 페이지가 이미지 배포 성공을 주장 | **통과.** 이 PR은 저장소 링크 형식만 수정한다. |
| 17 | 예상 밖 파일이 diff에 포함 | **통과.** 변경 대상은 README link 표현과 이 review artifact뿐이다. |
| 18 | 기존 Rspress 오류를 모두 해결했다고 주장 | **통과.** 수정 범위는 이 README의 신규 링크뿐이다. |
| 19 | terminal-browser를 검증 수단으로 사용 | **통과.** 이번 변경은 Agent Browser 검증 링크를 위한 markdown asset 경로다. |
| 20 | 계획·PR diff 검토 기록을 같은 것으로 중복 사용 | **통과.** link-fix PR 전용 review 파일을 별도로 둔다. |

## 결론

이 변경은 Rspress에서 JSON/fixture links가 page route로 오인되고 PNG link가 `.png.html` route로 바뀌는 문제를 피한다. 기존 Rspress dead links나 roadmap completion state는 변경하지 않는다.
