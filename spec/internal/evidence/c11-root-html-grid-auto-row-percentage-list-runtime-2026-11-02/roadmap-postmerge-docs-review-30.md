# C11.1 percentage `grid-auto-rows` 병합 후 문서 변경 검토 · 30개 관점

**검토 대상:** 병합 사실을 `plan/c11-css-grid.md`, 전용 구현 계획, `spec/STATUS.md`와 evidence에 반영한 문서 PR. 제품 코드나 테스트 결과는 변경하지 않는다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | GitHub merge 시각을 잘못 옮김 | PR API의 `2026-11-02T09:10:09Z`를 UTC 그대로 기록했다. |
| 2 | PR 번호를 계획 PR과 혼동 | 구현 PR #1047로 기록했고 이전 계획 PR 번호를 재사용하지 않았다. |
| 3 | merge SHA를 candidate head SHA로 대체 | GitHub `mergeCommit.oid` 값 `bc3430ac664e76a2f246bcbfc387f61121017bb6`을 기록했다. |
| 4 | source head hash를 merge hash처럼 표기 | 당시 PR head `70133f22c9e9a699a9de42777ad9b0f7719e4dd4`를 별도 설명으로 분리했다. |
| 5 | 기능 commit을 최종 merge tip과 혼동 | Rebase and merge 결과 코드 commit과 최종 tip을 각각 구별했다. |
| 6 | merge 방법을 부정확하게 표시 | GitHub가 실행한 Rebase and merge만 기록했다. |
| 7 | base branch가 아닌 다른 branch에 병합했다고 주장 | PR metadata의 base `main`을 기록했다. |
| 8 | open/mergeable 문구를 병합 뒤에도 남김 | 상위 계획의 candidate/open 상태 문구를 병합 완료로 교체했다. |
| 9 | 제한 구현을 C11.1 완료로 승격 | 각 문서는 제한 slice라고 한정하며 완료 체크는 그대로 둔다. |
| 10 | C11 parent를 완료 처리 | C11은 계속 미완료라고 명시했다. |
| 11 | 공식 전체 완료 수를 증가 | `72/213`을 유지한다고 세 문서에서 확인했다. |
| 12 | checkbox를 무심코 수정 | 변경된 status diff는 기존 후보 설명에 merge metadata만 추가하고 checkbox 줄은 건드리지 않는다. |
| 13 | CI 미보고를 성공으로 과장 | PR checks 없음과 required status checks 미설정을 기록하고 CI 통과를 주장하지 않는다. |
| 14 | required check 응답 404를 workflow green으로 해석 | API 응답은 required checks 미설정으로만 기록한다. |
| 15 | Rust 결과가 전체 workspace green을 뜻한다고 확대 | focused suites와 scoped lint의 제한된 범위만 링크된 evidence에 남긴다. |
| 16 | Android AVD를 실기기로 오표기 | API 37 ARM64 AVD로 기록하고 실기기 결과에서 제외했다. |
| 17 | iOS Simulator를 iPhone 실기기로 오표기 | iPhone 17 Pro/iOS 26.2 Simulator로 정확히 표기했다. |
| 18 | screenshot을 pixel equality로 표현 | 시각 자료로만 명시하고 PixelCopy/Metal readback 미실행을 적었다. |
| 19 | hardware GPU 검증을 주장 | hardware GPU 검증을 제외 범위로 둔다. |
| 20 | WPT/전체 CSS 지원을 주장 | WPT와 전체 CSS 적합성을 미검증으로 남긴다. |
| 21 | definite percentage basis를 생략 | 기본 Root HTML route와 definite height 조건을 상위 계획에 적었다. |
| 22 | 지원 범위를 임의 percentage syntax로 확대 | direct finite non-negative percentage scalar/list로 제한한다. |
| 23 | row-list 순환 동작을 빠뜨림 | 계획과 evidence 링크로 목록 순환 구현 범위를 확인할 수 있다. |
| 24 | 검증 숫자를 evidence와 다르게 씀 | cascade 7/7, layout 14/14, Root HTML projection 26/26, 13 boxes가 일치한다. |
| 25 | public roadmap status와 내부 status가 서로 다름 | `plan/c11-css-grid.md`와 `spec/STATUS.md`의 PR/완료 판정이 맞는다. |
| 26 | plan 링크가 잘못된 상대 경로를 가리킴 | 전용 plan과 evidence 경로가 저장소 내 파일을 가리킨다. |
| 27 | merge record 링크가 파일보다 먼저 존재한다고 오해 | merge note는 같은 evidence directory에서 함께 추가한다. |
| 28 | 실행 evidence를 병합 기록만으로 대체 | 원본 runtime README, screenshots, logs와 review 기록을 보존한다. |
| 29 | contract version을 근거 없이 승격 | 내부 계약 0059의 `0.1.0`은 변경하지 않는다. |
| 30 | 문서 동기화가 제품 코드 동작을 바꾼다고 암시 | PR은 계획·상태·병합 기록과 검토 문서만 변경한다. |

## 최종 판정

문서의 merge metadata와 limited-slice 판정은 GitHub PR API 및 기존 실행 evidence와 일치한다. 전체 완료 수와 C11 체크는 승격하지 않는다. 이 문서 PR에는 제품 코드·테스트 변경이 없다.
