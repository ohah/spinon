# C11.4 PR #695 병합 후 상태·roadmap PR diff 적대 검토 30개

검토 대상은 PR #695 병합 뒤 `spec/STATUS.md`, `spec/0001-conformance.md`, 집중 계획/runtime 근거, `spec/public/roadmap.html`에 추가한 후속 문서 diff다. 구현의 완료 상태와 public roadmap 표시가 서로 일치하는지, 잘못된 병합 정보·범위 확대·깨진 경로가 생기는지 30개의 별도 실패 가설로 확인했다.

| # | 실패 가설 | diff에서 확인한 근거·판정 |
|---:|---|---|
| 1 | PR 번호나 merge commit을 잘못 기록한다. | `gh pr view 695` 결과와 STATUS, conformance, plan, roadmap의 PR #695·merge SHA `d6dd9fd15d2897804142e6bb119149a633952a9b`가 일치한다. 통과. |
| 2 | merge 시각을 local timezone과 UTC 사이에서 잘못 옮긴다. | GitHub `mergedAt=2026-10-27T14:39:47Z`의 UTC 표기를 네 문서에서 그대로 사용한다. 통과. |
| 3 | 상위 C11.4를 완료로 바꾼다. | `spec/STATUS.md`의 상위 `[ ] C11.4 intrinsic track sizing`은 그대로이며 새 줄은 그 아래 runtime slice 이력이다. 통과. |
| 4 | 완료 집계를 72/213에서 올린다. | 새 항목은 checkbox가 없는 증거 기록이다. 모든 추가 문구는 `72/213` 유지와 미완료를 적는다. 통과. |
| 5 | `grid-auto-fit-columns-001.html` slice를 전체 C11.4 지원처럼 표시한다. | PR 요약은 exact pinned scene fixture라 명시하고 일반 CSS admission, full WPT, Stage B 및 전체 C11.4를 제외한다. 통과. |
| 6 | WPT 35/35 기준을 Spinon 35/35로 읽게 한다. | Chrome harness baseline과 Android/iOS runtime scene assertion을 별도 문서로 분리한다. roadmap 카드에는 앱 두 행렬만 35/35로 적었다. 통과. |
| 7 | 37 direct item을 scene count에 포함하거나 별도 장면처럼 센다. | STATUS/runtime 기록은 35 scenes와 37 direct items를 서로 다른 단위로 표기한다. 통과. |
| 8 | Android simulator 결과를 실기기 또는 hardware GPU로 확장한다. | Android 환경을 API 37 ARM64 AVD로 제한하고 hardware GPU/physical device는 미검증으로 남겼다. 통과. |
| 9 | iOS readback을 compositor 이후 화면으로 부른다. | 모든 후속 문구가 Metal drawable을 compositor 제출 전으로 명시하며 post-compositor pixels를 제외한다. 통과. |
| 10 | overflow 표본 24/24를 scene 수나 한 플랫폼 합계로 혼동한다. | 문서는 여섯 scene × 표본의 24/24를 Android와 iOS 각각으로 구분한다. 통과. |
| 11 | 전체 35-scene draw/present가 screenshot 캡처 횟수라고 과장된다. | 두 runtime matrix에서 각 35 scene draw/present를 확인한 자료를 링크하고 screenshot은 대표 장면으로만 둔다. 통과. |
| 12 | PR check rollup이 비어 있는데 CI 성공으로 단정한다. | status/conformance/plan/roadmap 및 PR diff review 모두 status check 미보고를 적고 pass로 간주하지 않는다. 통과. |
| 13 | app build나 focused check 설명에 근거 링크가 없다. | STATUS와 conformance에서 실행 README로 연결하고 README의 명령·로그 표에 결과를 구체적으로 둔다. 통과. |
| 14 | static FFI archive 검사를 Android APK/iOS IPA 전체 exclusion으로 일반화한다. | runtime README는 macOS ARM64 static archive만 확인했고 APK/IPA는 미검증이라고 한정한다. 통과. |
| 15 | official STATUS 새 링크가 잘못된 relative base를 쓴다. | STATUS 기준으로 집중 계획은 `../plan`, evidence는 `internal/evidence`; 대상 파일이 모두 존재한다. 통과. |
| 16 | conformance 항목이 적합성 scenario catalog 안에 들어간다. | 병합 이력은 기존 C11.4 evidence paragraph 다음, `## 시나리오 카탈로그`보다 앞에 놓였다. 통과. |
| 17 | conformance와 STATUS에서 한쪽만 C11.4 완료라고 표시한다. | 두 문서 모두 opt-in slice, 제한 범위, 상위 checkbox 및 `72/213` 유지가 일치한다. 통과. |
| 18 | 집중 계획이 PR #695를 아직 열린 상태라고 남긴다. | 상태 문구를 Rebase and merge 시각·merge SHA와 no-check 기록으로 갱신했다. 통과. |
| 19 | README의 실행 당시 source HEAD를 실제 commit SHA처럼 둔다. | README는 base HEAD `e37e076`과 당시 dirty-tree 실행을 구분하고, implementation commit `d4e9cd7` 및 merge SHA `d6dd9fd`를 별도로 기록한다. 통과. |
| 20 | final PR head와 구현 diff 범위를 혼동한다. | PR review ledger는 implementation commit `d4e9cd7`와 final PR head `6fa3b18`을 구분하고 후속 문서 commit의 별도 check을 적는다. 통과. |
| 21 | PR #695의 screenshot이 잘못된 splash capture를 가리킨다. | PR 및 roadmap scene 07은 verified final Android run `141559Z`와 iOS `140118Z` 파일을 가리킨다. superseded Android `141144Z`는 별도 표기한다. 통과. |
| 22 | public roadmap이 simulator screenshot link를 다른 route로 만든다. | 두 image route와 plan/evidence HTML routes가 source tree의 실제 파일 stem 및 Rspress `.html` mapping과 일치한다. 통과. |
| 23 | 공개 roadmap 신규 카드의 HTML closing tag가 어긋난다. | 카드는 hero-progress section 안, section closing 바로 전에 삽입했다. diff whitespace check 및 HTML parser 검사가 통과했다. |
| 24 | HTML 텍스트의 `&`, `<` 또는 인용 부호가 markup을 깨뜨린다. | 특수 구문은 `<code>`·`<a>` markup으로 감싸고 text node에 raw tag syntax를 넣지 않았다. HTML parse가 통과했다. |
| 25 | 새 카드가 공용 roadmap의 상위 진행 중 항목을 바꾼다. | `execution.inProgress`와 `inProgress: C11.4` 계산 코드는 수정하지 않았다. 진행 항목 표시는 그대로다. 통과. |
| 26 | 공식 count를 JS literal로 고정해 STATUS fetch와 달라진다. | 신규 카드는 표시 근거를 기록할 뿐 fetch/count code나 base task 배열을 건드리지 않는다. 72/213 유지가 명시돼 있다. 통과. |
| 27 | 새 카드가 PR #695 화면의 이미지를 외부 임의 출처로 끌어온다. | image href는 저장소의 두 simulator PNG route를 사용하며 별도 image host를 추가하지 않았다. 통과. |
| 28 | docs PR 자체가 Tailnet 파일을 merge 전에 덮어쓴다. | 이 PR은 source/docs만 변경한다. preview target은 Tailnet preflight에서 이전 hash로 확인했고 sync는 PR merge 뒤에만 한다. 통과. |
| 29 | HTML source sync가 전체 Pages/site deployment로 확대된다. | 변경 대상은 `spec/public/roadmap.html` 하나이며 Github Pages나 site build/deploy workflow를 실행하지 않는다. 통과. |
| 30 | 후속 문서 PR의 미완료 항목이나 PR check 상태를 숨긴다. | PR body와 plan은 Tailnet sync를 남은 일로, GitHub check을 미보고 상태로 분리한다. 이 기록은 check pass를 주장하지 않는다. 통과. |

**병합 후 상태·roadmap PR diff 검토:** 위 30개의 서로 다른 failure hypothesis를 현재 docs diff와 병합 근거에 대조했다. C11.4 checkbox·`72/213`에는 변경이 없다. Tailnet preview는 이 PR merge 뒤 단일 `roadmap.html` route만 별도 preflight·backup·atomic replacement·Agent Browser 확인으로 동기화한다.
