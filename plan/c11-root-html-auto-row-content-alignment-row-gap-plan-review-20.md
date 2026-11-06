# C11.4 implicit auto-row alignment + row-gap 계획 실패 관점 검토 20개

검토 대상: [고정 row-gap alignment 계획](./c11-root-html-auto-row-content-alignment-row-gap.md), [HeadlessChrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-gap-precomparison-2026-11-07/README.md). 구현 코드는 아직 변경하지 않았다.

| # | 서로 다른 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | Grid gutter가 첫/마지막 track 바깥에도 생겨 gap 총합을 잘못 계산하는가 | 두 인접 track 사이 두 gutter만 `2g`로 계산했다. |
| 2 | `row-gap`이 track base size와 별개로 free-space에서 빠지는가 | `40 - 30 - 2g` 공식을 네 gap 기준점에 사용했다. |
| 3 | `normal|stretch`가 explicit row까지 stretch할 수 있는가 | 두 implicit auto rows만 남은 positive space를 나눠 갖는다. |
| 4 | `align-items:normal`이 direct text의 최종 frame 높이에 주는 영향이 누락되는가 | Text item block-size를 final auto row 크기와 대조한다. |
| 5 | `g=0`에서 선행 PR #1237의 frames가 바뀌는가 | zero-gap baseline 10개 값을 별도 보존한다. |
| 6 | 중간 소수 gutter 때문에 browser quantization이 정확한 수식처럼 기록되는가 | 독립 기대값과 raw observation을 분리하고 1/32 CSS px 허용차를 명시했다. |
| 7 | `g=5`에서 free space zero인 equality 경계가 빠지는가 | 네 번째 free-space 집합에 0px boundary를 넣었다. |
| 8 | `g=7.5`에서 negative free space가 0으로 clamp되는가 | -5px을 독립 입력으로 고정하고 overflow frame을 검사한다. |
| 9 | `end`의 unsafe overflow가 start로 fallback되어 의미가 바뀌는가 | 첫 item y=-5px를 유지하는 별도 검증을 요구한다. |
| 10 | `center`의 양쪽 overflow가 한쪽에만 나타나는가 | 첫 item y=-2.5px 및 마지막 row가 y=32.5px에서 끝나는 frame을 고정한다. |
| 11 | Negative free space의 `space-between` fallback이 positional `end`와 같아지는가 | Grid non-flex default fallback start를 `end`/`center`와 분리했다. |
| 12 | `space-around`·`space-evenly`가 negative free space에서 unsafe center로 남는가 | 기본 safe-center fallback에 따라 start overflow를 피하는 Chrome/spec 결과를 고정했다. |
| 13 | `flex-start`/`flex-end`가 Grid에서 start/end와 다른 geometry를 내는가 | 두 alias의 모든 gap 기대값을 각각 표기했다. |
| 14 | `row-gap` authored 문자열을 재파싱해 cascade computed value를 건너뛰는가 | Typed computed finite px만 admission 대상으로 제한했다. |
| 15 | 7.5px 초과 gap에서 negative overflow 한계를 일반 지원으로 잘못 확대하는가 | 상한을 포함하고 초과는 property/node diagnostic과 atomic rejection 대상으로 둔다. |
| 16 | percentage·math row-gap이 px로 변환된다고 가정하는가 | 이 slice는 typed `LengthPx`만 허용하고 percentage/CSS math를 제외했다. |
| 17 | non-zero column gap이 row-only 계획에서 우회 허용되는가 | Column gap 0을 별도 정확 조건으로 유지한다. |
| 18 | `place-content` shorthand가 `justify-content` 축도 몰래 넓히는가 | computed longhands를 각각 검증하고 non-initial inline axis를 계속 거부한다. |
| 19 | `row-gap`만 바뀐 source에서 owner, direct children, revisions가 달라도 성공하는가 | Root owner, exact topology, source/style/layout revisions와 failure atomicity를 유지한다. |
| 20 | Agent Browser·simulator 화면을 서로 같은 pixel evidence로 합치는가 | Chrome oracle, RuntimeSession frame, Android PixelCopy, iOS Metal drawable을 분리 보고한다. |

## 결론

계획은 현재 `RootHtmlAutoRowStretchV1` 범위의 typed gap gate와 Taffy content distribution을 좁혀 재사용한다. Negative-free-space fallback과 top-edge overflow를 실제 runtime frame 및 가능한 Android/iOS surface evidence까지 검증하도록 acceptance criteria에 넣었다. C11.4/C11 공식 상태는 이 계획으로 바뀌지 않는다.
