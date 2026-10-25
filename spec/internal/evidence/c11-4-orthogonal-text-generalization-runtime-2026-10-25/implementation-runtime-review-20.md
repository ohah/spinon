# C11.4 구현/runtime 검토 (20개)

## 범위와 최초 판정

C11.4 직교 텍스트 intrinsic 일반화 계획, 현재 구현 diff, 시뮬레이터 증거, 산출물 경계 보고서를 읽기 전용으로 검토했다. 검토 과정에서 테스트·빌드·브라우저·시뮬레이터를 실행하지 않았다. 기록된 화면 캡처 SHA-256과 저장된 PNG 파일의 해시만 대조했다.

**최초 판정:** 검토한 구현·런타임 코드에서 이 작업 범위를 막을 결함은 찾지 못했다. 최초 산출물 검사는 Git `HEAD`만 기록해 빌드 당시의 수정 중인 후보 소스와 연결되지 않았다. 당시 검사 결과를 제출 patch에 결속된 증거로 볼 수 없다고 판정했다. 아래 #20과 후속 재검토에 최초 FAIL 및 해결 기록을 모두 남긴다.

## 검토 항목

| # | 판정 | 서로 다른 불변 조건 / 실패 경로 | 근거 |
|---:|---|---|---|
| 1 | PASS | 논리 intrinsic 입력 허용 판정은 지원하는 horizontal-tb/mixed 경로와 vertical-rl/mixed 프로파일만 받는다. 내부 writing-mode/orientation enum에도 다른 값을 표현할 variant가 없어 다른 조합이 우연히 허용되지 않는다. | `crates/spinon-text-layout/src/engine/intrinsic.rs:43-54`; `crates/spinon-core/src/writing.rs:6-20` |
| 2 | PASS | 세로쓰기 프로파일은 비어 있지 않은 ASCII 영문자·숫자와 U+0020만 허용한다. 빈 문자열, CJK, 구두점, 탭, 개행 입력은 거부한다. | `crates/spinon-text-layout/src/engine/intrinsic.rs:243-255`; `crates/spinon-text-layout/src/tests.rs:317-322` |
| 3 | PASS | 여러 텍스트 원본, inline 조상, inline box style, inline run style가 있는 입력을 거부해 지원하지 않는 descendant를 단일 Ahem 측정에 합치지 않는다. | `crates/spinon-text-layout/src/engine/intrinsic.rs:245-255`; `crates/spinon-text-layout/src/tests.rs:368-404` |
| 4 | PASS | 세로쓰기 측정은 Ahem 10px, 명시적인 10px line-height, weight 400, normal font style, normal whitespace/word-break/line-break, LTR, normal ligature로 제한한다. 변경된 활자 스타일 값에 대한 입력 거부 대조가 있다. | `crates/spinon-text-layout/src/engine/intrinsic.rs:256-266`; `crates/spinon-text-layout/src/tests.rs:324-366` |
| 5 | PASS | CSS family 이름이 맞는 것만으로 미등록 글꼴을 허용하지 않는다. `build_parley_layout`이 실제 font collection에서 사용 가능 여부를 확인해 `MissingFontFamily`를 반환하며, 등록은 family 검증 성공 후에만 registry revision을 올린다. | `crates/spinon-text-layout/src/engine/intrinsic.rs:115-123`; `crates/spinon-text-layout/src/engine.rs:393-405`; `crates/spinon-text-layout/src/engine/fonts.rs:47-74` |
| 6 | PASS | 정상 intrinsic 쌍과 whitespace 정규화가 고정 프로파일의 기대값과 일치한다. `ABC`는 30/10, `A BC`는 20/20 및 40/10, 세 단어는 30/30 및 80/10, 공백을 접은 문자열은 20/20 및 40/10, 공백만 있는 문자열은 0/0이다. 반복 줄 배치도 결정적이다. | `crates/spinon-text-layout/src/tests.rs:195-262`; `plan/c11-4-orthogonal-text-intrinsic-generalization.md:33-50`; 런타임 대상 표 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/scene.rs:70-139` |
| 7 | PASS | fixture HTML은 `text-orientation`을 직접 선언하지 않는다. text item은 mixed orientation을 상속하며, computed root/text/image writing style은 각각 horizontal-tb/mixed, vertical-rl/mixed, horizontal-tb/mixed인지 검사한다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs:124-126`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/style.rs:120-165`; projection `crates/spinon-style-to-layout/src/projection.rs:624-640` |
| 8 | PASS | `line-break:auto`는 기존 장면을 위한 호환값이다. `LineBreakMode`는 Normal/Strict만 표현하므로 text-layout 입력으로 Auto를 넘길 수 없다. 공용 projection은 기존 장면을 위해 computed Auto를 허용하지만, 새 10개 대상은 장면별 exact style 검사에서 computed Normal을 요구한다. “공용 projection이 Auto를 전부 거부해야 한다”는 해석은 오탐이다. 공용 gate를 바꾸면 기존 네 장면의 동작을 깨뜨린다. | `crates/spinon-text-layout/src/model.rs:90-96`; `crates/spinon-text-layout/src/engine/intrinsic.rs:256-266`; `crates/spinon-style-to-layout/src/projection.rs:642-655`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/scene.rs:232-237`; `.../orthogonal_rows/style.rs:128-149` |
| 9 | PASS | computed vertical node 집합이 paragraph 입력과 다르거나 vertical paragraph가 정확히 하나가 아니거나, root/item computed writing style이 예상 조합과 다르면 projection이 fail closed 한다. | `crates/spinon-style-to-layout/src/projection.rs:581-640` |
| 10 | PASS | horizontal-tb Grid 안의 vertical-rl child에서 logical block은 physical width로, logical inline은 physical height로 변환한다. row contribution에는 선택한 logical inline 값이 사용된다. | `crates/spinon-layout/src/intrinsic_measurement.rs:242-265,318-329`; runtime 검사 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:182-195` |
| 11 | PASS | orthogonal contribution input은 paragraph, revision, writing-mode/orientation, logical size가 서로 맞지 않으면 거부한다. 물리축 input을 만들기 전에 유한하지 않거나 음수인 크기도 거부한다. | `crates/spinon-layout/src/intrinsic_measurement.rs:241-265,311-316` |
| 12 | PASS | 측정된 `TextIntrinsicLogicalSizes` 전체가 Grid item source와 contribution source로 복제된다. opaque registry token도 함께 유지되어 provenance가 숫자 revision만으로 축약되지 않는다. | `crates/spinon-layout/src/calc_tree/text.rs:175-193`; `crates/spinon-layout/src/intrinsic_measurement.rs:273-307` |
| 13 | PASS | 서로 다른 registry의 숫자 font revision이 같아도 opaque token으로 구별된다. negative control이 이를 확인하고, 값 비교도 token identity를 비교한다. | `crates/spinon-text-layout/src/snapshot.rs:202-225`; `crates/spinon-layout/src/grid_text_stretch_tests.rs:342-356`; FFI negative control `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:440-449` |
| 14 | PASS | FFI는 JSON 직렬화 전에 measurement, Grid item, 파생 contribution 간 registry identity를 확인한다. opaque token은 JSON에 노출하지 않고, Rust 측 identity 검사를 통과한 뒤 logical 값과 숫자 revision만 보고한다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:48,69-92,214-295,430-438` |
| 15 | PASS | measurement와 item input revision은 완료된 source generation, document, render tree, style, environment, intrinsic-size revision에 맞춰 기록·검증된다. 완료된 runtime key가 다르면 거부한다. | `crates/spinon-layout/src/calc_tree/intrinsic_measurements.rs:205-213`; runtime `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:80-107`; FFI `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:19-48` |
| 16 | PASS | FFI dispatcher는 고정 target ID 10개만 노출하고 각 ID에 고정 text/track을 연결한다. 알 수 없거나 주입된 ID는 거부한다. Chrome 전용 no-image whitespace control 두 개는 모바일 route에 넣지 않는다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs:10-68,103-143`; 계획 `plan/c11-4-orthogonal-text-intrinsic-generalization.md:45-46,56` |
| 17 | PASS | 런타임 장면 matrix는 신규 대상 10개와 기존 회귀 12개로 정확히 구성된다. Android와 iOS 증거에는 동일한 scene ID 22개가 각각 한 번씩 있고 플랫폼별 통과 수는 22개다. | `capture-simulator-matrix.py:27-58`; `matrix-metadata.json:1101-1104`; `README.md:7-14` |
| 18 | PASS | Android와 iOS는 신규 대상 전체에서 같은 intrinsic 쌍과 row 계산을 기대한다. 저장된 matrix는 플랫폼별 22개 모두 native matcher 및 draw/present PASS를 기록한다. 읽기 전용 검사에서 PNG 44개 모두 metadata의 SHA-256과 일치했다. | Android `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:4686-4710`; iOS `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:3511-3533`; `matrix-metadata.json` 관측값 및 `README.md:7-20` |
| 19 | PASS | FFI fixture module은 fixture-test feature에서만 컴파일된다. 정적 archive 비교에서 기본 archive에는 대상 ID, C11.4 DOM prefix, fixture FFI export가 없고 fixture-test archive에는 포함 대조가 있다. 보고서도 검사를 macOS ARM64 static archive에 한정하며 APK/IPA로 확대 해석하지 않는다. | `crates/spinon-ffi/src/runtime_gpu.rs:29-30`; `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:91-103`; `artifact-boundary/README.md:11-29` |
| 20 | **초기 검토: FAIL — 증거 출처 결속** | 최초 artifact-boundary 보고서는 `sourceHead`만 기록하고 수정 중인 후보의 working-tree/diff 또는 implementation-file hash를 기록하지 않았다. 따라서 archive hash와 fixture 경계 결과는 관찰값이지만, 당시에는 제출 patch와 정확히 연결된 결과라고 확인할 수 없었다. 코드 결함은 아니다. 최초 FAIL은 당시 판정을 보존한다. **2026-10-25 후속 재검토에서 해소 여부를 다시 판정한다.** | 2026-10-25 최초 검토 판정. 이 문서의 이 항목에 당시 결함을 보존하고, 아래에 재생성 scan 결과를 별도로 기록한다. |

## 2026-10-25 후속 재검토: #20 증거 출처 결속 FAIL 해소

재생성된 산출물 경계 검사에서 빌드 전후 소스 snapshot이 product archive와 fixture-test archive 모두에서 같고, 지정된 후보 snapshot과도 일치하는 것을 확인했다. `implementationDiffSha256`은 컴파일되는 소스 경로 22개에 한정한 binary diff의 SHA-256이다. 문서와 증거 파일은 이 diff에서 제외한다. 이 값과 별도로 22개 경로 각각의 현재 파일 내용 hash를 기록하고 aggregate hash를 계산했으며, 새로 추가된 untracked FFI 소스도 포함한다. 전후 snapshot과 두 archive hash는 `artifact-boundary/README.md:3-14`, `artifact-boundary/scan-results.json:2-29,31-150,156,164-207`, `artifact-boundary/product-build.log:2-64`, `artifact-boundary/fixture-test-build.log:2-64`에서 대조된다.

| 결속 항목 | 재검토 결과 |
|---|---|
| Git HEAD | `0c7c34b8f956a6b063a180ec68942fd0d1c8798f` |
| 컴파일 소스 22개 경로에 한정한 implementation diff SHA-256 | `26cff5b6335feca9476017b62c336c6081f1a1b5228435ab590e8d0e96656010` |
| 경로별 파일 hash 및 aggregate SHA-256 | 컴파일 소스 경로 22개를 각각 hash; 문서·증거 제외, untracked FFI 소스 포함; aggregate `b505d00b10e143279053e58102028fd200d74593f8873552c49f056f51fd6b42` |
| 소스 snapshot 비교 | product와 fixture-test 각각 before = after = 기준 snapshot; 두 검사 모두 `True` |
| product archive SHA-256 | `0eb62fd8e73253e5f0cacb50777efea04a90aa6ed29616bdb8414885344d0d07` |
| fixture-test archive SHA-256 | `240aa82b4d00a003654476261cb40e392843a0da143612c0072f1d4675fac024` |

product archive에는 대상 ID 0/10, 고유 DOM prefix 없음, fixture FFI symbol 없음이 유지되고, fixture-test archive에는 대상 ID 10/10과 fixture FFI symbol이 있다. 기록된 두 archive hash와 전후 snapshot은 재생성된 `artifact-boundary/README.md`, `scan-results.json`, `product-build.log`, `fixture-test-build.log`에서 서로 확인된다.

**후속 판정:** #20은 PASS로 변경한다. 최초 검토의 FAIL은 당시 상태를 보존하기 위해 위 표에 남겼다. 현재 static archive 결과는 후보 소스 snapshot에 결속되어 있다. 이 결과의 범위는 macOS ARM64 `spinon-ffi` static archive이며 APK/IPA 검증을 뜻하지 않는다.

## 최종 판정

- **구현·런타임 코드:** 검토한 서로 다른 20개 관점에서 PR을 막을 결함을 찾지 못했다.
- **증거:** 최초 #20 증거 출처 결속 FAIL은 소스 결속 archive scan 재생성으로 해소됐다.
- **실행 범위:** 이 검토자는 테스트나 빌드를 실행하지 않았다. 시뮬레이터와 archive 결과는 저장된 증거를 대조했다.
