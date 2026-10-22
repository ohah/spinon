# PR #493 로드맵 반영 변경 적대 검토 30개

검토 대상은 PR #493 병합 결과를 `spec/STATUS.md`, 두 C11.4 계획 문서, 공개 roadmap HTML에 반영하는 문서 변경이다. 기능의 새 구현이나 C11.4 완료 판정은 이 PR 범위에 포함하지 않는다. GitHub 병합 정보, 저장소 실행 근거, 링크 대상과 HTML 삽입 지점을 대조했다.

| # | 독립 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | PR 번호를 직전 #491이나 #480으로 잘못 표기할 수 있다 | 새 카드와 상태 문구가 PR #493을 가리키고 GitHub PR metadata의 번호와 일치한다. 통과. |
| 2 | 병합 시각을 로컬 시간이나 push 시각과 혼동할 수 있다 | GitHub `mergedAt`의 `2026-10-22T02:41:32Z`를 UTC로 기록했다. 통과. |
| 3 | head commit을 merge commit으로 적을 수 있다 | `mergeCommit.oid` `3ffd2b5a39f27dc45d8235bdf830678e4d3c0a15`와 대조했다. 통과. |
| 4 | 부분 SHA의 오타가 원본 SHA를 가릴 수 있다 | roadmap, STATUS, 두 계획 문서의 전체 SHA가 같은지 확인했다. 통과. |
| 5 | GitHub check rollup이 비었는데 CI 통과라고 과장할 수 있다 | `statusCheckRollup`은 빈 배열이고 문구는 “비어 있었다”고 한정한다. 통과. |
| 6 | branch protection 조회 실패를 required checks 통과로 읽을 수 있다 | `main` status-check protection API는 404 `Required status checks not enabled`; 문구는 설정되지 않았다고 기록한다. 통과. |
| 7 | C11.4 slice를 전체 완료로 오해할 수 있다 | 모든 새 기록이 고정 fixture slice와 전체 미완료 경계를 명시한다. 통과. |
| 8 | 공식 완료 수가 72/213에서 임의로 바뀔 수 있다 | 상태 대장과 공개 카드에 72/213을 유지한다. 통과. |
| 9 | C11.4 checkbox를 완료 처리할 수 있다 | 상태 대장 checkbox는 변경하지 않고 미완료를 명시한다. 통과. |
| 10 | 두 축 최종 track 기대값을 서로 뒤바꿀 수 있다 | 양축 cyclic `[104]/[22]`와 계획 표를 대조했다. 통과. |
| 11 | column-only control 값을 row-only와 혼동할 수 있다 | `[104]/[10]` 표기가 상세 계획의 column-only 장면과 일치한다. 통과. |
| 12 | row-only wrapping 결과가 intrinsic basis와 어긋날 수 있다 | `[10]/[42]`가 상세 계획의 20px wrapping basis 계산과 일치한다. 통과. |
| 13 | definite control 값이 과거 slice 값으로 복사될 수 있다 | `[24]/[22]`를 네 장면 상세 표와 대조했다. 통과. |
| 14 | AVD 실행을 실기기 검증으로 넓혀 쓸 수 있다 | Android API 37 ARM64 AVD라고 특정하고 실기기 미검증을 남긴다. 통과. |
| 15 | iOS Simulator 결과를 실기기 결과로 넓혀 쓸 수 있다 | iPhone 17 Pro / iOS 26.2 Simulator로 범위를 한정한다. 통과. |
| 16 | draw/present 통과를 compositor 이후 pixel 검증으로 오해할 수 있다 | iOS 표본을 compositor 제출 전 Metal drawable로 명시한다. 통과. |
| 17 | 표본 수 28/28이 scene 수와 섞일 수 있다 | scene은 플랫폼당 4/4, pixel 표본은 플랫폼당 28/28로 구분한다. 통과. |
| 18 | PixelCopy 결과를 Android 화면 전체의 시각적 정확성으로 확대할 수 있다 | 설명은 고정 fixture의 표본과 trace로 제한하며 일반 CSS/WPT 미검증을 남긴다. 통과. |
| 19 | border가 존재한다는 사실을 border stroke 색 검증으로 오해할 수 있다 | 전체 C11.4 지원과 CSS border stroke를 확인했다고 주장하지 않는다. 통과. |
| 20 | static archive 문자열 검사를 APK/IPA 전체 제외 증명으로 확장할 수 있다 | 기본 archive와 opt-in positive control만 말하며 앱 전체 제외는 아니라고 기록한다. 통과. |
| 21 | fixture 결과를 일반 author CSS 지원으로 게시할 수 있다 | roadmap, 상태, 상세 계획에 일반 CSS admission 미검증을 유지한다. 통과. |
| 22 | 실행 근거 링크가 다른 slice 결과를 가리키거나 RSPress가 JSON을 문서 route로 오인할 수 있다 | README의 plan URL, Tailnet JSON evidence 링크와 새 카드의 README·precomparison 경로를 확인했다. 증거 폴더/파일이 양축 minimum slice와 일치한다. 통과. |
| 23 | 구현 PR diff 리뷰와 문서 PR diff 리뷰를 같은 문서로 연결할 수 있다 | 각각 `pr-diff-review-30`과 `roadmap-pr-diff-review-30`의 별도 경로를 둔다. 통과. |
| 24 | Android 이미지 경로가 임시 또는 실패 실행일 수 있다 | 링크는 최종 통과 AVD 디렉터리 `20261022T022619123Z-93c2e1cb`를 가리키고 파일 존재를 확인했다. 통과. |
| 25 | iOS 이미지 경로가 임시 또는 실패 실행일 수 있다 | 링크는 최종 통과 Simulator 디렉터리 `20261022T022743674Z-063697ed`를 가리키고 파일 존재를 확인했다. 통과. |
| 26 | roadmap 링크의 `/spinon/` 접두 경로가 빠질 수 있다 | 새 카드의 내부 링크가 기존 공개 roadmap 경로 규칙을 따른다. 통과. |
| 27 | `STATUS.html#c11-css-grid`가 현재 C11 항목과 연결되지 않을 수 있다 | 기존 roadmap의 공식 상태 링크와 같은 fragment를 쓴다. 통과. |
| 28 | 새 카드를 잘못된 지점에 넣어 #491 및 선행 기록을 가릴 수 있다 | 새 #493 카드는 기존 #491 카드 바로 앞에 삽입해 최신 항목으로 읽힌다. 통과. |
| 29 | HTML 삽입으로 기존 카드의 `<p>` 경계나 링크를 손상할 수 있다 | 고유 anchor 1회 확인 후 새 `<p>` 전체를 삽입하며 기존 #491 paragraph를 보존한다. 통과. |
| 30 | 문서 갱신이 Pages 배포나 상태 변경을 암묵적으로 포함할 수 있다 | PR은 문서·검토 기록만 다루며 Pages 배포와 상태 checkbox/API 버전 변경은 포함하지 않는다. PR 본문의 Android/iOS 이미지도 Agent Browser에서 각각 원본 1080×2400·1206×2622로 로드됐고 고유 session과 background process 종료를 확인했다. 통과. |

근거: PR #493의 GitHub `mergedAt`, `mergeCommit`, `statusCheckRollup`, `main` required status-check 설정 조회; [상태 대장](../../../STATUS.md); [slice 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-cyclic-minmax-calc-minimum-two-axis.md); [상위 Grid 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-grid-track-sizing.md); [Simulator 실행 근거](./README.md)와 해당 문서가 연결한 PNG들. `mise exec -- bun run docs:build`도 실행했다. 전체 빌드는 저장소의 기존 여러 dead link 오류로 종료됐지만 이 검토 파일과 runtime README에서는 오류가 더 이상 보고되지 않는다. PR #494 이미지·브라우저 종료 결과는 [Agent Browser 확인 기록](./agent-browser-pr-494-verification.json)에 남겼다. PR #494 병합 후 Tailnet 공개 roadmap의 반영 여부는 별도 Agent Browser 확인 대상으로 둔다.
