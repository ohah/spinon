# PR #1147 진행 기록 로드맵 문서 diff 검토 30개

검토 대상은 `spec/public/roadmap.html`의 새 C11.1 카드, `plan/c11-root-html-grid-mixed-explicit-one-axis.md`의 구현 PR 상태 수정, 이 검토표다. 구현 PR의 merged metadata와 고정 runtime evidence를 대조했다. 30개 실패 관점은 서로 다른 사실·범위·링크 오류를 확인한다.

| # | 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | 다른 PR을 구현 근거로 잘못 연결 | `gh pr view 1147` 제목·MERGED 상태를 대조했다. PR #1147이다. PASS. |
| 2 | merge timestamp 오기 | GitHub mergedAt `2026-11-04T06:01:15Z`와 plan·roadmap을 대조했다. PASS. |
| 3 | merge SHA 오기 | GitHub merge commit `c3b3c3d0ec8f841562b0a724ff596d65b0a3b83a`와 두 문서를 대조했다. PASS. |
| 4 | candidate head SHA를 merge SHA로 표기 | 문서는 GitHub merge SHA를 기록하고 candidate head를 혼동하지 않는다. PASS. |
| 5 | 제한 구현을 C11.1 전체 완료로 표시 | 카드와 plan이 C11.1/C11 미완료를 유지한다. PASS. |
| 6 | 공식 완료 수가 올라간 것처럼 표시 | 카드와 evidence가 `72/213`을 유지한다. PASS. |
| 7 | C11 체크박스를 암묵적으로 완료 처리 | 연결한 STATUS의 C11 checkbox는 unchecked이며 문서에 완료 주장이 없다. PASS. |
| 8 | 전체 CSS author input 지원으로 범위 확대 | 구현 계약은 기본 parsed Root HTML 제한 profile과 no-area owner 하나로 한정된다. PASS. |
| 9 | unsupported partial axis 조합을 수락한다고 오해 | 혼합 row/column partial axis는 명시적으로 범위 밖이다. PASS. |
| 10 | fully definite item 선점 순서를 뒤집음 | 구현 계획은 fully definite items가 occupancy를 먼저 예약한다고 기록한다. PASS. |
| 11 | named lines/areas 지원을 과장 | roadmap에서 named lines/areas는 범위 밖이다. PASS. |
| 12 | implicit track extent까지 지원한다고 과장 | 추가 implicit extent는 범위 밖이다. PASS. |
| 13 | WPT 적합성을 주장 | roadmap은 WPT 적합성을 주장하지 않는다. PASS. |
| 14 | Chrome 관측 장면 수 불일치 | precomparison README의 9 scenes와 roadmap의 9 scenes가 일치한다. PASS. |
| 15 | Chrome visible frame 수 불일치 | 독립 비교 기록과 roadmap은 19 visible frames를 사용한다. PASS. |
| 16 | Chrome 허용 오차를 숨김 | 비교 기록의 최대 차이 0 CSS px와 roadmap 문구가 일치한다. PASS. |
| 17 | Android 결과를 실기기로 확대 | 환경은 Android API 37 AVD로 표기하며 실기기 검증을 주장하지 않는다. PASS. |
| 18 | Android readback 종류를 혼동 | 실행 README는 PixelCopy 19/19 exact로 기록한다. PASS. |
| 19 | iOS 결과를 실기기로 확대 | 환경은 iPhone 17 Pro Simulator/iOS 26.2로 명시한다. PASS. |
| 20 | iOS compositor 뒤까지 검증했다고 주장 | Metal drawable은 OS compositor 제출 전이라고 명시한다. PASS. |
| 21 | Android/iOS 표본 수 불일치 | 최종 simulator evidence와 roadmap 모두 플랫폼별 19/19다. PASS. |
| 22 | layout focused test count 오류 | 최종 log의 placement layout module 결과 9/9와 일치한다. PASS. |
| 23 | Root HTML integration count 오류 | 최종 log의 integration module 결과 8/8과 일치한다. PASS. |
| 24 | selector focused test count 오류 | 최종 log의 selector 결과 3/3과 일치한다. PASS. |
| 25 | 넓은 suite를 전부 통과로 표현 | roadmap은 package suite를 431/432로 표시한다. PASS. |
| 26 | C15 실패를 숨기거나 잘못 귀속 | 보존된 log의 실패는 `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order`다. PASS. |
| 27 | baseline 비교 없이 C15 실패가 기존 문제라고 단정 | 문서는 실패를 보고하지만 baseline에서 재현됐다고 주장하지 않는다. PASS. |
| 28 | GitHub checks가 없는데 CI green으로 표현 | PR check 미보고 및 main required checks 부재를 기록하고 CI 성공을 주장하지 않는다. PASS. |
| 29 | 카드의 새 local route가 소스와 다른 경로 | plan, contract, precomparison, runtime, review, STATUS route가 실제 tracked source와 대응한다. PASS. |
| 30 | plan의 PR 진행 상태가 병합 후에도 open으로 남음 | stale `merge remains` 문장을 실제 mergedAt·SHA·check 상태로 교체했다. PASS. |

이 검토는 문서의 값·범위·경로를 고정 source와 대조한다. Tailnet 게시 성공이나 전체 문서 사이트 링크 검증을 주장하지 않는다.
