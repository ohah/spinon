# C11.1 PR #1106 병합 상태·로드맵 문서 PR diff 검토 30개

검토 대상은 PR #1106 병합 정보를 `spec/STATUS.md`, 내부 계약 0059, 계획 문서와 공개 로드맵에 맞추는 문서 변경이다. 제품 코드 변경은 없다. 서로 다른 실패 경로를 살펴 기존 `72/213`과 제한 범위를 보존하는지 확인했다.

| # | 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 열린 PR을 병합 완료로 기록한다. | PASS — GitHub API에서 #1106 상태 `MERGED`와 `mergedAt=2026-11-03T04:59:34Z`를 확인했다. |
| 2 | PR의 마지막 문서 commit을 구현 source commit으로 오인한다. | PASS — merge SHA `e631b0e1…`와 구현 commit `5459a7d4…`를 따로 기록했다. |
| 3 | PR #1105 계획 병합과 구현 PR #1106을 한 PR로 합친다. | PASS — 계획과 제한 구현을 각각의 번호와 병합 역할로 표현한다. |
| 4 | 날짜나 병합 시각을 local time으로 변환해 혼동한다. | PASS — `2026-11-03T04:59:34Z` 원본 UTC 값을 유지한다. |
| 5 | `main` ancestry에 구현 commit이 없는데 병합됐다고 기록한다. | PASS — `origin/main` 로그에서 `5459a7d4…`가 `e631b0e1…`의 parent로 이어짐을 확인했다. |
| 6 | Rebase and merge 대신 squash/merge commit을 썼다고 적는다. | PASS — GitHub merge SHA와 연속된 두 main commit을 대조했다. |
| 7 | PR check가 비었는데 CI-green이라고 암시한다. | PASS — status, plan, contract와 roadmap에서 check rollup 부재 및 CI 성공 미주장을 구분한다. |
| 8 | main에 required status check가 있는데 없다고 적는다. | PASS — branch protection 응답에서 `required_status_checks=null`을 확인했다. |
| 9 | C11.1 부분 구현을 상위 완료로 승격한다. | PASS — C11.1/C11 체크박스와 상위 상태는 미완료로 유지한다. |
| 10 | 공식 완료 수를 72에서 올리거나 213 분모를 바꾼다. | PASS — status header와 새 roadmap card 모두 `72/213`을 명시한다. |
| 11 | current product support count를 새 CSS feature 수로 잘못 표시한다. | PASS — STATUS의 공식 지원 완료 없음 및 기존 완료 집계 규칙을 변경하지 않았다. |
| 12 | direct percentage admission을 전체 `fit-content()` 지원으로 넓혀 쓴다. | PASS — named-area 없는 explicit rows/columns와 finite non-negative direct percentage만 기술한다. |
| 13 | fixed repeat 지원을 arbitrary auto-repeat까지 넓혀 읽을 수 있다. | PASS — positive-integer fixed `repeat()` expansion으로 한정한다. |
| 14 | 한 축만 definite하면 충분하다고 쓴다. | PASS — 기존 gate가 두 Grid content dimensions 모두 definite하도록 요구함을 반복해 명시한다. |
| 15 | 반대 축 `auto` 제외를 plan/status 중 한 곳에서 빠뜨린다. | PASS — 상태 문서와 계획에 동일한 제한을 유지한다. |
| 16 | percentage basis를 border box나 viewport 기준으로 잘못 설명한다. | PASS — 같은 축 content-box basis와 15px 대 17px discriminator를 연결한다. |
| 17 | 0%, zero basis, 100% 초과를 invalid 또는 clamp 처리한다고 적는다. | PASS — 제한 테스트 근거에 맞춰 typed percentage 보존과 definite available-space 차이를 적는다. |
| 18 | auto minimum floor와 max-content ceiling을 누락한다. | PASS — plan/status와 roadmap card가 두 기여 경계를 함께 기술한다. |
| 19 | gap을 percentage basis에서 빼야 한다고 적는다. | PASS — gap 제외 계약을 유지한다. |
| 20 | Android AVD를 물리 기기 또는 hardware GPU로 표현한다. | PASS — API 37 ARM64 SwiftShader AVD와 software rendering 한계를 적었다. |
| 21 | iOS Simulator Metal drawable을 compositor 이후 화면으로 표현한다. | PASS — compositor 제출 전 표본임을 명시한다. |
| 22 | 두 플랫폼 `4/4` 표본을 전체 C11 시나리오 통과로 합산한다. | PASS — 단일 fixed overflow scene 증거로 한정하고 상위 완료를 주장하지 않는다. |
| 23 | `72/213`을 표시하는 카드가 최신 사실 위에 오래된 카드로 들어간다. | PASS — 새 #1106 card를 현재 최상단 C11.1 진행 기록 앞에 삽입했다. |
| 24 | 같은 #1106 카드가 중복 삽입된다. | PASS — source marker는 한 곳이며 기존 #1102 카드와 분리된다. |
| 25 | roadmap HTML `<p>`/`<a>` 경계가 깨져 뒤쪽 진행 기록이 카드 안으로 들어간다. | PASS — 신규 paragraph를 단일 닫힌 `<p>`로 구성하고 marker/링크 수를 검사했다. |
| 26 | local route 링크가 raw `.md` 또는 잘못된 base path를 가리킨다. | PASS — 카드 링크는 `/spinon/` 아래 Rspress HTML route 또는 실제 PNG route를 가리킨다. |
| 27 | 계획·runtime·review·screenshot 링크의 source file이 존재하지 않는다. | PASS — linked plan, evidence, review, Android/iOS, Chrome source files를 검사했다. |
| 28 | 내부 계약 숫자 버전을 작업 완료만으로 올린다. | PASS — 계약 0059는 `0.1.0`을 유지한다. |
| 29 | historical implementation checksum을 최신 문서에 맞춰 바꾸어 원래 PR 증거를 덮어쓴다. | PASS — 55개 항목의 candidate manifest를 PR #1106 merge tree `e631b0e1…` 기준으로 검증했고 원본을 보존한다. |
| 30 | Tailnet 배포가 아직 전인데 이미 동기화했다고 주장한다. | PASS — 이 PR의 범위는 merge 후 Tailnet 동기화 전 source 상태와 card다. 실제 제공 hash·desktop/mobile 확인은 별도 실행 근거에서만 보고한다. |

## 판정

이 문서 diff에서 공식 상태나 지원 범위를 과장하는 결함은 발견하지 못했다. Candidate PR evidence의 55개 SHA-256 항목은 병합 직후 `e631b0e1db352ed0832efb0c67b1fb44e64141ea` tree에서 일치했다. Tailnet 미리보기의 실제 제공 바이트는 별도 동기화 후 확인한다.
