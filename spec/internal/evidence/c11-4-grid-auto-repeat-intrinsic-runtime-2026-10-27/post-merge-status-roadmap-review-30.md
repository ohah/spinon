# C11.4 PR #691 병합 후 상태 문서 검토 30개

검토 대상은 PR #691 병합 결과를 `spec/STATUS.md`, 계획, 내부 계약과 실행 README에 반영한 문서 diff다. 구현/runtime 및 PR diff 검토를 재사용하지 않고, 출처·수치·시점·주장 범위의 불일치 경로를 별도로 대조했다. 단일 수동 검토이며 외부 reviewer를 주장하지 않는다.

| # | 별도 실패 관점 | 판정 및 확인 근거 |
|---:|---|---|
| 1 | 다른 C11.4 PR의 상태를 #691 상태로 잘못 적음 | 통과. 문서 링크와 PR 번호가 모두 구현 PR #691을 가리킨다. |
| 2 | PR 공개 상태가 아직 open으로 남음 | 통과. GitHub에서 `MERGED`를 확인한 뒤 계획·README·상태 대장을 모두 병합 상태로 바꿨다. |
| 3 | UTC 병합 시각이 API 값과 다름 | 통과. `2026-10-27T10:30:12Z`가 `gh pr view`의 `mergedAt` 값과 일치한다. |
| 4 | Rebase and merge와 다른 병합 방식을 기록함 | 통과. PR 기록과 main history의 두 선형 commit을 대조했다. |
| 5 | PR head SHA를 main merge SHA라고 잘못 부름 | 통과. 최종 main SHA `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`를 PR mergeCommit 및 `origin/main` history와 대조했다. |
| 6 | 앞선 구현 commit의 main SHA가 혼동됨 | 통과. main history의 feature commit `1b8036717a70290b4a398746c7f2a00458a4aeaf`와 후속 문서 commit을 구분했다. |
| 7 | GitHub check가 없는데 통과했다고 주장함 | 통과. `gh pr checks 691`의 no-check 결과를 그대로 기록한다. |
| 8 | branch protection에 필수 check가 있는데 빠뜨림 | 통과. main protection의 `required_status_checks: null`을 다시 확인했다. |
| 9 | 필수 승인이 1개인데 review 없는 merge를 숨김 | 통과. required approval count는 0이고 독립 승인은 없었다고 적었다. |
| 10 | check 미보고를 CI 통과로 바꿔 말함 | 통과. STATUS, plan, README가 모두 CI 통과를 주장하지 않는다. |
| 11 | PR을 합쳤으니 C11.4 전체 checkbox도 완료됨 | 통과. C11.4 checkbox는 `[ ]` 그대로다. |
| 12 | 제한 scene 수를 공식 완료 개수에 더함 | 통과. 공식 `72/213`을 그대로 유지했다. |
| 13 | 이 runtime slice가 upstream WPT harness 통과로 기술됨 | 통과. 제한 app fixture와 WPT runner 통과를 분리한다. |
| 14 | 여덟 scene 실행을 일반 CSS 지원으로 해석함 | 통과. author CSS admission과 일반 CSS/Grid 조합은 미지원·미검증으로 남긴다. |
| 15 | 이웃 track 두 형태가 모든 auto-repeat 입력을 포괄한다고 기술됨 | 통과. plan과 내부 계약이 고정 trailing-column profile로 한정한다. |
| 16 | Android AVD가 실기기 또는 hardware GPU로 기재됨 | 통과. Android API 37 ARM64 AVD로 명시하고 실기기/hardware GPU 증거로 확대하지 않는다. |
| 17 | Android sample 근거의 Surface API가 모호해짐 | 통과. border 밖 표본은 PixelCopy라고 쓰며 scene별 3/3 RGBA 범위를 유지한다. |
| 18 | iOS 기기·OS가 Simulator를 실기기로 오해할 수 있게 작성됨 | 통과. iPhone 17 Pro / iOS 26.2 Simulator로 명시한다. |
| 19 | iOS Metal drawable 표본이 compositor 이후 결과로 오해됨 | 통과. 모든 관련 문구에 compositor 제출 전임을 유지했다. |
| 20 | screenshot을 pixel equality 근거로 확대함 | 통과. 화면은 screenshot 링크이고 정확한 pixel 결과는 별도 sample으로 설명한다. |
| 21 | overflow 표본 3개와 scene 2개의 집계가 애매함 | 통과. 두 overflow scene 각각 Android PixelCopy 및 iOS Metal 표본 3/3으로 명시한다. |
| 22 | 28px overflow가 74px root와 다른 geometry에서 계산됨 | 통과. 100px item, 74px border box, 28px right overflow의 관계가 plan·내부 계약·runtime README에서 같다. |
| 23 | `min-content`와 `max-content` 또는 auto-fill과 auto-fit scene이 섞임 | 통과. 8개 ID와 expected columns는 사전 비교의 독립 기대값에 고정돼 있다. |
| 24 | archive 검사가 APK/IPA payload exclusion으로 과장됨 | 통과. `.rlib` 증거로 한정하고 APK/IPA 전체를 검사하지 않았다고 명시한다. |
| 25 | 내부 계약 버전이 기능 추가만으로 상승함 | 통과. `0.1.0`을 유지한다. |
| 26 | `GridContainerInput` 내부 변경이 사용자 API 발표로 표현됨 | 통과. unpublished package의 내부 workspace interface로 표현하고 사용자 CSS/API 추가를 부인한다. |
| 27 | 계획 PR #690과 구현 PR #691의 순서가 반대로 기록됨 | 통과. #690 계획 병합과 #691 구현 병합을 분리한다. |
| 28 | 상태 문서의 상대 링크가 route 계산에서 깨짐 | 통과. 기존 spec 상대 경로와 PR 절대 링크를 확인했고 근거 file 이름도 존재한다. |
| 29 | C11.4 완료 checkbox 외의 공식 집계·상태가 조용히 변함 | 통과. 이 문서 변경은 C11.4 전체 체크와 `72/213` 집계를 바꾸지 않는다. |
| 30 | Tailnet 미리보기 sync가 아직 안 됐는데 완료라고 표시됨 | 통과. 이 문서 변경은 Tailnet 게시 완료를 주장하지 않는다. 미리보기는 별도 HTTP/hash/Agent Browser 확인 뒤 보고한다. |

차단되는 상태·로드맵 불일치는 찾지 못했다. 이 변경은 문서 전용이므로 Android/iOS 빌드나 앱 테스트 결과를 새로 만들지 않았고, Tailnet 미리보기는 별도 단계로 남긴다.
