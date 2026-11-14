# E08 시작 시간 최적화 실험 계획

- **상태:** 계획 초안 · 측정 미실행 · E08 미완료 유지
- **공식 항목:** [E08 시작 시간 최적화 실험](../spec/STATUS.md)
- **공개 계약 경계:** [0002 UI 트리·이벤트](../spec/0002-ui-tree-events.md) · [0004 실행·빌드·배포](../spec/0004-runtime-build.md)
- **고정 엔진 기준:** [V8 빌드 기준](../native/v8/VERSION.md)

## 목적과 범위

E08은 앱 시작 경로에서 번들 사전 변환, V8 code cache, V8 startup snapshot 후보의 효과와 호환성을 각각 측정한다. 이 문서는 측정 설계만 고정한다. 소스 코드를 바꾸거나 성능 결과를 만들지 않는다.

현재 `native/v8/VERSION.md`가 고정한 V8 15.6.0 revision은 후보 빌드 기준이며 안정된 ABI·artifact 호환 계약이 아니다. 측정마다 정확한 V8 revision과 빌드 설정을 기록한다. R11 CSS 조사 결과는 E08의 선행조건이나 성능 근거로 사용하지 않는다.

### 분리할 후보

| ID | 후보 | 실험 변수 | 현재 계획의 경계 |
|---|---|---|---|
| A | 번들 사전 변환 | V8 실행 중 반복되는 transform 단계가 실제 존재한다면 그 단계만 동일 의미의 빌드 산출물로 옮긴다. | E08의 “사전 변환”이 transpile, bundle, minify 중 무엇을 뜻하는지 아직 확정되지 않았다. 현재 `bundle:bootstrap`이 생성하는 파일만 보고 변환 효과를 가정하지 않는다. 기준 실행에서 이동 가능한 반복 작업과 입력·출력 의미를 특정하기 전에는 A를 실행하지 않는다. |
| B | V8 code cache | 동일한 JS bytes와 compile options에 대해 생성한 V8 compile cache를 새 process의 compile 경로에서 소비한다. | source 변경, V8 revision, ABI, 플랫폼 또는 compile options가 달라진 cache를 재사용하지 않는다. artifact rejection/fallback 동작도 기록한다. |
| C | V8 startup snapshot | 고정된 JS 초기화 일부를 snapshot creator로 직렬화하고 같은 기준 엔진이 새 Isolate에서 복원한다. | host callback, platform pointer, 세션별 상태가 snapshot에 들어간다고 가정하지 않는다. 직렬화 가능한 초기화 경계를 확정하지 못하면 C는 중단한다. |

A·B·C는 각각 baseline과 비교한다. 조합형 A+B, B+C, A+B+C는 각 단일 후보에서 의미 동등성·호환성·성능 이득을 확인한 뒤 별도 계획 검토를 거치기 전까지 범위 밖이다. X01의 ESM 청크 로더나 Rspack 통합은 이 실험에 포함하지 않는다.

## 관찰 경계와 질문

현재 [`RuntimeSession`](../crates/spinon-runtime/src/session.rs)의 `session_startup_us`는 세션 생성 중 `RuntimeUaCascadeCoordinator` 준비도 포함한다. [`spinon_v8.cc`](../native/v8/src/spinon_v8.cc)는 V8 platform 초기화, Isolate와 Context 생성, facade compile/evaluate 경로를 소유한다. 이 값들을 하나의 “V8 startup” 수치로 합치지 말고 단계를 따로 timestamp한다. 현재 C++ 경로의 facade는 default `v8::Script::Compile` 호출을 사용하므로, cache 후보는 compile API overload, `ScriptOrigin`, script/module mode, compile option을 별도로 명시해야 한다.

계측은 플랫폼별 monotonic clock으로 같은 프로세스 안의 경과 시간을 잰다. 각 stage에 시작·종료 timestamp와 duration을 기록하고, nested stage duration은 process total에 다시 더하지 않는다. 실제 호출 순서·중첩은 사전 baseline에서 확인한다. 한 stage가 다른 stage를 기다리는지 표시하고, stage 합계가 전체 시간과 같다고 가정하지 않는다.

1. 프로세스 시작과 앱 native entry.
2. V8 platform initialization 시작·완료.
3. runtime/Isolate/Context 생성 시작·완료.
4. host facade compile 시작·완료와 evaluate 시작·완료.
5. 고정 앱 bundle compile 시작·완료와 evaluate 시작·완료.
6. worker 시작, `RuntimeSession` 생성 시작, UA/CSS worker ready, `RuntimeSession` ready.
7. 첫 layout/cascade snapshot 완료.
8. GPU queue submit.
9. 플랫폼 presentation callback이 있다면 별도 event로 기록한다.

