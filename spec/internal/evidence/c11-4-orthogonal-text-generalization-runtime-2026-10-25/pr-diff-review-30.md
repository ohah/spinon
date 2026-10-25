# C11.4 orthogonal text intrinsic generalization: PR diff 적대 검토 30개

## 검토 범위

`origin/main`부터 현재 로컬 candidate까지의 PR diff를 검토했다. 검토 시 HEAD는 `8f825a10ce51403a7838fe564c0253483be24b16`이며 source scope, artifact 보고서, 계획 lane 표, 구현/runtime 검토 문서가 working tree에서 갱신된 상태다. 이 리뷰 파일을 제외한 현재 PR delta는 183개 경로다. 20-view 구현/runtime 검토와 20-view 계획 검토를 반복하지 않고, PR 변경 묶음의 계약·source provenance·증거 패키징 경계를 따로 확인했다.

아래 30개 관점은 모두 현재 자료에서 통과했다. 이전 source scope의 22-path snapshot은 역사적 부분 결과로 보존되고 최종 판정은 25개 변경 source path를 대상으로 한다. 이번 diff 리뷰에서는 테스트·빌드·브라우저·시뮬레이터를 실행하지 않았다. 저장된 증거 해시와 metadata를 읽기 전용으로 대조했다.

## 서로 다른 실패 경로 30개

