# PR #1157 병합 상태·공개 로드맵 변경 적대 검토 30회

검토 기준은 PR #1156이 main에 Rebase and merge된 `b667a80ea610827ad666e1fe5d3d039676f69b6c`와 이 PR의 상태 대장, 내부 계약, 정적 공개 로드맵 및 검토 기록이다. 제품 런타임 코드는 변경하지 않는다.

1. **C11 상위 항목이 완료로 체크되는가?** 통과 — 상위 체크는 미완료로 유지된다.
2. **C11.1이 완료로 처리되는가?** 통과 — #1156은 제한 구현으로만 기록된다.
3. **공식 완료 집계가 증가하는가?** 통과 — `72/213`을 유지한다.
4. **merge SHA가 GitHub 결과와 불일치하는가?** 통과 — `b667a80ea610827ad666e1fe5d3d039676f69b6c`가 상태 대장·0059·공개 페이지에서 일치한다.
5. **merge timestamp가 다른 시간대나 다른 PR 상태로 적히는가?** 통과 — GitHub의 `2026-11-04T12:52:44Z`를 그대로 기록한다.
6. **현재 PR 상태가 merge 이전의 CLEAN/MERGEABLE로 남는가?** 통과 — 상태 대장과 내부 계약은 Rebase and merge 완료로 갱신됐다.
7. **체크가 없는데 CI 성공으로 주장하는가?** 통과 — 빈 check rollup과 workflow run을 기록하고 성공 주장을 하지 않는다.
8. **main에 required status checks가 있다고 기술하는가?** 통과 — required checks 미설정 사실과 일치한다.
9. **제한 후보가 전체 CSS Grid 또는 WPT 완료로 읽히는가?** 통과 — 제한 profile과 C11/WPT 미완료 경계가 유지된다.
10. **Android AVD가 물리 기기나 hardware GPU 결과로 바뀌는가?** 통과 — software graphics 경로라는 한계가 유지된다.
11. **iOS Metal 표본이 compositor 이후 결과로 보이는가?** 통과 — compositor 제출 전 readback이라고 적는다.
12. **9 scene 전체를 positive로 잘못 합산하는가?** 통과 — 성공 8과 기대 거부 1을 분리한다.
13. **31 item/frame을 scene 수로 혼동하는가?** 통과 — scene과 item/frame 수를 별도 단위로 적는다.
14. **Chrome boundary rejection을 success로 취급하는가?** 통과 — positive 8개와 rejection 1개를 분리한다.
15. **geometry 비교와 surface sample을 혼동하는가?** 통과 — 기존 근거에서 각각 별도 측정으로 기록한다.
16. **sparse column-flow를 dense 또는 row-flow로 잘못 소개하는가?** 통과 — 새 카드와 상태 항목이 sparse column-flow로 한정한다.
17. **지원 item class 네 종류를 누락하거나 다른 순서 계약을 암시하는가?** 통과 — fully definite, row-locked, column-locked, fully-auto를 정확히 열거한다.
18. **Parsed Root HTML 제한을 일반 DOM/CSS admission으로 확대하는가?** 통과 — no-area Root HTML 제한으로 적는다.
19. **공개 카드가 #1152 또는 다른 PR로 연결되는가?** 통과 — 카드 제목·PR 링크·merge SHA가 #1156에 연결된다.
20. **새 병합 카드가 기존 최신 #1152보다 뒤에 놓이는가?** 통과 — #1156 기록을 최신 merged card 위에 삽입한다.
21. **계획·runtime evidence·0059 링크에 없는 경로를 적는가?** 통과 — 세 링크 대상이 저장소에 존재한다.
22. **Tailnet 하위 경로에서 상대 링크 base가 달라져 링크가 깨지는가?** 통과 — 새 카드 링크는 main 기준 절대 GitHub URL이다.
23. **정적 공개 카드의 완료 수가 공식 집계와 달라지는가?** 통과 — 카드에는 `72/213`이 그대로 기록된다.
24. **로드맵 JavaScript의 공식 status parser·checkbox key·task count를 바꾸는가?** 통과 — 이번 diff는 기존 hero progress 카드 한 줄만 추가하고 parser/data를 수정하지 않는다.
25. **요청하지 않은 STATUS.html이나 문서 route를 함께 재생성하는가?** 통과 — 변경 대상은 markdown 상태/계약과 `spec/public/roadmap.html`로 한정된다.
26. **새 HTML card의 nesting 또는 closing tag가 유효하지 않은가?** 통과 — 기존 paragraph 구조와 동일한 닫는 태그를 사용한다.
27. **플랫폼 해상도·scale 숫자가 기존 근거와 모순되는가?** 통과 — 정적 요약은 수치 한계를 확대하지 않고 세부 수치는 status/contract evidence와 일치한다.
28. **상태 대장과 내부 계약에 서로 다른 merge 결과가 적히는가?** 통과 — 두 문서의 timestamp, SHA, no-CI 설명이 일치한다.
29. **병합 카드가 제품 동작 변경이나 공식 완료 변경으로 오인되는가?** 통과 — 제한 slice·미완료·집계 유지가 카드에 명시된다.
30. **이 검토 기록이 Tailnet 배포나 사용자 화면 확인을 완료했다고 과장하는가?** 통과 — 이 PR 검토는 source diff만 다루고 live sync/HTTP/browser 확인은 배포 뒤 별도로 수행한다.

## 판정

30개 관점에서 상태·계약·공개 카드 사이 모순을 찾지 못했다. 이 PR은 문서만 변경하며 C11 완료 수를 바꾸지 않는다. Tailnet 공개 route는 PR 병합 이후 선택한 `roadmap.html`만 동기화하고 served SHA-256 및 실제 화면을 별도 확인한다.

## 2026-11-04 후속 정정

PR #1158 병합 뒤 main을 다시 읽으며 C11 상태 줄의 첫 문장이 PR #1156을 여전히 “열려 있다”고 적고 있음을 발견했다. 같은 줄 뒤쪽과 0059·공개 로드맵에는 merge 상태가 정확히 기록돼 있었지만 서로 모순됐다. 따라서 6번의 “상태 문구를 갱신했다” 판정은 불완전했다. [상태 문구 정정 PR 검토 30개](./status-wording-correction-review-30.md)와 PR #1159가 첫 문장을 병합 사실로 고친다. 구현 코드, 상위 C11 미완료 상태와 공식 `72/213`은 바뀌지 않는다.
