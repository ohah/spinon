# PR #1002 병합 후 C11 상태·로드맵 변경 적대 검토 30개

검토 대상은 `spec/STATUS.md`, `spec/public/roadmap.html`, C11.1 계획 2개, 내부 계약 0059 및 Tailnet 동기화 근거다. C11 전체 완료를 과장하거나, 기존 preview 자료를 손실하거나, 실제 브라우저 표시와 다른 내용을 게시하는 실패 경로를 각각 확인했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | PR #1002의 병합 상태·시각을 잘못 기록 | `gh pr view 1002`의 `MERGED`, `2026-11-01T20:53:41Z`와 대조했다. PASS |
| 2 | merge SHA가 PR과 다른 커밋을 가리킴 | PR merge commit `12b554c5e1a1946e7267545d5d5092ba33211af9`와 문서가 일치한다. PASS |
| 3 | PR check 미보고를 CI 통과로 오인 | `statusCheckRollup`이 빈 배열이다. 문서에 check run 미보고라고 썼다. PASS |
| 4 | C11 parent 체크박스가 우연히 변경됨 | `spec/STATUS.md` diff는 C11 parent 줄을 수정하지 않고 진행 기록만 추가한다. PASS |
| 5 | 공식 72/213 집계가 증가하거나 감소 | 공식 수는 기존 `72/213`으로 유지된다. PASS |
| 6 | 제한 profile 완료 수를 C11 전체 진행률로 환산 | 비선형인 하위 작업에 완료 퍼센트를 붙이지 않았다. PASS |
| 7 | C11.2.2 제한 profile 상태를 갱신 때문에 덮어씀 | 기존 checked 상태와 실행 범위가 그대로다. PASS |
| 8 | C11.2.3 제한 profile 상태를 갱신 때문에 덮어씀 | 기존 checked 상태와 실행 범위가 그대로다. PASS |
| 9 | C11.3을 완료 상태로 잘못 표시 | 공식 C11.3 checkbox가 unchecked로 남아 있다. PASS |
| 10 | C11.3.3 candidate를 완료로 잘못 표시 | candidate checkbox가 unchecked로 남아 있다. PASS |
| 11 | C11.4 intrinsic track sizing을 완료로 잘못 표시 | 공식 C11.4 checkbox가 unchecked로 남아 있다. PASS |
| 12 | PR #1000 base-runtime 통합과 #1002 gap slice를 혼합 | 계획 문서가 두 병합을 별도 시각·SHA로 구분한다. PASS |
| 13 | 제한 Root HTML 경로를 fixture-only profile이라고 표현 | 문서는 기본 `RuntimeRootHtmlTextPaintV1` 경로라고 명시한다. PASS |
| 14 | fixed-px 범위를 모든 CSS gap 값으로 확장 | finite non-negative `LengthPx`만 통과한다고 썼다. PASS |
| 15 | percentage gap 지원을 암시 | percentage는 계속 거부한다고 roadmap과 계획에 표시했다. PASS |
| 16 | math-backed `calc()` 지원을 암시 | math-backed `calc()`는 계속 거부한다고 표시했다. PASS |
| 17 | invalid negative CSS declaration을 runtime diagnostic과 혼동 | CSS-invalid shorthand cascade 동작과 computed unsupported 값의 경계를 구분했다. PASS |
| 18 | style snapshot의 gap 값이 두 번 전달되거나 다른 revision에서 옴 | 계약 설명은 같은 style snapshot의 row/column 값이 `LayoutGap`에 전달된다고 유지한다. PASS |
| 19 | 18/18 Root HTML 회귀를 전체 style-to-layout 성공으로 해석 | 범위를 Root HTML regression으로 한정했다. PASS |
| 20 | admission·invalid CSS 검사를 단일 test로 합산 | 두 검사를 각각 1/1로 나눠 기록했다. PASS |
| 21 | Chrome oracle verifier를 Spinon WPT 적합성으로 해석 | Chrome 156 verifier 1/1로만 기록했고 WPT 통과를 주장하지 않는다. PASS |
| 22 | Android AVD를 SM-S731N 실기기나 hardware GPU로 확대 | API 37 AVD로 표시하고 PixelCopy readback을 했다고 쓰지 않는다. PASS |
| 23 | iOS Simulator를 iPhone 실기기로 확대 | iPhone 17 Pro/iOS 26.2 Simulator로 명시했다. PASS |
| 24 | simulator screenshot을 pixel-exact surface evidence로 확대 | 캡처를 draw/present 보조 자료라고 표시하고 surface readback과 구분한다. PASS |
| 25 | 내부 계약 버전을 범위 추가만으로 올림 | 계약 0059의 `0.1.0` 고정을 유지한다. PASS |
| 26 | 구현 PR 대기 문구가 계획에 남음 | gap 계획·runtime 계획의 top-level 상태 문구를 병합 사실로 갱신했다. PASS |
| 27 | PR/evidence 경로가 존재하지 않거나 다른 slice를 가리킴 | #1002 URL과 gap evidence README·review 파일을 저장소에서 확인했다. PASS |
| 28 | public roadmap 새 카드가 기존 최신 카드와 순서를 혼동 | PR #1000 기본 통합 카드 다음에 #1002 gap 카드가 오도록 삽입하고 뒤 항목을 보존했다. PASS |
| 29 | Tailnet 교체가 기존 route를 백업 없이 덮거나 서버를 재시작 | 변경 전 hash를 별도 backup에 보존하고 route 한 개만 atomic rename했으며 server PID 28040을 유지했다. PASS |
| 30 | 브라우저가 stale page를 보거나 mobile overflow를 놓침 | Tailnet Agent Browser에서 merge SHA·72/213을 확인했고 390px viewport/document/body 폭이 모두 390px이다. 확인 뒤 Agent Browser 프로세스가 남지 않았다. PASS |

## 판정

**PASS 30 / FAIL 0.** 공식 상태는 C11 미완료와 `72/213`을 유지한다. 제한된 #1000/#1002 Root HTML runtime 증분, 문서 반영, Tailnet served hash 및 Agent Browser 화면을 서로 구분해 기록했다. CI·Spinon WPT·Android 실기기/hardware GPU·iOS 실기기·surface pixel readback은 주장하지 않는다.