| # | 판정 | 확인한 실패 경로 / 계약 경계 | 근거 |
|---:|---|---|---|
| 1 | PASS | 제한 fixture candidate가 C11.4 전체 완료나 공식 완료 수 증가로 오인되지 않는다. 상태 대장과 public roadmap 모두 공식 `72/213` 및 C11.4 미완료를 유지한다. | `spec/STATUS.md:368`; `spec/public/roadmap.html:181`; runtime `README.md:64` |
| 2 | PASS | 후보의 제한 입력 profile, 미지원 문자·writing mode·glyph paint·WPT·실기기·hardware GPU 범위가 실행 근거에 드러난다. 공개 CSS 지원 선언으로 확장하지 않는다. | runtime `README.md:43-64`; `plan/c11-4-orthogonal-text-intrinsic-generalization.md:15-20,76-80`; `spec/public/roadmap.html:181` |
| 3 | PASS | Chrome-only control 둘을 모바일 target으로 세지 않는다. Chrome 사전 비교는 target 10개와 이미지 없는 control 2개로 12/12이고, 모바일은 신규 target 10개와 회귀 12개로 플랫폼별 22/22다. | precomparison `README.md:5-6,19-20`; plan `:24,35-48,58-60`; runtime `README.md:7-9` |
| 4 | PASS | Scene ID→text/track→logical inline/block pair→row 기대값이 고정 표와 일치한다: single-token min/max `30/10→30`; two-words min `20/20→20`, max `40/10→40`; three-words min `30/30→30`, max `80/10→80`; collapsed min `20/20→20`, max `40/10→40`; spaces-only image targets min/max `0/0→16`. | plan `:35-46`; precomparison `README.md:28-48`; runtime FFI scene list `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs:10-68` |
| 5 | PASS | Chrome 값만을 규범 기대값으로 순환 사용하지 않는다. Ahem 10px advance와 whitespace collapse, 독립 track 계산을 계획·사전 비교에서 먼저 고정하고 Chrome 관측을 별도 oracle로 둔다. | precomparison `README.md:24-33,52`; plan `:26-33,52` |
| 6 | PASS | 반복·양끝 U+0020 collapse 후 min/max 값이 계산되며 공백 전용 문자열이 0 contribution으로 정규화되는 경우를 별도 다룬다. | plan `:18,28-32,43-50`; precomparison `README.md:28-33,43-50` |
| 7 | PASS | 공백 전용 text와 natural-size image가 같은 row에 있을 때 16px floor를 만들고, 이미지 없는 두 Chrome control은 0px row를 유지한다. | plan `:32,45-50`; precomparison `README.md:19-20,33,45-50` |
| 8 | PASS | `TextIntrinsicLogicalSizes` block pair의 API 회귀값을 CSS block min-content 보장으로 과장하지 않는다. Grid row에는 inline component만 사용한다고 명시한다. | plan `:50,78-80`; runtime `README.md:64`; implementation review `:78-80` |
| 9 | PASS | 새 target의 FFI/runtime HTML `line-break:normal`과 computed-style validator의 `normal` 검사가 서로 맞으며 기존 4개 orthogonal 회귀의 `auto` 호환도 보존한다. | plan `:56-60`; `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs:85,132`; `crates/spinon-style-to-layout/src/projection.rs:642-655`; `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/style.rs:128-149` |
| 10 | PASS | 비어 있는 입력, 비 ASCII, punctuation, tab/newline, RTL, 잘못된 style/source topology 등 profile 밖 입력은 fixed fixture 일반화로 새로 허용되지 않는다. | plan `:15-20,56-60`; `crates/spinon-text-layout/src/engine/intrinsic.rs:243-266`; `crates/spinon-text-layout/src/tests.rs:317-404` |
| 11 | PASS | 숫자 font revision만으로 provenance를 인정하지 않는다. 서로 다른 opaque registry token negative control이 있으며 `LayoutIntrinsicMeasurementSource`부터 Grid contribution 검사까지 identity가 이어진다. | `crates/spinon-text-layout/src/snapshot.rs:202-225`; `crates/spinon-layout/src/intrinsic_measurement.rs:61-82,239-250`; `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:223-290,440-451` |
| 12 | PASS | opaque font-registry token을 host JSON으로 직렬화하지 않는다. 불일치 검증 후 숫자 revision과 logical sizes만 제한 trace에 기록한다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:69-92,214-295,430-449`; runtime `README.md:35` |
| 13 | PASS | Lane 1/2/3 소유 경로가 실제 25개 source 변경을 모두 커버한다. test caller, public re-export, non-Copy clone 조정은 bridge integration으로 분류하고, 변경되지 않은 fixture 등록 파일을 변경 목록에 넣지 않는다. | plan `:64-74`; implementation review `:61-76`; source list `capture-simulator-matrix.py:63-90` |
| 14 | PASS | Candidate source inventory가 `origin/main` 이후 실제 변경된 `crates/`·`platforms/` 경로와 정확히 25개로 일치한다. 추가 누락이나 이전의 변경되지 않은 wrapper 오포함이 없다. | matrix metadata `sourceScopeReconciliation`, `source.candidateSourceFiles`; artifact `README.md:3-8`; implementation review `:59-70` |
| 15 | PASS | 25개 개별 파일 hash와 canonical aggregate가 현재 file bytes에서 재계산한 값과 일치한다. | matrix metadata `source.candidateSourceFiles`, `candidateSourceFilesSha256`; artifact `scan-results.json:2-29`; implementation review `:67-70` |
| 16 | PASS | Candidate diff hash는 기준 HEAD `0c7c34b8f956a6b063a180ec68942fd0d1c8798f`부터 candidate HEAD까지 25개 source path에 한정된다. 문서와 증거 자료가 source hash를 자기 자신으로 순환 참조하지 않는다. | artifact `README.md:3-8`; `scan-results.json:2-29,156`; scanner `build-and-scan.py:36-80,189` |
| 17 | PASS | 이전 22-path snapshot을 삭제하거나 25-path 최종 검사로 가장하지 않는다. 누락 4개와 미변경 1개를 명시하고 원본 partial snapshot을 metadata에 보존한다. | matrix metadata `originalPartialSourceSnapshots`, `sourceScopeReconciliation`; implementation review `:34-47,59` |
| 18 | PASS | Runtime evidence의 current source 및 `finalSource`가 같은 25-path candidate snapshot과 aggregate/diff hash에 일치한다. 변경된 source file이 남아 있으면 capture/verify gate가 실패한다. | matrix metadata `source`, `finalSource`, `currentSourceVerification`; `capture-simulator-matrix.py:111-138,472-479,531-536` |
| 19 | PASS | Source correction 후 Android APK와 iOS executable을 재빌드했고 두 hash가 저장된 simulator matrix와 일치한다. Simulator scene을 다시 실행하지 않았다는 사실도 `simulatorScenesRerun:false`로 분리해 기록한다. | runtime `README.md:41-43`; matrix metadata `applicationBuildReverification`; implementation review `:78` |
| 20 | PASS | Product archive 빌드 전·후 source snapshot이 25-path 기준 snapshot과 일치한다. 빌드 중 code drift를 통과시키지 않는다. | artifact `scan-results.json:31-62`의 `sourceSnapshotChecks.product`; `product-build.log`; scanner `build-and-scan.py:83-112,161-170` |
| 21 | PASS | Fixture-test archive도 별도 build에서 25-path 기준 snapshot 전·후가 일치한다. Product 결과만 재사용하는 양성 대조가 아니다. | artifact `scan-results.json:63-94`의 `sourceSnapshotChecks.fixtureTest`; `fixture-test-build.log`; scanner `build-and-scan.py:83-112,161-170` |
| 22 | PASS | 기본 product archive에서 10개 scene marker와 고유 DOM prefix, C11.4 fixture FFI export, module symbol rows가 모두 빠진다. | artifact `README.md:13-18,22-27`; `scan-results.json`의 `results.product`; `symbols-product.txt` |
| 23 | PASS | Fixture-test archive는 10개 scene ID, fixture FFI export, Rust module symbol을 포함해 negative scan이 우연히 모두 비었다는 가능성을 통제한다. | artifact `README.md:13-18,22-27`; `scan-results.json`의 `results.fixtureTest`; `symbols-fixture-test.txt` |
| 24 | PASS | DOM root ID의 prefix와 suffix를 개별 확인하며, runtime `format!` 조립인 expanded ID 부재를 실패로 판정하지 않는다. 공통 `-grid` suffix 단독도 product 제외 marker로 오해하지 않는다. | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs`의 runtime ID 조립; artifact `README.md:13-20`; `scan-results.json`의 `domRootScanNote` |
| 25 | PASS | Chrome precomparison은 Agent Browser 0.31.1 / HeadlessChrome 156, 고정 viewport/DPR, font 및 image decode 완료 환경과 12개 scene을 기록한다. | precomparison `README.md:5-6,19-24,52` |
| 26 | PASS | Chrome observation JSON과 전체 scene PNG의 SHA-256이 README의 값과 실제 파일 bytes에 일치한다. Plan branch `da64468f67c2743852adf0f2274cc5ac3a269746`가 remote에 있고 URL 경로가 해당 evidence 파일을 가리킨다. | precomparison `README.md:8-13,42-48`; `chrome156-agent-browser-dom-geometry.json`; `chrome156-agent-browser.png`; remote plan branch ref |
| 27 | PASS | Android matrix는 22개의 고유 scene ID/run ID가 모두 native matcher와 draw/present를 통과하고, 저장 PNG 22개의 hash가 metadata에 맞는다. PNG dimension은 1080×2400이다. | runtime `README.md:7-20`; matrix metadata `observations.android`, `counts.androidPassed`; `android/*.png` |
| 28 | PASS | iOS Simulator matrix도 22개의 고유 scene ID/run ID가 모두 통과하고, 저장 PNG 22개의 hash가 metadata에 맞는다. PNG dimension은 1206×2622다. | runtime `README.md:7-20`; matrix metadata `observations.iosSimulator`, `counts.iosSimulatorPassed`; `ios-simulator/*.png` |
| 29 | PASS | 각 관측은 scene run에 묶인 정확히 세 Grid node frame, 같은 process의 summary/draw, document/render-tree revision 및 기대 root frame에 대응한다. 캡처는 settle 1초 뒤 두 번 연속 같은 framebuffer hash를 요구한다. | matrix metadata `observations.*.frameBinding`, `drawBinding`, `screenshotCapture`; `capture-simulator-matrix.py:320-390,430-470`; runtime `README.md:9` |
| 30 | PASS | 제출된 94개 `.log.gz`가 manifest 경로 집합과 정확히 일치하고 압축·복원 bytes/hash가 모두 맞는다. 복원된 machine log의 trailing-space 줄은 제거하지 않았다(1,413,872줄 중 76,394줄); 평문 simulator `.log` 원본은 PR diff에서 제외된다. | `compressed-log-manifest.json:1-4`; `package-runtime-logs.py`; runtime `README.md:47-49`; PR delta의 Android/iOS `.log.gz` 목록 |

