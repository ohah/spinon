# 진행 칸 가시성 PR 변경 검토 30개

공개 roadmap 구조·responsive layout·칸반 상태 계산·브라우저 제공 경계를 서로 다른 실패 경로로 검토했다. 후보 화면은 Agent Browser desktop/mobile 관측으로 확인했으며, Tailnet 최종 제공 검증은 병합 뒤 별도로 한다.

| # | 실패 경로·불변 조건 | 확인 기준 | 결과 |
|---:|---|---|---|
| 1 | 진행 카드가 DOM에는 있지만 긴 과거 이력 뒤에 묻힐 수 있음 | 이전 Tailnet DOM에서 board y=`46672.66px`; 수정 후 desktop y=`357.02px` | 해결 |
| 2 | 보드가 제목보다 위에 놓여 페이지 맥락을 잃을 수 있음 | page title 다음에 Kanban section이 배치됨 | 통과 |
| 3 | 보드를 첫 grid 열에 가둬 일부 열이 잘릴 수 있음 | `.hero > .kanban-section`이 두 열을 span | 통과 |
| 4 | desktop 너비가 SVG보다 작아 board가 내부 clipping될 수 있음 | desktop frame 1260px, content 1260px | 통과 |
| 5 | 내부 가로 scroll이 document 전체를 가로로 밀 수 있음 | document width=1440px at desktop | 통과 |
| 6 | 모바일에서 긴 SVG의 첫 `할 일` 열만 보여 진행 카드가 숨을 수 있음 | 모바일 `scrollLeft`가 진행 열로 이동하고 카드가 x=74px에 보임 | 해결 |
| 7 | 자동 scroll offset이 SVG 열 배치와 따로 하드코딩될 수 있음 | SVG column placement와 mobile scroll이 같은 `kanbanColumnStep` 사용 | 통과 |
| 8 | 모바일 폭 기준이 CSS breakpoint와 어긋날 수 있음 | CSS와 JS가 모두 600px breakpoint 사용 | 통과 |
| 9 | viewport resize 후에도 첫 column으로 돌아가 active가 숨을 수 있음 | 단일 resize listener가 mobile active column을 다시 노출 | 통과 |
| 10 | render 재호출마다 resize listener가 누적될 수 있음 | listener는 `renderKanban` 바깥에 한 번 등록 | 통과 |
| 11 | desktop에서 모바일 scroll offset이 남을 수 있음 | 넓은 viewport에서는 `scrollLeft=0` | 통과 |
| 12 | 모바일 heading과 legend가 한 줄에서 서로 겹칠 수 있음 | mobile heading은 block flow, legend는 다음 줄 | 통과 |
| 13 | mobile note가 필요한 조작을 알리지 않을 수 있음 | 좌우 swipe로 나머지 열을 볼 수 있다는 문구 추가 | 통과 |
| 14 | `kanban-board` ID가 두 곳에 남아 JS가 엉뚱한 곳을 선택할 수 있음 | section 이동 후 ID는 단일 위치로 유지 | 통과 |
| 15 | `kanban-heading` 접근성 참조가 끊길 수 있음 | section `aria-labelledby`와 heading ID 유지 | 통과 |
| 16 | live status region이 없어 동기화 상태를 읽지 못할 수 있음 | board 및 status note의 기존 `aria-live` 속성 유지 | 통과 |
| 17 | 진행 카드 내용이 상태 데이터와 분리될 수 있음 | 동일 `execution.detailInProgress`에서 SVG card 생성 | 통과 |
| 18 | 계획을 공식 완료 상태로 올릴 수 있음 | `execution`의 세부 카드만 추가하고 공식 ID/status 변경 없음 | 통과 |
| 19 | 문서 이동으로 공식 완료 집계가 바뀔 수 있음 | Agent Browser status note에서 `78/214` 확인 | 통과 |
| 20 | C11.4/C11 완료를 과장할 수 있음 | 공개 상태 및 기존 hero copy 미완료 표기 유지 | 통과 |
| 21 | history 목록이 사라지거나 링크가 잘릴 수 있음 | history nodes는 board 아래에 유지 | 통과 |
| 22 | hero history wrapper가 페이지 section 닫힘을 먹을 수 있음 | DOM에서 board, history, overview가 서로 구분됨 | 통과 |
| 23 | 넓은 viewport hero grid에서 board가 좁은 sidebar에 놓일 수 있음 | desktop board bounds x=90..1350, width=1260 | 통과 |
| 24 | 중간 viewport에서 one-column hero layout이 무너질 수 있음 | `max-width:900px` hero rule은 유지, board grid span은 두 줄에서 유효 | 통과 |
| 25 | 작은 화면 전체 문서에 가로 overflow가 생길 수 있음 | mobile document width=390px, viewport=390px | 통과 |
| 26 | 모바일에 board 내부 scroll이 전혀 없어 active 열에 갈 수 없을 수 있음 | frame clientWidth=362px, contentWidth=1120px, horizontal swipe 가능 | 통과 |
| 27 | screenshot이 다른 viewport나 위치를 증명할 수 있음 | DOM JSON에 viewport·bounds·scrollLeft를 함께 기록 | 통과 |
| 28 | local 후보 화면을 live Tailnet evidence로 오인하거나 PR image URL이 끊길 수 있음 | local candidate와 post-merge Tailnet gate를 분리하고, PR #1258에서 desktop/mobile 두 이미지의 natural dimensions를 Agent Browser로 확인 | 통과 |
| 29 | browser 종료 요청 뒤 Agent Browser가 남을 수 있음 | session 종료 뒤 Agent Browser·HeadlessChrome process scan empty | 통과 |
| 30 | local preview cleanup이 기존 Tailscale preview server를 종료할 수 있음 | 임시 18769 server만 종료; Tailnet PID 28040은 별도 유지 | 통과 |

공식 count, 완료 checkbox, C11 contract version에는 변경이 없다. Tailnet preview와 live browser screenshot은 별도 sync change의 완료 조건이다.
