# 0001 · 범위와 적합성

**상태:** 제안 · **명세 버전:** `0.1.0-draft`

## 제품 경계

스피논은 같은 앱 작성 소스를 웹 DOM과 모바일 GPU UI 경로로 빌드하는 프레임워크다. 모바일은 브라우저 문서·DOM 전체·Web API 전체를 구현하지 않는다. 대신 Rust UI 트리에 연결하는 제한된 DOM API façade를 제안하며, 후보와 미정 계약은 [모바일 DOM 호환 명세](0007-dom-compatibility.md)에 둔다. Android와 iOS의 일반 UI 노드는 각각의 플랫폼 뷰와 1:1로 대응하지 않는다. WebView는 명시적인 별도 컴포넌트다.

React, Vue, Svelte는 서로 다른 작성 문법과 상태 관리 방식을 가진다. 스피논의 공통 계약은 태그 의미, 지원 CSS, 이벤트, 호스트 명령이다. 첫 적합성 대상은 React·Vite·단일 모바일 번들이다. 다른 프레임워크와 번들러는 각각 별도 적합성 대상이다.

## 지원 표기

**목표 범위**와 **현재 적합성 상태**는 별도 열로 기록한다. 구현 전에 정하는 첫 수직 구현의 포함·제외·미정 범위는 제품의 `지원` 판정이 아니다. 아래 표기는 구현 결과를 판정할 때 사용한다.

| 표기 | 의미 |
| --- | --- |
| `지원` | 지정 버전·플랫폼에서 적합성 시나리오와 실기기 또는 브라우저 증거가 있다. |
| `제안` | 문서가 목표 동작을 기술하는 단계다. 현재 적합성 판정이 아니다. |
| `미검증` | 목표 범위는 정했지만 해당 버전의 구현·진단 결과를 아직 확인하지 않았다. |
| `차이` | 같은 API 이름이나 문법을 쓰되 웹과 다른 관찰 결과가 검증됐다. |
| `미지원` | 해당 버전에서 제공하지 않으며, 빌드 또는 개발 실행에서 위치·항목을 식별할 수 있게 진단한 근거가 있다. |
| `미정` | 계약 자체가 결정되지 않아 지원을 주장할 수 없다. |

일부 CSS 선언이나 이벤트를 조용히 무시한 화면을 `지원`으로 표시해서는 안 된다. 지원표에는 최소한 플랫폼, 프레임워크 어댑터, 엔진·번들 형식 버전, 입력, 기대 결과, 오류, 접근성 결과를 기록한다.

첫 범위 표를 작성할 때는 기능마다 `첫 수직 구현 목표: 포함/제외/미정`과 `현재 적합성: 미검증/지원/차이/미지원/미정`을 분리한다. 제외하기로 결정했어도 진단이 구현되지 않았다면 현재 상태는 `미검증`으로 남기고, 근거 없는 `미지원` 판정을 붙이지 않는다.

## 구현 전 비교 모델

모든 기능 구현과 동작 수정은 코드를 바꾸기 전에 비교 모델을 정한다. 비교 모델은 구현 결과가 맞는지 판별하는 독립된 기준이다. 작업 단위마다 다음 항목을 기록한다.

1. **기준:** 고정 버전의 기준 구현(예: Chromium 또는 플랫폼 기본 동작), 표준·명세에서 도출한 예상값, 또는 골든 결과 중 무엇을 oracle로 쓸지 정한다. 구현 코드 자신을 유일한 기대값 원본으로 삼지 않는다.
2. **재현 입력:** fixture·초기 상태·기기/OS·빌드·엔진 버전·viewport·글꼴·타이밍 등 결과를 바꾸는 조건을 고정한다.
3. **관찰값:** 결과를 비교할 수 있도록 DOM/CSS 값, 좌표, 이미지, 이벤트 순서, 로그, 오류, 파일·바이트 등 기능에 맞는 값을 정한다.
4. **판정 규칙:** 정확 일치 여부, 허용 오차와 차이 분류, 실패 조건을 결과를 보기 전에 정한다. 평균 점수로 필수 조건 실패를 감추지 않는다.
5. **독립 확인:** 기존 기준 구현을 쓸 수 없으면 명세에서 만든 예상 결과, 불변 조건 또는 별도 구현과 대조하는 대체 oracle을 기록하고 그 한계를 공개한다.

