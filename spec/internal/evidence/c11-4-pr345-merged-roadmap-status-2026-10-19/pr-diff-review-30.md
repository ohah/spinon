# C11.4 PR #345 병합 상태 대장 정정 · PR diff 30관점 검토

**대상:** `docs/c11-4-pr345-merge-status`의 한 문장 상태 설명 및 검토 근거. PR #345는 main에 병합된 상태다. 이 변경은 C11.4를 완료로 표시하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | #345의 상태가 여전히 open임 | GitHub PR 조회에서 `MERGED`를 확인했다. |
| 2 | 병합 커밋을 잘못 인용함 | merge commit `3d2ec268fd23f977aa4c59d5c42bb319556f92a9`와 일치한다. |
| 3 | 다른 base branch에 병합된 PR을 설명함 | PR base는 `main`이다. |
| 4 | source branch tip을 main 반영 SHA로 혼동함 | 대장 문구는 특정 head SHA를 완료 근거로 사용하지 않는다. |
| 5 | C11.4 체크박스를 실수로 완료함 | 체크박스는 `[ ]`로 유지된다. |
| 6 | 상위 C11을 완료로 올림 | 상위 항목도 미완료 상태를 유지한다. |
| 7 | PR 병합을 전체 intrinsic sizing 지원으로 확대 해석함 | 첫 문장에서 전체 Grid sizing이 미완료임을 유지한다. |
| 8 | 앱 stage trace를 공개 product API로 오해하게 씀 | 문구는 앱 fixture의 trace 기록만 설명한다. |
| 9 | fixture 전용 경계를 없앤 것으로 읽힘 | 기존 문구의 fixture 전용·opt-in 경계는 그대로 남아 있다. |
| 10 | assertion 실패 경로를 검증 완료로 과장함 | 호출 순서 수정과 성공 scene 재실행을 구분한다. |
| 11 | frame mismatch를 실제 주입했다고 주장함 | mismatch injection 미수행 한계가 기존 대장에 남아 있다. |
| 12 | Android 실기기 실행으로 기록함 | API 37 AVD라는 환경을 그대로 유지한다. |
| 13 | iOS 실기기 실행으로 기록함 | iPhone 17 Pro Simulator/iOS 26.2라는 환경을 그대로 유지한다. |
| 14 | simulator frame 검사와 surface pixel 검사를 혼동함 | border stroke 색을 검사하지 않은 제한이 유지된다. |
| 15 | Android/iOS 결과 수를 누락하거나 바꿈 | 기존 7-scene 결과 문구는 변경하지 않았다. |
| 16 | canonical trace 동등성의 범위를 확대함 | 기존 7/7 비교 범위만 대장에 남아 있다. |
| 17 | text intrinsic input이 검증됐다고 주장함 | 앱 text/image intrinsic contribution이 남은 항목으로 명시돼 있다. |
| 18 | image intrinsic input이 검증됐다고 주장함 | 앱 text/image intrinsic contribution이 남은 항목으로 명시돼 있다. |
| 19 | intrinsic contribution 전체 행렬을 완료로 취급함 | 전체 contribution 적용 행렬이 미완료 목록에 있다. |
| 20 | nested/anonymous item coverage를 완료로 취급함 | 해당 item 범위가 미완료 목록에 있다. |
| 21 | cyclic percentage coverage를 완료로 취급함 | cyclic percentage와 전체 repeat 조합이 미완료 목록에 있다. |
| 22 | auto-repeat 지원 subset을 전체 grammar로 부풀림 | 제한된 cascade subset과 미지원 조합 설명이 유지된다. |
| 23 | WPT 결과를 C11.4 완료 근거로 사용함 | WPT가 미완료 목록에 있다. |
| 24 | 독립 normative oracle 비교를 했다고 주장함 | 독립 판정이 남아 있다는 설명이 유지된다. |
| 25 | 기존 PR #338/#340의 책임을 PR #345로 옮김 | 세 PR의 설명과 역할이 각각 보존된다. |
| 26 | evidence 링크가 원문 Markdown 다운로드로 연결됨 | 새 stage-trace와 후속 근거 경로는 `.html` 렌더 route를 사용한다. |
| 27 | 병합 커밋 링크를 PR 링크 대신 새로 임의 구성함 | 대장에는 안정된 PR #345 링크를 사용한다. |
| 28 | 상태 행 주변의 다른 작업 상태를 변경함 | diff는 C11.4 문장 한 곳에 한정된다. |
| 29 | 정리된 전체 루트 빌드를 원본 전체 빌드로 오해함 | 선택 렌더, 임시 사본 전체 렌더, 원본 전체 루트 실패를 README에서 분리하고 임시 변경을 기록했다. |
| 30 | 포맷·공백 문제나 우발 변경이 들어감 | `git diff --check`와 단일 파일 diff를 확인했다. |

**결론:** 발견한 상태 설명 불일치(병합된 PR을 “검토 중”으로 표시)는 수정했다. C11.4 자체는 미완료로 남는다.
