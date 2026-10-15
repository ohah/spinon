# C13.3 Tailnet roadmap sync PR 변경 검토 30개

검토 대상은 상태 대장 한 항목, 정적 Tailnet 로드맵 note, 이 동기화 설명과 review evidence다. 이번 PR은 코드나 실행 결과를 추가하지 않는다.

| # | 변경 실패 관점 | 판정 |
|---:|---|---|
| 1 | 공식 상태 기록이 빠지는가 | `spec/STATUS.md`를 수정한다. |
| 2 | 정적 Tailnet 홈은 과거 상태를 유지하는가 | `spec/public/roadmap.html`의 C13.3 영역도 같이 갱신한다. |
| 3 | PR #190 근거와 PR #191 문서 변경을 혼동하는가 | README는 runtime result가 PR #190에서 왔고 이 변경은 표시 동기화라고 분리한다. |
| 4 | iOS 결과를 Android 실제 입력으로 일반화하는가 | Android 실제 multi-touch를 잔여 gate로 보존한다. |
| 5 | iOS simulator 결과를 실기기로 일반화하는가 | 기기명과 Simulator 구분을 함께 쓴다. |
| 6 | fixture route의 한정이 사라지는가 | opt-in fixture 한정을 명시한다. |
| 7 | product event 지원이 끝난 것처럼 보이는가 | public DOM events/CSSOM 미완료를 유지한다. |
| 8 | wheel이나 trackpad를 완료한 것처럼 보이는가 | wheel/trackpad 미완료를 유지한다. |
| 9 | displayed frame targeting 증거로 과장되는가 | S04.10 잔여 gate를 유지한다. |
| 10 | C13.3 전체가 완료로 바뀌는가 | 상위 C13.3 미완료 문구와 checkbox 상태를 보존한다. |
| 11 | second touch cancel의 causal sequence가 빠지는가 | active pan→second touch→cancel 순서를 짧게 적는다. |
| 12 | suppressed movement 개수가 evidence와 다른가 | 기존 근거의 callback 네 건과 일치시킨다. |
| 13 | all-up 이전에 recovery했다고 잘못 쓰는가 | all-up 뒤 새 gesture라는 순서를 보존한다. |
| 14 | pixel comparison 범위가 전체 screenshot으로 넓어지는가 | canvas crop으로 명시한다. |
| 15 | recovery pixel 수를 다른 장면과 혼합하는가 | marker 변화 1,080 pixels 및 fresh gesture를 함께 연결한다. |
| 16 | relative route에서 README 링크가 빠지는가 | generated `README.html` route를 Tailnet 경로로 쓴다. |
| 17 | screenshot file이 PR #190 evidence와 다르게 선택되는가 | `ios-after-recovery.png` 경로를 직접 사용한다. |
| 18 | canonical PR 링크가 누락되는가 | public roadmap과 STATUS에 PR #190 링크를 둔다. |
| 19 | STATUS의 relative path가 `spec/` 기준에서 틀리는가 | `internal/evidence/...` 경로를 STATUS 위치 기준으로 쓴다. |
| 20 | roadmap 정적 link에 base path가 빠지는가 | `/spinon/` prefix를 사용한다. |
| 21 | HTML에서 text markup을 깨뜨리는 중첩 block element를 넣는가 | 기존 inline note 안에서 `strong`와 `a`만 사용한다. |
| 22 | 정적 HTML 변경이 C11 섹션 위치를 잘못 옮기는가 | 고유 `2026-10-13 C11.2.1 Android` marker 직전만 삽입한다. |
| 23 | 기존 C13.3 simulator matrix summary가 삭제되는가 | 기존 PR #185/#187/#188/#189 notes를 유지한다. |
| 24 | stale iOS gate가 다른 unrelated prose에 남는가 | 정적 화면의 두 C13.3 stale occurrence를 찾아 교체한다. |
| 25 | implementation in-flight 내용을 merge 완료로 잘못 표시하는가 | 이 PR은 existing iOS proof만 반영하며 다른 gates는 건드리지 않는다. |
| 26 | new evidence directory가 Tailnet sync에 빠지는가 | README와 화면이 새 evidence 디렉터리의 경로를 가리킨다. |
| 27 | PR 본문 이미지가 잘못된 commit SHA를 가리키는가 | PR #190 이미지 URL은 merge SHA pinned 상태로 Agent Browser에서 확인했다. |
| 28 | Tailnet source와 rendered output의 revision이 달라지는가 | sync 전에 main merge SHA와 build source tree를 대조한다. |
| 29 | Tailnet 서버를 재시작하거나 서비스 설정을 바꾸는가 | 기존 preview server root만 파일 sync하며 launch agent 설정은 바꾸지 않는다. |
| 30 | stale browser cache를 최신 증거로 오인하는가 | post-sync는 Agent Browser 재탐색과 HTTP/hash 확인을 요구한다. |

문서 주장은 PR #190 merge source와 simulator evidence만 참조한다. Tailnet 제공 확인은 merge 후 manifest로 별도 기록한다.
