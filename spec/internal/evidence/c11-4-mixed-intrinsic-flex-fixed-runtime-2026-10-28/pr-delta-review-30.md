# C11.4 mixed intrinsic/flexible/fixed PR 변경 실패 관점 30개

구현 diff와 신규 근거를 병합 전 별도로 검토했다. 각 항목은 서로 다른 계약 경계 또는 회귀 경로를 확인한다. 범용 CSS Grid 완료를 주장하지 않으며 C11.4와 공식 `72/213`은 미완료 상태로 유지한다.

| # | 변경 실패 가설 | 검토 결과 |
| ---: | --- | --- |
| 1 | 신규 FFI scene ID가 기존 fixture ID와 충돌한다 | 정확한 8개 고유 ID와 unknown ID negative test를 대조했다. 충돌은 보이지 않는다. |
| 2 | 아이템 DOM source order와 고정 grid placement가 모순된다 | forward/reversed source와 명시 placement를 교차 확인했다. 둘 다 같은 track base를 낸다. |
| 3 | 생성된 root dimensions/gap이 독립 기대값과 다르다 | columns `300×120`, rows `120×300`, target gap 10px를 `expected.json`과 대조했다. |
| 4 | CSS track 문자열이 min-content, 1fr, 40px, 0px, 2fr 의미를 잃는다 | 생성 source와 runtime adapter typed track equality를 대조했다. |
| 5 | text 길이가 equality 경계를 벗어나지만 다른 scene label을 사용한다 | 4/8/10 Ahem glyph 정의와 Rust expected frames·verifier table이 일치한다. |
| 6 | row-axis text가 수직 writing mode로 변환되지 않는다 | Rows source/style에는 `vertical-rl`이 text 두 개에만 적용된다. row 결과는 `orthogonalText` 입력으로 매치됐다. |
| 7 | nowrap 수정이 공백 접기나 일반 whitespace profile까지 바꾼다 | intrinsic gate에서 ASCII 제한과 기존 style 조건을 유지하고 `NoWrap`만 추가했다. |
| 8 | nowrap 테스트가 기존 no-wrap 거부 테스트와 모순된다 | 기존 거부 case를 제거한 뒤 고정 ASCII/Ahem positive test로 교체했다. 일반 밖 profile은 제한으로 남겼다. |
| 9 | C11.4 projection이 root writing style을 무시한다 | 기존 root writing-style check는 유지된다. 단일 vertical paragraph 개수 검사만 제거했다. |
| 10 | `C11_4MixedInput` 필드명 변경으로 인접 fixture의 projection 선택이 뒤집힌다 | 모든 struct literal을 검색하고 legacy 입력은 `false`, cyclic span fixture만 `true`인 것을 확인했다. |
| 11 | 신규 adapter가 이전 generic spanning adapter에 가려진다 | prepare 순서와 정확한 track signature를 확인했고 runtime test는 신규 generic path에서 8 scene을 만들었다. |
| 12 | image natural size가 fixture CSS width/height에 의해 강제로 맞춰진다 | image는 `auto` dimensions이며 CSSOM/layout trace에서 decoded `24×16`이 확인된다. |
| 13 | image leaf에 자식 또는 다른 source를 허용한다 | URL와 childless image를 검사한다. 앱의 natural-size diagnostic도 양 플랫폼 matcher가 확인한다. |
| 14 | Root/item margins, padding, borders 또는 aspect ratio가 sizing에 섞인다 | adapter typed zero-edge, border, dimension, aspect-ratio 조건을 점검했다. |
| 15 | 상속 또는 작성된 정렬값이 fixture 오류 없이 item frame을 바꾼다 | root/item alignment 속성을 고정하고 양쪽 native matcher에서 비교한다. |
| 16 | auto tracks, auto repeat, line names or template areas alter the selected grid | corresponding computed fields are rejected unless they match the fixed scene. |
| 17 | intrinsic `source_order` is inferred from node ID instead of DOM order | runtime trace records item order; Android/iOS expect node IDs 2/4/5 and source index separately. |
| 18 | text nodes make native frame indexing appear contiguous when HostNode IDs skip | Android/iOS `expectedFrameNode` explicitly checks IDs `[1,2,4,5]`; 8 scene capture passed. |
| 19 | row-axis cross size is accidentally used as flex sizing size | verifier independently chooses target axis and observed row track bases match 300px sizing axis. |
| 20 | internal gutters are mistaken for CSS tracks or boundary sentinels | native matchers validate alternating gutter slots, collapsed sentinels, and the three CSS track values. |
| 21 | `ExpandFlexibleTracks` runs zero or multiple times but only the final frame passes | Rust test selects target pass and matcher verifies the stage trace; evidence includes nine/eighteen trace chunks by platform. |
| 22 | no-freeze and equality accidentally freeze at the same boundary as 100px input | 80px exact equality and 40px control both produce `[80,40,160]`; verifier checks both independently. |
| 23 | reversed DOM order changes item identity/frame association | reversed Android and iOS scenes pass per-node frames, intrinsic contribution and source-order diagnostics. |
| 24 | Android scene allowlist accepts mixed cases but expected count stays at two children | `itemCount`, box count, and root-frame matcher require three items/four total nodes for mixed scenes. |
| 25 | iOS scene allowlist computes box count from the previous two-item assumption | Swift `itemCount` drives expected box count; four node frames passed for every mixed scene. |
| 26 | native diagnostics omit intrinsic source kind or the replaced natural dimensions | Java and Swift matchers require text/orthogonalText, replaced image, `24×16`, and axis-specific min-content. |
| 27 | fixture flags collide with another C11.2.1 runtime profile | conflicting profile was rejected on both simulator apps. |
| 28 | unlisted scene name is silently routed to a fallback fixture | unknown scene launch/runtime controls were rejected on both platforms. |
| 29 | successful launch is reported as presentation even when no frame was presented | capture requires layout-ready, node frame/trace diagnostics, screenshot signature/dimensions and presented state; verifier rechecks these records. |
| 30 | screenshots or diagnostics are detached from the final source being reviewed | final run manifests record source/input and app hashes; screenshot hashes and manifest verification are included. PR will link commit-pinned files from these runs. |

확인 결과:

- `git diff --check` 통과.
- `cargo fmt --all -- --check` 통과.
- 신규 runtime 2 tests와 기존 C11.4 runtime 61 tests, 신규 FFI 1 test와 기존 C11.4 FFI 96 tests, text-layout nowrap 1 test 통과.
- Android API 37 AVD 8/8, iPhone 17 Pro/iOS 26.2 Simulator 8/8 및 두 fail-closed control 통과.
- 독립 verifier의 자체 부정 대조 8개 통과.
- targeted Clippy에서 text-layout 및 style-to-layout은 통과했다. runtime 검사에서는 수정하지 않은 두 기존 파일의 `clippy::type_complexity`와 `clippy::map_identity`가 나왔고, FFI 검사는 의존 runtime의 `clippy::type_complexity`에서 중단됐다. 이를 PR CI 통과로 해석하지 않는다.
- Swift typecheck, capture script `node --check`, 전체 workspace rustfmt 검사는 통과했다.
