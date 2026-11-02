# PR #1080 공개 roadmap 항목 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 PR #1078의 C11.1 제한 구현 카드다. 코드 PR #1078의 구현 검토 및 PR diff 검토, 병합 상태 대장 PR #1079의 검토와 분리해 이 public roadmap markup과 카드의 근거·상태 표현을 확인했다.

| # | 별도 실패 경로 / 경계 | 검토 결과 |
|---:|---|---|
| 1 | 카드가 존재하지 않거나 다른 PR 번호를 링크함 | 통과 — 카드의 링크·제목 모두 실제 merged PR #1078이다. |
| 2 | merge 날짜가 UTC/현지 시간 변환 때문에 하루 어긋남 | 통과 — API에서 확인한 `2026-11-02T19:04:37Z`를 UTC 그대로 표시한다. |
| 3 | merge SHA가 source head와 혼동됨 | 통과 — PR merge commit과 main에서 확인한 `e900124dcc02e5c28e526a960e6a8ff93461d353`를 기록한다. |
| 4 | 새 카드가 과거 #1072보다 앞선 C11 목록 순서를 깨뜨림 | 통과 — #1072 바로 뒤, 더 오래된 #1002 앞에 추가했다. |
| 5 | PR #1078 카드가 이미 있어 중복 게시됨 | 통과 — 추가 전 원본에는 #1078 카드가 없었고 최종 등장 횟수는 하나다. |
| 6 | HTML `<p>`가 닫히지 않아 다음 roadmap 카드가 잘못 중첩됨 | 통과 — 기존 형식과 같은 단일 paragraph로 열고 닫는다. |
| 7 | 잘못된 HTML attribute/escape로 RSPress parse가 깨짐 | 통과 — 링크는 따옴표 속성, angle-bracket 함수 표기는 `<code>` 요소로 escape했다. |
| 8 | 계획 항목을 구현 완료로 표현함 | 통과 — 카드 타입은 `제한 구현`이며 계획 문구를 완료로 승격하지 않는다. |
| 9 | 일반 CSS author-facing 지원으로 기능 범위를 넓힘 | 통과 — math는 기존 Root HTML Grid route의 제한 slice라고 기록하고 일반 CSS admission을 제외한다. |
| 10 | `min()`/`max()` 또는 `clamp()` 지원 범위를 누락·확대함 | 통과 — 기존 typed AST의 산술과 해당 세 함수만 열거한다. |
| 11 | shorthand와 legacy alias 중 하나를 누락함 | 통과 — `gap`과 `grid-gap`을 모두 포함한다. |
| 12 | implicit-row 또는 sparse column-flow 영향을 빠뜨림 | 통과 — 두 기존 Root HTML 제한 경로를 따로 명시한다. |
| 13 | named-area 제한 경로가 빠짐 | 통과 — fixed named-area 경로라고 범위를 제한해 기록한다. |
| 14 | integration test 개수를 resolver unit 개수에 더해 잘못 표기함 | 통과 — Root HTML 8개와 resolver 3개를 서로 구분한다. |
| 15 | `spinon-layout` 제외 C15 실패를 통과 수에 포함함 | 통과 — 379 passed와 기존 실패 1개 제외를 함께 표시한다. |
| 16 | style-to-layout의 알려진 두 실패를 pass로 계산함 | 통과 — 321 passed와 비관련 실패 2개 제외를 명시한다. |
| 17 | scoped Clippy/rustfmt를 workspace 전체 성공으로 일반화함 | 통과 — scoped 도구 검사만 주장한다. |
| 18 | 기존 실패가 이번 코드 변경에서 발생한 것으로 오인됨 | 통과 — status row의 회귀 제외와 기존 failure 설명 링크를 붙인다. |
| 19 | Android AVD를 실기기로 표기함 | 통과 — API 37 ARM64 AVD와 SwiftShader software 경로를 구체적으로 쓴다. |
| 20 | iPhone Simulator를 실기기로 표기함 | 통과 — iPhone 17 Pro / iOS 26.2 Simulator라고 쓴다. |
| 21 | 플랫폼 간 runtime 입력이 달라 비교 결과가 무효가 됨 | 통과 — linked evidence README가 같은 runtime fixture SHA-256을 기록한다. |
| 22 | 시뮬레이터 화면을 pixel readback 결과로 과장함 | 통과 — visual smoke로 한정하고 pixel readback이 아니라고 표시한다. |
| 23 | hardware GPU 또는 실제 기기 검증을 암시함 | 통과 — 두 검증 모두 Simulator이며 hardware/device 증거를 지원 범위에서 제외한다. |
| 24 | 전체 CSS WPT conformance를 달성한 것처럼 보임 | 통과 — 전체 WPT를 명시적으로 범위 밖으로 둔다. |
| 25 | C11.1 또는 C11 부모 체크가 완료로 바뀜 | 통과 — public page counter와 completion language는 유지하고 제한 조각으로 표시한다. |
| 26 | official `72/213` 숫자가 새 slice 때문에 증가함 | 통과 — 카드가 72/213 유지 사실을 적고 새 completion checkbox를 만들지 않는다. |
| 27 | PR checks 없음이 CI 성공으로 해석됨 | 통과 — rollup 미보고 및 main required status checks 부재를 적으며 CI pass를 주장하지 않는다. |
| 28 | 내부 계약 링크가 숫자 버전 변경을 암시하거나 깨짐 | 통과 — 기존 계약 0059 route를 가리키고 contract version은 바꾸지 않는다. |
| 29 | 계획·실행·회귀 근거 중 하나가 잘못된 route에 연결됨 | 통과 — 링크 target은 merged evidence 폴더의 README와 package-test note route다. |
| 30 | public roadmap 수정과 Tailnet 게시를 같은 검증으로 취급함 | 통과 — 이 PR은 source HTML 변경만이며 Tailnet publication은 이후 별도 hash 검증으로 수행한다. |

결론: 공개 roadmap 카드의 날짜·병합 상태·범위·수치·링크에서 별도 차단 문제를 찾지 못했다. 이 검토는 HTML 문서 변경 검토이며 새 runtime test나 Tailnet 배포 확인은 아니다.
