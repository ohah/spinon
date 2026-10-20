# C11.4 auto-repeat 계획 정정 재검토 20개

**대상:** 수정한 auto-repeat/leading implicit auto-track 계획과 code-path 근거. 초기 계획 검토 20개의 잘못된 dispatch 가설을 재사용하지 않고, 변경된 실패 경계와 해결 방법을 검토했다.

**후속 정정:** 이 검토는 Taffy의 post-expansion line 처리만 확인하고 Spinon이 양축 repeat의 secondary axis를 `resolve_axis`로 먼저 처리하는 경로를 놓쳤다. 판정은 superseded이며 [새 축 분류 계획 검토 20개](./plan-axis-classification-review-20.md)로 대체한다.

| # | 새 실패 관점 | 확인 및 반영 |
|---:|---|---|
| 1 | 일반 track-sizing normalizer를 auto-repeat 입력의 경로로 착각하는 경우 | dense dispatcher의 auto-repeat 분기와 일반 normalizer 분기를 나눠 문서에 적었다. 초기 가설은 superseded로 표시했다. 통과. |
| 2 | Taffy가 line을 먼저 받고 repeat count를 나중에 계산한다고 가정하는 경우 | `compute_explicit_grid_size_in_axis` 다음 `into_origin_zero(explicit_count)`, 그 뒤 placement 순서를 vendor source에서 확인했다. 통과. |
| 3 | 이미 처리되는 negative line을 새 resolver로 다시 normalize해 line index를 이중 이동하는 경우 | 새 count resolver를 제외하고 Taffy의 기존 post-expansion line 경로를 유지하도록 계획을 수정했다. 통과. |
| 4 | CSS projection이 auto-repeat `GridAutoRepeat` metadata를 잃어 repeat count가 계산되지 않는 경우 | Stylo projection에서 metadata 보존, adapter에서 Taffy Repeat 전달을 그대로 둔다. 변경 대상은 auto-track sizing 전달이다. 통과. |
| 5 | `grid-auto-*` TrackList가 현재 지원되어 있다고 가정하는 경우 | auto-repeat dispatcher가 해당 list를 명시적으로 거부하는 오류와 두 축을 기록했다. 통과. |
| 6 | TrackList 거부문만 지우면 sizing list가 Taffy에 자동 전달된다고 착각하는 경우 | `to_taffy_style`에 `grid_auto_rows/columns` 대입이 없음을 별도 구현 경계로 명시했다. 통과. |
| 7 | auto-repeat 사용 시 단일 `LengthPx` implicit sizing도 보존된다고 가정하는 경우 | Taffy Style의 auto track field가 어댑터에서 설정되지 않는 정적 후보를 기록하고 scalar control을 요구한다. 통과. |
| 8 | `Auto` 기본값을 빈 track list와 달리 명시해 기존 동작을 바꾸는 경우 | `Auto`를 Taffy 기본 control로 유지하고 fixed scalar/list만 연결하도록 범위를 제한했다. 통과. |
| 9 | 음수 implicit track에서 `[31,47]`을 왼쪽부터 그대로 선택해 순서가 반대가 되는 경우 | Taffy의 `offset = len - leading_count % len` 동작과 `[47]`, `[31,47]` 결과를 회귀 기대값으로 고정한다. 통과. |
| 10 | implicit track 수가 list 길이의 배수일 때 cycle 시작점이 어긋나는 경우 | `leading_count % list_len == 0` case를 포함하도록 반복 경계 테스트를 적었다. 통과. |
| 11 | positive/trailing implicit cycle에 leading 뒤집기 규칙을 적용하는 경우 | positive-side control은 전방 순환으로 별도 검사한다. 통과. |
| 12 | auto-fit이 repeated count 계산 전에 empty slots를 제외하는 경우 | 기존 auto-fill과 같은 count 뒤 occupancy에 따른 collapse로 검증 순서를 유지한다. 통과. |
| 13 | collapsed track의 empty gutter 차이를 Taffy 또는 Chrome 하나로 덮어쓰는 경우 | 고정 CRD를 normative oracle로 유지하고 알려진 Chrome 155 gutter 관찰을 진단으로 분리한다. 통과. |
| 14 | 두 축 auto-repeat에서 row list가 column list 또는 count를 덮어쓰는 경우 | 각 축에 독립된 `grid_auto_rows/columns` field와 서로 다른 pattern/count fixture를 요구한다. 통과. |
| 15 | track list에 px 아닌 문법이 우연히 통과해 지원 범위가 넓어지는 경우 | Auto, finite non-negative px scalar, non-empty finite non-negative px list로 지원 후보를 제한한다. 통과. |
| 16 | line 0, 정수 범위 초과, reversed pair, span arithmetic이 Taffy conversion을 우회하는 경우 | 기존 input validation을 유지하고 각 잘못된 line/span 및 cap 회귀를 구현 범위에 넣었다. 통과. |
| 17 | hidden/out-of-flow item이 auto-fit occupancy에 영향을 주는 경우 | visible-item filtering 및 out-of-flow exclusion을 regression으로 확인하도록 범위를 적었다. 통과. |
| 18 | `grid-auto-*` list를 계산한 implicit vector와 Taffy style에 동시에 넣어 track을 중복 생성하는 경우 | auto-repeat 경로에서는 authored Repeat template 유지와 implicit auto-track field 전달을 역할별로 분리한다. 중복 template expansion은 하지 않는다. 통과. |
| 19 | static source 분석을 Spinon runtime failure로 오인해 PR 본문이나 상태 대장에 과장하는 경우 | 현재 runtime failure는 미재현이며 구현 후 oracle 비교를 한다고 모든 문서에 명시했다. 통과. |
| 20 | 정정으로 C11.4 완료 상태 또는 공식 지원 수가 바뀌는 경우 | 계획/evidence만 수정하고 `spec/STATUS.md`, public contract status, 완료 수는 유지한다. 통과. |

## 판정

정정된 구현 경계는 현재 소스 경로와 일치한다. 구현 가설은 Taffy auto-track adapter와 leading implicit cycle 결합이며, numeric negative line은 Taffy 기존 경로에 대한 regression 대상으로 남긴다. Spinon runtime 및 simulator 결과는 아직 없다.
