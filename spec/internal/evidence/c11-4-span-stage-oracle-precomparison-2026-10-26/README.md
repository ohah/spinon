# C11.4 span group 중간 track snapshot 독립 기대값

이 사전 비교 자료는 기존 Chrome 155 multi-span 입력에서 **중간 stage expected track state**를 독립 산술로 추가 고정한다. 규범 출처는 [CSS Grid Layout Module Level 1, 2025 CRD §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#extra-space)과 [§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grow-tracks)이다. 계획 작성 시 확인한 고정 CRD HTML SHA-256은 `53a47980a217f0b976e1ab8fa0b56944421f7e6311deef96a6aa591ddc693317`이다.

## 고정 입력

- 4개 column track은 모두 `minmax(0px, max-content)`, grid content width는 220px, gap은 0이다. 초기 base size는 `[0,0,0,0]`; 초기 max-content growth limit은 `[∞,∞,∞,∞]`이다.
- A: 80px definite-width item, tracks 1–2 (lines 1→3). B: 120px definite-width item, tracks 2–3 (lines 2→4). C: 200px definite-width item, tracks 1–3 (lines 1→4). item은 10px 높이이며 CSS margin/padding/border는 0이다.
- source order는 `A,B`, `B,A`, `A,B,C`, `C,B,A` control을 사용한다. 규범 track-sizing batch order는 `[span2,span3]`이고 source order와 독립이다.
- A/B의 span-2 범위는 한 track 겹친다. §11.5.1에서 같은 span group의 각 item은 자기 기여 크기와 기존 span track state로 필요한 증가를 구하고, 같은 track의 per-item planned increase는 최대값을 취해 group 끝에서 적용한다. span-3 group은 span-2 group이 적용된 뒤 계산한다.

## 규범 산술

| trace 지점 | span 기여 계산 | base sizes (px) | growth limits (px; `None`은 ∞) |
| --- | --- | --- | --- |
| `Initialize` | — | `[0,0,0,0]` | `[∞,∞,∞,∞]` |
| span-2 batch | A: `(80−0)/2 = 40`; B: `(120−0)/2 = 60`; 겹친 track 2는 `max(40,60)=60` | `[0,0,0,0]` | `[40,60,60,∞]` |
| span-3 batch | C: `200−(40+60+60)=40`; `40/3`씩 track 1–3에 계획 증가 | `[0,0,0,0]` | `[160/3,220/3,220/3,∞]` |
| `IntrinsicSizingComplete` | span에 걸리지 않은 track 4의 ∞를 base 0으로 닫음 | `[0,0,0,0]` | `[160/3,220/3,220/3,0]` |
| `MaximizeTracks` | definite 220px 안에서 finite growth limit까지 base를 증가 | `[160/3,220/3,220/3,0]` | `[160/3,220/3,220/3,0]` |

span-2 전용 controls는 마지막 span-3 row를 생략한다. 기대 최종 track `[40,60,60,0]px`, 나머지 공간 60px이다. span-2+span-3 controls는 normative final track `[160/3,220/3,220/3,0]px` (약 `[53.333…,73.333…,73.333…,0]px`), 나머지 공간 20px이다. 각 case assertion은 `Initialize` `[0,0,0,0]`/`[∞,∞,∞,∞]`부터 batch, `IntrinsicSizingComplete`, `MaximizeTracks`까지 확인한다. Taffy trace serializer는 non-finite growth limit을 `None`으로 바꾸므로 trace의 `None`은 기대 unbounded `+∞`의 표현이지만 내부 `+∞`, `-∞`, NaN을 구분하지 않는다. 이 slice는 해당 비유한 상태가 `None`으로 보존되는 것만 확인하며 실제 internal `+∞`의 증명으로 확대하지 않는다. finite 기대값은 actual `Some(finite)`의 절대차 `≤1e-4 CSS px`로 비교하고 `None`은 거부한다. 이 허용치는 기존 Chrome의 1/64px quantized final output과 별도로 둔다. 모든 기존 case는 `crosses_flexible_track=false`이며 flex interaction은 이 slice에서 생성하거나 추론하지 않는다. 이 계산은 4개의 max-content maximum track만 사용한다. finite/fixed upper-limit clamp, fit-content, gap, automatic minimum, row-axis 및 일반 track grammar로 일반화하지 않는다.

[`derive_expected.py`](./derive_expected.py)는 위 4 source-order controls와 planned increase를 `Fraction`으로 계산해 [`expected.json`](./expected.json)을 만든다. 이 계산은 해당 수치와 고정 span에만 한정된 validation script다. Taffy trace를 산술 입력으로 사용하지 않으며, Trace implementation의 correctness를 스크립트 실행만으로 주장하지 않는다.

## Chrome final oracle의 별도 역할

기존 [Chrome 155 observation](../c11-4-multispan-ordering-physical-android-2026-10-21/chrome155-observation.json)과 [입력·판정 기록](../c11-4-multispan-ordering-physical-android-2026-10-21/README.md)은 **최종 used columns 및 item frame**을 고정한다. Chrome의 1/64px used-track serialization은 span-2 controls에서 `[40,60,60,0]px`, span-2+span-3 controls에서 `[53.328125,73.328125,73.34375,0]px`이다. span-2+span-3의 규범상 ideal `[53⅓,73⅓,73⅓,0]px`와 Chrome quantized output은 구분한다. Chrome은 Taffy 내부 `IntrinsicItemBatch` snapshots를 노출하지 않으므로 이 최종 observation을 중간 snapshot expected로 재사용하지 않는다.

새 계획 test는 기존 네 입력을 사용해 batch 직후의 base/growth arrays를 위 규범 산술과 대조하고, final Chrome values는 기존 assertion대로 유지한다. 새 browser capture는 필요하지 않으며 이번 자료는 새 Chrome observation이나 Spinon 실행 증거가 아니다.

## 실행 상태와 한계

`python3 derive_expected.py > expected.json`을 실행해 exact rational JSON을 생성했다. Rust test, browser, simulator, app server는 실행하지 않았다. 실제 Spinon/Taffy stage trace가 이 중간값과 일치하는지, 수치 차이가 나오면 어느 알고리즘 단계가 원인인지는 구현 단계의 검증 관문으로 남는다.
