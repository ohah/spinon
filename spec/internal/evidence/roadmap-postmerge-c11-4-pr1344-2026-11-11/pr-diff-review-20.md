# PR #1345 post-merge roadmap diff 적대적 검토 20회

실제 원격 PR [#1345](https://github.com/ohah/spinon/pull/1345)의 초기 diff를 대상으로 20개의 독립 실패 조건을 확인했다. 검토 시 base는 `c5d3f7708e64dd503a8b2e16cb539ff0d3874ec8`, head는 `65129d8adb382ee5be4563e9065d80495ff38bef`였고 GitHub 원격 파일 4개와 로컬 base/head diff 4개가 정확히 일치했다. 각 항목은 잘못된 상태·숫자·지원 범위가 공개 상태 문서로 전파될 수 있는지 공격적으로 살폈다.

1. **잘못된 구현 PR을 merge 완료로 기록하는가?** GitHub PR #1344의 authoritative state는 merged/closed이고 merge SHA `c5d3f7708e64dd503a8b2e16cb539ff0d3874ec8`다. STATUS, plan, contract에서 동일 SHA를 쓴다.
2. **UTC timestamp를 merge date나 로컬 timezone과 혼동하는가?** 세 문서는 GitHub의 `2026-11-10T22:06:42Z` merged-at timestamp를 그대로 표기한다.
3. **PR head SHA를 merge SHA로 오기하는가?** 문서에 기록된 SHA는 PR metadata의 merge commit SHA이며 feature head `7696...`가 아니다.
4. **이전 계획 PR #1343의 merge 상태가 덮어써지는가?** Plan은 #1343 merge SHA를 보존하고 #1344 merge 사실을 별도로 기록한다.
5. **이미 종료된 slice가 공식 작업 진행 중으로 남는가?** Public roadmap에서 four-fit 완료 항목을 `detailInProgress`에서 제거한다.
6. **상위 C11.4/C11을 완료로 승격하는가?** STATUS, plan, contract와 public roadmap 모두 해당 체크를 미완료로 유지한다.
7. **공식 완료 분모·분자가 달라지는가?** 공식 집계는 계속 `78/214`다.
8. **내부 숫자 계약 버전이 불필요하게 올라가는가?** Contract는 `0.1.0`을 유지하며 문서 후속 PR은 numeric interface를 바꾸지 않는다.
9. **개발자용 구현 범위를 일반 CSS Grid 지원으로 넓혀 읽게 하는가?** Contract의 정확한 four-fit 제한 및 지원 밖 경계는 유지된다.
10. **CI가 비어 있는데 green으로 서술하는가?** #1344 PR check가 0개였음을 기록하고 CI 성공 주장을 하지 않는다.
11. **네 screenshot의 시각 smoke를 pixel parity로 과장하는가?** Simulator/AVD 표본과 Android software renderer 한계를 유지하고 readback·hardware·실기기 주장을 하지 않는다.
12. **새 five-fit 후보가 이미 구현·검증된 것으로 오해되는가?** Roadmap은 Chrome precomparison/계획 단계라고 명시하고 구현 미시작을 적는다.
13. **Next 후보를 진행 중으로 잘못 표시하는가?** Five-fit은 `detailNext`에만 두고 `detailInProgress`는 비운다.
14. **다음 후보의 scope가 완료 보장이나 전체 CSS Grid처럼 표현되는가?** Description은 한 bounded candidate의 precomparison과 plan만 가리킨다.
15. **공식 STATUS와 계획 문서가 서로 다른 merge state를 보이는가?** 두 파일 모두 #1344 merged SHA와 같은 미완료 숫자를 기록한다.
16. **Internal contract 제목은 merged인데 끝 문단은 open을 남기는가?** Heading과 closing paragraph 모두 #1344 Rebase and merge 상태로 맞춘다.
17. **Contract 설명이 기존 one/two/three-fit 경계를 지우는가?** 변경은 four-fit 단락의 lifecycle metadata만 수정하고 앞선 profiles의 scope를 덮어쓰지 않는다.
18. **새 문서 링크가 경로 기준을 잃는가?** Changed docs가 가리키는 plan, contract 및 implementation evidence 경로는 repository tree에서 확인 가능하다.
19. **문서 PR에 제품 동작 변경이 끼어드는가?** 초기 4-file remote diff는 STATUS, plan, contract, roadmap만 변경한다.
20. **Tailnet 배포를 수행하지 않았는데 공개 preview가 동기화됐다고 말하는가?** PR은 repository roadmap 소스만 갱신하고 실제 Tailnet publication/HTTP verification은 별도 후속 작업으로 남긴다.

**검토 결과:** 20개 adversarial pass 모두 통과했고 이 diff에서 상태·범위 오류를 발견하지 못했다. 이는 문서 상태 일관성만 검증하며 implementation behavior, Tailnet deployment 또는 C11.4 전체를 증명하지 않는다.
