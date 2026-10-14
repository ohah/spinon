# C13.2 visible overflow 표면 픽셀 계획 PR 변경 검토 30개

- **대상:** C13.2 표면 픽셀 하위 계획, 고정 Chrome probe fixture, observation JSON와 캡처 evidence.
- **독립성:** 계획 자체의 20개 실패 관점 검토와 별도로 PR diff가 repository 경계·근거·게시 동작에서 깨지는 경로를 검토했다.
- **근거:** `plan/c13-2-visible-overflow-surface-pixels.md`, `tests/fixtures/css/c13/c13-2-visible-border-pixel-probe.html`, Chrome 155 agent-browser screenshot/DOM geometry/PNG byte sampling, source and artifact hashes.

| # | PR 변경이 깨뜨릴 수 있는 경로 | 확인 및 판정 |
|---|---|---|
| 1 | 새 하위 계획이 parent C13.2 target을 대체하거나 전체 기능 완료로 오해된다. | 새 문서는 surface evidence follow-up으로 범위를 한정하고 C13.2/C13 미완료를 유지한다. **PASS** |
| 2 | 계획에 없는 runtime 구현을 PR이 함께 끼워 넣는다. | diff는 plan, HTML probe, observation/evidence, 두 review 문서뿐이다. **PASS** |
| 3 | internal contract의 `0.1.0-draft`를 구현 준비만으로 올린다. | version은 유지하고 공개 API 추가를 배제했다. **PASS** |
| 4 | plan link가 잘못된 sibling 경로를 가리킨다. | parent plan과 새 subplan은 같은 `plan/` 디렉터리에 있어 `./c13-2-visible-overflow-surface-pixels.md`로 resolve된다. **PASS** |
| 5 | fixture 이름이 C13.1/다른 작업의 기존 oracle과 충돌한다. | `c13-2-visible-border-pixel-probe.html`은 C13.2 전용 이름이며 기존 파일과 별도다. **PASS** |
| 6 | HTML fixture가 앱 코드나 외부 네트워크 script를 실행한다. | 같은 repository의 `runtime-scroll-container-c13-2.js`만 file-relative로 로드하고 network dependency가 없다. **PASS** |
| 7 | oracle은 source fixture를 참조하지만 실제 실행에서 로드되지 않았다. | agent-browser `file://` run에서 `c132-root`, parent와 child node가 실제 DOM에 생성됐다. **PASS** |
| 8 | HTML root 위치가 runtime viewport 좌표와 다르다. | Chrome DOM geometry는 root `(0,0) 380×180`, viewport `400×240`이다. **PASS** |
| 9 | target child를 잘못 선택하거나 matrix의 다른 red marker를 샘플한다. | `(150,30)` hit target이 `c132-visible-escape`; same source JS가 stable ID를 생성한다. **PASS** |
| 10 | screenshot path/PNG dimension/hash가 observation JSON과 다르다. | PNG file은 `400×240`, SHA-256 `9a0be14f…c81b6`이며 manifest와 일치한다. **PASS** |
| 11 | browser build가 Chrome 155라도 정확한 patch release가 달라진다. | userAgentData full version `155.0.8059.40`을 캡처했고 plan의 고정 revision과 대조했다. **PASS** |
| 12 | viewport/DPR source is implicit and future run applies default size. | observation은 `400×240`, DPR 1을 기록하며 agent-browser viewport를 명시했다. **PASS** |
| 13 | screenshot sample color comes from CSS declaration rather than rendered PNG. | expected RGBA를 repository PNG decoder로 각 PNG byte에서 직접 읽었다. **PASS** |
| 14 | encoded RGBA order is mistaken for native BGRA order. | Chrome PNG observation records RGBA; plan requires platform result to normalize to RGBA before comparing. **PASS** |
| 15 | red target coordinate is inside parent due border-box/client-box confusion. | parent border right `x=130`, target left `143`, sample cell center `150.5`; gap separately at 136.5. **PASS** |
| 16 | edge antialiasing makes exact pixel comparison unstable. | all five fixed sample centers are inside solid fills, not on antialiased boundaries. **PASS** |
| 17 | only escaped red is checked and a wrong viewport offset samples another coincident red box. | parent interior, gold border strip, dark gap and root background provide distinct colors. **PASS** |
| 18 | Android pixels are derived from emulator density instead of actual copied image size. | implementation plan explicitly maps viewport from PixelCopy image width/height and stores dimensions/hash. **PASS** |
| 19 | Android PixelCopy reads window UI instead of WGPU SurfaceView. | plan names SurfaceView PixelCopy and requires the copied PNG bytes and PixelCopy generation evidence. **PASS** |
| 20 | Android app evaluates another scene while PixelCopy is pending. | final matrix draw and captured surface generation/revisions are correlated; a mismatched run cannot pass. **PASS** |
| 21 | iOS checks app screenshot rather than renderer output. | plan requires Metal drawable RGBA via current C13.2 host and labels it pre-compositor. **PASS** |
| 22 | iOS queued samples are read from a different evaluation. | no DOM re-evaluation is permitted between request and following draw; draw revisions must match final matrix. **PASS** |
| 23 | platform fixture FFI is accidentally exported in default product build. | C13.2 feature/Android/iOS build flags and default/opt-in archive scan are explicit implementation gates. **PASS** |
| 24 | Metal surface readback result is promoted to final display/scanout parity. | scope excludes compositor/display scanout and real hardware. **PASS** |
| 25 | PixelCopy/Metal summaries count partial samples as success. | pass requires all five fixed IDs, exact sample count, dimensions, RGBA and hashes. **PASS** |
| 26 | comparator passes if sample ID is absent, duplicated, or a stale run. | isolated mutation gates require absent/duplicate IDs and run/generation drift to reject. **PASS** |
| 27 | comparator accepts the red color at an in-border coordinate. | a coordinate mutation into parent is explicitly a negative control; border containment is rechecked. **PASS** |
| 28 | expected mismatch is made to fail by another unrelated assertion. | expected-channel mutation must report the exact escaped sample mismatch before summary failure. **PASS** |
| 29 | docs site introduces new local binary links while existing build has dead-link findings. | plan README names evidence files inline; it adds no Markdown link to PNG/log/JSON. **PASS** |
| 30 | PR review accepts broken screenshot embed or treats Chrome oracle as app screenshot. | PR body must label and embed the Chrome reference as oracle, state it is not a simulator result, then verify rendered image after PR creation. **PENDING POST-PUBLISH** |

## 통합 경계

이 PR은 계획과 Chrome oracle만 게시한다. Android/iOS app surface implementation, PixelCopy/Metal runtime, simulator result, feature archive scan 및 C13.2 완료 판정은 뒤따르는 implementation PR에서 새 review와 근거를 요구한다.
