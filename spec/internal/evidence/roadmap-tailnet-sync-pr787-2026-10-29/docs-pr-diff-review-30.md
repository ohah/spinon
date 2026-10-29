# PR #787 병합 정보 공개 로드맵 반영 diff 검토 30개

2026-10-29 · PR #787 병합 후 `spec/STATUS.md`, C11.4 상위 계획, `spec/public/roadmap.html` 세 문서 변경을 서로 다른 실패 관점으로 검토했다. 구현 코드는 건드리지 않았으며 공식 완료 상태는 그대로 유지한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 공개 요약이 잘못된 PR을 가리키는가 | 구현 PR #787 링크가 실제 merged PR과 일치한다. |
| 2 | merge SHA를 feature branch commit으로 기록했는가 | Rebase merge 결과 `328e4e416b80012caac6297f1fabd556b85e2074`를 사용했다. |
| 3 | merge 시각의 timezone 또는 날짜를 바꿔 적었는가 | GitHub `mergedAt`의 UTC 값 `2026-10-29T03:04:46Z`를 그대로 기록했다. |
| 4 | 계획 문서가 merge 전 상태로 남아 있는가 | C11.4 parent plan에 #787 및 merge SHA를 추가했다. |
| 5 | 공식 상태 대장에 PR 연결이 빠졌는가 | 세 assertion의 현재 상태 항목에 #787 merge 정보를 넣었다. |
| 6 | 공개 로드맵이 오래된 latest C11.4 카드만 보여 주는가 | 새 #787 항목을 #778 카드보다 앞에 삽입했다. |
| 7 | merge 당시 check rollup을 성공으로 오표기하는가 | PR check rollup이 비었고 main required checks도 없었던 범위를 명시했다. |
| 8 | 기록이 GitHub check 상태를 현재도 그대로라고 확대하는가 | 병합 시점 기록으로 한정해 서술했다. |
| 9 | CSS Grid 상위 C11이 완료로 바뀌었는가 | C11 checkbox는 수정하지 않았다. |
| 10 | C11.4 하위 항목을 전체 완료로 표시했는가 | C11.4 checkbox `[ ]`를 유지했다. |
| 11 | 공식 완료 집계가 부분 fixture 때문에 증가했는가 | 상태와 공개 로드맵 모두 `72/213`을 유지한다. |
| 12 | assertion 세 개를 다른 WPT 파일로 기록했는가 | `fit-content-track-sizing.html`의 세 assertion 이름을 기록했다. |
| 13 | 40px boundary의 used row를 틀리게 적었는가 | `fit-content(40px)` 기대 row를 40px로 기록했다. |
| 14 | 30px boundary의 used row를 틀리게 적었는가 | `fit-content(30px)` 기대 row를 30px로 기록했다. |
| 15 | 200px boundary에서 max-content보다 크게 늘어나는가 | 기대 row와 100px child를 100px로 기록했다. |
| 16 | raw growth limit과 used base-size 설명이 다시 혼동되는가 | raw finite growth limit 100px과 argument가 제한하는 base size를 분리했다. |
| 17 | Android 실기기나 hardware GPU라고 확대하는가 | Android API 37 ARM64 software AVD로 한정했다. |
| 18 | iOS 실기기라고 확대하는가 | iPhone 17 Pro / iOS 26.2 Simulator로 한정했다. |
| 19 | iOS Metal sample의 compositor 경계를 과장하는가 | compositor 제출 전 표본으로 명시했다. |
| 20 | PixelCopy/Metal 표본 수가 다른 실행에서 섞였는가 | 양 플랫폼 11/11 exact로 같은 최종 실행 근거를 참조한다. |
| 21 | scene 수가 한 플랫폼에서 빠졌는데 3/3이라고 하는가 | Android와 iOS 각각 세 scene 3/3임을 기록한다. |
| 22 | fixture 실행을 Spinon WPT harness pass라고 말하는가 | pinned WPT assertions를 opt-in fixture로 연결한 결과로 제한한다. |
| 23 | fixture 결과를 일반 CSS admission으로 확대하는가 | 일반 CSS admission이 아님을 명시했다. |
| 24 | 공개 요약이 전체 C11.4 완료를 암시하는가 | 전체 C11.4 미완료를 별도 문장으로 남겼다. |
| 25 | public roadmap의 anchor 삽입으로 이전 entry를 지웠는가 | 기존 #778 이후 및 이전 카드 텍스트는 보존된다. |
| 26 | plan 링크가 PR branch나 잘못된 절대 경로를 가리키는가 | 공개 Tailnet 경로 `/spinon/plan/c11-4-fit-content-limit-boundaries.html`를 사용한다. |
| 27 | precomparison route가 빠져 근거 검토가 막히는가 | 사전 비교 README route를 연결했다. |
| 28 | runtime 결과·화면 링크가 빠지는가 | simulator 실행 README route를 연결했다. |
| 29 | 공식 상태 링크가 Tailnet의 이전 status 페이지를 가리키는가 | `/spinon/STATUS.html`의 CSS checklist anchor를 연결했다. |
| 30 | HTML 삽입이 script/style/API 동작을 바꾸는가 | 추가한 것은 기존 `hero-progress` 문단과 링크뿐이며 앱 코드·문서 theme·API는 변경하지 않았다. |

최종 diff는 공백 검사에서 통과했다. 문서 변경만 포함하며 구현 검증, 공식 지원 범위, 공식 집계를 변경하지 않는다.
