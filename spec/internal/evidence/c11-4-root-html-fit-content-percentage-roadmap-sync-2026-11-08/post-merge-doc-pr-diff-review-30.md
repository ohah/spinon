# C11.4 percentage fit-content · 병합 후 문서 PR diff 실패 관점 검토 30개

검토 대상은 기능 PR #1268 병합 뒤의 `spec/STATUS.md`, 내부 계약 0059, 전용 계획, `spec/public/roadmap.html`, Tailnet 동기화 자료다. 구현 PR의 30개 diff 검토 및 runtime 20개 검토와 구분했다.

| # | 독립 문서 diff 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | 상태 대장 기준일이 오늘과 맞는가 | 기준일을 2026-11-08로 갱신했다. 통과. |
| 2 | C11 상위 체크가 잘못 완료되는가 | C11 항목은 기존 미완료 체크를 유지한다. 통과. |
| 3 | C11.4 intrinsic track sizing 체크가 완료되는가 | C11.4 체크를 미완료로 유지한다. 통과. |
| 4 | 다른 C11 미완료 하위 단계가 변경되는가 | C11.3·C11.3.3 등 주변 체크는 수정하지 않았다. 통과. |
| 5 | 공식 완료 집계가 증가하는가 | public roadmap의 공식 API 응답은 계속 78/214를 보인다. 통과. |
| 6 | PR 번호·merge SHA가 일치하는가 | GitHub PR #1268 Rebase merge 정보와 `818d83b800d86c09bb379a93605aa9d5fc50aaa9`을 대조했다. 통과. |
| 7 | merge timestamp가 잘못된 날짜로 적히는가 | UTC `2026-11-07T16:22:21Z`와 KST 기준 문서일 2026-11-08을 구분했다. 통과. |
| 8 | Chrome 비교를 Spinon runtime 결과로 부르는가 | Chrome 157은 independent 7-scene oracle로 분리했다. 통과. |
| 9 | direct percentage 지원 범위가 과장되는가 | definite 40×100px Root HTML profile의 singleton cap으로 제한했다. 통과. |
| 10 | percentage basis가 Grid content box가 아닌가 | definite inner block size basis를 명시했다. 통과. |
| 11 | >100% cap을 100%로 clamp했다고 잘못 쓰는가 | 100% 초과 입력은 범위에 남고 used size는 max-content에 의해 제한된다. 통과. |
| 12 | item minimum contribution이 누락되는가 | minimum floor scene과 결과를 계획·근거에 유지했다. 통과. |
| 13 | 후속 implicit row 재사용 결과가 빠지는가 | 다음 probe row를 검증 범위에 기록했다. 통과. |
| 14 | Calc/mixed list 및 다른 shape 거부가 빠지는가 | percentage-bearing Calc, mixed list, indefinite height, areas, column flow, explicit placement의 거부를 보존했다. 통과. |
| 15 | internal contract의 버전이 불필요하게 증가하는가 | 내부 숫자 버전 `0.1.0`을 유지했다. 통과. |
| 16 | internal contract가 public API 지원을 암시하는가 | limited parsed Root HTML candidate로 기록하고 public support를 주장하지 않는다. 통과. |
| 17 | 테스트 수가 실행 근거와 다른가 | projection 1/1, explicit regression 2/2, layout 2/2, percentage runtime 3/3, px runtime 2/2로 대조했다. 통과. |
| 18 | scoped Clippy·rustfmt를 전체 CI로 오인하는가 | GitHub check rollup이 비었음을 문서화해 CI green 주장을 막았다. 통과. |
| 19 | Android 실행이 실기기나 hardware GPU로 오인되는가 | API 37 ARM64 AVD와 visual smoke 한계를 명시했다. 통과. |
| 20 | iOS 실행이 compositor 이후 pixel parity로 오인되는가 | iPhone 17 Pro Simulator screenshot을 visual smoke로 한정했다. 통과. |
| 21 | iOS data-container 경로 변경이 숨겨지는가 | 설치 전후 path 변경과 연속성 미확인을 status와 evidence에 남겼다. 통과. |
| 22 | plan PR #1267와 implementation PR #1268 순서가 뒤집히는가 | 계획 PR 선행, 구현 PR 후행을 전용 plan에 기록했다. 통과. |
| 23 | plan 문서가 아직 구현 시작 전이라고 남는가 | 상태 문구를 PR #1268 merge 결과로 갱신했다. 통과. |
| 24 | roadmap active card가 완료 상태를 표시하는가 | `C11.4 · ACTIVE`를 유지하며 완료 열이나 체크를 건드리지 않는다. 통과. |
| 25 | 현재 제목이 다시 추상적 catch-all인가 | 카드에는 짧은 `fit-content %`를 쓰고 상단 최신 진행 설명에 전체 Root HTML 항목명을 기록했다. 통과. |
| 26 | 카드 제목이 실제 viewport에서 잘리는가 | Agent Browser desktop/mobile에서 label 전체가 보여야 한다는 기준으로 제목을 줄이고 확인했다. 통과. |
| 27 | 최신 진행 설명이 이전 PR #1265에 머무는가 | 현재 설명을 PR #1268, merge SHA 및 실행 근거로 갱신했다. 통과. |
| 28 | relative evidence links가 다른 경로를 가리키는가 | Status/internal/roadmap 링크의 evidence 디렉터리 및 plan 경로를 대조했다. 통과. |
| 29 | Tailnet 제공본이 branch source와 다른가 | local HTTP·Tailnet HTTPS 모두 200과 최종 source SHA `505a4d…`가 일치한다. 통과. |
| 30 | sync가 서버나 GitHub Pages를 건드리는가 | preview 파일 하나를 atomic rename했고 PID 28040을 재시작하지 않았다. Pages deploy는 하지 않았다. 통과. |

## 판정

문서 diff에서 추가 수정이 필요한 오류는 찾지 못했다. C11/C11.4 미완료와 공식 78/214를 유지하고 Tailnet 제공본 및 두 viewport Agent Browser 결과를 별도 보존했다.
