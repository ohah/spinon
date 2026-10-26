# C11.4 row-axis overlapping-span 계획 독립 검토 20관점

**검토 대상:** [row-axis span-stage 계획](./c11-4-row-axis-span-stage-oracle.md), 고정 Chrome 입력·관찰 및 독립 `Fraction` 기대값. 계획 PR의 실제 변경 diff review는 별도 문서에서 수행한다.

| # | 실패 관점 | 확인한 근거와 판정 |
| ---: | --- | --- |
| 1 | 계획 단계 결과가 C11.4 완료나 공식 수 증가로 과장되는가? | 계획·precomparison만 추가하며 C11.4 checkbox와 `72/213`을 유지한다. **PASS** |
| 2 | 기존 column-axis slice와 신규 row-axis slice가 중복 주장되는가? | 기존 테스트는 `Columns` pass만 선택하고 새 scope는 `Rows` pass를 직접 선택하도록 분리했다. **PASS** |
| 3 | CSS Grid row/column이 물리 방향과 혼동되는가? | 문서·case id·typed matcher는 logical `Rows` axis로 고정하고 horizontal writing mode input을 쓴다. **PASS** |
| 4 | row contribution을 column width로 잘못 입력하는가? | A/B/C의 80/120/200px를 item `height` 및 block-axis max-content contribution으로 매핑한다. **PASS** |
| 5 | explicit row sizing과 definite container block size가 빠지는가? | 네 `minmax(0px,max-content)` row와 definite 220px `height`를 고정한다. **PASS** |
| 6 | column sizing이 row stage에 섞여 expected values를 바꾸는가? | 하나의 fixed 220px column만 두고 그 pass는 early return control로 분리한다. **PASS** |
| 7 | span-2 경쟁 item이 track을 실제로 겹치는가? | A rows 1–2, B rows 2–3이 row 2를 공유하며 A/B contributions는 각각 80/120px다. **PASS** |
| 8 | span-3 item이 정확히 앞의 두 span-2 batches 이후 계산되는가? | C rows 1–3, 200px이며 규범적 batch sequence `[2,3]`를 기대한다. **PASS** |
| 9 | source order reversal이 placements까지 바꾸는가? | forward/reversed controls는 DOM order만 다르고 IDs별 explicit row placement와 size를 유지한다. **PASS** |
| 10 | same span group의 overlapping planned increase를 덧셈하는가? | A의 40px와 B의 60px 증가가 overlap row에서 `max`로 적용되어 `[40,60,60,∞]`가 된다. **PASS** |
| 11 | span-3 extra space 계산이 초기값만 사용하거나 잘못 나누는가? | C의 200px에서 현재 160px growth sum을 빼고 남은 40px을 세 row에 `40/3`씩 분배한다. **PASS** |
| 12 | span 밖 네 번째 row가 batch 뒤 잘못 증가하는가? | 네 번째 growth limit은 batch 동안 무한 상태로 남고 `IntrinsicSizingComplete`에서 zero base로 닫힌다. **PASS** |
| 13 | `None` trace limit을 실제 positive infinity 증명으로 과장하는가? | serializer의 non-finite 합침을 명시하고 `+∞`와 NaN/`-∞`를 구분하지 않는 한계를 유지한다. **PASS** |
| 14 | `MaximizeTracks`의 free-space 계산을 잃는가? | span-2 only는 60px, span-2/3는 20px가 남는다고 exact oracle에서 고정한다. **PASS** |
| 15 | gutter와 CSS track index가 뒤섞이는가? | 기존 helper의 boundary-gutter, zero-gap, collapsed flag 및 CSS track count 검사를 재사용한다. **PASS** |
| 16 | Chrome quantization을 규범적 이상값으로 대체하는가? | Chrome의 1/64px `[53.328125,73.328125,73.34375]`와 규범적 thirds를 각각 기록한다. **PASS** |
| 17 | final item frame으로 내부 stage를 증명한다고 오해하는가? | Chrome은 final rows/frame만 관찰하고 stage snapshots는 §11.5.1/§11.6에서 산출한다고 구분한다. **PASS** |
| 18 | item frame 비교가 root origin이나 source order 변화에 취약한가? | Chrome·Rust frame은 root-relative y와 item ID로 대조하고 order reversal 별도 control을 둔다. **PASS** |
| 19 | 새 구현이 product code 또는 platform artifact를 건드리는가? | 새 Rust file은 기존 test-only module의 child이며 FFI/app/runtime/API 변경은 scope 밖이다. **PASS** |
| 20 | 계획 PR이 구현 PR 검증이나 CI를 대신하는가? | 계획 PR 뒤 별도 최신-main 구현 PR, 새 implementation review 20, PR diff review 20으로 분리한다. Batch-final snapshots와 hardcoded fixture model의 한계도 명시하며, 미실행 검사를 통과로 주장하지 않는다. **PASS** |

**결론:** 계획 입력·독립 산술·축 경계·공식 상태 경계는 서로 일치한다. 검토 결과가 결함을 찾으면 계획과 precomparison부터 수정하고 영향을 받은 항목을 다시 검토한다. 이 표는 구현 결과나 PR diff 검토가 아니다.

## 독립 hostile review 재검토

별도 agent의 첫 검토는 **17 PASS / 3 PARTIAL / 0 FAIL**이었다. 발견된 범위 불일치는 반영했다: Grid item HTML을 Rust builder와 같은 빈 `div`로 맞췄고, frame 판정에 root `(x,y,width,height)`와 item `x/width`를 추가했으며, CSSOM의 소수 넷째 자리와 DOMRect 좌표의 1/64px 정밀도를 분리했다. 사전비교 fixture·screenshot을 다시 캡처한 뒤 SHA256SUMS 5/5를 확인했다. 별도 agent가 다시 검토한 최종 결과는 **20 PASS / 0 PARTIAL / 0 FAIL**이다. 계획과 precomparison 수정 사항을 다시 대조했고 미해결 blocker는 없다.

추가 oracle audit는 20개 failure perspectives에서 값·구조 불일치를 찾지 못했다. 이 고정 스크립트는 HTML을 자동 파싱하지 않고, trace의 `IntrinsicItemBatch` 사이즈 서브패스 중간값도 모델링하지 않는다. 두 제한을 README와 계획에 밝혀 batch 종료값·현재 해시를 검토 대상으로 한정했다.
