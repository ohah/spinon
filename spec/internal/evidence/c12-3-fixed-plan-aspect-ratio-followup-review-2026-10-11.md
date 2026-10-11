# C12.3 fixed 계획의 aspect-ratio 경계 후속 검토

이 별도 계획 검토는 fixed fixture에 이미 포함된 `viewport-aspect-ratio`와 제한 runtime profile 경계가 계획에서 빠졌던 점을 반영한다. 기존 20개 계획 검토를 대체하지 않고, 계획 수정 뒤 계약 경계를 서로 다른 실패 관점으로 다시 확인한다.

| # | 실패 관점 | 확인 결과와 계획의 결정 |
|---:|---|---|
| 1 | 고정 Chrome 기준에 `aspect-ratio` case가 있는데 runtime 성공 subset에서 빠지는가 | `viewport-aspect-ratio`를 지원 subset에 포함하고 계획에 명시했다. |
| 2 | 지원 사례를 일반 aspect-ratio 지원으로 과장하는가 | `width:40px;height:auto;aspect-ratio:2 / 1` 한 조합만 C07.3 계산으로 지원한다. |
| 3 | auto height가 intrinsic text·replaced sizing으로 바뀌는가 | 고정 width와 author ratio로 산출되는 높이만 포함하고 intrinsic sizing은 계속 제외한다. |
| 4 | 두 축 definite인데 ratio가 한 축을 덮어쓰는가 | 두 definite 축 조합은 이 추가 성공 범위에 들어오지 않으며 기존 C07.3 정의를 유지한다. |
| 5 | ratio를 fixed containing block 대신 source parent 기준으로 계산하는가 | ratio는 크기 계산만 담당하고 fixed inset·owner는 계속 viewport 기준이다. |
| 6 | `box-sizing`·padding·border가 ratio 결과를 숨기거나 중복 적용하는가 | 고정 case는 border-box 및 0 padding/border라 계산 전제와 frame `40×20`을 명시했다. |
| 7 | ratio의 분자·분모를 반대로 해석하는가 | `2 / 1`은 width:height 비율 2:1이며 expected height는 20px이다. |
| 8 | 소수 ratio나 zero·invalid ratio를 이 case에서 암묵적으로 허용하는가 | 이 fixture는 유효한 2:1만 확인한다. invalid·다른 ratio는 이 추가 계약으로 확대하지 않는다. |
| 9 | computed ratio가 inline allowlist에는 있지만 Stylo snapshot에는 없는가 | RuntimeBlockPositioningV1 computed property와 author-property 목록을 같은 profile로 연결한다. |
| 10 | Block formatting 전용 inline allowlist가 위치 지정 profile에 재사용되는가 | 별도 `first_unsupported_runtime_block_positioning_inline_property` 경계를 사용하도록 계획을 보강했다. |
| 11 | `aspect-ratio` 선언이 author stylesheet에서만 허용되고 inline에서 거부되는가 | 양 입력 경로는 같은 positioning profile 속성 목록을 사용해야 한다. |
| 12 | inline만 통과하고 author stylesheet preflight가 거부하는가 | stylesheet allowlist와 inline 검사 모두 `RUNTIME_BLOCK_POSITIONING_AUTHOR_PROPERTIES`에 연결한다. |
| 13 | invalid ratio 뒤 이전 유효 선언이 유지되는 CSS cascade를 무시하는가 | Stylo winner와 진단을 사용하며 별도 문자열 파서로 ratio를 재해석하지 않는다. |
| 14 | ratio 계산 후 fixed owner가 일반 absolute owner로 합쳐지는가 | C07.3 dimension 계산과 C12.3 fixed-owner map을 별도 보존한다. |
| 15 | fixed element의 absolute 자손이 ratio frame 대신 viewport origin에 붙는가 | 자손 containing block은 ratio가 적용된 fixed element frame이다. |
| 16 | fixed 자손이 ratio가 적용된 fixed parent를 containing block으로 사용하는가 | fixed descendant의 owner는 여전히 viewport다. |
| 17 | hidden subtree의 ratio fixed node를 viewport root에 올리는가 | `display:none` owner 규칙이 ratio 계산보다 먼저 보존되어 frame이 없다. |
| 18 | DPR별 computed ratio 또는 CSS frame이 달라지는가 | 동일 CSS viewport의 DPR 1·2·2.625·3 모두 같은 CSS frame을 비교한다. |
| 19 | renderer가 40×20 geometry를 다른 node frame이나 다른 resize revision에 게시하는가 | reference 비교는 node identity, field, environment revision을 함께 확인한다. |
| 20 | 단일 ratio 사례 통과를 ratio+min/max·intrinsic·replaced 조합 지원으로 확대하는가 | 완료 표시는 이 fixed case와 나머지 명시 inventory만 뜻하며 제외 조합은 별도 작업으로 남긴다. |

## 계획 반영

- 고정 inventory에 이미 존재하는 `viewport-aspect-ratio`를 positive runtime subset에서 계산하도록 명시했다.
- C07.3의 ratio 계산을 재사용하되 `width:40px`, `height:auto`, `aspect-ratio:2 / 1` 한 입력의 `40×20 CSS px`만 이 단계의 추가 성공 범위로 고정했다.
- 위치 지정 profile의 inline 허용목록을 Block formatting profile과 분리하고 stylesheet·inline 양쪽에서 같은 제한 속성 집합을 적용한다.
- ratio·intrinsic sizing의 다른 조합은 이번 C12.3 완료 주장에 포함하지 않는다.
