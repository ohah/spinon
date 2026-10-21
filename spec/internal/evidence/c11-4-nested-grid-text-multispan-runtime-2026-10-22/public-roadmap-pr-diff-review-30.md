# PR diff 실패 관점 검토 30개 — C11.4 공개 roadmap 반영

이 검토는 병합된 PR #464의 제한된 nested Grid text multi-span 결과를 `spec/public/roadmap.html`에 요약한 변경을 대상으로 한다. 구현 코드는 수정하지 않는다.

1. **새 항목을 이전 C11.4 결과보다 앞에 둘 수 있음** — PR #462의 항목 다음, overview section 앞에 추가했다.
2. **병합되지 않은 PR을 완료처럼 보일 수 있음** — PR #464가 `MERGED`인 사실과 병합 시간을 다시 확인했다.
3. **UTC와 KST를 혼동할 수 있음** — GitHub UTC `2026-10-21T18:45:55Z`를 `2026-10-22 03:45:55 KST`로 변환했다.
4. **병합 방법을 잘못 쓸 수 있음** — `Rebase and merge`를 기록했다.
5. **PR head와 merge commit을 혼동할 수 있음** — merge SHA `3c03b51979127a0c52f5737cdd2d003b9dcf39c3`를 사용했다.
6. **check 미보고를 CI 성공으로 바꿔 쓸 수 있음** — rollup이 보고되지 않았다고 썼고 green 주장을 하지 않았다.
7. **required check 정책을 잘못 표현할 수 있음** — main branch protection의 required status checks 미설정을 확인했다.
8. **고정 fixture 하나로 완료 수를 올릴 수 있음** — 공식 `72/213`을 유지했다.
9. **C11.4 전체를 완료 처리할 수 있음** — 상위 체크가 미완료라는 경계를 명시했다.
10. **Grid 깊이와 계층을 축약해 오해시킬 수 있음** — root → middle → branch → inner 네 Grid와 세 parent 경계를 적었다.
11. **네 scene을 다른 개수로 셀 수 있음** — 두 span-2 order와 두 span-3 order의 네 scene으로 기록했다.
12. **span-2 control 값을 span-3 결과로 섞을 수 있음** — span-2 쌍의 독립 Columns `[25,35,35,0]`을 별도로 썼다.
13. **span-3 값을 정수로 반올림해 기대값을 바꿀 수 있음** — `[160/3,190/3,190/3,0]` 분수값을 유지했다.
14. **nested Grid 폭을 잘못 연결할 수 있음** — pair/3-item scene의 폭 95px/180px를 각각 명시했다.
15. **내부 Columns와 바깥 폭을 혼동할 수 있음** — 두 결과를 서로 다른 측정값으로 표현했다.
16. **scene 실행을 CSS cascade 지원으로 넓힐 수 있음** — 일반 CSS admission 미검증을 유지했다.
17. **Android AVD를 실기기로 부를 수 있음** — API 37 ARM64 AVD라고 썼다.
18. **iOS simulator 모델/버전을 누락할 수 있음** — iPhone 17 Pro / iOS 26.2 Simulator를 적었다.
19. **신규 scene과 control 결과를 합쳐 오해시킬 수 있음** — 신규 네 scene과 direct-leaf control의 각 4/4를 구분했다.
20. **플랫폼별 결과를 한 플랫폼 결과로 합칠 수 있음** — Android와 iOS 각각의 4/4 결과를 모두 기록했다.
21. **trace만 검증하고 frame/draw를 생략할 수 있음** — track·frame·source trace·draw/present 검증 범위를 적었다.
22. **검증하지 않은 pixel parity를 주장할 수 있음** — glyph나 플랫폼 간 pixel equality는 언급하지 않았다.
23. **AVD 결과를 hardware GPU 증거로 확대할 수 있음** — Android 실기기 검증이 미포함임을 명시했다.
24. **Simulator 실행을 WPT 적합성으로 확대할 수 있음** — WPT 미검증을 명시했다.
25. **중첩 fixture를 임의 Grid/subgrid 지원으로 읽을 수 있음** — arbitrary nested Grid 미검증 범위를 적었다.
26. **계획 링크의 public route가 빠질 수 있음** — rendered `plan/c11-4-nested-grid-multispan.html` 경로를 사용했다.
27. **독립값과 Chrome 관측 출처가 사라질 수 있음** — C11.4 precomparison README 링크를 추가했다.
28. **시뮬레이터·artifact 한계 출처를 잃을 수 있음** — runtime evidence README 링크를 추가했다.
29. **화면 링크가 잘못된 scene을 가리킬 수 있음** — Android와 iOS의 `nested-text-span2-before-span3.png` 파일 경로를 원본 evidence tree와 대조했다.
30. **공식 상태 링크가 잘못된 anchor로 갈 수 있음** — 기존 `/spinon/STATUS.html#c11-css-grid` 경로를 사용했다.

검토 자료: PR #464 merge API 및 check 결과, main branch protection 응답, runtime README/scene manifest, Android·iOS screenshot 파일 목록. HTML 변경 후 선택 Rspress build와 Agent Browser route 확인을 수행한다.
