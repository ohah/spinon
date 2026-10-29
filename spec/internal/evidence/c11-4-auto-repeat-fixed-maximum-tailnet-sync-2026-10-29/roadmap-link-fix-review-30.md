# C11.4 Tailnet roadmap link 수정 PR diff 실패 관점 30개

2026-10-29 Tailnet Agent Browser에서 새 계획 route가 `Not found`로 확인된 뒤, 공개 roadmap entry의 내부 링크를 병합된 GitHub `main` 문서 경로로 바꾸는 diff를 검토했다. 이 변경은 PR #798용이며 Tailnet의 현재 파일에는 아직 반영되지 않았다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 계획 링크가 preview에 없는 `/plan/*.html` route를 계속 가리키는가? | 계획 Markdown의 GitHub `blob/main` URL로 바꿨다. |
| 2 | 사전 비교 링크가 preview에 없는 `README.html` route를 가리키는가? | 병합된 Markdown 근거의 GitHub URL로 바꿨다. |
| 3 | runtime 근거 링크가 preview에 없는 `README.html` route를 가리키는가? | 병합된 runtime README의 GitHub URL로 바꿨다. |
| 4 | Android screenshot 링크가 아직 병합되지 않은 branch를 가리키는가? | `main`의 PR #796 merge tree에 포함된 PNG 경로를 사용한다. |
| 5 | iOS screenshot 링크가 branch-only asset을 가리키는가? | `main`의 PR #796 merge tree에 포함된 PNG 경로를 사용한다. |
| 6 | official status link가 오래된 `/STATUS.html` preview page를 가리키는가? | 최신 상태 Markdown의 GitHub URL과 CSS heading anchor를 쓴다. |
| 7 | GitHub link ref가 잘못된 PR commit SHA에 고정되는가? | `blob/main`을 사용해 PR #797 merge 이후 최신 main content를 연다. |
| 8 | `main`에서 아직 없는 파일을 link하는가? | plan, precomparison README, runtime README, Android PNG, iOS PNG, STATUS 파일이 merge tree에 있다. |
| 9 | PR #796 external link가 다른 PR 번호인가? | 제목과 merge SHA가 PR #796 metadata와 일치한다. |
| 10 | merge timestamp가 바뀌거나 오타가 있는가? | PR #796의 실제 `2026-10-29T07:08:47Z`를 유지한다. |
| 11 | merge SHA가 main merge commit과 다른가? | `48100cc93b32c00a9fb50bde68e0c12fde78f08b`를 사용한다. |
| 12 | official count가 이 제한 slice로 증가하는가? | 공식 `72/213` 문구를 유지한다. |
| 13 | C11.4 checkbox가 완료로 표시되는가? | roadmap entry는 C11.4 full completion을 명시적으로 미검증으로 둔다. |
| 14 | 49 scene 결과를 WPT harness pass로 읽히게 쓰는가? | 고정 Chrome 156 comparison/runtime scene이라고 설명한다. |
| 15 | Android software AVD가 hardware GPU라고 불리는가? | `software AVD` 표기를 유지한다. |
| 16 | iOS Metal sample이 compositor 이후 pixel로 표현되는가? | 제출 전 drawable boundary를 그대로 적는다. |
| 17 | 일반 CSS admission을 이 PR이 추가한다고 쓰는가? | fixture profile 한정 문구를 보존한다. |
| 18 | definite percentage 및 calc 제한이 빠지는가? | definite `px/%`와 제한 `calc()` 범위를 같은 문장에 둔다. |
| 19 | 0px maximum floor를 검증된 범위처럼 암시하는가? | 기존 limitation text는 그대로 둔다. |
| 20 | CI check rollup이 비어 있음을 성공으로 표현하는가? | CI 통과 아님을 알리는 문구를 유지한다. |
| 21 | review/diff가 C11.4 parent를 완료로 승격하는가? | 공식 unchecked 상태와 parent limitation을 유지한다. |
| 22 | 기존 다른 roadmap 카드의 링크를 바꾸는가? | 변경 범위를 PR #796 카드 한 줄의 여섯 href로 한정한다. |
| 23 | image link가 screenshot이 아닌 metadata file을 여는가? | 두 링크가 실제 representative PNG 파일을 가리킨다. |
| 24 | screenshot 파일 경로에 scene ID 오타가 있는가? | Android/iOS run ID와 `columns-auto-fill-auto-px40-130.png`를 대조했다. |
| 25 | links가 PR #797 이전 문서 상태를 고정하는가? | `main`은 PR #797 병합 후의 최신 docs/status를 제공한다. |
| 26 | HTML attribute quoting이 깨졌는가? | URL을 기존 double-quoted `href` 형식으로 두고 `git diff --check`를 통과했다. |
| 27 | URL의 한글 anchor가 잘못 인코딩되는가? | status link anchor slug는 URL-compatible 형태를 사용한다. |
| 28 | preview가 아직 바뀌지 않았는데 배포 완료로 표시하는가? | 이 review와 PR body는 Tailnet update pending이라고 구분한다. |
| 29 | 링크 수정으로 official source HTML 자체가 손실되는가? | 전체 roadmap HTML은 유지하고 대상 entry의 href만 교체했다. |
| 30 | broken route discovery와 수정 후 확인을 이어서 추적할 수 없는가? | pass01 preflight/result, Agent Browser observation, 이 review를 같은 Tailnet evidence directory에 둔다. |

**결과:** diff는 C11.4 entry의 preview 내부 문서 링크 여섯 개를 `main` GitHub 링크로 바꾼다. PR #798 병합 뒤 Tailnet에 반영하고 Agent Browser에서 다시 검증한다.
