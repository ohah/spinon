# PR #1326 이후 문서·roadmap diff 실패 관점 검토 30개

대상은 PR #1326 merge 후 내부 계약 0059, 공식 STATUS, 구현 계획, 공개 roadmap, 실행 근거와 Tailnet 공개본을 동기화하는 문서 변경이다. 공식 원본은 `spec/STATUS.md`이며 제한 구현 slice를 C11.4/C11 전체 완료로 승격하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | implementation PR 번호가 틀리는가 | 통과. 구현 PR #1326 링크를 사용했다. |
| 2 | merge 시각이 다른가 | 통과. `2026-11-10T05:39:19Z`로 기록했다. |
| 3 | merge SHA가 다른가 | 통과. `b5925aa274718a16c4e8f0bd9ef4bed4423aec6c`로 기록했다. |
| 4 | 계획 PR #1324와 구현 PR #1326을 혼동하는가 | 통과. 계획 기록과 실제 구현·병합 기록을 구분했다. |
| 5 | 지원하는 DOM/CSS 범위를 과장하는가 | 통과. parsed Root HTML repeated-text row-span one-fit profile의 제한 slice라고 적었다. |
| 6 | 지원 위치 1–3과 세 implicit row 조건이 빠지는가 | 통과. 내부 계약·STATUS에서 범위를 명시했다. |
| 7 | 모든 text item의 `min-height:auto`와 control 조건을 누락하는가 | 통과. span text item과 첫 explicit-row control의 `min-height:0`을 적었다. |
| 8 | 제외된 six-row positions 4–6, two-fit, auto-placement를 지원하는 것처럼 쓰는가 | 통과. 제외 범위를 명시했다. |
| 9 | 제한 fixture를 일반 CSS admission으로 넓히는가 | 통과. 일반 CSS admission을 지원한다고 주장하지 않는다. |
| 10 | paired matrix의 baseline/candidate 구분이 흐려지는가 | 통과. 각 2,880 scene과 합계 5,760 layouts로 분리했다. |
| 11 | scene 수와 layout 수를 이중 합산하는가 | 통과. 합계와 각 cohort를 한 번씩만 표기했다. |
| 12 | glyph line count와 frame/track 결과가 빠져 비교가 얕아지는가 | 통과. glyph, track/item/payload frame 및 trace를 나열했다. |
| 13 | rejected input·atomic recovery 근거를 생략하는가 | 통과. invalid fit position, zero-cap trace와 recovery evidence를 연결했다. |
| 14 | visible overflow 0을 전체 web 적합성으로 오해하게 하는가 | 통과. 결과를 이 제한 matrix로 한정했다. |
| 15 | Android AVD를 실기기로 표현하는가 | 통과. API 37.1 ARM64 AVD로 기록했다. |
| 16 | iOS Simulator를 실기기로 표현하는가 | 통과. iPhone 17 Pro / iOS 26.2 Simulator로 기록했다. |
| 17 | visual smoke를 표면 pixel readback으로 과장하는가 | 통과. surface readback이 아님을 명시했다. |
| 18 | Android software renderer 조건을 감추는가 | 통과. simulator 화면 근거에서 software renderer를 별도로 분류했다. |
| 19 | GitHub checks 부재를 CI green으로 표현하는가 | 통과. check-run/status 0개, CI green 아님을 명시했다. |
| 20 | broad suite의 실패를 숨기거나 clean-main baseline으로 오인시키는가 | 통과. 두 package 결과와 baseline 미검증을 같이 기록했다. |
| 21 | C11.4/C11 공식 checkbox를 완료로 올리는가 | 통과. 두 상태 모두 미완료로 유지했다. |
| 22 | 공식 완료 수를 임의 변경하는가 | 통과. `78/214`를 유지했다. |
| 23 | internal contract version을 근거 없이 바꾸는가 | 통과. `0.1.0`을 유지했다. |
| 24 | STATUS의 최신 C11.4 slice 설명이 현재 단계와 맞지 않는가 | 통과. #1326을 병합된 제한 구현으로 기록하고 직전 계획 후보를 구분했다. |
| 25 | 공개 roadmap 상단 카드가 이전 PR #1322에 머무는가 | 통과. 최근 병합 카드와 요약을 #1326으로 교체했다. |
| 26 | #1324 계획 카드에 구현 미시작 문구가 남는가 | 통과. #1326 구현·merge와 실제 검증 범위로 갱신했다. |
| 27 | 공개 카드의 계획·실행 근거 route가 잘못되는가 | 통과. 선택 publication에 계획·실행 route가 있고 pass03 staging 검사도 통과했다. |
| 28 | Tailnet 동기화가 다른 파일을 삭제하거나 서버를 재시작하는가 | 통과. route 삭제 0, server restart 없음으로 기록했고 260개 hash를 확인했다. |
| 29 | 교체 전 Tailnet 파일 rollback 자료가 누락되거나 검증되지 않는가 | 통과. 64개 backup의 SHA-256을 64/64 확인하고 외부 backup을 보존했다. |
| 30 | browser/simulator cleanup이 다른 작업의 공유 데이터를 지우는가 | 통과. Agent Browser 전용 process는 닫았고, S03 profile이 없음을 확인했으며 공유 simulator data는 보존했다. |

## 결과

30개 관점에서 병합 metadata 오류, 지원 범위·검증 과장, 공식 상태 변경, 공개 링크 누락, Tailnet 삭제·hash 오류 및 공유 simulator 데이터 손실을 찾지 못했다. 이 문서 PR은 병합 상태 동기화와 실행 근거를 기록하며 구현 동작이나 공식 완료 수를 추가로 변경하지 않는다.
