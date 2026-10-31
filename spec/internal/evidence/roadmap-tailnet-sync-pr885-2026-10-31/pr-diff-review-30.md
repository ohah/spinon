# PR #886 merge-record·roadmap diff 실패 관점 30개

PR #885의 병합 뒤 상태 기록과 public roadmap 최신 항목을 따로 검토했다. 검토 subject patch는 base `9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3` 대비 SHA-256 `86b658cc2a628de4bee7a250c6f23aaac44d141f7779b37b9066fe8f8e41053d`이며 이 보고서 파일은 해시에서 제외했다.

| # | 공격 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | PR 번호를 다른 작업에 연결하는가 | **PASS** — 본문과 문서가 C11.4 default tab slice PR #885만 가리킨다. |
| 2 | merge timestamp가 UTC 원본과 다른가 | **PASS** — GitHub `mergedAt`의 `2026-10-31T00:01:56Z`를 기록했다. |
| 3 | merge SHA가 GitHub 결과와 다른가 | **PASS** — `9ca5c04ac6aaeb45d378b33f1bfcfe616d04cfd3`를 그대로 썼다. |
| 4 | merge method가 선형 이력 정책을 어기는가 | **PASS** — Rebase and merge를 기록했다. |
| 5 | PR check rollup이 비었는데 CI 성공이라 하는가 | **PASS** — 빈 rollup과 미통과 보고를 문서에 분리했다. |
| 6 | main required checks가 없다는 경계를 감추는가 | **PASS** — 필수 status check 미설정을 기록했다. |
| 7 | PR `mergeStateStatus: UNKNOWN`을 통과 결과로 바꾸는가 | **PASS** — 문서는 merge 완료와 CI 상태를 혼동하지 않는다. |
| 8 | C11 공식 완료 수가 몰래 증가하는가 | **PASS** — `72/213`을 유지했다. |
| 9 | C11 상위 checkbox가 완료로 바뀌는가 | **PASS** — unchecked 상태를 유지한다. |
| 10 | C11.4 상위 checkbox가 완료로 바뀌는가 | **PASS** — unchecked 상태를 유지한다. |
| 11 | fixture 제한 slice를 전체 CSS 지원으로 승격하는가 | **PASS** — Ahem 고정 opt-in profile 범위로 한정했다. |
| 12 | 모든 tab-size 값을 지원하는 듯 서술하는가 | **PASS** — 기본 `tab-size:8`만 지원한다고 명시한다. |
| 13 | `tab-size:2` negative scene을 생략하는가 | **PASS** — negative 경계를 PR 증거와 같이 유지했다. |
| 14 | 시뮬레이터 결과 장면 수를 혼동하는가 | **PASS** — 두 대상 각각 기존 포함 40/40으로 기재했다. |
| 15 | positive draw/present count를 40/40으로 부풀리는가 | **PASS** — 39/39와 negative 1/1을 구별한다. |
| 16 | expected rejection을 실패나 성공 scene으로 세는가 | **PASS** — `tab-size:2` 기대 거부를 설명한다. |
| 17 | retry 상태를 누락하는가 | **PASS** — retry 0을 보존한다. |
| 18 | Android target을 실기기로 부르는가 | **PASS** — API 37 ARM64 AVD로 한정한다. |
| 19 | iOS Simulator 실행을 실기기로 부르는가 | **PASS** — iPhone 17 Pro/iOS 26.2 Simulator로 구분한다. |
| 20 | screenshot을 glyph pixel proof로 확대하는가 | **PASS** — surface 제출만 보이며 glyph/compositor pixel은 검증하지 않았다고 명시했다. |
| 21 | hardware GPU를 검증했다고 주장하는가 | **PASS** — 그런 주장을 하지 않는다. |
| 22 | 일반 authored CSS admission을 포함한다고 오해시키는가 | **PASS** — 일반 cascade/admission은 범위 밖으로 둔다. |
| 23 | 전체 WPT 적합성 결과로 오해시키는가 | **PASS** — 전체 WPT와 지원을 명시적으로 제외한다. |
| 24 | simulator 근거 링크가 다른 실행 evidence를 가리키는가 | **PASS** — PR #885의 날짜가 있는 결과 문서를 연결한다. |
| 25 | 계획 링크가 다른 C11.4 후보 계획을 가리키는가 | **PASS** — default-tab slice 전용 계획으로 연결한다. |
| 26 | 상태 문서가 PR merge record를 빼먹는가 | **PASS** — 상태 대장과 내부 계약 모두 PR·merge SHA를 기록한다. |
| 27 | roadmap 최신 항목이 과거 PR #880을 덮어쓰거나 제거하는가 | **PASS** — #885를 앞에 추가하고 #880 entry는 보존했다. |
| 28 | raw public roadmap에서 status anchor/link가 누락되는가 | **PASS** — 상태 anchor와 구현 evidence 링크를 포함한다. |
| 29 | 내부 계약 숫자 버전을 임의로 올리는가 | **PASS** — 기존 `0.1.0`을 유지했다. |
| 30 | 문서 후속에 무관한 제품 코드 변경이 섞이는가 | **PASS** — plan, status, 내부 계약, public roadmap 네 파일만 변경한다. |

이 검토는 PR #886의 문서 diff 범위만 다룬다. C11/C11.4 구현 적합성 또는 Tailnet 서버 반영을 대신하지 않는다.
