# PR post-merge roadmap diff review 30

검토 대상은 C11.3.3 fit-content cap 병합 뒤 상태 대장, 계획, 공개 roadmap에 병합 결과를 반영하는 문서 변경이다. C11의 완료 판정이나 지원 범위는 넓히지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR 번호가 실제 병합한 변경과 일치하는가 | PASS — PR #916 |
| 2 | 병합 시각이 GitHub 응답과 일치하는가 | PASS — 2026-10-31T09:59:59Z |
| 3 | merge SHA가 GitHub의 mergeCommit과 일치하는가 | PASS — 2c8d358e6d20b2cc63e2c75f4253056fded60c44 |
| 4 | 병합 방식이 저장소의 Rebase and merge 흐름과 일치하는가 | PASS |
| 5 | 빈 check rollup을 성공한 CI로 확대 해석하지 않는가 | PASS |
| 6 | 문서가 PR의 확인되지 않은 review approval을 주장하지 않는가 | PASS |
| 7 | 공식 구현 완료 수가 72/213으로 유지되는가 | PASS |
| 8 | C11 상위 체크박스가 미완료인가 | PASS |
| 9 | C11.3.3 candidate 체크박스가 미완료인가 | PASS |
| 10 | 병합 기록과 제품 지원 완료를 구분하는가 | PASS |
| 11 | 새 scene의 독립 기준이 Chrome 156임을 유지하는가 | PASS |
| 12 | Ahem text min-content 값이 20px인가 | PASS |
| 13 | Ahem text max-content 값이 110px인가 | PASS |
| 14 | 제한 cap의 기대 track 크기가 40px인가 | PASS |
| 15 | item frame 기대값이 x=20, y=0, width=40, height=60인가 | PASS |
| 16 | Android 근거가 API 37 AVD로 특정되는가 | PASS |
| 17 | Android 검증을 실기기로 표현하지 않는가 | PASS |
| 18 | Android PixelCopy backend 한계가 SwiftShader로 표시되는가 | PASS |
| 19 | 새 Android scene 표본 수가 3/3인가 | PASS |
| 20 | iOS 근거가 iPhone 17 Pro / iOS 26.2 Simulator로 특정되는가 | PASS |
| 21 | iOS 검증을 실기기로 표현하지 않는가 | PASS |
| 22 | 새 iOS scene 표본 수가 3/3인가 | PASS |
| 23 | iOS Metal readback의 compositor 전 한계를 보존하는가 | PASS |
| 24 | 기존 행렬이 39 scenes·126 nodes로 표시되는가 | PASS |
| 25 | 기존 행렬의 geometry 오차 0 CSS px 범위가 보존되는가 | PASS |
| 26 | hardware GPU 검증을 주장하지 않는가 | PASS |
| 27 | 일반 CSS author admission을 완료로 넓히지 않는가 | PASS |
| 28 | 전체 WPT 적합성을 완료로 넓히지 않는가 | PASS |
| 29 | #911의 기존 공개 roadmap 근거가 제거되지 않는가 | PASS |
| 30 | 모바일 overflow와 새 내부 링크를 확인했는가 | PASS — Agent Browser 390px에서 document width 390px/card 362px, 새 계획·실행 근거 HTTP 200; [측정·hash](postmerge-browser-verification-pass01.json) |

GitHub에서 병합 상태와 SHA를 다시 조회했다. 이 표는 문서 주장 검토이며 실행 테스트나 CI 결과를 대신하지 않는다.
