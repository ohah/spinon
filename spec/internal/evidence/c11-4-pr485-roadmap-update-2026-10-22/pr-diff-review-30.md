# PR #485 병합 기록 및 Tailnet roadmap 문서 diff 검토 30개

PR #485 구현 diff 검토와 별도로, merge 기록·공개 roadmap 항목·상태 대장 연결·문서 경로를 각각 교차 대조했다. GitHub PR 정보와 현재 `origin/main`을 기준으로 확인했다.

| # | 별도 실패 관점 | diff 대조 및 결과 |
| ---: | --- | --- |
| 1 | roadmap에 다른 PR의 merge SHA를 복사하는가 | PR #485의 GitHub `mergeCommit.oid`와 새 roadmap note의 SHA가 모두 `6a1fa412185a5707decdf112716b0dcbad2106fa`다. |
| 2 | merge 시간이 PR 이벤트 시각과 다르게 적히는가 | GitHub `mergedAt`의 `2026-10-22T00:02:34Z`를 UTC로 표기했다. |
| 3 | 상태 대장과 계획이 서로 다른 PR 번호를 가리키는가 | 두 문서 모두 PR #485와 같은 merge SHA를 가리킨다. |
| 4 | PR의 merge method를 squash 또는 merge commit으로 잘못 기록하는가 | PR 상태는 Rebase and merge로 기록했다. |
| 5 | 현재 main HEAD와 merge SHA가 불일치하는데 현재 tip처럼 설명하는가 | 병합 확인 시 `origin/main` HEAD가 같은 SHA였다. 계획은 병합 시점의 main 일치 사실로 한정한다. |
| 6 | PR 체크가 비었는데 통과한 CI가 있다고 주장하는가 | PR 상태 rollup과 `gh pr checks --required` 모두 체크 없음으로 확인했다. 성공한 CI라고 쓰지 않았다. |
| 7 | 보호 규칙 API 404를 다른 오류로 숨기는가 | GitHub가 main required status checks 비활성이라고 반환한 사실을 그대로 기록했다. |
| 8 | 네 scene 중 definite control을 빠뜨리는가 | cyclic column·row와 definite 40px·10px 두 control을 함께 명시한다. |
| 9 | 한 축 결과를 두 축 모두 검증했다고 과장하는가 | track 결과는 fixed scene 범위로만 기술하고 전체 Grid 축 조합 지원이라고 쓰지 않는다. |
| 10 | PixelCopy와 Metal 표본의 수를 장면 수로 혼동하는가 | 4/4는 scene 수, 9/9는 각 플랫폼 pixel sample 수로 분리해 표기했다. |
| 11 | iOS 표본이 compositor 이후 capture인 것처럼 보이는가 | roadmap과 `STATUS`에서 제출 전 Metal drawable이라고 명시했다. |
| 12 | Android API 37 AVD를 실기기라고 부르는가 | Android 환경은 ARM64 AVD로 명시한다. |
| 13 | 시뮬레이터 결과에서 hardware GPU를 유추하는가 | hardware GPU는 미검증 범위로 남겼다. |
| 14 | 글리프 표본을 border stroke 테스트로 오해하는가 | border stroke가 검증되지 않았다고 roadmap 및 상태 대장에 기록한다. |
| 15 | 글자 overflow 표본이 item/root 둘 다 벗어났는가 | README의 small-limit scene에서 item 9px/root 10px 및 `(15,5)` 표본으로 확인한다. |
| 16 | 글리프가 표면 끝까지 칠해졌다고 부정확하게 주장하는가 | text 바깥 `(25,5)` black 표본과 scope README에 맞춰 text 끝 바깥에서 멈춘다고 기록한다. |
| 17 | 기본 artifact에서 fixture가 빠진다는 결과를 앱 bundle 검사로 확대하는가 | PR의 FFI archive 경계 근거를 그대로 제한적으로 설명하고 앱 bundle 크기 주장은 하지 않는다. |
| 18 | fixture-test positive control 존재를 기본 제품 포함으로 거꾸로 설명하는가 | 기본 archive marker 부재와 opt-in positive control의 네 ID 포함을 분리해 적는다. |
| 19 | 일반 CSS 지원이 추가됐다고 읽히는가 | fixed allowlist scene이고 일반 CSS admission은 바꾸지 않았음을 명시한다. |
| 20 | C11.4의 일부 scene을 전체 항목 완료로 표시하는가 | `STATUS` checkbox는 unchecked이며 roadmap에도 C11.4 전체 미완료를 명시한다. |
| 21 | 완료 수가 기존 72/213에서 잘못 변하는가 | roadmap, 상태 대장, 계획 모두 공식 완료 수 `72/213`을 유지한다. |
| 22 | roadmap의 새 note가 이전 PR #480보다 아래에 묻히는가 | 새 PR #485 note를 #480보다 앞에 삽입해 최신 기록으로 보이게 했다. |
| 23 | HTML `<code>`와 anchor가 본문 text를 깨뜨리는가 | 새 note는 기존 단일 paragraph 구조를 따르고 `<code>`·`<a>` 태그를 각각 닫는다. |
| 24 | roadmap 집중 계획 href가 산출 페이지와 다른 slug를 쓰는가 | href는 tracked `plan/c11-4-cyclic-fit-content-calc.md`의 RSPress route와 일치한다. |
| 25 | 실행 근거 링크가 실제 tracked README와 맞지 않는가 | evidence README 경로와 파일명이 새 runtime evidence 디렉터리와 일치한다. |
| 26 | Android 또는 iOS 이미지 링크가 오래된 실패 capture를 가리키는가 | 두 링크 모두 최종 `reviewed` run 디렉터리의 small-limit PNG를 가리킨다. |
| 27 | 구현/runtime review 링크의 경로를 잘못 붙이는가 | markdown route는 실제 `implementation-review-20.md` 상대 경로와 일치한다. |
| 28 | PR diff review 링크가 순환적으로 다른 파일을 가리키는가 | 공개 roadmap에서 #485 구현 PR의 `pr-diff-review-30.md`를 별도로 연결한다. |
| 29 | status에 PR link와 review가 누락되어 계획·공개 페이지와 끊기는가 | `STATUS.md`에 #485와 PR diff review를 연결하고 계획에는 merge SHA와 체크 상태를 둔다. |
| 30 | 최신 화면 기록이 branch URL만 가리켜 merge 뒤 사라지는가 | roadmap의 PR 링크는 stable PR URL이고 실행 화면·문서는 source repo의 tracked 경로를 사용한다. |

**결과:** 이 30개 문서 diff 관점에서 잘못된 merge metadata나 깨진 source path를 찾지 못했다. public roadmap 수정 뒤 Tailnet 선택 build와 브라우저 렌더링을 별도로 확인한다. GitHub Pages는 배포하지 않는다.
