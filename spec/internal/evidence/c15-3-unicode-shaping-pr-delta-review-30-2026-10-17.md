# C15.3 계획 PR diff 적대 검토 30개

**검토 범위:** PR에 들어갈 정확한 파일 집합, status/spec 연결, 외부 원본과 바이너리 자산, evidence metadata, GitHub PR 준비 경계. 앞서 기록한 plan review 20개와 겹치지 않도록 계획의 shaping/layout 동작 기준 대신 변경 경로·산출물·출처·배포 검증을 살폈다. 이 문서는 runtime implementation PR의 검토가 아니다.

| # | PR diff 확인 관점 | 실패 조건 | 결과 |
| --- | --- | --- | --- |
| 1 | 작업 브랜치 기준 | 계획 PR이 main 위가 아니거나 관계없는 다른 branch 변경을 가져옴 | isolated worktree의 plan/c15-3-shaping-paint-20261017 branch에서 작업한다. PR 생성 전 origin/main과 merge base를 다시 확인한다. |
| 2 | root checkout 보존 | 사용자 root checkout의 기존 변경이 stage/commit에 포함됨 | 작업은 별도 worktree 안에 있다. root checkout은 수정하지 않았다. |
| 3 | explicit staging | git add -A가 exploratory untracked file을 포함 | 기존 shaping-paint screenshot/fixture/evidence와 c15-3-browser.png, c15-3-debug.png는 명시적 path 목록에서 제외한다. |
| 4 | PR 파일 집합 | GitHub diff에 이 PR 목적과 관계없는 파일이 섞임 | stage 후 diff를 계획·STATUS·0001·20/30 review·unicode evidence·unicode fixture·전용 font/license로 제한한다. |
| 5 | 기존 사전 비교 불변 | 기존 C15.1/C15.2 oracle 원본이 새 값으로 바뀜 | 기존 evidence와 fixture 경로를 수정하지 않았다. |
| 6 | 공식 상태 문서 경계 | STATUS의 78개 상위 체크 항목 수나 C15 완료 checkbox가 우연히 변함 | C15 parent와 C16/C19 상태는 미완료로 두고 상위 checklist 수를 조정하지 않는다. |
| 7 | 상태 링크 연결 | C15 row가 새 oracle/계획 검토를 찾을 수 없음 | STATUS의 C15.3 oracle, 계획, plan-review 20 링크를 함께 추가한다. |
| 8 | conformance 링크 연결 | 0001이 C15.3 결과를 제품 지원처럼 기술하거나 근거 링크가 없음 | 0001은 Chrome DOM 사전 비교와 아직 확인하지 않은 runtime/simulator 경계를 함께 적는다. |
| 9 | 내부 계약 version | 문서-only amendment가 내부 0070의 0.1.0 버전을 불필요하게 올림 | 0070이나 다른 numeric contract version은 변경하지 않는다. |
| 10 | 지원/API 카탈로그 | 새 fixture가 공개 API나 지원표에 자동 추가됨 | public API 목록·supported API·JS/CSS catalog는 변경하지 않는다. |
| 11 | 계획 경로 충돌 | 두 C15.3 계획 설명이 서로 다른 spec/status로 갈라짐 | 현재 plan/c15-inline-text-layout.md를 기준으로 links를 이어 붙이고 별도 계획 파일을 만들지 않는다. |
| 12 | 날짜와 artifact 식별 | 경로 날짜·문서 제목·검토 파일의 날짜가 서로 안 맞음 | 모든 새 evidence 경로와 두 review filename은 2026-10-17로 고정한다. |
| 13 | 새 fixture의 역할 | HTML oracle가 제품 fixture runner에 자동 포함된다고 오인 | 정적 browser precomparison이며 runtime route나 Cargo module 선언을 추가하지 않는다. |
| 14 | default build 영향 | fixture fonts가 product runtime asset에 자동 포함됨 | build/include/Cargo source 변경이 없다. 단, 경로 배치만으로 artifact 제외를 입증했다고 주장하지 않고 향후 구현 PR에 positive/negative artifact test를 남긴다. |
| 15 | asset filename collision | 기존 font 또는 oracle input이 새 Noto 파일로 덮임 | c15-3 전용 resource directory와 unique fixture filename을 사용한다. |
| 16 | binary source reproducibility | Noto font download source가 moving branch HEAD를 가리킴 | 세 font URL은 GitHub commit-pinned upstream file path를 사용한다. |
| 17 | license carry-through | font binary만 들어가고 재배포 조건이 빠짐 | font별 OFL text를 같은 폴더에 두고 upstream license provenance와 SHA-256을 기록한다. |
| 18 | docs language | 새 authored docs가 프로젝트의 Korean 문서 규칙을 위반 | resource README도 한국어로 작성했고 원본 license text만 upstream English 그대로 둔다. |
| 19 | evidence HTML link | README의 상대경로가 checkout 내부 fixture를 찾지 못함 | 새 fixture와 font README의 local links를 검증해 모두 존재했다. |
| 20 | screenshot embed path | README가 존재하지 않는 image 또는 외부 임시 path를 embed | chrome155.png를 같은 evidence directory에 저장하고 README의 상대경로로 embed한다. |
| 21 | screenshot metadata | PR image가 축소/crop된 파일로 바뀌어 SHA와 화면이 달라짐 | 저장된 PNG는 640×760 RGB, SHA-256 ba827180aac697b472b771b801102744c90b13f570816986c2d99c8d51810417이다. |
| 22 | observation metadata | browser capture JSON이 hash/UA/device 조건 없이 수치만 저장 | JSON hash, full Chrome version, platform, architecture, viewport, DPR와 font checks를 evidence README가 가리킨다. |
| 23 | fixture identity | JSON/screenshot이 현재 HTML 변경 전의 다른 fixture에서 나옴 | HTML SHA-256 afa39ab1cf879a2bfe1bcebd96c88600e8d720cb97cc4ef0eeedd0b5e56fa82a를 evidence에 기록했고 capture 후 HTML은 수정하지 않았다. |
| 24 | screenshot artifact hash | 이미지 bytes가 문서에 적힌 digest와 다름 | chrome155.png의 SHA-256 ba827180aac697b472b771b801102744c90b13f570816986c2d99c8d51810417을 다시 계산해 일치시켰다. |
| 25 | observation artifact hash | 수치 JSON이 문서에 적힌 digest와 다름 | chrome155-observation.json SHA-256 41dbe579530fb9fef963ab8915f4523c6d31534815469cf88c28d751701e4832를 다시 계산해 일치시켰다. |
| 26 | WPT source durability | WPT 링크가 branch tip이나 unpinned file에 연결됨 | 관련 후보 path 네 개는 같은 immutable WPT commit SHA를 가리킨다. |
| 27 | WPT wording | source link가 실행 결과나 conformance claim으로 읽힘 | WPT는 이 amendment에서 실행하지 않은 future candidate로 명시한다. |
| 28 | browser process lifecycle | PR 마무리 뒤 Agent Browser나 local HTTP server가 방치됨 | C15 전용 Chrome user-data-dir의 process와 port 8876 server가 닫혔다. 별도 hwpjs-alpha-demo session을 쓰는 공유 Agent Browser daemon은 사용 중이므로 종료하지 않았다. Terminal Browser는 사용하지 않았다. |
| 29 | 실행 주장 정확성 | 문서 diff만 검토한 것을 Rust tests, WPT, AVD 또는 iOS simulator pass라고 보고 | 이번 diff에서 git diff --check와 신규 local links/hash inspection만 수행한다. product tests와 Simulator runtime은 실행했다고 쓰지 않는다. |
| 30 | review artifact 분리 | plan review 20과 PR diff review 30이 한 파일/한 횟수로 합산됨 | 두 review는 별도 evidence 파일이다. 이 30-point review는 실제 stage된 PR diff를 확인한 뒤 마지막으로 갱신한다. |

## 검토 시점 한계

현재 browser precomparison은 Agent Browser가 실행한 Chrome DOM 화면·geometry oracle다. PR에는 원본 screenshot도 포함되며 description에 branch-pinned image를 첨부할 수 있다. 실제 Spinon runtime, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator, WGPU glyph paint, parent-border 바깥 ink 및 artifact positive/negative control은 후속 implementation gate다.
