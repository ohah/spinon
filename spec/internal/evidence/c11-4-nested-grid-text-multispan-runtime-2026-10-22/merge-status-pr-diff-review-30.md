# PR diff 실패 관점 검토 30개 — PR #464 병합 상태 동기화

이 문서는 PR #464 병합 뒤 `plan/c11-4-nested-grid-multispan.md`와 `spec/STATUS.md`에 남은 상태 문구를 갱신한 변경을 검토한다. 구현 코드는 수정하지 않는다.

1. **다른 PR의 병합 SHA를 섞을 수 있음** — PR #464의 merge API 결과와 `origin/main`이 모두 `3c03b51979127a0c52f5737cdd2d003b9dcf39c3`을 가리키는지 대조했다.
2. **PR이 닫힌 상태를 병합 완료로 오인할 수 있음** — `state=MERGED`와 `mergedAt=2026-10-21T18:45:55Z`를 확인했다.
3. **UTC 시각을 KST로 잘못 변환할 수 있음** — 9시간을 더해 `2026-10-22 03:45:55 KST`로 기록했다.
4. **Rebase merge를 squash로 잘못 기록할 수 있음** — GitHub 병합 방법이 Rebase and merge였음을 확인했다.
5. **원래 PR head SHA를 merge SHA로 적을 수 있음** — `abbf64f4…` head와 `3c03b519…` merge commit을 구분했다.
6. **로컬의 오래된 main만으로 최신 병합을 판정할 수 있음** — `origin/main`을 fetch한 뒤 결과 SHA를 대조했다.
7. **required check가 없다는 사실을 놓칠 수 있음** — main branch protection의 `required_status_checks`가 `null`임을 확인했다.
8. **체크가 미보고된 상태를 통과로 표현할 수 있음** — `gh pr checks 464`의 “no checks reported” 결과를 그대로 미보고라고 적었다.
9. **required check 없음과 모든 CI 통과를 혼동할 수 있음** — 어떤 check도 green이라고 쓰지 않았다.
10. **승인 요구 정책을 근거 없이 과장할 수 있음** — required approving review count가 0인 설정을 확인했고, 리뷰 승인 완료라고 쓰지 않았다.
11. **작업 계획에 열린 PR 상태가 남을 수 있음** — 계획 첫 상태 문장에서 “열려 있음”과 병합 대기 문구를 제거했다.
12. **공식 상태 대장에 열린 PR 상태가 남을 수 있음** — C11.4 기록을 병합 완료 상태로 바꿨다.
13. **병합 기록 중 PR 연결을 잃을 수 있음** — 두 문서 모두 PR #464 링크를 유지했다.
14. **두 문서의 merge SHA가 어긋날 수 있음** — 같은 전체 40자리 SHA를 양쪽에 기록했다.
15. **두 문서의 병합 일시가 어긋날 수 있음** — 같은 KST 시각을 양쪽에 기록했다.
16. **제한 fixture를 C11.4 전체 완료로 승격할 수 있음** — 공식 완료 수 `72/213`을 유지했다.
17. **C11.4 체크박스를 실수로 완료 처리할 수 있음** — 상태 대장의 상위 C11.4 미완료 체크는 변경하지 않았다.
18. **고정 scene 결과를 임의 중첩 Grid 지원으로 확대할 수 있음** — 기존 임의 입력 미검증 한계를 그대로 뒀다.
19. **Simulator 결과를 실기기 결과로 바꿔 쓸 수 있음** — Android API 37 ARM64 AVD와 iOS 26.2 Simulator 구분을 보존했다.
20. **화면 렌더링 근거의 범위를 확대할 수 있음** — iOS compositor 이후 pixels와 glyph equality 미검증 범위를 유지했다.
21. **PR 병합을 공개 CSS API 지원 선언으로 오해할 수 있음** — 공개 API 무변경 및 일반 CSS admission 미확인 범위를 유지했다.
22. **내부 계약 버전 상승이 병합으로 암시될 수 있음** — 숫자 버전 변경을 기록하지 않았고 기존 계약 링크를 보존했다.
23. **레이아웃 알고리즘 변경을 문서가 주장할 수 있음** — fixed-scene 검증 사실만 남기고 sizing algorithm 변경 주장은 추가하지 않았다.
24. **Chrome 사전 비교 또는 독립 계산 근거 링크가 끊길 수 있음** — 기존 상대 링크와 경로를 수정하지 않았다.
25. **scene 기대값이나 폭 결과를 병합 상태 편집 중 바꿀 수 있음** — Columns와 95/180px 결과 문구는 그대로 유지했다.
26. **같은 날짜의 다른 C11.4 기록을 덮어쓸 수 있음** — STATUS에서 PR #464에 해당하는 한 항목만 바뀌었다.
27. **공식 상태 목록의 체크 계층을 깨뜨릴 수 있음** — 기존 C11.4 하위 항목의 들여쓰기와 checkbox 구조를 유지했다.
28. **상대 Markdown 링크가 현재 저장소 위치에서 어긋날 수 있음** — 문서 경로를 기준으로 기존 plan/evidence 상대 링크를 그대로 보존했다.
29. **미래의 병합 일시를 과거 기록으로 추가할 수 있음** — 기록된 시각이 저장소의 2026-10-22 작업 날짜와 일치함을 확인했다.
30. **문서 변경이 Pages 배포나 Tailnet 갱신 완료를 허위로 주장할 수 있음** — 이 PR은 상태 문구만 기록하며 배포·미리보기 동기화 완료를 주장하지 않는다.

검토 기준 자료: `gh pr view 464`, `gh pr checks 464`, main branch protection 응답, fetch된 `origin/main` SHA. 로컬 문서 diff는 `git diff --check`로 확인한다.
