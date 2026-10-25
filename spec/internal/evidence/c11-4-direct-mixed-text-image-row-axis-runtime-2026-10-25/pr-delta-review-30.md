# C11.4 direct text/image intrinsic row-axis PR 변경 검토 · 30개 실패 관점

**검토 기준:** `origin/main` 대비 현재 `feat/c11-4-row-axis-mixed-text-image` 커밋 및 staged evidence 변경. 코드, FFI scene, Android/iOS adapter, capture script, Chrome observation, simulator manifests, archive 검사와 상태 문서를 각각 대조했다. 이 문서는 계획 검토나 구현/runtime 20 관점 검토를 재사용하지 않는 PR 변경 검토다.

| # | 공격 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 알 수 없는 scene ID가 새 runtime fixture를 반환해 제한 입력 경계를 넓힌다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic_rows.rs::runtime_source`는 네 ID만 검색하고, FFI test는 알 수 없는 ID가 `None`인지 확인한다. **차단 없음.** |
| 2 | scene table에 중복 ID가 들어가 먼저 선언된 scene이 뒤 scene을 가린다. | FFI test가 네 scene ID의 유일성을 검사하고, runtime test는 예상 네 ID와 순서를 항목별로 대조한다. **차단 없음.** |
| 3 | FFI source와 runtime expectations 사이에서 scene ID 집합이 어긋나 잘못된 CSS나 frame을 검증한다. | FFI scene ID·item ID 목록은 `c11_4_mixed/tests.rs::exposes_four_fixed_mixed_text_image_intrinsic_row_scenes`와 Android/iOS scene allowlist에 같은 네 이름으로 연결되고 네 런타임 scene 모두 성공 manifest에 있다. **차단 없음.** |
| 4 | reversed scene에서 배열만 바뀌고 source order 또는 item ID mapping은 그대로 남는다. | Runtime test의 예상 `item_ids`, FFI의 `MIN/MAX_REVERSED`, Android/iOS node 기대값이 image→text 순서를 각각 확인한다. 모든 scene 로그는 source order와 node frames를 남긴다. **차단 없음.** |
| 5 | row test가 사실상 span·auto-placement 경로를 거쳐 row intrinsic contribution을 증명하지 못한다. | `direct_row_*_item`과 FFI CSS는 모든 item에 row line 1, span 1을 지정한다. Runtime test는 두 자식만 있고 두 item 모두 같은 row인지 검사한다. **차단 없음.** |
| 6 | fixture가 예상과 다른 문장이나 여러 text node를 사용해 기여값 20을 우연히 만든다. | FFI source는 정확한 `XX XXXXX` text node 하나를 만들고 runtime test는 `ItemKind::Text("XX XXXXX")` 하나를 찾는다. **차단 없음.** |
| 7 | text intrinsic cache/font revision이 달라 다른 폰트 값이 row contribution으로 섞인다. | Android/iOS 입력 validator는 text intrinsic source의 `fontRegistryRevision=1`, `50×20` min-content와 `80×10` max-content를 검사한다. Chrome fixture는 Ahem readiness를 기록한다. **차단 없음; 고정 fixture 한정.** |
| 8 | image URL 실패나 임의 크기 fallback이 자연 크기 24×16을 대신한다. | 최종 platform logs/manifests는 natural image input `24×16`과 item frame을 기록한다. Chrome observation도 네 이미지 decode와 natural size를 확인한다. **차단 없음.** |
| 9 | PNG의 aspect-ratio metadata가 계산 경로에 예기치 않게 들어가 크기 기여를 바꾼다. | Android/iOS intrinsic input 검증은 `aspectRatio`가 null인 값을 요구하고, 이미지 높이 `16`과 너비 `24`를 별도로 검사한다. **차단 없음.** |
| 10 | root height가 고정돼 intrinsic row sizing이 실행되지 않는다. | FFI HTML은 `height:auto`; runtime style validator는 row scene에서 computed height `Auto`를 요구한다. 최종 결과 root height 20은 row track과 함께 기록됐다. **차단 없음.** |
| 11 | 열 sizing 변화가 row 결과인 것처럼 혼동된다. | 두 column은 `50px 24px`로 고정되고 adapter는 columns `[50,24]`를 확인한다. Row contribution은 Rows pass에서 별도로 읽는다. **차단 없음.** |
| 12 | min-content/max-content keyword가 두 행 값에서 바뀌지 않아 keyword 별 구현 차이를 검증했다고 과장한다. | 계획·README·0059는 두 keyword 모두 이 text의 block contribution이 20px이라 이 slice가 알고리즘 차이를 구별하지 못한다고 적는다. **한계 정확히 표시.** |
| 13 | row track이 두 item의 합 36px로 계산돼야 하는데 결과 20px만 확인한다. | CSS Grid track sizing은 같은 track의 item contribution을 합산하지 않고 최대 contribution으로 sizing한다. Chrome 관측과 양 플랫폼의 각 contribution 20/16 및 row `[20]`가 함께 고정돼 있다. **차단 없음.** |
| 14 | contribution이 열 축 값으로 보고되어 row축 동작이 검증되지 않는다. | Android/iOS matcher는 `Columns`와 `Rows` pass 순서 및 grid identity를 검사하고, text/image contribution은 Rows pass에서 `MinContent` 또는 `MaxContent` kind로 각각 찾는다. **차단 없음.** |
| 15 | row track이 초기부터 20px이라 intrinsic contribution 처리 없이도 결과가 통과한다. | Android/iOS matcher는 Columns 초기 크기 `[50,24]`, Rows 초기 크기 `[0]`, row intrinsic-item batch, 최종 Rows `[20]`를 확인한다. **차단 없음.** |
| 16 | 트랙 trace에 누락·추가된 non-gutter track이나 collapsed track이 있어도 선택한 두 값만 통과한다. | Row path의 Android/iOS stage-shape assertion은 모든 stage에서 비-gutter track 수, collapsed flag, finite base size와 zero gutter를 검사한다. **차단 없음.** |
| 17 | root alignment 초기값과 child computed self-alignment를 혼합해 다른 used alignment 입력이 들어온다. | `style.rs::validate_grid_style`는 row root `align-items:start`, `justify-items:normal`, `direction:ltr`, `writing-mode:horizontal-tb`를 확인하고 `validate_item_style`는 child `align-self:auto`·`justify-self:auto`를 확인한다. **차단 없음.** |
| 18 | text 또는 replaced image의 auto `align-self`가 부모 `align-items:start` 대신 stretch로 used되어 높이를 바꾼다. | Chrome 기본 정렬 observation과 두 simulator 모두 text `50×20`, image natural `24×16`, row `20`을 별도로 기록한다. **차단 없음; arbitrary CSS alignment는 지원 주장에 포함하지 않음.** |
| 19 | root/child margin, padding, border, gap, min/max 또는 positioning 값이 row size에 섞인다. | runtime style validator는 zero edges/gaps, default border, auto min-size, no max-size, static positioning을 요구한다. FFI source의 fixed style과 Chrome observation도 같은 조건이다. **차단 없음.** |
| 20 | direction/writing-mode가 플랫폼 기본값에 따라 달라져 row/block 축이 서로 다른 축이 된다. | 행 root 및 text computed style의 LTR·`horizontal-tb`를 validator가 요구한다. Author `writing-mode`는 현재 runtime allowlist가 거부해 FFI에서 생략했고 그 이력/한계를 계획과 evidence에 기록했다. **차단 없음; authored writing-mode 지원은 미포함.** |
| 21 | root frame만 맞고 child frame의 DOM identity/order나 image natural bounds는 다르다. | Android/iOS 기대 frame은 각 scene의 root와 두 child node ID·좌표·크기를 모두 검사하며 log는 `presented boxes=3`이다. **차단 없음.** |
| 22 | 각 scene이 layout에만 도달하고 WGPU draw/present 전 실패해 실행 화면을 증명하지 못한다. | 최종 Android/iOS runtime logs는 `layout=ready boxes=3`와 `presented boxes=3`를 함께 기록하고 각 simulator manifest에 연결한다. **차단 없음; 이는 pixel equality와 별개.** |
| 23 | 공유 Android/iOS matcher 수정이 선행 C11.4 column scenes를 회귀시키는데 새 row 4개만 실행해 놓친다. | `c114DirectMixedTrackPassMatches`/`stageTrackSizesMatch`는 기존 direct-column scene에도 사용되며 이 PR에서 generalized됐다. 최종 source `0af34026`에서 기존 direct column 네 scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에 각각 재실행해 4/4 통과했다. 각 scene의 기존 frames, tracks, contributions, trace와 draw/present가 확인됐다. [Android run](runs/c114-direct-mixed-column-regression-android-avd-20261025T022319630Z-adc546c2/android-simulator.json) · [iOS run](regression/runs/c114-direct-mixed-ios-simulator-20261025T022325437Z-aec18e56/ios-simulator.json). **해결됨.** |
| 24 | Android 결과가 실제 기기나 hardware GPU에서 나온 것으로 오해된다. | Manifest는 API 37 ARM64 16 KB AVD와 simulator serial을 고정하고, README/STATUS는 AVD·iOS Simulator와 실기기/hardware 한계를 구분한다. **한계 정확히 표시.** |
| 25 | iOS 실행 중 소스가 바뀌어 manifest hash가 앱 build artifact와 일치하지 않는다. | iOS manifest에서 tracked-diff hash 차이는 명시됐고, 양 시점 9개 source-file hash 및 aggregate는 같으며 dirty path는 evidence 문서 세 개뿐이다. 이를 README와 revision review에서 source identity와 evidence snapshot 차이로 분리한다. **한계 공개; 앱 source drift 증거는 없음.** |
| 26 | screenshots가 glyph/PNG pixel equality나 border-overflow 검증을 대신한다. | Runtime README는 화면을 box/layout 증거로 제한하고 glyph/PNG texel equality를 부인한다. 이 scene은 border가 없는 고정 fixture여서 바깥 테두리/ink overflow를 검증하지 않는다. **한계 공개; border-overflow는 별도 시나리오 필요.** |
| 27 | screenshot 파일이 빠졌거나 README 링크가 다른 simulator run을 가리킨다. | 최종 Android/iOS README embeds는 각각 최종-source 4/4 run 폴더의 PNG를 가리킨다. 캡처 script는 PNG signature 및 지정 simulator 해상도를 검사하고 run manifest에 screenshot path/hash를 남긴다. **차단 없음.** |
| 28 | 기본 product archive와 fixture positive control이 둘 다 fixture scene payload를 포함하거나, 둘 다 빠져서 boundary test가 무의미하다. | `artifact-boundary.json`은 final source `0af34026`에서 기본 archive 0/4 marker·fixture ABI 없음, opt-in positive control 4/4 marker·ABI 존재, 두 archive hash와 실행 로그를 기록한다. **차단 없음.** |
| 29 | static archive marker 결과가 최종 APK/IPA exclusion으로 확대 주장된다. | Archive README, verification summary, runtime README는 이 gate가 Rust static archive만 확인하며 최종 Android APK/iOS app bundle exclusion을 증명하지 않는다고 명시한다. **한계 정확히 표시.** |
| 30 | 좁은 fixture 성공을 전체 C11.4 지원·완료로 승격하거나 기존 0/4 실패를 최종 success count에 더한다. | `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, 계획과 evidence README는 C11.4 checkbox 미완료·공식 `72/213`을 유지한다. 실패 실행은 별도 historical run으로 남고 final summary는 `0af34026`의 플랫폼별 4/4만 집계한다. **상태 과장 없음.** |

## 판정

코드·fixture 경계·최종 row-scene 증거 및 기존 column regression에서 병합을 막는 기능 결함은 발견하지 못했다. #23의 shared matcher regression gate는 최종 source의 Android/iOS 각 4/4 결과로 해결했다. #26의 border-overflow 범위는 이번 intrinsic-row fixture가 다루지 않으며 지원 주장을 넓히지 않는다.

PR 검토 시점의 최종-source row matrix는 Android 4/4, iOS 4/4이며 기존 column regression도 Android 4/4, iOS 4/4다. 이 네 결과를 플랫폼별·matrix별로 유지하고 합산하지 않는다. `C11.4`는 미완료이고 공식 완료 집계는 `72/213`이다. Screenshots는 layout/box 배치만 보이며, 실기기·hardware GPU·border overflow·최종 app bundle fixture exclusion 증거가 아니다.
