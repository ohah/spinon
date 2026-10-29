# Auto-repeat 상한 계획 PR diff 검토 · 실패 관점 30개

검토 범위: C11.4 상위 계획의 Stage C 문구, 새 경계 계획, 계획의 20-angle 검토. 이 PR은 계획 문서만 바꾸며 구현·실행 결과를 포함하지 않았다.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | Stage C가 기존 상태를 완료로 올리는가 | 아니다. `부분 제한 구현`을 유지하고 Stage B/C 완료를 부정한다. |
| 2 | C11.4 상위 상태를 완료로 바꾸는가 | 아니다. 계획은 전체 C11.4 완료와 무관하다고 썼다. |
| 3 | authored track 10,000개 거부와 auto-repeat 확장을 혼동하는가 | 아니다. 기존 검사와 새 관측 대상을 구분했다. |
| 4 | 고정 dependency version이 모호한가 | 아니다. vendored Taffy 0.15.0으로 한정했다. |
| 5 | 9,999px 기대값이 상한보다 하나 어긋나는가 | 아니다. 1px track·0 gap·definite basis에서 9,999를 기대한다. |
| 6 | 10,000 기대값에 gutter 수를 포함하는가 | 아니다. non-gutter track 수를 비교하게 했다. |
| 7 | 10,001을 authored track 입력으로 구성하는가 | 아니다. 한 개 반복 pattern과 definite basis로 clamp를 본다. |
| 8 | 10,001 입력의 기대값을 오류로 정하는가 | 아니다. Taffy 0.15.0의 10,000 expanded-track cap을 성공 결과로 둔다. |
| 9 | Columns만 검증하는가 | 아니다. 세 크기와 두 mode를 양 축에서 각각 실행한다. |
| 10 | Rows의 block basis가 indefinite일 수 있는가 | 선택 축의 root size를 definite로 설정하도록 요구한다. |
| 11 | auto-fill과 auto-fit의 track 개수 차이를 잘못 가정하는가 | 아니다. 개수는 같고 빈 auto-fit collapse만 다르게 본다. |
| 12 | auto-fit 검사에 item occupancy가 섞이는가 | 아니다. root children과 item map을 비워둔다. |
| 13 | auto-fill track의 collapse 여부를 놓치는가 | 아니다. 모든 반복 track의 mode별 collapse flag를 확인한다. |
| 14 | target Initialize stage가 중복되는 상황을 놓치는가 | 아니다. 마지막 sizing pass에 stage가 하나인지 요구한다. |
| 15 | trace가 예산을 넘은 partial 결과를 성공으로 취급하는가 | 아니다. trace 실패 부재를 성공 조건에 둔다. |
| 16 | 두 축을 함께 10,000개로 확장해 100,000 예산을 넘기는가 | 아니다. 최대 축을 따로 실행하고 나머지는 track 하나로 둔다. |
| 17 | trace budget을 임의 값으로 인용하는가 | 아니다. vendored implementation의 100,000-state 제한을 근거로 든다. |
| 18 | prefix/suffix control이 cap 전에 이미 suffix를 잃는가 | 아니다. neighbor 5px를 빼고 10,003px fitting·10,004px cap을 정했다. |
| 19 | fitting control의 전체 개수가 10,000인가 | 예. 2px prefix + 9,998 × 1px repeat + 3px suffix다. |
| 20 | cap control의 전체 개수가 10,000인가 | 예. prefix 다음 9,999개 반복으로 채우고 suffix를 자른다. |
| 21 | source-order 관측이 같은 크기에 의존하는가 | 아니다. prefix/repeat/suffix 크기는 2px/1px/3px다. |
| 22 | suffix만 보고 prefix 보존을 놓치는가 | 아니다. 두 입력 모두 첫 track도 확인한다. |
| 23 | gap이나 overflow가 경계 계산에 섞이는가 | 아니다. gap 0, 유한한 양수 basis로 고정했다. |
| 24 | 계획이 지원되지 않는 grammar를 암묵적으로 범위에 넣는가 | 아니다. multi-track, named line, placement/span, intrinsic/percentage를 제외한다. |
| 25 | 10,000 track 전체 layout의 성공을 보장한다고 읽히는가 | 아니다. trace budget과 Taffy 내부 allocation 한계를 남겼다. |
| 26 | 시뮬레이터나 screenshot을 실행한 것처럼 적었는가 | 아니다. Rust unit-test slice이며 시각 자료는 해당 없음이다. |
| 27 | test-only 변경이 production ABI에 들어가는가 | 아니다. `#[cfg(test)]` layout test만 지시한다. |
| 28 | 계획 추가만으로 공식 상태 대장을 완료 처리하는가 | 아니다. C11.4와 C11을 미완료로 둔다. |
| 29 | evidence 산출 경로가 정해져 있는가 | 예. 날짜가 붙은 전용 evidence directory를 지정했다. |
| 30 | 완료 관문이 테스트·검토를 건너뛰는가 | 아니다. focused Grid tests, fmt/diff, 구현 리뷰 20과 PR diff 리뷰 30을 요구한다. |

**판정:** 30개 관점에서 계획 diff는 범위와 기대값이 일관된다. 이 PR에서 기능 구현이나 runtime 검증을 했다는 주장은 하지 않는다.
