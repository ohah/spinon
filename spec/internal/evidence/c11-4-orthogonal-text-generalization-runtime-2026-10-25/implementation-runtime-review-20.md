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

## 2026-10-25 이전 부분 범위 재검토 기록 (후속 감사로 대체됨)

당시에는 아래 22개 경로 집합에 한정해 archive source snapshot을 보강했고, 이 범위에서만 before/after가 일치했다. 이후 PR 전체 `crates/`·`platforms/` 변경 경로는 25개이며, 앞선 집합이 네 변경 경로를 빠뜨리고 미변경 경로 하나를 포함한 사실이 확인됐다. 따라서 이 이전 PASS는 22개 경로 부분 검사만 설명하며 PR 전체 source provenance의 결속 증거로 사용할 수 없다. 당시 기록과 hash는 감사 이력으로 보존한다. 전체 25개 경로의 최종 판정은 아래 최신 재검토 항목을 따른다.

| 이전 부분 snapshot 항목 | 당시 기록 |
|---|---|
| pre-commit Git HEAD | `0c7c34b8f956a6b063a180ec68942fd0d1c8798f` |
| 부분 22-path diff SHA-256 | `26cff5b6335feca9476017b62c336c6081f1a1b5228435ab590e8d0e96656010` |
| 부분 경로 aggregate SHA-256 | `b505d00b10e143279053e58102028fd200d74593f8873552c49f056f51fd6b42` |
| 부분 집합 내 archive snapshot | product와 fixture-test 각각 before = after; 당시 두 검사 모두 `True` |
| product archive SHA-256 | `0eb62fd8e73253e5f0cacb50777efea04a90aa6ed29616bdb8414885344d0d07` |
| fixture-test archive SHA-256 | `240aa82b4d00a003654476261cb40e392843a0da143612c0072f1d4675fac024` |

이전 보고서는 22개를 “컴파일되는 소스 경로”라고 과장했다. 해당 목록은 전체 변경 inventory가 아니었고 두 test-only 파일도 포함하지 않았다. 이전 archive scan은 해당 제한 경로 집합 안의 marker 결과만 기록했다. 최신 전체 경로 재검토와 archive 재생성은 아래에 따로 기록하며, 정적 archive 범위를 APK/IPA로 확대하지 않는다.

## 초기 20개 구현·런타임 검토 요약 (전체 소스 재검토 전)

- **구현·런타임 코드:** 검토한 서로 다른 20개 관점에서 PR을 막을 결함을 찾지 못했다.
- **증거:** 당시 #20 증거 출처 결속 FAIL은 22개 경로 subset에서만 보완된 것으로 판정했다. 후속 감사에서 전체 PR 변경 경로에는 불완전한 범위임을 찾아 이 부분 판정을 대체했으며, 전체 25개 경로 결속 결과는 아래 최신 재검토에 기록한다.
- **실행 범위:** 이 검토자는 테스트나 빌드를 실행하지 않았다. 시뮬레이터와 archive 결과는 저장된 증거를 대조했다.

## 2026-10-25 후속 재검토: 전체 25개 변경 경로 및 정적 archive

### 먼저 기록한 부분 범위 결함

앞선 22개 경로 snapshot은 PR의 전체 소스 변경 목록이 아니었다. 기준 HEAD `0c7c34b8f956a6b063a180ec68942fd0d1c8798f`와 PR HEAD `8f825a10ce51403a7838fe564c0253483be24b16`의 `crates/`·`platforms/` 변경 경로를 다시 비교한 결과 25개였다. 앞선 목록은 `crates/spinon-layout/src/c15_text_layout_tests.rs`, `crates/spinon-layout/src/intrinsic_size_tests.rs`, `crates/spinon-layout/src/lib.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs`를 빠뜨렸고, 변경되지 않은 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows.rs`를 포함했다. 따라서 직전 22개 경로 scan은 그 경로들에 한정한 결과였고, PR 전체 source inventory의 결속 증거로는 불완전했다. 최초 #20 provenance FAIL과 직전 22개 경로 snapshot은 지우지 않았으며 `matrix-metadata.json`의 `originalPartialSourceSnapshots`에 그대로 보존했다.

### 수정된 전체 경로 검토

현재 capture harness와 archive scanner는 기준 commit부터 candidate HEAD까지 `crates/`·`platforms/`에서 바뀐 경로 집합이 고정된 25개 경로 집합과 정확히 같아야 통과한다. 두 gate 모두 candidate 경로의 staged 및 unstaged diff가 남아 있으면 중단한다. 현재 candidate 경로는 clean이며, source 파일별 hash·aggregate·diff hash를 독립적으로 다시 계산해 저장 snapshot과 대조했다. 누락됐던 `c15_text_layout_tests.rs`와 `intrinsic_size_tests.rs`는 변경 inventory와 파일 hash에 포함했다. `lib.rs`에서 두 모듈은 `#[cfg(test)]` 아래 선언되므로 이 두 파일을 정적 archive에 링크되는 파일이라고 주장하지 않는다.

