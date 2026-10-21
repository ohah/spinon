# PR #429 병합 기록·공개 로드맵 변경 검토 30개

이 검토는 구현 PR #428의 기존 30개 diff 검토와 별개로, 병합 후 추가한 계획·상태 대장·공개 로드맵 문서 변경의 실패 가능성을 확인한다.

1. **C11.4 상위 항목을 완료로 바꾸었는가?** 체크박스는 그대로 미완료다. 통과.
2. **공식 완료 수를 잘못 올렸는가?** 72/213을 유지했다. 통과.
3. **PR #428의 병합 시각을 잘못 썼는가?** GitHub `mergedAt` 값 2026-10-21 06:05:26 UTC와 일치한다. 통과.
4. **merge SHA가 로컬 feature commit과 혼동됐는가?** GitHub merge commit `f930db59448a3adb9fafc840b6a609fe9d2181cf`를 기록했다. 통과.
5. **병합 방식을 잘못 기록했는가?** Rebase and merge 결과로 기록했다. 통과.
6. **빈 check rollup을 CI 통과라고 해석했는가?** rollup이 비어 있다고 분리해 기록한다. 통과.
7. **required status check 부재를 확인 없이 단정했는가?** PR required 조회와 branch-protection API 응답을 근거로 기록한다. 통과.
8. **계획 상단에 이전의 병합 대기 문구가 남았는가?** #428 merge 정보로 교체했다. 통과.
9. **후속 문서 변경이 새 runtime 지원으로 오해될 수 있는가?** 변경은 병합 기록과 공개 roadmap 노트로 제한했다. 통과.
10. **scene 수가 실제 고정 행렬과 다른가?** 네 source-order scene을 명시했다. 통과.
11. **두-image track을 틀리게 적었는가?** `[12,28,20,0]`으로 계획·실행 결과와 맞춘다. 통과.
12. **세-image track을 Chrome 관측으로 바꿨는가?** 독립 기대값 `[18,34,20,0]`을 유지한다. 통과.
13. **Chrome 155 차이를 숨겼는가?** 관측 `[16,32,24,0]`과 원인 미확정을 같이 기록한다. 통과.
14. **Chrome 차이가 Spinon 전체 호환성 결론으로 확대됐는가?** 고정 fixture의 차이로만 서술한다. 통과.
15. **Android model/runtime을 누락했는가?** API 37 ARM64 AVD라고 정확히 제한한다. 통과.
16. **iOS Simulator를 실기기라고 표시했는가?** iPhone 17 Pro/iOS 26.2 Simulator라고 썼다. 통과.
17. **네 scene의 플랫폼별 수치를 합산해 부풀렸는가?** Android 4/4, iOS 4/4로 각각 기록한다. 통과.
18. **`drawPresented`를 실제 compositor 표시 완료로 확대했는가?** host surface present 경로라고 한정했다. 통과.
19. **fixture에 없는 CSS border 검사를 주장하는가?** border stroke는 검증하지 않았다고 명시했다. 통과.
20. **root 밖 overflow나 border 바깥 overflow를 확인했다고 적었는가?** 테두리 밖 overflow는 범위 밖이라고 적었다. 통과.
21. **PNG screenshot을 texel readback으로 해석하는가?** texel/pixel equality는 검증하지 않았다고 적었다. 통과.
22. **OS compositor 이후 픽셀까지 확인했다고 오해할 수 있는가?** compositor 이후 픽셀은 미검증이라고 명시했다. 통과.
23. **WPT 또는 전체 CSS Grid 적합성을 주장하는가?** 주장 범위에 넣지 않았다. 통과.
24. **Android 실기기를 사용했다고 썼는가?** AVD로 구분하고 사용자의 Simulator 선택을 반영했다. 통과.
25. **PR 링크가 이전 구현 PR로 연결되는가?** #428 URL을 plan, status, public roadmap에 사용했다. 통과.
26. **plan route와 repository source 경로가 어긋나는가?** markdown route는 `.html`, repository 링크는 `.md`로 분리했다. 통과.
27. **evidence README route가 근거 폴더와 맞는가?** `internal/evidence/c11-4-anonymous-text-multi-image-2026-10-21/README.html`로 연결한다. 통과.
28. **roadmap screenshot 파일명이 실제 run artifact와 다른가?** Android·iOS 최종 `anonymous-text-three-images-forward.png` 경로를 사용한다. 통과.
29. **HTML `<code>`와 `<a>` 요소가 이스케이프/종료 누락으로 깨질 수 있는가?** 문서 source diff와 이후 browser-rendered DOM에서 링크 및 태그를 확인한다. 통과.
30. **새 note가 이전 C11.4 항목을 덮어쓰거나 집계를 바꾸는가?** PR #425 note 뒤 별도 paragraph로 추가하고 기존 note와 dynamic counter를 보존한다. 통과.

## 결과

추가 수정이 필요한 문서 결함은 발견하지 않았다. 최종 route와 image URL은 Tailnet `/spinon/` 미리보기에서 Agent Browser로 확인한 뒤 결과를 pass40 근거에 기록한다. 공식 완료 수는 72/213, C11.4는 미완료로 유지한다.
