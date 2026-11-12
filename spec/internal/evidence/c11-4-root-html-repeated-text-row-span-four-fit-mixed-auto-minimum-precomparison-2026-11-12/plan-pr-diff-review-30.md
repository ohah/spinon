# Four-fit mixed minimum 계획 PR · 변경분 실패 관점 30개 검토

**대상 변경:** 현재 계획 branch의 계획·Chrome precomparison·fixture·roadmap diff. 구현 코드 변경은 포함하지 않는다. 계획 자체의 20개 실패 관점 검토와 별도이며 이를 대체하지 않는다.

| # | Diff 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 의도하지 않은 제품 Rust/FFI 코드가 계획 PR에 섞임 | Staged path inventory에는 계획·review·Chrome evidence·HTML fixture·public roadmap만 있다. Rust/FFI source, Cargo feature, ABI 변경은 없다. |
| 2 | 기존 사용자의 tracked WIP가 함께 stage됨 | `git status --short --untracked-files=no`를 대조했다. stage된 두 tracked 문서는 `plan/c11-4-grid-track-sizing.md`, `spec/public/roadmap.html`뿐이다. |
| 3 | 새 파일이 다른 날짜/기존 evidence를 덮어씀 | 새 evidence는 고유한 `...four-fit-mixed...-2026-11-12/` 폴더에 생성됐다. 11월 11일 four-fit uniform reference는 읽기 전용으로 유지했다. |
| 4 | 공유된 3만여 untracked evidence가 자동 추가됨 | `git add -A`/상위 폴더 add를 쓰지 않았다. exact task-owned evidence directory와 fixture path만 stage했다. |
| 5 | 계획 링크가 잘못된 상대 경로를 가리킴 | 집중 계획, 20개 plan review, Chrome README, fixture, parent tracker의 상대 경로와 대상 파일을 대조했다. |
| 6 | Mermaid/Markdown 렌더링이 제목·표·code span을 깨뜨림 | 새 Markdown을 읽고 표 separator, fenced command, inline code, Korean headings를 확인했다. raw shell/browser output은 문서 prose에 삽입하지 않았다. |
| 7 | 규범 링크가 다른 CSS 문서/섹션을 참조 | CSS Grid 2025 CRD §6.6, §7.2.1, §11.5.1 링크를 계획의 eligibility와 track-sizing 설명에 대조했다. |
| 8 | Agent Browser 관측이 Spinon 동작으로 표시됨 | README, plan, verification limits와 PR 이미지 설명은 Chrome reference observation임을 반복해 구분한다. 제품 결과로 확장하지 않았다. |
| 9 | Browser 버전이나 DPR/viewport metadata가 실제 capture와 다름 | Manifest와 verifier가 Agent Browser 0.31.1, HeadlessChrome 157, DPR 1, `1280×900`을 묶고 전체 observation에서 동일함을 검사한다. |
| 10 | Capture가 Terminal Browser를 사용 | `capture-chrome.mjs`는 설치된 Agent Browser CLI의 격리 namespace/session을 사용한다. Terminal Browser 명령/세션은 없다. |
| 11 | Capture script가 종료 중에도 전용 browser context를 남김 | `finally`에서 정확한 namespace/session을 닫고 lifecycle JSON에 `closed:true`를 남긴다. 실제 session list는 `No active sessions`였다. |
| 12 | 평상시 shared simulator나 사용자 profile을 삭제/종료 | PR evidence에는 simulator 제거가 없다. 현재 iOS booted list는 비었고 S03 기기는 이전 inventory에도 없다. 계획은 task-owned 실행만 닫고 profile/data 삭제는 하지 않도록 제한한다. |
| 13 | Fixture가 기존 text/font/Grid input을 넓은 제품 admission으로 해석 | Fixture는 기존 payload grammar, fixed placement, `40×220px` Grid를 유지하고 per-item class만 mask에 따라 바꾼다. |
| 14 | Combinatorial matrix 수가 주석과 실제 실행에서 다름 | 720 lists × 192 scenes = 138,240을 capture와 verifier가 모두 확인했다. topology별 scene/mask count도 결과에 남는다. |
| 15 | Tail-order 순열이 누락되거나 max-content가 들어감 | Four-fit fixture와 verifier는 `[min-content,auto]`, `[auto,min-content]` 두 tail만 사용한다. |
| 16 | Mask order가 source order reverse에서 바뀜 | Capture metadata와 verifier가 semantic A–E/A–D bit order를 검사하고 DOM reverse pair의 item geometry를 비교한다. |
| 17 | Existing uniform controls가 baseline에 맞춰져 있지 않음 | 11월 11일 uniform observation의 hash를 먼저 확인하고 mask 0/all-auto 11,520 endpoints를 row/item/payload/line geometry로 비교한다. 최대 차이는 0px다. |
| 18 | Mixed mask 입력 일부가 관측되지 않음 | 각 topology/order의 32/16 masks가 720 contexts × 2 alignments로 전부 관측됐고 mask당 scene 수는 1,440이다. |
| 19 | Auto floor 결과가 mask=0의 값과 합산되어 숨겨짐 | 독립 verifier는 computed minimum과 eligibility를 item별로 검사하고 auto/zero shortfall 집계를 별도로 출력한다. |
| 20 | Overflow 결과를 예상대로 0으로 고정해 회귀를 숨김 | `Grid-end overflow`는 1,392건으로 집계됐고 모두 zero scenes였다. 계획/검증은 이를 사전 관측과 비교하도록 하되 0을 강제하지 않는다. |
| 21 | CSS row serialization만 확인하고 item frame은 빠짐 | Verifier는 computed rows와 authored list뿐 아니라 item frame, payload frame, text line rectangles, occupied span-row 합을 확인한다. |
| 22 | Fixture screenshot이 이미지 링크 없이 PR 본문에 남음 | PR 본문은 branch raw PNG URL을 포함하도록 작성할 예정이다. 대체 text에서 Chrome oracle sample임을 명시하고 제품 화면으로 호칭하지 않는다. |
| 23 | Screenshot crop/scale이 matrix CSS 좌표를 바꿈 | Screenshot은 별도 대표 scene만 3.5배 시각화하고 matrix 관측은 원래 `1280×900`, DPR 1에서 수집한다. PNG header/dimensions `480×900`와 manifest hash가 verifier에서 확인된다. |
| 24 | Runtime sample이 full oracle이라고 표현됨 | 계획은 RuntimeSession sample `3,840`, Chrome oracle `138,240`, existing uniform endpoint `11,520`을 별도 denominator로 적고 full RuntimeSession claim을 금지한다. |
| 25 | Sample 선택식의 반복 list/누락 tail이 계획에 있음 | 선택식 검산은 20 unique list index, 15 position sets, 20 cap orders, 두 tail과 3,840 scenes를 반환했다. |
| 26 | 기존 mixed-rejection test가 새 behavior와 계속 충돌 | 실제 test 이름을 확인하고 해당 assertion을 positive mask matrix와 invalid control/value rejection으로 교체하도록 계획에 넣었다. |
| 27 | Future Cargo command가 0-test selector로 성공 보고될 위험 | Uniform selector는 현재 test function의 정확한 이름을 사용한다. 새 mixed selector는 구현 시 test function을 추가한 뒤 실행해야 하고 no-match는 증거가 아니라고 규정한다. |
| 28 | Plan PR merge 시 공식 구현 count/contract가 앞서 증가 | `spec/STATUS.md`와 internal contract를 이 plan PR에서 수정하지 않았다. 공식 `78/214`, C11.4/C11, numeric `0.1.0`은 그대로다. |
| 29 | Public roadmap에서 이전/다음 단계 관계가 불일치 | Parent tracker의 다음 후보와 public roadmap link 모두 four-fit mixed 계획으로 바뀌었다. Agent Browser에서 새 anchor text와 href가 렌더링되는 것을 확인했다. |
| 30 | 이 review artifact 자체가 중복 관점/잘못된 review count를 담음 | 항목 번호 1–30을 확인했다. 이 표는 plan review 20개와 내용/목적을 분리하고 local/remote final diff hash 대조는 PR push 뒤 별도 확인한다. |

## 판정

Staged PR diff는 의도된 bounded plan·Chrome 기준자료·roadmap 동기화에 한정된다. `git diff --cached --check` 통과, Chrome verifier 통과, public roadmap link의 Agent Browser 확인을 기록했다. PR 생성 후 head와 GitHub가 제공하는 실제 diff를 local merge-base diff와 대조하기 전에는 merge하지 않는다.
