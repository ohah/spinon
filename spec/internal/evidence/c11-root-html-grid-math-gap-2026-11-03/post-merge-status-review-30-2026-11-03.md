# PR #1078 병합 상태 대장 변경 검토 30개

검토 대상은 병합된 PR #1078의 상태 행을 최종 merge 결과에 맞춘 문서 PR diff다. 코드·검증 범위는 이 문서 PR에서 다시 주장하지 않고 이미 병합된 PR #1078 근거를 연결한다.

| # | 별도 실패 경로 / 불변 조건 | 검토 결과 |
|---:|---|---|
| 1 | PR 번호 링크가 다른 C11 slice를 가리킴 | 통과 — `#1078`의 GitHub URL과 제목이 Root HTML Grid math gap PR을 가리킨다. |
| 2 | 상태가 열린 PR인데 병합 완료로 적힘 | 통과 — GitHub 조회에서 PR state는 `MERGED`다. |
| 3 | Rebase and merge 시각의 초 단위가 잘못 기록됨 | 통과 — API의 `mergedAt`은 `2026-11-02T19:04:37Z`다. |
| 4 | UTC 시각을 현지 날짜와 혼동함 | 통과 — 상태 행은 UTC `Z`를 표시하고 작업 기준 날짜는 KST 2026-11-03으로 둔다. |
| 5 | merge SHA가 PR head commit 또는 낡은 candidate SHA로 기록됨 | 통과 — PR `mergeCommit.oid`와 `origin/main` HEAD가 `e900124dcc02e5c28e526a960e6a8ff93461d353`로 같다. |
| 6 | PR base가 확인한 main과 달라 병합 근거가 어긋남 | 통과 — PR base OID는 병합 전 `88b4001949fca480ff70dc75000f52d0c13063e6`이며 현재 main은 기록된 merge commit이다. |
| 7 | rebase merge를 일반 merge commit이나 squash로 잘못 표기함 | 통과 — PR 병합 명령은 `--rebase`였고 GitHub에서 보고한 전략도 Rebase and merge다. |
| 8 | merge 뒤 PR checks가 생겼는데 비어 있다고 기록함 | 통과 — 병합 직후 check rollup과 `gh pr checks` 결과가 모두 비어 있었다. |
| 9 | 필수 status checks 부재를 CI 성공으로 과장함 | 통과 — required checks 조회의 404 사실을 적고 CI 통과로 기록하지 않는다고 명시한다. |
| 10 | 체크 부재를 branch protection 전체 부재로 확대 해석함 | 통과 — 문장은 required status checks만 언급한다. |
| 11 | 상태 행의 날짜가 merge 일자만 가리켜 실행 날짜를 혼동시킴 | 통과 — 행 시작일은 실행 근거 날짜 2026-11-03이고 병합 시각을 별도로 적는다. |
| 12 | 이번 제한 조각을 C11 상위 완료로 올림 | 통과 — C11 checkbox는 그대로 `- [ ]`다. |
| 13 | C11.1 부분 구현을 공개 CSS 지원 완료로 표시함 | 통과 — 기록을 제한 구현이라 부르고 일반 CSS admission과 C11.1/C11 완료를 제외한다. |
| 14 | 공식 완료 수가 merge 때문에 자동 증가한 것처럼 보임 | 통과 — 공식 `72/213` 유지 문장을 그대로 보존한다. |
| 15 | C11.3 또는 C11.4 checkbox를 변경함 | 통과 — 이 PR diff는 C11.1 이력 한 줄만 변경한다. |
| 16 | 전 버전 대장 이력이나 다른 PR 행을 덮어씀 | 통과 — `git diff`의 변경 범위는 PR #1078 행 한 줄이다. |
| 17 | API/인터페이스 숫자 버전 상승을 암시함 | 통과 — 기존 내부 계약 0059 `0.1.0`을 유지한 범위로 기록한다. |
| 18 | CSS 수식 지원 범위를 전체 math 함수로 넓혀 적음 | 통과 — 산술, `min()`, `max()`, `clamp()`만 열거한다. |
| 19 | `gap` shorthand가 빠지고 longhand만 지원한 것으로 축소/확대됨 | 통과 — 두 longhand, `gap`, legacy `grid-gap` alias를 기록했다. |
| 20 | implicit row, column-flow 또는 named-area 적용 경로를 누락함 | 통과 — 세 경로를 각각 제한 조건과 함께 적는다. |
| 21 | 8개 integration과 resolver 3개 수를 전체 패키지 수와 혼동함 | 통과 — 두 focused suite 수를 별도로 표기했다. |
| 22 | layout package의 제외 테스트 하나를 통과로 계산함 | 통과 — 379개 통과와 기존 C15 실패 1개 제외를 함께 쓴다. |
| 23 | style-to-layout의 두 기존 실패를 통과로 계산함 | 통과 — 321개 통과와 비관련 실패 2개 제외를 같이 쓴다. |
| 24 | scoped Clippy/rustfmt를 workspace 전체 통과로 확대함 | 통과 — scoped 도구 검사라고 적고 전체 workspace/CI 통과를 주장하지 않는다. |
| 25 | AVD를 Android 실기기로 잘못 표기함 | 통과 — Android API 37 ARM64 AVD와 SwiftShader software 경로를 명시한다. |
| 26 | iOS Simulator를 iPhone 실기기라고 표기함 | 통과 — iPhone 17 Pro / iOS 26.2 Simulator라고 기록한다. |
| 27 | 두 플랫폼 화면을 pixel-exact 또는 compositor 이후 근거로 부름 | 통과 — visual smoke이며 pixel readback 근거가 아니라고 명시한다. |
| 28 | evidence README의 plan/test/review 링크 중 하나가 끊김 | 통과 — 상대 경로의 계획, README, review와 package note 파일이 모두 존재한다. |
| 29 | 구현 PR diff review와 병합 상태 문서 review를 같은 검토로 셈 | 통과 — 구현 PR 30개와 이 문서 PR의 post-merge status 검토 30개 파일을 분리한다. |
| 30 | 후속 문서 PR이 runtime 동작을 새로 검증한 것처럼 보임 | 통과 — 이 기록이 이미 병합된 PR #1078 근거를 링크할 뿐 새 runtime 결과를 주장하지 않는다고 서두에 제한한다. |

결론: 병합 상태 변경에서 별도 수정이 필요한 사실·범위 오류를 찾지 못했다. 이 검토는 문서 diff 검토이며 새 기능 테스트나 CI 결과가 아니다.
