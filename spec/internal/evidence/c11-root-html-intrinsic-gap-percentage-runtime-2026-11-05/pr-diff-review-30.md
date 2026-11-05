# C11.1 intrinsic Grid percentage gap 구현 PR diff 적대 검토 30개

**대상:** PR [#1176](https://github.com/ohah/spinon/pull/1176)의 실제 diff. 기준 `5d47b06cc360d2c77cb27ab0fcd40f1ff63ccb3f`, 1차 구현 `12a2493efb87a914ac06d69d4a7987017de8b309`, simulator runner/evidence 보강 `5e97bcc31e65584059780ef76104ec6f5de90db3`.

**검토 방법:** 레이아웃·placement·Taffy adapter·runtime projection/test·fixture·실행 스크립트·시뮬레이터 산출물·상태 문서·PR 설명을 각각 실패 경로로 대조했다. CSS Gaps §2.3과 CSS Grid 2025-03-26 CRD §10.1은 Agent Browser로 W3C 원문을 확인했다. 이 검토에서 새 test/build는 실행하지 않았고, 아래 실행 판정은 PR evidence의 저장된 결과다.

| # | 독립 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 퍼센트 gap 허용이 fixed-track 외 Grid/Flex profile로 번지는가 | 신규 분기는 `RootHtmlIntrinsicTrackSizingV1` validator에 한정된다. 다른 one-axis profile은 여전히 `LengthPx(0)`만 허용한다. **통과**. |
| 2 | definite가 아닌 축을 basis로 percentage를 받아들이는가 | intrinsic profile은 기존 계약대로 finite positive fixed width와 height만 허용하고, validator도 `LayoutDimension::Fixed`가 아니면 실패한다. **통과**. |
| 3 | row/column percentage가 서로 반대 축을 참조하는가 | row는 height, column은 width를 넘긴다. CSS Grid `gap:25% 10%`의 120×80px 사례에서 row 20px·column 12px 사용값과 item 위치를 대조했다. **통과**. |
| 4 | 고정 dimension이 content-box basis와 다를 수 있는가 | 이 profile은 margin·padding·border 0을 요구한다. width/height가 content-box와 같아 W3C percentage basis와 일치한다. **통과**. |
| 5 | Percentage 값이 `0..100` scale로 잘못 변환되는가 | `LayoutLengthPercentage::percent(0.1)`가 Taffy `LengthPercentage::percent(value)`로 그대로 전달된다. 10% 사례가 10px 및 기대 frame으로 끝난다. **통과**. |
| 6 | Taffy 앞뒤에서 used gap을 두 번 환산하는가 | validator는 f64로 admission 경계만 확인하고 `CompactTaffyStyle`은 typed `style.gap`을 기존처럼 전달한다. 추가 px 변환·스타일 mutation은 없다. **통과**. |
| 7 | 첫/마지막 track 바깥에 gutter를 더하는가 | CSS Grid §10.1과 Taffy vector 의미를 대조했다. admission의 총 gutter 산정은 `track_count - 1`이며 fixture frame이 같은 계산을 따른다. **통과**. |
| 8 | track 개수가 0 또는 1일 때 underflow/허위 gutter가 생기는가 | 이 validator에서 `saturating_sub(1)`를 사용한다. profile 검증은 track 목록이 비어 있으면 gap 검사 전에 거부한다. **통과**. |
| 9 | shorthand의 첫 번째 값을 column gap으로 읽는가 | 독립 expected와 runtime test가 `gap:25% 10%`를 row 20px, column 12px로 확인한다. 실제 item origin도 x=52, y=40이다. **통과**. |
| 10 | legacy `grid-gap` alias가 longhand 경로를 우회하는가 | alias 장면은 같은 25% 10% expected frame과 matched Chrome comparison을 사용한다. **통과**. |
| 11 | `0%`가 기존 no-gap geometry를 바꾸는가 | layout test와 9-scene runtime expected에서 zero-percent를 control과 비교해 두 축 track 및 3개 frame이 일치한다. **통과**. |
| 12 | 100% 초과 gap이 clamp되거나 잘못 거부되는가 | 120% row/column을 positive expected로 고정했다. flexible second track은 0이 되고 fixed item은 120px gutter 뒤에 남아 overflow한다. **통과**. |
| 13 | 기존 fixed-px gap이 percentage 조건 때문에 깨지는가 | 같은 profile의 기존 fixed-px track/frame test가 유지되고 runtime regression도 통과했다. fixed length는 basis 없이 자기 값으로 검사된다. **통과**. |
| 14 | CSS-invalid negative declaration이 유효 gap으로 올라오는가 | runtime mutation은 `gap:-1px`를 넣고 cascade fallback에 따라 zero-gap control과 같은 전체 geometry를 확인한다. **통과**. |
| 15 | 유효한 `calc()` gap이 direct percentage 지원으로 섞이는가 | typed validator는 `Calc`를 거부하며 runtime test에서 cascade는 Ready, layout은 `column-gap` 오류, completed output은 None이다. **통과**. |
| 16 | negative/NaN/infinite typed gap이 Taffy까지 전달되는가 | layout input의 공통 finite/nonnegative 검증과 intrinsic validator가 각각 방어한다. 변경된 tests는 negative와 NaN을, 기존 spacing test는 infinity를 검사한다. **통과**. |
| 17 | CSS에서 표현할 수 없는 큰 computed percentage가 조용히 포화되는가 | `3e38%` runtime mutation은 cascade 이후 `unsupported_computed_value`의 `column-gap` 실패로 끝나며 partial layout을 만들지 않는다. **통과**. |
| 18 | finite f32 basis와 fraction 곱이 f32 범위를 넘는가 | `used_gap`은 f64 곱으로 구하고 유한성 및 f32 최대값을 확인한다. 단일 used gap overflow를 typed test가 거부한다. **통과**. |
| 19 | 여러 gutter의 총합만 f32 범위를 넘는가 | aggregate도 f64에서 `(track_count - 1) × used_gap`으로 검사한다. 3개 track row 사례가 두 gutter의 overflow를 거부한다. **통과**. |
| 20 | Taffy 내부 f32 연산에서 track line이 비유한 값이 되는가 | 계산 이후 `DetailedGridInfo`의 row/column position start/end를 frame 수집 전에 검사한다. positive used gap이 해당 축을 검사 대상으로 고른다. **통과**. |
| 21 | explicit count 산정에서 implicit track gutter를 빠뜨리는가 | placement 정규화는 definite endpoint가 explicit 범위를 벗어나면 거부하고 intrinsic 모드의 expanded row도 거부한다. `grid-auto-rows/columns` 역시 Auto만 허용해 implicit track을 scope 밖으로 둔다. **통과**. |
| 22 | track info가 만들어지지 않은 Grid에서 positive percentage gap이 검증을 건너뛰는가 | non-Grid `DetailedLayoutInfo` 분기에서도 fixed basis로 양수 used gap을 계산해 property error를 낸다. zero-used percentage만 skip한다. **통과**. |
| 23 | row gap 오류를 column으로, column gap 오류를 row로 보고하는가 | 두 layout admission 경계와 post-Taffy row/column position 검사 모두 각 gap·basis·property를 같은 tuple에 묶는다. runtime projection negative도 `column-gap`을 확인한다. **통과**. |
| 24 | 실패한 장면의 일부 frame 또는 render snapshot이 publish되는가 | unprojectable percentage·`calc()`·gutter/track overflow runtime tests는 `completed=None`을 단언한다. 성공 9 scenes는 27 item frame과 render box 포함을 검사한다. **통과**. |
| 25 | reference expected가 Chrome 관측값을 복사해 독립 검증을 가장하는가 | expected JSON은 고정 item 기여와 축별 산술 방법을 명시하고 Chrome observation은 별도다. comparison은 9/9 scene의 track/frame/gap syntax 일치를 별도 기록한다. **통과**. |
| 26 | simulator가 다른 자산 또는 다른 fixture를 실행했는가 | 보강 runner는 source fixture SHA/byte 수와 Android APK·iOS app bundle 자산 SHA가 같은지 각각 강제한다. 재실행 로그도 동일 SHA를 담는다. **통과**. |
| 27 | runtime log marker가 없어도 simulator 실행이 성공으로 기록되는가 | 1차 diff 검토에서 `rg ... || true` 누락을 발견했다. 보강 runner는 추출 실패를 전파하고 source·layout-ready 48·draw/present 48 marker를 모두 요구한다. 두 번째 실행에서 통과했다. **수정 후 통과**. |
| 28 | 캡처 명령이 실패하거나 빈 파일을 만든 상태로 화면 근거를 남기는가 | 보강 runner는 두 screenshot 파일이 non-empty인지 확인하고 최종 SHA-256을 쓴다. 저장된 Android/iOS PNG hash가 manifest와 일치한다. **통과**. |
| 29 | 중간 실패가 앱 프로세스나 앱 원본 HTML 자산을 남겨 두는가 | cleanup flag를 launch 전에 설정하고 EXIT trap으로 앱을 종료·asset backup을 복구한다. 저장된 원본 asset hash는 양 플랫폼 모두 `38813ca8…df85`이며 재실행 후 앱은 종료됐다. **통과**. |
| 30 | 제한 후보를 전체 C11 완료나 green CI로 과장하는가 | plan·STATUS·contract·README·PR 본문은 C11.1/C11 미완료, `78/214`, visual-smoke 한계와 broad-suite 기존 실패를 숨기지 않는다. 이 검토 시점 GitHub check rollup은 비어 있고 CI 통과라고 주장하지 않는다. **통과**. |

## 결과

30개의 서로 다른 실패 경계를 검토했다. 구현 검토에서 남은 제품 결함은 찾지 못했다. 첫 diff pass에서 simulator harness의 실패 은폐와 부족한 성공 판정 조건을 찾아 수정한 뒤 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator를 다시 실행했다. fixture hash, `layout=ready boxes=48`, draw/present 48개와 screenshot 존재 검사가 통과했다. PNG는 시뮬레이터 화면 확인용이며 PixelCopy·Metal drawable readback이나 실기기·hardware GPU·iOS compositor 이후 pixel 일치 증거가 아니다.

기존 저장 evidence에서 focused layout 8/8, runtime 11/11, scoped Clippy, FFI check, fixture boundary 33/33 및 rustfmt가 통과했다. 전체 layout suite 449/450의 단일 C15 HostDocument assertion은 fixed-px baseline 447/448에서도 실패했다. 전체 fixture-feature runtime suite는 후보 276/8/3, 기준 275/8/3이고 실패 집합이 동일하다. 두 crate 전체 실행을 green으로 취급하지 않는다. 이 PR 검토 당시 GitHub check rollup은 비어 있었다.

Normative references: [CSS Gaps §2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties), [CSS Grid 2025-03-26 CRD §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters-the-row-gap-column-gap-and-gap-properties).
