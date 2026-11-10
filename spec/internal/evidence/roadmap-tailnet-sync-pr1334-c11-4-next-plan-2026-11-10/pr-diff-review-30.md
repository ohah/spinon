# PR diff 적대 검토 30개 관점

**대상:** #1334 병합 뒤 다음 C11.4 계획 상태와 공개 로드맵의 다음 작업 표시를 맞추는 문서 변경.

1. 공식 완료 수를 증가시키지 않는가 — PASS: `78/214` 유지.
2. C11 parent checkbox를 완료 처리하지 않는가 — PASS: unchecked 유지.
3. C11.4 checkbox를 완료 처리하지 않는가 — PASS: unchecked 유지.
4. internal contract 숫자 버전을 올리지 않는가 — PASS: `0.1.0` 유지.
5. #1334를 계획 PR로 설명하는가 — PASS: plan-only 병합으로 표기.
6. 제품 구현을 완료로 암시하지 않는가 — PASS: 구현 미착수 명시.
7. 병합 시각이 실제 PR과 맞는가 — PASS: GitHub의 `2026-11-10T11:45:04Z` 사용.
8. merge SHA가 실제 merge commit인가 — PASS: `50678c278dc235bfb09ad2bf24e4ef174a46358a` 사용.
9. 계획 링크가 정확한 RSPress route인가 — PASS: canonical plan filename 사용.
10. precomparison 링크가 정확한 경로인가 — PASS: `README.md` route 사용.
11. 계획과 Chrome oracle을 혼동하지 않는가 — PASS: 제품 RuntimeSession 결과가 아니라고 설명.
12. 11,520 scene 수가 계획 근거와 맞는가 — PASS: 일치.
13. 5,760 paired key 수가 계획 근거와 맞는가 — PASS: 일치.
14. eligible occurrence 수가 계획 근거와 맞는가 — PASS: 24,576 일치.
15. content-floor 결과를 오해하게 쓰지 않는가 — PASS: Chrome precomparison 범위로 한정.
16. source-order 결과를 제품 완료로 넓히지 않는가 — PASS: 계획 비교 결과로 한정.
17. auto-only stretch 결과를 일반 Grid 보장으로 넓히지 않는가 — PASS: 제한 수치만 언급.
18. 최신 구현 PR #1331 기록을 지우지 않는가 — PASS: 병합된 구현 기록 보존.
19. 계획 PR을 최신 구현 slice로 잘못 부르지 않는가 — PASS: `계획`과 `다음 작업`으로 분리.
20. 계획을 `진행 중` 칸에 잘못 표시하지 않는가 — PASS: `할 일`/NEXT에 둠.
21. 구현 미착수인데 WIP가 있는 것으로 보이지 않는가 — PASS: WIP lane 비움 유지.
22. 계획을 NEXT로 한 번만 추가하는가 — PASS: bounded detailNext 한 건.
23. NEXT 카드가 공식 task 집계 ID를 새로 등록하지 않는가 — PASS: detail-only card.
24. NEXT 추가가 공식 완료 개수 계산에 영향을 주지 않는가 — PASS: 공식 상태 파서와 checkbox 수 그대로.
25. UI 동적 상태 fetch URL을 바꾸지 않는가 — PASS: `STATUS.md?ref=main` 유지.
26. 모바일에서 넓은 칸반이 문서 전체의 가로 overflow를 만들지 않는가 — PASS: Agent Browser 390px에서 document/body 폭 모두 390px이며 칸반 내부 스크롤만 발생.
27. 데스크톱에서 칸반 heading이 중복되지 않고 다음 작업 카드가 잘리지 않는가 — PASS: Agent Browser 1440×1000에서 duplicate heading 1개를 제거한 뒤 제목 하나와 NEXT 카드를 확인.
28. HTML 앵커와 텍스트 escaping이 유효한가 — PASS: 직접 HTML 링크는 저장소의 기존 패턴을 따름.
29. CI·시뮬레이터·실기기 근거를 새로 주장하지 않는가 — PASS: 문서 작업만 설명.
30. 이 PR diff가 활성 Tailnet server를 불필요하게 변경·재시작하는가 — PASS: PR에는 문서 원본만 포함하고 제공 갱신은 병합 후 별도 route-copy로 검증한다.

## 검토 후 반영

- 계획 문서의 `계획 PR 준비`는 이미 병합된 PR 상태와 어긋나 `계획 완료 · #1334 병합`으로 수정했다.
- 공식 상태 대장에는 #1334 병합과 구현 미착수 기록을 추가하고 완료 집계는 보존한다.
- 첫 desktop review에서 #1326과 #1331 칸반 heading이 중복됨을 발견해 낡은 #1326 heading을 제거했다. 재검토에서 heading 1개를 확인했다.
- 모바일·데스크톱 local Agent Browser 화면은 통과했다. Tailnet 제공 hash와 live screenshot은 병합 후 전달 검증으로 추가한다.
