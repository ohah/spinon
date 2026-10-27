# C11.4 행 축 겹침 span runtime slice

## 목표와 경계

[행 축 span 단계 사전 비교](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)의 네 고정 장면을 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in 앱 fixture 경로에서 실행한다. 행 축의 겹치는 span 두 개가 같은 단계에서 계획 증가량의 track별 최댓값을 사용하고, 더 긴 span을 그다음 단계에서 처리하는지 Android·iOS Simulator의 trace와 item frame으로 확인한다.

이는 앱의 fixture 입력 경로와 두 플랫폼의 검증 matcher를 확인하는 제한 실험이다. 일반 CSS admission, 공개 CSS 지원, 자동 최소값, implicit track, gap, intrinsic text/image, 다른 sizing function, 임의 placement, Stage B 전체, auto-repeat, 전체 C11.4를 완료 처리하지 않는다. 공개 API나 내부 계약 숫자 버전을 추가하지 않는다. 완료 후에도 `spec/STATUS.md`의 C11/C11.4 checkbox와 공식 완료 집계 `72/213`은 유지한다.

## 고정 입력

네 scene ID는 기존 column 장면 `competing-span2-forward`, `span2-before-span3`와 겹치지 않도록 `row-axis-` 접두어를 사용한다.

| Scene | DOM 순서 | 행 배치 | Item block size |
| --- | --- | --- | ---: |
| `row-axis-span2-forward` | A, B | A: 1–2; B: 2–3 | 80px, 120px |
| `row-axis-span2-reversed` | B, A | 동일 | 120px, 80px |
| `row-axis-span2-before-span3` | A, B, C | A: 1–2; B: 2–3; C: 1–3 | 80px, 120px, 200px |
| `row-axis-span3-before-span2` | C, B, A | 동일 | 200px, 120px, 80px |

각 root는 `220×220px`, `grid-template-columns:220px`, `grid-template-rows:repeat(4,minmax(0px,max-content))`, 0px gap, `align-content:start`를 사용한다. Item은 220px 너비의 고정 높이 빈 `div`이며 `grid-column:1 / 2`와 명시적인 `grid-row`를 모두 갖는다. 겹치는 row placement에서 `grid-column:auto`를 남기면 implicit column을 만들 수 있으므로 허용하지 않는다. margin, padding, border, self-alignment, text, image, auto-placement는 넣지 않는다. 입력은 고정 HTML fixture 및 Rust stage oracle와 같은 의미여야 하며 Android/iOS 양쪽에서 같은 scene ID와 authored child order를 사용한다.

## 독립 기대값

규범 기대값은 CSS Grid 2025 CRD §11.5.1/§11.6에서 유리수 산술로 도출한 [expected.json](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/expected.json)과 [Rust stage oracle](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis.rs)을 기준으로 한다. Chrome 156 관측은 최종 geometry의 별도 교차 확인이며 stage trace oracle로 사용하지 않는다.

- Initialize: 네 Rows track의 base size는 `[0,0,0,0]`이다.
- span-2 batch: item A/B의 planned increase는 각각 40/60px이고, 겹치는 두 번째 row에는 60px의 최댓값을 적용한다. Rows base size는 `[0,0,0,0]`, growth limit은 `[40,60,60,∞]`가 된다.
- span-2 + span-3: 다음 batch 순서는 source order와 무관하게 span 3이다. C가 앞 세 track에 나누어 주는 증가는 각 `40/3px`이고 해당 growth limit은 `[160/3,220/3,220/3,∞]`가 된다.
- IntrinsicSizingComplete 뒤 fourth row의 growth limit은 base size 0px로 닫힌다. MaximizeTracks의 최종 Rows base size는 span-2-only 장면에서 `[40,60,60,0]`, 두 batch 장면에서 `[160/3,220/3,220/3,0]`이다. root 높이와 track 합의 차이는 각각 60px와 20px다.
- 전체 stage 이름과 순서를 `Initialize`, batch(들), `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks { applied: false }`로 확인한다. fixed Columns control은 220px 단일 track, intrinsic batch 없음, `FixedTrackEarlyReturn` 하나여야 한다.
- 각 Rows pass의 contribution trace는 span 순서, `Minimum`·`MinContent`·`MaxContent` 종류 순서, semantic item의 authored source order, span 및 고정 block contribution을 확인한다. 없는 항목·중복 항목·다른 node 번호를 허용하지 않는다.
- 규범 stage 숫자와 양 플랫폼의 trace matcher는 `1e-4 CSS px`, Chrome에서 관찰한 최종 geometry 및 양 플랫폼의 node frame은 `1/64 CSS px` tolerance를 쓴다. Chrome CSSOM의 양자화 값으로 규범 stage를 대체하지 않는다.
- Serializer의 `null` growth limit은 “유한 숫자가 직렬화되지 않음”으로만 기록한다. 이를 단독으로 +∞와 NaN 또는 다른 비수치 값을 구분하는 증거라고 주장하지 않는다. runtime trace에 별도 free-space 필드가 없으면 남은 공간은 root 크기와 최종 track 합에서 계산한 값으로 보고한다.

