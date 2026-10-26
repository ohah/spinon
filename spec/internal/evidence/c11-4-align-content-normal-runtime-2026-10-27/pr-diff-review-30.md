# C11.4 균등 auto 행 구현 PR diff 30관점 검토

검토 대상은 `align-content-normal-equal-auto-rows` fixture와 그 Android/iOS matcher, 캡처 runner, 사전 비교, 계획 및 런타임 근거 diff다. 계획 검토와 구현/runtime 20관점 검토를 되풀이하지 않고 scene 격리, 계산, revision/presentation identity, 플랫폼 pixel 표본, artifact 및 공개 완료 경계를 나눠 확인했다. 검토 중 발견한 복수 scene 선택 우회는 runner에서 막고, 음성 CLI 입력으로 시뮬레이터 명령 전에 거부되는 것을 확인한 뒤 두 플랫폼을 재실행했다.

| # | 독립 실패 경로 | 결과와 근거 |
|---:|---|---|
| 1 | scene ID suffix가 새 fixture에 진입하는가 | **통과** — Rust dispatcher는 exact ID만 admission한다. |
| 2 | 임의 JavaScript나 unknown ID가 고정 scene allowlist를 우회하는가 | **통과** — allowlist test는 suffix와 unknown ID 거부를 확인한다. |
| 3 | scene tree에 예상 밖 node 또는 자식 수가 생기는가 | **통과** — 정적 source와 Rust test는 root 1개와 child 3개의 순서·크기를 고정한다. |
| 4 | alignment·track·gap 선언이 비교 입력과 달라지는가 | **통과** — CSS/app source는 runtime에서 허용된 `normal`/`start`, `auto auto 5px`, `row-gap:10px`이고 Rust oracle은 typed `FlexStart`다. 이 x축 차이는 여유 공간이 없는 고정 column이므로 row-axis geometry에 영향을 주지 않는다. |
| 5 | 이 변경이 일반 CSS admission이나 production parser를 넓히는가 | **통과** — 변경은 기존 opt-in C11.4 scene dispatch와 host fixture allowlist 안에 한정된다. |
| 6 | app containing block 차이를 root/viewport parity로 과장하는가 | **통과** — viewport `320×240`, root `160×100` 비교 범위와 Rust oracle의 별도 viewport가 계획·evidence에 기록돼 있다. |
| 7 | expected track 결과가 Spinon output에서 역산되는가 | **통과** — 독립 계산에서 남는 `45px`을 두 auto row에 `22.5px`씩 배분한다. |
| 8 | Chrome과 앱의 root/gutter/outside background 차이가 pixel 판정에 숨는가 | **통과** — Chrome fixture 배경을 app sample과 일치시키고 5개 RGBA를 CSSOM expectation에 고정했다. |
| 9 | 초기화 stage가 누락되거나 잘못된 base로 시작하는가 | **통과** — `Initialize` row base `[0,0,5]`를 모든 검사기가 확인한다. |
| 10 | intrinsic·maximize·flex expansion stage가 빠지거나 순서가 바뀌는가 | **통과** — 여섯 Rows stage의 exact name, count, order를 runner·Android·iOS가 각각 확인한다. |
| 11 | 다른 Grid trace나 중복 Rows pass가 결과를 만족시키는가 | **통과** — trace는 root node 1, 한 grid, Columns/Rows 두 pass, Rows pass 하나를 요구한다. |
| 12 | 7개 alternating track/gutter 내부 index가 바뀌는가 | **통과** — 각 entry의 index, gutter parity와 collapsed edge를 확인한다. |
| 13 | 두 내부 gutter가 row size에 더해지거나 10px을 벗어나는가 | **통과** — 두 gutter 모두 별도 10px entry로 검증된다. |
| 14 | `IntrinsicItemBatch` 뒤 stage별 row bases가 잘못되는가 | **통과** — 이후 stretch 전 네 stage는 `[10,20,5]`를 유지한다. |
| 15 | 마지막 fixed row가 auto-row stretch에 참여하는가 | **통과** — 최종 `[32.5,42.5,5]` 비교가 마지막 5px을 고정한다. |
| 16 | NaN·무한대·부동소수 오차가 허용 범위 안인 것처럼 보이는가 | **통과** — host matcher는 finite 값을 요구하고 tolerance는 `1e-4 CSS px`다. |
| 17 | report root/node identity 또는 child frame 순서가 섞이는가 | **통과** — root와 세 child의 node ID·좌표·크기를 순서대로 대조한다. |
| 18 | trace revision이 layout report와 다른 문서 세대에서 왔는가 | **통과** — style/environment 및 HostDocument generation/document/render-tree revision을 연결한다. |
| 19 | Android environment 결과와 presented draw가 다른 surface generation인가 | **통과** — root/viewport/box 수, nonempty surface, generation·sequence identity를 묶는다. |
| 20 | iOS environment 결과와 draw presentation이 다른 surface generation인가 | **통과** — 동일 revision identity와 surface generation·sequence를 비교한다. |
| 21 | runner가 새 scene을 다른 scene 여러 개와 함께 실행하는가 | **통과** — 대상 ID 포함 복수 선택을 simulator command 전 거부했고 [음성 입력 기록](./runner-multi-scene-selection-rejected.log)을 보존했다. |
| 22 | target ID 없이 해당 run-kind만 지정해 전체 matrix가 도는가 | **통과** — run-kind만 지정한 음성 입력도 simulator command 전 거부했고 [기록](./runner-run-kind-without-scene-rejected.log)을 보존했다. |
| 23 | Android 실행이 다른 API·ABI 또는 설치 APK를 사용했는가 | **통과** — API 37 ARM64 AVD이며 build/installed APK SHA-256이 같다. |
| 24 | iOS 실행이 다른 Simulator 모델·앱 설치본을 사용했는가 | **통과** — `iPhone18,1` iPhone 17 Pro이며 build/installed executable SHA-256이 같다. |
| 25 | runtime 결과가 실패·빈 draw 상태인데 screenshot만 성공으로 읽히는가 | **통과** — 두 최종 `result.json`에 한 scene, `drawPresented=true`, presentation 검증 pass가 기록됐다. |
| 26 | Android PixelCopy sample ID/node/좌표 수가 다른 조합을 통과하는가 | **통과** — 다섯 sample allowlist의 중복·누락을 거부하며 최종 PixelCopy는 5/5 exact다. |
| 27 | iOS Metal sample이 잘못된 node나 RGBA로 통과하는가 | **통과** — 동일 다섯 표본을 node·좌표·RGBA까지 검사하고 최종 draw log와 일치한다. |
| 28 | gutter/outside-root sample이 실제 경계 위치를 판별하지 못하는가 | **통과** — `y=37`은 두 row 사이, `x=160`은 root 오른쪽 경계 밖이며 둘 다 viewport 안이다. |
| 29 | default/fixture artifact 또는 evidence checksum이 반대로 기록되는가 | **통과** — default/positive-control scan, 설치 hash 및 6개 사전 비교 SHA-256을 확인했다. static scan 범위 제한도 분명히 적었다. |
| 30 | 제한 fixture가 공식 C11.4 완료로 표시되거나 검증 한계를 감추는가 | **통과** — `spec/STATUS.md`의 공식 C11.4 checkbox와 `72/213`을 바꾸지 않았고, 실기기·WPT·일반 CSS admission·전체 C11.4 미검증을 기록한다. |

**결과:** 최종 diff 30/30 통과, 미해결 blocker 없음. PR 본문에 커밋 SHA 고정 raw screenshot 두 개를 연결했고, PR 생성 후 Agent Browser에서 두 이미지가 모두 로드되는 것을 확인했다(Android `1080×2400`, iOS `1206×2622`). Agent Browser 세션과 daemon은 확인 뒤 종료했다.
