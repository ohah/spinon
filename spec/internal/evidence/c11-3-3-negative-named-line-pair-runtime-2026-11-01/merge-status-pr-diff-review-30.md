# C11.3.3 PR #967 병합 상태·로드맵 문서 diff 실패 관점 검토 30개

검토 대상은 PR #967 병합 뒤의 상태·내부 계약·계획·공개 roadmap 변경이다. 구현 PR의 30개 검토를 재사용하지 않고 병합 metadata, 완료 경계, 문서 간 표현과 공개 route 연결을 확인했다. 아래 관점에서 새 결함은 발견하지 않았다.

1. **PR 상태가 아직 OPEN으로 기록됐는가?** — `gh pr view 967`은 `MERGED`를 반환한다. 공식 상태·내부 계약·계획은 PR #967 병합 완료로 바꿨다.
2. **병합 시각이 현지 시간과 UTC를 혼합했는가?** — GitHub 응답 `2026-11-01T04:44:03Z`를 문서에 UTC 그대로 기록했다.
3. **merge SHA가 PR의 원래 head commit과 혼동됐는가?** — merge commit `eeda2155354236d3e97954f75217228893ceca21`을 별도로 기록하고 구현 원본 `0096e39d`는 runtime source revision으로 유지한다.
4. **PR head/base가 실제 GitHub 상태와 맞는가?** — merge 확인 파일은 병합 직전 head `43493ec578b8ce72bd11b3d6547c641090f40827`, base `915dbf39ecbafab532b87aec0444182ac7830b6b`를 기록한다.
5. **merge 방법이 repository 규칙과 다른가?** — 실행 명령 `gh pr merge 967 --rebase`와 완료를 확인해 `Rebase and merge`로 기록했다.
6. **check이 없는데 CI 성공으로 적었는가?** — `gh pr checks 967 --required` 결과를 그대로 기록했고 CI 성공이라고 주장하지 않는다.
7. **C11 상위 checkbox가 체크됐는가?** — 변경 diff는 C11 checkbox 줄을 수정하지 않았고 status sub-bullet만 갱신했다.
8. **C11.3.3 전체가 완료로 승격됐는가?** — 전체 항목은 미완료 그대로이며 이 제한 slice만 하위 진행 기록으로 추가했다.
9. **공식 수가 구현 PR 병합으로 잘못 증가했는가?** — 공개 roadmap과 관련 문서는 전체 `72/213`을 그대로 명시한다.
10. **내부 contract 숫자 버전이 올라갔는가?** — `0059`의 `0.1.0` 고정 선언을 유지한다.
11. **후보 구현을 일반 CSS 지원으로 표현했는가?** — 지원 경계는 `RuntimeBlockFlexGridTemplateV1` opt-in fixture로 제한한다.
12. **지원하지 않는 C11.3.3 조합이 문서에서 사라졌는가?** — numeric-only, named span, mixed sign, 두 축 implicit, auto-sized item, template areas, 다른 track grammar가 제외 범위에 남아 있다.
13. **Spinon WPT 결과를 확대 해석했는가?** — 상태·계획·공개 카드 모두 Spinon WPT 적합성을 미검증으로 유지한다.
14. **Android simulator를 실기기로 표현했는가?** — AVD API 37 ARM64/SwiftShader 소프트웨어 renderer로 명시했다.
15. **iOS drawable sample을 compositor 이후 결과로 표현했는가?** — iPhone 17 Pro/iOS 26.2 Simulator, compositor 제출 전으로 구분한다.
16. **hardware GPU 또는 실기기 검증을 주장했는가?** — 관련 환경은 미검증으로 적었다.
17. **검증 수치가 기존 evidence와 어긋나는가?** — layout 328/328, CSS reference 18/18, 전용 17 frame 및 표면 8/8, 기존 39 scene·126 node 값이 일치한다.
18. **Android artifact 경계가 APK/release 전체 보장으로 확대됐는가?** — linked ELF 부정/긍정 대조라는 기존 한정을 유지한다.
19. **source evidence가 main에서 접근 가능한가?** — PR #967 병합으로 계획·internal contract·runtime README·review 파일이 main에 포함됐다.
20. **병합 기록 경로와 링크 대소문자가 일치하는가?** — `merge-PR967.md` 동일 경로를 `README`, plan, status 및 internal contract에서 가리킨다.
21. **공식 상태 문구가 다른 날짜의 기록을 덮었는가?** — C11.3.3 하위 bullet 하나에만 병합 정보를 보탰고 과거 matrix 기록은 수정하지 않았다.
22. **내부 계약 서두의 통합 상태가 실제 merge 뒤처졌는가?** — C11.3.3 후속 slice의 PR·시각·SHA 및 check 부재가 현재 상태로 바뀌었다.
23. **구현 계획이 “PR 생성 전”이라고 계속 표시하는가?** — 구현 계획은 PR #967의 merge state와 no-check 결과를 적는다.
24. **C11 전체 계획이 전체 완료를 암시하는가?** — 최신 구현 note는 병합 완료와 별개로 C11.3.3/C11 및 `72/213` 미완료를 명시한다.
25. **공개 roadmap 카드가 기존 최신 항목 뒤에 묻혔는가?** — 새 PR #967 카드는 hero progress 영역의 가장 앞에 삽입했다.
26. **공개 카드가 실제 PR을 연결하는가?** — PR URL·병합 시각·merge SHA가 함께 나타난다.
27. **public roadmap의 로컬 경로가 repository base를 잃었는가?** — 신규 계획, 계약, evidence, review, merge record와 공식 상태 링크는 `/spinon/` 아래를 사용한다.
28. **route HTML 링크가 원본 Markdown 문서에 대응하는가?** — 새 카드의 6개 local links를 source 파일 존재 여부로 따로 확인했다.
29. **문서 변경에서 source code나 runtime behavior가 바뀌었는가?** — diff는 계획·상태·contract·evidence·`spec/public/roadmap.html`뿐이다.
30. **Tailnet preview update가 이미 끝난 것처럼 적혔는가?** — 이 PR diff는 source roadmap만 반영하며, Tailnet 동기화는 merge 후 별도로 실행·Agent Browser로 확인할 단계다.

## 남은 검사

PR #968의 GitHub check·merge 상태와 Tailnet preview HTTP/hash 및 화면 표시는 병합 후 별도 확인한다. 이 검토는 전체 문서 dead-link audit가 아니다.