## 발행·검증 경계

- 현재 로컬 PR diff의 source/evidence/plan 수정은 working tree에 있고 아직 remote branch에 push되지 않았다. PR #630에 이 문서와 같은 최신 diff가 보이는 시점은 해당 수정이 commit/push된 뒤다. 상위 에이전트가 Agent Browser에서 PR 본문 Android/iOS PNG attachment의 `complete=true`와 각각 1080×2400 / 1206×2622 크기를 확인했고 전용 browser session을 닫았다고 보고했다. 이 리뷰에서는 브라우저를 다시 열지 않았다.
- `spec/public/roadmap.html:181`의 plan, precomparison, runtime README, metadata, Android/iOS PNG 경로는 저장소 파일과 대응한다. Tailnet live page 반영은 merge 뒤 단계이며 이 review에서 배포됐다고 주장하지 않는다.
- Runtime README `:68`은 docs build 종료 코드 1과 기존 다른 문서의 893 dead-link error group을 명시한다. 이 review에서는 docs build를 다시 실행하지 않았고 통과로 세지 않는다.

## 최종 판정

현재 local PR diff에서 차단 결함을 찾지 못했다. 소스 범위의 이전 22-path provenance 결함은 역사적 부분 기록으로 남고, 현재 matrix와 archive gate는 전체 25개 변경 source path를 기준으로 재검증됐다. 최종 PR signoff 전에는 위 local evidence/plan 수정과 이 review를 commit/push한 뒤 remote PR diff가 같은 내용인지 확인해야 한다.
