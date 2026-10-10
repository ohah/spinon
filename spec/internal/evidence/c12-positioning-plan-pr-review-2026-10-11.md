# C12 위치 지정 계획 PR 변경 검토

**검토 범위:** C12 계획 추가와 상태·계획 인덱스·공개 로드맵 동기화

**검토 성격:** 문서 변경 및 의존성 검토. C12 기능이 구현되거나 검증됐다는 증거가 아니다.

| # | 실패 경로 | 대조 결과와 조치 |
| ---: | --- | --- |
| 1 | C10.3.4 병합 뒤 상태가 PR 대기라고 남는가 | PR #125의 리베이스 병합 상태와 계획·내부 계약·상태 대장·로드맵을 대조해 병합으로 맞췄다. |
| 2 | 앞선 C10.3.3 병합 상태가 내부 계약 인덱스에서 오래된 채 남는가 | PR #122 병합을 확인하고 내부 계약 0052 행을 갱신했다. |
| 3 | C10 부모 작업이 하위 Flex slice 일부만 끝났는데 완료로 바뀌는가 | C10 parent와 C10.3.5·C15 연결은 미완료로 유지된다. |
| 4 | 계획 추가만으로 C12 기능을 완료로 오인하는가 | C12 parent와 C12.1–C12.5 체크를 모두 미완료로 유지하고 문서에 구현 미완료를 적었다. |
| 5 | C12.1 계획 검토가 실제 static/relative 동작 구현처럼 표시되는가 | 상태 행에 계획 기록만 연결하며 구현 완료 표시나 runtime evidence를 추가하지 않았다. |
| 6 | 계획 실패 검토와 PR 변경 검토를 한 산출물로 합쳐 기능 검토를 생략하는가 | 기존 계획 검토와 별도의 PR 변경 검토 문서를 만들고 각각 내부 인덱스에 연결했다. |
| 7 | C10.3.5가 C12 전체를 기다리면서 C12.4와 순환 의존을 만드는가 | C10.3.5는 C12.2 뒤에, C12.4는 C10.3.5 뒤에 둬 의존 방향을 선형으로 정리했다. |
| 8 | C10.3.5의 구현 범위가 C12의 일부로 중복 소유되는가 | Flex static-position·`order: 0` paint 결합을 C10.3.5 전용 별도 PR로 분리했다. |
| 9 | C09.4 absolute shrink-to-fit을 C12가 완료 처리하는가 | C09.4는 별도 항목으로 남기고 C14·C15·C26 선행 조건을 유지했다. |
| 10 | C12 전체 완료 조건이 단계 목록보다 좁아지는가 | C12.1–C12.5와 C10.3.5 교차 계약이 모두 끝나야 parent를 닫도록 했다. |
| 11 | 계획이 Taffy `Position`을 CSS position의 전체 구현으로 취급하는가 | Taffy 0.14.0의 실제 enum/default 한계를 기록하고 CSS typed adapter 소유를 분리했다. |
| 12 | 구현자가 CSS 원문을 별도 파싱해 Stylo cascade 결과를 덮는가 | Stylo computed typed value를 입력으로 쓰고 shorthand·longhand cascade를 비교하도록 했다. |
| 13 | containing block을 찾기 위해 HostDocument 자식 그래프를 재부모화하는가 | source parent, formatting parent, containing-block owner, static-position owner와 paint traversal을 분리했다. |
| 14 | layout subtree로 제한된 요청이 실제 ancestor owner를 놓치고 viewport로 대체하는가 | owner style·edge·revision snapshot을 요구하고 누락 시 명시 오류로 처리한다. |
| 15 | Android safe area나 GPU texture 원점이 CSS containing block으로 섞이는가 | 앱 initial containing block을 `EnvironmentRevision`의 CSS viewport로 정하고 safe area·물리 pixel과 분리했다. |
| 16 | Block absolute의 `auto` inset이 Flex static-position 규칙과 혼동되는가 | Block hypothetical box와 Flex static-position rectangle을 각각 다른 단계·fixture로 분리했다. |
| 17 | absolute child의 `order` 정렬이 HostDocument source order를 파괴하는가 | layout child projection과 paint rank를 분리하고 `order: 0`은 Flex 교차 단계에 한정했다. |
| 18 | intrinsic text/replaced sizing이 아직 없는데 auto-size abs box가 0 크기로 성공하는가 | 필요한 C14/C15 측정이 없으면 진단 오류로 전파하도록 계획에 명시했다. |
| 19 | sticky가 실제 scrollport/clip 없이 viewport offset으로 먼저 구현되는가 | C13 scroll·offset·clip 계약 전에는 C12.5와 완료 주장을 차단했다. |
| 20 | 시뮬레이터 smoke나 적은 case가 모바일 전체 적합성으로 과장되는가 | 단계별 지원 fixture를 Android 연결 실기기와 iOS Simulator에서 동일하게 실행하며 미실행 플랫폼·범위는 별도로 남기도록 했다. |
| 21 | PR 본문 안의 상대 링크가 GitHub에서 잘못된 경로로 해석되는가 | 계획·상태·검토 근거 링크를 현재 PR 브랜치를 가리키는 절대 GitHub 경로로 바꿨다. |
| 22 | 점이 든 task ID를 객체 키에 인용하지 않아 inline script 전체가 실행되지 않는가 | 브라우저에서 칸반이 비어 있는 현상을 재현했고 `C10.3.5` 키를 인용했다. `node --check`를 통과하고 실제 Tailnet 보드에서 C12.1 ACTIVE, C12.2 NEXT와 공식 상태 동기화를 확인했다. |

## 이 PR에 포함하지 않은 결과

- C12 코드는 변경하지 않았고, Chromium 기준 fixture·WPT 실행·Android/iOS 앱 실행도 수행하지 않았다.
- 이 PR은 구현 계획만 추가한다. 호출 가능한 API가 없으므로 API/인터페이스 명세를 만들지 않으며, 각 구현 단계에서 내부 계약 ID·예제·오류 경계를 코드보다 먼저 추가한다.
- 공개 문서 사이트 배포는 요청 범위가 아니며 수행하지 않는다. 공개 로드맵 원본은 갱신했고, 별도 Tailscale 미리보기는 PR 내용이 반영된 뒤 상태를 확인한다.
