# C11.4 generic spanning 병합 후 로드맵 동기화 PR diff 검토 20개

이 문서는 PR #725 병합 결과를 계획과 상위 로드맵에 반영하는 별도 문서 PR의 변경분만 검토한다. 사실 근거는 [PR #725](https://github.com/ohah/spinon/pull/725), [runtime evidence](./README.md), 원래 [구현 PR diff 검토](./pr-diff-review-20.md)다.

| # | 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 잘못된 PR 번호나 저장소 URL을 기록하는가 | 제목·링크가 모두 `ohah/spinon#725`를 가리킨다. |
| 2 | PR open 상태를 현재 병합 상태로 잘못 추정하는가 | GitHub API의 `state=MERGED` 및 `mergedAt` 값으로 반영했다. |
| 3 | merge 시각의 UTC offset을 현지 시간과 혼합하는가 | `2026-10-28 00:12:19 UTC`를 명시한다. |
| 4 | Rebase and merge를 다른 merge 방식으로 기록하는가 | merge commit SHA와 PR timeline의 Rebase and merge 결과를 대조한다. |
| 5 | PR head SHA와 실제 merge commit SHA를 같은 값으로 취급하는가 | PR head `215482c…`와 merge commit `a0f7d0c…`를 구분한다. |
| 6 | 빈 check rollup을 CI 성공으로 표현하는가 | 두 문서 모두 rollup이 비어 CI 통과로 보지 않는다고 적는다. |
| 7 | 병합 사실 때문에 C11.4 상위 항목을 완료로 바꾸는가 | 상위 checkbox는 건드리지 않고 미완료 문구를 유지한다. |
| 8 | 공식 집계가 새 PR 하나만큼 증가하는가 | `72/213`을 유지한다. |
| 9 | 작업 계획의 이전 `검토 대기` 상태가 남아 있는가 | 구체 상태를 구현·플랫폼 검증·PR 병합 완료로 바꾸고 merge 근거를 추가한다. |
| 10 | 상위 계획과 세부 계획이 서로 다른 merge SHA를 기록하는가 | 동일한 merge SHA와 시각을 두 문서에서 확인한다. |
| 11 | Android 증거를 실기기 결과로 확대하는가 | API 37 ARM64 16 KB AVD라고 범위를 그대로 쓴다. |
| 12 | iOS 증거의 기종·runtime 버전을 누락하는가 | iPhone 17 Pro/iOS 26.2 Simulator를 그대로 쓴다. |
| 13 | 각 플랫폼의 장면 수를 합산 성공으로 감추는가 | 각 simulator에서 4/4임을 분리해 기록한다. |
| 14 | draw/present를 pixel-perfect paint 증명으로 표현하는가 | tracks·frames·trace·draw/present 범위만 기술한다. |
| 15 | focused test 집계의 기준을 잘못 적는가 | runtime 6/6, FFI 1/1을 README 결과와 대조한다. |
| 16 | static archive 검사를 APK/IPA 전체 제외로 부풀리는가 | default/product/fixture-test archive 결과만 언급한다. |
| 17 | 다른 C11.4 slices와 지원 계약을 합산하는가 | 이 네 mixed spanning scene의 generic HostNode route로 제한한다. |
| 18 | runtime evidence 링크를 잘못된 경로로 연결하는가 | 상위 계획과 세부 계획에서 같은 dated evidence 디렉터리를 가리킨다. |
| 19 | 불필요하게 상태 대장이나 명세 버전을 바꾸는가 | `spec/STATUS.md`와 버전 있는 인터페이스 명세에는 변경이 없다. |
| 20 | untracked 작업 파일이나 구현 코드를 문서 전용 PR에 섞는가 | PR 변경은 두 계획 문서와 이 review 문서로 한정한다. |

**판정:** 문서 동기화 diff 검토 20/20. 이는 원래 구현 PR의 20개 적대 검토와 별도다.