| 결속 항목 | 재검토 결과 |
|---|---|
| 기준 HEAD / candidate HEAD | `0c7c34b8f956a6b063a180ec68942fd0d1c8798f` / `8f825a10ce51403a7838fe564c0253483be24b16` |
| 전체 candidate diff SHA-256 | `d3912cbe66106f5848ce99dd7aa25733284422be1c4fc5d06b1aa33cab295f79` |
| 전체 경로 수 / 파일별 hash aggregate | 25개 변경 경로 / `82fccfc0f54eabfc6444fc33424b05cde2e315134c5611328e04bd4eaac92eeb` |
| 경로 목록 및 현재 파일 내용 | commit의 `crates/`·`platforms/` 변경 경로와 정확히 일치; 25개 파일 hash 일치; staged·unstaged candidate 변경 없음 |
| product archive 전후 source snapshot | 기준 snapshot과 before/after 모두 일치; `True` |
| fixture-test archive 전후 source snapshot | 기준 snapshot과 before/after 모두 일치; `True` |
| product archive | `0eb62fd8e73253e5f0cacb50777efea04a90aa6ed29616bdb8414885344d0d07`; scene ID 0/10, fixture/module symbol 없음, `candidateModuleSymbolRows` 0행 |
| fixture-test archive | `240aa82b4d00a003654476261cb40e392843a0da143612c0072f1d4675fac024`; scene ID 10/10, fixture FFI symbol과 module symbol 1행 존재 |

`scan-results.json`의 source snapshot은 `matrix-metadata.json`의 전체 25개 경로 snapshot과 동일하다. 두 archive build log는 각각 전체 25개 경로의 before/after hash를 기록하고 snapshot 일치를 `True`로 표시한다. 양쪽 archive 파일의 실제 SHA-256도 scan 결과와 일치하고 `llvm-nm`은 두 archive에서 exit 0이다. 제품 archive의 negative control에는 scene ID, 고유 DOM prefix/template parts, fixture FFI export, candidate module symbol 행이 없고, fixture-test positive control에는 열 개 scene ID와 fixture symbol들이 나타난다. 스캔 범위는 macOS ARM64 `spinon-ffi` static archive다. APK/IPA 전체에서 fixture가 제외됐다는 증거로 해석하지 않는다.

시뮬레이터 증거는 기존 파일을 다시 실행하지 않고 대조했다. metadata의 Android·iOS 결과는 각각 22/22이며, 모든 row에 서로 다른 scene/run/process ID, 정확히 세 개의 process-bound grid node frame, scene-bound draw와 2개 연속 동일 screenshot sample이 기록돼 있다. 44개 scene의 runtime log, launch log, PNG hash가 metadata와 일치한다. Android APK와 iOS executable을 다시 빌드한 결과 hash도 보존된 matrix hash와 같으며, metadata의 `simulatorScenesRerun`은 `false`다.

근거: `capture-simulator-matrix.py:63-90,111-138,468-535`; `crates/spinon-layout/src/lib.rs:28-30,660-662`; `matrix-metadata.json:1-36,3473-3610`; `README.md:30,41-43`; `artifact-boundary/build-and-scan.py:34-73,140-180`; `artifact-boundary/scan-results.json:2-36,169-276`; `artifact-boundary/product-build.log:2-72`; `artifact-boundary/fixture-test-build.log:2-72`; `artifact-boundary/README.md:1-30`.

**최종 후속 판정:** 전체 25개 경로 source inventory 및 그 범위로 재생성된 정적 archive 증거는 PASS다. 앞선 22개 경로 판정은 부분 범위였다는 finding을 보존하고, 이를 전체 PR source 결속의 증거로 사용하지 않는다. 최초 #20의 provenance FAIL도 기록에서 지우지 않는다. 이 검토자는 테스트·빌드·시뮬레이터를 실행하지 않았고 저장된 snapshot, 로그, 산출물과 hash만 읽어 대조했다.
