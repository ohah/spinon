# C11.4 PR #385 문서 동기화 PR diff 적대 검토 30개

검토 대상은 현재 문서 동기화 PR diff 전체다. 기준은 PR #385 병합 이후 main의 C11.4 상태, 구현 실행 README, 계약 0059와 공식 프로젝트 규칙이다. 각 관점은 별도의 경계나 문서 실패를 확인한다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | PR에 제품 코드나 무관한 파일이 섞이는가 | 변경 대상은 C11.4 관련 계획 3개, STATUS, 계약 0059, 내부 인덱스, public roadmap, 이 sync의 README와 두 리뷰 자료다. 통과. |
| 2 | 공개 로드맵 상단에 중복되거나 다른 기능인 기록이 삽입되는가 | 새 C11.4 PR #385 note 한 건을 기존 최신 note 앞에 추가했다. 기존 항목 본문은 그대로 유지했다. 통과. |
| 3 | 계획 문서가 아직 미구현 상태라고 남아 병합 결과와 충돌하는가 | 현재 계획 상태와 결론을 구현·병합으로 갱신하고, 제한 scope 및 잔여 범위를 함께 적었다. 통과. |
| 4 | 구현 전 관찰을 현재 코드의 미구현 결함으로 읽는가 | 과거 코드 경계 절을 구현 전 관찰로 표시했다. 현재 제한 결과는 별도 절에서 설명한다. 통과. |
| 5 | C11.4 전체 완료로 부모 summary가 앞서 바뀌는가 | C11.4 checkbox는 unchecked로 유지하고 full Grid/WPT 등 잔여 범위를 기록했다. 통과. |
| 6 | 공식 완료 수에 PR #385를 완료 항목으로 더하는가 | 새 체크 항목이 없으며 완료 집계는 그대로다. 통과. |
| 7 | PR 번호나 Rebase and merge SHA가 부정확한가 | 각 신규 기록의 PR #385와 merge SHA가 078f5708a5c13d956150b311d392490cc4faa311로 일치한다. 통과. |
| 8 | PR #375의 positive implicit 범위를 PR #385 결과로 재서술하는가 | #375의 범위를 유지하고 #379 및 #385의 후속 관계만 덧붙였다. 통과. |
| 9 | PR #379 당시 미검증이었던 결과를 그 시점에 이미 지원했다고 쓰는가 | PR #379가 결합을 다루지 않았고 #385가 나중에 추가했다는 시간 순서를 구분했다. 통과. |
| 10 | 제한 auto-repeat 문법이 임의 CSS Grid 문법으로 확장되는가 | 축마다 하나까지인 현재 제한 grammar로 한정하고 여러 repeat, named lines/areas 등은 남겼다. 통과. |
| 11 | placement가 authored track 수 기준으로 풀리는가 | negative/positive numeric line이 expanded repeat count 기준이며 secondary axis 선해석 결함을 고쳤다고 기술한다. 통과. |
| 12 | leading implicit grid-auto list cycle 순서가 반대로 기록되는가 | leading은 역방향, trailing은 authored 순서로 구분했다. 통과. |
| 13 | grid-auto 지원 타입이 잘못 넓어지는가 | Auto, 유한·0 이상 px LengthPx와 TrackList subset으로 한정했다. unsupported values는 fail-closed다. 통과. |
| 14 | auto-fit이 반복 수 감소로 기술되는가 | repeat count를 보존하고 placement 뒤 empty repeated tracks와 규범상 인접 gutter를 collapse한다고 기록했다. 통과. |
| 15 | Chrome 155 관찰이 W3C 규범 oracle을 대체하는가 | 관찰과 규범 기대를 분리하며 gutter 차이에서 규범값을 사용했다고 명시했다. 통과. |
| 16 | 18개 신규 case가 기존 106 IDs를 대체하거나 중복하는가 | 실행 근거의 합집합 124 scene과 각 플랫폼 124/124 결과를 반영했다. 통과. |
| 17 | Android 결과가 실기기 또는 hardware GPU 검증으로 읽히는가 | API 37 ARM64 AVD, software renderer 한계를 명시했다. 통과. |
| 18 | iOS Metal readback이 compositor 뒤의 완성 화면 검증으로 읽히는가 | iPhone 17 Pro/iOS 26.2 Simulator와 compositor 이전 readback 한계를 명시했다. 통과. |
| 19 | draw/present count와 trace chunk count가 한 지표로 뭉쳐지는가 | 플랫폼별 124/124 draw/present와 676/1,284 trace chunks를 분리했다. 통과. |
| 20 | 표면 비교가 border stroke까지 검증했다고 주장하는가 | border stroke 미구현·미검증 범위를 source evidence에 맞춰 유지한다. 통과. |
| 21 | 세 overflow scene 표본의 개수가 잘못 요약되는가 | 각 플랫폼에서 각 scene 5/5로 기록해 세 scene 합계와 혼동하지 않는다. 통과. |
| 22 | 문서 링크가 상대 경로 이동이나 잘못된 확장자로 끊기는가 | plan, contract, README, implementation review 경로와 GitHub PR 링크를 파일 트리와 대조했다. 통과. |
| 23 | 새 공개 로드맵 plan 링크가 제공되지 않는 RSPress route를 가리키는가 | 새 항목의 plan 링크를 GitHub 원문 URL로 수정했다. 통과. |
| 24 | public roadmap HTML 속성 따옴표나 삽입된 markup이 손상되는가 | 해당 anchor의 href만 교체했고 surrounding HTML 구조는 유지된다. 통과. |
| 25 | STATUS에서 plan 링크가 로컬 빌드 dead route로 남는가 | plan link는 canonical GitHub URL로 두고 내부 자료만 문서 상대 경로로 연결한다. 통과. |
| 26 | 내부 계약의 숫자 버전을 출시 전 임의로 올리는가 | 계약 0059 버전 0.1.0을 유지한다. 통과. |
| 27 | 문서 변경이 새 public API를 만들어 낸다고 암시하는가 | 새 공개 API가 없음을 명시한다. 통과. |
| 28 | 새 evidence README가 이전 106-scene 근거와 최신 124-scene 근거를 혼동하는가 | 최신 124-scene 표와 과거 106-scene 기준 표를 별도 섹션으로 가리킨다. 통과. |
| 29 | required checks가 없는 상황에서 CI 성공을 주장하는가 | GitHub required status check 미설정 사실과 빌드 미통과 상태를 명시한다. 통과. |
| 30 | 문서 빌드가 최종 링크 수정 뒤 통과했다고 오해되는가 | 빌드가 마지막 URL 수정 전 실행되어 exit 1이었고 이후 재실행하지 않았다고 README에 기록했다. 통과. |

## 최종 판정

최종 diff에서 완료 범위, 구현 근거, 플랫폼 한계 및 링크 경계를 확인했다. 발견된 public roadmap plan 경로는 GitHub 원문으로 고쳤다. 이 diff review는 전체 문서 빌드 성공을 주장하지 않는다.
