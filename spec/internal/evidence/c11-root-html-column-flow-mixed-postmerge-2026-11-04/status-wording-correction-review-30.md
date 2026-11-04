# PR #1159 C11 병합 상태 문구 정정 검토 30회

검토 대상은 C11.1 candidate PR #1156 merge 이후 `spec/STATUS.md` 첫 기록 문구, 후속 정정 설명과 이 PR 검토 기록이다. 변경은 상태 문서만 다루며 런타임 동작을 바꾸지 않는다.

1. **C11.1 PR이 여전히 open으로 표시되는가?** 통과 — stale `후보 PR #1156이 열려 있다` 문구를 제거한다.
2. **표시한 PR 번호가 다른 C11 slice를 가리키는가?** 통과 — 구현 merge 대상은 #1156이다.
3. **merge timestamp가 GitHub 결과와 다른가?** 통과 — `2026-11-04T12:52:44Z`를 쓴다.
4. **merge SHA가 다른 commit으로 기록되는가?** 통과 — `b667a80ea610827ad666e1fe5d3d039676f69b6c`를 쓴다.
5. **same-line duplicate merge phrase 때문에 앞 문장이 모호한가?** 통과 — 뒤쪽 중복 merge 문장을 제거해 merge 사실을 한 번 기록한다.
6. **C11 parent checkbox가 완료로 바뀌는가?** 통과 — `[ ]` 상태를 유지한다.
7. **C11.1을 전체 완료로 체크하는가?** 통과 — bounded implementation PR merge 상태만 기록한다.
8. **공식 count가 증가하는가?** 통과 — `72/213` 유지 문장을 보존한다.
9. **#1156 rejection scene이 success로 세어지는가?** 통과 — 검증 문장은 success 8과 expected rejection 1을 분리한다.
10. **31 item frame이 scene 개수로 표현되는가?** 통과 — scene과 item/frame 개수 표기가 유지된다.
11. **Chrome 156 비교 범위가 달라지는가?** 통과 — 8 positive scene과 1 explicit-boundary rejection 결과를 그대로 둔다.
12. **Android hardware GPU나 실기기 성능으로 확대되는가?** 통과 — API 37 ARM64 AVD 및 ADB density 근거와 software 제한을 유지한다.
13. **iOS 표본이 compositor 이후 출력처럼 바뀌는가?** 통과 — compositor 이전 Metal drawable 조건을 유지한다.
14. **CI 성공을 잘못 주장하는가?** 통과 — 빈 status/workflow run과 main required-check 부재를 명시한다.
15. **구현 review 20과 PR diff review 30의 구분이 사라지는가?** 통과 — 별도 링크와 횟수를 유지한다.
16. **Tailnet sync evidence가 문서에서 끊기는가?** 통과 — merged roadmap sync README 링크를 보존한다.
17. **새 correction review 링크가 잘못된 상대 경로인가?** 통과 — 파일은 같은 postmerge evidence directory의 명시된 경로다.
18. **이전 review pass 6의 불일치가 감춰지는가?** 통과 — 이전 기록에 발견과 수정 사유를 덧붙인다.
19. **기존 review file 전체가 삭제되거나 rewrite되는가?** 통과 — 원문 review는 유지하고 후속 정정만 append한다.
20. **상태 줄에 PR #1157 docs merge와 #1158 evidence sync를 섞는가?** 통과 — #1156 product merge 상태만 고치고 후속 PR 링크는 증거 링크로 남긴다.
21. **공개 roadmap과 0059의 이미 맞는 merge 상태를 오염시키는가?** 통과 — 두 파일은 변경하지 않는다.
22. **C11.2.2/C11.2.3 등 기존 하위 체크가 바뀌는가?** 통과 — 다른 checkbox를 수정하지 않는다.
23. **C11.3/C11.4 남은 범위를 완료로 바꾸는가?** 통과 — 해당 체크 상태와 설명은 그대로 둔다.
24. **support profile이 broad CSS Grid로 요약되는가?** 통과 — existing bounded sparse column-flow wording을 유지한다.
25. **fixture evidence link가 다른 date/run을 가리키는가?** 통과 — existing precomparison/runtime paths를 건드리지 않는다.
26. **한국어 상태 표현에서 open/merged 의미가 헷갈리는가?** 통과 — 최초 소개 문구 자체를 `Rebase and merge됐다`로 바꾼다.
27. **현재 line의 duplicate timestamp/SHA가 남는가?** 통과 — SHA/timestamp는 한 번만 나타난다.
28. **문서 correction이 Tailnet public page resync를 요구하는가?** 통과 — 공개 `spec/public/roadmap.html`은 변경하지 않는다.
29. **branch/PR review가 runtime verification을 새로 실행했다고 주장하는가?** 통과 — 이미 merged simulator evidence만 참조한다.
30. **후속 correction이 completion progress를 과장하는가?** 통과 — 72/213 및 C11 미완료를 명시적으로 보존한다.

## 판정

30개 관점에서 정정 범위는 status wording에 한정된다. 앞선 review의 pass 6은 후속 addendum으로 바로잡았으며, merge·runtime facts는 PR #1156의 GitHub result와 0059·public roadmap에 대조했다.
