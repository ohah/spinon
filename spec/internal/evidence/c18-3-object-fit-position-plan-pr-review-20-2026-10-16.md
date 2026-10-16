# C18.3 계획 PR 변경분 적대적 검토 20개

이 검토는 계획 PR의 실제 파일 변경과 사전 비교 산출물만 대상으로 한다. C18.3 runtime 구현 검토를 대신하지 않는다.

| # | PR diff에서 공격한 지점 | 결과 |
|---:|---|---|
| 1 | 계획을 이미 구현된 기능으로 표시하는가? | 미착수로 표시했고 `STATUS.md`는 수정하지 않았다. |
| 2 | 상위 ID와 다음 세부 slice가 겹치거나 기존 plan ID를 재사용하는가? | 새 C18.3이며 기존 C18.1/2 계약·계획과 별도다. |
| 3 | 규범 문서가 object-fit과 position 의미를 정의하는가? | CSS Images 3의 두 property section을 직접 연결했다. |
| 4 | 기존 내부 contract가 선행 조건으로 보이지 않는가? | 0065와 C14.1–3/C18.1을 선행으로 기록했다. |
| 5 | PNG hash와 generator가 일치하는가? | 24×12 생성 PNG SHA를 capture observation과 대조했다. |
| 6 | Chrome 실행이 pinned product가 아닌가? | DevTools version endpoint에서 Chrome `155.0.8059.85`를 확인했다. |
| 7 | viewport·DPR가 다른 결과와 섞였는가? | screenshot과 observation 모두 224×72, DPR 1이다. |
| 8 | HTML frame 입력이 추정값뿐인가? | 각 이미지 DOMRect와 computed property를 browser capture가 저장했다. |
| 9 | 12개 이미지 중 미로드 source가 있는가? | 모든 image가 complete이고 natural size 24×12인 것을 assert했다. |
| 10 | 다섯 fit keyword 중 하나가 빠졌는가? | fixture에서 fill/contain/cover/none/scale-down을 각각 사용한다. |
| 11 | scale-down의 한 갈래만 oracle에 있는가? | natural보다 작은 frame과 natural을 유지하는 frame을 모두 포함했다. |
| 12 | 0–100 밖 position이 CSS computed style에서 거부되는가? | Chrome computed values `-25% 150%`, `150% -25%`를 캡처했다. |
| 13 | none의 image-box clip negative control이 부족한가? | frame 안 source pixel과 frame 바깥 root pixel을 함께 기록했다. |
| 14 | background anchor 및 contain bar를 생략했는가? | 두 pixel 영역을 별도 표본으로 기록했다. |
| 15 | screenshot을 source PNG처럼 잘못 링크했는가? | 화면 파일과 observation JSON을 별도 경로·hash로 보존한다. |
| 16 | capture script가 session cleanup 없이 끝나는가? | `finally`에서 전용 session을 닫고 inactive/pid-null을 확인했다. |
| 17 | test fixture와 production payload를 같은 artifact에 넣는다고 적었는가? | implementation gate가 opt-in feature와 default artifact 음성 검사를 요구한다. |
| 18 | AVD/Simulator가 실기기 검증으로 확대 해석되는가? | 두 simulator만 요구하고 hardware/device/compositor 범위를 제외했다. |
| 19 | PR 계획에서 공식 완료 체크를 선행해버리는가? | 상태 체크 및 계약 갱신은 실제 구현 PR 관문에 한정했다. |
| 20 | plan-review와 PR-diff review를 같은 표로 재사용하는가? | 계획 실패 검토와 이번 PR diff 검토를 별도 문서로 기록했다. |
