# C11.3.3 area-backed fit-content cap 구현/runtime 실패 관점 검토 20개

계획 review와 분리해 source, feature gates, Android/iOS 실행 로그, Chrome-bound samples 및 static archive를 실패 관점으로 검토했다. 최초 검토 뒤 source commit `b815f483a253072cbbca1c0cd473d519fc95a220`을 만들고 새 scene, 양 플랫폼 전체 matrix, static archive 경계를 같은 source에서 다시 실행했다. 아래 PR evidence는 해당 commit으로 고정했다.

| # | 적대적 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | default FFI archive에 fixture payload가 우연히 포함됨 | Release feature-off archive에서 scene marker 3개가 모두 absent. [archive boundary](artifact-boundary/README.md) |
| 2 | opt-in build flag가 fixture를 전혀 켜지지 않음 | opt-in positive control archive에 scene ID·root ID·fixture module marker 3개가 모두 present. |
| 3 | 입력 scene ID로 다른 JS source를 주입함 | FFI source 선택은 정확한 `fit-content-area-cap` allowlist이며 suffix injection 단위 검사가 통과했다. |
| 4 | Android activity가 알 수 없는 scene을 runtime으로 전달함 | API 37 AVD route negative control은 unknown scene을 draw 전에 거부했다. |
| 5 | iOS route가 알 수 없는 scene을 Metal readback으로 전달함 | iOS Simulator route/sample negative controls와 기존 matrix rejection logs가 readback 이전 경계를 확인했다. |
| 6 | 같은 root ID를 가진 다른 tag, profile 또는 document 위치를 허용함 | runtime fixture preparer는 `div`, exact root ID, root parent, exact template profile을 각각 검사한다. |
| 7 | root에 추가 grid child나 바뀐 source order가 들어감 | 고정 direct-child 수, tag/ID, parent ownership과 style-node set이 다르면 fail-closed다. |
| 8 | fixed item 또는 text item에 추가 child를 숨김 | fixed item은 empty, fit item은 한 direct text node만 허용한다. |
| 9 | text node 종류나 내용이 바뀌어 다른 intrinsic source를 사용함 | `HostNodeKind::Text`와 정확한 `AA AA AA AA`를 확인하고 node ID를 보존한다. |
| 10 | Ahem 누락 또는 system fallback으로 min/max가 바뀜 | system-font-free engine에 pinned Ahem을 등록하고 20px/110px가 아니면 중단한다. |
| 11 | paragraph owner가 다른 node이거나 여러 owner로 넓어짐 | adapter는 direct fit-area item 하나만 owner로 내보내고 render helper도 단일 direct owner를 재검증한다. |
| 12 | 일반 template-grid CSS admission이 조용히 확대됨 | parser/author admission은 바꾸지 않았다. runtime bridge와 renderer text exception은 exact scene 및 opt-in feature에 묶었다. |
| 13 | area-created track을 positive implicit track으로 계산함 | typed layout과 trace는 `TemplateGrid`, explicit 2 / positive implicit 0을 요구한다. [규범 정정](../c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/runtime-model-correction-2026-10-31.md) |
| 14 | `grid-auto-columns` 대신 authored 20px track이 second column에 복제됨 | typed columns `[20px, fit-content(40px)]` 및 auto-columns 단일 `fit-content(40px)`을 별도 assertion한다. |
| 15 | `fit-content()` upper cap은 맞지만 min-content floor가 무시됨 | intrinsic source 20px/110px와 exact 40px used column을 서로 독립 assertion한다. |
| 16 | track trace와 item frame이 서로 달라짐 | trace `20px 40px`, root `[0,0,200,60]`, item `[20,0,40,60]`을 Android/iOS에서 비교했다. |
| 17 | renderer가 text node를 임의로 건너뛰거나 일반 scene text를 허용함 | style-to-render skip은 새 opt-in feature와 exact root marker에만 열고, Ahem overlay는 strict runtime owner에 부착한다. |
| 18 | Android capture가 draw/readback 전 로그를 성공으로 판정함 | 실제 run에서 summary-only early exit가 드러나 wait condition을 PixelCopy terminal summary까지 기다리도록 고쳤다. JS 16/16 및 최종 AVD capture가 이를 확인했다. |
| 19 | iOS sample log가 compositor 후 픽셀 또는 text glyph pixel 증거로 과장됨 | Metal drawable 3/3 sample만 exact로 기록하고 compositor 이후 및 glyph별 pixel은 검증하지 않았다고 README에 적었다. |
| 20 | 제한 scene 결과가 전체 C11 또는 공식 집계 증가로 전파됨 | 계획과 STATUS에서 C11 미완료 및 공식 `72/213`을 유지한다. 기존 39-scene 행렬은 Android/iOS에서 별도로 통과했다. |

검토 중 추가로 발견한 결함도 반영했다. capture README의 hard-coded geometry를 실제 frames에서 생성하도록 바꿨고, Android/iOS capture는 layout summary만으로 종료하지 않고 surface sample terminal marker를 기다린다. `GridContainerInput` trace flag를 추가한 뒤 기본 및 C11.4 opt-in Rust test initializer compile도 각각 실패했으므로 기존 test inputs에 explicit `false`를 추가했다. 수정 후 기본 layout suite 320/320, C11.4 opt-in layout suite 371/371이 통과했다. 이는 evidence integrity와 compile boundary에서 별도로 발견한 문제였다.

## 검증 근거

- [Android API 37 AVD 새 scene](android-avd-c1133-fitcap-android-avd-20261031T094514549Z-fc6bef6c/README.md) 및 [iPhone 17 Pro/iOS 26.2 새 scene](ios-simulator-c1133-fitcap-ios-simulator-20261031T094545391Z-17a7f5d0/README.md): source commit `b815f483a253072cbbca1c0cd473d519fc95a220`, geometry exact, each surface 3/3 RGBA samples exact; negative controls rejected at the documented boundary.
- [Android existing matrix](../c11-3-3-grid-template-areas-runtime-2026-10-31/android-avd-c1133-matrix-20261031T094711326Z-ff0e1d13/README.md) and [iOS existing matrix](../c11-3-3-grid-template-areas-runtime-2026-10-31/ios-simulator-c1133-ios-matrix-20261031T094825291Z-76286963/README.md): same source commit, 39 scenes / 126 nodes each.
- [feature boundary](artifact-boundary/README.md), FFI focused test 1/1, CSS/reference + fixture-boundary Node tests 41/41, default `spinon-layout` 320/320, C11.4 opt-in `spinon-layout` 371/371, `cargo fmt --all -- --check`, `node --check`, `git diff --check` 통과.

이 검토는 PR diff 검토 30개, required checks, 병합 후 검증을 대신하지 않는다.
