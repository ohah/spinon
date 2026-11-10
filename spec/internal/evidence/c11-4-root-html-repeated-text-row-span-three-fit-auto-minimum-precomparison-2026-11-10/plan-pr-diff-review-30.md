# C11.4 three-fit row-span 계획 PR diff 실패 관점 검토 30개

검토 대상은 base `96badf824d0b8e34016d8677a5c9770ee6bad7da`부터 plan commit `4f7e73d4d`까지의 실제 변경분이다. 계획 문서, 계획 자체의 20개 review, Chrome 157 fixture/capture/observation/verifier, hash manifest와 sample image를 각각 확인했다. 이 PR은 계획·비교 자료만 추가하며 제품 구현은 포함하지 않는다.

| # | 적대 검토 관점 | 확인한 실패 경로 | diff 판정 |
|---:|---|---|---|
| 1 | 변경 범위 오염 | 제품 Rust/style/runtime 파일이나 미분류 경로가 포함되는가 | 통과 · 계획, fixture, 사전 비교 evidence만 포함 |
| 2 | 공식 완료수 오염 | C11.4 체크·공식 `78/214`·계약 `0.1.0`을 올렸는가 | 통과 · 체크/계약 원본은 변경하지 않았다 |
| 3 | 계획과 실행 혼동 | Chrome 오라클을 Spinon 제품 구현 성공으로 서술하는가 | 통과 · README와 계획 모두 사전 비교로 구분 |
| 4 | 규범 URL 오류 | CSS Grid 규범 링크가 비공식 글이나 다른 버전을 가리키는가 | 통과 · W3C 2025 CRD의 §6.6/§7.2/§11.5를 고정 |
| 5 | automatic-minimum 조건 누락 | overflow·auto minimum track·flexible track 조건 중 하나가 생략됐는가 | 통과 · CSS §6.6 세 조건을 모두 적었다 |
| 6 | fit-content 최소 함수 오해 | fit cap을 minimum function 또는 fixed maximum으로 처리하는가 | 통과 · `auto` minimum과 content floor 제한을 명시 |
| 7 | indefinite percentage | `%` cap이 indefinite basis에서 허용될 여지가 있는가 | 통과 · definite `220px` basis만 지원 |
| 8 | 단위 혼합 | `px`와 `%` 숫자를 직접 같은 단위로 비교하는가 | 통과 · cap은 authored typed token/position에 묶고 `%` basis를 고정 |
| 9 | zero cap overclaim | cap이 항상 used size의 상한이라고 가정하는가 | 통과 · content floor 때문에 cap 초과 가능성을 부정하지 않는다 |
| 10 | fit multiplicity 모호성 | one/two/four-fit 목록이 신규 positive 범위로 섞이는가 | 통과 · 새 profile의 정확한 multiset과 주변 거부가 명시됨 |
| 11 | track count 오류 | six-member list에 잘못된 길이/중복이 들어가는가 | 통과 · 길이와 multiset을 장면 생성·검증에서 확인 |
| 12 | fit-position 누락 | 20개 위치 triple이 덜 생성되거나 잘못 정렬되는가 | 통과 · verifier가 고유 triple 수 20과 authored 위치를 확인 |
| 13 | cap assignment 중복 | 6개 cap 배치에 누락이나 중복이 있는가 | 통과 · 독립 verifier가 각 group의 모든 순열을 재검증 |
| 14 | tail permutation 누락 | remaining min/max/auto order가 일부 빠지는가 | 통과 · 여섯 고정 순서를 모두 사용 |
| 15 | 반복 phase offset | explicit row가 implicit list phase를 바꾸는가 | 통과 · explicit 10px row와 여섯 used implicit row를 분리 관측 |
| 16 | geometry 혼합 | definite Grid 크기·column·gap·writing mode가 장면마다 변하는가 | 통과 · fixture의 고정 CSS와 measured `40×220px` frame 검사 |
| 17 | span 범위 불일치 | 두 topology가 의도한 여섯 row를 만들지 않는가 | 통과 · seven used rows, item placement와 row-sum 관계를 확인 |
| 18 | source-order mutation | forward/reverse가 text payload 또는 semantic placement도 바꾸는가 | 통과 · 각 text id/placement를 고정하고 semantic frame을 pair 비교 |
| 19 | stretch 부작용 | stretch가 fit/min/max row 또는 총 frame을 임의로 바꾸는가 | 통과 · auto-only growth와 total used size를 독립 검증 |
| 20 | overflow mode 누락 | scrollable overflow가 candidate의 eligibility를 바꾸는가 | 통과 · fixture는 visible overflow이고 negative boundary에 scrollable을 둠 |
| 21 | text input drift | wrapping/font/payload 변화가 sizing 결과에 섞이는가 | 통과 · 기존 ASCII payload, monospace 10px/line-height 10px와 line count를 고정 |
| 22 | content-floor 계산 | automatic minimum 후보가 payload 아래로 내려가는가 | 통과 · eligible candidate 24,576회, floor violation 0 |
| 23 | frame/track 단위 오판 | computed string이나 item frame만으로 row size를 추정하는가 | 통과 · 7개 CSSOM row와 span sum/frame을 양 경로에서 비교 |
| 24 | rounding 분배 가정 | two-fit rounding rule을 근거 없이 three-fit에 재사용하는가 | 통과 · 실제 Chrome/Runtime trace 대조 후 최소 제한으로 결정하도록 함 |
| 25 | independent verifier 상관오류 | capture 결과만 같은 코드 경로로 스스로 승인하는가 | 통과 · 별도 프로세스의 `verify-chrome-matrix.mjs`가 NDJSON을 다시 읽는다 |
| 26 | matrix count/coverage | 11,520이 실제 곱 또는 pair denominator와 맞지 않는가 | 통과 · verifier가 720 track lists, 5,760 minimum/source/alignment pair 및 1,920 cap groups를 재확인 |
| 27 | browser source 혼동 | Terminal Browser나 사용자 기존 세션을 이용/종료하는가 | 통과 · 지정 Agent Browser session만 열고 close 후 비었음을 확인 |
| 28 | cleanup 과잉 | 사용자 simulator profile/data 또는 다른 브라우저 세션을 삭제하는가 | 통과 · precomparison은 sim을 열지 않았고 재사용 profile은 보존 대상 |
| 29 | artifact 무결성 | fixture/NDJSON/image/script와 hash manifest가 서로 다른 버전인가 | 통과 · `shasum -a 256 -c SHA256SUMS`의 모든 항목 OK |
| 30 | PR 표현 범위 | sample screenshot을 pixel parity/native evidence로 과장하는가 | 통과 · 4× screenshot은 단일 Chrome 장면이며 OS/app 검증과 분리 |

**결과:** 30개의 서로 다른 PR diff 관점에서 병합을 막는 결함은 발견되지 않았다. Chrome 캡처기에서 두 번 수정한 검증기 group-count/name 오류는 최종 commit의 verifier와 hash manifest에 반영됐으며 최종 11,520-scene capture와 독립 verifier를 다시 실행해 통과했다. 제품 구현은 계획 PR이 병합될 때까지 시작하지 않는다.