주 비교 지표는 1–6의 고정 stage 시간과 artifact 크기다. 7은 명시된 fixture에서 별도 보조 지표로 둔다. `0002-ui-tree-events.md`는 초기 JavaScript와 첫 GPU frame의 순서, 초기화 지연·실패 시 표시 동작이 미정이라고 한다. GPU submit, `Queue::present()` 또는 플랫폼 presentation callback은 각각 따로 보존하며 첫 픽셀이 실제 표시됐다는 증거로 간주하지 않는다. 그러므로 이 실험은 제품의 “첫 화면/첫 픽셀 startup” 성능을 주장하지 않는다. 실제 표시 지표로 쓸 수 있게 되려면 해당 presentation 계약과 독립 증거가 먼저 필요하다.

### 비교에 사용할 관찰 oracle

기본 비교 입력은 `index.html`과 `root-app.js`의 현재 Android/iOS bootstrap 경로로 고정한다. `app.js`의 별도 smoke 경로를 섞지 않는다. route가 실제 플랫폼 앱 진입에서 사용되지 않으면 그 플랫폼 integrated run은 보류하고 그 사실을 기록한다. baseline과 각 변형은 동일한 document, viewport/environment, host API profile, route를 써야 한다.

의미 동등성은 `root-app.js`의 사전 assertion이 검증하는 고정 관찰값으로 판정한다: root는 `html[lang=ko]`, viewport meta는 `width=device-width, initial-scale=1`, color-scheme meta는 `light dark`, style text는 `:root { color-scheme: light dark; }`, `main#app` 초기 text는 `Spinon 루트 HTML`, evaluate 후 `data-runtime=ready`다. 추가로 RuntimeSession ready 결과와 초기화 오류 없음, 첫 layout snapshot의 존재와 fixture key를 저장한다. 이 oracle bytes와 expected JSON을 baseline 측정 전에 고정한다. `correctness hash` 하나로 서로 다른 의미 필드를 숨기지 않고 각 field와 원본 hash를 남긴다.

## 비교 대상과 입력 고정

기준 B0는 현재 고정된 플랫폼 V8 빌드와 현재 앱 bootstrap 경로다. 시작 입력은 [bootstrap package script](../package.json), `examples/bootstrap/index.html`, `examples/bootstrap/root-app.js`로 후보를 특정하되, `build/spinon/bootstrap` 산출물과 Android/iOS가 실제 실행하는 bytes가 일치하는지 먼저 확인한다. baseline artifact는 확인 뒤 run-owned directory에 복사하고 원본·복사본의 hash를 기록한다. E08 variant를 만들기 위해 shared `build/spinon/bootstrap`을 다시 생성하거나 덮어쓰지 않는다. 실제 앱이 `root-app.js`가 아닌 다른 파일을 실행하면 계획 0단계에서 해당 경로로 수정·재검토한다.

각 run은 아래를 저장한다.

- Git commit, 관련 source tree status, 모든 JS/HTML/CSS 입력의 SHA-256.
- `mise.toml`, `package.json`, Bun lockfile 및 해당 build tool의 버전, 정확한 build 명령과 종료 코드.
- V8 full commit, GN args/build flags, V8 static library hash, `native/v8` adapter source hashes, target platform, OS version, device/simulator profile, target ABI/CPU, JIT/JITless mode.
- 선택한 bootstrap entrypoint와 host API profile, RuntimeSession profile, fixture input hash, output/correctness hash.
- process 상태, persistent cache 상태, artifact 생성/소비 순서, 각 stage monotonic timestamp, raw stdout/stderr, 실패·fallback 이유.
- code cache/snapshot 및 bundle의 bytes, SHA-256, artifact format/version, 생성기 executable/adapter/library identity, 생성 시간, 저장·읽기 시간과 API가 artifact를 실제로 채택/복원했음을 나타내는 consume result.

