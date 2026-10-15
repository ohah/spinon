# C13.3 Android AVD touchpad raw-point PR delta 검토 · 30개 관점

PR에 실릴 최종 변경 후보를 계획, Android Java/Gradle 경계, evidence README/log/screenshot, 공식 상태와 public roadmap 연결로 나누어 읽었다. 각 항목은 서로 다른 사용자·운영 실패를 검사한다.

| # | PR 변경 실패 관점 | 검토와 판정 |
|---:|---|---|
| 1 | PR 설명이 ADB command를 실제 touchpad hardware sample로 확대하는가 | ACTION_SCROLL은 Android API 37 AVD input shell이 만든 event라고 한정한다. |
| 2 | root cause가 단순 ADB 연결 오류로 기록되는가 | raw point (0,0)과 SurfaceView origin (15,991)의 좌표 차이 및 queue 전 reject를 설명한다. |
| 3 | product code가 cursor fallback을 추가한 것으로 오해되는가 | event point는 수정하지 않고 test-only view mapping을 적용했다고 쓴다. |
| 4 | test mode가 제품 UI 상태처럼 보이는가 | screenshot caption에 surface 이동과 crop이 fixture 전용임을 명시한다. |
| 5 | 기본 surface failure control이 빠지는가 | default surface before/after image와 identical SHA-256, point-outside-surface log를 함께 연결한다. |
| 6 | before/after 비교가 기기나 화면 크기가 다른가 | 두 PNG는 같은 emulator-5554 1080×2400 AVD에서 캡처했다. |
| 7 | positive screenshot과 positive log가 서로 다른 run인가 | 같은 opt-in build/install 및 연속 callback run의 screenshot/log를 evidence 폴더에 둔다. |
| 8 | 화면이 보여 주는 상태가 queue admission과 연결되지 않는가 | screenshot status와 log sequence 1, target 7, revision 2를 짝지었다. |
| 9 | source/action을 tool name 추정만으로 판단하는가 | source 0x100008, action 8, pointer count/tool을 raw callback log에 남긴다. |
| 10 | delta 단위 변환을 source event에 대해 입증하지 않는가 | factor 168 physical px, scale 2.625, CSS delta 64를 같은 callback에서 기록한다. |
| 11 | CSS target이 Chrome oracle와 무관한 임의 지점인가 | 기존 fixed fixture target (132,112)와 일치시키고 오차를 0.5 CSS px 이내로 제한한다. |
| 12 | parent/child scroll state의 변경을 과장하는가 | root 0, inner (86,106), outer X 64로 정확히 나눠 적었다. |
| 13 | feature-off artifact 근거가 소스 경로만 보는가 | default APK의 실제 DEX scan과 APK hash, opt-in positive control을 기록한다. |
| 14 | opt-in flag가 빌드 조건과 intent 조건을 혼동하는가 | 두 build flag와 runtime gesture/extra 조건을 따로 설명한다. |
| 15 | helper 소스가 default 앱 코드에 섞이는가 | conditional source set 경로와 flag 값별 BuildConfig/log를 연결한다. |
| 16 | 초기 artifact boundary 결함을 숨기는가 | inline lambda와 unguarded MotionEvent diagnostics의 default DEX 유입을 발견해 helper는 opt-in source set으로, 진단은 compile-time flag로 옮긴 수정과 재검증을 기록한다. |
| 17 | 이전 screenshot byte hash를 최신 캡처에 적용하는가 | 최신 default screenshot hash f186a078…와 동일성을 README에 기록한다. |
| 18 | intermediate build log가 최종 build로 오인되는가 | 최종 default/opt-in build log를 각각 연결하고, 초기 DEX 누출 실험은 discovery note로 따로 표시해 최종 성공 build와 혼동하지 않게 한다. |
| 19 | iOS host 차단을 제품 실패 또는 pass라고 쓰는가 | iOS callback 미관찰을 명시하고 simulator gate를 미완료로 둔다. |
| 20 | 실기기 또는 hardware GPU proof를 함축하는가 | AVD/QEMU/API 37/ARM64를 명시하고 physical input·hardware GPU·compositor는 남긴다. |
| 21 | C13.3 공식 checkbox가 증거만으로 체크되는가 | 상위 C13.3 상태는 미완료로 유지하며 descriptive note만 고친다. |
| 22 | 계획과 실행이 서로 다른 target/scale을 쓰는가 | parent plan, follow-up plan, README의 target, origin, factor, revision을 대조했다. |
| 23 | internal index에서 실행 증거를 찾지 못하는가 | C13.3 internal row에 README 링크를 추가한다. |
| 24 | public roadmap이 이전 fixed-origin 오류 설명을 유지하는가 | 새 AVD fixture 결과와 실제 hardware 미검증을 함께 반영한다. |
| 25 | roadmap의 새 README.html이 이미지 없이 렌더되거나 이전 파일을 제공하는가 | 격리 Rspress 빌드에서 같은 디렉터리 이미지에 `./`가 필요한 오류를 수정했다. Tailnet roadmap 링크가 evidence route로 연결되고 Agent Browser에서 제목, scope 설명, embedded image 1080×2400과 최종 제공 hash를 확인했다. [Tailnet 동기화 기록](https://macstudio.tailed42f2.ts.net/spinon/spec/internal/evidence/c13-3-android-avd-touchpad-point-tailnet-sync-2026-10-16/README.html). |
| 26 | PR screenshot URL이 default branch에서 404가 되는가 | PR #212 body는 현재 feature branch의 raw GitHub URL을 사용한다. Agent Browser에서 실제 img `complete=true`, natural size 1080×2400과 visible 상태를 확인했다. |
| 27 | screenshot이 HTML alt/caption 없이 맥락을 잃는가 | alt text와 shifted test-only view 설명을 PR 본문에 넣는다. |
| 28 | 내부 계약 버전이 docs-only harness로 불필요하게 올라가는가 | 0063 contract version은 바꾸지 않는다. |
| 29 | ADB serial 생략으로 물리 Samsung에 입력이 가는가 | 모든 command와 evidence는 emulator-5554를 명시한다. |
| 30 | CI 상태나 merge mode가 PR 본문에 숨겨지는가 | PR #212는 `CLEAN`/`MERGEABLE`이고 `gh pr checks`에 보고된 check가 없다. workflow inventory에는 수동 `workflow_dispatch`만 있으며 pull-request CI는 없고, main required-status-check endpoint는 404(미설정), ruleset 목록도 비었다. 프로젝트 규칙에 따라 Rebase and merge를 사용한다. |

PR #212의 실제 body image, Tailnet roadmap/evidence page, 58개 제공 파일의 hash, artifact·상태 diff를 확인했다. Tailnet sync는 기존 파일 8개를 백업하고 삭제 없이 적용했으며 상세 결과는 [sync evidence](c13-3-android-avd-touchpad-point-tailnet-sync-2026-10-16/README.md)에 있다. PR에는 required CI가 구성되어 있지 않아 mergeability를 다시 확인한 후 저장소 규칙대로 Rebase and merge한다.
