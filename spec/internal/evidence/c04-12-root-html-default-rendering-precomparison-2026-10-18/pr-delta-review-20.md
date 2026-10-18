# C04.12 기본 root HTML 렌더링 계획 PR · diff 실패 관점 20개

이 PR의 문서·fixture·Chrome 관찰 diff를 병합 전 서로 다른 20개 관점으로 검토했다. 계획의 기술 가능성 검토와 구현 실패 검토는 별도 기록이며, 이 PR은 구현 완료를 주장하지 않는다.

| # | diff 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | STATUS가 계획을 구현 완료 checkbox로 잘못 표시할 수 있다. | 새 bullet은 C04.12 parent와 구현 checkbox를 바꾸지 않고 계획 단계라고 명시한다. |
| 2 | C04.12/C15/C16/C19 parent가 완료로 오인될 수 있다. | plan header와 STATUS 모두 상위 미완료를 보존한다. |
| 3 | 로드맵 완료 개수가 수동으로 바뀌어 공식 계산과 달라질 수 있다. | 상태 카운터나 체크박스에는 손대지 않았다. |
| 4 | STATUS의 계획 링크가 내부 docs route에서 dead link가 될 수 있다. | main의 GitHub blob URL을 사용해 `plan/` 경로가 docs route로 해석되지 않게 했다. |
| 5 | STATUS에서 Chrome evidence 또는 plan review 링크가 잘못된 위치를 가리킬 수 있다. | 두 경로가 `spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/` 아래에 실제 존재한다. |
| 6 | plan의 명세 참조가 존재하지 않는 링크일 수 있다. | 저장소 source 경로는 inline code로 두고, docs route로 검증되는 STATUS 및 evidence Markdown만 링크했다. |
| 7 | screenshots를 Markdown page 링크로 해 Rspress가 `.png.html`을 찾을 수 있다. | evidence README는 파일명을 inline code로 표시해 dead page link를 추가하지 않는다. |
| 8 | screenshot 경로가 evidence 안에서 복사·해시 검증되지 않을 수 있다. | SHA manifest의 fixture와 screenshot 경로를 repo-root 기준으로 통일했다. |
| 9 | screenshot 자체가 fixture와 다른 viewport에서 캡처될 수 있다. | light/dark 이미지 모두 Agent Browser viewport `320×240`, DPR 1이며 실제 PNG 크기를 확인했다. |
| 10 | browser 버전과 실행 host가 없어 관찰을 재현할 수 없다. | HeadlessChrome `155.0.0.0`, macOS ARM host와 user agent를 JSON에 기록했다. |
| 11 | OS color scheme이 실제로 emulated 되었는지 알 수 없다. | light/dark media emulation을 별도 설정하고 `prefers-color-scheme` 및 computed colors를 각각 읽었다. |
| 12 | JSON 수치가 화면/실제 DOM 관찰과 어긋날 수 있다. | main, heading, paragraph, strong, Range geometry와 computed color/font를 같은 browser run의 출력에서 기록했다. |
| 13 | fixture source가 보고된 style/selector와 다를 수 있다. | `#content`, `#heading`, `#paragraph`, `#emphasis`와 light/dark CSS가 기록된 oracle와 일치한다. |
| 14 | SHA manifest가 오래된 파일을 가리키거나 잘못된 hash를 기록할 수 있다. | `shasum -a 256 -c`가 fixture 및 두 screenshot 모두 `OK`를 반환했다. |
| 15 | plan review 20개를 PR review 횟수로 중복 계산할 수 있다. | 별도 `plan-review-20.md`와 `pr-delta-review-20.md`에 분리 기록했다. |
| 16 | plan review 항목이 실제로 20개가 아니거나 반복 문구일 수 있다. | plan review는 1–20 번호이며 parser/identity, viewport/scheme, font, text layout, paint 경계를 분리한다. |
| 17 | 기본 FFI host 경로가 없는 상태를 있는 것으로 가정할 수 있다. | `RuntimeGpuHost::new()`와 fixture-only root HTML constructor source를 확인해 계획에 미구현 연결을 기록했다. |
| 18 | Fontique system backend를 활성화하면 된다고만 하고 target 차이를 숨길 수 있다. | plan은 잠금 dependency source의 Android/CoreText backend를 target builds와 simulator에서 별도로 검증하도록 한다. |
| 19 | Chrome host metrics를 iOS/Android와 같다고 과장할 수 있다. | plan과 README는 resolved system font가 다를 수 있다고 적고 native platform별 값을 기록하도록 한다. |
| 20 | 문서 build 실패나 simulator 미실행을 숨겨 merge 기준을 오해시킬 수 있다. | README는 checkout에서 본 560개 dead-link group 실패와 Simulator 미실행을 기록하며 새 경로는 실패 목록에 없는 것을 확인했다. |

검토에서 발견한 link-rendering·manifest path 문제를 수정한 뒤 SHA 검증과 문서 build를 재확인했다. 전체 docs build는 기존 dead links로 실패했고 이 PR의 파일 경로는 실패 목록에 나타나지 않았다.
