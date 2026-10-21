# C11.4 · depth-three nested Grid text multi-span contribution 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [depth-three nested Grid 계획](./c11-4-nested-grid-depth-three.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 고정 Chrome 156 사전 관측과 독립 CSS Grid 기대값을 기록했다. 계획 실패 관점 20개 검토를 마쳤으며, 구현은 시작하지 않았다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다.

## 목적과 미구현 경계

기존 direct-leaf text multi-span fixture는 단일 Grid에서 겹치는 span-2와 span-2/3 item 기여를 검사한다. 별도 depth-three fixture는 한 direct text leaf의 min/max-content 기여를 세 Grid 경계로 전달한다. 기존 depth-three anonymous text + image fixture는 inner Grid의 두 자동 배치 item을 결합하지만 multi-span track 분배를 하지 않는다. 따라서 inner Grid의 겹치는 multi-span text contribution이 계산된 뒤 그 intrinsic width가 root → middle → branch의 세 parent Grid 경계를 통과하는 고정 실행은 아직 없다.

이번 slice는 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture-test 경로에 네 개의 고정 scene을 추가한다. topology는 `root → middle → branch → inner` 네 HTML `div` Grid와 inner의 direct-leaf text item 둘 또는 셋이다. 일반 CSS cascade admission, 공개 API, 임의 nested Grid, anonymous text, image, WPT, 전체 C11.4 지원을 추가하지 않는다.

## 입력과 기대값

HTML 재현 입력과 Chrome 관측은 [precomparison evidence](../spec/internal/evidence/c11-4-nested-grid-multispan-precomparison-2026-10-22/README.md)에 고정했다. Ahem은 10px/10px, box edge·gutter는 0px다. 각 outer Grid는 `width:min-content`, `height:60px`, 단일 min-content column과 60px row다. inner Grid는 같은 intrinsic width·고정 height에 네 min-content column과 세 20px row를 사용한다. leaf item placement와 문자열은 exact allowlist로 검사한다.

| scene | inner item 순서 | 독립 inner Columns | 각 Grid width |
| --- | --- | --- | --- |
| `nested-text-span2-forward` | span-2 A → span-2 B | `[25,35,35,0]` | `95×60px` |
| `nested-text-span2-reversed` | span-2 B → span-2 A | `[25,35,35,0]` | `95×60px` |
| `nested-text-span2-before-span3` | span-2 A → span-2 B → span-3 C | `[160/3,190/3,190/3,0]` | `180×60px` |
| `nested-text-span3-before-span2` | span-3 C → span-2 B → span-2 A | `[160/3,190/3,190/3,0]` | `180×60px` |

CSS Grid 2025 CRD §11.5.1에 따라 A와 B의 min-content 50px·70px span-2 contribution 뒤에 C의 180px span-3 contribution을 분배한다. 이상적인 분수 track과 Chrome의 1/64 CSS px quantization은 각각 다른 비교 입력이다. Chrome 관측, 독립 기대값, 모든 fixed node frame은 사전 비교 파일에 별도 저장되어 있다.

## 구현 및 검증 계획

1. 계획 PR을 main에 Rebase and merge한 뒤 같은 checkout에서 최신 `origin/main`을 기준으로 구현 branch를 만든다. worktree는 만들지 않는다.
2. 현재 nested depth-three text adapter와 direct text multi-span adapter를 읽고, exact topology·node owner/order·text/placement·Ahem style·scene ID를 검사하는 새 fixture adapter를 opt-in fixture 경계에 추가한다. 다른 depth, 추가/누락 child, 잘못된 text·span·track·style은 부분 layout 없이 거부한다.
3. 기존 intrinsic input/stage trace에서 각 text source의 min/max measurement, 실제 처리 span, stage 순서와 네 Grid의 `Columns`/`Rows`, root-relative node frame, source revision을 함께 검사한다. inner contribution source는 leaf text node, parent contribution source는 nested Grid node여야 하며 중복 귀속이 없어야 한다.
4. 네 신규 scene과 기존 nested depth-one·two·three text control 및 direct text multi-span 네 scene을 focused Rust/runtime/FFI matrix에서 실행한다. 기본 product artifact에는 새 scene ID와 고정 text payload가 빠지고 opt-in fixture-test artifact에는 들어가는지 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 네 신규 scene을 실행한다. trace completeness, frames/tracks, draw/present marker, local/installed app hash를 보존한다. 이는 Android 실기기, hardware GPU, iOS compositor 이후 화면 또는 glyph pixel equality 증거가 아니다.
6. 관련 focused Rust·FFI fixture 검사, `cargo fmt --all -- --check`, workspace all-target Clippy `-D warnings`, Android/iOS Simulator fixture build를 실행한다. 범위 밖 전체 WPT/일반 CSS 행렬은 실행했다고 쓰지 않는다.
7. 구현/runtime을 계획 검토와 겹치지 않는 새 실패 관점 20개로 검토한다. PR diff는 별도 30개 관점으로 검토한다. PR 본문에는 두 Simulator의 실행 화면을 링크하고 Agent Browser에서 이미지가 로드되는지 확인한 뒤 Agent Browser 프로세스를 종료한다.
8. 현재 PR required check와 merge state를 확인하고 Rebase and merge한다. 병합 후 Tailnet 미리보기는 최신 main의 문서와 근거를 선택 build·백업·hash 대조하고 Agent Browser로 확인한다. 브라우저와 이번 작업에서 시작한 서버를 종료한다.

## 실패·범위 경계

- CSS Grid 알고리즘의 독립 기대값과 Chrome 관측이 다르면 어느 한쪽에 맞춰 고치지 않고 mismatch를 기록하고 명세 계산을 재검토한다.
- Ahem 미로딩, 잘못된 text/owner/order/span, 무제한 grid child, 중간 trace 누락, stale revision, 일부 scene만 통과, 잘못된 draw marker, fixture symbol의 기본 artifact 유입은 완료 거부 조건이다.
- 성공은 이 fixed input의 중첩 contribution 전달에만 적용한다. 일반 font/script, anonymous item, image 혼합, nested span/outer item span, subgrid, arbitrary CSS Grid, WPT 및 C11.4 전체 완료를 의미하지 않는다.
