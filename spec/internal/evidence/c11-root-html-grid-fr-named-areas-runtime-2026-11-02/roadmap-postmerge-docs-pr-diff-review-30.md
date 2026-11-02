# PR #1030 병합 후 상태·로드맵 동기화 diff 실패 관점 30개

PR #1030 병합 후 `spec/STATUS.md`, 전용 계획, `spec/public/roadmap.html`의 C11 기록을 대조했다. 이 검토는 코드 PR의 구현/runtime 20개 및 구현 diff 30개와 별도이며, Tailnet 파일 교체 자체는 아직 실행 근거에 포함하지 않는다.

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | C11 top-level checkbox가 제한 Root HTML slice 때문에 완료로 바뀌었는가? | PASS. C11 checkbox는 계속 미완료다. |
| 2 | 전역 완료 수가 `72/213`에서 잘못 증가했는가? | PASS. 공식 수치는 그대로다. |
| 3 | C11.1 또는 C11.3.3 candidate checkbox를 제한 slice 하나로 승격했는가? | PASS. 두 하위 항목도 미완료로 유지한다. |
| 4 | PR 기록에 다른 pull request 번호를 썼는가? | PASS. 구현 PR은 #1030으로 연결된다. |
| 5 | GitHub merge commit SHA가 PR #1030 상태와 다른가? | PASS. `579435b1955e7ad157baa13d100d75f13a5dd283`를 사용한다. |
| 6 | merge 날짜를 로컬 시각과 UTC로 혼동했는가? | PASS. GitHub `mergedAt`의 `2026-11-02T04:29:37Z`를 기록한다. |
| 7 | implementation scope를 일반 CSS Grid로 기술했는가? | PASS. 기본 parsed Root HTML 제한 profile로 명시했다. |
| 8 | 기존 named-area 고정 px 경로의 보존을 빠뜨렸는가? | PASS. status와 roadmap 카드 모두 기존 fixed-px 경로 유지라고 명시한다. |
| 9 | fractional 지원을 한 축만으로 오기했는가? | PASS. definite 두 축의 exact direct `1fr 2fr` 조합으로 기록한다. |
| 10 | 임의 template map이나 fraction weights까지 넓혀 썼는가? | PASS. 동일 2×2 named-area map과 exact ratio만 언급한다. |
| 11 | 성공한 테스트 수의 집계가 중복됐는가? | PASS. Grid focused 22/22는 Root HTML 65/65 안에 든다고 적어 합산하지 않는다. |
| 12 | C11.3.3 26/26을 새 변경의 전체 area 적합성으로 오인하는가? | PASS. regression 시험이라고 한정한다. |
| 13 | scoped Clippy 결과를 workspace 전체 결과로 확대했는가? | PASS. scoped 패키지 Clippy라고 그대로 표현한다. |
| 14 | simulator를 실기기 실행으로 기술했는가? | PASS. Android API 37 AVD, iPhone 17 Pro/iOS 26.2 Simulator로 구분한다. |
| 15 | screenshot/draw를 GPU surface-pixel equality로 주장했는가? | PASS. PixelCopy와 Metal drawable readback을 하지 않았다고 기록한다. |
| 16 | 첫 simulator harness mismatch가 숨겨져 로그와 설명이 충돌하는가? | PASS. 상세 실패/수정은 구현 evidence README에 연결한다. |
| 17 | CI check가 비어 있었는데 통과했다고 적었는가? | PASS. GitHub check run과 required status check이 없었다고 명시한다. |
| 18 | 새로운 내부 API/FFI나 contract version을 추가한 것처럼 보이는가? | PASS. 내부 계약 0059는 0.1.0으로 유지하며 API 추가가 없다고 적는다. |
| 19 | 계획 문서의 상태가 PR이 열린 상태에 머무는가? | PASS. merged status와 merge SHA로 고쳤다. |
| 20 | 계획의 구현 결과가 status 수치와 서로 모순되는가? | PASS. 결과는 제한 구현으로 기술하고 완료 수를 유지한다. |
| 21 | 공식 상태의 새 항목이 #1027 기록과 잘못된 순서로 들어가는가? | PASS. C11.1 최신 항목 뒤에 새 PR을 놓는다. |
| 22 | 공식 상태에서 계획의 relative Markdown 링크가 저장소 루트 기준으로 잘못됐는가? | PASS. `spec/STATUS.md`의 기존 경로 규칙에 맞춰 `../plan/`으로 연결했다. |
| 23 | Chrome precomparison route가 없거나 다른 fixture를 가리키는가? | PASS. 전용 C11 named-area+fr precomparison README를 연결한다. |
| 24 | Android/iOS evidence 링크가 pass02 harness 수정 이전 결과만 가리키는가? | PASS. README가 최종-source pass03 실행을 설명한다. |
| 25 | runtime/implementation review와 PR diff review 링크가 뒤바뀌었는가? | PASS. 두 review 문서를 각각 정확한 상대 경로에 연결했다. |
| 26 | 새 HTML roadmap card가 기존 card paragraph 안쪽으로 들어가 markup을 깨는가? | PASS. PR #1027 `</p>` 경계 바로 뒤에 독립 `hero-progress` paragraph를 삽입했다. |
| 27 | HTML card가 status/plan route를 잘못된 prefix로 가리키는가? | PASS. local route는 `/spinon/` base에 맞춰 절대 경로로 쓴다. |
| 28 | 카드에 수동 screenshot 경로가 깨진 링크로 표시되는가? | PASS. 카드는 source/README routes만 연결하며 화면 이미지는 PR #1030 본문과 evidence README가 소유한다. |
| 29 | 카드가 WPT, public CSS admission, physical GPU validation을 암시하는가? | PASS. 세 범위를 남은 항목으로 명시해 제한을 유지한다. |
| 30 | 문서 변경 외의 코드나 기존 work-in-progress까지 status sync 범위에 섞였는가? | PASS. 이번 변경은 상태 Markdown, 기존 계획, public roadmap HTML, 이 review artifact로 한정했다. |

결과: 30개 관점 모두 PASS. Tailnet preview에 이를 게시하고 local/Tailnet HTTP bytes·hash 및 Agent Browser viewport를 확인하는 것은 별도 후속 동기화 작업이다.