원본 fixture와 변형 산출물은 실행 ID·variant·platform·ABI별 directory에 둔다. 기본 `target/`과 `build/spinon/bootstrap`을 서로 다른 후보가 공유하지 않는다. variant별 output root와 Cargo target dir를 고정하고, Android/iOS 빌드 산출물도 독립 경로를 쓴다. 도구가 output root를 선택할 수 없으면 각 변형을 격리 checkout에서 순차 생성하고, 완료 직후 run-owned directory로 복사·hash한 뒤에만 다음 변형을 빌드한다. baseline은 확인된 기존 bytes의 복사본을 사용하고 공유 경로에서 재빌드하지 않는다. 실행 중 source나 toolchain을 바꾸지 않는다.

## 실행 단계와 판정

| 단계 | 실행 내용 | 통과 조건 |
|---|---|---|
| 0. 입력·경계 확정 | Android/iOS 실제 bootstrap entry, 앱 JS bytes, V8 boundary, A의 의미, baseline stage trace를 읽기 전용으로 고정한다. 선택 후보의 실제 API overload·artifact producer/consumer path·consume signal을 pinned V8 headers/source로 입증한다. | 실행 입력을 경로·hash로 특정하고, A를 구체적으로 정의하거나 근거와 함께 해당 후보를 `미적용` 처리한다. pinned source/API에서 경계·artifact 소비를 입증하지 못한 후보는 `미해결`로 남기고 구현 및 GN 설정 변경을 멈춘다. RuntimeSession ready와 first-pixel을 구별한다. |
| 1. baseline B0 | 각 지원 실험 플랫폼에서 동일 fixture와 app route를 사용해 no-cache baseline을 반복 측정한다. | 각 stage의 분산과 계측 overhead를 보고하고, candidate와 비교할 기준을 미리 고정한다. |
| 2. 단일 후보 A/B/C | 후보 하나만 활성화하고 다른 source, flags, bundle, route, host API는 B0와 고정한다. | 각 field와 사전 oracle이 baseline과 같은지 확인하고 baseline 대비 시간·artifact·memory 차이를 분리한다. |
| 3. 호환성 negative controls | cache/snapshot 생성 뒤 source bytes, compile API/`ScriptOrigin`/compile options, facade/adapter source, V8 revision·library hash, build flags, JIT mode, ABI 또는 platform을 하나씩 바꾼다. 별도로 artifact를 byte-flip, truncate, metadata 누락·모순, cache 부재 상태로 만든다. | 호환성 또는 무결성이 입증되지 않은 artifact는 평가 전에 명시적으로 거부된다. code cache rejection/consume status와 snapshot restore path를 기록한다. 해당 V8 API에 신뢰할 consume signal이 없으면 “사용됐다”는 성능 결론을 내지 않는다. |
| 4. 재현성·정리 | 원본 로그, hashes, manifest, variant별 산출물을 검사하고 task-owned temp만 분류한다. | 다른 사람이 같은 pinned inputs로 같은 case를 재현할 수 있고, 실패·손상 artifact를 포함한 증거가 보존된다. cleanup 전에 정확한 run directory에 대한 활성 process/file handle이 없음을 확인한다. |

실험 cohort를 혼합하지 않는다. ① `fresh-process/no-artifact`: 새 앱 process·새 Isolate, task-owned persistent cache 없음. ② `fresh-process/valid-artifact`: 새 앱 process·새 Isolate, 직전 run에서 만든 해당 variant의 persistent artifact만 있음. ③ `same-process/new-isolate`: 같은 앱 process에서 새 Isolate. ④ `same-isolate/re-evaluate`: 진단용 script 재평가이며 startup 개선 수치에는 포함하지 않는다. ②/③/④는 서로 별도 cohort다. `fresh-process`는 process만 새롭다는 뜻으로 OS page cache·thermal state가 cold하다는 주장이 아니다. 시스템 cache는 임의로 지우지 않는다. 각 run 전 persistent cache state, artifact create/consume history, process identity를 기록한다.

후보별 전용 startup artifact가 없는 B0와 비교하여 process starts를 variant 순서를 섞어 수집한다. 수집 순서는 cohort 안에서 사전 생성한 seed의 randomized AB/BA blocks로 정하고 각 block에서 같은 device/OS profile을 사용한다. 기본 계획은 platform/build cohort별 variant당 최소 30개의 독립 process start이며, 같은 Isolate의 반복은 독립 표본으로 세지 않는다. 중앙값·p90을 주 요약으로 하고, p95는 variant당 100회 이상일 때만 보고한다. median 및 paired block delta의 95% CI는 사전 고정한 bootstrap 절차와 resample 수로 계산한다. thermal throttling, background load, build/install 시간, process 종료 실패 및 측정 누락은 원시 기록을 보존하고 run ID와 제외 사유를 남기며, outcome을 보기 전에 고정한 기준에 따라 제외하거나 별도 표시한다. simulator 값은 simulator 결과로만 보고하며 실제 기기 지연·에너지·배터리 성능으로 외삽하지 않는다.

