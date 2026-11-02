# 로드맵 상태 집계 identity 구현 적대 검토 · 20개

**대상:** `spec/public/roadmap.html`의 official status parser와 렌더링 변경. 계획 검토 20개와 구분해 현재 구현·Agent Browser 실행을 대조했다. 실패 재현은 [negative-agent-browser.json](./negative-agent-browser.json)에 보존했다.

| # | 실패 관점 | 검토와 판정 |
|---:|---|---|
| 1 | 비공식 상태 원본을 읽음 | 고정 GitHub `main`의 `spec/STATUS.md`만 읽고 체크 상태는 읽기 전용이다. 통과. |
| 2 | base task ID 단독 key로 alias가 덮임 | base key는 ID와 정규화 제목이며, unique official ID만 요약 제목을 허용한다. 통과. |
| 3 | 중복 base identity가 조용히 합쳐짐 | `baseTaskKeys.size`와 row 수를 비교해 중복이면 전체 sync가 실패한다. 통과. |
| 4 | 반복 official ID 상태가 마지막 값으로 합쳐짐 | `C04.12` 7개가 ID+제목 key 7개로 분리되고 원본 체크 상태 5/2가 보존된다. 통과. |
| 5 | 반복 ID에서 제목이 없는/여러 base 대응을 임의 선택 | 동일 ID 후보는 정규화 제목이 하나일 때만 연결한다. 0개·2개 이상은 fail-closed. 통과. |
| 6 | 동일 ID·정규화 제목 행 중복 | parser가 `statuses.has(task.key)`를 검사해 두 번째 행에서 중단한다. Agent Browser negative case 통과. |
| 7 | unique ID의 요약 제목 alias를 거부 | 후보 ID가 하나일 때만 status를 base identity에 연결한다. existing phase summary와 원본 short title이 달라도 상태가 연결된다. 통과. |
| 8 | alphabetic suffix를 놓침 | `C06.6a`/`C06.6b`가 별도 stable rows로 나타나며 checked/unchecked가 각각 유지된다. 통과. |
| 9 | 숫자 뒤 임의 문자를 정상 suffix로 허용 | stable-row marker count와 parsed count가 다르면 전체 sync가 실패한다. 문법이 손상된 suffix negative case 통과. |
| 10 | 유효 ID task 행의 제목이 parser에서 잘림 | 코드·제목을 분리하고 Markdown code/link를 같은 `markdownText` 규칙으로 정규화한다. C04.12 title별 비교 통과. |
| 11 | malformed stable ID row가 상세 목록에서 조용히 누락 | ID형 checkbox marker 수와 parsed stable row 수를 대조한다. suffix 손상 case가 fail-closed. 통과. |
| 12 | malformed special no-ID child가 조용히 누락 | 특수 child marker 수와 parsed no-ID row 수를 대조한다. 구분점 손상 case가 fail-closed. 통과. |
| 13 | 두 special child가 같은 DOM identity를 공유 | parent별 source-order ordinal을 key에 넣어 각각 구분한다. valid synthetic fixture에서 215개 unique DOM IDs. 통과. |
| 14 | 일반 설명용 체크리스트를 task로 오인 | parser는 두 no-ID child 문구와 ID 형식만 읽고 R/S/C02 설명형 체크 행은 추가하지 않는다. 표시 count는 215. 통과. |
| 15 | 공식 source가 정상적으로 행을 제거해도 고정 denominator가 남음 | denominator는 실제 `boardTasks.length`에서 계산한다. `C11` detail 제거 control은 214로 자연스럽게 갱신된다. 통과. |
| 16 | 실제 static base row 누락이 성공으로 처리 | `R01` base identity 누락 case에서 sync가 실패하고 누락 코드를 표시한다. Agent Browser negative case 통과. |
| 17 | 완료 수를 서로 다른 ID 수로 계산 | repeated ID 포함 각 board row의 상태를 합산한다. 정상 fixture는 78/215다. 통과. |
| 18 | Open/search filter가 완료 총계를 바꿈 | visible count는 표시 행만 반영하고 완료 총계·분모는 전체 official board 기준을 유지한다. 기존 Agent Browser filter/search 결과 통과. |
| 19 | detail ID가 summary kanban state를 오염 | kanban 진행/대기는 실행 목록의 base code만 참조하고 상세 행은 status summary/count에만 들어간다. 통과. |
| 20 | 실패 응답·모바일 clipping·접근성 상태 혼동 | fetch/catch는 판정 불가 문구를 보여 주며, disabled checkbox/aria-label은 공식 상태를 담는다. desktop/mobile overflow 0px, mobile title은 두 단어 묶음으로 줄바꿈됐다. 통과. |

## 발견 및 수정

초기 구현은 base identity 누락은 잡았지만, malformed special no-ID child marker가 파싱되지 않는 경우를 표시 count 감소로만 넘길 수 있었다. marker 수와 파싱 수 parity check를 추가한 뒤 Agent Browser negative case로 sync failure를 확인했다. 실제 static base는 `R/S/U/E/X/D` 항목이므로 첫 결함 재현 때 detail인 `C11`을 제거한 입력이 214행으로 성공한 것은 source-of-truth 갱신 동작이었다. 누락 base 경계는 실제 base `R01`로 재검증했다.

**판정:** 20개 관점을 현재 코드와 실행 결과로 확인했다. 발견한 parser omission은 수정됐고 미해결 결함은 없다. Tailnet 배포는 PR 병합 뒤 별도 확인한다.
