# C11.1 percentage implicit columns 계획 적대 검토 20개

기준 계획: [`c11-root-html-grid-column-flow-percent-implicit-columns.md`](./c11-root-html-grid-column-flow-percent-implicit-columns.md)

이 검토는 계획 진입 판단이다. implementation/runtime 검증 또는 PR diff 검토로 재사용하지 않는다. **통과 20, blocker 0, 계획 보완 0**이다. 다만 코드 동작은 구현 뒤 별도 검증하기 전까지 미확인이다.

| # | 적대 관점 | 계획 판정 | 확인 근거 |
|---:|---|---|---|
| 1 | `grid-auto-columns` 목록이 여러 implicit track에 반복되는 규범 범위를 explicit track list와 혼동하지 않는가 | 통과 | §7.6 implicit sizing과 §7.2 explicit sizing을 각각 명시했다 |
| 2 | percentage basis를 available leftover width 또는 remaining space로 잘못 잡지 않는가 | 통과 | definite 240px content-box width를 basis로 고정하고 `[24,36]px`로 계산했다 |
| 3 | `column-gap`을 percentage basis에서 빼는가 | 통과 | 240×10%, 240×15%를 gap과 분리하고 gap은 x 시작점에만 더한다 |
| 4 | 목록의 첫 implicit track에서 pattern index가 0으로 시작하는가 | 통과 | explicit prefix `[30,40]` 이후 `[24,36,24,36]`을 기대값으로 고정했다 |
| 5 | 목록이 단 한 번만 적용되고 네 implicit column에서 순환을 놓치는가 | 통과 | 12개 item·2개 explicit row로 네 implicit column을 요구한다 |
| 6 | px/percentage mixed list의 단위 순서가 바뀌거나 전체 목록이 한 값으로 축약되는가 | 통과 | `10px 15%` 대조의 implicit tracks `[10,36,10,36]`을 고정했다 |
| 7 | scalar percentage와 단일 px compatibility가 서로 다른 표현에서 빠지는가 | 통과 | `10%` scalar case와 기존 px regression을 각각 요구한다 |
| 8 | 0% track을 무효/auto로 바꾸거나 item의 visible overflow를 잃는가 | 통과 | zero track, 고정 item frame 및 overflow 경계를 별도 case로 뒀다 |
| 9 | 100% 초과 percentage를 100%로 clamp하거나 root 폭으로 제한하는가 | 통과 | `120% → 288px`와 track starts `373,666,959`를 독립값으로 고정했다 |
| 10 | definite zero width를 indefinite width와 같은 방식으로 거부하는가 | 통과 | `0px`는 valid basis, resolved percentage는 0px인 별도 case를 뒀다 |
| 11 | column-major placement/source order가 track 크기 비교와 섞여 오판되는가 | 통과 | 12 semantic item frame을 track list 및 row positions와 별도로 기록한다 |
| 12 | implicit sizing 확대가 explicit columns/rows 또는 fixed-px auto list를 바꾸는가 | 통과 | fixed-px precedents와 explicit prefix를 회귀 항목으로 유지한다 |
| 13 | indefinite/root width를 computed fixed width로 오인하는가 | 통과 | computed direct finite non-negative px width를 요구하고 0px만 허용한다 |
| 14 | min/max width, aspect-ratio, math-backed width가 percentage basis를 변경하는가 | 통과 | 해당 조건을 각각 unsupported control로 명시했다 |
| 15 | CSS syntax-invalid 선언 fallback과 syntax-valid profile 거부를 같은 오류로 다루는가 | 통과 | cascade fallback과 typed-but-unsupported 사례를 분리 검증한다 |
| 16 | synthetic negative, NaN, infinity percentage가 브라우저 입력에서는 재현되지 않아 누락되는가 | 통과 | browser positive와 별개로 typed model을 직접 만드는 projection/layout negative를 요구한다 |
| 17 | resolved track, 반복 수, gap 누계가 f32 범위를 넘거나 quota보다 먼저 큰 allocation을 하는가 | 통과 | 10,000 track, multiplication 및 aggregate extent의 preflight/rejection을 수락 기준에 넣었다 |
| 18 | placement reservation/track expansion 실패가 partial frame이나 render scene을 공개하는가 | 통과 | atomic no-publication와 다음 valid request recovery를 요구한다 |
| 19 | desktop Chrome observation만으로 Android/iOS runtime parity를 과장하는가 | 통과 | 두 mobile simulator에서 동일 fixture hash/runtime geometry를 따로 확인하고 screenshot/readback 범위를 분리했다 |
| 20 | 제한 fixture를 일반 CSS support 또는 C11 완료로 과장하는가 | 통과 | product admission, WPT, 전체 C11.1/C11과 `72/213` 변경을 명시적으로 제외했다 |

## 구현 전제와 유보

- Chrome 156 observation은 [raw JSON](../spec/internal/evidence/c11-root-html-grid-column-flow-percent-auto-columns-precomparison-2026-11-02/chrome156-observation.json)로 고정했고, 별도 arithmetic은 [expected.json](../spec/internal/evidence/c11-root-html-grid-column-flow-percent-auto-columns-precomparison-2026-11-02/expected.json)에 기록했다.
- percentage implicit columns가 Taffy expansion과 aggregate overflow preflight에서 예상대로 동작하는지는 구현 전에는 증명되지 않았다. 해당 부분은 runtime code와 tests에서 실패하면 plan assumption으로 통과시키지 말고 계획 범위에 반영해야 한다.
- browser screenshot은 reference UI 표시일 뿐 Spinon 구현 evidence가 아니다.
