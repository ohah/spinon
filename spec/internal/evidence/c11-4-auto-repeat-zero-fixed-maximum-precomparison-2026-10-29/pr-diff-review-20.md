# PR #800 변경분 실패 관점 검토 20개

**검토 대상:** PR #800, head `b2da02fed497bc8b6cf70accbbabe0ad2ea74846`
**범위:** 계획 문서, Chrome oracle fixture·관찰값·화면, 검증기, 검토 근거와 PR 본문 링크를 서로 다른 실패 경로로 확인했다. 계획 자체의 사전 검토 20개와는 별도다.

| # | 실패 관점 | 판정 및 근거 |
| ---: | --- | --- |
| 1 | 계획 전용 PR에 일반 CSS나 runtime admission 코드가 섞였는가 | 통과. 변경 파일은 계획과 oracle/evidence뿐이며 layout/runtime 제품 소스는 없다. |
| 2 | 변경이 기존 완료 상태를 올리거나 오래된 `72/213`을 바꾸는가 | 통과. 계획과 PR 본문 모두 C11.4 미완료 및 `72/213`을 명시한다. |
| 3 | CSS Grid가 1px UA floor를 규범상 강제한다고 오인시키는가 | 통과. 명세가 요구하는 것은 UA 지정 최소값이며 1px은 제안값/현 oracle로 구분했다. |
| 4 | fixed-size 문법이 0px maximum 사례를 제외하는가 | 통과. 이번 검증은 기존 음이 아닌 fixed breadth 경계로 제한한다. |
| 5 | 24 scene Cartesian product에서 조합이 빠지거나 중복되는가 | 통과. verifier가 axis 2 × minimum 3 × mode 2 × gap 2의 24 unique key를 검사한다. |
| 6 | definite repeat basis와 root 외곽 크기를 혼동하는가 | 통과. padding/border 0, definite 16px 반복축이며 root·item rect를 별도 기록한다. |
| 7 | gap이 repeat count에 반영되지 않는가 | 통과. 독립식은 `n + (n − 1) × gap ≤ 16`이고 verifier가 16/4를 검사한다. |
| 8 | 복수-track pattern 식을 단일-track pattern에 잘못 적용하는가 | 통과. 고정 pattern은 한 track이고 plan의 2px control은 별도 산술 기대값을 명시한다. |
| 9 | 세 번째 line 배치가 최소 repeat 수보다 뒤에 있어 implicit track이 생기는가 | 통과. 최소 기대 반복 수는 4이고 item은 세 번째 repeated track에 배치된다. |
| 10 | item min-size가 intrinsic 환경값에 좌우되는가 | 통과. 12×12px item의 min-width/min-height를 고정했고 이 한계도 문서화했다. |
| 11 | occupied 12px track과 선언 0px maximum을 잘못 동일시하는가 | 통과. JSON은 authored maximum과 CSSOM used track list를 별도 필드에 담는다. |
| 12 | auto-fill과 auto-fit 구분이 gap 0에서만 검사되는가 | 통과. gap 0과 4px을 모두 검사하며 gap 4px에서 offset 차이를 확인한다. |
| 13 | auto-fit에서 빈 track은 접혀도 연결 gutter가 남는가 | 통과. Chrome oracle의 4px gap item offset 0px과 verifier 기준에 반영했다. |
| 14 | column 장면에서 row gap 또는 row 장면에서 column gap을 잘못 바꾸는가 | 통과. 반복축 gap만 0/4px이고 직교축 gap은 0px으로 고정한다. |
| 15 | 축별 track list 길이를 전체 CSS grid track 수와 혼동하는가 | 통과. verifier는 반복축의 computed list만 세고 직교축 크기도 따로 검사한다. |
| 16 | raw JSON이 현재 fixture 입력 또는 브라우저 버전과 다를 수 있는가 | 통과. capture metadata와 scene inputs를 보존했고 base revision을 기록했다. |
| 17 | verifier가 입력만 확인하고 관찰 geometry를 검증하지 않는가 | 통과. track 개수/크기, root dimensions, item size와 axis별 offset을 검사한다. |
| 18 | screenshot이 비어 있거나 깨진 이미지로 PR 본문에서 누락되는가 | 수정 후 통과. 최초 확인에서 이미지가 1280×960으로 렌더링됨을 확인했다. |
| 19 | PR 본문의 상대 문서 링크가 PR 경로 아래로 잘못 resolve되는가 | 수정 후 통과. Agent Browser에서 잘못된 `/pull/<path>` 목적지를 발견해 commit 고정 `/blob/<sha>/<path>` 링크로 바꾸고 네 링크를 다시 확인했다. |
| 20 | hash, label, merge policy, status check 상태를 잘못 보고하는가 | 통과. sha256 manifest 전 항목이 맞고 `planning`·`area: layout` 라벨이 설정됐다. main은 linear history를 요구하며 `--required` 조회는 check가 없다고 반환했다. |

**최종 확인:** PR 본문 screenshot `complete=true`, intrinsic image size `1280×960`이다. 변경분에는 제품 동작 검증이나 C11.4 완료 주장이 없다. relative link 수정 뒤 PR 본문을 다시 열어 링크와 이미지 렌더링을 확인했고, Agent Browser 세션을 닫은 뒤 관련 브라우저 프로세스가 남지 않은 것을 확인했다.
