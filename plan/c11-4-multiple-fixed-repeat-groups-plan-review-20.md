# C11.4 multiple fixed-repeat groups 계획 검토 · 실패 관점 20개

**검토 기준:** [계획](./c11-4-multiple-fixed-repeat-groups.md), CSS Grid 2025 CRD §7.2.3.1–§7.2.3.2, 독립 계산, Chrome 156 사전 관찰. 계획-only 검토이며 Spinon 코드·runtime 결과를 포함하지 않는다.

| # | 독립 실패 관점 | 검토 결과와 통제 |
| --- | --- | --- |
| 1 | 규범이 fixed-repeat 두 개와 auto-repeat의 공존을 금지할 수 있음 | W3C CRD §7.2.3.1은 auto-repeat 하나와 fixed-repeat 그룹들을 같은 list에 허용한다고 확인했다. |
| 2 | CSS grammar가 셋째 fixed-repeat도 허용하므로 제한 profile이 전체 표준처럼 읽힐 수 있음 | 계획은 정확히 두 fixed groups까지만 새로 허용하고 세 번째는 거부·범위 밖으로 고정한다. |
| 3 | Stylo computed AST가 integer repeat을 다른 variant/순서로 정규화할 수 있음 | 구현 전에 runtime cascade에서 count·pattern·line metadata를 다시 검사하고 어긋나면 코드를 쓰지 않는다. |
| 4 | fixed-repeat count가 0, 음수, `usize` 변환 불가일 수 있음 | 양의 정수 변환을 각 group별 검사하고 CSS parser rejection과 adapter rejection을 구분한다. |
| 5 | 각 group의 pattern size가 비어 있거나 3개 이상일 수 있음 | 1~2개의 finite nonnegative computed `LengthPx`만 허용하고 나머지는 거부한다. |
| 6 | 두 그룹 전개에서 count×pattern overflow 또는 합계 overflow가 생길 수 있음 | 각 곱과 total에 checked arithmetic, 10,000 descriptor 한도를 적용한다. |
| 7 | 첫 group 확장 뒤 두 번째 group이 누락되거나 중복될 수 있음 | prefix·split·suffix·reversed 관찰을 authored order oracle로 두고 track vector를 전량 비교한다. |
| 8 | auto-repeat index가 원래 AST component index에 남을 수 있음 | 확장된 track 수 기준 `start/end`를 만들고 prefix/split/suffix scenes에서 검사한다. |
| 9 | 2×20px와 3×10px의 breadth 또는 track 사이 gap이 auto-fill count에서 빠질 수 있음 | 독립 수식 `126 + 38n`으로 `n=5`와 `n=6` 경계를 각각 316/354px로 비교한다. |
| 10 | 두 track짜리 첫 pattern의 breadth가 계산에서 일부 빠질 수 있음 | `repeat(2,20px 12px)` positive control은 fixed breadth 64px와 네 expanded tracks를 별도로 확인한다. |
| 11 | fixed group의 반복 순서와 반복 횟수가 뒤바뀔 수 있음 | 서로 다른 count·size인 2×20px 및 3×10px group을 반전한 Chrome scene과 대조한다. |
| 12 | columns test가 rows에 inline-size를 적용할 수 있음 | rows matrix는 80×320 grid의 definite block axis를 독립 사용한다. 한 scene의 양축 확장은 제외한다. |
| 13 | auto-fit empty track collapse가 count 산정보다 먼저 적용될 수 있음 | empty scene은 먼저 5회 산정 후 0px×5 결과를 확인한다. |
| 14 | occupied auto-fit에서 item이 prefix fixed tracks에 놓이거나 경계 gutter가 잘못 접힐 수 있음 | CSS line 6의 item이 Chrome에서 x=110px/y=110px에 놓이고 used repeated track이 186px인 별도 scene을 고정한다. |
| 15 | empty line metadata가 expanded track 수와 `N+1` 관계를 잃을 수 있음 | 모든 line names는 비어 있어도 outer/inner group vector 길이를 cascade·projection 검사로 확인한다. |
| 16 | named line 또는 세 번째 group이 일부 성공 후 잘린 결과를 반환할 수 있음 | named lines·세 번째 fixed repeat·두 번째 auto-repeat을 negative controls로 유지해 contextual error와 결과 부재를 검사한다. |
| 17 | allocation fault가 tracks 일부를 결과에 남기거나 다음 계산까지 오염시킬 수 있음 | fallible reserve와 fault injection에서 no partial result 및 다음 호출 recovery를 확인한다. |
| 18 | opt-in fixture가 default app artifact에 들어갈 수 있음 | default product negative control과 fixture-test positive control을 각각 빌드해 IDs/symbols/payload를 비교한다. |
| 19 | Chrome CSSOM 결과가 규범 기대 또는 제품 지원 결과로 과대 해석될 수 있음 | 별도 산술을 기준값으로 두고 Chrome은 pinned environment의 관찰로만 사용한다. PR 설명에도 WPT·일반 지원이 아님을 쓴다. |
| 20 | 제한 slice가 공식 C11.4 completion과 72/213 집계를 부당하게 승격할 수 있음 | 계획, status, implementation, runtime review 및 roadmap에서 상위 checkbox를 미완료로 둔다. |

## 결론

계획은 fixed-repeat 두 그룹으로 범위를 제한하고 author order, count 산식, collapse, parser/resource/allocation 오류와 artifact 경계를 구현 전에 다룬다. Chrome 10 scenes는 별도 evidence에 있고 구현/runtime은 아직 실행하지 않았다. 계획 관문을 통과시킬 수 있다.
