# C11.4 두 개 초과 fixed-repeat 계획 검토 · 실패 관점 20개

**기준:** [계획](./c11-4-unbounded-fixed-repeat-groups.md), CSS Grid 2025 CRD §7.2.3.1, 독립 산술, Chrome 156 Agent Browser 관찰. 계획-only 검토이며 Spinon 코드/runtime 결과는 포함하지 않는다.

| # | 독립 실패 관점 | 검토 결과와 통제 |
| --- | --- | --- |
| 1 | 규범이 여러 fixed-repeat과 auto-repeat의 공존을 막을 수 있음 | CRD §7.2.3.1은 auto-repeat 한 개와 같은 list 내 fixed-repeat 그룹을 함께 허용한다. |
| 2 | “임의 개수”가 메모리 무제한 입력으로 읽힐 수 있음 | 계획은 expanded descriptor 10,000개 한도, checked arithmetic과 fallible reserve 안에서만 허용한다. |
| 3 | 각 list component가 최소 한 track을 만든다는 가정이 깨질 수 있음 | 구현 전 pattern 비어 있음, count 0, parser rejection을 확인하고 잘못된 component는 fail closed한다. |
| 4 | track-list index와 dynamic metadata slot이 어긋날 수 있음 | metadata를 authored component index로 주소 지정하고 3·4·8 group order assertions를 둔다. |
| 5 | group 증가로 매 component마다 모든 group을 훑어 O(n²)이 될 수 있음 | index lookup 또는 단일 순회 방식으로 접근 비용을 선형에 묶고 8 group case를 검사한다. |
| 6 | dynamic metadata allocation이 OOM에서 panic/abort할 수 있음 | `try_reserve` 계열 경로와 별도 allocation fault-injection을 통과 조건으로 둔다. |
| 7 | allocation 실패 뒤 다음 cascade가 오염될 수 있음 | 오류가 node/property에 연결되고 guard 뒤 동일 fixture의 후속 계산이 성공하는지 확인한다. |
| 8 | count 변환에서 0·음수·플랫폼 usize overflow가 통과할 수 있음 | `usize::try_from` 및 positive-count 검사를 각 group에 유지하고 parser와 adapter rejection을 구분한다. |
| 9 | `count × pattern length` 또는 group 합산에서 정수 overflow가 생길 수 있음 | 각 곱·합을 checked arithmetic으로 보호하고 contextual unsupported error를 확인한다. |
| 10 | 10,000 경계에 ordinary/auto-pattern tracks가 누락될 수 있음 | expanded fixed groups뿐 아니라 보통 track과 auto-repeat pattern을 합한 authored descriptor를 검사한다. |
| 11 | 10,000개 list component 전 예약이 명세 허용값을 잘못 배제할 수 있음 | valid 입력의 최소 descriptor 비용을 기준으로 선제 cap을 정당화하고 경계 양쪽을 테스트한다. |
| 12 | fixed pattern의 2개 track 내부 순서가 섞일 수 있음 | 8개 group input 중 다중 pattern과 17px/19px distinct breadth를 두어 전체 배열을 비교한다. |
| 13 | auto-repeat 앞·중간·뒤 group의 source order가 바뀔 수 있음 | 네 group prefix/split precomparison과 Rust expected track list를 대조한다. |
| 14 | auto-repeat `start/end`가 원래 AST index를 가리킬 수 있음 | expanded track count를 기준으로 half-open range를 계산하고 split scene의 item 위치를 확인한다. |
| 15 | auto-fill count가 fixed breadth 또는 gap을 빠뜨릴 수 있음 | 독립식 `163 + 28n`과 5회 303px/6회 331px, Chrome CSSOM을 함께 고정한다. |
| 16 | row 축 검증이 column 축과 같은 basis/placement를 혼동할 수 있음 | definite block-size 320px row scene 하나를 두고 item `(0,74,10,10)`을 별도 비교한다. |
| 17 | existing auto-fit collapse가 신규 bookkeeping으로 바뀔 수 있음 | 기존 10개 scene을 함께 회귀 실행하고 auto-fit input의 기존 expected track/collapse를 보존한다. |
| 18 | named lines나 두 번째 auto-repeat이 새 동적 경로를 우회할 수 있음 | line-name metadata 및 duplicate auto-repeat은 계속 negative control로 둔다. |
| 19 | 내부 fixture가 기본 제품 바이너리에 들어갈 수 있음 | default artifact negative control과 opt-in feature positive control을 별도로 비교한다. |
| 20 | 제한 구현이 C11.4 완료나 체크 수 증가로 과장될 수 있음 | 계획, STATUS, contract와 roadmap에 미완료 범위 및 `72/213` 유지 조건을 명시한다. |

## 결론

규범이 허용하는 복수 fixed-repeat을 기존 descriptor/metadata 한도 안에서 단계적으로 다룬다. 계획은 allocation, 시간복잡도, order, auto-repeat rebasing, row axis, CSSOM oracle와 제품 범위 경계를 구현 전에 고정했다. 계획 gate를 통과시킬 수 있다.