비교 모델·fixture가 준비되지 않은 구현 작업은 제품 지원 완료로 판정하지 않는다. 새 기능이나 수정이 이 모델을 바꾸면 입력·기대값·판정 기준도 함께 갱신한다. 기능별 시각·레이아웃 비교의 세부 측정은 [CSS 호환 명세](0008-css-compatibility.md)를 따른다.

C04 runtime CSS→Taffy 작업의 구현 전 oracle, 입력 hash, computed value와 CSS px 좌표 기준은 [runtime style-layout 비교 근거](internal/evidence/css-c04-runtime-style-layout-precomparison-2026-10-09.md)에 고정했다. 이는 Chromium 출력 기준이며 Spinon 구현 결과를 포함하지 않는다.

S03.3 세션 종료 검증은 [종료 경합 사전 비교 모델](internal/evidence/s03-shutdown-precomparison-2026-10-05.md)에 고정했다. 이 검증은 Rust 세션의 `Closing` 전환, 실행 중 V8 취소, 대기 명령의 종료 오류, 이후 접수 거부와 소유 스레드 종료를 대상으로 한다. raw C ABI 세션 포인터에 대한 동시 `free`는 기존 계약에서 금지하므로 시험하지 않는다.

R06 우선순위 공정성 후보 비교의 동일 입력·정책 매개변수·판정 기준은 [R06 공정성 정책 비교 모델](internal/evidence/r06-priority-policy-comparison-2026-10-08.md)에 기록한다. 이 모델은 선택 규칙만 비교하는 결정론적 내부 fixture이며 제품 동작이나 사용자 API를 정하지 않는다.

R06 큐 포화·복구 검증의 구현 전 기준과 사전 판정값은 [큐 포화·복구 비교 모델](internal/evidence/r06-queue-saturation-precomparison-2026-10-08.md), 작업 분해와 미해결 경계는 [R06 큐 포화 계획](../plan/r06-queue-saturation.md)에 기록한다. 이는 현재 내부 queue admission의 관찰 기준이며 제품 역압력 계약이 아니다.

C13.3 normalized wheel runtime의 고정 입력, Chrome 155 oracle, 축별 사전 판정값과 Simulator 검증 경계는 [wheel delta routing 계획](../plan/c13-3-wheel-delta-routing.md)에 기록한다. 입력은 opt-in fixture가 CSS px로 정규화한 sample이며 OS native wheel/trackpad adapter 또는 공개 `WheelEvent`를 검증하지 않는다.

