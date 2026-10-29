# C11.4 상위 계획 상태 동기화 PR 실패 관점 검토 30개

**검토 범위:** `plan/c11-4-grid-track-sizing.md`의 PR #811 기록, 이 변경의 README와 Agent Browser 화면 근거. 제품 코드·상태 checkbox·API 계약은 변경하지 않는다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 완료된 항목을 미구현으로 남기는가 | **통과.** 기존 상위 계획의 “구현/runtime 미시작” 문구를 PR #811 병합 결과로 교체했다. |
| 2 | 다른 C11.4 작업의 계획 상태를 덮어쓰는가 | **통과.** 수정한 단락은 fit-content intrinsic limit 열 축 slice 하나뿐이고, 다음 collapse/auto-repeat 계획 항목은 그대로다. |
| 3 | 날짜를 계획·병합 시점과 혼동하는가 | **통과.** 계획 단락은 2026-10-30 기록으로 두고 PR 병합 시각은 2026-10-29T17:19:06Z로 별도 적었다. |
| 4 | PR 번호가 다른 축 작업을 가리키는가 | **통과.** 집중 계획과 공개 roadmap이 모두 PR #811을 참조한다. |
| 5 | Merge SHA가 잘못되었는가 | **통과.** 기록한 SHA `8b2e3088c84ae4a3df1fd56c9e75eba150ce1f59`가 STATUS와 공개 roadmap 기록과 일치한다. |
| 6 | `fit-content()` 표준 계산의 근거가 빠졌는가 | **통과.** CSS Grid 2025 CRD §11.2를 연결하고 Chrome 관측과 독립 기대값 근거는 별도 precomparison에 둔다. |
| 7 | 행 축 기존 동작을 신규 열 축 구현과 혼합하는가 | **통과.** 기존 row-limit 장면은 선행 범위로, 변경은 implicit columns로 명시한다. |
| 8 | scene 수가 과장되었는가 | **통과.** 세 limit 경계와 한 automatic-minimum control, 총 네 장면으로 고정했다. |
| 9 | 세 limit 장면의 item minimum이 숨겨졌는가 | **통과.** main scene의 `min-width:0`을 명시했다. |
| 10 | automatic-minimum floor control이 main scene에 섞였는가 | **통과.** `min-width:auto`, 40px limit 및 100px floor를 분리해 적었다. |
| 11 | 200px limit이 200px used track을 뜻하는 것처럼 쓰였는가 | **통과.** 문서는 scene 역할만 요약하고 정확한 used-value 표를 집중 계획에 위임한다. |
| 12 | Chrome이 규범 기대값으로 승격되었는가 | **통과.** Chrome Headless 156은 관측 근거로, 계산은 CRD 식과 독립 기대값으로 표현한다. |
| 13 | 두 simulator가 물리 기기 검증으로 확대되었는가 | **통과.** Android는 software AVD, iOS는 Simulator라고 썼다. |
| 14 | PixelCopy 표본이 전체 화면 동일성을 뜻하는가 | **통과.** 고정 표본 수 14/14만 적고 표면 전체 동등성을 주장하지 않는다. |
| 15 | iOS drawable 표본이 compositor 이후 pixel로 확대되었는가 | **통과.** compositor 제출 전 Metal이라고 한정한다. |
| 16 | AVD가 hardware GPU라고 표시되었는가 | **통과.** software AVD로 명시하고 hardware GPU를 미검증으로 둔다. |
| 17 | PR check 부재를 CI 성공으로 서술하는가 | **통과.** check rollup/required check 부재를 명시하고 성공으로 세지 않는다. |
| 18 | 제한 fixture slice를 전체 CSS admission으로 확대하는가 | **통과.** fixture-test opt-in 범위와 일반 author CSS 미지원 경계를 적었다. |
| 19 | pinned WPT 한 부분을 전체 WPT 통과로 확대하는가 | **통과.** 전체 WPT를 완료하지 않았다고 명시한다. |
| 20 | 전체 C11.4 또는 C11이 완료된 것처럼 읽히는가 | **통과.** 두 항목 모두 미완료라고 명시한다. |
| 21 | 하위 완료 수를 바꾸는가 | **통과.** 계획 문서만 수정하고 `spec/STATUS.md`는 수정하지 않는다. |
| 22 | 공식 `72/213` 집계를 올리는가 | **통과.** 공식 집계를 그대로 유지한다고 반복 확인한다. |
| 23 | 내부 계약 버전을 조용히 변경하는가 | **통과.** 계약 문서를 수정하지 않고 버전 변경도 주장하지 않는다. |
| 24 | 구현 근거의 상대 링크가 잘못되었는가 | **통과.** master plan에서 기존 focused plan과 두 evidence 디렉터리의 경로를 상대 링크로 연결한다. |
| 25 | README가 변경 목적과 한계를 분리했는가 | **통과.** 불일치 원인, 동기화 결과, 화면 근거, 판정 경계를 별도 절로 둔다. |
| 26 | Tailnet 화면이 변경 이후 상태인 것처럼 오해되는가 | **통과.** 이 화면은 PR 병합 전 공개 roadmap route이며 master-plan Tailnet 반영은 병합 후 별도 수행한다고 명시한다. |
| 27 | 스크린샷이 PR #811 항목을 실제로 보여주는가 | **통과.** Agent Browser 화면에서 merge SHA, `72/213`, C11.4 미완료 문구가 보인다. |
| 28 | 브라우저 종류 또는 정리 상태가 모호한가 | **통과.** Agent Browser를 썼고 터미널 브라우저는 쓰지 않았으며 닫은 뒤 daemon 0개를 확인했다. |
| 29 | 브라우저 이미지가 깨진 상태를 정상으로 기록하는가 | **통과.** 최종 캡처는 화면에서 확인했고 PNG 크기와 SHA-256을 기록했다. |
| 30 | 무관한 스냅샷이나 source 파일이 변경 대상에 들어가는가 | **통과.** 제품 코드·runtime·기존 snapshot·공식 status는 변경 대상에 포함하지 않고 plan 한 단락과 전용 evidence만 다룬다. |

**판정:** 30개의 서로 다른 문서·근거 실패 관점을 확인했다. 코드 검토나 simulator 재실행을 대신하지 않는다. 이 PR은 기록 동기화만 하며 제품 검사는 실행하지 않았다.
