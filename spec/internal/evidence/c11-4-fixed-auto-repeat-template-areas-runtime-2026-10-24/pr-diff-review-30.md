# C11.4 fixed-repeat + auto-repeat + template areas PR diff 검토 30

검토일: 2026-10-25
범위: 현재 staged 및 unstaged 변경, 런타임 evidence metadata, canonical Chrome screenshot.
수행: read-only 코드/evidence 대조. 이 검토에서 테스트·빌드·시뮬레이터·브라우저는 실행하지 않았다.

## 30개 실패 경로

1. **고정 scene ID와 수 — 통과.** FFI fixture는 새 9개 ID를 하나씩 요구하고 18개 전체 matrix의 고유성도 검사한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:88,161-180`.
2. **oracle ID 변환 — 통과.** `fixed-auto-area-` 접두어를 제거한 ID로 고정 expected-values scene을 찾고 `oracleId`와 대조한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:178-191`.
3. **fixture source 경계와 생성 크기 — 통과.** 장면별 runtime HTML 생성은 fixture 모듈에 있고 8 KiB 상한을 검사한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:1-58,334-339`.
4. **Android opt-in admission — 통과.** 새 ID는 고정 목록에 있고 `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST`가 꺼지면 adapter에서 인정하지 않는다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:536-546,632-643`.
5. **iOS opt-in admission — 통과.** scene allowlist는 fixture-build `#if` 블록 안에 있다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:17-52`.
6. **canonical CSS track/area 입력 — 통과.** 각 scene의 columns, rows, areas, auto-columns/rows 값을 scene CSS 블록과 비교한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:193-223`.
7. **root frame과 definite available size — 통과.** expected root 크기가 fixture 크기 및 독립 oracle available size와 일치하는지 확인한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:226-260`.
8. **item 순서·이름·frame — 통과.** fixture item 배열 순서가 Chrome child 순서와 이름·frame까지 일치해야 한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:263-283`.
9. **실제 node count/order — 통과.** capture는 관측 node의 order, ID, x/y/width/height를 순서대로 비교한다. `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:5691-5706`.
10. **Chrome 좌표계 — 통과.** child 위치는 root의 Chrome 좌표를 빼서 fixture root-relative 예상값과 비교한다. `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:4504-4528`.
11. **양축 used track vector — 통과.** Chrome CSSOM columns/rows track 개수와 각 폭을 `1/64 CSS px` 이내로 확인한다. `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:4508-4514`.
12. **empty auto-fit collapse — 통과.** 비어 있는 반복 tracks만 collapse되며 0px used size와 collapse bit를 대조한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:320-330`; Android/iOS trace assertions도 있다.
13. **occupied auto-fit — 통과.** 배치된 hero가 차지하는 반복 tracks는 non-collapsed expected vector로 확인한다. Android/iOS frame·trace 기대값이 별도 정의되어 있다.
14. **authored/generated named-line collision — 통과.** collision 장면은 positive `3/3`, negative `-1/-1` placement 입력을 확인한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:348-350`.
15. **area가 explicit template보다 큰 경우 — 통과.** 오른쪽 item이 root를 넘는 양을 oracle의 20px overflow와 산출식으로 대조한다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:302-317`.
16. **area가 더 짧은 경우 — 통과.** 짧은 area에서도 template trailing tracks와 tail frame이 보존되는 기대값을 확인한다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:887-891`; Android frame table은 `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:690-732`.
17. **area→none→area revision — 범위 명시됨.** 테스트는 같은 HostDocument node의 세 snapshot을 새 immutable StyloDocumentView로 cascade한다. 새 view마다 metadata가 제거·복원되는 것은 검사하지만 동일 view cache 재사용은 하지 않는다. README가 이 한계를 명시한다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:779-889`; `spec/internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-24/README.md:36`.
18. **cross-axis fixed/auto 조합 — 통과.** columns-fixed/rows-auto와 columns-auto/rows-fixed를 각각 reject하는 negative control이 있다. `crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs:546-585`.
19. **Android trace cardinality와 순서 — 코드 수정 확인.** trace 하나, pass 정확히 2개, `[Columns, Rows]` 순서를 현재 verifier가 요구한다. 이전의 2개 이상/순서 무관 허용은 닫혔다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:748-769`.
20. **iOS trace cardinality와 순서 — 통과.** 같은 두 pass 및 axis 순서를 고정한다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:2246-2276`.
21. **trace final sizes/collapse bits — 통과.** 두 앱의 final-stage tracks를 scene별 사용 폭과 collapse vector에 비교한다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:775-804`; `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:2255-2276`.
22. **trace evidence 보존 — 통과, summary는 축약형.** capture는 chunk를 재조립해 JSON parse와 track 검증을 수행한다. `result.json`에는 final tracks와 chunk 개수만 두지만 전체 stage trace는 `runtime-<scene>.log`에 남는다. `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:5711-5725,6756-6764,6976,7058`.
23. **Android sample payload 대상/좌표 — 통과.** area-larger 두 장면만 허용하고 root/right 대상 ID, 4개 좌표, root 경계 관계와 중복 ID를 검증한다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:6662-6725`.
24. **Android PixelCopy RGB — 통과.** root-gap green, root edge/overflow blue, item 이후 opaque black의 실제 PixelCopy RGBA를 고정 기대값과 비교한다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:11166-11178`; 기존 저장 로그는 각 네 표본 exact 일치를 기록한다.
25. **iOS sample admission/color — 통과.** parser가 scene별 예상 sample 목록, node ID, 유일한 ID/좌표, 색상 채널 범위를 확인한다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:4721-4803,4909-4919,5695-5715`.
26. **CSS pixel→drawable pixel 및 clear color — 통과.** CSS pixel 중심을 drawable scale로 바꾸고 실제 RGBA를 비교하며 clear 표본은 기존 Metal drawable의 opaque black contract를 쓴다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:4727-4752,12125-12141`.
27. **기본 run-kind/evidence 경로 — 이전 finding 해결.** scene IDs로 fixed-auto-area run kind를 자동 추론하고 기본 evidence root로 보낸다. fixture boundary test가 이제 `--run-kind` 없이도 경로를 검사한다. `tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs:147-165,187-190`; `tools/css-reference/fixture-module-boundary.test.mjs:1281-1288`.
28. **CSS placement selector scope — 이전 finding 해결.** declaration matcher는 정확한 `.oracleId .itemName` selector의 rule에서 placement를 찾는다. 다른 scene의 동일 property/value로 통과할 수 있던 검사는 제거됐다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs:66-86,284-299`.
29. **팔레트·canonical screenshot/hash — 이전 finding 해결.** canonical HTML root/items는 앱 팔레트와 일치하고 Chrome PNG를 2026-10-25 Agent Browser에서 새로 캡처했다. `SHA256SUMS`의 HTML/PNG hash는 현재 파일과 일치한다: fixture `08bd06c4…`, PNG `026bb171…`. 새 PNG 경로는 `spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/chrome156-fixed-auto-repeat-areas.png`; screenshot update가 geometry oracle JSON를 바꾸지는 않았다고 README가 범위를 설명한다.
30. **metadata budget/실패 복구/확장 cap — 코드와 테스트 경로 확인.** combined 1 MiB budget은 checked arithmetic으로 line/area metadata를 센다. exact/over, 양 auto-repeat 축, line-only exemption, 다섯 reservation failure context/recovery, 그리고 layout의 expanded-line metadata cap negative가 있다. `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs:101-209`; `crates/spinon-style-to-layout/src/projection/style_values/grid_values/combined_metadata_tests.rs:51-217`; `crates/spinon-layout/src/grid_tests.rs:486-511`.

## Capture lineage 재검토

**이전 P2 해소 — capture가 source revision과 일치한다.** 두 새 `simulator.json`의 `sourceRevision.trackedDiffSha256`는 `b0d127b9f49e4e947958b6fb32e2503a3fa920d95edb38cc98e274013f63b170`이다. 메타데이터의 `trackedDirtyPaths`만으로 현재 작업 트리의 `git diff --binary HEAD -- <trackedDirtyPaths>`를 다시 계산한 hash도 같은 `b0d127…`다. 전체 `git diff --binary HEAD`는 generated evidence와 review output이 stage될 때 바뀌므로 capture metadata의 source revision을 비교하는 기준으로 사용하지 않는다. 출력 파일은 자기 자신을 포함할 수 없으므로 source revision 비교에는 메타데이터가 고정한 `trackedDirtyPaths` 집합을 사용했다. Android 증거는 `spec/internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/android-avd-final-2026-10-25/`, iOS 증거는 `spec/internal/evidence/c11-4-fixed-auto-repeat-template-areas-runtime-2026-10-25/ios-simulator-final-2026-10-25/`에 있다. 각 `result.json`은 서로 다른 9개 scene 관측을 기록하고, 모든 관측의 `drawPresented`가 true다. Android PixelCopy 로그와 iOS Metal 로그에는 `area-larger` 두 scene의 표본 각각 4개, 총 8/8 exact sample match가 기록되어 있다. 따라서 현 Android trace verifier를 포함한 source diff의 simulator 실행 증거가 생겼으며, 이전 capture hash 불일치 finding은 닫는다.

**물리 Android 시도는 범위에서 제외.** `android-device-c114-fixed-auto-area-android-device-20261024T152234713Z-5a38fede/capture-error.txt`는 `화면 표시 결과가 없습니다` assertion으로 capture가 실패했다고 기록한다. 같은 디렉터리의 `screen-state.txt`는 `mWakefulness=Dozing`이다. 이 시도는 draw/present 성공을 입증하지 못하므로 실기기 통과 증거로 세지 않는다. AVD와 iOS Simulator capture의 성공 판정에는 영향을 주지 않는다.

## 실행 범위

이번 lineage 갱신에서는 read-only Git diff, JSON metadata, capture 로그와 physical-device 실패 파일만 확인했다. 테스트·빌드·브라우저·시뮬레이터를 실행하지 않았고 리뷰 파일 외에는 수정하지 않았다. 위 diff hash는 리뷰 파일 갱신 전에 계산했다. 리뷰 파일은 untracked이므로 `git diff HEAD`에 포함되지 않는다.
