# One-fit mixed minimum 계획 PR · 변경분 실패 관점 30개 검토

**대상 변경:** 현재 plan PR의 one-fit mixed-minimum 계획·계획 검토·Chrome precomparison 자료·고정 fixture·공식 상태 기록·public roadmap. 구현 코드는 포함하지 않는다. 이 검토는 계획 자체의 20개 실패 관점 검토와 별도다.

| # | Diff 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 의도하지 않은 runtime/layout/FFI source가 섞임 | Stage된 목록은 계획·증거·fixture·상태·로드맵 파일만 포함하며 Rust/FFI source 변경은 없다. |
| 2 | 다른 세션의 tracked WIP가 stage됨 | 기존 tracked 파일 중 수정된 것은 `plan/c11-4-grid-track-sizing.md`, `spec/STATUS.md`, `spec/public/roadmap.html`뿐이다. 추가된 계획·증거·fixture 파일은 현재 후보에 한정되고 다른 기존 untracked 경로는 stage하지 않았다. |
| 3 | 기존 untracked evidence가 통째로 stage됨 | `git add -A`를 사용하지 않고 새 one-fit evidence 디렉터리와 단일 fixture를 정확한 경로로 지정했다. |
| 4 | 이전 날짜/기능의 증거가 덮어써짐 | 기존 uniform precomparison의 관측 파일은 읽기 전용으로 참조하고 새 자료는 `2026-11-14` one-fit mixed 전용 경로에 둔다. |
| 5 | 기준 branch와 현재 checkout이 다름 | 작업 branch는 기존 main에서 만든 단일 checkout이며 추가 worktree나 병렬 code branch를 만들지 않았다. |
| 6 | 공식 구현 완료 수가 계획 선택으로 증가 | 상태 문구와 plan은 공식 `78/214`를 명시적으로 유지한다. |
| 7 | C11.4/C11 상위 항목이 완료로 표시 | 공식 `[ ]` 상태를 유지하고 선택 계획은 구현 완료가 아니라 다음 후보로만 기록했다. |
| 8 | 내부 계약 버전이 변경 | Internal numeric contract `0.1.0`을 유지하며 공개 API/FFI 변경을 주장하지 않는다. |
| 9 | 최근 병합 slice가 오래된 #1375로 남음 | parent plan과 roadmap은 #1378 three-fit을 최근 병합으로 두고 그 다음 후보로 one-fit을 기록한다. |
| 10 | 다음 후보가 계획 인덱스와 공식 status에서 다름 | 두 문서가 동일한 one-fit row-span mixed `min-height:0|auto` 계획과 같은 증거 README를 가리킨다. |
| 11 | public roadmap의 칸반에 선택 후보가 없음 | `detailNext` 카드에 C11.4 one-fit mixed plan을 넣고 Agent Browser snapshot/screenshot에서 `NEXT` 카드를 확인했다. history 문장에도 제품 구현 미시작을 표시한다. |
| 12 | 칸반 카드가 공식 completion count를 바꿈 | 카드는 미착수 상세 후보로만 증가하며 공식 완료 수를 계산하거나 대장 체크를 바꾸지 않는다. |
| 13 | plan과 plan-review 링크가 존재하지 않는 파일을 가리킴 | 상대 링크의 파일 경로를 checkout에서 확인했고 Markdown 문서 링크로 유지한다. |
| 14 | fixture HTML이 RSPress page link로 해석됨 | Fixture 파일은 본문 코드 경로로 표기해 `.html.html` dead route를 만들지 않도록 했다. |
| 15 | JSON/JS/Python/gzip를 RSPress Markdown page로 링크 | machine-readable 자료와 스크립트는 코드 경로로 표기하고 Markdown link checker의 dead route 후보에서 제외했다. |
| 16 | 사전 비교 README가 plan과 다른 scope를 서술 | 두 문서는 one-fit positions 1–6, 120,960 Chrome scenes와 Spinon 구현 미검증 경계를 일치시킨다. |
| 17 | Chrome 기준을 Spinon 동작으로 표시 | 계획·README·화면 설명 모두 Chrome oracle/precomparison으로 표현하며 제품 RuntimeSession 결과로 승격하지 않는다. |
| 18 | screenshot이 앱 동작 화면으로 잘못 소개됨 | PR 이미지 caption을 Chrome 157 oracle representative sample로 제한하고 Spinon 앱 화면이나 pixel parity가 아님을 표시한다. |
| 19 | matrix denominator가 부분 sample과 혼동 | README의 Chrome 전체 120,960 scenes, 900 unique lists, 14,400 uniform endpoints를 서로 다른 분모로 적었다. |
| 20 | overflow boundary가 검증됐다고 과장 | 고정 Chrome 입력에서 Grid-end overflow 0 scene인 점을 plan index와 README에서 명시하고 overflow fixture 주장에 사용하지 않는다. |
| 21 | source-order/alignment 수치를 잘못 일반화 | 60,480 source-order/alignment pairs와 non-auto stretch 0 CSS px는 해당 고정 matrix 결과로 범위를 제한한다. |
| 22 | auto-row stretch 뒤 item y 이동이 non-auto row 변경으로 오판 | 계획은 track 크기와 downstream item 위치/크기를 독립 불변 조건으로 검증한다. |
| 23 | uniform baseline 자료가 혼합되어 기존 구현을 덮음 | 기존 one-fit uniform baseline hash 두 개를 capture manifest에서 고정했고 새 mixed fixture는 별도 파일이다. |
| 24 | Agent Browser와 Terminal Browser를 혼동 | capture README는 Agent Browser/HeadlessChrome 환경을 기재하고 Terminal Browser 사용은 없다고 기록한다. |
| 25 | 이번 Agent Browser 세션이 남음 | lifecycle evidence는 task namespace close를 기록하고, 현재 session list는 비어 있다. |
| 26 | S03/기존 Simulator를 이번 작업 cleanup으로 종료/삭제 | 현재 booted simulator가 없고 이번 plan PR은 시뮬레이터를 부팅하지 않았다. 기존 simulator profiles와 역사적 S03 evidence는 건드리지 않았다. |
| 27 | raw observation·screenshot hash가 manifest와 다름 | `verification.json`과 capture manifest는 fixture, raw observation 및 representative PNG의 SHA-256을 명시한다. |
| 28 | 별도 plan review 20개를 diff review 횟수에 합침 | `plan-review-20.md`는 계획 위험을, 이 파일은 staged 변경 경계를 검토하며 산출물을 나눈다. |
| 29 | RSPress 전체 build 실패를 이 diff의 통과로 보고 | `mise exec -- bun run docs:build`는 저장소의 1,570개 Markdown 문서에서 기존 dead-link 오류로 exit 1이다. 이 후보에 있던 fixture/JSON/script 및 STATUS plan 경로 dead link는 코드 경로로 바꿨고 최종 build 로그에서 후보 경로 오류가 더 나오지 않았다. 전체 문서 build 또는 CI green을 주장하지 않는다. |
| 30 | Public Pages/Tailnet preview를 PR merge 전에 갱신 | PR은 문서 소스만 변경한다. Tailnet sync는 계획 PR merge 후 사용자 지정 미리보기 경로에서 별도 검증한다. GitHub Pages workflow는 실행하지 않는다. |

## 검토 후 판정

각 행은 별도 경로·상태·렌더링 경계를 다루며 계획 검토 20개와 중복해 세지 않는다. `SHA256SUMS`에는 이 evidence directory와 고정 fixture의 hash를 담는다. 계획 PR은 RSPress의 기존 저장소 전체 dead-link 한계와 후보 경로 검사를 구분해 기록한다.
