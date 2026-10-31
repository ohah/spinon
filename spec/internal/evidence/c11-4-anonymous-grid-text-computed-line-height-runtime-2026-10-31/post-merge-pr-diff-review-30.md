# C11.4 anonymous Grid text computed `line-height` 병합 metadata PR diff 검토 30개

대상은 PR #905가 병합된 뒤 plan, official status, 내부 계약의 병합 metadata만 갱신하는 후속 문서 diff다. 이전 PR diff 30개와 병합 상태 일치성 검토 30개를 반복하지 않고 link·상태·범위 경계를 별도로 검사했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | official source가 Tailnet preview나 plan file로 바뀐다 | **통과.** `spec/STATUS.md`가 구현 상태의 원본임을 유지한다. |
| 2 | official completion counter가 merge 때문에 증가한다 | **통과.** `72/213` 그대로다. |
| 3 | C11 parent checkbox가 checked로 바뀐다 | **통과.** 미완료 상태를 유지한다. |
| 4 | C11.4 checkbox가 checked로 바뀐다 | **통과.** 제한 fixture 후보로만 설명한다. |
| 5 | merged PR을 열린 후보로 표시한다 | **통과.** #905의 merge 시각과 SHA를 기록한다. |
| 6 | merge commit SHA 대신 pre-rebase head SHA를 기록한다 | **통과.** `454036612372241fa7b162d6f87c78674ab7e559` 사용. |
| 7 | UTC merge time을 다른 날 또는 offset으로 기록한다 | **통과.** GitHub의 `2026-10-31T05:46:15Z` 사용. |
| 8 | `Rebase and merge` 결과를 squash/merge commit으로 쓴다 | **통과.** 실제 병합 방법을 기록했다. |
| 9 | 빈 required-check rollup을 성공적인 CI 결과로 해석한다 | **통과.** 필수 checks와 보고된 check이 없었다고 쓴다. |
| 10 | 계획 PR #903과 구현 PR #905를 혼동한다 | **통과.** metadata는 구현 PR #905만 가리킨다. |
| 11 | branch 삭제 뒤 plan link가 깨진다 | **통과.** 이미 병합된 계획 링크를 `main` URL로 정리한다. |
| 12 | plan의 작업 상태가 merge 이전 문구로 남는다 | **통과.** PR merge 완료와 남은 전체 범위를 함께 표시한다. |
| 13 | status에서 현재 구현이 review 단계에만 머문다 | **통과.** 현재 제한 구현 근거와 PR merge를 표시한다. |
| 14 | internal contract가 PR 미생성이라고 말한다 | **통과.** 해당 stale 문구를 교체했다. |
| 15 | internal interface numeric version이 범위 추가로 상승한다 | **통과.** 0059/0070의 `0.1.0`을 유지한다. |
| 16 | plan link가 다른 C11.4 task를 가리킨다 | **통과.** line-height plan path를 사용한다. |
| 17 | precomparison 근거가 빠져 기대값 provenance를 잃는다 | **통과.** Chrome 156 precomparison link 유지. |
| 18 | runtime evidence가 빠져 Simulator 주장 근거를 잃는다 | **통과.** platform matrix/evidence README link 유지. |
| 19 | implementation review link가 계획 검토를 대신한다 | **통과.** 구현/runtime review와 plan review를 따로 연결한다. |
| 20 | PR diff review link가 구현/runtime review를 대신한다 | **통과.** 각 review artifact를 분리해 연결한다. |
| 21 | post-merge state review가 구현 실패 경로 검토로 오인된다 | **통과.** 병합 후 plan/contract/status 일치만 검토한다. |
| 22 | C15 general text contract 변경까지 의미한다고 읽힌다 | **통과.** C15 generic scope는 그대로다. |
| 23 | finite zero line-height slice가 inline-run 지원 확장으로 읽힌다 | **통과.** `InlineRunStyle`의 기존 경계를 유지한다. |
| 24 | fixed Ahem profile이 general font 지원으로 승격된다 | **통과.** 10px Ahem/weight 400 제한을 유지한다. |
| 25 | AVD/Simulator를 실기기 evidence로 바꿔 쓴다 | **통과.** platform 이름과 미검증 범위를 유지한다. |
| 26 | route/status image를 glyph pixel 증거로 표기한다 | **통과.** glyph paint 및 compositor pixel 미검증을 유지한다. |
| 27 | candidate update에서 WPT/general CSS support를 암시한다 | **통과.** 둘 다 미완료·미지원 범위로 둔다. |
| 28 | docs build dead-link 실패를 green으로 표기한다 | **통과.** 1,131 file-level dead-link report를 성공과 분리한다. |
| 29 | Tailnet preview를 공식 state authority로 부른다 | **통과.** preview는 read-only projection이다. |
| 30 | docs follow-up에 무관한 source/evidence files가 섞인다 | **통과.** 변경은 plan, `spec/STATUS.md`, contract와 이 review artifact로 제한된다. |

**판정:** metadata는 PR #905의 실제 merged state와 일치하며 C11/C11.4의 incomplete 상태와 공식 `72/213`을 보존한다. 변경은 문서 전용이고 implementation code를 수정하지 않는다.
