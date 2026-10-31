# PR #916 실제 변경 실패 관점 검토 30개

- PR: https://github.com/ohah/spinon/pull/916
- 검토 base: `332aa20d32cbf42709bc2cbe72cf91f9f76cee32`
- 검토 head: `93f65605a7371d284f23b0ebc695a8084bb2cf8e`
- 범위: 실제 코드 diff, 모바일 fixture와 캡처, 상태·계획 문서, source-bound AVD/Simulator evidence.
- 결과: 아래 30개 관점에서 병합 차단 결함을 찾지 못했다. 발견한 값은 고정 fixture 계약 및 결과 산출물과 일치한다.

| # | 독립 실패 경로 또는 불변 조건 | 검토 결과 |
| ---: | --- | --- |
| 1 | template area가 만든 두 번째 track을 positive implicit track으로 잘못 세는 경우 | PASS — `TemplateGrid`, explicit 2 / positive implicit 0을 런타임 trace에서 요구한다. `runtime_layout/fixtures/c11_3_3_fit_content_area_cap.rs:195-207,239-243`. |
| 2 | area-created track의 크기에 `grid-auto-columns`가 적용되지 않는 경우 | PASS — typed auto-columns를 정확히 `fit-content(40px)`로 검사한다. 같은 파일 `:171-175`. |
| 3 | authored 첫 track이 20px에서 달라져 cap 사례가 바뀌는 경우 | PASS — 입력 columns `[20px]`, output columns `[20px, fit-content(40px)]`와 Chrome used list `20px 40px`를 확인한다. 같은 파일 `:196-200,239-243`. |
| 4 | min-content가 cap보다 작거나 max-content가 cap보다 작아 upper cap 효과를 입증하지 못하는 경우 | PASS — pinned Ahem 기여가 20px/110px인지 검증하므로 `20 < 40 < 110`을 고정한다. 같은 파일 `:97-130`. |
| 5 | 시스템 fallback font가 Ahem 측정값에 섞이는 경우 | PASS — system fonts 없는 engine에 저장소 Ahem bytes를 등록하고 exact min/max가 아니면 fail-closed한다. 같은 파일 `:23,97-104,124-130`. |
| 6 | 공백·텍스트 node가 사라지거나 다른 content로 intrinsic 입력이 대체되는 경우 | PASS — direct text node 하나, exact `AA AA AA AA`, 원본 text node ID를 확인한다. 같은 파일 `:70-94,146-152`. |
| 7 | 다른 element, root 위치 또는 profile이 fixture처럼 통과하는 경우 | PASS — exact root ID, `div`, document root parent, `RuntimeBlockFlexGridTemplateV1`을 검사한다. 같은 파일 `:30-40`. |
| 8 | fixed 또는 fit item의 추가 child/nested text가 paragraph owner를 바꾸는 경우 | PASS — root child 수·ID·순서와 fixed empty / fit 단일 direct text 불변 조건을 검증한다. 같은 파일 `:42-80`. |
| 9 | unrelated node style이 추가되어 입력 tree가 달라지는 경우 | PASS — computed style node set이 root와 두 element에 정확히 대응해야 한다. 같은 파일 `:133-143`. |
| 10 | grid area가 2열 직사각형으로 구성되지 않거나 이름이 서로 바뀌는 경우 | PASS — `fixed`와 `fit` area의 각 row/column 경계를 별도로 확인한다. 같은 파일 `:176-194`. |
| 11 | row 또는 used frame이 oracle과 달라 열 cap만 우연히 통과하는 경우 | PASS — row 60px, root/item/fixed frame 세 개를 각각 검증한다. 같은 파일 `:201-207,274-294`. |
| 12 | track trace가 다른 Grid에서 오거나 한 축 단계만 실행되는 경우 | PASS — trace 1개, root node identity와 Columns·Rows pass 둘 다 요구한다. 같은 파일 `:233-257`. |
| 13 | layout intrinsic source가 다른 node 또는 다른 min/max 값에 귀속되는 경우 | PASS — text item, owning Grid, source kind와 입력 시 측정한 크기를 대조한다. 같은 파일 `:259-289`. |
| 14 | completed runtime diagnostics가 layout 결과와 분리되거나 text source를 잃는 경우 | PASS — completed trace, item, paragraph owner, 단일 source node와 원문을 다시 확인한다. 같은 파일 `:300-341`. |
| 15 | 다른 public author CSS가 C11.3.3 Grid 경로에서 새로 허용되는 경우 | PASS — projection 예외는 fixture feature, template profile, 고정 root ID 및 한 direct owner 아래에서만 track trace를 켠다. `spinon-style-to-layout/src/projection.rs:1433-1444`; 일반 admission 코드는 이 diff에 없다. |
| 16 | fixture feature가 꺼진 제품 빌드에도 text skip 또는 scene source가 들어가는 경우 | PASS — FFI fixture module과 renderer text 예외는 fixture-test feature 경계에 있고 기본/opt-in archive 대조 결과가 각각 marker 0/3이다. `spinon-runtime/Cargo.toml:17-24`, [archive evidence](artifact-boundary/README.md). |
| 17 | 새 FFI scene route가 접두어나 suffix가 붙은 임의 입력을 허용하는 경우 | PASS — exact scene ID만 source를 반환하며 suffix 주입 negative unit test가 있다. `spinon-ffi/src/runtime_gpu/fixtures/c11_3_3.rs:210-212,392-408`. |
| 18 | 새 fixed JavaScript source가 기존 39-scene list/expected node order를 변경하는 경우 | PASS — 새 source는 별도 파일이고 기존 scene 목록에서 분리되어 있다. `spinon-ffi/src/runtime_gpu/fixtures/c11_3_3.rs:4-5,207-212`; 기존 matrix는 source-bound 39/126으로 재통과했다. |
| 19 | Android intent route가 fit-cap scene 또는 다른 scene을 잘못 받는 경우 | PASS — Android route의 named allowlist에 exact `fit-content-area-cap`만 추가했다. `MainActivity.java:130-136`, `C0410RuntimeGpuDemo.java:1392-1397`. |
| 20 | iOS launch argument route가 Android와 달리 scene을 누락하거나 임의 값을 받는 경우 | PASS — iOS allowlist도 exact scene ID를 추가하고 기존 목록을 보존한다. `C0410RuntimeGpuDemo.swift:28-30`. |
| 21 | Android PixelCopy 표본 이름·node·좌표가 fixture와 어긋나는 경우 | PASS — 세 표본은 fixed `(10,30)`, cap 내부 `(59,30)`, cap 직후 `(60,30)`에 고정되며 allowlist가 전체 set을 검사한다. `C0410RuntimeGpuDemo.java:15038-15055,15189-15200`. |
| 22 | iOS Metal 표본이 잘못된 node·좌표·색상을 가리키는 경우 | PASS — iOS allowlist가 같은 세 node/좌표와 Chrome RGBA를 사용한다. `C0410RuntimeGpuDemo.swift:10883-10893`; 3/3 drawable 비교가 통과했다. |
| 23 | Android density 변화로 CSS pixel 표본이 Surface pixel의 다른 지점을 읽는 경우 | PASS — capture result는 density/scale과 PixelCopy image hash를 기록하고 fixture CSS coordinates를 기준으로 비교한다. Android source-bound `result.json` 및 `runtime.log`. |
| 24 | layout summary만 보고 Android draw/readback 전 실행을 성공 판정하는 경우 | PASS — capture는 terminal PixelCopy summary를 기다리고 `presented boxes`와 세 geometry를 확인한 뒤 결과를 쓴다. `capture-c11-3-3-grid-template-areas-runtime.mjs:68-104,207-231`. |
| 25 | iOS layout summary만으로 Metal 표본이 준비되었다고 판정하는 경우 | PASS — iOS capture는 terminal Metal summary를 기다린 뒤 geometry와 drawable sample을 비교한다. 같은 capture file `:307-341`. |
| 26 | 변조된 표본 좌표가 실제 surface readback을 통과하는 경우 | PASS — Android PixelCopy 및 iOS Metal negative control은 invalid coordinate를 readback 전에 거부했다. source-bound scene `result.json`의 `negative` 결과. |
| 27 | 미등록 scene이 runtime draw/readback까지 실행되는 경우 | PASS — Android scene negative control은 runtime draw 전에 거부했고 iOS/기존 matrix rejection도 기록됐다. source-bound Android scene result 및 matrix `result.json`. |
| 28 | fit-cap scene 결과를 기존 C11.3.3 회귀 결과와 섞거나 source revision을 잘못 표시하는 경우 | PASS — 별도 run ID/evidence directory와 source tree 기록을 사용하며 두 플랫폼 39-scene matrix 결과도 commit `b815f483a253072cbbca1c0cd473d519fc95a220`에 결속된다. |
| 29 | simulator/drawable/archive 한정 결과를 hardware, compositor-after 또는 WPT 전체 증거로 확대하는 경우 | PASS — 계획·상태·evidence는 SwiftShader AVD, compositor 이전 Metal drawable, macOS staticlib 한계와 미검증 범위를 명시한다. |
| 30 | 부분 fixture 결과로 C11.3.3/C11 또는 공식 완료 수를 잘못 승격하는 경우 | PASS — `spec/STATUS.md`에서 checkbox를 유지하고 공식 수 `72/213`을 그대로 둔다. PR 본문의 Android/iOS 이미지도 Agent Browser에서 각각 완전히 로드됨을 확인했다. |

## 검증 참고

- 별도 구현/runtime 리뷰: [20개 관점](implementation-runtime-review-20.md).
- PR body images: Android AVD 1080×2400, iPhone 17 Pro Simulator 1206×2622. GitHub PR page에서 두 raw image 모두 `complete=true`, natural dimensions가 0보다 큰 것을 확인했다.
- PixelCopy 및 Metal 표본은 surface 결과다. screenshot은 UI 확인 자료이며 iOS compositor 이후 픽셀이나 전체 Grid 지원 근거가 아니다.