C11.4 Grid track sizing의 구현 전 Chrome 155 oracle 26개, Ahem/image resource hash, fixed WPT source revision·file hash와 미검증 경계는 [사전 비교 기록](internal/evidence/c11-4-grid-track-sizing-precomparison-2026-10-19/README.md)에 둔다. C14.3 image 입력은 재사용 대상이다. Rust는 in-flow direct Grid text/replaced leaf의 Taffy intrinsic-size callback 출처·조건·revision trace도 기록한다. 별도 opt-in 후속 fixture는 한 단계 nested Grid의 Ahem text와 natural PNG image를 함께 입력해 parent contribution을 분리한다. 세 고정 scene에서만 nested Grid contribution `MinContent=98px`(span 1/2) 및 definite flex `Minimum=0px`(span 2), inner tracks `70/24px`를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 확인했다. [혼합 text/image 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-nested-grid-replaced-item.md) · [Chrome 156 사전 비교](internal/evidence/c11-4-nested-grid-replaced-item-precomparison-2026-10-21/README.md) · [Android/iOS 실행·화면·artifact 경계](internal/evidence/c11-4-nested-grid-replaced-item-2026-10-21/README.md). 이는 고정 한 단계 topology의 opt-in fixture이며 arbitrary nested Grid, subgrid, 일반 HTML admission, image async replacement, WPT 또는 전체 track-sizing support를 뜻하지 않는다. anonymous Grid text, automatic minimum의 전체 행렬, 전체 Grid algorithm도 계속 미완료다. C11.4와 공식 완료 수 77/213은 유지한다. [기존 nested text 후속](../plan/c11-4-nested-grid-intrinsic.md) · [PR #442](https://github.com/ohah/spinon/pull/442). opt-in auto-repeat은 각 track list에 최대 하나의 repeat component와 제한 fixed-minimum pattern을 보존한다. 2026-10-20 fixed-neighbor 후속은 한 축의 repeat 앞뒤에 유한·0 이상 fixed `px` explicit neighbor를 허용했다. 같은 날 이어진 양축 후속은 columns와 rows에 하나씩 auto-repeat을 독립 허용하고 `auto-fill`/`auto-fit`, repeat range, repeat count와 collapse를 분리한다. line names, template areas, 비-`px` neighbor, 한 list 안의 복수 repeat 및 integer-repeat 혼합은 계속 거부한다. Chrome 155 사전 비교 12개와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 각 33/33 scene에서 axis별 track, collapse, root/item frame, trace와 draw/present를 확인했다. overflow 장면 item x=86은 root width 40을 넘고 root 경계 및 item color 표본은 두 simulator에서 각각 4/4 exact다. 이는 border-stroke 비교가 아니다. WPT, 실기기, Android hardware GPU 및 iOS compositor 이후 surface는 검증하지 않는다. [양축 후속 계획](../plan/c11-4-two-axis-auto-repeat.md) · [Chrome 12-case 사전 비교](internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/README.md) · [Android/iOS 실행·화면·hash](internal/evidence/c11-4-two-axis-auto-repeat-2026-10-20/README.md) · [구현 검토 20개](internal/evidence/c11-4-two-axis-auto-repeat-2026-10-20/implementation-review-20.md). [내부 계약 0059](internal/0059-c11-grid-layout.md).


2026-10-22 C11.4 cyclic `minmax(calc())` cross-axis span groups fixed opt-in runtime: 네 고정 scene의 source-order, item별 text intrinsic contribution, 두 축 track/frame과 draw/present를 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 4/4 확인했다. intrinsic 차이는 1/64 CSS px 안이다. definite overflow 표본은 AVD PixelCopy와 compositor 제출 전 iOS Metal drawable에서 각 4/4 exact다. 기본 release FFI archive에는 신규 scene ID/fixture entry marker가 없고 opt-in archive에는 있다. [계획](../plan/c11-4-cyclic-minmax-calc-cross-axis-span-groups.md) · [실행·화면·artifact·한계](internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/README.md) · [구현/runtime 검토 20개](internal/evidence/c11-4-cross-axis-span-groups-runtime-2026-10-22/implementation-runtime-review-20.md). 이 제한 fixture는 전체 C11.4 완료를 뜻하지 않는다. WPT, 실기기/Android hardware GPU 및 iOS compositor 이후 화면은 미검증이며 공식 집계 `77/213`도 유지한다.

C14.1 live V8 image measurement의 고정 Chrome 155 PNG oracle, V8 JS 생성 입력, actor-owned HostDocument snapshot, source/revision identity와 CSS px frame 판정은 `plan/c14-1-live-v8-image-measurement.md`에 기록한다. Android/iOS Simulator opt-in report는 실제 V8 변경 문서 snapshot을 기존 decoder/layout adapter에 연결하는 내부 evidence이며, product resource loader·async replacement·image paint 또는 공개 CSS 지원을 판정하지 않는다.

C18.1은 같은 제한 PNG fixture를 live V8→WGPU 실제 image paint로 연결해 Android API 37 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 visible border-overpaint/overflow 및 hidden padding-edge clip 표본을 검사한다. 이는 고정 opaque natural-size PNG 하나의 opt-in 내부 profile이며, 제품 image loader·공개 HTML/CSS 지원·C14/C18/U05 전체를 완료로 표시하지 않는다. 계약은 [0064](internal/0064-c18-1-image-paint.md), 실행·artifact 근거는 [C18.1 evidence](internal/evidence/c18-1-image-paint-artifact-boundary-2026-10-16/README.md)에 둔다.

C18.2는 고정 RGBA8 PNG의 alpha 0/128/255 수직 띠를 불투명 `#112233` 위에 natural size로 그리는 내부 fixture profile이다. Agent Browser Chrome `155.0.8059.85`의 80×40 DPR 1 oracle를 Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 10/10 exact로 비교했고, 두 Simulator 전체 화면 대응 sample 10/10과 fixture 밖 anchor 4/4도 확인했다. 구현과 fixture-only interface는 [계획](../plan/c18-2-static-alpha-image-compositing.md)·[내부 계약 0065](internal/0065-c18-2-alpha-image-compositing.md), 원본과 제한은 [사전 비교](internal/evidence/c18-2-alpha-precomparison-2026-10-16/README.md)·[Android 근거](internal/evidence/c18-2-alpha-image-android-api37-2026-10-16/README.md)·[iOS 근거](internal/evidence/c18-2-alpha-image-ios-simulator-2026-10-16/README.md)에 둔다. 이는 제품 image loader, 일반 surface format 전체, C18/U05/C14 또는 공개 HTML/CSS 지원을 완료 처리하지 않는다.

C19.1은 Stylo computed color의 원본 색 공간·missing bit, 매핑 전 sRGB 변환, CSS Color 4 Local MINDE 결과와 Chrome 155 sRGB/unorm8 compatibility 출력을 구분한다. 비-system fixture 37개는 Agent Browser 기준과 exact 일치하지만, 네 system-color 값은 Stylo Servo palette로만 검증하며 macOS system palette 동등성을 주장하지 않는다. runtime mount의 합성 body 상속을 별도로 검사했지만, 이 내부 carrier는 실제 GPU surface, opacity/group compositing, text glyph, hit-test 또는 contrast를 검증하지 않는다. [계약 0068](internal/0068-c19-1-computed-color.md) · [Chrome 사전 비교](internal/evidence/c19-1-computed-color-precomparison-2026-10-16/README.md) · [호환성 차이 분류](internal/evidence/c19-1-computed-color-compatibility-triage-2026-10-16.md)

C15 사전 비교는 Agent Browser Chrome `155.0.8059.85` / macOS `26.5.1 ARM`, locale `ko-KR`, 320×480 CSS px, DPR 1을 쓴다. public-domain Ahem line fixture와 hash-pinned OFL Roboto variable fixture의 Range/element geometry를 고정하며, 같은 글자 수 `iiii`/`WWWW`가 서로 다른 shaped advance를 내야 한다. 이 결과는 Chrome DOM oracle이고 Spinon runtime·Android/iOS Simulator 앱 검증이 아니다. [C15 계획](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md) · [사전 비교·font hash·screenshot](internal/evidence/c15-inline-text-precomparison-2026-10-17/README.md). C15.1에는 Parley 기반 내부 shaping과 Taffy measure adapter를 구현했으며 spinon-text-layout 11개, spinon-layout 144개 테스트와 Clippy/rustfmt/build/artifact-boundary 확인 로그를 보존했다. C15.2a는 두 단계 nested span, fixed px border/padding, line별 fragment와 Blink baseline metric quantization을 추가했다. C15.2b는 inline run별 font family/size/line-height와 direct Block child 주변 anonymous paragraph flow를 내부 Taffy geometry에 연결했고, focused 결과는 spinon-text-layout 17/17와 spinon-layout C15 16/16이다. [C15.2b 근거와 fixture 한계](internal/evidence/c15-2b-mixed-baseline-anonymous-flow-2026-10-17/README.md)에 기록한다. C15.2a/b는 layout geometry만 제공하고 runtime style projection, renderer/glyph paint, Android/iOS Simulator 앱 화면, overflow ink를 검증하지 않는다. C15 parent는 미완료다. [내부 계약 0070](internal/0070-c15-1-inline-text-layout.md) · [C15.1 실행 근거](internal/evidence/c15-1-inline-text-layout-2026-10-17/README.md) · [구현 실패 관점 20개](internal/evidence/c15-1-inline-text-layout-2026-10-17/implementation-review-20.md) · [plain span Chrome 관찰](internal/evidence/c15-inline-text-precomparison-2026-10-17/plain-inline-fragment-observation.md). C15.3는 Hangul·CJK strict line break·combining mark·ligature·Arabic/Bidi·emoji ZWJ, Block/Flex/Grid intrinsic sizing, overflow/paint-order/hit-test의 Chrome 155 DOM 사전 oracle을 추가했으며 actual glyph paint와 Android/iOS Simulator 앱 경로는 아직 확인하지 않았다. [C15.3 사전 비교와 한계](internal/evidence/c15-3-unicode-shaping-precomparison-2026-10-17/README.md) · [C15.3 계획 검토 20개](internal/evidence/c15-3-unicode-shaping-plan-review-20-2026-10-17.md). 다음 C15.3 제한 slice의 Chrome DOM 사전 기준은 `scrollLeft=3`, `scrollTop=18`, nested offset 합 `4+5`에서 text range 이동과 고정 scrollport clip을 기록하며, 구현 통과나 시뮬레이터 결과로 세지 않는다. [scroll-transform oracle와 해시](internal/evidence/c15-3-text-scroll-transform-precomparison-2026-10-17/README.md)

## 시나리오 카탈로그

첫 적합성 시나리오의 필드와 제안 목록은 [0009 적합성 시나리오 카탈로그](0009-conformance-scenarios.md)에 둔다. 이 버전의 일곱 시나리오는 모두 제안·미실행이며 공개 API의 지원 상태를 바꾸지 않는다. 플랫폼별 실행 근거가 갖춰지면 결과를 분리해 기록하고, `지원` 표기는 이 문서의 조건과 구현 상태 대장에 따라 별도로 판정한다.

## 적합성의 최소 단위

- **화면:** 텍스트 내용, 노드 순서, 크기·위치, 이미지 실패 표시와 접근성 이름을 확인한다. 웹과 모바일의 폰트 픽셀 차이는 별도로 기록한다.
- **입력:** 터치·키보드·보조 기술 활성화가 해당 이벤트를 내고, 제거된 핸들러가 다시 호출되지 않는지 확인한다.
- **비동기:** JS 호출, 호스트 콜백, 마이크로태스크, 커밋과 화면 표시의 순서를 구분한다.
- **수명:** 회전·백그라운드 복귀·표면 재생성 뒤 노드·콜백·GPU 자원이 일관된지 확인한다.
- **진단:** 미지원 태그·속성·CSS·API의 위치와 이유를 개발자가 알 수 있다.

## 미정

시각적 허용 오차, 브라우저 기본 스타일과의 차이 허용치, 공개 명세·번들의 정식 버전 정책, 플랫폼별 최소 OS 버전은 첫 실기기 수직 구현 전에 정한다. 내부 인터페이스의 미출시 숫자 버전은 [내부 계약 버전 규칙](internal/README.md)에 따른다. 이 값이 없으면 `100% 웹 호환` 또는 `성능 우위`를 주장하지 않는다.
