# C12.3 계획 PR 변경 검토

이 검토는 C12.3 계획 문서와 상태·인덱스·로드맵·Tailnet preview를 한 변경으로 묶을 때 누락되거나 서로 다른 상태를 게시하지 않는지 살핀다. 계획 자체의 CSS·layout 검토는 [별도 기록](./c12-3-fixed-positioning-plan-review-2026-10-11.md)에 두며 여기서 다시 세지 않는다.

| # | 변경 실패 관점 | 대조 결과 |
|---:|---|---|
| 1 | 계획을 만든 뒤에도 `plan/c12-positioning.md`가 아직 “계획 작성 예정”이라고 남는가 | C12.3 전용 계획·검토 완료, 다음 산출물은 Chrome 사전 비교라고 갱신했다. |
| 2 | 공식 상태 대장이 계획만 끝난 C12.3을 구현 완료로 체크하는가 | C12.3와 상위 C12는 미완료 체크를 유지한다. |
| 3 | C12 상위 완료와 C12.4/C12.5 잔여가 사라지는가 | stacking과 sticky를 남은 단계로 그대로 표기한다. |
| 4 | C12.3 전용 계획이 상태 대장에서 발견되지 않는가 | C12.3 행이 전용 계획과 별도 계획 검토를 직접 링크한다. |
| 5 | 내부 evidence index가 계획 검토를 누락하는가 | `spec/internal/README.md`에 C12.3 계획 검토 파일을 추가했다. |
| 6 | 별도 계획 검토와 PR 변경 검토가 서로 같은 문서로 합쳐지는가 | 두 기록을 별도 파일·별도 관점으로 관리한다. |
| 7 | 계획 URL이 RSPress 실제 산출물로 존재하지 않는가 | custom build가 `plan/c12-3-fixed-positioning.html`을 만들고 원격 HTTP 200을 반환했다. |
| 8 | 로드맵 HTML은 이전 단계 문구나 링크를 남기는가 | 로드맵을 계획 완료·사전 비교 다음으로 바꾸고 전용 계획 링크를 넣었다. |
| 9 | markdown 계획의 상대 링크가 존재하지 않는 파일을 가리키는가 | C12.2 계약, C10.3.5 계약, C04.10 resize evidence 경로를 확인하고 C04.10 링크를 실제 파일명에 맞췄다. |
| 10 | spec 내부 index 링크가 존재하지 않는 review 문서를 가리키는가 | 생성한 evidence 파일과 RSPress HTML route가 모두 존재한다. |
| 11 | 공개 API가 아닌 계획을 내부 API 계약처럼 등록하거나 문서 ID를 미리 소비하는가 | 구현 전이므로 계약 문서를 만들지 않았다. 계획에는 구현 때 사용할 다음 내부 ID만 예고했다. |
| 12 | 내부 버전이 계획 갱신만으로 올라가는가 | 숫자 버전은 변경하지 않고 `0.1.0` 원칙을 유지한다. |
| 13 | planned reference 조건을 이미 실행한 결과처럼 보이게 하는가 | Chrome fixture와 WPT 실행을 앞으로의 게이트로 표시한다. WPT suite 실행을 주장하지 않는다. |
| 14 | 계획 검토를 Android/iOS 구현 증거로 읽게 되는가 | preview build와 검토 문서 모두 계획 산출물이며 런타임 검증이 아니라고 구분한다. |
| 15 | 실기기 우선 원칙이 시뮬레이터 결과로 대체되는가 | 구현 후 Android 실기기, 이후 iOS Simulator를 같은 fixture로 검증하도록 쓴다. |
| 16 | 화면 캡처만으로 geometry·scroll·GPU 성능을 주장하는가 | 캡처와 정량 geometry·scroll·hardware GPU 판정의 경계를 계획에 적었다. |
| 17 | GitHub Pages를 실수로 배포하는가 | GitHub Pages action은 실행하지 않았다. 별도 Tailnet preview만 갱신했다. |
| 18 | Tailnet이 구 source/build를 계속 보여주는가 | 새 preview에서 roadmap SHA-256이 repo 파일과 일치하고 plan·review URL이 HTTP 200으로 열렸다. |
| 19 | browser가 실제 사용자 경로 대신 로컬 파일만 확인하는가 | agent-browser에서 Tailnet roadmap와 C12.3 계획 route를 열어 새 문구·본문·링크를 확인했다. |
| 20 | 이번 계획 PR에 C07.2 추가 물리 증거나 spike를 섞어 보존하지 못하는가 | C07.2 미추적 evidence와 두 spike directory는 staging 범위에서 제외하고 그대로 둔다. |

## 조치와 남은 관문

- 상태 대장·상위 계획·내부 index·공개 roadmap을 C12.3 계획의 현재 단계에 맞췄다.
- C04.10 resize evidence의 파일명 오류를 수정했다.
- Tailnet preview에서 계획 문서와 검토 문서가 실제 링크로 열린다. GitHub Pages는 배포하지 않았다.
- 다음 구현 전 관문은 C12.3 고정 Chrome HTML/inventory/reference와 hash를 만들고 별도 사전 비교를 확정하는 것이다.
