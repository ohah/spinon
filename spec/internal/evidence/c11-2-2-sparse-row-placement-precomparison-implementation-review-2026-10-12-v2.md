# C11.2.2 Chrome 사전 비교 v4 fixture·capture 도구 재검토 · 2026-10-12

**대상:** versioned HTML/inventory v3, Chrome 154 capture tool v4, reference v4, 16 DPR 1 PNG 및 실행 근거.
**성격:** oracle fixture와 수집 도구 검토다. Spinon runtime, 모바일 surface, 성능, WPT 적합성 검토가 아니다.
**검사:** v4 단독 검사 4/4 통과 · `git diff --check` 통과. 기존 v3 기준과 이전 WPT runner는 이번 재검토에서 재실행하지 않았다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | 새 결과가 기존 v3 reference를 덮어씀 | v3/v4 출력 경로가 다르고 v4 reference는 `wx` 생성한다. 기존 v3는 수정하지 않았다. |
| 2 | 새 장면을 기존 fixture에 덧붙여 이전 해시가 무효화됨 | versioned HTML/inventory v3 및 reference v4를 별도 파일로 두고 source hash를 연결한다. |
| 3 | Chrome 업데이트 뒤 같은 기준을 재사용 | product version `154.0.8037.98`과 revision을 reference 및 검사에서 고정한다. |
| 4 | Chrome 실행파일 교체를 놓침 | 실행 binary 경로와 SHA-256을 reference에 남기고 fixture test가 제품·revision을 확인한다. |
| 5 | capture code 변경 뒤 이전 관측과 혼합 | capture tool SHA-256을 reference에서 현재 파일과 대조한다. |
| 6 | helper 동작 변경 뒤 capture hash만으로 충분하다고 오인 | Chromium session helper의 SHA-256도 별도로 기록하고 검사한다. |
| 7 | HTML 또는 inventory 변경을 놓침 | 양쪽 SHA-256을 reference와 비교하고 HTML의 scene/preorder를 inventory와 대조한다. |
| 8 | 장면 개수는 맞지만 ID 순서나 node 구성이 빠짐 | 16 scene의 순서와 각 scene의 `data-node` preorder를 검사한다. 총 69 node가 수집됐다. |
| 9 | node ID 중복으로 결과가 다른 node에 붙음 | 전역 중복 ID를 거부하고 각 node의 가장 가까운 수집 parent를 검사한다. |
| 10 | Grid root를 item으로 오분류하거나 parent가 잘못됨 | 각 fixture root ID 및 DOM 유래 parent ID를 관측에 대조한다. |
| 11 | explicit와 computed track 수가 같은 값으로 기록됨 | authored px track 수와 computed track list를 따로 저장하고 implicit 수를 차이로 검증한다. |
| 12 | `grid-auto-flow` 직렬화 또는 computed property 누락 | 고정 computed property 목록과 각 관측 키 집합을 검사한다. |
| 13 | DPR 차이가 CSS geometry 차이로 숨겨짐 | DPR 1·2의 CSS rect, computed values, track observations 전체 equality를 검사한다. |
| 14 | `column-cursor-wrap` 장면이 이름만 있고 규칙은 증명하지 않음 | column 2 item `(64,20)`, 뒤따르는 column 1 item `(20,44)`, explicit row 1·computed row 2를 직접 assert한다. |
| 15 | 신규 장면이 Grid content 좌표와 scene-root 좌표를 혼동 | JSON frame은 viewport/scene 기준이고 README 표에서 grid content offset `(20,20)`을 빼 별도 설명한다. |
| 16 | PNG가 누락·변조·잘린 크기임 | scene별 DPR 1 PNG 수, 파일 hash, PNG signature, inventory viewport 크기를 검사한다. |
| 17 | capture 재실행이 과거 screenshot을 덮어씀 | run ID별 새 directory를 만들고 PNG와 reference는 exclusive create를 사용한다. |
| 18 | pinned WPT source hash 검증을 Spinon 실행으로 표현 | 이번 신규 장면은 WPT로 실행하지 않았다. 이전 Chrome WPT 후보 결과도 Chrome oracle로 구분한다. |
| 19 | Chrome 관측을 Spinon runtime/mobile/pixel pass로 확대 | README·계획은 runtime, Android/iOS, GPU surface 및 Spinon 적합성 미실행을 명시한다. |
| 20 | 계획·상태표·Tailnet가 옛 15/66 기준을 계속 보여줌 | 이번 검토 후 `spec/STATUS.md`, internal index, public roadmap와 Tailnet 사본을 16/69 v4로 맞춘다. |

## 발견·남은 경계

이전 기준은 definite-column cursor가 뒤로 이동할 때 row-wrap하는 장면을 포함하지 않았다. 새 장면과 직접 좌표 assertion을 추가했다. v4 구조·hash·DPR·PNG 검사는 통과했다. 이 run에서는 Chrome을 다시 실행하지 않았으므로 캡처 재현성의 신규 실행 결과는 주장하지 않는다. capture가 중간 실패해도 versioned run evidence를 보존하는 방식이며, 이 자료에서 Spinon 적합성이나 모바일 결과를 추론하지 않는다.
