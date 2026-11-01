# C11 master plan Root HTML 상태 변경 실패 관점 검토 30개

**범위:** `plan/c11-css-grid.md`의 Root HTML 진행 문단을 구현 PR #1006 병합 후 실제 코드·검증·공식 상태에 맞췄다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 계획 PR #1004를 구현 PR로 표기한다. | #1004는 계획, #1006은 구현으로 분리했다. 통과. |
| 2 | 구현 PR 번호가 잘못됐다. | PR #1006을 연결한다. 통과. |
| 3 | merge timestamp가 GitHub 값과 다르다. | `2026-11-01T22:11:00Z`를 기록한다. 통과. |
| 4 | merge SHA가 다르다. | `f4bcce22566c147277bf51ab1f3eae37f12caf4f`를 사용한다. 통과. |
| 5 | PR #1006이 아직 pending이라고 남는다. | 해당 문단에서 pending 문구를 제거했다. 통과. |
| 6 | precomparison의 계획 PR provenance를 잃는다. | #1004 merge SHA와 Chrome 156 precomparison 링크를 유지한다. 통과. |
| 7 | 구현 범위를 일반 CSS Grid로 확장한다. | 고정 2×2 areas, 세 semantic items, 기본 parsed Root HTML route로 제한한다. 통과. |
| 8 | C11.1 전체를 완료로 표시한다. | C11.1 미완료로 명시한다. 통과. |
| 9 | C11.3.3 전체를 완료로 표시한다. | C11.3.3 미완료로 명시한다. 통과. |
| 10 | C11.4 또는 C11 상위를 완료로 표시한다. | C11.4와 C11 모두 미완료다. 통과. |
| 11 | 공식 전체 완료 수를 증가시킨다. | `72/213`을 유지한다. 통과. |
| 12 | focused named-area test count가 다르다. | 7/7을 runtime evidence와 대조했다. 통과. |
| 13 | Root HTML regression count가 다르다. | 50/50을 runtime evidence와 대조했다. 통과. |
| 14 | allocation failpoint 결과가 다르다. | 1/1을 runtime evidence와 대조했다. 통과. |
| 15 | Android AVD를 실기기라고 부른다. | API 37 ARM64 AVD로 적는다. 통과. |
| 16 | iOS Simulator를 실기기라고 부른다. | iPhone 17 Pro/iOS 26.2 Simulator로 적는다. 통과. |
| 17 | draw/present를 pixel readback이라고 과장한다. | visual capture와 readback 미수행을 분리한다. 통과. |
| 18 | post-compositor exactness를 암시한다. | 해당 주장이 없다. 통과. |
| 19 | hardware GPU 결과라고 확대한다. | hardware GPU 주장이 없다. 통과. |
| 20 | 전체 style-to-layout suite를 green 처리한다. | 278/279와 실패 1건을 공개한다. 통과. |
| 21 | C07 mismatch의 실제 값이 바뀐다. | `1.7777778 / 1` 대 `1.77778 / 1`을 정확히 유지한다. 통과. |
| 22 | C07 unrelated failure를 candidate regression이라 부른다. | 기존 C07 serialization 비교로 분리한다. 통과. |
| 23 | check run 부재를 CI 통과로 기술한다. | check run 없음 및 required checks 미설정을 기록한다. 통과. |
| 24 | 구현/런타임 근거 링크가 빠진다. | dedicated runtime README를 연결한다. 통과. |
| 25 | 공개 roadmap 링크가 빠진다. | `spec/public/roadmap.html`을 연결한다. 통과. |
| 26 | C11 계획 문서가 최신 상세 named-area 계획과 분리된다. | 상세 plan 문서를 별도 링크한다. 통과. |
| 27 | Root HTML의 선행 PR #1000/#1002 기록을 덮어쓴다. | 기존 기본 Grid/gap 기록을 유지한다. 통과. |
| 28 | 문단이 체크박스/공식 상태를 변경한다. | 본 계획의 상태 문단만 갱신한다. 통과. |
| 29 | 다른 계획 섹션의 milestone 순서를 바꾼다. | 변경 diff는 Root HTML 진행 문단 한 곳이다. 통과. |
| 30 | 문서 변경이 공개 API 계약 버전을 올린다. | API와 internal contract version은 수정하지 않는다. 통과. |

## 판정

C11 master plan의 Root HTML runtime 상태가 #1006 병합, 제한 검증 결과, 표면 근거 한계 및 공식 `72/213`과 일치한다. 전체 C11과 하위 범위는 미완료다.
