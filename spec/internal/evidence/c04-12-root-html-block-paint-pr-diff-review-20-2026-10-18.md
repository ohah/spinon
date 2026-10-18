# C04.12 루트 HTML Block scene 계획 PR 변경 검토

구현 전 계획 PR의 실제 변경 파일을 검토했다. 이 표는 계획 자체의 실패 경로 표와 별개로, 공식 상태·계획·Chromium oracle·첨부 이미지·문서 범위를 대조한다. 코드나 simulator 동작을 검증한 것으로 세지 않는다.

| # | 변경에서 확인한 경계 | 확인 결과 |
| --- | --- | --- |
| 1 | 기존 C04.12 상위 checkbox | 계속 미완료다. |
| 2 | 신규 Block scene 하위 checkbox | 구현 전 계획으로 미완료 상태다. |
| 3 | STATUS 완료 카운터와 별도 Tailnet roadmap | 두 곳 모두 공식 완료 수를 `74/207`로 유지하고 신규 항목을 미구현 계획으로 표시한다. |
| 4 | 상위 C04.12 계획 링크 | 새 계획 파일을 가리키며 경로가 존재한다. |
| 5 | 사전 비교와 계획 검토 링크 | evidence와 20개 표 파일을 가리키며 경로가 존재한다. |
| 6 | fixture 원본 경로 | 계획의 `../tests/...` 링크는 repo와 Tailnet의 `/spinon/tests/...`로 연결되고, README는 혼동되는 상대 URL 대신 저장소 경로를 코드로 표시한다. |
| 7 | fixture SHA-256 | README의 값이 현재 fixture 바이트 해시와 일치한다. |
| 8 | light/dark 캡처 해시 | 두 PNG 해시가 README에 기록된 값과 일치한다. |
| 9 | oracle 버전 | Chrome full version과 macOS 버전이 적혀 있으며 이전 프로젝트 버전에서 유추하지 않았다. |
| 10 | 뷰포트·DPR·색상 preference | CSS `320×240`, DPR 1, light/dark를 구분해 기록한다. |
| 11 | CSSOM 값과 화면 픽셀 | geometry/computed color를 screenshot RGBA 표본과 별도 표로 나눴다. |
| 12 | head/metadata 처리 | head의 UA `display:none`을 확인했고 author `head {display:none}` 규칙으로 결과를 인위적으로 만들지 않았다. |
| 13 | hidden descendant | 자식 computed `display:block`과 0 frame/비노출 pixel을 동시에 기록했다. |
| 14 | root Canvas와 body paint | opaque html root + transparent body의 oracle만 다루며 body background propagation으로 범위를 넓히지 않는다. |
| 15 | scheme cascade | dark media가 자식 배경만 바꾸는 관찰을 포함하고 system theme 전체를 지원한다고 주장하지 않는다. |
| 16 | CSS/HTML 지원 범위 | visible node·속성·text 제한과 fail-closed를 계획에 명시한다. |
| 17 | 기본 앱 표시 범위 | Bootstrap HTML 전체, 글리프, 제품 UI 렌더링을 확인한 것으로 쓰지 않았다. |
| 18 | API/ABI 및 계약 버전 | 이번 PR은 계획·fixture·evidence만 변경하며 공개 API, ABI, 계약 번호를 올리지 않는다. |
| 19 | fixture artifact 경계 | 향후 simulator adapter의 opt-in feature/build flag와 기본·opt-in artifact 비교를 계획에 둔다. 아직 symbol이나 product feature가 생긴 것처럼 쓰지 않았다. |
| 20 | 후속 검증 독립성 | 계획 표, 구현 코드 20개 검토, 향후 PR diff 검토를 각각 분리하며 이번 계획 PR에 simulator 통과를 주장하지 않는다. |

README의 fixture URL은 저장소와 Tailnet의 문서 경로 기준이 다른 문제를 피하도록 코드 경로로 표시했다. 최종 변경에서 추가 수정은 필요하지 않았다. Rspress 전체 문서 build는 이번 파일의 dead link가 아니라 기존 C11/C13/C06 증거 문서의 누락 asset/link에서 실패했다. 새 문서의 상대 링크와 `git diff --check`를 별도로 확인했고, 필요한 페이지만 격리한 Rspress `2.0.22` preview build는 기존 누락 링크 검사만 끈 상태로 성공했다. 이 targeted build 성공은 전체 문서의 기존 누락 링크를 통과로 바꾸지 않는다.
