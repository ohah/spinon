# Tailnet 로드맵 동기화 PR diff 실패 관점 검토 30개

검토 범위는 C11.4 병합 기록 동기화, 공식 상태 대장 문구, 공개 roadmap 렌더링 데이터, Tailnet 교체·백업 script와 browser evidence다. 각 항목은 서로 다른 실패 경로를 확인했다.

| # | 실패 관점 | 판정과 확인 |
|---:|---|---|
| 1 | 최신 feature PR 번호 오류 | PASS — 최신 제한 slice 링크가 #1274다. |
| 2 | feature merge 시각/방식 오류 | PASS — `2026-11-07T22:12:47Z`, Rebase and merge로 기록했다. |
| 3 | feature merge SHA 혼동 | PASS — `b361097257f0fb6a9739de139724bef24abadb04`를 #1274 merge SHA로 구분했다. |
| 4 | PR 상태와 CI 상태 과장 | PASS — status/workflow 결과 부재를 기록하고 CI green이라 하지 않는다. |
| 5 | 두 허용 list order 누락/추가 | PASS — `fit-content min-content auto` 및 `fit-content auto min-content`만 썼다. |
| 6 | finite px cap 비교 누락 | PASS — Chrome 157 사전 비교에 별도 20px cap scene이 있음을 연결했다. |
| 7 | finite percentage cap basis 오해 | PASS — definite 100px block-size의 20%가 20px라는 경계를 유지했다. |
| 8 | visible overflow 장면을 clipping으로 오기술 | PASS — overflow는 독립 비교/실행 근거에서만 가리키고 clipping이라 하지 않는다. |
| 9 | 자동 행 크기 분배 범위 확대 | PASS — positive free space의 auto track 동작만 설명하며 다른 sizing profile은 열지 않는다. |
| 10 | general CSS support 주장 | PASS — 기록을 bounded parsed Root HTML slice로 제한했다. |
| 11 | WPT 전체 적합성 주장 | PASS — WPT pass를 주장하지 않는다. |
| 12 | Android Simulator 증거 유형 혼합 | PASS — `layout=ready boxes=12`와 draw-presented는 기록하되 PixelCopy로 부르지 않는다. |
| 13 | iOS screenshot을 surface readback으로 과장 | PASS — layout-ready와 OS screenshot visual smoke만 기록한다. |
| 14 | iOS data continuity 누락 | PASS — install 전후 container가 달랐고 continuity와 원인은 미확인이라고 기록한다. |
| 15 | 공식 체크를 조기 완료 처리 | PASS — C11.4/C11 체크를 유지하고 78/214를 반복 명시한다. |
| 16 | 내부 계약 버전 증가 | PASS — `0.1.0`을 유지한다. |
| 17 | 계획 문서의 pending 상태 잔존 | PASS — feature plan status를 #1274 merged로 바꿨다. |
| 18 | 내부 계약에 candidate 상태 잔존 | PASS — 제목과 본문을 merged bounded implementation으로 맞췄다. |
| 19 | C11 계획 최신 병합 이력 누락 | PASS — dated #1274 entry와 테스트/한계를 추가했다. |
| 20 | 공식 상태 대장 unchecked task 변형 | PASS — C11.4 이력 문장만 추가했고 checkboxes는 변경하지 않았다. |
| 21 | 공개 roadmap이 #1272에서 멈춤 | PASS — 최신 #1274 history를 위에 추가하고 #1272 과거 기록은 보존했다. |
| 22 | 진행 카드가 실제 active 상태와 다름 | PASS — hardcoded detail card는 계속 C11.4이며 제목만 `Intrinsic sizing`으로 명확히 했다. |
| 23 | 미구현 순열 후보를 지원 기능으로 게시 | PASS — 나머지 member order는 다음 후보로만 표시하고 계획·oracle 선행을 적었다. |
| 24 | 표시 카드가 좁은 화면에서 확인 불가 | PASS — Agent Browser 390px에서 board를 272px 밀어 active card를 확인했다. |
| 25 | 모바일 board scroll을 문서 넘침으로 오판 | PASS — document/body 390px, board 자체 scrollWidth 1120px를 분리했다. |
| 26 | 교체 전 preview 유실 | PASS — 이전 target을 백업하고 old target/backup hash가 동일함을 확인했다. |
| 27 | source·target bytes 불일치 | PASS — source와 target 각각 631,732 bytes, SHA-256 `855dbb7b…3160d4`다. |
| 28 | local/Tailnet 캐시·배포본 불일치 | PASS — 두 endpoint가 HTTP 200이고 source와 byte-identical이다. |
| 29 | Tailnet server 상태/범위 부주의 | PASS — Tailscale Running, 기존 server PID 유지, 단일 `roadmap.html`만 atomic replace했다. |
| 30 | Agent Browser 프로세스 방치 또는 Pages 배포 | PASS — 세션 inactive, active session 및 agent-browser process 없음; Pages/gh-pages 변경은 없다. |

**결과:** 30개 관점 검토에서 blocking defect는 발견되지 않았다. Browser screenshots, sync manifests, server/backup state는 같은 evidence 디렉터리의 원본 기록과 README에 연결했다.
