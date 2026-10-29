# PR #807 적대적 diff 검토 30개

검토 대상은 PR #807의 `main...feat/c11-3-3-area-auto-tracks` 변경이다. 구현/runtime 20개와 별도 diff 20개를 재사용하지 않고 merge 설정, 계약·fixture 경계, 두 플랫폼 route, 실제 결과와 문서 연결을 각각 확인했다. 이 기록은 GitHub check run이 없다는 사실을 CI 성공으로 바꾸지 않는다.

| # | 실패 관점 | 확인과 근거 | 판정 |
|---:|---|---|---|
| 1 | 구현 branch가 PR base와 잘못 연결되거나 worktree로 갈라짐 | PR #807 base는 `main`, head는 `feat/c11-3-3-area-auto-tracks`; 이 변경은 기존 `/Users/yoonhb/Documents/workspace/spinon` checkout에서 만들었다. merge-base는 계획 PR #806 commit `ca8542150`이다. | PASS |
| 2 | unrelated staged code 또는 다른 feature가 implementation diff에 섞임 | PR diff를 source, docs, fixture, Chrome/Android/iOS C11.3.3 evidence로 분류했다. 별도 untracked C04/C11.4 evidence와 `vendor/taffy/target`은 commit에 포함되지 않았다. | PASS |
| 3 | PR 분류 라벨이 누락 | 기존 라벨 `enhancement`, `area: layout`, `area: android`, `area: ios`, `area: runtime`이 PR #807에 있다. | PASS |
| 4 | PR 본문이 필수 변경/명세/근거/남은 범위를 생략 | 저장소 PR 양식의 3개 절과 C11.3.3 candidate ID, 제한 profile, 테스트, 플랫폼 경계가 본문에 있다. | PASS |
| 5 | PR 화면 screenshot 링크가 깨지거나 text link만 남음 | 최종 PR 본문에서 Agent Browser의 `img.complete=true`, natural size 1206×2622를 확인했고, image URL은 공개 commit-pinned raw path다. 사용 후 browser를 닫았다. | PASS |
| 6 | required checks가 pending인데 merge 가능으로 오인 | `gh pr checks --required`는 `no checks reported`, PR `statusCheckRollup`은 빈 목록이다. branch protection에는 required status check가 없고 ruleset도 없다. 이 상태는 CI pass가 아니다. | PASS |
| 7 | official completion count가 구현 후보만으로 증가 | `spec/STATUS.md`, plan, conformance 문서가 `72/213`과 C11 parent 미완료를 유지한다. | PASS |
| 8 | limited profile candidate가 public CSS Grid/WPT 지원으로 표현 | plan·0059·conformance·PR 본문 모두 opt-in `RuntimeBlockFlexGridTemplateV1`, simulator-only, WPT·실기기·hardware 한계를 적었다. | PASS |
| 9 | internal interface version을 올려 호환 계약을 잘못 시사 | 0059 numeric version은 `0.1.0` 그대로다. | PASS |
| 10 | 계획 PR과 implementation PR 순서가 없어짐 | 계획 PR #806은 먼저 merge됐고 별도 구현 PR #807이 그 뒤에 열렸다. | PASS |
| 11 | 기존 Chrome 154 기준을 보충 관측으로 덮음 | 기존 26 scene·65 node의 HTML/inventory/reference는 수정하지 않았다. generated matrix도 reference 관측 26개를 별도로 검사한다. | PASS |
| 12 | Chrome 156 supplemental scene 수·ID·순서가 고정되지 않음 | 새 5개 scene ID와 DOM order가 supplemental HTML 및 observation에서 고정되고 node preorder가 일치한다. | PASS |
| 13 | screenshot/CSSOM oracle가 다른 fixture bytes를 가리킴 | supplemental observation fixture path와 SHA가 builder에서 HTML bytes의 SHA와 일치해야 한다. screenshot 및 원시 Agent Browser eval envelope도 evidence에 있다. | PASS |
| 14 | 생성된 runtime JSON이 수동 변경으로 oracle에서 이탈 | `build-c11-3-3-grid-template-areas-runtime-matrix.mjs --check`가 committed matrix bytes를 oracle 입력에서 다시 만든 출력과 비교한다. 이 실행은 31 scene·90 node를 확인했다. | PASS |
| 15 | runtime count가 matrix·FFI·capture 집계에서 다름 | 전 scene 집계는 success 25, cascade fallback 5, expected rejection 1이다. FFI matrix validator와 Android/iOS capture expected counts가 같은 값을 요구한다. | PASS |
| 16 | fixture FFI가 기본 artifact로 새거나 opt-in positive control에서 빠짐 | 기본 archive는 fixture symbol 0개·payload 없음, `c04-runtime-gpu-fixture-test` archive는 symbol 2개·payload 포함이다. archive hash와 toolchain은 [artifact boundary record](./ffi-artifact-boundary.json)에 있다. | PASS |
| 17 | 기존 dimension mismatch rejection을 새 성공 경로가 잘못 삼킴 | `area-track-count-mismatch`는 Chrome 154 기준에 맞는 success geometry 대조군으로 바뀌고 마지막 cell의 `(160,0,0,60)` 기준을 유지한다. | PASS |
| 18 | Android activity가 신규 5 scene을 허용하지 않음 | `MainActivity` 및 C0410 demo allowlist가 보충 5 ID를 등록했다. 새 Android matrix 로그 5개가 실행되고 결과가 `matched`다. | PASS |
| 19 | iOS demo가 신규 5 scene 또는 scene별 oracle을 누락 | iOS scene set과 capture 경로가 5 ID와 `scene.oracle` override를 사용한다. 기존 26-scene reference 길이는 별도 고정한다. | PASS |
| 20 | expected rejection logger가 구 rejection code를 계속 요구 | Android/iOS reject route는 unknown implicit area sizing의 `unsupported_grid_auto_rows`/`grid-auto-rows`와 frame 미생성을 요구한다. runtime result에 기대 거부 1건이 있다. | PASS |
| 21 | `grid-template:none`가 areas 없이도 허용 | cascade test는 areas가 있을 때만 empty track list를 받아들이고, 동일 `none` 입력에서 areas를 제거하면 오류가 나는지 확인한다. | PASS |
| 22 | 두 축을 동시에 늘릴 때 generated track 수·값이 다름 | `area-auto-tracks-none`과 projection assertion이 rows/columns 양축의 area 크기, `grid-auto-rows` 7px, `grid-auto-columns` 9px를 검사한다. | PASS |
| 23 | 한 축만 짧거나 authored track prefix가 덮임 | `short-row`/`short-col` Chrome scene은 각각 12px row 또는 13px column authored prefix 뒤에만 missing track을 추가하고 실제 frame을 비교한다. | PASS |
| 24 | area보다 긴 authored list가 축소됨 | `area-auto-tracks-long`과 Rust regression test는 3×3 authored tracks를 보존하고 area의 첫 2×2 item 위치를 검사한다. | PASS |
| 25 | scalar `auto` track의 content sizing이 px로 고정되거나 반대 축에 적용 | Chrome observation의 auto scene은 50×30 root에서 rows 10/20px·columns 20/30px used size와 모든 item frame을 기록한다. Android/iOS도 같은 값을 냈다. | PASS |
| 26 | unsupported auto track list 또는 unsupported profile이 새 예외를 우회 | auto-repeat은 TrackSizing profile gate를 그대로 쓰며, 단일 `auto`/px 밖의 generated `grid-auto-*` TrackList는 typed unsupported error로 닫는다. | PASS |
| 27 | expanded line-name groups 및 area metadata가 크기 제한/할당 실패를 우회 | combined metadata budget은 생성 line-group headers를 포함한다. 1 MiB 경계 테스트와 fallible reserve 오류 경로가 있으며 style-to-layout suite 255개가 통과했다. | PASS |
| 28 | 기본 Rust/layout/reference gate가 최종 변경을 놓침 | 기록된 로컬 결과: layout 306 passed, style-to-layout 255 passed, fixture FFI matrix 1 passed, cascade focused 1 passed, CSS reference 24/24, rustfmt 및 diff check 통과. 이들은 GitHub CI와 구별된다. | PASS |
| 29 | Android geometry, pixels, route negative control이 성공처럼 보이지만 불일치 | API 37 ARM64 AVD/SwiftShader 결과 `matched`; visible 88 frame 최대 0 CSS px, PixelCopy 2/2 exact, overflow 3/3, leading-line 7/7 exact, 잘못된 sample/scene은 readback/draw 전 거부. | PASS |
| 30 | iOS 결과를 compositor/device proof로 확대하거나 review 후 scope 문서가 낡음 | iPhone 17 Pro/iOS 26.2 Simulator 결과 `matched`; frame 오차 0 CSS px, Metal 2/2, overflow 3/3, leading-line 7/7 exact. 문서가 Metal compositor 전 한계와 실기기·hardware·WPT 제외를 명시한다. | PASS |

## 판정

30개 관점은 PASS다. PR #807은 open이며 merge되지 않았다. GitHub check run은 보고되지 않았고, branch protection에 required status check가 설정되지 않았다. repository 규칙에 맞춰 `Rebase and merge`만 허용되는지 최종 확인한 뒤 merge state를 다시 읽어야 한다.
