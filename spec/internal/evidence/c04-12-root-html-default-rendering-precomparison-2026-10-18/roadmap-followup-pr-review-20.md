# C04.12 기본 root HTML 로드맵 후속 PR · diff 실패 관점 20개

PR #322의 `spec/public/roadmap.html` 수정과 갱신한 근거 연결을 서로 다른 20개 관점으로 검토했다.

| # | 적대적 검토 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 표준 `RuntimeGpuHost::new()`가 root HTML을 받지 않는다는 설명이 source와 다를 수 있다. | 구현은 `RuntimeSession::new_runtime_gpu()`로 시작하며 source-taking root HTML GPU constructor가 없다. |
| 2 | 기본 RootHtmlDocument가 render scene을 만든다고 잘못 설명할 수 있다. | `RuntimeCalculationProfile::has_paint()`은 RootHtmlDocument에서 false이며 scene 부재를 source로 확인했다. |
| 3 | opt-in fixture text paint를 제품 path처럼 보이게 할 수 있다. | 새 note는 text 화면 연결이 opt-in fixture뿐이라고 명시한다. |
| 4 | 계획 PR #321이 아직 열려 있다고 보일 수 있다. | #321의 GitHub state는 MERGED이며 Rebase and merge commit을 확인했다. |
| 5 | 계획을 구현 완료로 잘못 세거나 C04.12 parent를 완료로 보일 수 있다. | note는 계획이라 표현하고 C04.12/C15/C16/C19가 미완료라고 남긴다. |
| 6 | Android/iOS 화면이 이미 확인된 것처럼 읽힐 수 있다. | default runtime 및 두 Simulator 검증이 아직 없다고 명시했다. |
| 7 | 계획에 없는 범위를 이번 계획이 모두 지원한다고 과장할 수 있다. | note는 지원 결과가 아니라 다음 slice 방향만 기록한다. |
| 8 | rendered document의 출처가 별도 fixture tree로 보일 수 있다. | 같은 parsed HostDocument를 기본 GPU host와 paint에 연결하는 목표로 표현했다. |
| 9 | OS scheme와 CSS color-scheme 연결이 빠질 수 있다. | planned scope에 computed style과 `color-scheme`이 함께 적혀 있다. |
| 10 | 새 plan route 링크가 base prefix를 빠뜨릴 수 있다. | plan 링크는 기존 `/spinon/plan/...` Tailnet path와 일치한다. |
| 11 | 사전 기준 README가 틀린 route를 가리킬 수 있다. | `internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/README.html`의 route를 확인했다. |
| 12 | light/dark PNG link가 존재하지 않는 파일을 가리킬 수 있다. | 두 PNG가 evidence source directory에 있고 SHA manifest가 통과했다. |
| 13 | raw screenshot 경로가 router에 의해 HTML route처럼 해석될 수 있다. | Tailscale preview server는 파일 suffix를 유지해 PNG bytes를 그대로 제공한다. |
| 14 | 한글 note가 HTML entity/encoding 손상으로 표시될 수 있다. | UTF-8 source를 유지했고 Agent Browser rendering에서 문구를 확인한다. |
| 15 | source line 끝에 HTML tag가 잘못 붙어 DOM 구조가 깨질 수 있다. | 새 `<p>`는 앞 문단 `</p>` 뒤에 삽입했으며 기존 줄/섹션 구조를 보존한다. |
| 16 | note가 중복으로 삽입되거나 비슷한 old note를 변경할 수 있다. | 고유 marker occurrence는 1개이며 diff는 roadmap file 1줄만 변경한다. |
| 17 | 추가 문구가 공식 completion count를 별도 고정값으로 표시할 수 있다. | count·checkbox·STATUS는 건드리지 않고 공식 상태 원본을 유지한다. |
| 18 | 화면 캡처가 이전 브라우저나 다른 fixture를 보여줄 수 있다. | PR 캡처는 수정한 public `roadmap.html`을 Agent Browser로 연다. |
| 19 | Tailscale preview 서버를 재시작하거나 tmux를 바꿀 수 있다. | 동기화는 기존 listener가 읽는 파일만 atomically replace하며 server/tmux lifecycle은 건드리지 않는다. |
| 20 | Tailnet sync가 아직 실행되지 않았는데 완료했다고 표시할 수 있다. | PR 본문은 source change만 설명하고, Tailnet URL/hash 검증은 병합 뒤 별도 evidence로 남긴다. |

이 검토는 static roadmap diff에 대한 것이며 기본 runtime 구현, Simulator 결과 또는 Tailnet 복사 완료를 대신하지 않는다.
