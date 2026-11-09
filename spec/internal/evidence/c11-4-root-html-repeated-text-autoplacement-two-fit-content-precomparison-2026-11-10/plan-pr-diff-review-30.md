# C11.4 두-fit 자동 배치 계획 PR · 변경 diff 실패 관점 30개

검토 대상은 신규 계획·20개 계획 검토·Chrome 사전 비교, PR #1311 merge-state 정정, `spec/STATUS.md` 및 공개 roadmap 변경이다. 계획의 20개 관점 검토와 별도로 문서 diff 전체를 확인했다.

1. **PR #1311 상태가 여전히 draft로 남을 수 있다.** 기존 계획과 상태 대장의 문구를 실제 merged state로 바꾸고 merge SHA를 적었다. 통과.
2. **잘못된 merge SHA가 기록될 수 있다.** GitHub 재조회 값 `d64e8c9c1d0464d5e908ef66bfca6241b4aef31d`와 로컬 `origin/main`을 대조했다. 통과.
3. **merge 시각이 로컬 시각과 뒤섞일 수 있다.** `2026-11-09T15:33:12Z` UTC 값을 그대로 사용했다. 통과.
4. **Rebase and merge가 다른 merge 방식으로 축약될 수 있다.** 계획·roadmap 모두 Rebase and merge로 기록했다. 통과.
5. **빈 GitHub check rollup이 CI 통과로 보일 수 있다.** PR #1311 check rollup 0건을 명시하고 CI green 주장을 하지 않는다. 통과.
6. **3,840 RuntimeSession 결과가 3,840 native 장면으로 오해될 수 있다.** 계획과 roadmap에서 RuntimeSession, 대표 Android sentinels, screenshot smoke 범위를 각각 나눴다. 통과.
7. **Android screenshot이 PixelCopy/parity 증거로 과장될 수 있다.** Android 이미지를 visual smoke로 한정하고 PixelCopy/surface parity가 아님을 썼다. 통과.
8. **iOS screenshot이 전 기기 matrix 성공으로 오해될 수 있다.** iOS 화면을 기존 default-route screenshot 한 장의 visual smoke로만 연결했다. 통과.
9. **C11.4 체크가 새 bounded slice merge로 완료될 수 있다.** 모든 새 문서는 C11.4/C11을 미완료로 두고 공식 `78/214`를 유지한다. 통과.
10. **상위 완료 수가 숨은 채 변경될 수 있다.** status와 roadmap에서 78/214를 확인했고 체크박스는 건드리지 않았다. 통과.
11. **계획을 구현 완료로 표시할 수 있다.** 새 plan과 kanban header 모두 계획·사전 비교 완료, 구현 전이라고 구분한다. 통과.
12. **이전 one-fit auto-placement 완료 근거를 두-fit 구현으로 합칠 수 있다.** PR #1302는 one-fit 상태로 분리 링크하고 새 계획을 별도 경계로 설명했다. 통과.
13. **PR #1311의 span-only 변경을 auto-placement 지원으로 오해할 수 있다.** 소스 predicate 차이를 기록하고 span 경계를 유지한다고 명시했다. 통과.
14. **두 개의 fit position count가 잘못될 수 있다.** 1–6의 서로 다른 pair는 15개이며 plan과 fixture/summary가 같은 수를 사용한다. 통과.
15. **2,880 scene과 1,440 authored list가 서로 혼동될 수 있다.** alignment 두 개를 분리해 두 수를 각각 정의했다. 통과.
16. **12 tail order가 잘못된 multiplicity에서 나올 수 있다.** `[min,min,max,auto]` 고유 순열을 verifier가 독립 생성해 12개로 확인했다. 통과.
17. **대표 cap을 전체 허용값으로 오인할 수 있다.** 계획에서 cap pair를 사전 관측 표본으로 한정하고 직접 px/% 허용 경계와 구분했다. 통과.
18. **CSSOM이 cap 순서를 정규화하며 위치를 바꿔도 발견하지 못할 수 있다.** capture/verifier가 authored member와 computed CSSOM 순서를 장면마다 대조한다. 통과.
19. **auto-placement의 item 순서가 document order와 다를 수 있다.** 고정 row sparse flow를 밝히고 8 item frame/line 순서를 독립 검사한다. 통과.
20. **explicit row가 implicit track list 순환에 포함될 수 있다.** 빈 explicit control과 여덟 implicit row를 구분하고 `[1…6,1,2]` cycle을 검증한다. 통과.
21. **`align-content:stretch`가 fit track도 늘릴 수 있다.** 1,440 alignment pair에서 auto occurrence와 non-auto 변화를 분리한다. 통과.
22. **`min-height:0`과 min-content floor control이 모순될 수 있다.** 계획에서 floor 확장을 제외하고 해당 profile의 고정 minimum을 유지하도록 정정했다. 통과.
23. **text line box 높이와 track/frame height가 같은 것으로 혼동될 수 있다.** payload line rect, payload box, item frame, used row를 별도 관계로 기록한다. 통과.
24. **overflow 장면을 clip해서 잃을 수 있다.** 288 visible-overflow scene과 최대 30px를 독립 요약에 보존했다. 통과.
25. **W3C auto-placement 출처가 sizing 문단과 혼합될 수 있다.** track sizing, implicit tracks, placement algorithm의 세 절 링크를 각각 분리했다. 통과.
26. **이미지 경로가 docs route로 해석될 수 있다.** 표본 PNG는 Markdown image로 넣고 원본 JSON/NDJSON/Python은 plain code names로 남겼다. 통과.
27. **브라우저 세션이 남을 수 있다.** session lifecycle은 `closed: true`이며 PID 확인에서 이번 전용 session은 없었다. 통과.
28. **불필요한 simulator가 사전 비교에서 실행됐다고 쓰일 수 있다.** lifecycle은 `simulatorStarted: false`; 세션 기록과 README가 일치한다. 통과.
29. **공유/다른 작업 자원이 변경 파일에 섞일 수 있다.** stage 대상은 이 계획과 증거·기존 stale-doc 수정으로 제한하고 나머지 기존 untracked 파일은 제외한다. 통과.
30. **새 내부 버전 또는 배포를 승인 없이 올릴 수 있다.** internal `0.1.0` 유지, 자동 Pages 배포 금지, 사용자 지정 Tailnet preview만 merge 후 source/hash를 확인해 동기화하도록 명시했다. 통과.

## 결과

별도 30개 diff 관점에서 범위·상태·evidence·정리 경계를 확인했다. 새 기능은 아직 구현하지 않았으며 공식 완료 수와 internal interface version은 변하지 않았다.
