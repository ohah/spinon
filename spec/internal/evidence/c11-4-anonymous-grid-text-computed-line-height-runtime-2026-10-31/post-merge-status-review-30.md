# C11.4 anonymous Grid text computed `line-height` 병합 상태 문서 검토 30개

대상은 PR #905 병합 뒤의 `spec/STATUS.md`, 전용 계획, 내부 계약 `0059`이며, 구현/runtime 검토 및 PR diff 검토와 구분한 상태 일치성 확인이다. GitHub PR 상태와 `main` SHA를 확인해 근거를 대조했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 문서가 PR을 열린 상태로 남긴다 | **통과.** 세 문서의 현재 작업 상태를 #905 병합 완료로 기록했다. |
| 2 | PR 번호가 실제 작업과 다른 번호다 | **통과.** GitHub PR #905 제목과 feature branch를 대조했다. |
| 3 | Rebase merge를 squash/merge commit으로 잘못 표기한다 | **통과.** `Rebase and merge`로 병합한 기록과 merge commit을 적었다. |
| 4 | merge 시각을 다른 timezone의 날짜로 착각한다 | **통과.** GitHub API `mergedAt`의 UTC `2026-10-31T05:46:15Z`를 기록했다. |
| 5 | merge SHA가 branch head의 pre-rebase SHA와 혼동된다 | **통과.** merge SHA `454036612372241fa7b162d6f87c78674ab7e559`는 GitHub PR와 `main` SHA 양쪽에서 확인했다. |
| 6 | merge 뒤 main에 반영되지 않았는데 완료 처리한다 | **통과.** `origin/main` 및 fast-forward된 local `main`이 동일 SHA다. |
| 7 | required check가 없는 상태를 CI 통과로 표기한다 | **통과.** branch protection에 required checks가 없고 PR check rollup도 비어 있음을 기록했다. |
| 8 | `72/213` 공식 완료 집계가 임의로 증가한다 | **통과.** 현재 공식 집계 `72/213`을 유지했다. |
| 9 | C11 parent checkbox를 fixture 하나로 완료 처리한다 | **통과.** C11 상위 항목은 미완료로 유지한다. |
| 10 | C11.4 전체 track sizing을 완료 처리한다 | **통과.** C11.4 checkbox는 미완료이고 이 변경은 제한 fixture 후보로 남긴다. |
| 11 | 날짜가 붙은 기존 계획 검토 링크가 feature branch에 묶인다 | **통과.** 병합된 계획과 계획 검토 링크를 `main` 경로로 정리했다. |
| 12 | 내부 계약이 다른 계획 문서를 가리킨다 | **통과.** 0059의 plan link는 이 computed line-height 계획과 일치한다. |
| 13 | 상태 대장의 계획 링크가 오래된 feature branch를 가리킨다 | **통과.** official status의 계획 URL은 `main`을 가리킨다. |
| 14 | plan이 PR 완료를 말하면서 evidence 링크를 잃는다 | **통과.** precomparison, runtime, 구현 review, diff review, plan review 링크를 유지했다. |
| 15 | runtime contract가 PR 미생성 문구를 유지한다 | **통과.** “PR 생성·병합 상태는 아직 없다” 문구를 제거하고 병합 metadata로 대체했다. |
| 16 | PR merge 문구가 internal contract version을 올린 것으로 오해된다 | **통과.** 숫자 버전 `0.1.0`을 유지한다. |
| 17 | implementation PR merge가 public CSS support를 뜻한다고 읽힌다 | **통과.** 해당 범위는 opt-in fixture에 한정된다고 반복 명시한다. |
| 18 | C15 generic text 또는 inline style까지 완료됐다고 읽힌다 | **통과.** span 전용 `InlineRunStyle` 경계와 C15 일반 범위 미완료를 유지한다. |
| 19 | 16-scene 행렬을 전체 C11.4 coverage라고 확대한다 | **통과.** scene 수와 지원 profile을 제한 후보로 기록했다. |
| 20 | Android AVD를 실기기로 표기한다 | **통과.** API 37 ARM64 AVD로 구분한다. |
| 21 | iOS Simulator를 실기기로 표기한다 | **통과.** iPhone 17 Pro / iOS 26.2 Simulator로 구분한다. |
| 22 | route screenshot을 glyph rendering 증거로 표기한다 | **통과.** glyph paint가 꺼져 있고 화면은 route/status 전용이라고 적는다. |
| 23 | 검은 Android screenshot을 PR 증거로 남긴다 | **통과.** Android 캡처를 사용하지 않는다는 기록을 유지한다. |
| 24 | Android PixelCopy 결과를 있다고 표기한다 | **통과.** PixelCopy 미검증을 명시한다. |
| 25 | iOS compositor 이후 pixel을 확인했다고 표기한다 | **통과.** compositor 이후 화면은 미검증으로 남긴다. |
| 26 | hardware GPU 또는 physical device evidence를 확대 해석한다 | **통과.** 두 범위 모두 미검증이다. |
| 27 | scene implementation이 일반 CSS author admission을 뜻한다 | **통과.** 일반 author cascade는 범위 밖이다. |
| 28 | Chrome fixture 비교를 Spinon WPT 적합성으로 읽는다 | **통과.** WPT 적합성은 별도 미완료다. |
| 29 | status update가 Tailnet preview의 source of truth를 바꾼다 | **통과.** 공식 source는 `spec/STATUS.md`, Tailnet preview는 이를 표시하는 read-only view로 둔다. |
| 30 | 문서 반영 뒤 테스트·문서 빌드 결과를 성공으로 과장한다 | **통과.** runtime/Simulator 결과와 저장소-wide `docs:build` dead-link 실패를 구분하고 공식 완료 수는 그대로 유지한다. |

**판정:** 병합 metadata와 제한 지원 경계가 plan, 0059, official status에서 일치한다. GitHub PR #905의 `MERGED`, `mergedAt`, merge SHA를 확인했다. 공식 C11/C11.4 완료 상태와 `72/213`은 바뀌지 않았다.