번들 transform은 build wall time과 결과 bytes를 startup time과 별도로 보고한다. 계측 오버헤드는 동일한 B0에서 계측 on/off process를 무작위 교차 실행해 측정하고, 그 오버헤드·분산을 모든 후보의 trace로 보정한다. 보정 오차가 baseline noise와 분리되지 않으면 성능 비교는 중단한다. 모든 후보에서 root-app oracle의 각 field와 초기 오류 상태가 동일해야 한다. 시작 지연 개선이 artifact 크기, RSS/resident memory, 생성·설치 시간, startup failure 또는 cache invalidation 비용과 맞바뀌면 각각 함께 보고한다. baseline pilot은 분산과 noise floor를 추정하는 데만 쓴다. candidate 결과를 열기 전에 계획 변경으로 최소 효과 threshold, memory·artifact size budget, 제외 규칙 및 비교 분석 절차를 고정하고 영향받은 plan review를 다시 통과한다. 제품 목표나 사전 threshold를 정하지 못하면 결과를 탐색값으로만 게시하고 E08 완료 근거로 쓰지 않는다.

## 버전·플랫폼 호환성

V8 code cache와 startup snapshot은 현재 실험 빌드에 묶인 파생물로 취급한다. cache key에는 적어도 V8 full revision·static library hash, `native/v8` adapter source hash, relevant GN/build flags, target platform/ABI, OS/device profile, JIT/JITless mode, source bytes hash, source origin, script/module mode, `ScriptOrigin`, compile options, facade/bundle version, artifact format/version을 넣는다. snapshot에는 같은 항목 외에 snapshot seed inputs와 generator executable identity를 기록한다. 현재 adapter call이 사용하는 실제 V8 API overload와 cache/snapshot artifact producer·consumer path를 pinned V8 headers에서 확인해 manifest에 남긴다. `v8_use_external_startup_data=false` 등 현재 startup data 조건도 기록한다. 누락 필드가 있으면 cross-run/cross-build 재사용은 금지한다.

동일한 engine/build/input에서 생성·소비하는 positive control은 각 후보 artifact가 실제로 사용됐음을 V8 API의 rejection/consume/restore 결과와 stage trace로 확인한다. file hash만으로 cache hit나 snapshot restore를 판정하지 않는다. snapshot restore 뒤에는 새 Isolate마다 해당 세션의 `SpinonV8Runtime*`, `user_data`, callbacks를 새 포인터로 bind하고 첫 script evaluation 전에 세션 격리를 확인한다. snapshot seed에 process/isolate pointer, callback 또는 session-specific state가 직렬화되면 실패다. V8 revision, adapter/library hash, platform/ABI, JIT mode, build flag, source/compile option 또는 artifact format 중 하나가 다른 negative control은 안전하게 거부되고 기준 compile/rebuild로 fallback한다. miss, stale metadata, V8 rejection, corrupt/truncated file은 서로 다른 결과 코드로 기록하고 task-owned artifact만 invalidate한다. V8 15.6.0 후보 pin은 version compatibility promise가 아니며, 다른 revision에 걸친 호환을 전제하지 않는다.

Android와 iOS는 각각 별도 cohort다. 최초 실행 설계는 Android ARM64 emulator API/AVD profile과 iOS Simulator OS/device profile을 고정하며, 실제 사용 가능한 runtime configuration이 다르면 같은 cohort에 합치지 않는다. iOS Simulator의 JIT 설정을 실제 iOS의 JITless 조건으로 가장하지 않는다. 사용자 요청이 없는 실기기 연결은 하지 않는다. Simulator만 측정하면 결론도 Simulator로 제한한다. 실제 실기기 결과가 없으면 physical startup improvement를 주장하지 않는다. 각 platform candidate의 V8 build output은 격리하고, 같은 default bootstrap bundle 및 V8 checkout/build output을 병렬 variant에 재사용하지 않는다.

## 자원 격리·선행 조건

