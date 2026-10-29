# C11.4 auto-repeat fixed maximum PR diff 실패 관점 30개

2026-10-29 candidate diff의 Rust/layout, Stylo projection/cascade, Taffy, FFI fixture, 두 native harness, capture tool, 상태·계약·로드맵·증거 문서를 구현/runtime 검토와 다른 30개 관점으로 확인했다.

| # | 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | 새 layout branch가 기존 fixed-minimum `<fixed-size>` 분기를 대체해 회귀시키는가? | 기존 branch는 그대로 두고 validator 함수 이름/호출만 공통화했다. 기존 fixed-min positive control이 49-scene 집합에 남아 있다. |
| 2 | intrinsic minimum 분기가 세 키워드 외의 minimum을 허용하는가? | pattern match가 `Auto`, `MinContent`, `MaxContent`만 받으며 default는 `false`다. |
| 3 | 새 branch가 fraction/auto/intrinsic maximum을 fixed로 오인하는가? | fixed-maximum helper에서 세 경우 모두 거부한다. |
| 4 | `Percentage` maximum이 indefinite axis에서 사용되는가? | layout validator는 definite positive content basis 없이는 통과시키지 않는다. |
| 5 | 백분율 해석 중 overflow/NaN이 발생해 repeat count에 들어가는가? | 해석된 breadth가 finite인지 다시 검사한다. |
| 6 | calc ID의 node/property owner가 다른 선언을 통과시키는가? | 기존 typed CSS math registry 검사에 node와 `GridTemplateColumns/Rows` property가 결합돼 있다. |
| 7 | arbitrary calc AST가 accidentally admission되는가? | 추가 경로는 existing exact two-term `length + percentage` validator를 재사용한다. |
| 8 | 0px breadth의 CSS auto-repeat floor가 검증됐다고 과장하는가? | 49 oracle에는 zero maximum이 없다. README와 구현 review에 이 boundary를 미검증으로 명시했다. |
| 9 | auto-repeat 최대 breadth가 repeat-count 계산 전에 minimum 기여로 바뀌는가? | 36 positive scenes의 track count와 bases가 definite maximum oracle에 일치한다. |
| 10 | gap이 repeat 수 또는 다중 track pattern 계산에서 누락되는가? | 5px gap threshold와 두-track pattern 4 scene이 계산식을 확인한다. |
| 11 | `auto-fill` 빈 repeat track이 빠지는가? | 49 scene의 expected track lengths와 collapse flags를 양 플랫폼에서 검사한다. |
| 12 | `auto-fit` 비어 있는 repeat track이 남는가? | `auto-fit` 18 positive matrix scene과 multi-track control이 collapse vector를 확인한다. |
| 13 | collapse된 track 옆 gutter가 잘못 유지되는가? | Chrome scene의 collapsed track/gutter 기대값과 양 platform trace를 대조했다. |
| 14 | row/column axis 구현이 한 축만 갱신되는가? | 49 scene은 두 축을 모두 포함하며 Java/Swift matcher가 axis별 root frame을 계산한다. |
| 15 | line-name 혹은 multiple-repeat 동작까지 이 diff가 넓혔다고 설명하는가? | JSON은 fixed pattern input만 정의하며 설명은 arbitrary/multiple auto-repeat을 명시적으로 제외한다. |
| 16 | intrinsic item 최소값이 fixed maximum으로 잘려 overflow regression이 생기는가? | Taffy 변경 결과는 max10/basis10 6 overflow scene으로 확인하고 사용 범위를 문서화했다. |
| 17 | Taffy item minimum change가 replaced/percentage preferred-size 경로를 깨뜨리는가? | 변경은 해당 두 입력에서 기존 fixed-area limit 호출을 유지한다. CSS Grid unit/runtime matrix가 통과했다. |
| 18 | root content-box와 viewport basis가 달라 percentage result가 우연히 맞는가? | direct layout inputs 및 fixture metadata가 definite axis size를 명시하고 30%/130px 기대값 39px을 검사한다. |
| 19 | two-track pattern에서 internal 20px track/gap이 repeated breadth에 빠지는가? | 4 scene이 pattern의 두 track과 사이 gap을 포함한 반복 수/track vector를 oracle과 대조한다. |
| 20 | style-to-layout projection이 keyword minimum을 `LengthPx(0)`로 변환해 의미를 잃는가? | projection test는 세 intrinsic enum을 그대로 기대한다. |
| 21 | Stylo cascade에서 maximum calc branch가 기존 fixed-minimum maximum 정책을 넓히는가? | `auto_repeat_calc_is_supported`의 exact term count/type 및 sign 제한을 유지한다. |
| 22 | 신규 FFI module 또는 payload가 default artifact에 포함되는가? | module 선언과 route 모두 `c04-runtime-gpu-fixture-test` cfg로 감싸고 default archive marker 부재를 확인했다. |
| 23 | fixture JSON이 반복 파싱에 과도한 임의 script/input을 제공하는가? | catalog test가 49 exact IDs와 per-scene script 8 KiB 상한을 검사한다. |
| 24 | unknown scene ID가 fallback으로 잘못 route되는가? | Rust adapter와 shared C11.4 router test가 `fixed-maximum-unlisted` 거부를 확인한다. |
| 25 | shared capture catalog의 이전 scene 수 545가 남아 검사가 약화되는가? | capture expectation과 boundary assertions를 594로 함께 갱신했고 Node suite 21/21을 통과했다. |
| 26 | capture runner가 fixed maximum JSON을 읽지 않거나 scene 수를 중복 더하는가? | loader는 신규 catalog를 한 번 append하고 selected IDs가 exact fixture set인지 boundary test가 확인한다. |
| 27 | Android allowlist가 Chrome/runtime JSON과 다른 ID를 가지는가? | static check에서 세 집합이 각각 49이고 누락/추가 ID가 0이다. |
| 28 | Android overflow sample이 viewport 내부 point를 outside-root 증거로 잘못 라벨링하는가? | matcher는 sample ID·target node·CSS coordinate를 모두 검사하며 column/row별 외부 좌표를 별도로 쓴다. |
| 29 | iOS capture가 fixture define 없이 성공처럼 보고되거나 다른 scene set을 허용하는가? | Set은 fixture compile flag 안에서만 있고 49 exact IDs다. 마지막 실행은 Metal pixel define을 켠 앱 hash와 설치 앱 hash가 일치한다. |
| 30 | 문서가 제한 slice를 전체 C11.4로 승격하거나 이미지 링크가 없는 캡처를 가리키는가? | STATUS checkbox와 `72/213`을 유지하고 README/roadmap은 제한을 쓴다. Android/iOS 대표 PNG와 run evidence 경로가 실제로 존재한다. |

**결과:** 차단할 diff 결함은 남기지 않았다. 0px maximum repeat-count behavior는 oracle이 없는 경계로 남겨 지원 주장에 포함하지 않았다. 이 후보의 전체 한계와 source/run hash는 [runtime 근거](./README.md), 별도 코드/runtime 검토는 [implementation review 20](./implementation-review-20.md)에 있다. PR required checks, 외부 승인 및 merge 결과는 PR 생성 후 별도로 확인한다.
