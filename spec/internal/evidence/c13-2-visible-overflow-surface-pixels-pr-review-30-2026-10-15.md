# C13.2 visible overflow 표면 픽셀 PR 변경 적대 검토 30개

- **대상:** `origin/main` 대비 C13.2 visible overflow surface-pixel 구현, comparator, simulator evidence와 문서 변경.
- **검토 시점:** run5 재실행, PR #176 본문 게시, GitHub 이미지 렌더링 확인 뒤.
- **검토 방법:** 제품 코드 경계, feature packaging, oracle·adapter·비교기 연결, 증거·문서의 사실 범위를 서로 다른 위험 관점으로 대조했다.
- **판정:** 30개 변경 관점의 코드·증거를 대조했고, 발견한 stale 표본 경로는 수정했다. PR #176의 run5 이미지 링크도 agent-browser로 실제 로드와 dimensions를 확인했다.

| # | PR 변경이 잘못 전달될 실패 관점 | 변경 대조와 판정 |
|---|---|---|
| 1 | PR 범위가 overflow paint 검증을 넘어 CSS 기능 지원 전체로 읽힌다. | 계획·상태 문서는 고정 C13.2 fixture의 surface 표본 확인으로 범위를 제한하고 공개 CSS 지원을 주장하지 않는다. **PASS** |
| 2 | 부분 결과가 C13.2 또는 상위 C13 완료로 체크된다. | 공식 `spec/STATUS.md`는 C13.2/C13.1/C13 모두 미완료로 유지한다. 이번 후속은 별도 하위 bullet로 기록한다. **PASS** |
| 3 | Android/iOS 모두 compositor 이후 화면의 동일성을 확인했다고 오해한다. | Android는 SurfaceView PixelCopy, iOS는 `metal-drawable-before-compositor`로 분리했고 README에 iOS 한계를 명시한다. **PASS** |
| 4 | 고정 fixture ABI가 public app/API 약속으로 보인다. | 신규 constructor/request는 `fixture-test` feature와 native define 아래 있고 opt-in 화면 경로만 호출한다. **PASS** |
| 5 | C header 선언은 꺼져 있어도 Cargo symbol이 기본 archive에 남는다. | default/opt-in 별도 target 빌드에서 default symbol 없음, positive control 두 symbol 존재를 확인했다. **PASS** |
| 6 | FFI symbol은 빠져도 fixture 전용 문자열/payload가 default artifact에 포함된다. | `strings`에서 `C13.2 surface-pixel host`가 default에는 없고 opt-in에만 있음을 확인했다. **PASS** |
| 7 | Cargo feature 이름은 opt-in이어도 Android JNI define가 다른 이름으로 매핑된다. | Gradle flag `SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST`와 native `SPINON_ENABLE_C132_SCROLL_FIXTURE_TEST`, Cargo feature 전달을 script에서 대조했다. **PASS** |
| 8 | iOS scheme build에서 Cargo와 Objective-C++ compile define이 서로 다른 값을 쓴다. | `build-ios-sim.sh`, Xcode build setting, `SpinonRunner.mm` 전처리 guard가 동일 opt-in flag를 전달한다. run5 opt-in build로 확인했다. **PASS** |
| 9 | C13.2 fixture source가 `fixtures/` 경계를 벗어나 default runtime 파일에 무심코 연결된다. | 신규 Rust fixture는 `runtime_gpu/ffi/fixtures/c13_2_surface_pixels.rs` 아래 module gate로 연결되고 boundary test의 기대 설명만 갱신했다. **PASS** |
| 10 | feature-on 정적 archive만 검사해 feature-off 기본 제품 격리를 증명하지 못한다. | 두 target 디렉터리에서 default와 opt-in archive를 각각 build하고 hash, symbols, string을 대조했다. **PASS** |
| 11 | Chrome oracle fixture가 현재 runtime source와 다른 복사본이라 기준이 드리프트한다. | 고정 observation의 `fixture`와 `source_fixture` bytes/hash를 comparator가 repository 파일과 exact 대조한다. **PASS** |
| 12 | Chrome 버전·viewport·DPR 또는 캡처 PNG가 바뀌어도 같은 기대 RGBA를 재사용한다. | comparator는 Chrome `155.0.8059.40`, `400×240`, DPR 1, PNG 크기/해시 및 실제 다섯 PNG pixel을 검사한다. **PASS** |
| 13 | Android와 iOS가 표본 ID/순서/색상 정의를 따로 가져 기대값이 갈라진다. | 두 adapter의 fixed five-point table, Chrome observation, 비교기 expected sample 목록을 좌표/색상 단위로 대조했다. **PASS** |
| 14 | viewport가 달라졌는데 이전 surface 좌표로 overflow 색상을 우연히 통과한다. | Android draw report와 iOS request/report에서 `400×240`을 확인하고 실제 drawable 크기에서 scale을 계산한다. **PASS** |
| 15 | 빨간 pixel은 찾았지만 parent border 안쪽이라 visible escape를 증명하지 못한다. | sample center `(150.5,30.5)`를 parent right border `130`과 비교한다. run5 Android/iOS 모두 `escape_outside_parent=true`. **PASS** |
| 16 | Android screenshot을 PixelCopy 표면 데이터로 오인한다. | app screenshot(1080×2400)과 PixelCopy PNG(1050×630)를 별도 파일·해시·링크로 보존하고 comparator는 PixelCopy PNG를 다시 decode한다. **PASS** |
| 17 | iOS screenshot 자체를 Metal sample source라고 쓴다. | iOS 결과는 draw report의 Metal drawable samples를 source로 삼고 screenshot은 시각 확인용이라고 README에서 분리한다. **PASS** |
| 18 | 각 플랫폼의 sample이 최종 26-box matrix 이전의 초기 화면에서 왔다. | Android comparator는 마지막 `boxes=26` draw를 사용하고 summary revisions와 대조한다. iOS comparator는 최종 draw sample JSON 및 revisions를 비교한다. **PASS** |
| 19 | 같은 run ID라도 이전 process의 결과를 섞는다. | 모든 sample/begin/summary의 run ID와 PID를 검사하고 run5 로그의 Android PID 3081, iOS PID 57858을 기록했다. **PASS** |
| 20 | 같은 process 안에서 surface 재생성 뒤 예전 결과를 새 화면에 귀속한다. | surface generation을 begin·summary·각 Android/iOS sample에서 비교하고 positive run5에서 Android 2, iOS 1을 일치시켰다. **PASS** |
| 21 | layout/document/tree/style/environment/scroll revision이 달라진 sample을 같은 장면으로 설명한다. | 플랫폼 내부 draw-to-summary revision을 모두 비교하며 Android/iOS 공통 document/tree/style/scroll과 layout generation을 교차 비교한다. environment revision 차이 `0/1`은 의도된 platform-local 값으로 명시했다. **PASS** |
| 22 | screenshot이 존재해도 어떤 run/device에서 얻었는지 추적할 수 없다. | run5 Android/iOS screen dimensions·SHA-256, Android APK SHA, device/API/OS와 해당 log/PixelCopy 링크를 README에 기록했다. **PASS** |
| 23 | PR 본문 첫 이미지가 Chrome oracle인데 앱 실행 화면인 것처럼 보인다. | #171은 Chrome oracle 이미지가 먼저 나오고 이전 #173 앱 화면은 본문 아래라 혼동할 수 있었다. PR #176은 Android/iOS run5 앱 캡처를 `동작 화면` 첫 블록에 배치했다. agent-browser 화면에서도 첫 이미지가 앱 화면임을 확인했다. **PASS** |
| 24 | PR 이미지 URL이 존재하지 않거나 다른 파일을 가리킨다. | PR #176 raw URL 두 개의 commit/path가 run5 파일과 일치한다. agent-browser DOM에서 모두 `complete=true`, Android `1080×2400`, iOS `1206×2622`, `visible=true`를 확인했다. **PASS** |
| 25 | run4 screenshot과 run5 log를 섞어 이미지가 다른 화면을 보여준다. | README, comparator, log, PNG를 run5로 통일했다. run5는 최신 renderer 수정이 포함된 APK/app에서 다시 실행했고 두 화면을 직접 열어 overflow square와 5/5 status를 확인했다. **PASS** |
| 26 | comparator가 우연히 success marker 한 줄만 찾아 오류·중복 로그를 통과시킨다. | begin/summary/assertion marker는 각각 한 줄이어야 하고 fixed sample count/order, final draw identity와 exact RGBA를 요구한다. **PASS** |
| 27 | positive comparison만 있어 좌표·색상·파일 drift를 걸러내지 못한다. | 6개 집중 test에서 positive match 외에 색상, parent 안 좌표, 누락/중복, stale run/generation, PNG 크기/hash, Chrome fixture/screenshot 변조를 거부했다. **PASS** |
| 28 | Android의 invalid run ID negative control이 UI host/PixelCopy를 실제로 막는지 알 수 없다. | `android-invalid-run-id-negative-control.log`는 route validation의 `invalid-run-id` 거부를 기록한다. 신규 run5의 positive sample과 별도 파일로 보존했다. **PASS** |
| 29 | stale draw/copy 실패 때 표본이 남는 renderer 결함이 성공 증거 뒤에도 숨는다. | review에서 발견한 제출 전 stale 요청 잔류를 정리했고 focused Rust 회귀 검사, workspace Clippy, opt-in Clippy와 run5 양 플랫폼 실행을 다시 확인했다. **PASS — 수정 후** |
| 30 | 원래 dirty checkout의 사용자 변경이 PR diff에 섞이거나 문서·format 상태가 검증되지 않는다. | 원본 dirty checkout 대신 `origin/main`에서 분리한 worktree를 사용했다. 새 변경 목록은 C13.2 구현/evidence/doc으로 한정하고 `cargo fmt --check`, `git diff --check`를 확인했다. **PASS** |

전체 `test:css-reference` 442/442는 run5 comparator 기준으로 통과했고, PR #176은 `MERGEABLE`/`CLEAN`이다. `gh pr checks`에는 보고된 check가 없었으며, main branch protection은 required status check와 approving review를 요구하지 않는다. rebase merge 직전에도 PR 상태와 protection을 다시 조회한다.