2026-11-14 초안 작성 시점에 main checkout은 HEAD `401911e6b78b543e3c9553c3e6e162e43ca381b2`에서 C11.4 120,960-scene `RuntimeSession` matrix를 실행 중이었다. 실행 process는 PID 98424였고 raw log는 main checkout 안의 `spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-mixed-auto-minimum-runtime-2026-11-14/sibling-one-fit-mixed-row-span-120960-current-candidate-pass01.log`에 기록 중이었다. 이 log는 당시 untracked 상태였고 E08 근거로 포함하거나 링크하지 않는다. E08 실행 전 C11 terminal result, 최종 HEAD와 log hash를 E08의 별도 run evidence에 고정한다. 이 기록은 plan 작성 시점의 동시성 근거이며 E08 결과가 아니다. C11 run이 끝나고 그 결과를 기록할 때까지 E08의 Rust/V8 build, app launch, benchmark, startup tracing 및 cache artifact 생성은 실행하지 않는다. E08은 별도 worktree와 variant 전용 output root에서 시작하고 C11 결과를 E08 sample로 재사용하지 않는다. 다른 고부하 실험이 E08 실행 시점에 같은 build/cache 자원을 점유하면 해당 측정은 별도 시점으로 미룬다.

`spec/STATUS.md`는 E·X 독립 항목의 병렬 작업을 허용하지만, E08 실험은 목표인 X01 chunk graph와 현재 S06 단일 bundle 경계가 다르다. 이 계획은 현재 bundled app path를 선택적으로 측정하며 OTA, X01 loader, Vite/Rspack parity 또는 `runtimeId` policy를 완료하지 않는다. 제품 app route가 선택한 input bytes와 manifest/version identity를 고정할 수 없으면 integrated-app run은 보류한다.

## 중단 조건과 완료 판단

실패·stale·손상 artifact, stderr/stdout, stage trace와 관련 process metadata는 negative control 판단이 끝나고 hash manifest를 만든 뒤까지 보존한다. 정리는 run-owned temporary directory에만 한정한다. 삭제 전 exact path와 ownership을 확인하고 `ps`/`lsof` 등으로 그 path를 여는 process/file handle이 없는지 확인한다. task-owned process만 정상 종료하며 사용자·다른 session의 process는 종료하지 않는다. cleanup 후 path 부재를 다시 확인하고 command/status를 보존한다.

다음 중 하나면 해당 variant를 중단하고 결과를 `unresolved`로 남긴다.

- A의 build-time transform이 baseline의 JS/host-observable 의미와 동등한지 판정할 수 없다.
- pinned V8 API/source에서 해당 후보의 artifact producer, consumer, compatibility key와 신뢰할 consume/restore signal을 입증할 수 없다.
- cache/snapshot이 variant에서 실제 consume되었는지 증명할 수 없거나 stale/incompatible artifact가 실행된다.
- snapshot에 session-specific host callback/state를 안전하게 연결할 방법이 없다.
- stage clocks가 중첩/이중 계수되며 분리할 수 없거나 instrumentation overhead가 baseline noise보다 크다.
- process cohort, source bytes, V8 build, ABI 또는 app route가 변형 사이에서 달라진다.
- startup ordering/failure 또는 first-frame 의미를 추가로 결정해야 “시작 완료” 시점을 정의할 수 있다.
- C11.4 matrix가 아직 실행 중이거나 shared build/cache 경합을 배제할 수 없다.

E08 결과 package는 A/B/C 각각을 `측정`, `근거가 있는 미적용`, `미해결` 중 하나로 기록한다. 항목별 baseline/candidate 결과, correctness, version/platform rejection controls, size/memory/build 비용, 전체 raw evidence와 limitation을 하나로 연결한다. 어느 한 후보가 미해결이면 E08 완료 후보로 올리지 않는다. 사용자 visible first-frame 계약이 없으면 해당 측정은 명시적으로 제외한다. 구현 채택·완료 상태 및 성능 목표는 결과 확인 뒤 별도 검토하며, 이 계획 초안은 `spec/STATUS.md` 상태·공식 완료 집계를 바꾸지 않는다.

### 구현 전 필수 관문

이 계획은 [서로 다른 실패 경로 20개 적대 검토](./e08-startup-optimization-plan-review-20.md)를 거쳤고, 발견 사항을 보정한 뒤 영향을 받은 관점을 재검토했다. 그 판정은 계획 문서에 대한 것이며 측정·구현 승인이 아니다. baseline·threshold·API 경계처럼 후보 실행 기준을 바꾸면 영향받은 계획 관점을 다시 검토한다. 실험 구현 PR에는 실제 코드·런타임 검토와 별도의 PR 변경분 적대 검토가 필요하다.
