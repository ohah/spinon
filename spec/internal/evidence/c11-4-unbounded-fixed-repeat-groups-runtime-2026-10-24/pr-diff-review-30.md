# C11.4 fixed-repeat 그룹 수 제한 제거 · PR diff 실패 관점 30개

이 검토는 계획·구현/runtime review와 별도로 현재 PR diff의 코드, Android/iOS fixture adapter, capture 도구, 계약·상태·공개 roadmap 변경을 확인한다. 각 행은 하나의 독립 실패 경로를 판정한다.

| # | 검토 관점 | 판정 |
| ---: | --- | --- |
| 1 | 변경 범위에 무관한 renderer/layout 알고리즘이 함께 바뀌는가 | cascade admission과 opt-in scene validator에 한정 |
| 2 | 기존 `[None; 2]` 고정 용량 접근이 남아 있는가 | 제거됨 |
| 3 | group counter의 type·overflow 검사가 target-width 의존적인가 | `usize`와 checked increment 사용 |
| 4 | invalid repeat count가 narrowing conversion 뒤 양수로 alias되는가 | 변환 성공 후 `>0` 필터 |
| 5 | 두 번째 auto-repeat guard가 count 유형별로 누락되는가 | AutoFill/AutoFit 모두 같은 index guard 적용 |
| 6 | fixed metadata가 auto-repeat component를 잘못 분류하는가 | `RepeatCount::Number`만 metadata slot 생성 |
| 7 | list component 수 상한보다 먼저 dynamic allocation하는가 | 초기 validation에서 10,000 cap 적용 |
| 8 | metadata `Vec` allocation을 `resize_with`가 대신해 OOM panic을 유발하는가 | resize 전에 fallible reserve 성공 요구 |
| 9 | failpoint 이름이 실제 reserve call site와 일치하는가 | `FixedRepeatGroups`가 reservation site에 연결됨 |
| 10 | metadata 인덱스가 authored component index와 다른 단위인가 | 저장·조회 모두 `list.values` index |
| 11 | metadata 누락이 `unwrap` panic으로 전파되는가 | 모든 lookup은 `Option`을 contextual error로 변환 |
| 12 | empty 또는 3-track fixed pattern이 새 경로로 허용되는가 | 거부 test 유지 |
| 13 | named-line metadata 계산이 아직 지원되지 않는데 조용히 폐기되는가 | 지원 전에 line names를 명시 거부 |
| 14 | computed track에 NaN·infinity·negative breadth가 새로 수용되는가 | `is_finite && >= 0` gate 적용 |
| 15 | expanded count 곱셈 overflow가 saturating/wrapping 되는가 | `checked_mul` 사용 |
| 16 | 각 component total overflow/cap이 final expansion 이후에만 확인되는가 | expansion 전 checked total 계산 |
| 17 | max descriptor 초과에서 typed partial tracks가 노출되는가 | cascade 오류로 종료; 10,001 regression test |
| 18 | line metadata byte limit 검사가 새 list layout에서 사라지는가 | 기존 byte cap과 checked byte arithmetic 보존 |
| 19 | no-fixed-group auto-repeat path에 빈 vector lookup 회귀가 생기는가 | auto-repeat index 분기 유지; 기존 전체 style tests 통과 |
| 20 | group 확장이 auto-repeat prefix/suffix order를 바꾸는가 | 기존 2-group·신규 3/4/8-group cascade tests 통과 |
| 21 | auto-repeat index expansion으로 auto-fit collapse 범위가 밀리는가 | fill/fit-empty/occupied runtime 장면에서 checked |
| 22 | columns/rows cross-axis mixed profile이 무심코 넓어지는가 | 기존 one-axis guard 유지 |
| 23 | template-area admission이 fixed group 추가만으로 바뀌는가 | areas guard와 regression test 유지 |
| 24 | Android/iOS scene 수·IDs·oracle IDs가 서로 불일치하는가 | JSON 12개 unique; Rust matrix·boundary test·capture catalog 일치 |
| 25 | 4-group prefix root 높이 80px가 기존 320px validator에 오인되는가 | 양 플랫폼 root/frame/pixel 기대값에서 별도 처리 |
| 26 | row split item의 `grid-row:5/6` frame이 fixture helper에서 빠지는가 | Android/iOS expected node frame `(0,74,10,10)` |
| 27 | Android PixelCopy가 item 수·center coordinate를 잘못 검사하는가 | four-row split 2 samples, item center `(5,79)`로 검사 |
| 28 | iOS Metal sample이 root y=40 또는 row item 표본을 놓치는가 | 4-prefix root `(160,40)`, row item `(5,79)` 검증 |
| 29 | supplemental Chrome observation이 기존 10-scene immutable oracle를 덮어쓰는가 | 별도 observation source를 병합; 원본 10개 evidence는 수정하지 않음 |
| 30 | HTML/STATUS/docs가 slice를 전체 C11.4 또는 공식 완료로 표시하는가 | C11.4 checkbox와 `72/213` 미변경; 미지원 범위 명시 |

검토 결과 수정해야 할 diff defect는 남아 있지 않다. 검증 기록은 [runtime README](./README.md), 구현/runtime 관점은 [별도 review](./implementation-runtime-review-20.md)에 있다.
