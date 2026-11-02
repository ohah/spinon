# 로드맵 identity/counter 수정 PR diff 적대 검토 · 30개

**대상 diff:** `README.md`, `spec/public/roadmap.html`, 상태·내부 계약·계획 동기화와 이 evidence directory. Review는 현재 PR candidate diff를 기준으로 실행했다.

| # | diff 실패 관점 | 결과 |
|---:|---|---|
| 1 | README 표시 task 수를 잘못 올림 | 상위 78개와 표시 215개를 분리해 썼고 완료 체크 수를 README에 추가하지 않았다. 통과. |
| 2 | parser가 old task code 단독 identity를 남김 | `taskIdentity(code,title)`가 모든 base/stable detail row에 쓰인다. 통과. |
| 3 | Markdown 정규화 결과가 화면 이름과 다름 | 링크/code 표기를 같은 helper로 정규화하며 visible title은 원본에서 만든 text node다. 통과. |
| 4 | `encodeURIComponent` key가 HTML ID에서 충돌 | ID prefix와 normalized title를 포함하고 전체 215개 DOM id distinct를 실행으로 확인했다. 통과. |
| 5 | `<label for>`가 새 checkbox id를 따라가지 않음 | renderer가 새 id를 input과 label 양쪽에 동일 사용한다. 통과. |
| 6 | C04.12 이전 last-write-wins가 남음 | 상태 Map key가 row identity이고 동일 key 재등재는 throw한다. duplicate negative case 통과. |
| 7 | `C06.6a/b` 새 문법이 수용되지 않음 | 선택적인 alphabetic suffix를 읽고 둘 다 표시하는 브라우저 결과를 확인했다. 통과. |
| 8 | suffix regex가 잘못된 trailing code를 조용히 건너뜀 | broad stable-row marker와 parsed count가 달라지면 throw한다. malformed suffix test 통과. |
| 9 | well-formed detail row가 base row로 소비됨 | `coveredOfficialKeys`에는 연결된 base official row만 들어가며 나머지 stable rows는 detail로 렌더된다. 통과. |
| 10 | duplicate ID의 제목 alias를 잘못 고름 | 반복 ID에는 exact normalized title match만 허용한다. 통과. |
| 11 | unique base row title alias로 오연결 | 같은 code 후보가 하나일 때만 alias를 허용하고 code별 candidate를 검사한다. 통과. |
| 12 | 같은 official row가 두 base row에 재사용됨 | covered key 재사용을 throw한다. 통과. |
| 13 | static base row 자체 중복 | `baseTaskKeys.size` 검사에서 실패한다. 통과. |
| 14 | special child malformed punctuation이 누락됨 | marker/parsed child count parity guard가 잡으며 Agent Browser test를 기록했다. 통과. |
| 15 | no-ID child key가 전 페이지에서 중복 | parent별 ordinal key를 보존하고 unique IDs 215개로 확인했다. 통과. |
| 16 | unrelated plain checkbox가 status board에 합류 | generic R/S/C02 checklist를 marker parser에 포함하지 않는다. 통과. |
| 17 | 공식 행 추가/삭제에 count 상수가 남음 | `taskCount`와 completed count는 파싱된 list 길이 및 row 상태로 계산한다. 통과. |
| 18 | 빈/누락 base 대응을 open 처리 | 실제 base R01 누락은 `상위 작업 identity` 오류로 fail-closed. 통과. |
| 19 | invalid data가 0/N 완료로 표시 | 실패 시 판정 불가 안내, 기존 base list 78개, `공식 상태 확인 중` counter가 보인다. 통과. |
| 20 | details append가 DOM order/search에 등록되지 않음 | renderer가 phase list에 append한 뒤 `boardTasks`/search update를 실행한다. 통과. |
| 21 | open filter에서 완료 counter가 표시 행 수로 변질 | status total은 전체 board 기준이며 visible counter만 filter에 따라 달라진다. 통과. |
| 22 | aria label에서 source title/status가 빠짐 | status 반영 후 aria label에 완료/미완료를 갱신한다. 통과. |
| 23 | STATUS 사용자 text가 HTML로 실행됨 | dynamic title/description은 `textContent`에만 넣는다. 통과. |
| 24 | malformed UTF-8/title를 인코딩 경계에서 깨뜨림 | API base64 bytes를 TextDecoder UTF-8로 복원한 뒤 encodeURIComponent 한다. 통과. |
| 25 | GitHub 응답/timeout 시 stale count가 authoritative처럼 보임 | non-OK, payload 오류, abort는 기존 catch 문구와 board error로 간다. 통과. |
| 26 | merged PR card가 비한정 CSS 지원을 주장 | PR #1086 link/merge SHA·limited scope·시뮬레이터 경계·미완료 문구를 포함한다. 통과. |
| 27 | roadmap card의 evidence URL이 repository path와 다름 | 기존 `/spinon/plan/...`·`/spinon/spec/...` preview 경로와 맞춘다. 통과. |
| 28 | 내부 계약이 제한 범위를 확장 주장 | px/% minimum, px/%/fr maximum과 definite basis/fail-closed 범위를 명시하고 WPT/public support는 제외한다. 통과. |
| 29 | STATUS merge entry가 C11 checkbox/공식 count를 승격 | bounded merge history만 추가했고 parent/child checkboxes와 official state source는 그대로다. 통과. |
| 30 | PR screenshot/evidence가 검증 후보와 불일치 | 최종 desktop/mobile Agent Browser PNG와 SHA, synthetic negative JSON을 evidence README에 고정했다. 실제 PR 설명은 체크 rollup이 비어 있음을 명시한다. 통과. |

## 최종 판정

30개 항목에서 미해결 diff 결함은 찾지 못했다. 구현 검토에서 추가한 ID/child marker parity guard를 포함해 정상 215/78 및 fail-closed negative cases를 Agent Browser로 확인했다. Evidence와 화면 SHA는 [README](./README.md)에 있다. Tailnet은 아직 동기화하지 않았으며 PR 병합 후 별도 검증한다.
