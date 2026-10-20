# C11.4 · definite minimum-size에서 축별 `auto-fit` 반복 수 계획

**상위 원본:** [C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing 계획](./c11-4-grid-track-sizing.md) · [CSS Grid Level 1 §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획과 Chrome 155 사전 관찰을 고정했다. 계획 검토 20개를 마쳤으며, 구현 검토는 구현 변경 뒤 별도 수행한다. 이 계획은 C11.4 체크와 공식 완료 수를 바꾸지 않는다.

**계획 PR diff 검토:** [독립 30개 관점 기록](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/pr-diff-review-30.md).

## 목적

C11.4의 제한 auto-repeat 경로에서 definite minimum-size만 반복 수의 기준으로 쓰는 `auto-fit`을 columns와 rows에 적용했을 때, 반복 횟수가 같은 CSS 입력의 `auto-fill`과 일치하고 빈 반복 track만 item 배치 뒤 접히는지 확인한다. 현재 Chrome 155 관찰은 columns와 rows에서 서로 다른 반복 수를 반환해 규범과 충돌한다. 원인을 확인하기 전에 Chrome 결과를 구현 기대값으로 복사하지 않는다.

## 확인된 현상

[Chrome 155 사전 관찰](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/README.md)은 같은 fixture의 `minmax(40px, 1fr) 20px`, 8px gap, `20px` prefix·`10px` suffix, max-content preferred size와 200px minimum-size를 비교한다.

- `auto-fill`은 columns·rows 양쪽에서 반복 pattern 3회와 266px intrinsic used size를 반환한다.
- 빈 `auto-fit`은 rows에서 3회를 직렬화하지만 columns에서는 2회를 반환한다. 두 축 모두 최종 root size는 200px이고 빈 repeated track은 0px로 직렬화된다.
- 차이는 고정 `40px 20px` pattern 대조에서도 재현된다. item 한 개 또는 두 track을 span하는 item을 놓아도 축별 count 차이가 남는다.
- `190px`과 `342px` 경계에서는 columns·rows 모두 각각 2회와 4회다. `200px` definite preferred-size 대조에서는 두 축의 `auto-fill`·`auto-fit` 모두 2회다.
- `writing-mode: vertical-rl`에서는 columns가 물리 세로축으로, rows가 물리 가로축으로 움직여도 차이가 columns/rows에 그대로 붙는다. 이는 차이가 단순한 물리 width/height 지정이 아니라 논리 Grid 축의 intrinsic-size 경로와 관련됐을 가능성을 보여주지만, Blink 내부 sizing 입력을 기록하지 않았으므로 원인은 미확정이다.

재현 fixture는 [grid-auto-fit-min-size-axis-c114.html](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/tests/fixtures/css/c11/grid-auto-fit-min-size-axis-c114.html)이고 30개 입력의 관찰값·resolved track list는 [chrome155-observation.json](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-observation.json)에 둔다. [화면](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-auto-fit-axis.png)은 Chrome reference 출력이며 Spinon 동작 증거가 아니다. Chromium의 `ComputeAutomaticRepetitions` 구현은 columns/rows의 available, minimum, maximum size를 별도 전달받아 반복 수를 계산한다. 해당 함수 호출 인자의 실제 값을 캡처하지 않았으므로 이것만으로 root cause를 단정하지 않는다.

## 규범 기대와 판정 기준

§7.2.3.2는 `auto-fit`의 반복 횟수를 `auto-fill`과 같게 하고, item placement 뒤 빈 repeated track과 양옆 gutter를 접는다. 이번 계획의 기준은 이 규범 문장과 아래에서 고정한 입력 계산이다. Chrome 155는 독립 관찰기이며 상호 운용 차이를 기록하는 용도다.

`minmax(40px, 1fr) 20px` fragment의 반복 수 산정 기여는 `40px + 20px`이고, 8px gap과 explicit neighbor까지 포함한 최소 사용량은 다음과 같다.

| Repeat 밖 고정 track | 최소 사용량 | 190px minimum | 200px minimum | 342px minimum |
| --- | --- | ---: | ---: | ---: |
| `20px` prefix + `10px` suffix | `38 + 76n` | 2회 | 3회 | 4회 |
| `20px` prefix만 | `20 + 76n` | — | 3회 | — |
| `10px` suffix만 | `10 + 76n` | — | 3회 | — |
| neighbor 없음 | `68n - 8` | — | 4회 | — |

두 fixed neighbor가 있는 definite 200px preferred-size 대조는 content box를 넘지 않는 최대 양의 반복 수를 선택해 2회가 되어야 한다. 이는 minimum-size 경로의 3회와 구별한다.

`auto-fit` 결과 판정은 반복 수와 collapse를 분리한다.

1. 같은 axis·sizing basis·track list의 `auto-fill`과 `auto-fit`은 placement 전 반복 track 수가 같다.
2. 빈 `auto-fit`은 모든 repeated track을 접고 repeat 밖의 explicit track은 보존한다.
3. 첫 repeated track에 놓인 in-flow item은 그 track만 비어 있지 않게 한다. 두 repeated track을 span하는 in-flow item은 두 track을 점유 상태로 남긴다. out-of-flow item은 track을 점유한 것으로 계산하지 않는다. 남은 빈 반복 track과 해당 gutter만 접는다.
4. `auto-fill` 반복 track은 item 부재만으로 접히지 않는다.
5. Chrome 관찰과 규범 기대가 계속 다르면 둘을 별도 열로 보고한다. 제품 acceptance는 규범 계산과 명시된 제한 subset을 기준으로 하고, Chrome 호환 완료나 WPT 통과로 표현하지 않는다.

## 사전 oracle과 실행 행렬

- **Chrome 입력:** 30개 고정 case. `auto-fill`/`auto-fit`, columns/rows, 190/200/342px minimum boundary, empty/in-flow single/span-two/out-of-flow placement, `minmax(40px,1fr) 20px`와 fixed `40px 20px`, prefix/suffix/neighbor 없음, 200px definite preferred-size 및 vertical writing-mode 진단 control을 포함한다.
- **환경:** Chrome `155.0.8059.85` arm64, macOS `26.5.1 (25F80)`, Agent Browser `0.31.1`, viewport `1600×1500` CSS px, DPR 1. observation JSON에 user agent와 모든 case의 입력·computed track·root/item rect를 보존한다.
- **독립 규범:** [CSS Grid Level 1 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation). Chromium source는 진단 참고이며 규범 기대값을 대체하지 않는다.
- **정밀도:** repeat count, track 순서, collapse 상태와 integer px geometry는 정확히 일치해야 한다. fractional used size가 생기는 case만 1/64 CSS px 이내를 허용한다.
- **실패:** auto-fit이 auto-fill과 다른 count를 갖는 것, 반복 fragment 밖 track이 repeat count나 collapse에 잘못 포함되는 것, placement 후 occupied track이 접히거나 empty track이 남는 것, collapse된 gutter가 살아남는 것, trace 누락/축 교환, partial layout을 성공으로 반환하는 것을 각각 실패로 판정한다.

## 구현 범위와 절차

1. 계획 PR을 병합하기 전에 계획 자체를 20개의 독립 실패 관점으로 검토한다. [계획 검토](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/plan-review-20.md)를 구현 검토와 섞지 않는다.
2. 구현 전 고정 fixture를 Taffy `0.15.0` auto-repeat expansion·repeat range·auto-fit collapse와 대조한다. 기존 layout input에서 definite minimum basis, explicit neighbors, row/column axis 사용값을 확인하고, Chrome과 다르면 입력 크기 및 repeat-count 산출을 별도로 기록한다.
3. Rust layout·style projection 시험을 추가해 columns·rows 양쪽의 expected repeat count, half-open range, fixed neighbor 순서, placement occupancy, collapse/gutter 불변식을 확인한다. 코드가 이미 규범대로 동작하면 제품 경로를 불필요하게 바꾸지 않고 앱 fixture와 증거를 보강한다. 불일치가 재현되면 책임 있는 projection/layout 단계만 고친다.
4. opt-in app fixture에서 minimum-size 190/200/342 경계, prefix/suffix/both/neighbors 없음, empty/single/span-two, fixed-pattern control과 definite-size positive control을 검증한다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 root/item frame, 모든 used track, active/collapsed gutter, axis trace, trace chunk completeness와 draw/present를 확인한다. 실기기 연결은 하지 않는다.
5. Rust workspace·Clippy·rustfmt와 기본/fixture FFI artifact 경계를 검증한다. 이 fixture는 검증 전용 opt-in 경로이며 공개 API가 아니다.
6. 코드나 runtime 변경 뒤에는 새 코드·동작 관점 20개, 최종 PR diff에는 별도의 독립 관점 20개를 수행한다. 계획 검토를 재사용하지 않는다. 결함을 찾으면 수정 후 영향을 받은 관점을 다시 실행한다.
7. PR 병합 전 현재 required checks와 merge state를 조회한다. required check가 설정되지 않았다면 그 사실을 그대로 기록하고 CI 통과로 표현하지 않는다. 병합은 Rebase and merge를 사용한다.

## 범위 밖과 상태

현재 계획은 horizontal-tb의 제한 fixed-neighbor auto-repeat profile을 구현 판정 대상으로 둔다. Vertical writing-mode case는 원인 분리용 Chrome 진단이며 runtime acceptance나 지원 선언이 아니다. 한 list 안의 복수 repeat, repeated line names, template areas, arbitrary/non-px track grammar, general auto-placement, 10,000-track 전체 경계, WPT 적합성, physical device/hardware GPU, iOS compositor-after pixel equality는 이 계획으로 닫지 않는다.

새 공개 API는 없다. 내부 계약은 `0059` `0.1.0`을 유지한다. C11.4와 C11 상위 체크 및 공식 완료 수는 구현 증거와 별도 적합성 관문이 충족되기 전까지 미완료로 둔다.
