# C11.4 PR #385 문서·상태 동기화 실패 관점 검토 20개

검토 대상은 PR #385 병합 결과를 반영하는 계획, 상태 대장, 계약 0059, 내부 인덱스, 공개 로드맵 변경이다. 구현 PR의 기존 20개 runtime 관점과 별도로 문서 간 의미와 범위를 대조했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 제한 slice만으로 C11.4 또는 C11 전체가 완료 처리되는가 | 상태 대장의 C11.4 checkbox와 완료 집계는 그대로 unchecked다. 통과. |
| 2 | 새 PR 기록이 공식 완료 수를 한 건 올리는가 | 완료 checkbox를 추가하지 않았고 진행 기록만 추가했다. 통과. |
| 3 | PR 번호나 merge SHA가 다른 변경을 가리키는가 | #385와 078f5708a5c13d956150b311d392490cc4faa311이 구현 근거와 일치한다. 통과. |
| 4 | PR #375, #379, #385의 책임이 뒤섞이는가 | positive implicit, negative implicit, auto-repeat 결합의 순서를 분리해 기록했다. 통과. |
| 5 | 기존 106 scene과 신규 18 case를 더한 수가 잘못되는가 | 계획·계약·상태에서 양 플랫폼 124/124로 일치한다. 통과. |
| 6 | Android 기기 정보가 AVD를 실기기로 오인하게 하는가 | API 37 ARM64 AVD 및 sdk_gphone16k_arm64로 구체화했다. 실기기로 확대하지 않는다. 통과. |
| 7 | iOS Simulator 결과가 iOS 실기기 결과로 표현되는가 | iPhone 17 Pro / iOS 26.2 Simulator로 한정했다. 통과. |
| 8 | trace chunk 수가 draw/present 수와 혼동되는가 | 원본 README의 Android 676, iOS 1,284와 각 124/124 draw/present를 분리해 기록했다. 통과. |
| 9 | overflow 표본 개수가 한 플랫폼 전체 합계로 오해되는가 | 세 scene 각각 각 플랫폼 5/5로 표현했다. 통과. |
| 10 | 표면 표본이 CSS border stroke 검증으로 과장되는가 | root background, overflow 표본이며 border stroke는 검증하지 않았다는 원본 한계를 유지한다. 통과. |
| 11 | Android AVD가 hardware GPU 증거로 읽히는가 | software rendering 한계를 유지했다. 통과. |
| 12 | iOS Metal drawable이 compositor 이후 화면으로 오해되는가 | OS compositor 제출 전 readback 한계를 유지했다. 통과. |
| 13 | Chrome 관찰이 독립 규범 기대를 덮어쓰는가 | Chrome auto-fit gutter 차이는 관찰과 규범 기대를 분리해 기록했다. 통과. |
| 14 | 새 계획 링크가 빌드에 없는 /spinon/plan 경로를 가리키는가 | 공개 로드맵 링크를 GitHub의 plan 원문으로 바꿨고 상태 대장도 같은 원칙을 사용한다. 수정 후 통과. |
| 15 | 내부 계획·근거 링크가 존재하지 않는 파일을 가리키는가 | 추가된 실행 README와 20/30 review 경로를 원본 evidence 트리에서 대조했다. 통과. |
| 16 | 각 축에 여러 auto-repeat 또는 모든 repeat grammar가 지원된다고 읽히는가 | 각 축의 현재 제한 grammar상 하나까지라고 쓰고 전체 grammar를 남은 범위로 둔다. 통과. |
| 17 | grid-auto sizing이 임의 CSS track grammar로 넓혀지는가 | Auto와 finite non-negative px LengthPx/TrackList subset으로 제한했다. 비지원 값은 fail-closed다. 통과. |
| 18 | leading implicit list 순환 방향이 positive track과 같다고 읽히는가 | leading은 역방향, trailing은 authored 순서로 구분했다. 통과. |
| 19 | auto-fit이 반복 수 자체를 줄이는 것으로 설명되는가 | repeat count는 유지하고 placement 뒤 빈 repeat track과 규범상 gutter를 접는다고 기록했다. 통과. |
| 20 | status check이 없는데 CI 통과로 보고되는가 | required status check 미설정을 명시하고 CI green 주장을 하지 않는다. 통과. |

## 최종 판정

문서 범위에서 발견한 새 plan route 문제를 GitHub 원문 URL로 수정했다. 마지막 URL 수정 이후 문서 전체 빌드는 다시 실행하지 않았으므로 통과로 간주하지 않는다. 구현 동작에 대한 근거는 병합된 PR #385의 별도 simulator 자료를 가리킨다.
