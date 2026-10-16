# C14.3 · 이미지 intrinsic contribution과 percentage resolution

## 목표와 범위

HTML namespace `img` leaf의 decoded natural metadata를 사용해 CSS `min-content`, `max-content`, `fit-content` sizing keyword와 containing block percentage를 제한된 Block·Flex·Grid item context에서 계산한다. 현재 C14.1/C14.2 코드가 이미 가진 자연 크기·비율·revision snapshot을 입력으로 사용한다. 내부 candidate이며 공개 CSS 지원을 선언하지 않는다.

이번 단계는 non-text replaced element만 다룬다. 텍스트 노드·글꼴·버튼 intrinsic size는 C14.4 이후에 둔다. C14.1 resource loader, async replacement schedule, GPU image paint, clip/overflow pixel은 이 단계에서 추가하지 않는다. C14.1과 C14.2, 상위 C14/U05/S02/X16도 완료로 표시하지 않는다.

## 기준 명세

- [CSS Box Sizing Module Level 3](https://www.w3.org/TR/css-sizing-3/) §2.1–2.3: min-content/max-content 및 fit-content 공식, replaced element natural dimensions, intrinsic contributions.
- 같은 명세 §3.2, §5.2.1: sizing keyword와 containing-block percentage, cyclic percentage의 intrinsic contribution 규칙. `width:fit-content` keyword와 `fit-content(<length-percentage>)` 함수는 구분한다.
- [CSS Flexible Box Layout Module Level 1](https://www.w3.org/TR/css-flexbox-1/) §4.5, §9.9.3: replaced flex item의 content-based minimum 및 item contribution 경계.
- [CSS Grid Layout Module Level 1](https://www.w3.org/TR/css-grid-1/) §7.2.1: 이번 단계에서 사용하는 fixed track의 definite size 문맥. intrinsic track sizing은 범위에서 제외한다.
- 사전 비교는 agent-browser가 실행한 Google Chrome `155.0.8059.85` / macOS `26.5.1`이다. fixture의 reduced UA는 `HeadlessChrome/155.0.0.0`; 같은 탭의 high-entropy UA는 전체 버전과 플랫폼 버전을 반환한다. viewport `1280×633 CSS px`, DPR `1`이다. 이 결과를 다른 브라우저 엔진 또는 OS로 일반화하지 않는다.

## 고정 입력과 비교 결과

- Fixture: [`intrinsic-contributions.html`](../tests/fixtures/css/c14/intrinsic-contributions.html).
- PNG: `tests/fixtures/resources/c14-1-png-basic.png`, 137 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`, natural size `24×16 CSS px`.
- 원본 observations: [Chrome 155 JSON](../spec/internal/evidence/c14-3-precomparison-2026-10-16/chrome155-observation.json) · [화면](../spec/internal/evidence/c14-3-precomparison-2026-10-16/chrome155.png) · [14개 고정 기대값](../tests/fixtures/css/references/c14-3-image-intrinsic-contributions-v1.json) · [측정 방법과 선택 결과](../spec/internal/evidence/c14-3-precomparison-2026-10-16/README.md).
- 선택한 CSS px oracle: HTML image `width:min-content`와 `width:max-content`는 각각 `24×16`; image 자체가 `box-sizing:border-box; padding:4px; border:1px solid`인 `width:min-content`는 natural content size에 edge를 더한 `34×26`; 12px content basis의 `fit-content`는 min-content 하한 때문에 `24×16`이며 content area보다 12 CSS px 넓고 부모 border box 오른쪽으로 7 CSS px 나간다; 같은 keyword에 `max-width:12px`를 추가하면 `12×8`; definite 200px parent의 `width:50%`는 `100×66.65625`; definite 120px parent의 `height:50%`는 `90×60`. tolerance는 각 frame field당 `0.05 CSS px`이며 DPR 변환 전 좌표를 비교한다.
- 동일한 `min-content`, 50% width, 50% height case를 definite-size Flex container와 fixed-track Grid item에서도 수집한다. Block·Flex·Grid의 컨테이너 content size와 image frame을 각각 비교한다.
- cyclic percentage control은 min-content width를 계산하는 containing block 안의 `img { width:50%; height:auto }`다. 고정 Chrome 결과는 parent basis와 image frame 모두 `0×0`이다. 계산 중 zero basis와 최종 definite basis를 혼동해 12px 또는 natural `24px`로 대체하면 실패한다.
- 초기 auto-height probe는 oracle에서 제외했다. 부모가 auto-sized outer Grid row의 item이어서 Chrome의 `75.75px` 결과에 percentage resolution과 Grid row sizing이 섞였다. 이 값은 indefinite block-axis 규칙의 근거로 쓰지 않는다.
- 브라우저 oracle screenshot의 `.case-label`은 case 식별용 주석이다. Android/iOS opt-in fixture는 14개 case ID와 sizing 선언을 유지하되 text node는 빼서 C14.4 shaping/font metric이 C14.3 geometry를 바꾸지 않게 한다. CSS px로 image와 containing block frame을 비교하고 `fit-content-small-basis`에서 image 오른쪽이 부모 border-box보다 7px 나가는지, image 자체의 border-box `min-content`가 `34×26 CSS px`인지 확인한다. 이는 layout frame 검사다. C14.3에는 image painter가 없으므로 native image pixel의 border 바깥 페인트·clip 여부를 입증하지 않는다.

## 구현 전 통과 기준

1. HTML `img`만 기존 immutable `LayoutIntrinsicSizeSnapshot`을 소비한다. source/generation/document/render/style/environment/intrinsic revision이 일치하고 node가 연결된 leaf여야 한다.
2. `min-content`, `max-content`, `fit-content` keyword의 CSS px used size가 고정 oracle과 일치한다. `fit-content`는 min/max constraint와 content/border-box edge를 한 번만 반영한다. `fit-content()` 함수는 `LayoutError::UnsupportedIntrinsicSizing { node, .. }`로 거부하며 `auto`로 바꾸지 않는다.
3. width/height percentage는 definite containing-block content axis에 대해 각각 해석된다. 다른 축이 `auto`이면 C14.2의 natural ratio solver를 사용한다. Block, definite single-line Flex item, definite fixed-track Grid item을 분리해 관찰한다.
4. `min-content` parent의 cyclic image percentage는 sizing 단계에서 CSS Sizing 3 §5.2.1의 replaced-element rule을 따른다. cyclic min-content contribution과 최종 item layout을 다른 pass/source로 섞지 않는다.
5. 구현 전 oracle의 14개 sizing case만 완료 판정 집합으로 삼는다. 기존 C14.1/C14.2 regression tests가 source/generation/revision, natural ratio, box-sizing, min/max 입력을 계속 소유한다. C14.3 추가 negative control은 `fit-content(<length-percentage>)` 미지원 구문을 해당 node의 `LayoutError::UnsupportedIntrinsicSizing`으로 거부하는 경로에 한정하며 `auto`/0 fallback은 허용하지 않는다. 각 결과에는 case ID, CSS property, containing-block content size, image frame과 오류를 기록한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 opt-in app route에서 text-free live V8-authored HostDocument로 14개 case를 실행한다. width/height와 basis를 비교하고 `fit-content-small-basis`의 border 밖 7 CSS px frame overflow와 image-own border-box `min-content`의 `34×26 CSS px` frame을 양쪽 report에서 확인한다. 원본 report log와 screen capture를 보존한다. 이 확인은 layout frame geometry이며 native image pixel paint/clipping은 후속 image-paint 범위다. 실기기 검증으로 표현하지 않는다.
7. plan의 표와 다른 20개 구현 실패 관점, PR diff에 대한 별도 20개 검토에서 미해결 P1/P2가 없다. 필요한 CI와 두 simulator 결과를 PR merge 직전에 다시 확인한다.

## 명시적 비지원

- text/shaping, button·form·video·canvas·SVG image, object-fit/object-position, image decode/network/cache/replacement scheduling, image pixels/paint/clip은 제외한다.
- Grid의 intrinsic `min-content`/`max-content` track sizing, auto-repeat, multi-track spanning contribution, auto-sized cyclic track percentage는 제외한다. 이 단계의 Grid case는 definite fixed tracks 안 item의 sizing만 검사한다.
- indefinite percentage axes, nested/orthogonal writing mode, calc()/negative percentage, full WPT, 전체 C14/CSS 지원은 제외한다. unsupported style profile은 조용히 `auto`나 0으로 바꾸지 않고 식별 가능한 오류로 닫는다.

## 구현 순서

1. [계획 실패 관점 20개](../spec/internal/evidence/c14-3-image-intrinsic-contributions-plan-review-20-2026-10-16.md)의 P1/P2를 먼저 해결하고 기준 HTML/reference hashes를 고정한다.
2. `LayoutDimension`, Stylo projection/preflight 및 C14.3 image solver의 keyword/percentage 입력 모델을 확장한다. 기존 C14.1/C14.2 path는 fixture별 regression control로 둔다.
3. Taffy 0.14.0이 주는 available-size/containing-block basis와 replaced sizing 결과를 비교한다. CSS ratio solver와 Taffy ratio 재보정이 같은 축을 두 번 바꾸지 않게 한다.
4. unit/style-to-layout와 Chrome 155 CSS-reference tests를 추가하고, `fit-content()` unsupported profile과 cyclic percentage 경계가 명시적으로 거부/해결되는지 adversarially 주입한다. C14.1의 stale snapshot·source mismatch 회귀는 기존 소유 테스트로 확인한다.
5. Android/iOS opt-in runtime fixture route에서 같은 V8 DOM matrix를 실행해 basis, frame, revision, error, report buffer 및 screen evidence를 수집한다. default artifact에서 fixture ABI/payload가 빠지고 opt-in positive control에만 있는지 확인한다.
6. 계획과 다른 20개 구현 실패 관점으로 검토해 결함을 고치고 영향 관점을 다시 실행한다. spec/internal/0009, 부모 C14 계획, 상태 대장과 evidence index를 함께 갱신하되 C14 완료 체크는 하지 않는다.
7. 별도 PR 변경 실패 관점 20개를 기록하고, screenshots/oracle/실행 한계, labels와 Tailnet 후속을 PR 본문에 연결한다. 필수 checks/mergeability가 허용하면 `Rebase and merge`한다.

## 2026-10-16 구현 후보 결과

- 제한된 `img` sizing, 14-case Chrome 155 oracle, AVD와 iOS Simulator live V8 route, 기본 artifact exclusion/opt-in positive control을 구현했다. border-box `min-content`의 `34×26 CSS px`, 작은 basis 밖 7px frame overrun, cyclic percentage의 `0×0`을 양쪽 report에서 확인했다.
- workspace test는 `873 passed`, `4 ignored`, `0 failed`; Clippy·rustfmt·diff·native project/shell·fixture boundary 검사를 통과했다. 구현 실패 관점과 PR 변경을 각각 별도 20개로 검토했고 발견 사항은 수정했다. 상세 원본은 [실행 근거](../spec/internal/evidence/c14-3-image-intrinsic-contributions-runtime-2026-10-16/README.md)에 있다.
- 구현 PR #216의 required checks와 merge가 남았다. 이 항목이 main에 반영되면 체크 상태는 65/198에서 66/198로 바뀐다. Android 실기기, iOS 실기기, image pixel paint/clip, 전체 WPT/C14 지원은 이 결과에 포함하지 않는다.
