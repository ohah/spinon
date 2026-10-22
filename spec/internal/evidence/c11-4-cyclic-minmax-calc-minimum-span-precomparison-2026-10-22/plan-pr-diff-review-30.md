# C11.4 cyclic minmax calc minimum span 계획 PR 변경 검토 30개

검토 대상은 계획 PR의 변경 파일이다. 계획 자체의 별도 20개 검토를 반복하지 않고, PR diff가 기존 상태·문서·근거와 충돌하거나 독자가 범위를 넓혀 해석할 수 있는 경계를 확인했다.

| # | 독립 검토 관점 | 결과 |
|---:|---|---|
| 1 | 계획만 추가했는데 공식 구현 상태를 바꿔 완료로 오인시킨다 | `spec/STATUS.md`는 변경하지 않았고 계획·사전 비교 단계라고 명시한다. 통과. |
| 2 | 로드맵의 공식 완료 수가 계획 추가 때문에 변한다 | 카드와 계획은 `72/213`, C11.4 미완료를 유지한다. 통과. |
| 3 | 부모 작업 계획의 다음 slice 설명과 신규 계획의 입력이 다르다 | 양쪽 모두 두 explicit track, span-2 item, cyclic/definite 네 scene을 가리킨다. 통과. |
| 4 | PR #493이 이미 span 경로를 다뤘다고 잘못 요약한다 | #493의 범위를 한 explicit track/direct text로 제한하고 span은 제외됐다고 적는다. 통과. |
| 5 | 신규 계획 링크의 경로/파일명이 불일치한다 | 부모 계획, 로드맵, precomparison README가 동일 plan slug로 연결된다. 통과. |
| 6 | CSS Grid 자료가 버전 없는 초안 또는 움직이는 URL이다 | 날짜가 고정된 2025 CRD 문서와 관련 절 링크를 사용한다. 통과. |
| 7 | Chrome을 규범 동작 결정자로 서술한다 | 계획과 근거에서 규범 계산과 Chrome 관측을 나누고 로드맵의 “oracle” 표현도 관측으로 고쳤다. 통과. |
| 8 | HTML precomparison 입력이 계획에서 지칭한 fixture와 다르다 | 계획이 체크인 fixture 경로를 직접 연결하며 README/JSON이 같은 경로를 기록한다. 통과. |
| 9 | 네 scene의 식별자가 중복되거나 서로 혼동된다 | cyclic/definite 및 columns/rows 조합으로 네 ID가 분리된다. 통과. |
| 10 | 반복 track 문법이 실제 multi-track 경로를 숨긴다 | 두 explicit `minmax()` track을 나란히 선언하고 `repeat()`을 사용하지 않는다. 통과. |
| 11 | spanning item이 실제로 두 column을 차지하지 않는다 | column scene은 `grid-column:1 / span 2`로 고정하고 Chrome CSSOM 관찰에 placement를 기록한다. 통과. |
| 12 | spanning item이 실제로 두 row를 차지하지 않는다 | row scene은 `grid-row:1 / span 2`로 고정하고 Chrome CSSOM 관찰에 placement를 기록한다. 통과. |
| 13 | spanning contribution에 text/font fallback 차이가 섞인다 | 고정 Ahem 10px fixture와 font-loaded gate를 사용하고 두 입력의 SHA를 기록한다. 통과. |
| 14 | column intrinsic basis를 border-box 또는 definite control과 다르게 센다 | cyclic content width는 50px이고 2px border는 별도이며 definite control도 content width 50px이다. 통과. |
| 15 | column gap을 contribution에서 0회 또는 2회 차감한다 | 50px max-content span에서 4px gutter를 한 번 빼 46px을 분배한다고 양쪽 문서에서 설명한다. 통과. |
| 16 | column 초기 max-content track 배분과 final track 값을 혼동한다 | intrinsic `[23,23]`과 final cyclic resolution `[29,29]`을 단계별로 분리한다. 통과. |
| 17 | cyclic percentage minimum을 intrinsic 단계에서 미리 resolve한다 | intrinsic sizing 중 auto 취급과 결정된 50px basis 이후 29px resolution을 분리한다. 통과. |
| 18 | max-content limit이 min보다 작은 경우 minmax 범위 적용이 누락된다 | 23px max-content limit보다 29px minimum이 커 final track이 최소값으로 올라감을 명시한다. 통과. |
| 19 | final column 합계에서 gutter 또는 item 폭을 누락한다 | `29 + 4 + 29 = 62px`로 item과 50px content root 차이를 명시한다. 통과. |
| 20 | definite column control이 다른 percentage basis를 사용한다 | control은 50px content width여서 cyclic intrinsic basis와 같은 minimum을 검증한다. 통과. |
| 21 | row text가 지정 inline size에서 두 줄이 되지 않는다 | 20px content width와 Ahem `a a`로 두 줄/20px block contribution을 고정한다. 통과. |
| 22 | row contribution에 4px row gap이 잘못 포함된다 | 20px span contribution에서 gap 4px을 한 번 제외해 두 8px intrinsic rows를 만든다. 통과. |
| 23 | row cyclic percentage가 inline-size를 basis로 오용한다 | block max-content height 20px을 basis로 분명히 적고 12px final minimum을 계산한다. 통과. |
| 24 | row definite control이 cyclic scene과 다른 height basis를 갖는다 | definite control height와 cyclic max-content basis가 모두 content height 20px이다. 통과. |
| 25 | final row 합계에서 gutter 또는 item height를 누락한다 | `12 + 4 + 12 = 28px`과 content root height 20px을 구분한다. 통과. |
| 26 | root border-box와 content-box 수치를 같은 기준으로 비교한다 | column `50×10 → 54×14`, row `20×20 → 24×24`로 2px border를 별도 계산한다. 통과. |
| 27 | border 바깥 표본이 item 내부를 확인하지 못한다 | root-relative `(58,6)`과 `(8,27)`은 각 border-box를 벗어나며 item frame 안에 있다. 통과. |
| 28 | 바깥 표본이 item 배경에 덮이거나 border stroke 검증으로 확대된다 | `(66,6)`과 `(8,31)`은 item frame 밖 흰 page control이고 border stroke 판정은 제외한다. 통과. |
| 29 | Chrome 캡처가 다른 viewport·DPR·입력 결과를 가리킨다 | observation JSON은 Agent Browser/Chrome 156, `1280×633`, DPR 1, 네 scene frame 및 입력 hash를 기록한다. 통과. |
| 30 | 수정된 README가 낡은 manifest hash로 배포된다 | README·JSON·PNG·fixture·Ahem을 포함해 `SHA256SUMS.txt`를 재생성했고 전 항목 검증했다. 통과. |

**판정:** 이 PR diff에서 계획·로드맵·사전 비교의 경계가 충돌하거나 완료 상태를 과장하는 결함은 찾지 못했다. 실제 runtime 동작 판정은 계획 PR 병합 뒤 네 고정 scene을 구현해 별도 검토·시뮬레이터에서 해야 한다.
