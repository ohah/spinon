# C11.4 zoomed-grid 로드맵·Tailnet 변경 실패 관점 검토 30개

2026-10-29 · `spec/public/roadmap.html`의 runtime 상태 추가와 선택 Tailnet 경로 동기화 산출물을 서로 다른 30개 실패 관점으로 확인했다. 이 검토는 PR #783 구현 diff 및 병합 후 상태 문서 검토와 별개다.

| # | 실패 관점 | 확인 및 결과 | 판정 |
|---:|---|---|---|
| 1 | 사전 비교 항목이 현재 runtime 결과인 것처럼 적히는가 | 기존 항목을 `사전 비교`로 명시하고 runtime 결과를 별도 항목에 둔다. | 통과 |
| 2 | 미실행이라고 적힌 계획 문구가 현재 상태로 남는가 | 이전의 “미실행” 표현은 runtime 카드에 남지 않고 실제 결과로 대체했다. | 통과 |
| 3 | zoom WPT CSSOM 값과 Taffy typed input이 혼동되는가 | `gridTemplateRows:40px`와 Taffy `fit-content(80px)`을 분리한다. | 통과 |
| 4 | max-content contribution과 used row를 같은 수치로 표시하는가 | contribution `800px`, used row `80px`을 구분한다. | 통과 |
| 5 | frame 순서나 CSS px 단위가 바뀌는가 | root/item/child `120×600 / 120×80 / 120×800 CSS px`로 고정했다. | 통과 |
| 6 | 다른 fixture를 현재 scene으로 연결하는가 | scene ID `fit-content-zoomed-grid-wpt`를 그대로 기록한다. | 통과 |
| 7 | 한 exact route를 일반 CSS `zoom` 지원으로 확대하는가 | 일반 `zoom`과 전체 WPT 미지원 문구를 유지한다. | 통과 |
| 8 | 제한 slice를 C11.4 전체 완료로 승격하는가 | C11.4 미완료와 공식 `72/213`을 명시한다. | 통과 |
| 9 | PR #783의 병합 사실이나 SHA를 잘못 기록하는가 | GitHub의 merged state, UTC 시각과 merge SHA `664f4a848a3bf21bec4d2be6080633a8e3b4c5a4`를 대조했다. | 통과 |
| 10 | 상태·계약 sync PR #784를 구현 PR로 잘못 표시하는가 | #783은 구현, #784는 상태·계약 동기화로 구분한다. | 통과 |
| 11 | merge 시각을 로컬 시간과 혼동하는가 | 두 `mergedAt` 값을 UTC ISO 형식 그대로 표시한다. | 통과 |
| 12 | 허용되지 않은 merge 전략을 표시하는가 | 두 PR 모두 Rebase and merge로 기록한다. | 통과 |
| 13 | 빈 CI rollup을 green으로 해석하는가 | 두 PR의 check rollup 부재를 밝히고 CI 통과라고 쓰지 않는다. | 통과 |
| 14 | Android simulator를 실기기/hardware GPU로 과장하는가 | API 37 ARM64 software AVD와 PixelCopy로 제한한다. | 통과 |
| 15 | Android pixel 수가 실제 evidence와 맞지 않는가 | overflow 세 점과 outside-root control, 합계 4/4를 확인했다. | 통과 |
| 16 | iOS simulator 표본을 compositor 이후 화면이라고 부르는가 | Metal drawable, compositor 제출 전이라고 제한한다. | 통과 |
| 17 | iOS 표본 수가 evidence와 맞지 않는가 | 세 overflow 점과 outside-root control, 합계 4/4를 확인했다. | 통과 |
| 18 | static FFI archive 검사를 APK/IPA 전체 exclusion으로 확장하는가 | 공개 문구는 archive 결과를 앱 번들 전체의 증거로 확대하지 않는다. | 통과 |
| 19 | 검증하지 않은 zoom 조합을 지원한다고 오해하는가 | ancestor/descendant, auto-repeat, percentage/calc, text intrinsic 조합은 범위 밖이다. | 통과 |
| 20 | 계획 route가 잘못된 Tailnet URL을 가리키는가 | `/spinon/plan/c11-4-fit-content-zoom-wpt.html`가 선택 사이트 route에 생성됐다. | 통과 |
| 21 | 내부 계약 링크가 0059와 다른 문서를 가리키는가 | `/spinon/internal/0059-c11-grid-layout.html`의 zoomed assertion section을 Agent Browser로 열었다. | 통과 |
| 22 | runtime evidence 링크가 stale 또는 없는 route인가 | README route title과 한계 문구를 Agent Browser에서 읽었다. | 통과 |
| 23 | 구현·병합 후 review 링크가 빠지거나 route가 잘못되는가 | 계획 20, 구현/runtime 20, PR diff 30, 병합 후 30 리뷰 route를 생성했다. | 통과 |
| 24 | roadmap 이미지 링크가 다른 run의 화면을 가리키는가 | Android `014400643Z-28ab6d1f`, iOS `014407699Z-670c5b73` 최종 capture 경로를 연결했다. | 통과 |
| 25 | screenshot 원본의 기기 크기가 잘못 표시되는가 | Agent Browser에서 Android `1080×2400`, iOS `1206×2622`로 로드됨을 확인했다. | 통과 |
| 26 | 두 문단이 2열 레이아웃의 좁은 칸에 들어가 읽기 어려운가 | 최초 화면에서 288px 열을 발견해 두 항목에 `hero-progress`를 적용했다. 최종 browser card 너비는 1240px이다. | 통과 |
| 27 | 기존 C11.4 이전 작업이 새 항목으로 대체되는가 | 기존 progress 문단은 보존하고 zoom 계획/실행 문단을 추가했다. | 통과 |
| 28 | 동일한 runtime 카드가 중복 노출되는가 | Tailnet 페이지의 정확한 runtime marker는 한 번 나타난다. | 통과 |
| 29 | global docs build 실패를 성공으로 보고하는가 | 전체 build는 기존 unrelated dead links와 `rust,ignore` Shiki 오류로 실패했다. 선택 route build만 성공으로 구분한다. | 통과 |
| 30 | Tailnet sync가 active server를 재시작하거나 기존 target을 삭제하는가 | PID 28040을 유지하고, 바뀐 14개 파일을 backup한 뒤 111 route의 local/remote 222 response가 전부 HTTP 200 및 SHA-256 일치했다. 삭제는 0건이다. | 통과 |

## 검토 결과

30개 실패 관점에서 공개 문구, 제한 범위, image 경로와 Tailnet 제공 hash 사이에 남은 불일치는 없다. 첫 sync 후보의 동일 basename 충돌은 파일 쓰기 전에 거부됐고, 경로 prefix를 바로잡은 뒤 두 번째 화면 검토에서 좁은 column 배치를 발견해 전체 폭으로 수정했다. 최종 Agent Browser 화면은 별도 evidence screenshot으로 보존한다.
