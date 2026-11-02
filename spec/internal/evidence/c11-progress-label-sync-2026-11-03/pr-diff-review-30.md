# C11 진행 표기 정정 PR 변경 검토 30개

**검토 대상:** `plan/c11-4-grid-track-sizing.md`, `spec/STATUS.md`, 병합된 PR 근거 [#1052](https://github.com/ohah/spinon/pull/1052)와 [#1070](https://github.com/ohah/spinon/pull/1070).

각 항목은 계획 후보/병합 slice 표기와 완료 상태를 혼동할 수 있는 별도 실패 경로를 확인한다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | #1052가 실제 `main` 병합 전인데 완료로 쓰임 | **통과.** GitHub PR 상태는 merged다. |
| 2 | #1052 병합 시각이 다른 기록과 불일치 | **통과.** PR 기록의 `2026-11-02T10:29:53Z`를 유지한다. |
| 3 | #1052 merge SHA가 잘못됨 | **통과.** `472c5a6600603b2e69a0d42d5c5a1f68345094f3`를 PR 및 상태 근거와 대조했다. |
| 4 | zero font-size slice가 일반 CSS author admission으로 확대 해석됨 | **통과.** 고정 opt-in anonymous direct-text fixture로 한정한다. |
| 5 | 네 scene의 플랫폼 실행 수가 잘못 표시됨 | **통과.** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각각 4/4다. |
| 6 | fixture text-layout 전체 실행 수가 잘못됨 | **통과.** `99/99`를 유지한다. |
| 7 | fixture-enabled FFI ignored 수가 pass 수에 합산됨 | **통과.** `189 passed·2 ignored`로 구분한다. |
| 8 | feature runtime의 기존 실패가 누락됨 | **통과.** `425 passed·4 failed·3 ignored`를 명시한다. |
| 9 | default runtime의 기존 실패가 누락됨 | **통과.** `219 passed·2 failed·3 ignored`를 명시한다. |
| 10 | 위 실패가 이번 문서 정정에서 새로 발생한 것으로 오해됨 | **통과.** 실패 세부사항은 기존 실행 근거로 연결한다. |
| 11 | status의 #1052 구현 기록이 계획 index와 충돌 | **통과.** 계획 index를 PR 병합 상태에 맞춘다. |
| 12 | `font-size: 0`에서 glyph rasterization까지 지원한다고 오해 | **통과.** glyph rasterization 미포함을 적는다. |
| 13 | scene screenshot이 surface pixel readback으로 설명됨 | **통과.** pixel readback 근거라고 주장하지 않는다. |
| 14 | Android AVD가 hardware GPU 실기기로 오해됨 | **통과.** AVD 한계를 실행 근거에 맡기고 확대 주장하지 않는다. |
| 15 | 해당 slice가 CSS WPT 적합성을 증명한다고 오해 | **통과.** WPT를 지원 범위에서 명시적으로 제외한다. |
| 16 | #1052 하나로 C11.4 완료 checkbox가 승격됨 | **통과.** C11.4는 미완료라고 유지한다. |
| 17 | C11 parent가 완료된 것으로 표기됨 | **통과.** C11 parent checkbox는 변경하지 않는다. |
| 18 | 전체 완료 `72/213`이 증가함 | **통과.** 공식 수를 그대로 유지한다. |
| 19 | zero-font-size 계획 링크가 사라짐 | **통과.** 집중 계획 링크를 유지한다. |
| 20 | Chrome 사전 비교 근거 링크가 사라짐 | **통과.** 기존 precomparison README 링크를 유지한다. |
| 21 | 별도 계획 실패 검토가 계획 index에서 빠짐 | **통과.** 계획 검토 20개 링크를 유지한다. |
| 22 | #1070이 아직 candidate로 남아 문장 안의 merge 기록과 모순 | **통과.** candidate 문구를 제한 runtime slice로 정정한다. |
| 23 | #1070 구현 범위가 `grid-auto-rows` 목록 이외로 확대 | **통과.** 기존 px/%/fr implicit-row-list 범위만 표시한다. |
| 24 | #1070 PR 링크 또는 merge SHA 기록이 지워짐 | **통과.** 기존 링크와 병합 정보를 그대로 둔다. |
| 25 | #1070 범위를 전체 C11.1 지원으로 격상 | **통과.** 제한 slice 문맥을 유지한다. |
| 26 | status 이력의 실행 결과가 계획 문구로 대체됨 | **통과.** 상태 행은 기존 실행·merge 세부사항을 보존한다. |
| 27 | 변경이 내부 계약 숫자 버전 상승을 암시 | **통과.** 인터페이스 계약은 수정하지 않는다. |
| 28 | 공개 roadmap의 정확한 #1052 카드와 다른 상태를 새로 만듦 | **통과.** public roadmap의 기존 merged 기록과 일치시킨다. |
| 29 | 한글 문서 원칙을 위반하거나 제품 지원을 모호하게 표현 | **통과.** 기술 식별자 외의 설명은 한글이며 제한 범위를 직접 쓴다. |
| 30 | whitespace 또는 우발적 변경이 포함됨 | **통과.** `git diff --check`를 실행하고 두 의도 파일과 본 검토 기록만 변경한다. |

**결론:** 두 stale label을 확인된 merge 기록으로 정정했다. 코드, 공개 지원 범위, C11 체크박스와 공식 완료 수는 변경하지 않는다.
