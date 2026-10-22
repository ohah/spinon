# C11.4 cyclic minmax calc minimum span 계획 실패 관점 검토 20개

검토 대상: [계획](./c11-4-cyclic-minmax-calc-minimum-span.md), [독립 기대값·Chrome 156 관찰](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-span-precomparison-2026-10-22/README.md), [고정 입력](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-span-c114-precomparison.html). 아래 경계는 서로 다른 실패 경로다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 두 명시 track이 아닌 단일 `repeat()`을 써서 component 개수·span이 다른 경로를 검증한다 | fixture는 두 개의 explicit `minmax()`만 선언하고 item placement를 1/span 2로 고정한다. 통과. |
| 2 | column percentage를 height나 border-box 기준으로 resolve한다 | 열 basis는 intrinsic content width 50px이고 root의 좌우 2px border는 basis에서 제외한다. 통과. |
| 3 | row percentage를 inline-size 기준으로 resolve한다 | 행 basis는 content block size 20px이며 definite inline-size는 20px로 고정한다. 통과. |
| 4 | intrinsic contribution 계산에서 cyclic minimum을 이미 resolve된 수치로 사용한다 | 2025 CRD의 cyclic percentage 규칙대로 intrinsic basis 단계에서는 auto 취급하고 final sizing에서만 resolve한다. 통과. |
| 5 | span 기여에서 column gap을 track contribution에 두 번 뺀다 | `aa aa` max-content 50px에서 column gap 4px을 한 번 제외해 총 46px을 두 track에 배분한다. 통과. |
| 6 | spanning item의 source string이 의도한 max-content size를 만들지 않는다 | Ahem 10px에서 `aa aa`는 50px, 폭 20px의 `a a`는 두 줄/20px block contribution이 된다. font-ready gate를 둔다. 통과. |
| 7 | column max-content track 배분이 gap 포함 전체 50px으로 계산된다 | 두 track `[23,23]`과 4px gap 합이 50px이며 root intrinsic width를 54px로 올리지 않는다. 통과. |
| 8 | row max-content basis에 row gap을 중복 반영한다 | 20px span contribution에서 4px row gap을 제외한 16px을 두 row로 나눠 intrinsic `[8,8]`과 height 20px을 얻는다. 통과. |
| 9 | definite-size controls가 cyclic scene과 다른 percentage basis를 쓴다 | column control width는 50px, row control height는 20px로 cyclic scene의 독립 basis와 동일하다. 통과. |
| 10 | `minmax()` maximum이 minimum보다 작은 경우를 빠뜨린다 | column max-content limit 23px < minimum 29px, row limit 8px < minimum 12px이다. 범위 규칙으로 각각 최소값에 올라가야 한다. 통과. |
| 11 | row item이 실제로 spanning 두 행에 배치되지 않는다 | CSS `grid-row:1 / span 2`와 CSSOM observation으로 두 행 배치를 확인한다. 통과. |
| 12 | columns에서 item의 width가 track 두 개와 gap 합을 반영하지 않는다 | final item width는 29+4+29=62px로 root content width 50px보다 12px 크다. 통과. |
| 13 | border box와 content box를 같은 값으로 비교한다 | 2px border를 각 변에 더해 column root `54×14px`, row root `24×24px`로 분리했다. 통과. |
| 14 | overflow sample이 root 안쪽만 확인한다 | column은 root-relative `(58,6)`에서 item 내부/오른쪽 border 바깥, row는 `(8,27)`에서 item 내부/아래 border 바깥을 표본화한다. 통과. |
| 15 | item 바깥 표본이 여전히 item 배경에 덮여 잘못 판정된다 | column `(66,6)`, row `(8,31)`은 item frame 밖 page-white control이다. 통과. |
| 16 | sample 좌표가 content/border origin을 혼동한다 | 모든 색 sample은 root-relative이며 item 시작 offset `(2,2)`와 root border-box 크기에서 계산했다. 통과. |
| 17 | Chrome 관찰을 규범 기대값으로 역산한다 | README는 Ahem metric, gap subtraction과 CSS Grid sizing order에서 기대값을 계산하고 Chrome JSON을 별도 관찰 자료로 기록한다. 통과. |
| 18 | 화면·JSON이 다른 입력 버전이나 글꼴을 참조한다 | fixture와 Ahem TTF의 SHA-256을 관찰 JSON 및 manifest에 기록했다. 통과. |
| 19 | border 밖 item paint를 CSS border stroke 검증으로 과장한다 | 계획은 item overflow 색 sample만 판정하고 border stroke의 pixel 정확도를 주장하지 않는다. 통과. |
| 20 | 네 scene 결과를 WPT·임의 CSS·C11.4 완료로 확대한다 | 고정 allowlist, Simulator 범위, WPT/실기기 한계와 `72/213` 상태를 계획에 반복 명시했다. 통과. |

**판정:** 20개 독립 경계를 검토했다. 발견된 차단 결함은 없었다. Chrome 156은 비교 관찰자이며 규범 기대값은 CSS Grid 2025 CRD와 고정 입력으로 계산했다.
