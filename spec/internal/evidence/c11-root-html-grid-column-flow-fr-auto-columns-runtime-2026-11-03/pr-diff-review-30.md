# C11.1 `fr` implicit-column 실제 변경 diff 적대 검토 30개

검토 대상은 구현 후보의 31개 staged 파일 diff다. 이 검토 기록 파일 자체와는 별개로 확인했다. 계획 문서의 20개 실패 검토를 재사용하지 않았다. 각 관점은 실제 코드·테스트·근거·상태 문서 경계를 별도로 대조했다. 이는 작성자 자체 검토 기록이며 GitHub CI 또는 별도 승인자 검토를 뜻하지 않는다.

| # | diff 관점 | 확인 결과 |
|---:|---|---|
| 1 | 공식 완료 상태를 후보 구현 때문에 올렸는가 | `spec/STATUS.md` C11 checkbox를 그대로 두고 72/213 유지 문장을 명시한다. |
| 2 | plan 상태가 계획-only로 남았는가 | 계획 PR #1065 병합과 구현 후보 검증 상태를 구분해 적었다. 구현 merge 완료로 표시하지 않는다. |
| 3 | Root HTML auto-column cascade gate | Fraction list 옵션은 기존 Root HTML fixed-auto-column profile와 `grid-auto-flow:column`이 모두 맞을 때만 켜진다. |
| 4 | auto-row parser 분리 | auto-row list는 새 fraction option을 false로 넘기며, 이 변경에서 row fraction 지원을 추가하지 않는다. |
| 5 | scalar allocation failure | Fraction scalar를 typed list로 만들 때 기존 reservation failure hook와 `try_reserve_exact`를 쓴다. |
| 6 | multi-item list allocation failure | mixed list도 quota 확인 후 한 번 예약하며 기존 실패 원자성·복구 테스트를 확장한다. |
| 7 | computed style typed values | Projection은 direct px/%/fr typed variant만 허용해 원본 cascade type과 일치한다. |
| 8 | CSS invalid negative fraction fallback | `-1fr`가 CSS initial `auto`로 cascade되는 기존 의미를 고정했다. Layout에 negative factor를 전달하지 않는다. |
| 9 | unsupported grammar regression | Calc, minmax, fit-content, intrinsic breadth, repeat와 auto remain rejected. |
| 10 | style-to-layout profile isolation | Column-flow candidate validation 외에는 `grid-auto-columns` non-auto values를 거부한다. |
| 11 | layout placement profile | `RootHtmlColumnFlowV1`에서만 direct Fraction을 확장하며 dense/other placement mode 허용 목록은 바꾸지 않았다. |
| 12 | invalid factor diagnostic | NaN, infinity와 negative direct factors는 CSS projection/layout boundaries 모두에서 거부한다. |
| 13 | empty/oversized pattern | Empty list와 10,000 track 상한 검사는 새 branch보다 먼저 유지된다. |
| 14 | axis width basis | Percentage/fraction implicit pattern만 definite finite non-negative width를 요구하고 px-only track behavior는 그대로다. |
| 15 | width auto, math, negative and overflow | Computed author property, layout math map 및 resolved f32 CSS-px bound를 점검한다. 거부 테스트와 error property가 있다. |
| 16 | min/max width override | Cascade/projection과 lower layout boundary에서 non-auto constraints를 거부하며 각 concrete property negative case가 있다. |
| 17 | aspect ratio override | Typed ratio와 serialized non-auto property를 확인한다. Auto/initial은 양성 path를 불필요하게 막지 않는다. |
| 18 | percentage basis | `%`는 definite width를 기준으로 두고 column gap을 basis에서 차감하지 않는다. 기존 percent extent validator에 regression coverage가 있다. |
| 19 | fractional remaining space | Fixed/% extents와 gaps를 제외한 뒤 Taffy Grid algorithm에 fraction tracks를 보내며 gap 중복 계산을 추가하지 않는다. |
| 20 | subunit factor sum | `0.25fr` 케이스가 sum floor 1과 trailing free space를 고정한다. |
| 21 | zero factor | `0fr`는 auto-minimum base가 남을 수 있고 test가 fixed 10px item을 확인한다. |
| 22 | very large finite factors | `1e38fr` path를 4-track layout case와 cascade typed test로 덮고 f64 accumulator 근거와 대조했다. |
| 23 | item count and implicit extent | Root HTML collector의 visible owned direct-child count와 explicit row/column calculation을 바꾸지 않았다. |
| 24 | sparse 배치 순서 | 신규 runtime 장면은 direct item 12개를 두며, item x/y frame에서 필요한 암시적 열 시작점 여섯 개와 두 행을 확인한다. |
| 25 | Root/viewport 경계 주장 | 앱 로그에 실제 viewport와 root 크기가 남아 있다. 300 CSS px Grid가 viewport 안에 전부 들어온다고 쓰지 않았다. |
| 26 | runtime fixture 범위 | Fixture는 중첩 없는 Grid 하나와 지원되는 빈 fixed-size div item만 쓰며 selector에 의존하는 배치 계약이 없다. |
| 27 | FFI fixture 경계 | 기존 opt-in C11.1 fixture 함수를 재사용하며 새 FFI 심볼이나 제품 route를 추가하지 않았다. |
| 28 | Android route 선택 | 기존 opt-in fixture flag를 쓰고 API 37 AVD serial만 지정했다. Android 실기기에는 설치하지 않았다. |
| 29 | iOS route와 bundle identity | iPhone 17 Pro/iOS 26.2 Simulator log와 설치된 executable hash가 runtime README와 일치한다. |
| 30 | documentation/evidence and claim boundary | Runtime README preserves the initial nested-grid harness failure, limits screenshot claims, records platform/build hashes, and keeps 72/213 unchanged. |

## 결과

실제 변경 diff에서 새 결함은 발견하지 못했다. 첫 artifact 확인에서 runtime README fixture 링크가 저장소 경로보다 한 단계 위를 가리키는 것을 발견해 네 단계 상대 경로로 수정했고, staged diff를 다시 확인했다. `git diff --cached --check` 통과 후 기록했다. PR check rollup과 merge는 PR 생성 이후 별도 확인한다.
