# C11.4 상위 계획 병합 상태 동기화 · PR diff 적대 검토 30개

검토 대상은 `plan/c11-4-grid-track-sizing.md`의 PR #804 및 #796 상태 갱신과 이 검토 근거 파일이다. 검토는 각 사실을 해당 PR, 구현/runtime evidence, 공식 상태 대장과 대조했다. 결과 30/30 통과이며 수정이 필요한 차이는 없었다.

## 검토 기준

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 병합이 아닌 닫힘 상태를 병합으로 기록 | `gh pr view 804`에서 `MERGED`와 병합 시각 확인. 통과. |
| 2 | #804 제목을 다른 PR과 혼동 | PR 제목·번호가 `Grid collapse 제한 runtime fixture 연결`과 일치. 통과. |
| 3 | #804 병합 시각 오기 | 원격 PR의 `2026-10-29T11:08:12Z`와 계획 문구 대조. 통과. |
| 4 | #804 merge SHA를 source/head SHA로 오기 | 원격 `mergeCommit.oid` `85b41e75aaa4452355a8cb38d557a702604fa0cf`와 문구 대조. 통과. |
| 5 | #804 scene 수를 fixture 수와 혼동 | 구현 evidence 표의 Rows/Columns 10 scene 행렬과 `10/10`을 대조. 통과. |
| 6 | Android AVD 결과를 실기기로 확대 | 요약에 API 37 software AVD가 명시되고 실기기 주장은 없다. 통과. |
| 7 | Android 표본 수 또는 정확도를 과장 | evidence 표에서 PixelCopy `14/14 exact` 확인. 통과. |
| 8 | iOS Simulator 모델/버전을 바꿔 기록 | evidence 표에서 iPhone 17 Pro / iOS 26.2와 `10/10` 확인. 통과. |
| 9 | Metal drawable을 compositor 이후 화면으로 오인 | 계획 문구가 compositor 제출 전 표본임을 명시. 통과. |
| 10 | computed `collapse`를 `hidden`으로 덮어쓴 것으로 서술 | implementation/runtime evidence의 computed-style 보존 확인과 문구 대조. 통과. |
| 11 | collapse sizing과 `display:none`을 같은 결과로 서술 | 계획 요약에서 `hidden`과 같은 contribution/frame만 설명하며 별도 대조군을 남김. 통과. |
| 12 | visible descendant의 예외 동작을 누락해 무조건 subtree 숨김처럼 보임 | 연결된 상세 계획과 evidence가 visible descendant control을 포함함. 통과. |
| 13 | Grid 결과를 Flex/Table 규칙으로 일반화 | 요약의 scope는 C11.4 Grid fixture로 한정하고 일반 지원을 제외함. 통과. |
| 14 | #804 근거 링크의 상대 경로가 잘못됨 | 계획·precomparison·runtime·review 경로가 저장소에 존재함. 통과. |
| 15 | 계획의 기존 “계획 PR 병합 전” 문구가 남음 | 해당 stale 문구가 병합 결과와 제한 범위 설명으로 대체됨. 통과. |
| 16 | #796 PR 번호나 제목을 잘못 연결 | PR #796 제목 및 URL을 원격 PR과 대조. 통과. |
| 17 | #796 병합 시각 오기 | 원격 `2026-10-29T07:08:47Z`와 계획 문구 대조. 통과. |
| 18 | #796 merge SHA를 잘못 연결 | 원격 `mergeCommit.oid` `48100cc93b32c00a9fb50bde68e0c12fde78f08b`와 계획 문구 대조. 통과. |
| 19 | fixed-maximum 문법을 임의 `minmax()` 전체로 확대 | `auto|min-content|max-content` minimum과 fixed-breadth subset만 문서에 적음. 통과. |
| 20 | fixed maximum의 계산 기준 제약을 숨김 | indefinite percentage basis와 임의 calc AST를 미지원으로 남김. 통과. |
| 21 | #796 scene 수와 실행 결과가 맞지 않음 | 구현 evidence의 고정 49 scene과 Android/iOS 각 `49/49` 대조. 통과. |
| 22 | #796 surface sample 수를 합계/플랫폼별로 혼동 | Android PixelCopy 및 iOS Metal 표 각각 `104/104`임을 확인. 통과. |
| 23 | 서로 다른 surface 종류를 동등한 OS 화면 증거로 취급 | Android PixelCopy와 iOS Metal drawable을 별도 표면 계층으로 연결. 통과. |
| 24 | PR check rollup이 비었는데 CI 성공으로 주장 | #796 및 #804 모두 check 부재와 required check 미설정을 명시. 통과. |
| 25 | 제한 fixture slice가 일반 author CSS admission을 완료했다고 주장 | 두 기록 모두 일반 CSS admission을 제외함. 통과. |
| 26 | 계획 기록 때문에 공식 C11.4 checkbox가 완료된 것처럼 보임 | 두 기록 모두 C11.4를 미완료로 유지함. 통과. |
| 27 | 좁은 구현을 전체 C11.4 또는 전체 Grid로 확대 | 전체 WPT 및 C11.4 완료를 제외한다고 명시함. 통과. |
| 28 | 공식 집계가 변경된 것처럼 숫자를 바꿈 | 공식 `72/213`을 두 기록에서 그대로 유지. 통과. |
| 29 | 링크·공백 검사에서 회귀 | 계획 문서의 저장소 상대 링크 385개가 모두 존재하고 `git diff --check`가 통과함. 통과. |
| 30 | 검토 근거 파일이 계획 링크와 맞지 않거나 실제 검사 범위를 과장 | 링크 대상이 이 파일과 일치하며, 실행하지 않은 제품 테스트를 PR diff 검토 통과로 표현하지 않음. 통과. |

## 검증 경계

이 변경은 병합 결과와 계획 문서의 동기화다. 제품 코드나 공식 체크박스를 바꾸지 않아 Rust/앱 테스트는 다시 실행하지 않았다. 원본 구현의 Android/iOS 실행 및 제한은 연결한 각 runtime evidence에 기록되어 있다.
