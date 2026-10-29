# C11.3.3 leading named implicit lines 병합 후 상태 대장 검토 30개

## 검토 범위

PR #790 병합 뒤 `spec/STATUS.md`에 이번 제한 slice의 근거만 추가했다. C11/C11.3.3 완료 판정은 변경하지 않는다. 검토 기준은 GitHub PR metadata, 내부 계약 0059, PR #790의 runtime/evidence 문서와 현재 상태 대장이다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 PR #789와 구현 PR을 혼동 | 구현 변경은 PR #790, C11.3.3 계획은 별도 PR #789로 기록한다. |
| 2 | PR이 실제로 병합되지 않았는데 완료 이력을 기록 | GitHub state `MERGED`를 확인했다. |
| 3 | 날짜 또는 시간대를 잘못 기록 | GitHub `mergedAt` UTC 값 `2026-10-29T04:59:50Z`를 사용했다. |
| 4 | squash/merge commit을 사용했다고 잘못 표기 | 저장소 정책에 따른 `Rebase and merge`로 기록한다. |
| 5 | PR head SHA를 merge SHA로 혼동 | 최종 merge SHA `d993eb4114b17a49b2d096f319053fd43c6fcf27`를 API에서 확인했다. |
| 6 | check rollup이 비었는데 CI 통과를 주장 | PR rollup과 required-check 조회가 비어 있음을 기록하고 CI 성공이라 하지 않는다. |
| 7 | 승인 리뷰 설정이 없는데 독립 승인을 암시 | `main` required approving review 수가 0이며 승인자를 기록하지 않는다. |
| 8 | C11.3.3 항목을 완료로 체크 | `[ ]` candidate 표기를 유지한다. |
| 9 | 상위 C11을 완료로 체크 | C11 상위 항목을 unchecked로 유지한다. |
| 10 | C11.3 그룹을 완료로 체크 | C11.3 상위 항목을 unchecked로 유지한다. |
| 11 | 부분 slice를 공식 완료 수에 더함 | 공식 `72/213`을 그대로 유지한다. |
| 12 | 기존 PR #152의 26-scene 구현과 혼동 | 기존 26 scene 근거는 보존하고 새 slice를 별도 날짜 기록으로 추가한다. |
| 13 | 기준 26 scene을 새 scene 합계로 대체 | 기준 행렬 26 scene을 그대로 유지하고 별도 행렬과 구분한다. |
| 14 | unknown-area overflow를 새 named-line scene에 합침 | 별도 unknown-area scene 1개로 표기한다. |
| 15 | 새 fixture 장면 수를 잘못 기록 | 새 leading named implicit scene 7개를 기록한다. |
| 16 | simulator geometry 비교를 track-vector readback으로 확대 | CSS geometry 오차 0으로만 표현하고 vector 검증은 내부 layout evidence 범위로 둔다. |
| 17 | Android PixelCopy 결과를 전체 화면 정확성으로 확대 | 새 장면 sample 7/7 exact로 제한한다. |
| 18 | Android AVD를 실기기로 오해 | API 37 ARM64 AVD라고 명시한다. |
| 19 | Android AVD software renderer를 hardware GPU로 오해 | SwiftShader software renderer라고 남긴다. |
| 20 | iOS Metal drawable을 compositor 이후 화면으로 오해 | compositor 제출 전 표본이라고 제한한다. |
| 21 | Simulator 결과를 실기기 지원으로 확대 | 두 플랫폼 모두 Simulator/AVD 결과로만 기록한다. |
| 22 | Chrome oracle 실행을 Spinon WPT pass로 오해 | Spinon WPT 적합성은 미완료로 유지한다. |
| 23 | 고정 fixture를 일반 author CSS 지원으로 오해 | 제한 fixture 범위와 candidate 상태를 유지한다. |
| 24 | 이번 slice가 전체 CSS Grid를 완료한다고 읽힐 수 있음 | 전체 Grid 미완료를 명시한다. |
| 25 | 내부 계약 버전이 구현 변경으로 올라감 | 0059 버전 `0.1.0`을 유지한다. |
| 26 | 공개 API가 추가된 것으로 오해 | 이 경계는 내부 계약이며 공개 API가 아니라고 유지한다. |
| 27 | 전체 `spinon-style` 테스트가 모두 통과한 것으로 기록 | 247개 중 246개와 기존 실패 1개를 근거 README에 정확히 보존한다. |
| 28 | 기본 FFI와 opt-in fixture artifact 경계를 생략 | README의 default/positive-control 한계 링크를 유지한다. |
| 29 | 새 상태 문장에 구현 검토 경로가 누락 | 구현 review 20, PR diff review 30, 실행 evidence README를 직접 연결했다. |
| 30 | 다른 C11/C11.4 항목이나 근거가 수정됨 | 변경 범위는 C11.3.3 merge 근거 한 줄과 이 검토 문서로 한정한다. |

## 판정

병합 metadata와 검증 범위를 근거에 맞게 반영했다. C11.3.3 candidate, 상위 C11 checkbox, 공식 `72/213`은 그대로다. 수정 blocker는 발견하지 못했다.

## 원격 PR diff 판정

- 문서 PR [#791](https://github.com/ohah/spinon/pull/791)의 base는 `d993eb4114b17a49b2d096f319053fd43c6fcf27`, 최초 원격 검토 head는 `63d1a8c8d7cd41d5ea99c784e83e210653b3a25c`다.
- GitHub PR files API의 두 경로는 로컬 `origin/main...origin/docs/c11-3-3-merge-status-sync-20261029` diff와 일치한다: `spec/STATUS.md` 및 이 검토 문서다. 제품 코드·체크박스·공식 집계 변경은 없다.
- PR 본문은 #790의 merge metadata, 미보고 CI, candidate 상태와 `72/213` 보존을 정확히 설명한다. 위 30개 관점은 이 두 문서의 원격 변경 범위에 대조했다.
- PR check rollup과 required-check 조회는 비어 있다. `main`은 required approving review 0개이며 linear history를 요구한다. 저장소 정책에 맞춰 Rebase and merge만 허용된다.

이 검토 기록 변경 후 PR head가 바뀌므로 병합 전에 원격 경로 목록, 최종 head SHA, merge state와 required checks를 다시 확인한다.
