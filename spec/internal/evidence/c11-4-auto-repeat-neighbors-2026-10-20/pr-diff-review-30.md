# C11.4 explicit-neighbor auto-repeat PR diff 적대 검토 30개

**대상:** 계획 PR #361 merge base `9f66a41c540f1ffeece7ba65d3b684d7bddf128e`부터 `feat/c11-4-auto-repeat-explicit-neighbors-20261020`의 구현·fixture·명세·실행 근거 diff까지. 구현 리뷰와 별개로 pull request 변경의 위험을 30개 관점에서 확인했다.

| # | PR diff 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | 변경이 약속한 fixed `px` explicit neighbor 범위를 넘어 공개 CSS 지원으로 읽히는가? | **통과.** plan, conformance, internal contract 및 STATUS 모두 제한 후보라고 명시하고 C11.4 미완료를 유지한다. |
| 2 | 공식 C11.4 checkbox나 완료 수를 조용히 올리는가? | **통과.** `spec/STATUS.md`의 `[ ]`와 공식 집계 입력은 수정되지 않았다. |
| 3 | 공식 상태, 내부 계약, plan, public roadmap이 서로 다른 scene 수나 결과를 말하는가? | **통과.** 13-case Chrome/Rust와 양 simulator 각각 13/13으로 일치시켰다. |
| 4 | 이전 3-scene 실행 기록을 소급해 전체 13-scene 실행처럼 고쳤는가? | **통과.** 3-scene 이전 기록은 역사 값으로 남기고 2026-10-20 후속 기록만 13/13으로 갱신했다. |
| 5 | internal contract가 overflow를 border-stroke pixel parity로 과장하는가? | **통과.** root `40px`, item `x=86px` geometry와 screenshot 표시로 한정하고 border stroke 및 hardware GPU를 미검증으로 적었다. |
| 6 | 구현 전 Chrome fixture/고정 revision과 구현 후 관찰값이 연결되는가? | **통과.** evidence README가 사전 비교 README와 13개 고정 case를 연결하고 허용 오차를 기록한다. |
| 7 | Chrome observation을 WPT pass나 독립 표준 적합성으로 잘못 확대하는가? | **통과.** WPT를 실행하지 않았고 Chrome은 oracle 관찰값이라고 구분한다. |
| 8 | track sizing 결과가 최종 값만 있어 empty/collapse 상태를 검토할 수 없는가? | **통과.** final used track과 per-track collapse flag를 Android/iOS result JSON에 저장한다. |
| 9 | 화면 frame 판정이 root/item 크기만 보고 offset·다른 node를 누락하는가? | **통과.** 13 scene 각각에서 전체 `node_frames_css_px` 문자열을 검사하고 보존한다. |
| 10 | 앱이 trace를 일부만 출력해 최종 track 판단을 재현할 수 없는가? | **통과.** capture script가 전체 trace chunk 수·번호를 확인한 뒤 JSON을 복원하며 run별 runtime log도 저장한다. |
| 11 | 표시 완료 전에 draw 요청만 확인해 simulator 화면을 pass 처리하는가? | **통과.** WGPU draw marker와 `presented boxes=N` 완료 marker를 모두 요구한다. |
| 12 | overflow 화면 link가 잘못되었거나 실제 실행 screenshot이 아닌가? | **통과.** Android AVD와 iOS Simulator 각 13-scene PNG가 run directory에 있고 overflow 두 화면을 README에서 직접 표시한다. |
| 13 | screenshot 이미지가 PR branch에서 접근 가능한 committed path를 가리키는가? | **통과.** PR 본문은 branch push 뒤 `raw.githubusercontent.com` 경로를 사용하며 화면 원본은 evidence directory에 포함한다. |
| 14 | Android 결과가 실기기와 AVD를 혼동하거나 기기 식별값이 빠지는가? | **통과.** capture guard는 `ro.kernel.qemu=1`, API 37을 요구하며 serial·model·fingerprint를 기록한다. |
| 15 | iOS 결과가 실기기나 다른 OS/runtime 이미지로 대체되는가? | **통과.** capture guard는 booted iPhone 17 Pro 및 iOS 26.2 runtime을 요구하고 UDID·runtime을 기록한다. |
| 16 | build 재현 명령이 worktree 안의 없는 V8 source를 가리키는가? | **통과.** README는 기존 checkout의 고정 V8 source revision을 `SPINON_V8_DIR`로 읽기 전용 지정하는 절차를 적는다. |
| 17 | 재현 절차가 fixture feature flags를 빠뜨려 앱에 장면이 안 들어가는가? | **통과.** iOS/Android build 명령에 runtime 및 fixture-test flags가 모두 포함된다. |
| 18 | evidence hash가 README 또는 runtime files와 분리되어 나중에 바뀐 screenshot을 놓치는가? | **통과.** `SHA256SUMS`는 run JSON/log/screenshot, simulator metadata, review docs와 README를 모두 고정한다. |
| 19 | 비어 있는 성공 로그를 빌드 성공 근거로 오인하는가? | **통과.** 플랫폼 화면 run의 빈 `install.log`는 simulator install CLI의 stdout이 없다는 기록이며 README가 build 재현 명령과 runtime 결과를 분리한다. |
| 20 | root-only scene을 item frame assertion 생략 이유로 전체 검증에서 빼는가? | **통과.** empty/max/min scene은 정확한 root-only node-frame 배열과 track trace를 검사하며 `boxCount=1`을 확인한다. |
| 21 | two-track repeated pattern의 fragment order와 마지막 suffix 경계가 빠지는가? | **통과.** `auto-fill-pattern-neighbors` trace와 suffix item frame이 pattern 및 trailing fixed track 위치를 확인한다. |
| 22 | auto-fit empty 또는 span occupancy에서 suffix line이 틀려도 검출되지 않는가? | **통과.** empty, single occupied, two-track span, suffix item 결과가 별도 고정 case로 검사된다. |
| 23 | definite min/max와 indefinite sizing의 반복 수 기준이 빠지는가? | **통과.** min-size, max-size, max-content indefinite case가 각각 root width와 full track list를 고정한다. |
| 24 | row 경로만 columns code의 단순 복제로 검사하지 않아 축 매핑 실수를 놓치는가? | **통과.** row auto-fill/auto-fit 장면은 final Rows trace와 y-offset/node frame을 별도로 검사한다. |
| 25 | expected scene source가 외부 입력 문자열을 JavaScript로 삽입하는가? | **통과.** fixture source는 `&'static str` allowlist만 사용하며 request parameter는 고정 scene ID로 조회한다. |
| 26 | fixture route 변경이 제품 ABI/default artifact를 불필요하게 확장하는가? | **통과.** 새 scene source는 기존 opt-in C11.4 fixture 경로에만 추가했고 Cargo feature·제품 ABI 선언 변경은 없다. |
| 27 | Android/iOS host allowlist가 불일치해 한 플랫폼만 scene을 실행하는가? | **통과.** 양쪽 route와 root/item size mapping에 같은 13개 ID가 들어 있고 simulator 결과가 모두 13/13이다. |
| 28 | 실패한 scene의 stale log가 새 실행 결과처럼 소비될 수 있는가? | **통과.** 매 scene마다 고유 run ID와 새 process를 사용하고 process/PID 및 scene/run marker를 대조한다. |
| 29 | PR 문서 변경이 다른 Grid/roadmap 업무 또는 사용자 변경을 포함하는가? | **통과.** 변경 경로는 C11.4 explicit-neighbor 구현·fixture·계획/상태·근거로 한정된다. 원 checkout의 미커밋 C11.3 작업은 이 worktree diff에 없다. |
| 30 | 최종 PR에서 필요한 검토 근거, 라벨, screenshots 및 merge gate가 누락되는가? | **통과 조건 기록.** 이 diff report와 implementation report를 PR에 포함한다. PR 생성 뒤 screenshot markdown 렌더링, `enhancement`/`area:*` 라벨, live required checks와 merge state를 별도 확인한 뒤에만 merge한다. |

## 결과

코드·문서·실행 근거 diff에서 차단 결함은 찾지 못했다. #30의 GitHub PR 렌더링, 라벨 및 live checks는 PR 생성 뒤 별도로 확인해야 하며, 이 보고서만으로 merge-ready라고 판정하지 않는다.
