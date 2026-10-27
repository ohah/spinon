# PR 변경분 실패 관점 검토 30개

검토 대상은 네 fixed scene builder, Android/iOS 앱 matcher 연결, 독립 evidence verifier, FFI artifact 경계, 상태·계획 문서와 simulator 산출물이다. 각 항목은 별도 실패 경로를 다룬다. 판정은 이번 고정 입력과 실행에 한정한다.

| # | 공격 가설 | 판정과 근거 |
| ---: | --- | --- |
| 1 | scene ID에 JavaScript가 삽입되어 임의 fixture가 실행될 수 있다. | **PASS** — Rust builder는 네 exact ID만 검색하며 미등록 ID를 반환하지 않는다. dispatcher와 앱의 unknown-scene 거부도 확인했다. |
| 2 | dispatcher가 새 scene을 기존 fixture와 잘못 매칭하거나 알 수 없는 입력에 fallback할 수 있다. | **PASS** — 새 exact lookup을 기존 dispatch 앞에 두고 그 외 입력은 기존 경로로 전달한다. `unknown;throw 1`도 오류다. |
| 3 | Rust·Java·Swift·독립 expected JSON의 scene 표가 달라 일부 플랫폼만 검증될 수 있다. | **PASS** — verifier가 네 ID 집합과 양 플랫폼의 두 CSS track 값 표를 비교했다. |
| 4 | Rows 입력이 Columns로, 또는 반대로 매핑될 수 있다. | **PASS** — 각 scene의 축을 expected JSON과 Android/iOS matcher 표에서 교차 확인했고 8개 실행에서 target pass도 일치했다. |
| 5 | definite 200px 축과 120px 반대축이 바뀌어 flex 계산 기준이 달라질 수 있다. | **PASS** — fixture builder의 축별 root 크기와 manifest의 root/node frame을 독립 기대값에 대조했다. |
| 6 | root padding, border, margin이 available size를 줄여 결과를 왜곡할 수 있다. | **PASS** — root는 `border-box`, margin/padding/border 0이며 200×120 또는 120×200 frame이 두 앱에서 일치했다. |
| 7 | marker의 내용·intrinsic minimum이 flex base를 올릴 수 있다. | **PASS** — 빈 `div` 둘에 `min-width:0;min-height:0`, padding/border 0을 적용한다. source 검사와 두 플랫폼 marker frame이 통과했다. |
| 8 | marker auto-placement가 implicit track을 추가해 2-track 계산을 우회할 수 있다. | **PASS** — 두 item 모두 grid row와 column을 명시한다. trace는 target 5 internal entries 및 고정축 3 entries만 허용한다. |
| 9 | target gap이 반대축으로 배치되거나 빠질 수 있다. | **PASS** — builder는 scene 축에만 10px gap을 두고, trace와 item offset이 10px 위치를 확인한다. |
| 10 | `c04-runtime-gpu` 제품 archive에 fixture ID나 ABI가 포함될 수 있다. | **PASS** — product static archive에서 ID 0/4, fixture ABI text와 정의 symbol 부재를 기록했다. opt-in fixture-test는 positive control로 4/4 및 ABI 존재를 확인했다. |
| 11 | scene 선택 UI가 새 ID를 전달하지 않아 다른 입력을 실행할 수 있다. | **PASS** — Android/iOS scene allowlist에 네 ID를 연결했고 각 platform run에서 해당 scene ID가 성공 summary와 일치했다. |
| 12 | root는 맞지만 item 개수가 기존 matcher의 기본값과 달라 assertion이 빠질 수 있다. | **PASS** — 양 플랫폼은 root와 marker 둘을 포함한 3 node를 요구하며 manifest의 모든 summary가 `boxes=3`이다. |
| 13 | 유한 반대축 track이 조용히 stretch 또는 intrinsic 계산으로 들어갈 수 있다. | **PASS** — 고정축 pass에서 빈 contribution과 `Initialize → FixedTrackEarlyReturn`, 120px CSS track, 0px boundary gutter를 확인한다. |
| 14 | Columns/Rows 두 pass가 빠지거나 같은 축을 중복 실행할 수 있다. | **PASS** — matcher와 verifier가 정확히 두 pass, sequence `[0,1]`, axis `[Columns, Rows]`를 확인한다. |
| 15 | scene target이 아닌 pass에서 `ExpandFlexibleTracks`를 찾아 통과할 수 있다. | **PASS** — scene 축으로 target pass를 선택하고, 그 축의 trace에서만 flex track base를 판정한다. |
| 16 | Android가 `Initialize` 다음에 `MaximizeTracks`가 오는 규칙을 놓칠 수 있다. | **수정 후 PASS** — 리뷰 중 Android가 초기화 횟수만 보던 차이를 발견했다. matcher에 `Initialize < MaximizeTracks`를 추가했고 역순 trace를 독립 verifier 음성 대조가 거부한다. 새 AVD 빌드는 4/4 통과했다. |
| 17 | stage가 누락·중복되거나 순서가 뒤집혀도 결과만 우연히 맞아 통과할 수 있다. | **PASS** — target pass의 Initialize, Maximize, Expand, Stretch가 각각 한 번씩 정해진 순서로 있어야 한다. 중복·누락은 matcher가 세며 verifier는 Expand 누락을 거부한다. |
| 18 | internal track index나 CSS/gutter 순서가 바뀌어 gap을 CSS base로 셀 수 있다. | **PASS** — target trace는 5개 entry에서 index, gutter flag, collapsed 경계를 확인하고 두 CSS base와 내부 gutter를 나눠 비교한다. |
| 19 | boundary gutter가 살아 있어 사용 가능한 공간을 잘못 차지할 수 있다. | **PASS** — target boundary index 0·4와 고정축 0·2는 collapsed이며 base 0이어야 한다. |
| 20 | `weighted-columns`의 1fr:2fr 오차가 총 너비 비교만으로 숨을 수 있다. | **PASS** — 두 base를 각각 독립 기대값 `190/3`, `380/3`과 `1e-4 CSS px`로 비교한다. |
| 21 | frozen minimum 이후 다른 flex item의 재분배가 빠질 수 있다. | **PASS** — `frozen-row`는 `[80,110]` stage base와 y/height frame `[0,80]`, `[90,110]`을 함께 비교한다. |
| 22 | 1 미만 flex factor가 1로 보정되어 남은 공간이 사라질 수 있다. | **PASS** — `subunit-columns`는 base `[85,20]`과 남은 85px를 독립 계산과 대조한다. |
| 23 | `0fr`가 최소 24px을 무시하고 0으로 줄어들 수 있다. | **PASS** — `zero-fr-row` stage base `[24,166]`과 item frame을 Android/iOS 모두에서 확인했다. |
| 24 | Android의 f32 근사값이 수학상 정확한 결과와 다르다는 이유로 오판되거나 과도한 허용오차가 숨길 수 있다. | **PASS** — Android runtime `63.333332…`/`126.666664…`를 두 플랫폼 matcher와 독립 verifier의 `1e-4` CSS px 허용치로 비교한다. |
| 25 | item frame 순서·node ID가 바뀌어 좌표만 맞는 다른 node를 통과시킬 수 있다. | **PASS** — frame index와 node ID가 각자 기대 순서와 일치해야 하며 세 node 전체의 x/y/width/height를 검사한다. |
| 26 | frame에 NaN, Infinity, 잘린 숫자 또는 중복 frame marker가 있어도 비교가 통과할 수 있다. | **PASS** — 양 플랫폼 parser와 독립 verifier는 숫자 형식·유한성·frame 수·단일 marker를 요구한다. 비유한 값 음성 대조는 거부됐다. |
| 27 | log chunk 유실·중복이나 중복 JSON key가 trace 해석을 바꿀 수 있다. | **PASS** — 독립 verifier는 각 trace part의 번호·총수와 JSON key 유일성을 확인한다. 중복 key 음성 대조가 거부됐다. |
| 28 | 미지원 C11.2.1 profile 또는 알 수 없는 scene을 성공 입력으로 오인할 수 있다. | **PASS** — Android와 iOS manifest의 두 rejected route가 unknown scene 및 conflicting profile을 각각 거부한 것을 기록한다. |
| 29 | 이전 화면이 남아 있는데 새 layout 결과의 screenshot으로 제출할 수 있다. | **PASS** — 같은 run ID의 성공 summary, frame, trace, `presented boxes=3`, PNG signature와 SHA-256을 verifier가 대조했다. Pixel equality를 주장하지 않는다. |
| 30 | 플랫폼 빌드 근거가 잘못된 기기·V8 입력을 사용하거나 프로젝트가 새 Swift 파일을 누락할 수 있다. | **PASS** — Android API 37 ARM64 16 KB AVD를 명시 serial로 실행했고 V8 revision·clean 상태를 manifest에 기록했다. iOS source는 Xcode project에 등록되어 iPhone 17 Pro/iOS 26.2 Simulator 4/4 run에서 빌드·실행됐다. |

## 판정 범위

검토는 definite sizing axis 200px, 반대축 120px, 단일 10px gap, 빈 marker 둘을 쓰는 네 scene에만 적용된다. 일반 CSS admission, intrinsic contribution, spans, indefinite free space, 전체 WPT, 실기기, GPU readback과 pixel equality는 이 변경으로 통과 판정하지 않는다.
