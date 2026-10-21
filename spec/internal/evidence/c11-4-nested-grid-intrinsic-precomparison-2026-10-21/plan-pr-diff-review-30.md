# C11.4 nested Grid intrinsic plan PR 변경 적대 검토 30개

대상은 `plan/c11-4-nested-grid-intrinsic.md`, 상위 계획과 `spec/STATUS.md` 연결, Chrome 사전 비교 README/JSON/fixture/screenshot, 계획 검토 20개다. 계획 PR diff에서 서로 다른 실패 경로 30개를 검토했다.

| # | 확인한 실패 경로 | diff의 방어 또는 결과 |
| ---: | --- | --- |
| 1 | nested slice를 C11.4 완료 체크로 승격 | `STATUS.md`의 C11.4 checkbox는 그대로 `[ ]`다. |
| 2 | 계획 추가를 공식 완료 항목 증가로 계산 | 계획·상태 본문에 완료 수를 올리지 않고 그대로 유지한다고 명시한다. |
| 3 | 현재 Spinon 동작을 검증한 것처럼 씀 | README와 plan은 Chrome 관찰이며 Spinon/app 실행은 아직 없다고 분리한다. |
| 4 | 상세 계획이 C11.4 원 계획과 고립됨 | 상위 plan에 한 줄 요약과 상세 계획/evidence 링크를 추가했다. |
| 5 | 공식 상태 대장에 후속 작업이 나타나지 않음 | C11.4 하위 항목에 계획과 사전 비교를 링크했다. |
| 6 | 계획에서 evidence 파일 상대경로가 틀림 | root `plan/`에서 `../spec/internal/evidence/` 경로를 사용한다. |
| 7 | README에서 plan 링크가 잘못 올라감 | evidence README 기준 네 단계 상위 `../../../../plan/` 링크를 사용한다. |
| 8 | 계약 번호를 새로 올리거나 공개 동작으로 선언 | 내부 계약 0059는 참고 링크이며 변경 또는 version bump를 요구하지 않는다. |
| 9 | 규범 참조가 편집 가능한 최신 초안으로 바뀜 | CSS Grid Level 2의 고정 2025 CRD URL과 절 번호를 기록했다. |
| 10 | browser 기대값이 규범 문장으로 잘못 표기 | §12.5/§12.5.1로부터의 기대 계산과 Chrome CSSOM 관찰을 분리했다. |
| 11 | W3C 텍스트를 과도하게 그대로 복사 | 규범 알고리즘은 짧게 요약하고 긴 원문 인용은 넣지 않았다. |
| 12 | 요구한 브라우저 방식 대신 terminal-browser를 사용 | oracle은 사용자 지침에 맞춰 Agent Browser로 렌더했다. |
| 13 | 직전 C11.4 결과의 Chrome 155를 새 관찰 버전으로 혼용 | 캡처의 실제 User-Agent `HeadlessChrome/156.0.0.0`을 JSON/plan에 썼다. |
| 14 | 캡처 시각을 실행 로그 없이 분 단위로 꾸며냄 | observation에는 확인 가능한 날짜만 저장하고 시각은 기록하지 않는다. |
| 15 | Ahem fallback이 로드됐는지 확인하지 않음 | font readiness가 true일 때 geometry를 캡처했다. |
| 16 | 변경된 Ahem source bytes를 동일 자료로 취급 | Ahem 파일 SHA-256을 JSON과 README에 함께 기록했다. |
| 17 | 화면이 이전 CSS 결과와 섞임 | 개선된 캡처를 같은 fixture에서 다시 저장하고 hash를 갱신했다. |
| 18 | Screenshot의 pixel dimensions/hash가 빠져 검증 불가 | 1280×633과 캡처 SHA-256을 기록했다. |
| 19 | observation JSON의 scene ID가 fixture id와 다름 | browser 원본 `span-two`, `one-track`를 유지하고 계획 표에서 `definite` 원 selector와 control 설명 ID를 연결했다. |
| 20 | Ahem text intrinsic 폭 두 개를 혼동 | 70/100px 문자열과 60px 문자열을 각각 지정했다. |
| 21 | inner gap과 outer gap을 같은 크기에 중복 가산 | nested width와 parent track distribution을 서로 다른 gap으로 계산했다. |
| 22 | span-two 기대값을 nested item width 대신 외부 width에서 역산 | 134px inner contribution에서 4px parent gap을 뺀 130px을 나눠 65/65px로 도출했다. |
| 23 | one-track 장면의 빈 track이 nonzero가 됨 | 예상 used tracks `134/0px`와 외부 total `138px`를 분리 기록했다. |
| 24 | definite flex control을 intrinsic contribution matrix로 셈 | 120px root의 58/58px tracks를 별도 control로 제한한다. |
| 25 | item overflow 14px을 parent intrinsic contribution에 더함 | control의 내부 overflow는 parent sizing 값과 별도 검사로 명시했다. |
| 26 | inner item span/implicit row가 nested contribution 기대를 오염 | 해당 장면은 진단 자료로 남기고 acceptance에서 제외했다. |
| 27 | 일반 nested Grid와 subgrid가 같은 구현으로 취급 | subgrid 제외와 parent track 비공유를 명시했다. |
| 28 | `is_leaf` input skip만으로 알고리즘 defect 확정 | 실제 `LayoutInput`/Taffy trace의 첫 divergence를 확인하고 필요할 때만 수정하도록 한다. |
| 29 | trace에서 parent item ID와 child intrinsic input owner가 뒤바뀜 | outer query는 nested Grid node, leaf text source는 inner Grid owner로 각각 검사한다. |
| 30 | 계획의 simulator·artifact·pixel 검증을 이미 수행한 것으로 오인 | Android API 37 AVD/iOS Simulator, 기본/opt-in artifact, draw/present와 표면 pixel은 모두 후속 실행 acceptance로 표시했다. |

이 검토에서 고친 사항은 실제 브라우저 버전의 정확한 표기, 허구의 분 단위 timestamp 제거, fixture scene ID 정렬, fixture-test feature 이름 구체화, inner-span 진단의 acceptance 제외다. 남은 blocker는 없다. 이 PR은 계획·비교 자료만 추가하며 product code를 바꾸지 않는다.
