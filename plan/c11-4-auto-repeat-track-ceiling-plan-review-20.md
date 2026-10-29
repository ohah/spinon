# Auto-repeat track 상한 계획 검토 · 실패 관점 20개

각 항목은 이 한정 계획이 잘못된 근거를 만들거나 범위를 부풀릴 수 있는 별도 경로를 확인한다.

| # | 실패 관점 | 계획의 방어 조건 |
| --- | --- | --- |
| 1 | authored descriptor만 보고 Taffy 확장 결과로 오인한다 | `Initialize` stage에서 gutter가 아닌 실제 확장 track을 읽는다. |
| 2 | 9,999 경계 입력에 실수로 10,000px basis를 쓴다 | 각 입력 크기와 기대 개수를 같은 table row에서 가져온다. |
| 3 | 10,001 요청을 조용히 잘못된 오류로 처리한다 | 고정 Taffy 0.15.0에서 성공한 10,000 track clamp를 확인한다. |
| 4 | Columns에는 definite width가 있지만 Rows는 indefinite하다 | 각 입력에서 선택 축의 root size를 고정하고 확인한다. |
| 5 | nonzero gap이 반복 횟수를 바꾼다 | 모든 입력의 gap을 0으로 둔다. |
| 6 | `auto-fill`과 `auto-fit`의 track 수만 비교하고 collapse를 놓친다 | mode별로 모든 반복 track의 collapse flag도 검사한다. |
| 7 | auto-fit의 collapse가 예기치 않은 placement에 좌우된다 | root child와 item map이 비어 있는 입력인지 확인한다. |
| 8 | Rows와 Columns를 동시에 최대로 확장해 trace 예산을 넘긴다 | 축을 독립 실행하고 나머지 축은 고정 track 하나만 둔다. |
| 9 | partial trace를 완전한 관찰로 취급한다 | trace 실패가 없고 target axis의 마지막 pass에 Initialize가 하나인지 확인한다. |
| 10 | gutter를 CSS track으로 잘못 센다 | `is_gutter`를 제외하고 원시 stage 배열의 길이도 확인한다. |
| 11 | source-order control에서 prefix·repeat·suffix를 구분하지 못한다 | 세 역할에 2px·1px·3px의 서로 다른 크기를 쓴다. |
| 12 | source-order control이 개수만 보고 suffix 절단을 놓친다 | 10,003px fitting과 10,004px cap 입력의 첫 track·마지막 track을 확인한다. |
| 13 | prefix를 반복 시작 index에서 잘못 차감한다 | prefix 한 개 이후 남은 반복 수를 기대값에 반영한다. |
| 14 | 시스템에 설치된 다른 Taffy 버전에 기대게 된다 | vendored 0.15.0 source의 clamp 로직에 기대값을 고정한다. |
| 15 | trace 100,000 예산을 무제한 10,000 track 성공 보장처럼 해석한다 | 이 고정 matrix만 검사하고 trace 예산 한계를 별도로 남긴다. |
| 16 | Taffy 호출 전에 10,001개 authored track buffer를 예약한다 | 한 개짜리 반복 pattern만 구성하고 제한된 trace를 관찰한다. |
| 17 | 부동소수 tolerance 때문에 off-by-one을 허용한다 | 정수 track count는 정확히 비교하고 1·2·3의 고정 크기를 사용한다. |
| 18 | test-only 변경이 production feature에 포함된다 | 기존 Grid unit-test 모듈 아래에만 코드를 두고 feature boundary를 살핀다. |
| 19 | 이 matrix를 일반 CSS Grid 또는 WPT 지원으로 확대한다 | C11.4/C11을 미완료로 두고 제외 grammar와 runtime 경로를 적는다. |
| 20 | 시각 결과가 없는 unit test에 screenshot 근거를 덧붙인다 | screenshot 해당 없음으로 두고 raw test output과 source revision을 남긴다. |

**검토 결과:** 계획 관점 20개를 모두 반영했다. prefix/suffix 고정 크기 5px가 반복 계산에서 차감된다는 점을 확인하고 최초 경계값 10,001px를 fitting/cap 구분이 되는 10,003px/10,004px로 수정했다.
