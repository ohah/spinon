# C11.3.3 unknown-area auto implicit track 계획 PR diff 적대 검토 30개

**검토 대상:** [작업 계획](../../../plan/c11-3-3-unknown-area-auto-implicit-tracks.md) 및 [계획 실패 관점 검토 20개](./c11-3-3-unknown-area-auto-implicit-tracks-plan-review-20.md)

**검토 범위:** 계획 PR의 실제 문서 diff. 구현 코드, runtime 성공, 제품 지원 또는 별도 구현 PR의 검토로 확대하지 않는다. 계획 실패 관점 검토 20개는 이 PR diff 검토를 대체하지 않는다.

**결과:** PASS 30 · PARTIAL 0 · FAIL 0.

| # | 독립 PR diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획 문서가 작성만으로 기능 구현·완료를 암시한다 | PASS — 첫 문단과 완료 기준이 계획 단계임을 명시하고 C11.3.3/C11 체크를 유지한다. |
| 2 | 기존 expected rejection을 성공 결과로 소급 덮어써 증거 이력이 사라진다 | PASS — source-bound 새 runtime result로 갱신하고 과거 기준 파일 자체는 덮어쓰지 않는다고 적었다. |
| 3 | 전체 로드맵 공식 완료 수를 계획 PR만으로 올린다 | PASS — 공식 `72/213` 유지가 상태와 완료 기준에 적혀 있고 `spec/STATUS.md` 변경은 포함하지 않았다. |
| 4 | 계획 대상 fixture를 식별할 수 없어 다른 unknown-area 사례에 적용된다 | PASS — `unknown-area-implicit-tracks`와 입력 fixture·runtime matrix의 경로를 고정했다. |
| 5 | reference 자료의 stale 또는 오염 여부를 재현할 수 없다 | PASS — fixture, reference, runtime matrix, Chrome 실행 파일, screenshot의 SHA-256과 환경을 기록했다. |
| 6 | Chrome oracle 결과를 규범 계약이나 Spinon WPT 통과로 표현한다 | PASS — 규범 계산과 Chrome 교차 확인을 분리하고 custom fixture 및 WPT 미실행 한계를 명시했다. |
| 7 | 독립 기대값이 없어 oracle의 동일한 오답을 복제한다 | PASS — named-line 검색, implicit track 생성, track sizing 순서를 사용해 기대값을 별도로 산출하도록 요구한다. |
| 8 | 행·열 implicit track 결과가 뒤바뀌어도 통과한다 | PASS — columns와 rows의 track 배열 및 축별 item frame을 고정한다. |
| 9 | unknown item의 0×0 frame만 검사해 implicit track 수 오류를 놓친다 | PASS — resolved track arrays, line groups, frame, root identity를 함께 검사하도록 요구한다. |
| 10 | DPR 차이에 따른 geometry 분기를 놓친다 | PASS — pinned reference의 DPR 1·2 결과 일치와 고정 viewport를 명시했다. |
| 11 | 입력 환경의 방향·writing mode·gap 차이가 결과에 섞인다 | PASS — LTR, horizontal-tb, zero gap 및 viewport·root 크기를 계획에 고정했다. |
| 12 | `grid-area` shorthand의 네 이름 배치 중 일부만 처리한다 | PASS — row/column의 start·end 네 longhand와 shorthand 확장을 각각 확인한다. |
| 13 | 알 수 없는 area 이름을 실제 named area나 area-derived alias보다 먼저 처리한다 | PASS — 이름 해석 우선순위를 계약에 명시했다. |
| 14 | trailing implicit placement가 leading/negative extent까지 잘못 열어 준다 | PASS — 이 계획의 범위를 trailing implicit track으로 제한하고 leading·negative extent를 제외한다. |
| 15 | 같은 축의 시작·끝 named line을 합쳐 implicit track 하나를 누락한다 | PASS — 2 explicit + 2 implicit track 수와 N+1 line groups를 비교하도록 요구한다. |
| 16 | 기본 `auto`를 0px 상수로 바꿔 유효한 content contribution을 숨긴다 | PASS — 일반 sizing 경로를 요구하고 값만 보고 0px 처리하는 특례를 금지한다. |
| 17 | empty item의 0px 결과를 모든 auto implicit track에 일반화한다 | PASS — 이미 지원하는 C11.4 contribution을 같은 sizing 경로로 보존하고 미지원 기여는 거부한다. |
| 18 | definite root의 남은 free space를 잘못 계산해 auto track stretch가 달라진다 | PASS — explicit track 합과 definite root 크기를 고정해 기준 계산을 설명한다. |
| 19 | 기존 fixed-px `grid-auto-*` track-list cycling phase가 변경된다 | PASS — C11.3.3 목록 순환 회귀와 mode 경계를 독립 조건으로 둔다. |
| 20 | area dimension으로 생기는 implicit track 지원(PR #807)이 거부로 되돌아간다 | PASS — 기존 area-dimension 성공을 회귀로 유지한다고 검토 근거에 적었다. |
| 21 | C11.3.1–3.2의 named-line 및 repeat 결과가 새 경로로 바뀐다 | PASS — 이 작업은 C11.3.3 opt-in profile로 한정하며 상위 profile 회귀를 요구한다. |
| 22 | known-area sibling의 frame·DOM 순서·paint ordinal이 바뀐다 | PASS — 동일 template scene sibling의 geometry와 순서를 고정 회귀 조건으로 명시했다. |
| 23 | numeric placement나 명시 named-line 입력이 unknown-area 지원으로 우회된다 | PASS — 각각 별도 positive/negative control로 분리하도록 했다. |
| 24 | malformed computed value, stale revision 또는 quota 초과 뒤 일부 frame이 게시된다 | PASS — error, revision, no-partial-output과 quota 경계를 완료 기준에 넣었다. |
| 25 | allocation 실패를 복구 불가능한 panic 또는 부분 snapshot으로 바꾼다 | PASS — fallible reservation 및 error atomicity 계약을 재사용하게 했다. |
| 26 | simulator 검증을 Android 실기기나 hardware GPU 근거로 확대한다 | PASS — Android API 37 ARM64 AVD의 software renderer 한계를 명시했다. |
| 27 | iOS Metal drawable 표본을 compositor 이후 화면 픽셀로 표현한다 | PASS — 표본이 compositor 제출 전임을 계획과 제외 범위에 적었다. |
| 28 | fixture FFI가 기본 제품 artifact에 들어가도 검출되지 않는다 | PASS — 기본/product artifact와 opt-in fixture-test positive control을 따로 확인하도록 했다. |
| 29 | Tailnet roadmap 문구가 병합 전·후 서로 다른 상태를 표시한다 | PASS — Tailnet 반영은 merge 후 실제 결과 갱신으로 제한하고 Agent Browser 확인 및 종료를 요구한다. |
| 30 | 계획 PR과 구현 PR의 검토 횟수·증거·merge 기준이 섞인다 | PASS — 계획 20개와 구현/runtime 20개는 작업 단위 검토로 분리하고, 각 PR diff는 30개 관점으로 검토하며 Rebase and merge를 지정한다. |

## 변경 범위 확인

- 계획 문서, 계획 검토표, 이 PR diff 검토표만 계획 PR에 포함한다.
- 공식 상태 대장, 공개 지원 명세, fixture 기대값, 구현 코드와 simulator evidence는 구현 결과가 나올 때 별도 PR에서 다룬다.
- 이 문서는 계획 PR diff 검토 결과이며 기능 구현 검토·runtime 실행·CI 결과를 주장하지 않는다.