Forward/reversed 장면은 source order만 바꾸며 각 의미 item의 frame, Rows stage와 batch 순서가 같아야 한다. semantic item ID와 runtime node index mapping을 각 scene별로 명시해 DOM 순서 역전이 검증 대상에 실제 반영되게 한다. root와 모든 child의 전체 frame을 검사한다.

## 구현과 엄격한 수락 기준

1. Rust `fixtures/c11_4` 아래 독립 scene builder를 추가하고 `c11_4_grid_track_sizing_runtime_source`에 연결한다. 정확히 네 ID만 반환하고 알 수 없는 유사 ID는 거부한다. 생성된 source에 대한 focused Rust test는 root/track CSS, item 개수·행 배치·높이, DOM 순서, scene/root/node ID와 unlisted 거부를 검사한다.
2. Android와 iOS의 opt-in scene allowlist에 동일한 네 ID만 추가한다. 두 플랫폼은 기존 permissive C11.4 helper의 `expected == null => true` fallback에 기대지 않는 strict row-stage path를 사용한다. 신규 scene에서 예상 root/item/node frame이나 stage matcher가 빠지면 성공할 수 없어야 한다. 미등록 scene ID와 C11.4 이외 profile selector를 함께 넘기는 잘못된 인자는 두 플랫폼에서 거부되어야 한다.
3. 각 scene의 pass가 정확히 `[Columns, Rows]` 두 개여야 한다. Columns는 고정 220px early return이어야 한다. Rows 각 stage의 내부 track 배열은 정확히 9개로, 5개 gutter와 4개 CSS track이 번갈아야 한다. 두 경계 gutter만 collapsed이고 모든 gutter base size는 0이어야 한다. CSS track의 base size와 growth limit을 단계별로 비교한다. span batch 순서는 `[2]` 또는 `[2,3]`이어야 한다. 전체 stage 누락·중복·순서 변경, 예상 밖 batch, 추가 implicit track은 거부한다.
4. 각 semantic item의 contribution trace와 모든 runtime node frame을 scene별 DOM order로 비교한다. Span-2-only 장면은 root `220×220px`, A `(0,0,220,80)`, B `(0,40,220,120)`이다. span-2/3 장면은 root `220×220px`, A `(0,0,220,80)`, B `(0,160/3,220,120)`, C `(0,0,220,200)`이다. Reversed scene의 contribution node index와 frame 순서는 해당 semantic item에 맞게 바뀐다.
5. 기존 column `competing-span2-*` 및 `span*-before-span3` controls와 direct row text/image matcher를 회귀 확인한다. 새로운 scene의 전체 run matrix를 기존 대조군 성적으로 합치지 않는다.
6. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 장면 모두 각각 4/4 실행한다. frame·trace·scene marker·`layout=ready` 및 `presented` 결과를 로그에서 검사하고, 최소 한 장면의 Android/iOS screenshot을 근거로 저장한다. screenshot은 보이는 장면과 box 배치의 보조 증거이며 전체 화면 pixel equality를 뜻하지 않는다. readback을 별도로 exact 검사하지 않으면 pixel exact라고 보고하지 않는다.
7. Android/iOS 앱 source input hash와 앱 artifact hash, simulator/OS 식별자, scene별 원본 runtime log·screenshot hash를 manifest에 남긴다. 지정 simulator/AVD만 실행하고, 실행 후 앱 프로세스를 끝낸다. simulator boot 상태를 실행 전에 기록한다. 이 작업 전에 이미 booted된 simulator는 유지하며, 스크립트가 직접 boot한 simulator만 종료한다. 다른 booted device는 건드리지 않는다.
8. `c04-runtime-gpu-fixture-test`를 끈 기본 release FFI archive에는 신규 scene marker/string과 fixture ABI가 없어야 하고, opt-in fixture-test positive control에는 네 marker가 있어야 한다. 이 결과는 final APK/IPA 전체 exclusion 증거로 확대하지 않는다.
9. 구현/runtime 코드와 플랫폼 결과가 고정 source revision에서 통과한 뒤 별도의 새 실패 관점 20개 검토, PR 변경 전체의 30개 검토를 진행한다. 결함을 고치면 영향을 받은 관점을 다시 확인한다. PR merge와 문서 sync 뒤 공식 C11.4 미완료 및 `72/213`을 유지하고, Tailnet에 배포할 경우 Agent Browser로 확인한 뒤 해당 Agent Browser 프로세스를 종료한다. Terminal Browser는 사용하지 않는다.

## 제외 사항

이 slice는 네 고정 scene의 앱 fixture 경로와 두 플랫폼 matcher를 검증한다. 자동 최소값, fixed/flexible/fit-content track 조합, intrinsic text/image item, implicit row, gap, flex expansion, 일반 author CSS, WPT, 실기기, hardware GPU, compositor 이후 표시, 앱 전체 release fixture exclusion은 범위에 없다.
