# C11.4 direct mixed text/image 구현 검토 — 20개 실패 관점

검토 범위는 런타임 mixed fixture, FFI scene generator, Android/Swift scene adapter, 캡처 스크립트입니다. 20개 관점은 최초 검토 시점의 소스를 기준으로 기록했으며, 수정된 네 관점의 현재 상태는 아래 사후 재검토 섹션에 반영했습니다. 소스와 기존 저장 로그만 읽었고 테스트·시뮬레이터·기기는 실행하지 않았으며, 완료 판정을 뜻하지 않습니다.

## 우선순위 finding

- **P2 — fixture CSS fail-closed 검사 누락:** 초기 검토 finding. 사후 재검토에서 수정 확인.
- **P2 — iOS trace assertion이 Android보다 약함:** 초기 검토 finding. 사후 재검토에서 수정 확인.
- **P2 — 캡처 근거 무결성:** 초기 검토 finding. 사후 재검토에서 수정 확인.
- **P3 — Android 실패 정리:** 초기 검토 finding. 사후 재검토에서 수정 확인.
- **증거 범위 한계:** draw marker는 `presented boxes=3`을 확인하지만 텍스트/이미지 paint를 입증하지 않습니다. 기존 direct mixed 로그는 `runtime_image_count=0`, `runtime_text_count=0`을 기록합니다.

## 검토 관점

1. **fixture 기능 경계** — **통과.** 런타임 fixture 모듈은 `c11-4-grid-track-sizing-fixture-test` cfg 아래에 있습니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs:6-8,46-48`, `crates/spinon-runtime/Cargo.toml:12-16`.

2. **direct scene ID의 런타임 선택** — **통과.** cyclic mixed route를 먼저 확인하되 고정 scene 표의 exact ID가 아니면 direct route는 `None`을 반환합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:208-225`.

3. **기존 cyclic FFI route 보존** — **통과.** 새 generator는 `direct-*` exact IDs만 먼저 받으며, 기존 `max-content-forward` 등 cyclic mixed IDs는 기존 route로 계속 흘러갑니다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:1091-1124`; direct generator의 허용 목록은 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic.rs:57-86`.

4. **일반 혼합 입력 fail-closed** — **통과(코드 경로).** direct scene ID와 일치하지 않는 혼합 문서는 mixed fixture가 채택하지 않고, fixture 전용 이미지 탐지기는 미등록 root에 HTML image가 있으면 오류를 반환합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs:391-399,460-469`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs:53-67`.

5. **root 식별·소유권** — **통과.** root namespace/tag, 문서 최상위 부모, fixture style profile을 모두 확인합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:216-245`.

6. **직접 자식 수·DOM 순서·ID** — **통과.** 정확히 두 direct children이어야 하고, 순서별 fixture ID·namespace·부모가 씬 표와 맞아야 합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:236-263`.

7. **텍스트 leaf 구조와 내용** — **통과.** text item은 단일 direct text leaf만 허용하고 문자열도 `XX XXXXX`와 완전히 일치해야 합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:264-286`.

8. **replaced image leaf와 URL** — **통과.** image descendants를 허용하지 않고 고정 PNG URL만 받습니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:288-299`; generator URL은 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic.rs:115-123`.

9. **column 1 / span 1 placement** — **통과.** generator CSS와 runtime computed placement 검사가 모두 `grid-column:1 / span 1`에 맞습니다. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic.rs:106-122`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs:96-101`.

10. **root 및 두 row 계약** — **통과.** direct min/max 씬은 root 120×48, 24px 두 row, 단일 min-content/max-content column과 일치하는 computed style을 요구합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs:31-77`; 생성 CSS는 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/mixed_direct_text_image_intrinsic.rs:94-101`.

11. **텍스트 intrinsic 측정 입력** — **통과.** Ahem 10px, 고정 텍스트 및 관련 font/wrap 속성을 확인하고 전용 폰트를 등록합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs:118-150`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:365-371`.

12. **PNG natural size 입력** — **통과.** 문서 요청으로부터 고정 PNG를 디코드하고 24×16 메타데이터를 확인합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:300-353`; fixture 상수는 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs:16-21`.

13. **intrinsic revision과 layout API 선택** — **통과.** 초기 revision 다음 값을 snapshot에 넣고, direct scene은 two-axis root intrinsic 경로를 opt-in하지 않습니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:328-378`; 단일-axis mixed layout 분기는 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs:548-568,723-725`.

14. **변형 CSS에 대한 fail-closed 보장** — **최초 판정 결함 P2; 사후 상태는 아래 재검토 참고.** item validator는 placement·크기·margin/padding/border는 검사하지만 `layout_position`을 확인하지 않고, text item은 `align-self`/`justify-self`도 검사하지 않았습니다. 같은 direct fixture ID에서 text에 `align-self:center`를 주면 예상 y 위치가 달라질 수 있고, `position:absolute`는 normal-flow intrinsic contribution을 바꿀 수 있는데 fixture adapter가 이 입력을 거부하지 않았습니다. 최초 검토 당시 경로: `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs:88-174`. 비교 기준으로 기존 cyclic mixed validator는 static position을 확인했습니다: `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/cyclic_minmax_mixed_text_image_span.rs:251-284`.

15. **image decode·snapshot 오류 전파** — **통과.** 요청 생성, 디코드, snapshot 생성, metadata 누락·불일치는 각각 구체 오류로 반환되고 기본값으로 조용히 대체되지 않습니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed.rs:300-363`.

