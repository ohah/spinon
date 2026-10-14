# C13 · overflow·스크롤·클리핑

**상태:** 계획 초안 · C13.1 제한 profile 구현 중 · C13 미완료
**공식 상위 항목:** [구현 상태 대장](../spec/STATUS.md#css-구현-체크리스트)
**전제:** C04 CSS cascade, C07 상자 geometry, C08/C12 paint 입력, C12.1–C12.4의 source/position/paint order
**후속:** C12.5 sticky는 C13 scrollport·scroll offset·clip 기초 뒤 진행한다. C17 logical axis, C22 border radius, C23 transform/clip, C29 CSSOM, S05 입력·이벤트와의 교차 경계를 완료 게이트로 유지한다.
**비교 기준:** Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059`, macOS arm64. 구현 전 관찰은 [C13.1 사전 비교](../spec/internal/evidence/c13-overflow-precomparison-2026-10-14/README.md)에 고정했다.

## 목표와 완료 경계

C13은 `overflow-x/y`와 `overflow`의 `visible`, `clip`, `hidden`, `scroll`, `auto`, scroll container와 scrollport, 실제 clip geometry, 스크롤 상태·사용자 입력, 스크롤바와 gutter, overscroll chaining, scroll snap을 CSS 계산부터 최종 paint/hit-test까지 연결한다. 선언을 받아들이고 Taffy에 전달하는 것만으로 동작을 완료하지 않는다.

공식 상태표의 C13 항목은 계속 미완료다. 이 계획은 제품 API 완료를 주장하지 않는다. 모든 C13 단계의 구현 PR·20개 별도 구현 검토·실행 근거와 아래 교차 의존성이 닫히기 전에는 C13이나 C12를 완료로 표시하지 않는다.

## 규범 원본

| 범위 | 고정해 읽을 원본 | 이 계획에서 쓰는 경계 |
| --- | --- | --- |
| overflow 값, 축 정규화, clip edge, viewport 전파, scroll behavior, gutter | [CSS Overflow Module Level 3](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/) · 2025-10-07 WD | `clip`은 scroll container가 아니며 formatting context를 만들지 않는다. `hidden`은 programmatic scroll 가능한 scroll container다. mixed-axis computed value와 root/body 전파를 별도 검증한다. |
| element/window scroll API·scroll offset 조회 | [CSSOM View Module](https://www.w3.org/TR/2025/WD-cssom-view-1-20250916/) · 2025-09-16 WD | Element/Window scroll state와 `scroll()`, `scrollTo()`, `scrollBy()`, `scrollTop`/`scrollLeft`, `scrollIntoView()`의 DOM/API owner는 C29와 함께 정한다. |
| 경계 동작과 nested scroll chaining | [CSS Overscroll Behavior Module Level 1](https://www.w3.org/TR/css-overscroll-1/) · 2019-06-06 FPWD | `auto`, `contain`, `none`은 scroll container의 경계 동작이다. OS가 제공하지 않는 overscroll 효과를 있다고 가장하지 않는다. |
| snap area·snapport·axis·strictness·re-snap | [CSS Scroll Snap Module Level 1](https://www.w3.org/TR/2021/CR-css-scroll-snap-1-20210311/) · 2021-03-11 CR snapshot | 사용자/프로그램 scroll 종료 뒤 snap, oversized 대상의 접근 가능성, layout 변경 뒤 같은 대상 재-snap을 함께 검증한다. 정확한 물리 애니메이션은 spec이 UA 정의로 남긴다. |
| scrollbar color·width | [CSS Scrollbars Styling Module Level 1](https://www.w3.org/TR/2021/CR-css-scrollbars-1-20211209/) · 2021-12-09 CR snapshot | CSS `scrollbar-color`, `scrollbar-width`만 다룬다. OS/UA가 소유하는 scrollbar 내부 부품이나 `::-webkit-scrollbar`는 계약으로 노출하지 않는다. |

규범 원본은 초안/CR snapshot이며 상호 운용성 자체를 보증하지 않는다. 각 단계의 Chrome fixture와 Android/iOS 실행을 독립 비교 자료로 남기고, Chrome/WPT oracle 결과를 Spinon 적합성 통과로 바꾸어 보고하지 않는다.

## 계획 작성 시점의 코드 기준선

- `spinon-layout::LayoutStyle`에는 CSS overflow 값이 없다. `CompactTaffyStyle`은 Taffy `Overflow`와 `scrollbar_width`를 저장하지만 현재 계산에서 CSS overflow 의미의 source of truth로 쓰지 않는다. viewport용 내부 block style은 Taffy `hidden`을 사용한다.
- `RuntimeRenderSnapshot`은 box의 사각 frame·단색 paint·paint order만 보존한다. ancestor clip과 scroll offset 자료형은 없다. WGPU geometry builder는 각 사각형을 독립 quad로 만든다.
- S04.9 hit-test는 고정 fixture snapshot의 사각 box hit만 확인한다. CSS overflow clip과 제품 scroll input/event 경로는 아니다. S05 입력 전달 문서도 첫 click proposal이며 제품 adapter는 미구현이다.
- `RuntimeRenderKey`에는 문서·트리·style·environment·intrinsic size revision이 있지만 독립적인 scroll state revision은 없다.

이 관찰은 구현을 시작하기 전 기준이다. 구현 PR은 `origin/main` 최신 소스와 현재 고정 Taffy 버전을 다시 조사한 뒤, Taffy field나 예전 fixture 동작을 CSS 계약으로 오해하지 않는지 기록한다.

## 단계별 작업과 관문

각 단계는 독립 기능 PR 하나와 자체 fixture·버전 있는 내부/공개 계약·구현 실패 관점 20개를 가진다. 계획 실패 관점 20개는 [별도 계획 검토](../spec/internal/evidence/c13-overflow-scroll-clipping-plan-review-2026-10-14.md)에 둔다. 다음 단계의 코드를 앞 단계 PR에 섞지 않는다.

| 단계 | 포함 범위 | 완료 관문 |
| --- | --- | --- |
| C13.1 | `overflow`/`overflow-x`/`overflow-y`의 typed cascade, `visible`/`clip` 의미, mixed-axis computed value 정규화, `overflow-clip-margin`의 edge keyword와 적용 조건, ancestor clip chain의 layout→paint→hit-test 전달. 첫 profile은 현 지원 box와 `horizontal-tb`·LTR의 직사각 clip이다. `clip`이 formatting context를 만들지 않는 경계를 포함한다. | Chrome computed value·clip edge·box geometry 대조, clip 바깥 draw/hit negative controls, clip과 ink overflow의 독립 판정, 고정 Chrome hidden/clip/clip+flow-root float 관찰과 C13 float fail-closed 확인(Spinon float geometry parity는 C26 이후), clip-node allocation 실패와 동일 worker 복구. C22 border radius/C23 transform을 silent fallback하지 않는다. |
| C13.2 | `hidden`/`scroll`/`auto` scroll container, scrollable overflow area와 scrollport를 ink overflow와 구분하고, CSS px scroll offset, clamp/initial position, root viewport 및 HTML body overflow propagation과 실제 DOM scroll owner, style/layout 변경·node 이동/삭제 후 offset 보존/재설정을 다룬다. `ScrollStateRevision`을 document/style/environment revision과 분리해 immutable render snapshot 및 hit-test 입력에 포함한다. | Chrome scrollWidth/Height·clientWidth/Height·scrollTop/Left·root/body scroll owner oracle, no-overflow/one-axis/nested/out-of-flow/ink-only 경계, stale scroll revision 거부, 재계산 뒤 offset clamp와 원자 snapshot, 오류 후 복구. `clip`에는 offset을 만들지 않고 `hidden`은 programmatic scroll 입력을 받는다. |
| C13.3 | Android/iOS scroll gesture를 scrollport와 scroll chain에 연결하고, nested scroll target 선택·축별 남은 delta 및 대각 입력 전달·축 결정/재설정·gesture cancel/다중 pointer·새 surface generation을 처리한다. `overscroll-behavior-x/y`의 `auto`/`contain`/`none`은 scroll chaining 및 실제 플랫폼 boundary action을 차단하는 지점까지 연결한다. scroll event 전달은 S05 owner와 통합한다. | Android API 37 ARM64 AVD와 iOS Simulator에서 같은 fixture gesture trace, target scroll revision/offset/event 순서 비교. nested inner→outer chaining과 남은 delta, diagonal/axis-lock, contain/none, no-scroll-axis, cancel, stale frame/generation, 입력 이후 layout 변경 negative controls. 플랫폼 고유 bounce/glow가 없는 경우 해당 효과를 PASS로 꾸미지 않는다. |
| C13.4 | `scrollbar-gutter`, classic/overlay scrollbar의 layout 기여, 재계산, paint, thumb 이동, `scrollbar-width:auto/thin/none`, `scrollbar-color`와 root viewport 전파. `none`이어도 touch/keyboard/programmatic scroll 및 접근 가능한 대체 조작을 보존한다. | Chrome 및 각 Simulator의 scrollbar mode를 실행 환경으로 기록한다. scrollbar 유무 전후 content box·absolute child·background 위치·gutter geometry, stable/both-edges와 root/body 대조, overlay/classic geometry 분리, palette/width 및 키보드·접근성 대체 입력을 확인한다. OS가 숨긴 scrollbar와 측정 불가 geometry는 미측정으로 남긴다. |
| C13.5 | `scroll-snap-type`, `scroll-snap-align`, `scroll-snap-stop`, `scroll-padding`, `scroll-margin`, `scroll-behavior`; user/programmatic scroll 완료 뒤 snap 결정과 layout/style/tree 변화 뒤 re-snap. mandatory/proximity, axis, oversized snap target을 포함한다. | Chrome offset과 선택된 snap target을 끝 상태에서 비교하고, 입력 시간/애니메이션 표본은 별도 기록한다. oversized content를 중간부터 모두 접근할 수 있는지, nested container가 가장 가까운 snap owner를 고르는지, DOM update 후 target 유지/삭제, mandatory와 proximity를 각각 negative/control fixture로 확인한다. |
| C13.6 | C17의 writing-mode/direction 계약과 합쳐 `overflow-block/inline`, logical overscroll, snap logical axes 및 scroll origin/negative RTL offset을 통합한다. | C17 merge 뒤 horizontal-tb/LTR 외 vertical/RTL fixtures, DOM scroll APIs의 좌표 부호, platform gesture 방향·scrollbar inline edge와 Chrome 대조를 수행한다. 이 관문 전에는 C13 완료를 표시하지 않는다. |

### 현재 C13.1 브랜치 진행 상태 · 2026-10-14

- C13.1 제한 `RuntimeBlockFlexOverflowClipV1` profile과 별도 [계약 초안 0060](../spec/internal/0060-c13-1-overflow-clip.md)을 구현했다. 이는 제품 CSS 지원 선언이나 C13.1 완료가 아니다.
- 고정 Chrome 155 fixture에서 Android/iOS Simulator의 13개 node geometry가 모두 일치했다. agent-browser screenshot에서 사전 등록한 11개 solid-color paint sample을 Android API 37 ARM64 AVD SurfaceView PixelCopy와 exact 비교했고, 고정 hit-test query는 Android 앱 15/15, iOS Simulator 15/15와 viewport 밖 null control 4/4가 일치했다. iOS 결과는 Metal surface draw 및 화면 캡처를 기록하며 drawable readback은 포함하지 않는다. 원본 log·PNG·hit-test 좌표는 [AVD paint 근거](../spec/internal/evidence/c13-1-overflow-clip-android-avd-2026-10-14/README.md), [앱 hit-test 근거](../spec/internal/evidence/c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md), [viewport negative-control 근거](../spec/internal/evidence/c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/README.md), [iOS Simulator 근거](../spec/internal/evidence/c13-1-overflow-clip-ios-simulator-2026-10-14/README.md)에 있다. 별도 5-node border route는 Android API 37 ARM64 AVD에서 5/5 frame, 6/6 hit query, 6/6 PixelCopy와 기존 13-node route의 15/15 hit 및 11/11 PixelCopy 회귀를 통과했다([AVD 근거](../spec/internal/evidence/c13-1-overflow-clip-border-android-avd-2026-10-14/README.md)).
- 고정 Chrome 기준에서 `clip-y`, border-box `overflow-clip-margin`, hidden의 margin 무시 세 행을 Rust clip geometry와 전체 정수 hit range로 대조했다([선택 matrix 근거](../spec/internal/evidence/c13-1-overflow-clip-matrix-2026-10-14/README.md)). 17행 런타임 disposition은 후속 [runtime ledger](../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/README.md)에서 13개 유효행과 4개 reject/fail-closed로 정리했다. 유효행 전체 surface matrix와 clip 밖 draw/hit/event negative-control breadth는 남아 있다.
- 현재 17행 런타임 판정은 13개 유효행의 geometry·hit interval·topmost NodeId, invalid margin 1행의 cascade/layout 거부, float 3변형의 fail-closed로 확인했다. 13개 유효행의 exact fallback ID는 [hit identity 근거](../spec/internal/evidence/c13-1-overflow-clip-hit-identity-2026-10-14/README.md)에 기록했다. 전체 surface matrix와 clip 밖 draw·hit/event negative-control breadth는 남아 있다. 이번 확장 matrix는 앞선 세 선택 행에 Chrome 신규 10행을 더해 13개 target의 frame, 축별 정수 hit interval 및 target frame 안의 topmost NodeId를 Rust와 대조했고, 별도 content-box positioned hit control을 추가했다([근거](../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/README.md)). Android API 37 ARM64 AVD 앱 회귀는 기존 13-node route에서 hit query 15/15와 PixelCopy 11/11을 통과했지만 새 contained-absolute 행은 포함하지 않았다. C13.1 WPT 이름 범위의 58개 source는 고정 commit에서 13개 fixture candidate, 17개 Chrome oracle-only, 28개 unsupported로 분류했지만 WPT runner는 실행하지 않았다([분류 기록](../spec/internal/evidence/c13-1-overflow-clip-wpt-classification-2026-10-14/README.md)). 고정 Chrome float/BFC 관찰과 C13 runtime fail-closed는 [float 비교 근거](../spec/internal/evidence/c13-1-overflow-clip-float-boundary-2026-10-14/README.md)에서, radius/transform의 inline·author stylesheet fail-closed는 [unsupported effect 근거](../spec/internal/evidence/c13-1-overflow-clip-unsupported-effects-2026-10-14/README.md)에서 확인했다. Spinon float geometry parity는 C26 이후 비교한다. clip ancestry 세 번째 성장 예약 실패를 실제 CSS runtime worker에 주입해 이전 snapshot 제거와 layout 오류 게시를 확인하고, 같은 worker의 다음 environment revision에서 직접 fault-free 계산과 같은 전체 snapshot으로 복구함을 확인했다([worker recovery 근거](../spec/internal/evidence/c13-1-overflow-clip-worker-recovery-2026-10-14/README.md)). 이는 전역/플랫폼 allocator OOM 증거가 아니다. snapshot builder의 같은 reserve 재사용 회귀도 통과했다. 기존 구현·hit-test delta 구현 검토와 새 worker recovery 구현 검토는 각각 [기존 구현 검토](../spec/internal/evidence/c13-1-overflow-clip-implementation-review-2026-10-14.md), [query delta 검토](../spec/internal/evidence/c13-1-overflow-clip-hit-test-review-2026-10-14.md), [Android 앱 delta 검토](../spec/internal/evidence/c13-1-overflow-clip-app-hit-test-review-2026-10-14.md), [viewport negative-control 검토](../spec/internal/evidence/c13-1-overflow-clip-hit-test-negative-controls-review-2026-10-14.md), [worker recovery 구현 검토](../spec/internal/evidence/c13-1-overflow-clip-worker-recovery-implementation-review-2026-10-14.md)에 기록했다. 이번 iOS 통합의 신규 20개 구현 관점과 30개 PR delta 관점은 각각 [iOS implementation review](../spec/internal/evidence/c13-1-overflow-clip-ios-simulator-implementation-review-2026-10-14.md), [iOS PR review](../spec/internal/evidence/c13-1-overflow-clip-ios-simulator-pr-review-2026-10-14.md)에 기록한다. 이번 runtime matrix의 20개 구현 실패 관점과 30개 PR delta 관점은 각각 [구현 검토](../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/implementation-review-20.md), [PR 변경 검토](../spec/internal/evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/pr-delta-review-30.md)에 기록한다. PR 변경 관점 검토는 [기존 변경 검토](../spec/internal/evidence/c13-1-overflow-clip-pr-review-2026-10-14.md)와 [worker recovery delta 검토](../spec/internal/evidence/c13-1-overflow-clip-worker-recovery-pr-review-2026-10-14.md)로 분리했다. [PR #158](https://github.com/ohah/spinon/pull/158)은 계획 PR #157 branch 기반 draft이고 현재 GitHub status checks는 보고되지 않았다.
- PixelCopy 표본은 paint 결과만 확인한다. `elementFromPoint()` 대조는 고정 좌표의 box hit query이며 DOM/CSSOM 이벤트 전달과 전체 hit-test matrix는 미검증이다. 이 실행은 Android software renderer의 AVD 결과다.
- [구현 실패 관점 검토 20개](../spec/internal/evidence/c13-1-overflow-clip-implementation-review-2026-10-14.md)와 [PR 변경 관점 검토 30개](../spec/internal/evidence/c13-1-overflow-clip-pr-review-2026-10-14.md)는 별도 기록으로 유지한다.

`overflow-clip-margin`의 rounded edge, overflow와 border-radius, transformed descendant가 scrollable overflow에 미치는 효과는 각각 C22/C23에서 입력 계약이 제공된 뒤 교차 fixture를 추가한다. 현재 단계가 이를 지원하지 않으면 해당 조합은 진단과 함께 fail-closed하며 C13 완료 관문에 통합 실행을 남긴다.

scrollable overflow area에는 descendant box overflow 등 scrollable overflow를 포함하되 shadow/outline 같은 ink overflow를 자동으로 더하지 않는다. clip painting, hit-test rejection, `scrollWidth`/`scrollHeight`는 서로 다른 관찰로 고정하고 같은 fixture에서 어느 하나의 성공을 다른 하나의 성공으로 대체하지 않는다.

### CSSOM·이벤트 교차 소유권

- C13.2의 내부 scroll state/command는 C13 소유다. 앱이 호출하는 `Element.scrollTop/scrollLeft`, `scroll()/scrollTo()/scrollBy()/scrollIntoView()`, `Window` scroll API 및 `scroll` event 공개 동작은 [CSSOM View](https://www.w3.org/TR/cssom-view-1/) 원본과 DOM/CSSOM owner인 C29/S05 계약으로 제공한다. 이 API 또는 event가 구현되기 전에는 C13에서 programmatic scrolling/public event support 완료를 주장하지 않는다.
- layout/tree/style 변경은 `DocumentRevision`/`StyleRevision`을 소유자 규칙대로 갱신한다. scroll-only change는 별도 monotonic `ScrollStateRevision`을 올리고, overflow/scroll geometry를 쓰는 snapshot과 frame/hit-test 모두 같은 값을 요구한다. counter overflow나 allocation 실패에서 revision/offset 일부만 바뀌면 안 된다.
- scroll state는 `NodeId`와 document generation에 귀속한다. node 제거/재사용, source parent 이동, root 전환 때 이전 scroll offset이 다른 요소에 붙지 않아야 한다. `scroll` event는 최종 scroll revision에 연결하고 JS 실행을 UI/GPU 스레드에서 기다리지 않는다.

## 구현 전 비교 기준과 판정

### C13.1 고정 관찰

agent-browser로 만든 Chrome 155 사전 비교 fixture는 [원본 HTML](../tests/fixtures/css/c13/overflow-c13-1.html), [computed/geometry observation](../spec/internal/evidence/c13-overflow-precomparison-2026-10-14/chrome155-observation.json), [screenshot](../spec/internal/evidence/c13-overflow-precomparison-2026-10-14/chrome155.png)에 있다. `visible`, `hidden`, `clip`, `scroll`, `auto`, 두 mixed-axis 조합, clip margin, rounded border 대조, hidden/clip/flow-root float 경계를 포함한다. 관찰값은 plan PR에서 C13.1의 시작 기준으로만 사용한다.

### 각 단계 비교 원칙

1. Chromium fixture source·stylesheet·inventory·browser binary SHA, build/revision, viewport·DPR, user-agent, OS를 캡처 전에 기록한다. 사이트/fixture 상호작용은 agent-browser를 사용한다. terminal-browser는 사용하지 않는다.
2. 지원 CSS computed values는 정확 비교한다. frame/scroll offset은 CSS px로 node별 대조하며 geometry 축별 허용 오차는 `0.5 CSS px`다. 누락 node, 잘못된 scroll owner, clip 누수, revision mismatch는 평균 오차로 숨기지 않고 즉시 실패다.
3. 표면 비교는 같은 Chrome PNG와 같은 CSS viewport/DPR에서 선택 좌표를 사전 등록한다. Android API 37 ARM64 AVD PixelCopy와 iOS Simulator drawable을 별도로 비교한다. 캡처된 surface의 backend/scale을 기록하며 pixel exact는 선택한 표본에만 주장한다.
4. WPT는 해당 단계 전용 후보를 고정 commit/source hash로 분류한다: `Spinon fixture 후보`, `Chrome oracle-only`, `미지원`. WPT upstream/Chrome PASS는 Spinon WPT 적합성 PASS가 아니다.
5. 직접 oracle 수집이 막히는 항목은 명세에서 도출한 예상값과 불변조건을 먼저 기록하고, 구현 결과를 그대로 expected 값으로 삼지 않는다.

## 실패·수명 관문

- clip ancestor chain, nested scroll chain, scrollbar geometry, snap target 수집은 bounded allocation을 하고 `try_reserve`/checked arithmetic 실패를 구체 오류로 반환한다. 자식/일부 축만 새 상태로 게시하지 않는다.
- parent/child 재배치, 숨김, display 변경, scrollable area 축소, viewport/scale 변경, surface generation 교체를 각각 stale control로 둔다. style/layout revision이 달라진 동안 예전 clip/scroll snapshot을 새 frame과 hit-test에 쓰지 않는다.
- 손가락 cancel, 두 번째 pointer, OS가 입력을 취소한 경우 현재 gesture를 종료하고 늦은 move/up을 다른 scroll container에 재결합하지 않는다.
- diagonal delta에서 한 축만 소진됐을 때 남은 축 성분을 부모 scrollport에 전달하는지, axis-lock 이후 direction 변경을 받아들이는지 별도 trace로 기록한다. 임의의 각도 임계값은 oracle 없이 제품 계약으로 고정하지 않는다.
- ink-only overflow는 scroll range를 늘리지 않아도 clip되거나 그려질 수 있으므로 clip draw/hit 관찰과 scroll range 관찰을 별도 판정한다.
- `overflow:clip`과 `hidden`, `visible`과 `auto`, overflow shorthand와 longhands, `overflow-clip-margin`이 무시되는 scrollable value를 서로 바꿔치기하지 않는다.
- scroll event/callback은 UI/GPU 호출 흐름과 비동기 경계를 지키며, queue full/removed document/stale root의 정확한 결과를 기록한다.

## 제외 및 선행 교차 단계

- `text-overflow`, line clamp와 line-box 기반 scroll area는 C15 텍스트 layout 단계가 소유한다. C15 line box를 반영하기 전 C13 profile은 빈 fixed-size box fixture만으로 전체 문서 scroll overflow를 완료했다고 주장하지 않는다.
- replaced content의 ink/scroll overflow는 C14/C18와의 교차 fixture로 닫는다.
- `clip-path`, mask, transform/perspective, filter, general compositing은 C23 소유다. CSS `overflow` clip과 합쳐지는 경우 C23 이후 교차 검증한다.
- CSSOM 전체, `scrollIntoView()`와 `scroll` event 공개 계약은 C29/S05 owner를 따른다. C13 내부 scroll engine test가 공개 API 완료를 대신하지 않는다.
- scroll-linked animation/view timeline은 C27 이후다. print/paged media, smooth-scroll easing/시간의 완전한 OS별 동일성, browser-wide scrollbar pseudo-element, virtualized content/listener API는 이 계획으로 지원했다고 주장하지 않는다.

## PR·검토 규칙

- 코드를 시작하기 전에 이 계획을 별도의 실패 관점 20개로 검토한다. 이 계획 PR을 포함한 모든 PR은 필수 프로젝트 규칙에 따라 서로 다른 변경 관점 30개를 병합 전에 추가 검토한다.
- 구현 전 비교·fixture inventory·pass/fail 기준을 각 단계 PR에 고정한다. 각 단계 구현 뒤에는 계획 검토와 겹치지 않는 20개 신규 implementation failure perspective를 코드·실행 결과와 대조한다. 이 20개는 PR 필수 변경 검토 30개를 대체하지 않는다. 결함을 고치면 영향받은 관점을 다시 확인한다.
- Android 실기기 연결을 요구하지 않는다. Android는 명시한 API 37 ARM64 AVD serial만, iOS는 지정 Simulator UDID만 쓴다. AVD software backend와 Simulator drawable 결과는 physical hardware/display 결과로 확대하지 않는다.
- runtime fixture module·FFI·payload는 전용 opt-in `*-fixture-test` feature에 격리한다. 기본/제품 artifact와 fixture-test artifact를 각각 검사해 FFI symbol/string/payload의 부재·존재 negative/positive control을 남긴다.
- 기능 PR에서는 `cargo fmt`, 해당 unit/integration/reference tests, workspace test, Clippy, Android/iOS simulator build/run을 변경 책임에 맞춰 실행한다. 실행하지 않은 WPT·플랫폼·실기기는 미실행으로 기록한다.
- C13 계획/구현 PR을 squash/merge commit으로 병합하지 않는다. main merge 전 required CI, 리뷰, mergeability를 다시 확인한다. Pages workflow는 실행하지 않는다.
