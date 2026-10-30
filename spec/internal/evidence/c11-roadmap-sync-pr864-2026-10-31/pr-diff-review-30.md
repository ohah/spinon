# C11.4 상위·상세 계획 상태 동기화 PR diff 검토 30개

**검토 범위:** `plan/c11-css-grid.md`의 C11.4 현황 행과 `plan/c11-4-grid-track-sizing.md`의 PR #850/#861 실행 기록 추가.

| # | 독립 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | C11.4 slice를 상위 C11 완료로 과장함 | PASS — C11.4와 C11 모두 미완료라고 유지한다. |
| 2 | `72/213`을 C11 전용 진행률로 표시함 | PASS — 저장소 전체 공식 집계라고 명시한다. |
| 3 | C11.5/C11.6 완료로 오해하게 만듦 | PASS — 수정하지 않았고 미완료 범위를 그대로 둔다. |
| 4 | C14.3/C15 선행 작업이 끝난 것으로 표시함 | PASS — intrinsic image/text prerequisites를 계속 남긴다. |
| 5 | PR #837 ceiling test를 CSS 지원으로 확대함 | PASS — test-only로 한정했다. |
| 6 | PR #842 ratio 값을 fixture 밖으로 일반화함 | PASS — limited direct-text ratio profile로만 쓴다. |
| 7 | PR #847 `auto <ratio>` content-box transfer를 일반 projection으로 확대함 | PASS — 제한 fixture라는 상위 경계를 유지한다. |
| 8 | PR #850 scene 수가 틀림 | PASS — 42 scene과 기존 23 + 신규 19의 구분이 evidence와 맞다. |
| 9 | PR #850 surface sample 수/정확성이 틀림 | PASS — 양 플랫폼 129/129 exact를 기록한다. |
| 10 | Android PR #850 결과를 실기기·hardware GPU라고 표시함 | PASS — API 37 ARM64 QEMU AVD로 특정한다. |
| 11 | iOS PR #850 drawable을 compositor 후 pixel로 표시함 | PASS — compositor 제출 전 경계를 유지한다. |
| 12 | PR #861 normal control의 track/frame이 뒤바뀜 | PASS — 20px track, 20×20px root와 두 줄을 기록한다. |
| 13 | PR #861 nowrap track/item/root가 혼동됨 | PASS — 50px track·50×10px item·20×10px root를 분리한다. |
| 14 | nowrap overflow 크기가 틀림 | PASS — root 밖 30px overflow를 기록한다. |
| 15 | draw/present trace를 glyph pixel proof로 확대함 | PASS — glyph pixel 동일성을 주장하지 않는다고 적는다. |
| 16 | Android/iOS scene 범위를 다른 플랫폼으로 확대함 | PASS — Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 특정한다. |
| 17 | 일반 author CSS admission을 추가한 것으로 읽힘 | PASS — fixture boundary와 일반 CSS 미완료를 함께 적었다. |
| 18 | WPT 전체 통과를 주장함 | PASS — 전체 WPT를 남은 범위로 유지한다. |
| 19 | §11.3–§11.8 stage 전체를 완료로 표시함 | PASS — 전체 stage/contribution matrix를 미완료로 둔다. |
| 20 | 일부 조합 완료를 모두 미지원이라고 과장함 | PASS — 개별 slices와 전체 교차 조합을 구분한다. |
| 21 | auto-repeat ceiling과 ratio/nowrap 변경이 뒤섞임 | PASS — test-only ceiling과 runtime fixture 구현을 나눠 적었다. |
| 22 | C11.4 상위 표에서 detailed plan 링크가 깨짐 | PASS — `c11-4-grid-track-sizing.md`가 존재한다. |
| 23 | PR #837 두-track ceiling 근거 링크가 깨짐 | PASS — 집중 계획 route를 확인했다. |
| 24 | PR #842/#847/#850/#861 번호가 오기됨 | PASS — 링크를 각 병합 PR에 대조했다. |
| 25 | PR #850 계획/evidence 링크가 빠짐 | PASS — plan과 runtime README를 연결한다. |
| 26 | PR #861 plan/precomparison/runtime 링크가 빠짐 | PASS — 세 자료를 모두 연결한다. |
| 27 | 상세 계획 변경이 chronology에서 PR #847보다 앞에 놓임 | PASS — #850과 #861 실행 기록을 #847 뒤에 추가했다. |
| 28 | 기존 계획 검증 결과를 새 기능 검증처럼 재사용함 | PASS — 이 PR은 상태 문서 갱신이며 새 구현·새 실행을 주장하지 않는다. |
| 29 | 새 relative link가 빠진 파일을 가리킴 | PASS — diff에 추가된 14개 local link target이 모두 존재한다. 전체 기존 문서 링크 검증은 주장하지 않는다. |
| 30 | 문서 변경에 코드/test 통과를 주장함 | PASS — diff check만 사용했고 전체 site build·WPT·코드 검증을 주장하지 않는다. |

전체 C11.4/C11 지원 또는 전체 WPT 적합성을 이 계획 갱신으로 완료 처리하지 않는다.
