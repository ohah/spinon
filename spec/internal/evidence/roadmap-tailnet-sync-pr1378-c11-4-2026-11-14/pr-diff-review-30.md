# PR #1378 병합 후 문서·Tailnet 동기화 변경 검토 30개

각 항목은 상태 대장, 공개 로드맵, 계획, 내부 계약, 병합 메타데이터, 선택 publication 및 실제 Agent Browser 결과를 서로 다른 실패 관점으로 대조했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 기록한 PR 번호가 다른 구현을 가리키는가 | #1378 제목과 대상 저장소를 GitHub PR에서 대조했다. 통과. |
| 2 | source head가 병합한 코드와 다른가 | PR source head `96644af6a8430be8f9f388e5c81ac9dee1ccc65d`가 병합 기록과 맞는다. 통과. |
| 3 | merge SHA가 현재 main과 다른가 | `cbf3b8579b7aa4915998378acd2d0b03c7916077`가 merge 기록과 현재 main HEAD다. 통과. |
| 4 | Rebase and merge가 아닌 방식을 기록했는가 | PR 병합 방식과 merge SHA의 단일 parent `1a2ddca325667a2e372eddfcfa76f1a2feed48c0`를 대조했다. 통과. |
| 5 | merge UTC 시간이 잘못되었는가 | GitHub의 `2026-11-13T19:42:09Z`와 세 문서의 시간 표기를 대조했다. 통과. |
| 6 | PR check가 비었는데 CI green으로 기록하는가 | GitHub `statusCheckRollup`이 비어 있고 모든 문서는 CI 통과를 주장하지 않는다. 통과. |
| 7 | 문서 PR에서 구현 영역 라벨을 잘못 옮겼는가 | #1378 merge record에는 구현 PR의 `enhancement`, `area:layout`만 적었다. 통과. |
| 8 | 병합만으로 공식 완료 수를 올렸는가 | 상태 대장, 계획, 계약, 로드맵 모두 `78/214`를 유지한다. 통과. |
| 9 | 제한 slice를 C11.4 또는 C11 전체 완료로 표시하는가 | C11.4/C11 체크는 미완료로 남고 하위 bounded slice의 병합만 추가했다. 통과. |
| 10 | 내부 계약 버전을 올렸는가 | 내부 숫자 버전 `0.1.0`이 그대로다. 통과. |
| 11 | 공식 C11.4 체크박스를 켰는가 | `spec/STATUS.md`에서 C11.4 intrinsic track sizing 체크박스는 미완료다. 통과. |
| 12 | Chrome oracle 장면 수를 RuntimeSession 전체 실행으로 오해하는가 | 138,240 oracle scenes와 6,912 runtime sample을 구분해 썼다. 통과. |
| 13 | 6,912 sample 분모가 누락되거나 전수 실행처럼 보이는가 | RuntimeSession sample `6,912/138,240`의 범위를 실행 근거와 연결했다. 통과. |
| 14 | uniform endpoint 수치가 다른 profile에서 왔는가 | three-fit uniform endpoint `11,520/11,520`을 전용 실행 근거와 대조했다. 통과. |
| 15 | three-fit auto-placement 분모가 과장되었는가 | `5,760` 회귀를 계획/runtime README에 있는 profile과 대조했다. 통과. |
| 16 | one-/two-/four-/five-fit 회귀가 무관한 새 기능처럼 표현되는가 | 모두 인접 회귀로만 표현했고 공식 완료 수에 더하지 않았다. 통과. |
| 17 | six-fit RuntimeSession 전체가 재실행된 것으로 읽히는가 | metadata 전체와 첫 `11,520/138,240` runtime chunk를 분리했다. 통과. |
| 18 | Android smoke를 실제 기기 또는 GPU 픽셀 검증으로 읽는가 | API 37 ARM64 AVD 화면 smoke로 한정했다. 통과. |
| 19 | iOS smoke를 실제 기기 검증으로 읽는가 | iPhone 17 Pro/iOS 26.2 Simulator로 표기했다. 통과. |
| 20 | 화면 smoke를 PixelCopy/Metal pixel parity로 확대하는가 | readback과 surface pixel parity 미검증을 반복해 명시했다. 통과. |
| 21 | 전체 CSS Grid 또는 WPT 적합성을 암시하는가 | 내부 제한 Root HTML slice이며 일반 CSS admission/WPT는 범위 밖이다. 통과. |
| 22 | 작업용 iOS Simulator가 여전히 켜져 있다고 기록하는가 | 임시 device 삭제 및 현재 booted device 부재를 구분해 기록했다. 통과. |
| 23 | roadmap 최신 요약에 이전 #1375가 남아 있는가 | 최신 요약은 #1378이고 #1375는 과거 기록으로 유지된다. 통과. |
| 24 | 과거 #1375 검증 기록을 #1378 결과로 덮어썼는가 | #1375와 #1378에 서로 다른 merge 정보와 분모가 남는다. 통과. |
| 25 | `detailNext`가 이미 병합된 three-fit 작업을 다음 항목으로 표시하는가 | stale 항목을 제거하고 새 bounded slice 미선정을 표시했다. 통과. |
| 26 | 다음 후보를 정하지 않았는데 진행 중으로 꾸미는가 | 칸반은 `0/2`이고 현재 bounded C11.4 slice 미선정으로 표시된다. 통과. |
| 27 | 로드맵 summary의 다섯 local link가 publication에서 빠지는가 | 최신 카드의 plan, contract, evidence, merge, STATUS route 모두 manifest에 있다. 통과. |
| 28 | 계획·실행 근거·병합 기록 route가 빈 링크인가 | Agent Browser에서 세 route를 실제로 열어 title과 병합 내용을 확인했다. 통과. |
| 29 | Tailnet 교체 중 이전 내용을 백업하지 않거나 페이지를 삭제하는가 | 교체 18개 모두 backup hash 일치, 삭제 0개다. 통과. |
| 30 | 게시 후 remote 내용이 local 선택 파일과 다르거나 브라우저가 남는가 | HTTPS route `215/215` byte/hash가 일치했고 전용 Agent Browser session은 닫혔다. 통과. |

초기 선택-root build는 public SVG 누락으로 실패했다. 해당 로그를 보존하고 아이콘 파일을 선택 루트에 추가한 뒤 Rspress `2.0.22` build와 Tailnet sync를 완료했다. 이는 앱 구현 검증 실패가 아니다.
