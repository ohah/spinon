# 계획 PR 변경 실패 관점 검토 20개

계획 문서, fixture, 독립 기대값, Chrome observation, public roadmap과 parent plan 변경을 한 revision에서 검토했다.

| # | 변경 검토 질문 | 결과 |
| --- | --- | --- |
| 1 | 새 계획은 기존 계획의 실제 제외 범위에 연결되는가? | `nested-grid-intrinsic.md`가 nested implicit track expansion을 제외한 문구와 parent link를 대조했다. 통과. |
| 2 | Chrome 수치가 독립 기대값에 복사되어 oracle 순환이 생기는가? | 규범 계산 JSON과 observation JSON이 다른 파일이며 계산식도 설명한다. 통과. |
| 3 | fixture URL과 README 경로가 정확한 파일을 가리키는가? | repository-relative fixture path를 확인했다. 통과. |
| 4 | screenshot 링크가 저장된 파일과 일치하는가? | README 상대경로와 PNG 파일명을 대조했다. 통과. |
| 5 | track pattern 배열 순서와 scene 이름이 문서 전체에서 바뀌는가? | `spanTwo`/`oneRow`, `[18,18]`/`[40,0]`, 40/44px를 계획·JSON·README에서 대조했다. 통과. |
| 6 | image natural size가 증거 없이 가정되는가? | Agent Browser observation의 두 image dimensions와 PNG hash를 남겼다. 통과. |
| 7 | browser 버전 표현이 UA가 증명하는 정도를 넘는가? | `HeadlessChrome/156.0.0.0` 및 Agent Browser 0.31.1만 기록했다. 통과. |
| 8 | simulator evidence가 이미 있는 것처럼 쓰였는가? | 상태·계획에 Spinon/Android/iOS 미실행을 명시했다. 통과. |
| 9 | C11.4 완료 checkbox 또는 72/213이 잘못 올라가는가? | status 변경을 하지 않고 parent/public roadmap에서 미완료 수를 유지한다. 통과. |
| 10 | HTML fixture가 이 계획 밖 auto-placement를 은밀히 포함하는가? | 각 inner row가 numeric line으로 명시됐고 auto-placement는 없다. 통과. |
| 11 | fixture에 unintended font dependency가 남는가? | Ahem/font를 사용하지 않고 고정 block과 PNG만 쓴다. 통과. |
| 12 | embedded image path가 repository asset으로 resolve되는가? | `tests/fixtures/resources/c14-1-png-basic.png`가 존재하고 hash가 README와 맞다. 통과. |
| 13 | test/implementation plan이 default product artifact를 오염시키는가? | fixture-test opt-in positive/negative artifact controls를 구현 단계에 포함했다. 통과. |
| 14 | Android 물리기기 검증이 implicit 요구로 포함되는가? | AVD와 iOS Simulator만 명시하고 실기기는 연결하지 않는다. 통과. |
| 15 | simulator frame만으로 pixel capture를 주장하는가? | draw/present와 optional surface readback을 구별하고 beyond-border는 제외한다. 통과. |
| 16 | 계획이 내부 계약 버전을 불필요하게 올리는가? | 계약 의미 변경 전에는 version bump하지 않으며 계획 PR은 0059를 수정하지 않는다. 통과. |
| 17 | parent plan과 public roadmap의 상태 설명이 충돌하는가? | 두 곳 모두 계획/oracle만 완료, runtime 미실행으로 일치한다. 통과. |
| 18 | 내부 PR template와 맞지 않게 계획 PR을 implementation complete로 쓰는가? | 변경 유형·상태 ID·API 명세 부재 사유·남은 범위를 PR body에 구분하도록 한다. 통과. |
| 19 | user-scope 변경이나 기존 untracked evidence가 같이 stage되는가? | 변경 파일을 명시적으로 stage하고 기존 untracked 파일은 제외한다. 통과. |
| 20 | Agent Browser session/process가 남아 있는가? | close 뒤 process 목록에서 agent-browser/HeadlessChrome가 없음을 확인했다. 통과. |

계획 PR diff의 수정 blocker는 발견하지 않았다. 이는 문서 변경 검토이며 코드/runtime 검증이나 GitHub required checks 통과를 뜻하지 않는다.
