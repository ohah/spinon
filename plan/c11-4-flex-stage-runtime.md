# C11.4 · flexible track expansion 앱 runtime slice

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [§11.7 pure-Rust stage oracle](./c11-4-flex-stage-oracle.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**계획 검토:** [실패 관점 20개](./c11-4-flex-stage-runtime-plan-review-20.md) · [계획 PR diff 30관점](./c11-4-flex-stage-runtime-plan-pr-diff-review-30.md)

## 목표와 경계

이미 병합된 [§11.7 stage oracle](./c11-4-flex-stage-oracle.md)의 네 고정 입력을 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in 앱 fixture 경로에서도 실행한다. Rust-only `TaffyLayoutEngine` 결과를 앱의 실제 HostDocument → computed style → layout trace → render/present 경로와 연결하고, Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 `ExpandFlexibleTracks` stage와 root frame을 비교한다.

이 작업은 네 고정 Grid scene의 fixture 경로와 플랫폼 관측만 추가한다. CSS 계산 알고리즘, 일반 author CSS admission, 공개 지원 범위, span 기여, indefinite free space, auto-repeat 조합이나 WPT 전체 실행은 바꾸지 않는다. API·내부 계약 버전은 올리지 않는다. C11.4 checkbox와 공식 완료 `72/213`은 미완료로 유지한다.

## 고정 입력과 독립 기대값

scene ID, axis, 200px definite sizing axis, 120px opposite axis, 10px target-axis gap, CSS track descriptor와 기대 base 값은 [기존 expected JSON](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json)의 네 case와 일치해야 한다.

| Scene ID | 대상 축 | Track CSS | 기대 track base | 남은 공간 |
| --- | --- | --- | --- | ---: |
| `weighted-columns` | Columns | `minmax(0px, 1fr) minmax(0px, 2fr)` | `[190/3,380/3]px` | 0px |
| `frozen-row` | Rows | `minmax(80px, 1fr) minmax(0px, 2fr)` | `[80,110]px` | 0px |
| `subunit-columns` | Columns | `minmax(0px, 0.5fr) 20px` | `[85,20]px` | 85px |
| `zero-fr-row` | Rows | `minmax(24px, 0fr) minmax(0px, 1fr)` | `[24,166]px` | 0px |

`expected.json`의 CSS Grid 2025 CRD §11.7.1 유리수 계산을 규범 기대값으로 사용한다. 고정 Agent Browser 0.31.1 / Chrome 156 관측은 별도의 최종 geometry control이다: `[63.328125,126.671875]`, `[80,110]`, `[85,20]`, `[24,166]` CSS px. Chrome의 1/64px 직렬화 값으로 exact rational 기대값을 대체하거나 browser 출력에서 내부 stage를 추론하지 않는다. 기존 source fixture와 hash를 입력 기준으로 삼고, 독립적으로 발견한 불일치가 계획 수정을 요구하지 않는 한 재생성하지 않는다.

## 구현 범위와 matcher 계약

1. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/` 아래에 fixture 전용 scene builder를 추가하고 기존 C11.4 runtime-source dispatcher에 연결한다. 위 네 ID만 제공하고, root의 sizing 축은 definite 200px, 반대 축은 120px로 둔다. root는 `box-sizing:border-box`, `margin/padding/border:0`, `align-content:start`, `justify-content:start`를 사용한다. target 축의 예상 CSS track 두 개와 `10px` gap을 선언하고 반대 축은 고정 track 하나로 둔다. Chrome 원본처럼 각 track에 빈 자식 marker를 둔다. marker는 `min-width:0; min-height:0`이며 text/image/content·padding·border가 없어 intrinsic contribution을 추가하지 않고 지정한 grid cell을 채운다. Rust source test는 정확한 ID 집합과 CSS, root/node ID, marker 두 개, DOM 순서 및 unknown ID 거부를 확인한다.
2. Android/iOS fixture allowlist에 네 ID를 추가하고 플랫폼별 strict matcher를 둔다. 모르는 ID, 다른 layout profile, root/trace 누락, target-axis pass/stage 중복, 예상 밖 stage 순서, 누락·non-finite CSS track size, 잘못된 gap/topology 또는 예상 frame 누락은 모두 실패해야 한다. permissive한 `expected == null` fallback은 사용하지 않는다.
3. runtime trace에서 첫 pass를 가정하지 않고 case table의 target axis를 선택한다. target-axis `ExpandFlexibleTracks`가 정확히 한 번 나타나며 `MaximizeTracks` 뒤, `StretchAutoTracks` 앞인지 확인한다. vector 위치가 아니라 trace flag로 CSS track을 선택하고, 10px 내부 gutter 및 collapsed boundary sentinel과 구분한다. 독립 oracle이 뒷받침하는 축, track 개수·순서, finite 최종 CSS track base, gap, stage 순서만 비교한다. 내부 fr fraction이나 non-finite growth limit 의미를 추가로 주장하지 않는다.
4. `tools/css-reference/c11-4-flex-stage-runtime/verify.py`에서 Android/iOS 원본 report log를 [expected JSON](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json)과 대조한다. 이 verifier는 네 ID의 정확한 집합과 유일성, platform별 4개 report, checked 정수 numerator/양수 denominator, runtime `f32`의 finite 여부와 `1e-4 CSS px` 오차를 확인한다. platform matcher의 기대값 표도 expected JSON과 대조해 상수 drift를 막는다. Chrome root-relative 최종 geometry는 별도 `1/64 CSS px` control로 둔다. `subunit-columns`가 남기는 free space 85px 외에는 track 크기 합과 단일 gap이 200px이어야 한다. Verifier negative controls는 누락·중복 scene, 오염된 JSON, axis/stage 누락, non-finite track, wrong frame을 거부해야 한다.
5. 두 simulator에서 네 scene을 모두 실행한다. 실행 전 boot 상태를 기록하고 다른 device를 boot/shutdown하지 않는다. source/input 및 앱 artifact hash, simulator/OS 식별자, scene run ID, 정확한 trace/frame과 draw/present 로그, screenshot과 hash를 보존한다. screenshot은 layout 근거이며 별도 exact readback 없이는 pixel equality를 주장하지 않는다. 매 실행 후 fixture app을 종료하고, Agent Browser를 시작한 경우 해당 프로세스도 종료한다.
6. `c04-runtime-gpu-fixture-test`를 끈 기본 release `spinon-ffi` archive와 대응 opt-in positive control을 검사한다. 신규 scene marker와 fixture ABI는 기본 archive에서 빠지고 opt-in archive에는 들어가야 한다. 이 결과를 APK/IPA 전체 exclusion 증거로 확대하지 않는다.

### root-relative frame 기대값

각 scene의 root frame은 표의 `root`와 같고 marker `a`·`b`는 정해진 grid cell을 채워야 한다. 유한 좌표·크기는 `1e-4 CSS px` 이내로 비교하며, 첫 열 flex case의 Chrome DOMRect는 별도 `1/64 CSS px` 허용치를 적용한다.

| Scene | Root `(x,y,w,h)` | Marker `a` `(x,y,w,h)` | Marker `b` `(x,y,w,h)` |
| --- | --- | --- | --- |
| `weighted-columns` | `(0,0,200,120)` | `(0,0,190/3,120)` | `(220/3,0,380/3,120)` |
| `frozen-row` | `(0,0,120,200)` | `(0,0,120,80)` | `(0,90,120,110)` |
| `subunit-columns` | `(0,0,200,120)` | `(0,0,85,120)` | `(95,0,20,120)` |
| `zero-fr-row` | `(0,0,120,200)` | `(0,0,120,24)` | `(0,34,120,166)` |

## 합격 기준과 검증

- 구현은 이 계획 PR이 Rebase and merge된 뒤 시작한다.
- Focused FFI source test는 정확한 네 scene, author CSS와 거부 경로를 확인한다. verifier의 정상 report와 누락·오염·중복 negative controls를 모두 실행한다. 각 화면의 root frame은 `(0,0,200,120)` 또는 `(0,0,120,200)`이며, 두 marker의 root-relative frame은 expected track size와 gap에서 계산한 값이어야 한다.
- 기존 pure-Rust §11.7 oracle과 expected JSON은 바뀌지 않는다. 기존 C11.4 fixed, span, row-axis 및 direct text/image fixture regression도 계속 통과해야 한다.
- `cargo fmt --all -- --check`, 관련 fixture-feature Rust test, targeted Clippy와 `git diff --check`가 통과해야 한다. Android/iOS debug simulator build 뒤 각 target에서 최종 source로 네 scene을 모두 실행한다.
- archive 기본/positive-control 경계를 원본 명령 로그와 hash로 기록한다. static library 결과를 전체 app bundle 또는 release exclusion으로 확대하지 않는다.
- 구현 뒤 새로운 실패 관점 20개의 implementation/runtime 적대 검토와 별도 30관점 PR diff 검토를 완료하고, 수정 후 영향받은 관점을 다시 확인한다. PR 본문에는 commit된 Android/iOS screenshot의 안정된 링크를 각각 하나씩 포함한다. 전체 WPT 또는 C11.4를 완료했다고 쓰지 않고 남은 제한을 밝힌다.

## 제외 사항

이 slice는 intrinsic contribution이 양수인 Grid item, span, automatic minimum sizing, indefinite free space, percentage/calc gap, repeat, 일반 CSS parsing/admission, 실기기, Android hardware GPU, iOS compositor 제출 뒤의 pixel 또는 C11.4 전체 algorithm을 검증하지 않는다. `spec/STATUS.md`에서 C11.4를 완료 처리하거나 공식 `72/213`을 변경하지 않는다.

## 검증 결과 · 2026-10-28

Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene을 각각 4/4 통과했다. `weighted-columns`, `frozen-row`, `subunit-columns`, `zero-fr-row` 모두 target `ExpandFlexibleTracks` base, 10px gap topology, root-relative node frame, presented marker를 확인했다. [최종 로그와 screenshot](../spec/internal/evidence/c11-4-flex-stage-runtime-2026-10-28/README.md), [독립 verifier 출력](../spec/internal/evidence/c11-4-flex-stage-runtime-2026-10-28/verification.json), [FFI static archive 결과](../spec/internal/evidence/c11-4-flex-stage-runtime-2026-10-28/artifact-boundary/artifact-boundary.json)를 연결했다. FFI fixture test는 147 passed / 0 failed / 2 ignored였고 패키지 자체 Clippy와 rustfmt도 통과했다. 의존성까지 포함한 Clippy 실행은 변경하지 않은 `spinon-runtime`의 기존 `type_complexity` warning에서 중단됐으며, 실행 범위와 미지원 항목은 evidence README에 기록했다. C11.4 checkbox와 공식 `72/213`은 유지한다.
