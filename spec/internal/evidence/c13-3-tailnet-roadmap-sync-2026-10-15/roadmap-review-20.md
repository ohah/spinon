# C13.3 정적 로드맵 문서 검토 20개

범위는 `spec/public/roadmap.html`과 `spec/STATUS.md`의 iOS second-touch 상태 정합성이다. Runtime source는 변경하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 이전 iOS multi-touch 미검증 문구가 남아 현재 상태와 충돌하는가 | 상태 대장의 과거 요약을 PR #190 Simulator 결과로 교체한다. |
| 2 | Android synthetic pointer probe가 실제 multi-touch로 승격되는가 | Android 실제 multi-touch를 남은 gate로 명시한다. |
| 3 | iOS Simulator가 실기기로 표시되는가 | iPhone 17 Pro/iOS 26.2 Simulator라고 표기한다. |
| 4 | 한 Simulator fixture를 제품 경로 전체로 일반화하는가 | opt-in fixed fixture 범위를 목적과 판정 범위에서 유지한다. |
| 5 | iOS multi-touch를 전 플랫폼 gate 완료로 오인하는가 | Android actual multi-touch와 상위 C13.3 미완료를 계속 표시한다. |
| 6 | second touch가 pan 시작 전 들어온 것처럼 기술되는가 | 요약은 active pan 진행 중 추가 touch가 들어온 경로를 가리킨다. |
| 7 | cancel reason이 불명확하거나 잘못 바뀌는가 | `SecondPointer` 이름과 PR #190 evidence를 연결한다. |
| 8 | 남은 contact의 추가 movement가 처리되는 것처럼 읽히는가 | 네 callback 억제와 all-up 전 suppression을 기록한다. |
| 9 | all-up 이후 회복이 확인되지 않은 채 주장되는가 | fresh gesture recovery를 로그 근거에 연결한다. |
| 10 | pixel 결과가 전체 화면 불변을 뜻하는 것처럼 쓰이는가 | 0 RGB 차이는 canvas crop 구간으로 제한한다. |
| 11 | 1,080 pixel 변경의 원인이 설명되지 않는가 | fresh gesture 뒤 marker 변화로 한정한다. |
| 12 | evidence README 경로가 정적 Tailnet route와 맞지 않는가 | `/spinon/spec/internal/evidence/.../README.html` Rspress route로 연결한다. |
| 13 | recovery 이미지 경로가 이전 PR의 다른 캡처로 이어지는가 | 새 C13.3 evidence 디렉터리의 `ios-after-recovery.png`를 가리킨다. |
| 14 | PR 링크가 존재하지 않거나 branch 전용으로 남는가 | 이미 병합된 PR #190의 canonical GitHub URL을 사용한다. |
| 15 | 정적 HTML 안에 중첩된 문단 같은 잘못된 마크업이 생기는가 | 새 요약은 기존 `kanban-note` 단락 안에 inline strong/link 요소로 추가한다. |
| 16 | HTML tag가 닫히지 않아 이후 C11 note까지 손상되는가 | 변경은 고유 C11 marker 직전에 삽입하며 링크/strong tag를 짝지어 확인한다. |
| 17 | 새 내용이 CSS/JS 또는 렌더 동작을 변경하는가 | 추가는 텍스트와 링크만이며 script/style는 수정하지 않는다. |
| 18 | C13.3 전체 완료 checkbox나 상태가 변경되는가 | 전체 C13.3 미완료 표시는 그대로 둔다. |
| 19 | iOS second-touch 근거가 road map만 있고 공식 STATUS에서 사라지는가 | 공식 상태 bullet와 정적 화면 양쪽에 PR/evidence를 연결한다. |
| 20 | Tailnet의 기존 파일을 저장소 결과 없이 직접 고치는가 | source 변경을 먼저 병합하고 Rspress output을 생성해 sync할 계획이다. |

두 기존 iOS pending 표현과 새 문단을 현재 PR #190 evidence 및 남은 C13.3 gate와 대조했다. 링크 제공/모바일 UI 판정은 Tailnet sync 후 Agent Browser에서 별도 수행한다.
