# C11.4 zoomed `fit-content()` 병합 후 문서 변경 실패 관점 검토 30개

2026-10-29 · PR #783 병합 후 `STATUS.md`, 내부 계약 0059, runtime evidence의 병합 기록을 확인했다. 이 표는 병합 사실을 반영하는 docs-only PR diff에 대한 적대적 검토 30개로 사용하며, 구현/runtime 20개 및 PR #783 diff 30개 검토와 별개다. C11.4 기능 완료 판정은 하지 않는다.

| # | 실패 관점 | 대조 및 결과 | 판정 |
|---:|---|---|---|
| 1 | PR이 실제 병합되지 않았는데 완료형 문구를 쓰는가 | GitHub PR #783의 `state`가 `MERGED`다. | 통과 |
| 2 | merge 시각을 현지 시간과 UTC로 혼동하는가 | `mergedAt` 원본 `2026-10-29T01:51:27Z`를 그대로 적었다. | 통과 |
| 3 | rebase 결과 SHA 대신 PR head를 merge SHA로 쓰는가 | `mergeCommit.oid` `664f4a848a3bf21bec4d2be6080633a8e3b4c5a4`를 기록했다. | 통과 |
| 4 | 허용되지 않은 squash 또는 merge commit 방식으로 합친 것으로 적는가 | PR은 저장소 정책의 Rebase and merge로 병합됐다. | 통과 |
| 5 | 병합 전 head SHA를 병합 후 main commit이라고 부르는가 | PR head `a7ee4feac221e4823271bd1a8385d9ca6efd92e6`와 merge SHA를 분리했다. | 통과 |
| 6 | base SHA와 merge 결과를 혼동하는가 | PR base `60ffaf1509902c94a11644331d82280916767a1d`와 merge commit을 각각 기록했다. | 통과 |
| 7 | 현재 main에 실제 반영되지 않은 SHA를 보고하는가 | fetch한 `origin/main` 최상단은 PR merge SHA `664f4a848a3bf21bec4d2be6080633a8e3b4c5a4`다. | 통과 |
| 8 | 빈 check rollup을 녹색 CI로 추론하는가 | PR의 `statusCheckRollup` 배열이 비어 있음을 기록하고 성공이라 해석하지 않는다. | 통과 |
| 9 | required check 조회의 빈 결과를 체크 통과로 읽는가 | `gh pr checks 783 --required` 결과는 branch에 보고된 체크가 없다는 내용이다. | 통과 |
| 10 | 보호 규칙에 없는 status requirement를 있다고 과장하는가 | main protection response에는 required status checks가 구성돼 있지 않다. | 통과 |
| 11 | CI 미실행과 CI 실패를 같은 상태로 취급하는가 | 증거가 없는 상태를 CI 실패나 통과 대신 체크 부재로 기록한다. | 통과 |
| 12 | PR 승인 요구가 없다는 사실을 코드 리뷰 승인으로 부풀리는가 | 보호 규칙상 required approving review count는 0이며, 상태 문서는 reviewer 승인을 주장하지 않는다. | 통과 |
| 13 | official 완료 수를 병합으로 증가시키는가 | `spec/STATUS.md`의 공식 완료 수 `72/213`을 유지한다. | 통과 |
| 14 | C11.4 parent checkbox를 제한 fixture로 체크하는가 | C11.4 checkbox는 계속 미완료다. | 통과 |
| 15 | zoom slice만으로 C11.4 subtask 전체가 끝났다고 읽히는가 | 문구는 pinned WPT assertion 하나의 제한 opt-in slice로 범위를 고정한다. | 통과 |
| 16 | 새 status bullet이 기존 계획 단계와 충돌하는가 | 계획 runtime 결과와 C11.4 전체 미완료 문구가 함께 유지된다. | 통과 |
| 17 | 내부 계약 숫자 버전을 작업마다 올리는가 | 계약 0059의 numeric version `0.1.0`을 유지한다. | 통과 |
| 18 | public API/ABI가 생겼다고 암시하는가 | 계약과 evidence는 fixture-test opt-in 경로로 제한한다. | 통과 |
| 19 | 일반 CSS `zoom` 지원으로 일반화하는가 | exact fixture root/scene/feature 제한과 일반 admission 미지원 문구가 유지된다. | 통과 |
| 20 | CSSOM `40px`과 Taffy used track `80px`을 한 값으로 섞는가 | CSSOM serialization, Taffy typed limit, contribution, used row를 분리해 설명한다. | 통과 |
| 21 | `80px` limit이 원본 WPT computed CSSOM 값이라고 쓰는가 | `fit-content(80px)`은 Stylo zoom이 반영된 Taffy 입력이라고 명시한다. | 통과 |
| 22 | root/item/child frame 순서 또는 단위를 바꾸는가 | `120×600 / 120×80 / 120×800 CSS px` 순서와 단위를 유지한다. | 통과 |
| 23 | 표면 4/4 결과를 WPT suite 전체 통과로 확장하는가 | 4/4는 각 simulator의 네 고정 pixel sample로 한정된다. | 통과 |
| 24 | Android software AVD를 실기기/hardware GPU로 보고하는가 | Android 결과는 API 37 ARM64 software AVD/PixelCopy로 제한한다. | 통과 |
| 25 | iOS pre-compositor Metal 표본을 최종 OS 화면으로 보고하는가 | compositor 제출 전 Metal drawable이라고 반복해서 명시한다. | 통과 |
| 26 | screenshot의 source snapshot이 merge commit이라고 오인되는가 | README는 캡처 때의 source revision/diff hash를 별도 기록하고 merge SHA로 대체하지 않는다. | 통과 |
| 27 | screenshot 경로와 SHA가 병합 뒤 유실되는가 | README의 Android/iOS run directory와 각 screenshot SHA-256 파일을 유지한다. | 통과 |
| 28 | static archive 결과를 APK/IPA fixture exclusion으로 확대하는가 | archive 양성·음성 대조와 APK/IPA 한계를 분명히 구분한다. | 통과 |
| 29 | 미검증 조합이나 전체 WPT가 지원되는 것으로 읽히는가 | ancestor zoom, auto-repeat, percentage/calc, text intrinsic sizing, full WPT 등 미지원 범위를 열거한다. | 통과 |
| 30 | roadmap/site sync가 병합 기록만으로 끝났다고 잘못 암시하는가 | 이 기록은 GitHub main의 status/contract만 다룬다. 별도 미리보기 동기화를 수행 완료로 주장하지 않는다. | 통과 |

## 검토 결과

30개 문서 실패 관점에서 병합 SHA, 체크 상태, 공식 완료 수 또는 지원 범위를 오인시키는 잔여 결함은 없다. 이 문서는 status/evidence 갱신을 검증하며 외부 미리보기 배포 상태를 대리하지 않는다.
