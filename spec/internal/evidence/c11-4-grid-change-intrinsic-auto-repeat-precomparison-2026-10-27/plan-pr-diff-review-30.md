# auto-repeat intrinsic-size 계획 PR 변경분 · 적대 검토 30관점

검토 대상은 upstream `main` `195ce166e51c993da90a0efe20fb6dd982943584` 기준 계획 PR 변경분 전체다. 계획 검토 20개와 별도로 문서·oracle·증거 경계 및 PR payload를 다시 확인했다.

| # | 변경분 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | PR base가 계획 작성 당시 `main`과 다를 수 있음 | **PASS.** 원격 `main`은 base `195ce166`과 일치한다. |
| 2 | 로컬에 다른 파일이 함께 올라갈 수 있음 | **PASS.** stage 대상은 계획·계획 검토·상태/계획 인덱스·고정 evidence 폴더로 제한한다. task 밖 `wpt-grid-change-intrinsic-initial.png`는 변경분에 포함하지 않는다. |
| 3 | 계획 PR이 공식 C11.4 완료로 보일 수 있음 | **PASS.** 상태 대장은 C11.4 unchecked를 유지하고 새 기록도 구현 전 계획이라 적는다. |
| 4 | 오래된 집계가 현재값으로 노출될 수 있음 | **PASS.** 계획과 상태 기록은 현재 공식 `72/213`만 사용한다. |
| 5 | C11.4 누적 기록의 상대 순서나 기존 근거가 삭제될 수 있음 | **PASS.** 기존 긴 C11.4 문단은 보존하고 새 계획 기록만 추가했다. |
| 6 | 상위 C11.4 계획의 최신 진행 현황에서 새 lane이 누락될 수 있음 | **PASS.** 상위 계획 첫 기록에 세부 계획과 oracle 링크를 연결한다. |
| 7 | auto-repeat + implicit track list 후보를 중복 구현할 수 있음 | **PASS.** 이미 병합된 PR #391을 계획에서 제외 대상으로 확인했다. |
| 8 | 특정 WPT checkout이나 파일이 바뀌면 입력 재현이 불가능할 수 있음 | **PASS.** WPT revision, test path, helper hash를 manifest에 고정했다. |
| 9 | 검증 범위가 testharness 밖의 browser state를 포함하는 것처럼 보일 수 있음 | **PASS.** README와 JSON은 HeadlessChrome 156의 해당 WPT harness 8/8로 한정한다. |
| 10 | Agent Browser 캡처가 WPT test 결과 화면인지 확인하기 어려울 수 있음 | **PASS.** 두 full-page screenshot은 harness summary 8 Pass와 마지막 rendered grid를 함께 보인다. |
| 11 | `full` screenshot과 viewport screenshot이 헷갈릴 수 있음 | **PASS.** README/PR image는 full-page 이름의 파일만 가리킨다. |
| 12 | viewport screenshot 중복을 새 독립 증거로 세어 부풀릴 수 있음 | **PASS.** duplicate viewport images는 full-page proof로 링크하지 않으며 PR 변경 목적도 확대하지 않는다. |
| 13 | source hash와 checked-in WPT input이 달라질 수 있음 | **PASS.** test HTML과 helper hashes는 checkout 내용으로 기록했고 validator가 pinned manifest를 읽는다. |
| 14 | expected JSON이 Chrome 결과를 그대로 복사한 값일 수 있음 | **PASS.** README는 CSS Grid/Sizing 계산과 Chrome observation을 분리하며 validator는 독립 expected와 관측을 대조한다. |
| 15 | repeat count와 explicit tail track 합계가 틀릴 수 있음 | **PASS.** 각 상태의 count와 expanded columns가 expected JSON에서 함께 고정된다. |
| 16 | min-content item 15px에서 min-width floor 누락이 숨을 수 있음 | **PASS.** min-content 15px 상태는 30px root floor 및 `[20,10]` tracks를 별도 기대값으로 둔다. |
| 17 | border/padding을 content width 또는 coordinate에 중복 적용할 수 있음 | **PASS.** content width와 border-box size를 분리하고 README는 root-relative helper coordinates를 설명한다. |
| 18 | WPT root의 positioned containing block 조건이 빠질 수 있음 | **PASS.** 계획은 root `position:relative` 및 probe containing block을 고정한다. |
| 19 | absolute probes가 intrinsic contribution에 합산될 수 있음 | **PASS.** 고정 입력 설명은 probes의 out-of-flow 역할과 sizing 제외를 명시한다. |
| 20 | track trace 없이 final frame만으로 repeat sizing을 주장할 수 있음 | **PASS.** 계획 통과 기준에는 typed used tracks와 runtime repeat trace/count가 모두 있다. |
| 21 | root-relative WPT offsets가 viewport absolute frames로 혼동될 수 있음 | **PASS.** 계획은 좌표계를 구분해 기록하고 expected JSON은 두 종류를 명시한다. |
| 22 | CSS 오차 허용치가 evidence validator와 다를 수 있음 | **PASS.** 계획의 `1/64 CSS px`와 JSON의 `0.015625`가 일치한다. |
| 23 | same-document 재계산 조건이 app fixture의 독립 scene으로 대체될 수 있음 | **PASS.** runtime sequence와 simulator matrix의 역할/증거 한계를 따로 적는다. |
| 24 | runtime 스타일 revision과 source revision이 서로 다르게 증가할 수 있음 | **PASS.** sequence acceptance는 generation/node identity 고정, document/style revision 증가, environment revision 고정을 분리한다. |
| 25 | 스타일 변경 후 stale layout/trace가 현재 결과로 남을 수 있음 | **PASS.** 각 단계에서 revision stamp, trace, draw/present 확인을 요구한다. |
| 26 | 계획 변경으로 공개 API나 내부 계약 버전이 암묵적으로 바뀔 수 있음 | **PASS.** public API 제외, 내부 `0.1.0` 유지 및 공식 status 미승격을 기록한다. |
| 27 | 지원하지 않는 CSS grammar를 고정 fixture가 받아들일 수 있음 | **PASS.** opt-in profile과 root/style/topology allowlist를 계획에 넣었다. |
| 28 | Android/iOS simulator evidence가 실기기/hardware GPU로 일반화될 수 있음 | **PASS.** simulator와 hardware evidence를 구분하고 나머지 범위를 유지한다. |
| 29 | 리뷰 기록이 구현 후 review 의무를 대신할 수 있음 | **PASS.** 계획 review와 새 구현/runtime review, PR diff review를 각각 요구한다. |
| 30 | PR 본문 또는 라벨이 작업 유형·영향 영역을 전달하지 못할 수 있음 | **PASS.** `planning` 및 layout/docs/testing 영역 라벨, 상태 ID·oracle·남은 범위를 양식에 맞춰 기재한다. |

**판정:** PR 변경분 관점 30/30 PASS. PR에는 계획/oracle 단계만 포함하고 구현 완료, simulator pass, 전체 C11.4 완료를 주장하지 않는다.
