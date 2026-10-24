# C11.4 fixed-repeat + auto-repeat 사전 비교

**상태:** 표준 규칙·독립 기대값·Chrome DOM 관찰과 Stylo computed-AST 정규화를 기록했다. 기존 cascade adapter는 여전히 integer-repeat 혼합을 거부한다. 새 mixed implementation, layout/runtime, Android/iOS Simulator, WPT는 실행하지 않았다. C11.4 완료 상태와 공식 완료 수 `72/213`은 바뀌지 않는다.

## 규범 경계

- 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)이다.
- §7.2.3.1은 `auto-repeat`이 한 track list에서 한 번만 나타날 수 있고, 같은 목록에 `<fixed-repeat>`도 둘 수 있다고 명시한다. `<fixed-repeat>`은 양의 정수 횟수와 `<fixed-size>` track 목록을 반복한다.
- §7.2.3.2에 따라 definite `width`에서 `auto-fill` 반복 수는 non-repeated track, repeated track의 definite minimum, 그리고 전체 gutter를 더해 content box를 넘지 않는 최대 양의 정수다. `auto-fit`은 이 반복 수를 먼저 정한 뒤 배치에서 비어 있는 repeated track을 접는다.
- 이 사전 비교는 `<fixed-repeat>` 하나, 두 개의 nonnegative computed absolute-length track size, `repeat(2, 20px 10px)`을 쓴다. Stylo computed AST는 `calc(20px)`를 `Length(20px)`로 정규화하며 property serialization도 `20px`를 낸다. 그러므로 mixed adapter 경계는 authored spelling/unit이 아니라 finite nonnegative `LengthPx`다. Chrome scene `fill-prefix-calc-constant`는 literal px control과 동일한 used tracks를 확인한다. auto-repeat은 기존 제한 입력인 `minmax(32px, 1fr)`다. Percentage/unresolved `Calc`, 이름 붙은 line, area, 두 번째 fixed-repeat, 복수 auto-repeat, flex/intrinsic fixed-repeat breadth는 별도 범위다.

## 독립 기대값

columns 장면은 definite content width `200px`, rows 장면은 definite content height `200px`이며 해당 축의 gap은 `8px`다. 정수 repeat은 `20px 10px` 두 track으로 구성되고 횟수 2이므로 non-repeated 합은 `20 + 10 + 20 + 10 + 16 = 76px`이며 track 수는 5개다. auto-repeat의 최소 breadth는 `32px`이다.

반복 횟수 `n`에서 전체 사용량 하한은 `76 + 32n + 8(4+n) = 108 + 40n`이다. `n=2`는 `188px`이고 `n=3`은 `228px`이므로 `auto-fill`과 collapse 전 `auto-fit` 모두 반복 track 두 개를 만든다. `auto-fill`의 남은 `12px`는 두 `1fr` track에 나뉘어 각각 `38px`가 된다.

빈 `auto-fit`은 두 repeated track을 `0px`로 접는다. 첫 repeated track만 점유한 장면에서는 두 번째 track만 접히며, repeated range의 시작·끝 gutter는 유지된다. 이 경우 내부 fixed-repeat gutter 3개와 repeated range 양쪽 gutter 2개가 남아 `20 + 10 + 20 + 10 + 84 + 0 + 16 + 5×8 = 200px`가 된다.

## Chrome 156 관찰

- 실행 브라우저: Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, macOS `26.5.1` ARM64, viewport `1280×633` CSS px, DPR 1.
- 고정 입력: [grid-auto-repeat-fixed-repeat-c114-precomparison.html](../../../../tests/fixtures/css/c11/grid-auto-repeat-fixed-repeat-c114-precomparison.html), SHA-256은 [SHA256SUMS](./SHA256SUMS)에서 확인한다.
- Stylo probe: [computed AST와 property serialization](./stylo-computed-ast-observation.json). 이 관찰은 현재 adapter의 `RepeatCount::Number` 거부 이유를 확인하며 mixed feature를 구현하지 않는다.
- 전체 관찰: [chrome156-observation.json](./chrome156-observation.json). 원본 DOM screenshot: [chrome156-fixed-repeat-auto-repeat.png](./chrome156-fixed-repeat-auto-repeat.png), SHA-256 `4ef459d0c585668611103ed2cccd5310ee2dec982f7960adedbe083e5270f378`.
- 모든 입력·관찰·계획·상태·로드맵 파일의 저장소-root 기준 hash는 [SHA256SUMS](./SHA256SUMS)에 있다. 저장소 루트에서 `shasum -a 256 -c spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/SHA256SUMS`로 확인한다.

| Case | fixed-repeat 위치 | auto-repeat | Chrome used columns | item |
| --- | --- | --- | --- | --- |
| `fill-prefix-empty` | prefix | `auto-fill` | `20 10 20 10 38 38 16px` | 없음 |
| `fill-suffix-empty` | suffix | `auto-fill` | `16 38 38 20 10 20 10px` | 없음 |
| `fill-prefix-calc-constant` | prefix, `calc(20px)` | `auto-fill` | `20 10 20 10 38 38 16px` | literal `20px` control과 동일 |
| `fit-prefix-empty` | prefix | `auto-fit` | `20 10 20 10 0 0 16px` | 없음 |
| `fit-prefix-occupied` | prefix | `auto-fit` | `20 10 20 10 84 0 16px` | 첫 repeated track: root-relative `x=92px`, `10×10px` |
| `row-fill-prefix-empty` | rows prefix | `auto-fill` | `20 10 20 10 38 38 16px` | 없음 |
| `row-fill-suffix-empty` | rows suffix | `auto-fill` | `16 38 38 20 10 20 10px` | 없음 |
| `row-fit-prefix-empty` | rows prefix | `auto-fit` | `20 10 20 10 0 0 16px` | 없음 |
| `row-fit-prefix-occupied` | rows prefix | `auto-fit` | `20 10 20 10 84 0 16px` | 첫 repeated track: root-relative `y=92px`, `10×10px` |

Browser track serialization is compared within `1/64` CSS px; case order and integer geometry are exact. These Chrome observations corroborate the separate spec-derived calculation on each axis; they are not Spinon results or WPT conformance.

## 범위 밖

- Spinon의 integer-repeat + auto-repeat cascade admission은 계속 미지원이다. `calc(20px)`는 사전 Chrome scene과 Stylo computed-AST observation에만 있다. 이 자료는 계획의 oracle일 뿐 지원을 선언하지 않는다.
- CSS cascade fallback, malformed/unsupported values, 10,000-track cap, allocation failure, line-name merge/collision, rows, named areas, placement combinations, arbitrary CSS admission, WPT, Android/iOS 앱 실행, 실기기 및 GPU surface는 검증하지 않았다.
- Chrome output만으로 normative rule을 정하지 않는다. 구현 수락은 이후 계획의 제한된 profile, 독립 track/gap 산식, 정확한 error/no-partial-output 경계, 앱 fixture trace를 함께 판정한다.
