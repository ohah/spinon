# C11.3.3 계획 상태 동기화 검토 30개

## 검토 범위

구현 PR #790 병합 뒤 계획 상단의 stale `no implementation yet` 문구를 실제 제한 slice 결과로 바꾼다. 계획 전체, C11.3.3, C11의 공식 완료 상태는 미완료로 둔다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #790이 실제 병합 상태인지 | GitHub state `MERGED`를 다시 확인했다. |
| 2 | merge 시각을 임의 시간대로 쓰는지 | PR API의 UTC `mergedAt`을 유지한다. |
| 3 | Rebase 방식 외 머지 방식으로 표기하는지 | 저장소 정책의 Rebase and merge 결과와 일치한다. |
| 4 | merge SHA 대신 head SHA를 적는지 | PR API의 mergeCommit SHA `d993eb4114b17a49b2d096f319053fd43c6fcf27`를 적었다. |
| 5 | 계획 제목이 구현보다 넓은 범위를 약속하는지 | 상태 문장은 이 계획의 제한 fixture 구현만 가리킨다. |
| 6 | `no implementation yet`가 계속 남는지 | stale 상태 문장을 병합 결과로 교체했다. |
| 7 | 제한된 fixture와 전체 C11.3.3을 혼동하는지 | C11.3.3은 여전히 미완료라고 명시한다. |
| 8 | 부분 구현을 전체 C11 완료로 해석하는지 | C11 미완료를 별도로 명시한다. |
| 9 | 공개 CSS 지원을 선언하는지 | 공개 CSS 지원 선언이 아니라고 명시한다. |
| 10 | C11 checkbox를 계획에서 완료로 오해하는지 | 공식 상태 링크와 unchecked status는 유지한다. |
| 11 | 공식 `72/213`을 바꿨는지 | 공식 count는 상태 대장에서 그대로 72/213이다. |
| 12 | Android 결과를 실기기로 오해 | Android API 37 ARM64 AVD로 한정한다. |
| 13 | Android software renderer를 감추는지 | 상세 SwiftShader 경계는 링크된 implementation report에 보존한다. |
| 14 | iOS 결과를 실기기로 오해 | iPhone 17 Pro / iOS 26.2 Simulator로 한정한다. |
| 15 | Metal sample을 compositor 이후로 오해 | compositor 제출 전 표본 한계를 implementation report에 둔다. |
| 16 | Android PixelCopy exact 결과를 geometry 전체로 확대 | plan 상태 문장에 추가 성능·정확성 주장을 넣지 않았다. |
| 17 | simulator geometry를 track-vector readback으로 오해 | vector 및 normalized ordering 세부는 evidence README의 한계를 따른다. |
| 18 | 고정-pixel track 조건을 제거해 일반 profile이 되는지 | 계획의 fixed CSS-pixel input 제한은 그대로다. |
| 19 | 일반 numeric implicit placement가 포함되는지 | 기존 non-goal 문장을 유지한다. |
| 20 | auto-placement 또는 intrinsic track sizing을 지원한다고 읽히는지 | 기존 non-goal을 유지한다. |
| 21 | 일반 named-line parsing이 지원된다고 읽히는지 | 제한 allowlisted fixture 경계를 그대로 둔다. |
| 22 | 전체 WPT 완료를 주장하는지 | 기존 전체 WPT non-goal을 유지한다. |
| 23 | oracle revision 또는 기준 날짜를 바꿨는지 | pinned WPT revision과 CRD references는 변경하지 않았다. |
| 24 | parent C11 plan 링크가 바뀌었는지 | 기존 relative parent 링크를 유지했다. |
| 25 | internal contract 링크가 끊겼는지 | 계약 0059의 기존 relative link를 유지했다. |
| 26 | 구현 report 경로가 틀렸는지 | 링크된 README 파일이 저장소에 존재한다. |
| 27 | 문서 언어가 저장소 규칙과 맞는지 | 신규 상태 문장은 한글로 썼다. |
| 28 | plan 변경에 source code가 섞였는지 | PR diff는 plan과 이 review 문서에 한정한다. |
| 29 | plan review 20 또는 이전 PR review를 재사용하는지 | 이 문서는 병합 후 plan state 동기화의 새로운 30 관점이다. |
| 30 | PR #790 구현 evidence와 상충하는지 | PR metadata, implementation README, `spec/STATUS.md`와 대조했다. |

## 판정

계획의 구현 상태 문구는 현재 merged state와 맞는다. 완료 범위는 제한 fixture이며 C11.3.3/C11 및 공식 `72/213`은 미완료다.
