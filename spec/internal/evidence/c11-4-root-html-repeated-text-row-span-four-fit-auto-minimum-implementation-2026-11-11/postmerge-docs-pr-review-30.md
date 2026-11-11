# PR #1357 병합 후 문서 동기화 · PR diff 실패 관점 30개

검토 대상은 merged implementation PR #1357의 상태를 계획·상태 대장·내부 계약·공개 로드맵·실행 evidence에 반영하는 문서 변경이다. 각 항목은 별도의 실패 경로를 기준으로 checked; 구현/runtime 20개 및 implementation PR diff 30개 리뷰를 재사용하지 않았다.

1. **PR 상태가 실제 병합 상태와 다른가?** GitHub PR #1357을 다시 조회해 `MERGED`인지 대조했다. `MERGED`, 통과.
2. **병합 시각의 시간대나 값이 잘못됐는가?** GitHub의 `2026-11-11T11:46:18Z`를 계획·계약·상태 기록과 대조했다. 세 문서가 같은 UTC 시각을 사용, 통과.
3. **최종 PR head를 merge SHA로 오인했는가?** PR head `f5a85607fb1a3678a5a30b55ad032a34d08b9585`와 merge SHA `30bc0acceeb1cdc90b55d3ee29bc938e05c61f83`를 구분해 기록했다. 통과.
4. **로컬 `main`이 병합본을 가리키지 않는가?** 동기화 브랜치 시작점 `main`/`origin/main`이 `30bc0acceeb1cdc90b55d3ee29bc938e05c61f83`인지 확인했다. 통과.
5. **PR 원격 diff와 실제 최종 diff가 달랐는가?** 병합 직전 원격 `gh pr diff 1357`와 로컬 `origin/main...HEAD`의 byte 동일, SHA-256, 2,697 lines/42 files 기록을 merge evidence에서 대조했다. 통과.
6. **check 부재를 CI 성공으로 과장했는가?** GitHub rollup이 빈 배열임을 조회했다. 세 문서 모두 check가 보고되지 않았다고 기록, 통과.
7. **PR 유형·영역 라벨이 누락됐는가?** GitHub가 반환한 `enhancement`, `area:testing`, `area:layout`과 merge evidence를 대조했다. 통과.
8. **공식 완료 수가 제한 slice만으로 올라갔는가?** `spec/STATUS.md`, 내부 계약, 계획, 로드맵 및 Tailnet 표시가 모두 `78/214`다. 통과.
9. **상위 C11.4/C11 상태가 완료로 체크됐는가?** 상태 대장 checkbox와 변경 diff를 확인했다. 이번 변경은 checkbox를 바꾸지 않음, 통과.
10. **내부 숫자 계약이 불필요하게 올랐는가?** `0.1.0` 유지 문구를 각 관련 문서에서 대조했다. 통과.
11. **공개 API 또는 FFI 추가를 암시하는가?** 계약·상태 문구는 public API/FFI가 없다고 명시한다. 통과.
12. **상태 대장의 설명이 병합 전 단계에 머무는가?** PR #1357 open 표현을 제거하고 merge 시각·SHA·check 한계로 교체했다. 통과.
13. **계획 상단 상태가 PR을 아직 열려 있다고 표시하는가?** 계획 첫 상태 줄에서 merged 사실과 남은 상위 범위를 확인했다. 통과.
14. **내부 계약 section 제목이 candidate로 남았는가?** section heading을 bounded implementation과 PR merge 식별자로 고쳤다. 통과.
15. **지원 profile의 member multiplicity가 넓어졌는가?** 정확히 `fit-content() ×4`, `min-content ×1`, `auto ×1`인지 계약과 계획에서 확인했다. 통과.
16. **cap 값·위치 조합의 범위가 다르게 기록됐는가?** `0px`, `10px`, `20%`, `30px`, 15 quadruple, 24 cap order 및 두 tail order를 대조했다. 통과.
17. **percentage cap basis가 indefinite로 기술됐는가?** definite 220px block basis 문구와 plan을 대조했다. 통과.
18. **baseline/control과 candidate minimum이 바뀌었는가?** explicit empty control은 `min-height:0`, span items는 전부 0 또는 전부 auto만 admission한다. 통과.
19. **일반 CSS Grid 지원으로 확대 해석되는가?** owner, direct-text grammar, sparse row flow, visible overflow, 고정 topology와 alignment 제한이 남아 있음을 확인했다. 통과.
20. **rejection 또는 회복 계약이 사라졌는가?** out-of-profile atomic rejection 및 같은 RuntimeSession 다음 revision recovery가 계약·실행 근거에 유지된다. 통과.
21. **1/64px 보정이 일반 Grid 반올림 계약으로 승격됐는가?** rounding 경계가 이 고정 profile에 한정된다는 문구를 확인했다. 통과.
22. **RuntimeSession scene 수가 잘못 기록됐는가?** row-span matrix `11,520/11,520` 및 paired baseline/candidate 구조를 로그·verification JSON과 대조했다. 통과.
23. **auto-placement 회귀 수치가 새 row-span 결과와 섞였는가?** 기존 four-fit auto-placement `5,760/5,760`을 독립 regression으로 표기했다. 통과.
24. **focused rejection, trace 또는 projection/layout 결과를 누락했는가?** logs와 verification JSON의 projection, layout, trace, rejection counts를 확인하고 연결했다. 통과.
25. **format/lint 또는 Android/iOS 빌드를 전체 검증으로 과장했는가?** 실행 근거는 실제 명령과 로그를 연결하며 PR check 부재와 별도로 서술한다. 통과.
26. **simulator smoke가 feature geometry 검증처럼 기록됐는가?** iOS/Android가 기본 Root HTML app startup smoke이며 four-fit fixture를 로드하지 않았다고 명시한다. 통과.
27. **플랫폼 한계가 빠졌는가?** Android SwiftShader 및 readback, compositor parity, 실기기, hardware GPU, WPT 미검증을 유지했다. 통과.
28. **로드맵 상단 요약이 이전 PR #1354를 최신 merge로 계속 표시하는가?** 요약과 history에 #1357/merge SHA/11,520 matrix를 표시한다. 통과.
29. **로드맵에서 완료된 four-fit slice가 진행 중이거나 미완료 공식 수에 더해졌는가?** `detailInProgress`를 비우고 5-fit row-span만 Chrome 사전 비교 후보·미착수로 표시했다. Agent Browser에서 PR #1357, `78/214`, 빈 진행 열, NEXT 후보를 확인했다. 통과.
30. **Tailnet 배포본이 원본과 다르거나 사용 후 브라우저가 남았는가?** source/preview/HTTP response SHA-256이 일치하고 직전 preview 백업을 보존했다. 이번 Agent Browser 세션은 `active=false`, PID `null`로 닫았고 공유 시뮬레이터·타 프로젝트 session은 정리 대상으로 취급하지 않았다. 통과.

## 판정

30개 항목에서 문서·표시 불일치는 남지 않았다. `postmerge-docs-pr-review-30.md` 자체와 Tailnet 캡처는 최종 PR diff에 포함하며, 그 뒤 원격 PR diff와 로컬 candidate tree를 다시 비교한다. 이 검토는 새 구현의 테스트나 CI 결과를 추가하지 않는다.
