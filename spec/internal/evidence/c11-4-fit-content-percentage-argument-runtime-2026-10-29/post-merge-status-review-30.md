# C11.4 percentage fit-content 병합 상태 문서 검토 30개

## 검토 범위

공식 상태 대장, 내부 계약 0059, C11.4 전용 계획에 구현 PR #771의 병합 시각·방식·SHA를 반영한 문서 변경을 독립 관점 30개로 대조했다. 이 변경은 기존 구현 범위나 완료 판정을 바꾸지 않는다.

| # | 실패 관점 | 확인 결과 |
|---|---|---|
| 1 | PR이 실제로 열린 상태라고 잘못 남길 수 있음 | GitHub PR state가 MERGED임을 확인했다. |
| 2 | 병합 날짜의 시간대를 혼동할 수 있음 | GitHub가 반환한 UTC 시각 2026-10-28T20:48:31Z를 그대로 썼다. |
| 3 | 구현 head SHA를 merge SHA로 잘못 적을 수 있음 | merge SHA f89dfc2be9ff732167986bd4e2e453f3819343d1을 기록했다. |
| 4 | 병합 방식을 다른 정책과 혼동할 수 있음 | 저장소 규칙에 맞는 Rebase and merge로 기록했다. |
| 5 | 빈 check 조회를 CI 성공으로 해석할 수 있음 | required-check 결과가 비어 있었고 CI 통과가 아니라고 명시했다. |
| 6 | 문서 갱신으로 공식 완료 수를 올릴 수 있음 | 공식 72/213을 그대로 유지했다. |
| 7 | 부분 slice가 C11 상위 완료로 보일 수 있음 | C11 상위 체크 상태를 바꾸지 않았다. |
| 8 | 부분 slice가 C11.4 전체 완료로 보일 수 있음 | C11.4 미완료 문구와 체크 상태를 유지했다. |
| 9 | 기존 완료된 C11.2 하위 ID를 덮어쓸 수 있음 | C11.2.2와 C11.2.3 상태를 수정하지 않았다. |
| 10 | C11.3 candidate를 완료로 오해할 수 있음 | C11.3.3 candidate 상태를 수정하지 않았다. |
| 11 | 한 WPT assertion이 전체 파일 완료로 확대될 수 있음 | 계획과 상태는 percentage argument assertion 하나로 제한한다. |
| 12 | WPT revision이 다른 원본으로 바뀔 수 있음 | 고정 revision c999c58338ee1d223df5ad62e48be1202ced0d37을 유지했다. |
| 13 | 원본 source hash가 누락되거나 바뀔 수 있음 | SHA-256 828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736를 유지했다. |
| 14 | definite percentage basis가 잘못 기재될 수 있음 | 300px block basis와 150px row를 기존 근거대로 유지했다. |
| 15 | raw growth limit을 used size로 오해할 수 있음 | trace의 단계별 값과 최종 150px row를 구분하는 계약 문구를 유지했다. |
| 16 | 400px child overflow가 clipping 지원으로 확대될 수 있음 | overflow 관찰을 유지하고 clipping/scrolling 지원은 선언하지 않는다. |
| 17 | Android AVD를 실기기로 오해할 수 있음 | Android 증거를 API 37 ARM64 AVD로 한정한다. |
| 18 | Android AVD를 hardware GPU 결과로 오해할 수 있음 | software AVD 경로라는 한계를 유지한다. |
| 19 | iOS drawable sample을 compositor 이후 결과로 오해할 수 있음 | compositor 제출 전 Metal drawable 한계를 유지한다. |
| 20 | 두 Simulator 결과가 실기기 지원을 뜻할 수 있음 | 실기기 근거로 확대하지 않도록 한계를 유지한다. |
| 21 | Spinon WPT 실행이 통과한 것으로 오해할 수 있음 | upstream pinned WPT와 앱 fixture 결과를 분리했다. |
| 22 | 전체 WPT suite가 검증된 것으로 오해할 수 있음 | 나머지 assertion과 전체 suite는 범위 밖으로 둔다. |
| 23 | arbitrary Grid tree 지원을 선언할 수 있음 | 고정 allowlisted fixture 범위만 유지한다. |
| 24 | 일반 author CSS admission이 추가된 것으로 오해할 수 있음 | 공개 CSS admission을 하지 않는다고 유지한다. |
| 25 | 다른 percentage basis까지 지원하는 것으로 오해할 수 있음 | 다른 basis와 cyclic/indefinite 경우는 제외한다. |
| 26 | 내부 계약 버전 상승을 잘못 포함할 수 있음 | 내부 계약 숫자 버전 0.1.0을 유지했다. |
| 27 | 계획·계약·상태 사이 merge SHA가 다를 수 있음 | 세 문서의 merge SHA와 UTC 시각을 일치시켰다. |
| 28 | PR 링크가 잘못된 PR을 가리킬 수 있음 | 세 문서 모두 ohah/spinon PR #771을 연결한다. |
| 29 | 증거 링크가 병합 변경으로 끊길 수 있음 | 기존 계획, 계약, simulator evidence와 review 경로를 유지했다. |
| 30 | 로드맵 집계가 narrative 변경 때문에 증가할 수 있음 | 체크박스는 그대로여서 공식 집계는 72/213으로 남는다. |

## 판정

문서 변경은 PR #771의 병합 메타데이터만 추가한다. C11과 C11.4는 미완료이며 공식 완료 수는 72/213이다. 발견한 수정 필요 항목은 없다.