16. **Android scene dispatch와 역순 노드 mapping** — **통과(소스 대조).** 네 ID가 scene allowlist에 있고, 정/역순 node ID와 frame 기대값 및 text/image intrinsic source order가 구분됩니다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:145-146,3025-3036,4467-4497`.

17. **Android trace 구조 assertion** — **통과(소스 대조).** column 초기 0, 최종 track, span-one intrinsic batch, row track 및 item contribution을 확인합니다. `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:4566-4620`.

18. **Swift scene dispatch 및 trace assertion 강도** — **최초 판정 부분 결함 P2; 사후 상태는 아래 재검토 참고.** Swift는 direct IDs, reverse node order, natural size, final track과 contributions를 확인하지만 최초 검토 시점에는 column 초기 track과 span-one intrinsic batch를 검사하지 않았습니다. 최초 검토 당시 경로: `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:2737-2792`; Android 비교 기준: `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:4599-4620`.

19. **플랫폼별 capture 완료 marker** — **통과(소스와 기존 로그 대조).** Android JNI의 `SPINON_C0410_DRAW status=0 presented boxes=3`를 predicate가 사용하고, 뒤따르는 Java generation 행도 존재합니다. iOS의 generation 포함 draw 행 역시 정규식과 맞습니다. `capture-direct-mixed.mjs:88-99,222-236,281-295`; emitter는 `platforms/android/app/src/main/cpp/spinon_jni.cc:2163-2169`, `platforms/ios/Sources/SpinonRunner.mm:3097-3108`; 기존 로그 예: `android-runtime-direct-max-content-reversed.log:26-27`, `ios-runtime-direct-min-content-forward.log:16`.

20. **capture 근거 무결성·실패 정리** — **최초 판정 결함 P2/P3; 사후 상태는 아래 재검토 참고.** 최초 구현은 trace chunk 중복을 거르지 않았고, `sourceRevision()`은 신규 untracked FFI generator를 해시하지 않았으며 APK와 현재 소스의 빌드 연관도를 입증하지 않았습니다. 또한 Android `force-stop`은 전체 loop 성공 뒤에만 호출되어 검증 예외 시 앱이 남을 수 있었습니다. 최초 검토 당시 경로: `capture-direct-mixed.mjs:69-75,122-130,202-211,221-241`.

## 사후 수정 후 재검토

현재 작업 트리의 수정본을 다시 읽어 위 네 finding을 각각 확인했습니다. **네 항목 모두 코드상 수정됨**으로 판정합니다. 이는 수정된 코드를 실행해 확인했다는 뜻은 아닙니다.

1. **fixture CSS fail-closed 검사 — 수정됨 (P2 해소).** root와 item에 `ComputedCssPosition::Static` 검사가 들어갔고 모든 item에 `justify-self:auto`, `align-self:auto` 검사를 적용합니다. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/style.rs:51-60,97-115,194-196`. 이 property들은 runtime grid profile의 computed-property 목록에 포함되고 computed style map에 기록됩니다: `crates/spinon-style/src/stylo_dom/cascade/runtime_layout.rs:60-65`, `crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs:164-172`.

2. **Swift Initialize 및 intrinsic batch 검사 — 수정됨 (P2 해소).** 각 pass에서 `Initialize`와 최종 stage를 따로 검사합니다. column 초기 크기 0, row 초기 크기 24×24, 최종 track 값 및 column span-one intrinsic batch 존재를 확인합니다. `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:2755-2775,2794-2809`.

3. **캡처 source/build provenance 및 trace 중복 — 수정됨 (P2 해소).** `sourceInputPaths`에는 신규 untracked generator와 이번 slice의 Rust/runtime/Java/Swift 앱 입력 7개를 명시하고 파일별 SHA-256과 목록 digest를 기록합니다. 캡처 절차는 별도 script SHA-256으로 manifest에 연결하고 빌드 전·후와 실행 종료 시 동일성을 검사합니다. `capture-direct-mixed.mjs:53-61,82-143,145-184`. 스크립트가 선택 플랫폼 앱을 빌드하고 빌드 전후 source digest를 비교한 뒤 artifact SHA-256을 저장합니다; Android에서는 빌드 APK·설치 APK와 source hash를, iOS에서는 앱 executable과 source hash를 빌드 provenance에 대조합니다(`:302-392,395-466`). trace는 chunk count가 전체 수와 같아야 진행하므로 중복·누락을 거릅니다(`:206-215`). 이 판정은 명시된 7개 앱 입력과 별도로 해시한 캡처 절차, 그리고 해당 산출물 연결 범위에 한정합니다.

4. **Android 실패 시 앱 정리 — 수정됨 (P3 해소).** 전체 Android build/capture 경로를 `try`로 감싸고, 성공·실패 여부와 무관하게 `finally`에서 `force-stop`을 허용 실패 처리로 호출합니다. `capture-direct-mixed.mjs:251-254,317-319`.

## 분리한 런타임·환경 증거 한계

- 저장된 API 37.0 Activity/Package-service collapse는 Android harness 환경 실패로 분리합니다. 이를 런타임 layout 코드 결함의 증거로 세지 않았습니다.
- 현재 draw marker는 3개 layout box의 성공적 present를 확인합니다. 기존 direct mixed 성공 로그의 draw report는 `runtime_image_count=0` 및 `runtime_text_count=0`을 기록하므로, 이 capture는 이미지·글리프의 실제 paint를 검증한 것으로 해석할 수 없습니다. 예: `android-runtime-direct-max-content-reversed.log:26`, `ios-runtime-direct-min-content-forward.log:16`.
- 이 검토는 정적 코드와 이미 저장된 텍스트 로그만 확인했습니다. 현재 기기/시뮬레이터 실행, 빌드, 테스트 판정은 포함하지 않습니다.
