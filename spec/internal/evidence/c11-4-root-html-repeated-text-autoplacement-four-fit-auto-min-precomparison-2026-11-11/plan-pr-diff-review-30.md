# C11.4 네 fit auto-placement 계획 PR diff 적대 검토 30개

**검토 대상:** four-fit 계획, 계획 20관점 검토, Chrome 사전 비교 corpus/verifier, `spec/STATUS.md`, `spec/public/roadmap.html`의 실제 staged PR diff. 계획 검토표·이 파일 자체·해시 목록은 본 검토의 계약 대상에서 제외한다.

| # | 독립 실패 관점 | diff 확인 결과 |
|---:|---|---|
| 1 | 상태 대장 기준일이 실제 작업일보다 오래됨 | `2026-11-11`로 맞췄다. |
| 2 | 계획 추가가 공식 완료 checkbox를 바꿈 | origin/main과 checkbox 수가 같고 `78/214`를 유지한다. |
| 3 | 출시 전 내부 계약 숫자 버전이 오름 | `0.1.0`을 그대로 유지한다. |
| 4 | 가장 최근 merged bounded slice가 오래된 PR로 남음 | Roadmap header를 PR #1342와 `origin/main` merge SHA `9cba169c…`로 맞췄다. |
| 5 | merged three-fit slice가 진행 중 열에 남아 완료·진행을 혼동 | `detailInProgress`를 비우고 next planned candidate 한 건만 둔다. |
| 6 | next card가 완료된 일반 CSS 기능처럼 표시 | 제목·설명에 C11.4 bounded profile과 미완료 상태를 남겼다. |
| 7 | roadmap candidate에서 계획 문서로 이동할 수 없음 | hero note가 plan 및 precomparison evidence를 직접 연결한다. |
| 8 | 공식 대장과 public roadmap의 next task가 다름 | 두 문서가 four-fit auto-placement 후보와 동일한 5,760-scene 근거를 가리킨다. |
| 9 | last merged 구현의 경계가 과장됨 | PR #1342 note에 5,760/5,760, simulator visual-smoke, CI 0개 한계를 유지했다. |
| 10 | 계획을 이미 구현된 것으로 오인 | 계획 상태에 구현 시작 전이라고 명시하고, 계획 merge 뒤에만 제품 코드를 수정하도록 했다. |
| 11 | 4개 fit track 외의 tail multiplicity가 열려 있음 | `min-content`와 `max-content` 각각 하나, 두 순서만 positive다. |
| 12 | `fit-content()` minimum sizing function을 틀리게 인용 | 공식 CSS Grid 2025 CRD §7.2/§6.6 링크와 eligibility 설명을 대조했다. |
| 13 | automatic minimum의 overflow 또는 flexible track 조건을 빠뜨림 | non-scrollable, single-row, no-flex 범위와 unsupported rejection을 기록했다. |
| 14 | 15개 fit position 조합 중 누락 | fixture generator·metadata·verifier가 15개를 모두 확인한다. |
| 15 | 네 cap이 위치와 맞지 않거나 중복 배정 | `[0px, 10px, 20%, 30px]`의 24 순열을 authored index로 기록한다. |
| 16 | percentage cap의 basis가 불명확 | input grid block-size 220px를 definite로 고정했다. |
| 17 | scene denominator가 matrix 차원을 빠뜨림 | 720 lists × 2 alignments × 2 DOM orders × 2 min-height modes = 5,760 scenes다. |
| 18 | baseline/candidate pair가 서로 다른 scene과 짝지어짐 | 2,880 paired keys는 authored list·alignment·DOM order가 같고 min-height만 다르다. |
| 19 | text/font 변화가 line-count oracle을 바꿈 | text payload, 10px monospace/line-height와 `[2,2,2,3,2,2,3,1]`을 고정했다. |
| 20 | row list cycle의 implicit row 7·8을 놓침 | verifier가 8 row occurrence를 six-member list에 modulo 대응한다. |
| 21 | baseline under-floor와 candidate result를 섞어 보고 | baseline fit-row 7,200 under-floor와 candidate 0 violation을 분리 저장한다. |
| 22 | row 안 payload overflow를 Grid block overflow로 표현 | 검토 중 혼동 가능성을 찾아 row overflow 7,200/0과 Grid 끝 초과 0을 별도 행으로 나눴다. |
| 23 | text가 220px Grid 밖으로 나가는데 0 overflow라고 기록 | verifier가 item frame과 payload 끝을 각각 검사하고 0/0을 확인한다. |
| 24 | `align-content:stretch` 결과가 auto tail track 존재를 전제로 해석 | auto member가 없는 이 profile에서 start/stretch row-size equality만 보고한다. |
| 25 | scene JSONL 순서·중복·list count가 달라짐 | 5,760 contiguous index, 720 unique track list, 15/2/24 dimension을 검증했다. |
| 26 | reproduction script가 임시 `/tmp` 경로를 참조 | collector는 자기 폴더의 fixture/output을 기준으로 동작하도록 구성했다. |
| 27 | browser evidence가 Terminal Browser 결과일 수 있음 | collector는 고유 Agent Browser session으로 HeadlessChrome 157을 열고 Terminal Browser 미사용을 기록한다. |
| 28 | 종료 뒤 Agent Browser session/process가 남음 | session list는 빈 배열이고 owned session/daemon selector와 일치하는 프로세스가 없다. |
| 29 | 결과가 app runtime·simulator pixel·WPT로 과장 | README가 RuntimeSession, Android/iOS, readback, compositor, device, WPT의 미검증 범위를 분리했다. |
| 30 | evidence 일부가 바뀌었는데 hash manifest가 오래됨 | 관측 corpus·fixture·collector·verifier·요약·screenshot·README 9개 해시가 모두 일치한다. |

## 판정

실제 변경분에서 발견한 표현 문제는 row 내부 payload overflow와 Grid block-end overflow를 분리해 바로잡았다. 공식 체크 수·C11.4/C11 상태·계약 버전은 그대로이며 계획 PR은 Chrome 관측을 제품 구현 근거로 주장하지 않는다. Rebase and merge 및 지정 Tailnet preview hash 확인은 계획 PR 병합 뒤 수행한다.
