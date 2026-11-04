# PR #1153 병합 metadata·공개 로드맵 diff 적대 검토 30개

검토 대상은 PR [#1153](https://github.com/ohah/spinon/pull/1153)의 계획·상태·내부 계약·실행 근거·공개 로드맵 변경이다. 검토한 source diff head는 `e70edd5aa4c5b41a47e8495b5844e1246c412a0d`, base는 구현 PR #1152의 merge commit `6fe4980533f3da1e7250ff00fdb4d21e541a3a55`다. 이 검토는 구현 PR #1152의 별도 30-point review를 대체하지 않는다.

| # | 적대 관점 | 확인 결과 |
|---:|---|---|
| 1 | 구현 PR 번호가 다른 C11.1 PR로 잘못 연결되는가 | 계획·STATUS·contract·evidence가 모두 #1152를 가리킨다. 통과. |
| 2 | merge SHA가 feature branch head와 혼동되는가 | 문서에 최종 merge commit `6fe4980533f3da1e7250ff00fdb4d21e541a3a55`를 사용한다. 통과. |
| 3 | 병합 시각이 commit author time과 GitHub mergedAt 사이에서 어긋나는가 | GitHub PR `mergedAt` `2026-11-04T08:59:45Z`를 문서와 공개 카드에 맞췄다. 통과. |
| 4 | 제한 구현이 공식 완료 수에 산입되는가 | 모든 관련 기록이 `72/213`을 유지한다. 통과. |
| 5 | C11 또는 C11.1 parent checkbox가 실수로 완료되는가 | STATUS의 상위 체크는 그대로 미완료다. 통과. |
| 6 | bounded slice 제목이 전체 CSS Grid 구현 완료를 암시하는가 | 계획·상태·공개 카드에 제한 구현이라고 쓰고 parent 미완료를 명시한다. 통과. |
| 7 | 내부 계약 숫자 버전이 변경되는가 | contract `0.1.0`을 그대로 유지한다. 통과. |
| 8 | 구현 전 비교와 구현 결과가 한 결과처럼 섞이는가 | 계획과 evidence는 Chrome oracle 및 Android/iOS runtime을 별도로 구분한다. 통과. |
| 9 | merged source commit과 PR review head가 바뀌어 기록되는가 | 구현 commit `fb4135c3`, PR head `335805d1`, merge SHA가 별도 필드로 유지된다. 통과. |
| 10 | 계획 checkpoint가 아직 PR 전이라고 말하는가 | merge metadata를 기록하고 남은 단계 문구를 제거했다. 통과. |
| 11 | 다른 문서에 과거의 “PR 생성 후 반영” 또는 “병합 전” 상태가 남는가 | 변경한 계획·상태·contract·evidence 파일에서 stale 상태 문구가 없다. 통과. |
| 12 | plan의 PR 검토 횟수가 프로젝트 규칙과 충돌하는가 | plan은 runtime/code 20, source diff 20, PR diff 30을 구분한다. 통과. |
| 13 | 구현/runtime 20개와 PR diff 30개를 같은 검토로 계산하는가 | 두 검토를 별도 파일과 링크로 유지한다. 통과. |
| 14 | 최근 C11 summary가 새 병합 기록을 이전 #1147 기록과 바꿔치기하는가 | #1152를 최신 slice로 추가하고 #1147은 직전 구현으로 보존한다. 통과. |
| 15 | 공식 상태 항목의 측정값이 구현 evidence와 달라지는가 | 5 scenes·22 frames, focused 4/13/10, Android/iOS 26/26을 evidence README와 대조했다. 통과. |
| 16 | Rust 결과가 전체 workspace 통과로 확대되는가 | 문서는 focused/scoped 결과만 말하며 broad suite나 전체 CI 통과를 주장하지 않는다. 통과. |
| 17 | Android AVD 결과가 실기기나 hardware GPU로 표현되는가 | SwiftShader software renderer라고 명시한다. 통과. |
| 18 | iOS drawable 결과가 compositor 이후 pixel parity로 표현되는가 | 표본은 compositor 제출 전이라고 명시한다. 통과. |
| 19 | flag-off artifact 검사 범위가 제품 artifact 전체로 확대되는가 | DEX/executable marker scan 범위로 제한해 표현한다. 통과. |
| 20 | GitHub에 없는 status check를 통과로 표시하는가 | PR check rollup이 비어 있고 required checks가 없어 CI 통과로 세지 않는다고 기록한다. 통과. |
| 21 | Tailnet preview가 source merge 전 이미 갱신됐다고 주장하는가 | PR 본문은 source merge 이후 sync 예정이라고 구분한다. 통과. |
| 22 | 공개 로드맵 새 카드가 C11.1 최신 항목 뒤에 묻히는가 | PR #1152 카드를 hero 영역에서 직전 #1147 카드 앞에 넣었다. 통과. |
| 23 | 기존 #1147 기록이 교차축 구현도 포함한 것처럼 바뀌는가 | #1147의 same-axis 범위와 cross-axis 제외 설명은 보존한다. 통과. |
| 24 | 새 카드가 일반 CSS admission 또는 WPT 지원을 암시하는가 | bounded Root HTML profile이며 WPT·일반 CSS Grid를 완료하지 않았다고 쓴다. 통과. |
| 25 | public roadmap merge timestamp 또는 SHA가 틀리는가 | PR #1152 GitHub metadata와 대조해 정확히 기록했다. 통과. |
| 26 | public roadmap의 계획 링크가 오타 또는 다른 slice를 가리키는가 | `plan/c11-root-html-grid-mixed-cross-axis.md`로 연결된다. 통과. |
| 27 | runtime 링크가 기존 one-axis evidence를 가리키는가 | `c11-root-html-grid-mixed-cross-axis-runtime-2026-11-04/README.md`를 사용한다. 통과. |
| 28 | 공식 상태 링크가 내부 초안이거나 오래된 route인가 | `spec/STATUS.md`의 main 경로를 직접 연결한다. 통과. |
| 29 | roadmap HTML 카드의 paragraph 경계나 screenshot 대상이 깨지는가 | 추가 카드는 기존 `.kanban-note hero-progress` 구조를 따르고 PR 본문 PNG 링크 둘은 HTTP 200·image/png다. 통과. |
| 30 | WIP나 무관한 파일이 문서 PR에 포함되는가 | PR 파일 목록은 계획 2개, STATUS, contract, evidence README, public roadmap만 포함한다. 통과. |

결과: 30개 관점에서 수정이 필요한 문서 결함은 발견하지 못했다. 이 PR은 공개 roadmap source를 갱신하지만 Tailnet preview 동기화 자체는 source merge 뒤 별도로 검증한다.
