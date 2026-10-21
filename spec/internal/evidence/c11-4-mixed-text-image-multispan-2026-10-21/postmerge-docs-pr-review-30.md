# C11.4 구현 병합 후 roadmap 문서 PR 적대 검토 30개

**대상:** 구현 PR #419의 병합 상태를 계획·공식 상태·공개 roadmap에 반영하는 문서 변경
**검토 기준:** 2026-10-21, #419 병합 commit `1e2c5352ff9932a776420fdf533c9d040e4a0746`
**결과:** 서로 다른 30개 실패 관점을 적용했다. 이 문서는 이 변경의 PR diff 검토이며, 기존 계획·구현/runtime 검토를 대신하지 않는다.

| # | 적대 관점 | 확인 |
|---:|---|---|
| 1 | GitHub가 #419를 아직 열림으로 표시하는가 | PR API의 `state=closed`, `merged_at=2026-10-21T01:25:11Z`를 확인했다. 병합 상태와 일치한다. 통과. |
| 2 | 병합 시간이 계획·상태 문서에서 다르게 적히는가 | 두 문서 모두 `2026-10-21 01:25:11 UTC`다. 통과. |
| 3 | 계획·상태의 merge SHA가 GitHub와 다른가 | PR API와 Git 객체 모두 `1e2c5352ff9932a776420fdf533c9d040e4a0746`다. 통과. |
| 4 | 현재 기준점이 병합 뒤 `main`과 어긋나는가 | 작업 checkout의 `HEAD`와 `origin/main`이 같은 merge SHA다. 통과. |
| 5 | Rebase and merge 기록에 불필요한 merge commit이 끼었는가 | 해당 commit은 부모가 하나인 선형 commit이다. 통과. |
| 6 | PR check 결과가 문서의 표현보다 더 강한가 | PR의 `statusCheckRollup`은 빈 배열이다. 통과했다고 표기하지 않았다. 통과. |
| 7 | 필수 check가 없다는 표현이 branch 보호 설정과 충돌하는가 | `main`의 required status checks API는 `Required status checks not enabled`를 반환했다. 통과. |
| 8 | 병합 후에도 계획에 check 대기 문장이 남았는가 | 변경한 계획·상태 행에 `체크를 기다린다`나 `pending` 문구가 없다. 통과. |
| 9 | 계획 상단이 여전히 PR #419를 미병합으로 설명하는가 | 계획 상단은 병합 시각과 SHA를 적는다. 통과. |
| 10 | 제한 구현 PR 병합을 C11.4 완료로 과장하는가 | 계획 상단은 C11.4와 공식 완료 수가 미완료라고 유지한다. 통과. |
| 11 | 공식 상태 행의 체크가 실수로 완료 처리됐는가 | C11.4 상위 checkbox와 완료 수를 그대로 두었다. 통과. |
| 12 | 문서 갱신이 다른 체크박스 상태를 바꾸는가 | diff는 계획·상태의 설명 행만 바꾸고 checkbox 문자를 변경하지 않는다. 통과. |
| 13 | 공식 상태 대장의 전체 체크 수가 변했는가 | `STATUS.md` 전후가 체크 87개·미완료 139개로 같다. 이 수는 세부 checkbox까지 포함한다. 통과. |
| 14 | 전체 checkbox 수를 공개 roadmap 집계와 혼동하는가 | Agent Browser의 읽기 전용 보드는 GitHub `main` 원본에서 공식 완료 `72/213`을 표시한다. 통과. |
| 15 | 보드 분모 213의 구성이 불명확한가 | 같은 화면에서 상위 78개, 세부 ID 133개, ID 없는 하위 체크 2개를 확인했다. 통과. |
| 16 | C11.4 문단에서 지원 범위를 일반 CSS Grid로 넓히는가 | 문단은 네 고정 scene과 direct-leaf text/image로 제한한다. 통과. |
| 17 | Ahem text와 PNG image가 같은 실행에 들어갔다는 근거가 빠졌는가 | roadmap·계획은 두 source 종류가 한 intrinsic revision에 함께 참여한다고 일치하게 기록한다. 통과. |
| 18 | scene 개수가 실행 자료와 다른가 | 공개 문단은 네 scene이며 실행 README의 네 scene 이름과 일치한다. 통과. |
| 19 | span-2 track 기대값이 틀렸는가 | 공개 문단의 `[25,36,36,0]`이 계획과 실행 README의 값에 맞는다. 통과. |
| 20 | span-3 track 기대값이나 순서가 달라졌는가 | `[158/3,191/3,191/3,0]`과 span-2 뒤 span-3 순서를 계획·README와 대조했다. 통과. |
| 21 | DOM order 대조를 안정성 검증으로 과장하는가 | forward/reversed 네 scene을 언급하지만 임의 순서·모든 item 조합으로 확대하지 않는다. 통과. |
| 22 | Android AVD를 실기기 근거로 오인하는가 | API 37 ARM64 AVD와 SM-S731N 실기기를 별도 대상으로 적었다. 통과. |
| 23 | 실기기 모델·OS가 실행 근거와 다른가 | Samsung SM-S731N Android 16/API 36 표기가 계획·상태·README에서 일치한다. 통과. |
| 24 | iOS Simulator를 iOS 실기기로 오인하는가 | iPhone 17 Pro/iOS 26.2 Simulator로 명시한다. 통과. |
| 25 | 실행 성공을 실제 화면 표시 픽셀 성공으로 넓히는가 | frame·track·trace·draw/present 결과만 주장하며 PixelCopy/Metal pixel은 미확인이라고 쓴다. 통과. |
| 26 | 이미지 source 확인을 PNG texel paint 검증으로 오해하는가 | PNG texel painting 미검증을 공개 문단과 계획 한계에 남긴다. 통과. |
| 27 | 앱 화면의 background overflow를 CSS border stroke로 오해하는가 | CSS border 전체 검증 미완료를 명시한다. 통과. |
| 28 | 고정 fixture를 WPT나 전체 C11.4 완료로 표시하는가 | WPT와 전체 C11.4/CSS Grid 미완료를 명시한다. 통과. |
| 29 | markdown source 폴더에 있는 PNG 링크가 배포에서 빠지는가 | 첫 상대 링크는 RSPress가 정적 파일로 복사하지 않아 404였다. 두 캡처를 `public/internal/evidence` 정적 route로 연결해 수정하고 최종 선택 빌드의 두 이미지 URL을 Agent Browser에서 다시 확인했다. 통과. |
| 30 | 공개 roadmap 화면이 잘못된 문단·집계를 표시하는가 | Agent Browser에서 #419 병합 문단·세 링크·SHA 및 별도 72/213 집계 화면을 확인하고 두 screenshot을 저장했다. 통과. |

## 빌드 한계

저장소 전체 `mise exec -- bun run docs:build`는 기존 evidence Markdown 여러 곳의 dead link에서 실패했다. 예를 들어 C11 iOS Simulator 근거의 JSON 링크와 C11.4 artifact-boundary의 로그·스크립트 링크가 RSPress의 `.html` route 검사에서 실패한다. 이 PR은 그 파일을 수정하지 않는다. 변경된 네 route는 고정 RSPress 2.0.22 선택 빌드에서 생성했고 Agent Browser에서 직접 확인했다. 전체 문서 빌드가 통과했다고 주장하지 않는다.

선택 빌드는 이 문서 작업의 route 확인용이다. 공식 상태 수의 원본은 계속 `spec/STATUS.md`이고 roadmap 보드는 GitHub `main` 상태를 읽기 전용으로 표시한다.
