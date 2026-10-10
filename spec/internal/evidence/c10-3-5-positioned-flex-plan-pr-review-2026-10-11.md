# C10.3.5 계획 PR 변경 검토

**검토 범위:** C10.3.5 전용 계획, 계획 실패 검토, C10/C12 상태 대장, 내부 계약 인덱스, Tailnet 로드맵 원본과 생성 결과 · **성격:** 문서 PR의 동기화·경계 검토. 기능 구현이나 제품 적합성 증거가 아니다.

| # | 실패 경로 | 대조 결과와 조치 |
| ---: | --- | --- |
| 1 | C12.2 병합 후에도 공식 상태가 PR 검토 중으로 남는가 | GitHub PR #130의 `MERGED` 상태와 대조해 `spec/STATUS.md`의 병합 상태를 바로잡았다. |
| 2 | 내부 계약 0055가 완료 상태를 과장하는가 | 제한 Block/LTR 범위와 공개 CSS 전체가 미완료라는 경계를 유지하고 PR #130 병합만 반영했다. |
| 3 | 내부 계약 인덱스와 0055 본문이 서로 다른가 | 두 위치의 상태 링크와 제한 구현 범위를 맞췄다. |
| 4 | C12 계획이 이전 PR 상태나 잘못된 다음 단계를 가리키는가 | C12.2 병합을 표시하고 C10.3.5 전용 계획을 연결했다. |
| 5 | C10.3.1–C10.3.4의 기존 병합 기록이 새 계획 때문에 덮이는가 | 기존 PR 번호와 실행 경계를 보존하고 C10.3.5만 별도 행으로 추가했다. |
| 6 | C10.3 parent가 C10.3.5 계획 추가만으로 완료되는가 | C10.3과 C15 text baseline 연결은 계속 미완료로 뒀다. |
| 7 | C10.3.5 상태가 구현 완료로 체크되는가 | 체크를 해제하고 계획/계획 검토와 기능 미구현을 명시했다. |
| 8 | C12 parent가 C12.2 하나의 병합으로 완료되는가 | fixed, sticky, stacking 하위 단계가 남아 C12 parent 미완료를 유지했다. |
| 9 | 계획 링크가 PR 변경 파일이 아니라 오래된 공용 계획을 가리키는가 | C10.3.5 전용 파일을 로컬 계획 인덱스와 공식 상태에서 직접 연결했다. |
| 10 | 로드맵의 현재 진행 안내가 계획과 충돌하는가 | C12.2 실행 완료와 C10.3.5 계획 검토를 구분해 요약했다. |
| 11 | 칸반 보드가 병합된 C12.2를 계속 ACTIVE로 표시하는가 | `execution.inProgress`를 C10.3.5로 이동하고 C12.2 대기 조건을 제거했다. |
| 12 | 로드맵 HTML만 갱신하고 내부 상태 문서는 오래된 상태로 두는가 | 상태 대장, C10/C12 계획, 0055, 내부 인덱스와 로드맵 source를 같은 변경으로 맞췄다. |
| 13 | Tailnet 미리보기 입력 소스가 원본과 다른 오래된 사본인가 | PR 브랜치 파일을 기존 preview source의 대응 경로에 복사하고 빌드 결과를 다시 생성했다. |
| 14 | 빌드 결과가 아닌 원본 파일만 확인해 Tailnet이 낡은 페이지를 내보내는가 | local preview와 외부 route가 새 병합 문구와 C10.3.5 링크를 응답하는지 확인했다. |
| 15 | 새 계획 링크가 Tailnet에서 404가 되는가 | 새 Markdown 경로를 같은 RSPress preview root에 넣어 생성된 계획 HTML이 200으로 응답함을 확인했다. |
| 16 | WPT 경로 존재 확인을 suite 통과로 과장하는가 | 고정 WPT revision에서 후보 raw 경로가 응답하는지만 확인하고 suite는 미실행으로 유지했다. |
| 17 | CSS 범위·오차 기준이 비교 중 바뀌거나 계획에 빠지는가 | C10.3 기준의 Chrome 154, viewport, DPR와 node별 오차 기준을 전용 계획에 복사하고 변경하지 않았다. |
| 18 | 미실행 Android/iOS 기능 실행을 PR 증거로 가장하는가 | PR 범위가 계획·문서 동기화임을 명시하고, 실제 V8 경로·Android 실기기·iOS Simulator 실행을 구현 gate로 남겼다. |
| 19 | GitHub Pages 배포나 RSPress 버전 변경으로 범위가 새는가 | 로컬/Tailnet preview만 재빌드했고 Pages workflow, RSPress dependency, 버전 계약은 변경하지 않았다. |
| 20 | 잘못된 label·PR 본문 외부 링크·임시 미리보기 상태가 PR에 남는가 | 저장소에 실제 존재하는 `planning`, `area: layout`, `area: docs` 라벨을 지정하고 PR 본문을 한국어로 정리했으며 Tailnet URL은 넣지 않았다. |

## 확인된 실행 경계

- PR #130은 `MERGED`이며 C12.2 제한 구현 상태를 반영했다. WPT suite·RTL·Flex/Grid static-position·intrinsic sizing은 여전히 미검증 또는 미지원이다.
- RSPress `2.0.22` preview build는 성공했고 local/external preview에서 업데이트된 로드맵과 신규 계획 페이지를 확인했다. 이는 GitHub Pages 배포가 아니다.
- Rust·CSS reference test, V8 앱, Android 실기기, iOS Simulator는 이 문서 PR에서 실행하지 않았다.
