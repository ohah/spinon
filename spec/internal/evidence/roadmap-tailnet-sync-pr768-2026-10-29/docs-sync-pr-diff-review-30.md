# PR #768 병합 후 문서·Tailnet diff 실패 관점 검토 30개

검토 대상은 구현 PR #768의 병합 상태 기록, 공식 상태 대장, 내부 Grid 계약, 공개 roadmap card와 Tailnet 미리보기 동기화 evidence다. C11.4 전체를 완료 처리하거나 공식 완료 수를 올리는 변경은 포함하지 않는다.

| # | 별도 실패 경로 | 확인 및 결과 |
|---:|---|---|
| 1 | 구현 PR 번호가 계획 PR 번호와 혼동된다. | 계획 PR #767과 구현 PR #768을 각 링크와 역할에 맞게 분리했다. 통과. |
| 2 | 잘못된 rebase merge SHA가 기록된다. | GitHub PR `mergeCommit.oid`와 local `origin/main` HEAD가 `7c004e8f5ed3197d696abcc8f84bcace83b97fe2`로 일치한다. 통과. |
| 3 | 병합 시각을 local timezone으로 바꾸어 오기한다. | GitHub `mergedAt`의 UTC 시각 `2026-10-28T18:41:57Z`를 사용했다. 통과. |
| 4 | merge method가 squash/merge commit으로 서술된다. | PR은 repository가 허용하는 Rebase and merge로 완료됐다. 통과. |
| 5 | 기록한 구현 PR의 base/head가 다른 브랜치와 연결된다. | PR #768의 base `main`, head `feat/c11-4-fit-content-spanning-wpt-20261029`를 확인했다. 통과. |
| 6 | 단일 fixture 결과가 C11.4 전체 완료로 바뀐다. | `spec/STATUS.md`의 C11.4 checkbox는 unchecked 상태다. 통과. |
| 7 | 공식 완료 집계가 근거 없이 증가한다. | 상태 대장과 public roadmap 모두 `72/213`을 유지한다. 통과. |
| 8 | 상태 대장의 단일 장면 설명이 실제 fixture 증거를 놓친다. | STATUS 항목이 계획과 final runtime evidence를 연결한다. 통과. |
| 9 | 계획 문서가 구현 결과를 merge 상태와 분리하지 못한다. | 계획 상단에 구현 PR merge SHA와 체크 경계를 명시했다. 통과. |
| 10 | 계획의 WPT source가 다른 revision으로 바뀐다. | 계획에 기록된 pinned revision `c999c58338ee1d223df5ad62e48be1202ced0d37`을 유지했다. 통과. |
| 11 | 독립 기대값이 runtime 결과로 사후 보정된다. | WPT/CRD 기반 `[40,40]px` expected value와 simulator 결과를 구분했다. 통과. |
| 12 | Android AVD가 물리 기기나 hardware GPU로 확대 해석된다. | roadmap/evidence는 API 37 ARM64 AVD와 PixelCopy 경계로 제한한다. 통과. |
| 13 | iOS drawable sample이 compositor 이후 최종 화면으로 확대된다. | iOS sample을 Metal drawable의 compositor 제출 전 결과로 표기했다. 통과. |
| 14 | 내부 계약 숫자 버전이 제한 slice 때문에 증가한다. | 계약은 `0.1.0`을 유지한다. 통과. |
| 15 | 제한 opt-in fixture가 일반 author CSS API를 추가한 것으로 읽힌다. | 계약과 실행 evidence가 새 공개 API 및 일반 CSS admission을 부인한다. 통과. |
| 16 | runtime README가 다른 simulator build 또는 app hash를 가리킨다. | README의 Android/iOS hash와 final run directory를 각각 대조했다. 통과. |
| 17 | static archive boundary scan을 APK/IPA 전체 검사로 확대한다. | README는 정적 FFI archive 검사이며 APK/IPA exclusion 검사가 아니라고 적는다. 통과. |
| 18 | PR check rollup이 비어 있는 것을 CI 성공으로 기록한다. | PR `statusCheckRollup`과 `gh pr checks --required`는 결과 없음이며 CI 통과로 표현하지 않는다. 통과. |
| 19 | required check이 없는 이유를 확인하지 않는다. | main branch protection에서 required status checks가 설정되지 않은 것을 확인했다. 통과. |
| 20 | merge state를 병합 직전 상태와 혼동한다. | 병합 직전 PR #768은 `CLEAN`; merged 이후 `mergeStateStatus`는 `UNKNOWN`일 수 있음을 분리한다. 통과. |
| 21 | public roadmap에서 최신 C11.4 항목이 오래된 기록 아래에 숨는다. | PR #768 항목을 기존 #747 뒤이자 다음 단계 섹션 앞에 추가했다. 통과. |
| 22 | public roadmap의 PR link/merge SHA가 내부 기록과 다르다. | roadmap card가 PR #768 및 동일 merge SHA를 가리킨다. 통과. |
| 23 | roadmap note가 작업 날짜와 GitHub merge UTC를 하나로 합친다. | 항목 표기일과 parenthetical UTC merge 시각을 각각 유지한다. 통과. |
| 24 | roadmap의 frame/track 설명이 implementation evidence와 달라진다. | `[40,40]px` tracks, `60×80px` item, `60×200px` child를 evidence README와 대조했다. 통과. |
| 25 | roadmap에서 Tailnet evidence link가 빠지거나 잘못된 route를 가리킨다. | `/spinon/internal/evidence/roadmap-tailnet-sync-pr768-2026-10-29/README.html` 링크를 연결했다. 통과. |
| 26 | Tailnet URL이 다른 listener나 server root를 가리킨다. | `tailscale serve status`에서 기본 host route가 `127.0.0.1:8768`로 연결되고, listener가 `SpinonPreview/tailnet-preview-server.mjs`에서 실행 중임을 확인했다. manifest의 `previewRoot`도 동일하다. 통과. |
| 27 | preview에 복사한 HTML이 현재 merged source에서 생성되지 않는다. | Rspress 2.0.22 selected-root build가 완료됐고, manifest에 170개 대상의 bytes/SHA-256을 기록했다. served hash check에서 선택 파일과 manifest 두 route, 총 172개가 HTTP 200 및 byte/hash 일치, 실패 0건이다. 통과. |
| 28 | 기존 preview 파일을 backup하지 않은 채 덮어쓴다. | pass02 manifest의 모든 `replace` entry에 사전 SHA-256·mode·backup path가 있고 backup 파일의 byte/hash를 원본 사전 값과 대조했다. 이전 pass01 backup도 그대로 보존했다. 통과. |
| 29 | 동기화 중 기존 미선택 경로나 파일을 삭제한다. | manifest `noDeletes=true`, `deletionPaths=[]`이며 script에는 delete/unlink 단계가 없다. 기존 preview 파일 삭제는 수행하지 않았다. 통과. |
| 30 | 사용자에게 보이는 Tailnet page가 오래되었거나 검증 브라우저가 남는다. | Agent Browser에서 roadmap 제목과 PR #768 카드의 viewport 가시성을 확인하고 screenshot을 저장했다. STATUS의 C11.4 checkbox는 unchecked이고 공식 표시는 `72/213`이다. 종료 후 active session과 Chrome/Agent Browser 프로세스가 없다. 통과. |

26–30번은 preview route, served response, backup, 비삭제 상태와 Agent Browser 종료를 각각 확인했다.
