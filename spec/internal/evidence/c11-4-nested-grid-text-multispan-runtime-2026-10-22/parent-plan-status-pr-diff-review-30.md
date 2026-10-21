# C11.4 상위 계획 진행 상태 정정 PR diff 실패 관점 검토 30개

이 검토는 병합된 PR #453, #462, #464의 결과가 `plan/c11-4-grid-track-sizing.md`에 여전히 “구현은 아직 시작하지 않았다”고 기록된 세 항목을 고치는 문서 변경만 대상으로 한다. 구현, 공식 상태 대장, 공개 roadmap, API 및 내부 계약은 변경하지 않는다.

1. **문서 범위를 구현 완료로 확대할 수 있음** — 변경 대상은 상위 계획의 세 하위 항목 진행 요약으로 한정했다.
2. **C11.4 전체를 완료로 읽을 수 있음** — 각 항목에서 상위 C11.4 checkbox와 공식 `72/213`을 유지한다고 명시했다.
3. **공식 상태 대장과 상위 계획이 충돌할 수 있음** — `spec/STATUS.md`가 이미 기록한 병합 사실 및 미완료 경계와 대조했다.
4. **공개 API나 버전 계약 변경을 암시할 수 있음** — API와 내부 계약을 변경하지 않았고 상위 계획에도 새 지원 약속을 추가하지 않았다.
5. **문서 수정만으로 런타임 재검증을 주장할 수 있음** — 세 PR의 기존 실행 근거를 링크하고 이번 변경에서 시뮬레이터를 다시 실행했다고 쓰지 않았다.
6. **#453의 PR 상태를 잘못 쓸 수 있음** — GitHub PR 조회에서 `MERGED`임을 확인했다.
7. **#453의 주제를 다른 intrinsic sizing slice와 혼동할 수 있음** — 두 단계 nested Grid 양 축 sizing 계획 및 고정 fixture 결과에 한정했다.
8. **#453의 병합 방식을 잘못 쓸 수 있음** — `Rebase and merge`를 기록했다.
9. **#453의 시간을 UTC와 혼동할 수 있음** — GitHub `2026-10-21T15:46:18Z`를 `2026-10-22 00:46:18 KST`로 변환했다.
10. **#453의 PR commit을 merge SHA로 적을 수 있음** — GitHub merge API가 반환한 `619a99209ec38d838c90428b8d153cf24a1aa0d3`를 사용했다.
11. **#453의 미보고 check를 성공으로 표현할 수 있음** — check rollup 미보고와 main required checks 미설정을 각각 구분했다.
12. **#453 AVD를 실기기로 부를 수 있음** — Android API 37 ARM64 AVD라고 정확히 기록했다.
13. **#453 iOS 실행 환경을 축약해 오해시킬 수 있음** — iPhone 17 Pro / iOS 26.2 Simulator라고 적었다.
14. **#453 플랫폼별 scene 수를 합칠 수 있음** — Android와 iOS 각각 4/4 실행으로 표현했다.
15. **#453의 독립 min-content 값을 잘못 적을 수 있음** — `70×20px`, Columns `[70px]`, Rows `[20px]`를 원 계획과 대조했다.
16. **#453의 독립 max-content 값을 잘못 적을 수 있음** — `100×10px`, Columns `[100px]`, Rows `[10px]`를 원 계획과 대조했다.
17. **#462의 PR 상태를 잘못 쓸 수 있음** — GitHub PR 조회에서 `MERGED`임을 확인했다.
18. **#462의 시간을 잘못 변환할 수 있음** — GitHub `2026-10-21T17:49:32Z`를 `2026-10-22 02:49:32 KST`로 변환했다.
19. **#462의 merge SHA 대신 head SHA를 쓸 수 있음** — GitHub merge API 값 `c1b7f0c1451da2274d58903590bfe44f10f50213`를 사용했다.
20. **#462의 신규 scene 수를 회귀 control 수와 혼동할 수 있음** — 새 네 scene과 기존 열 scene을 구분했다.
21. **#462의 합계를 플랫폼 간에 잘못 셀 수 있음** — Android AVD와 iOS Simulator 각각 14/14로 썼다.
22. **#462의 기대값을 Chrome 관측과 혼동할 수 있음** — CSS Grid 2025 CRD 독립 기대값인 min-content `94×20px`, max-content `124×16px`로 표현했다.
23. **#462 실행을 이미지 픽셀 동등성으로 확대할 수 있음** — glyph/PNG pixel equality는 확인하지 않았다고 남겼다.
24. **#462 Simulator 실행을 실기기·WPT 결과로 읽을 수 있음** — Android 실기기, iOS compositor 이후 pixels, WPT를 미검증으로 적었다.
25. **#462 고정 fixture를 일반 Grid 지원으로 확대할 수 있음** — 일반 CSS Grid 지원은 검증하지 않았다고 명시했다.
26. **#464의 PR 상태와 merge 방식을 잘못 쓸 수 있음** — 병합된 PR의 `Rebase and merge` 상태를 확인해 기록했다.
27. **#464의 병합 시간을 잘못 쓸 수 있음** — GitHub `2026-10-21T18:45:55Z`를 `2026-10-22 03:45:55 KST`로 변환했다.
28. **#464의 SHA를 PR head와 혼동할 수 있음** — merge SHA `3c03b51979127a0c52f5737cdd2d003b9dcf39c3`를 사용했다.
29. **#464의 span-2와 span-3 결과를 섞을 수 있음** — `[25,35,35,0]`과 `[160/3,190/3,190/3,0]`, ancestor 폭 95px/180px를 각각 구분했다.
30. **#464의 회귀 및 남은 범위를 누락할 수 있음** — 신규/control 4/4를 분리하고 임의 nested Grid, 일반 CSS admission, WPT, 실기기 및 iOS compositor 이후 pixels를 미검증으로 유지했다.

검토 근거: 세 집중 계획과 실행 README, `spec/STATUS.md`, GitHub PR #453/#462/#464의 병합 상태·시각·merge SHA. 이번 PR에서는 앱 테스트·빌드와 Simulator를 재실행하지 않는다.
