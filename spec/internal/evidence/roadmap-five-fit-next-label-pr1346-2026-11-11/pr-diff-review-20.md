# PR #1346 five-fit 다음 단계 라벨 적대 검토 · 2026-11-11

## 검토 기준

- 원격 PR: [#1346](https://github.com/ohah/spinon/pull/1346)
- 기준 commit: `dc1b73c2dd7640917a0d0d49d20fa346b68b716c`
- 검토한 구현 commit: `65aa332418eb7d27e58c4a7128af566183d797bf`
- 구현 전용 원격 patch는 `spec/public/roadmap.html` 한 파일, 2 insertions / 2 deletions였고 commit `65aa332418eb7d27e58c4a7128af566183d797bf`의 checkout diff와 일치했다.
- 이 기록을 포함한 최종 원격 PR diff도 checkout의 `origin/main...HEAD`와 대조했다. 변경 경로는 이 검토 문서와 `spec/public/roadmap.html` 두 개뿐이다.
- 이 검토는 카드 문구와 기존 상태 불변조건을 다룬다. 최종 remote-render 검증은 merge 후 별도로 수행한다.

## 20개 실패 관점

| # | 적대적 실패 조건 | 결과 |
|---:|---|---|
| 1 | 이전처럼 모든 `detailNext`를 `계획 완료`로 고정 표시한다. | PASS — 고정 완료 문구를 제거했다. |
| 2 | 카드가 존재하지 않는 stage 속성에서 상태를 읽는다. | PASS — five-fit 항목에 `stage` 값을 선언했다. |
| 3 | 렌더러가 선언한 항목별 stage를 무시한다. | PASS — subtitle이 `task.stage`를 읽는다. |
| 4 | 아직 시작하지 않은 사전 비교를 완료로 표현한다. | PASS — 값은 `사전 비교·계획 전`이다. |
| 5 | Chrome 157 사전 비교가 이미 실행됐다고 설명한다. | PASS — 아직 시작하지 않았다고 명시한다. |
| 6 | 계획 수립도 끝났다고 설명한다. | PASS — 계획 수립이 아직 시작되지 않았다고 명시한다. |
| 7 | 다음 후보 이름이 four-fit 구현으로 되돌아간다. | PASS — `C11.4 five-fit candidate`를 유지한다. |
| 8 | five-fit 후보가 다음 열에서 빠진다. | PASS — 항목은 `detailNext`에 남는다. |
| 9 | five-fit 후보를 진행 중으로 과장한다. | PASS — `detailInProgress`는 빈 배열이다. |
| 10 | C11.4가 진행 중인 공식 상위 작업으로 잘못 분류된다. | PASS — `inProgress`는 빈 배열이다. |
| 11 | 공식 완료 수가 변경된다. | PASS — `78/214`를 유지한다. |
| 12 | 상위 C11.4 또는 C11을 완료로 표시한다. | PASS — 둘 다 미완료 문구다. |
| 13 | 직전 four-fit PR #1344의 merge 사실이 바뀐다. | PASS — 기존 merge 문구를 유지한다. |
| 14 | GitHub 공식 STATUS fetch endpoint가 변경된다. | PASS — patch는 해당 endpoint를 건드리지 않는다. |
| 15 | 다음 후보 수가 두 번 증가하거나 사라진다. | PASS — 기존 `queue.length + detailNext.length` 계산을 유지한다. |
| 16 | 일반 `nextOrder` 카드 subtitle이 바뀐다. | PASS — 해당 map은 변경하지 않았다. |
| 17 | iOS/Android/GPU 외부 대기 상태가 바뀐다. | PASS — waiting 목록을 변경하지 않았다. |
| 18 | WIP 제한이 바뀐다. | PASS — `wipLimit:2`를 유지한다. |
| 19 | 스타일, DOM 구조, URL 또는 제품 런타임까지 변경된다. | PASS — roadmap HTML의 두 데이터·표시 표현만 수정했다. |
| 20 | 원격 patch에 checkout diff에 없는 hunk가 포함된다. | PASS — 원격 patch와 로컬 한 파일 diff가 일치했고 `git diff --check`, inline JavaScript syntax 검사가 통과했다. |

## 판정

위 범위에서 서로 다른 실패 조건 20개를 검토했고 모두 PASS다. PR status check는 0개이며 CI green으로 세지 않는다. 공식 완료는 `78/214`, C11.4/C11은 미완료다.
