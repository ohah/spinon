# C11.4 후속 fit-content 계획 PR · diff 실패 관점 검토 30개

검토 대상은 계획 PR의 실제 변경분이다: 상위 계획 갱신, 후속 계획과 계획 검토 20개, STATUS 한 줄, Chrome 사전 비교 fixture·관측·screenshot·verifier다. 제품 Rust 코드 변경은 없다. 이 검토는 계획 실패 관점 20개를 대체하지 않고, 구현/runtime 검토도 미리 통과 처리하지 않는다.

| # | diff 실패 관점 | 확인한 diff·근거 | 판정 |
|---:|---|---|---|
| 1 | 계획 PR에서 제품 동작을 이미 구현한 것처럼 보임 | 변경 파일은 계획·상태·CSS 비교 fixture와 evidence이며 제품 Rust 경로는 변경하지 않았다. STATUS에도 제품 구현 전이라고 썼다. | 통과 |
| 2 | 이전 작업 기준점이 잘못되거나 미병합 상태 | 상위 계획은 PR #1295의 Rebase and merge 시각과 merge SHA `40b416b4de4942c9251db0f4862626443310d092`를 기록한다. | 통과 |
| 3 | 선행 slice 범위를 새 범위와 혼동 | 후속 계획은 #1295에서 추가된 반복 direct-text span profile을 선행 계약으로 링크하고 동일 topology를 전제로 한다. | 통과 |
| 4 | 다음 후보가 상위 계획과 STATUS에서 달라짐 | 상위 계획, 새 계획, STATUS 모두 fit-content position 2·3의 60개 order라는 같은 후보를 가리킨다. | 통과 |
| 5 | 계획을 고정하기 전에 구현을 시작 | 상위 계획은 계획 PR merge 전 제품 변경 금지를 명시하고 새 계획도 계획·구현 검토 관문을 분리한다. | 통과 |
| 6 | 공식 완료 수를 후보 선택만으로 올림 | 상위 계획·새 계획·STATUS 모두 공식 `78/214`와 C11.4/C11 미완료 상태를 유지한다. | 통과 |
| 7 | STATUS의 C11.4 완료 체크가 바뀜 | diff는 미완료 C11.4 항목 아래 후보 설명 한 줄만 추가하며 체크박스를 완료로 바꾸지 않는다. | 통과 |
| 8 | 새 계획이 기존 계획 파일을 덮거나 제거 | 기존 plan 파일은 보존되고 새 날짜·범위의 후속 계획과 검토 파일이 추가된다. | 통과 |
| 9 | 기존 position-1 order를 새 후보가 대체 | 새 계획은 기존 30개 position-1 order를 유지하고 position 2·3에 60개만 추가한다고 고정한다. | 통과 |
| 10 | fit-content 이외 member 수나 multiplicity 변경 | 목록은 fit-content 1개, min-content 2개, max-content 2개, auto 1개로 고정된다. | 통과 |
| 11 | 반복 tail 순서 수를 잘못 계산 | 다섯 member multiset의 고유 순서는 30개이며 독립 verifier가 두 위치마다 이를 검사한다. | 통과 |
| 12 | matrix 곱집합에서 조합 축 누락 | 위치 2개 × tail 30개 × cap 4개 × alignment 2개 × topology 4개로 총 1,920 scene이다. | 통과 |
| 13 | fit-content 위치별 표본 불균형을 숨김 | 관측 요약과 verifier는 위치 2·3 각각 960 scene을 확인한다. | 통과 |
| 14 | topology별 표본 누락 | 관측 요약과 verifier는 네 topology별 480 scene을 확인한다. | 통과 |
| 15 | percentage cap basis가 viewport나 track으로 바뀜 | 계획과 evidence는 definite 220px content block basis를 고정하고 20%·0%·120%를 44·0·264px로 기록한다. | 통과 |
| 16 | 120% cap을 임의로 clamp | 계획은 120%를 별도 authored cap으로 두며 100% clamp를 기대하지 않는다. | 통과 |
| 17 | 0% cap을 min-content floor까지의 최종 상한으로 취급 | 계획과 verifier 설명은 fit-content minimum floor가 cap보다 클 수 있음을 명시하며 used size ≤ cap 검사를 하지 않는다. | 통과 |
| 18 | explicit row가 implicit list cycle에 포함 | explicit 10px row와 그 뒤의 세 implicit row를 구분하고 관측·검증에서 4개 used row를 검사한다. | 통과 |
| 19 | 새 위치가 span sizing에 실제로 기여하지 않음 | topology는 첫 세 implicit row를 span-2 또는 span-3으로 덮어 두 번째·세 번째 위치가 sizing 입력에 나타난다. | 통과 |
| 20 | implicit column 생성으로 row 결과가 오염 | 모든 item의 column을 1 / 2로 제한하고 계획·evidence는 40px payload 폭과 고정 x 위치를 확인한다. | 통과 |
| 21 | source-order permutation이 semantic frame을 변경 | 독립 관측은 forward/reversed 의미상 대응 frame 차이의 최댓값 0 CSS px를 기록한다. | 통과 |
| 22 | span item frame이 차지한 row track 합과 불일치 | verifier 실행 결과 span frame 대 used-row 합 최대 차이는 0 CSS px다. | 통과 |
| 23 | stretch가 fit/min/max track까지 늘어남 | 검증 관계는 사용된 auto track만 stretch되며 non-auto used row 크기는 유지됨을 비교한다. | 통과 |
| 24 | browser 값을 규범 기대값으로 둔갑 | 계획·README는 CSS Grid 2025 CRD의 sizing/alignment 조항을 독립 관계 근거로 두고 Chrome 값은 교차 확인으로 한정한다. | 통과 |
| 25 | Chrome fixture를 Spinon runtime 성공 근거로 오인 | README는 비교가 Spinon layout/runtime을 사용하지 않았다고 밝히고 runtime·플랫폼 검증은 계획에 남긴다. | 통과 |
| 26 | observation 수나 pinned input hash가 맞지 않음 | NDJSON은 header 포함 1,921 lines이며 fixture SHA `faefbffa75dad33af7fc32cc193fb4741c6c4cb761dda61c89e5e1bc886b96d4`, observation SHA `12fa49a03436604477e121335335d1bc14b3afded957a921206addfaba0f3d90`가 실제 파일과 verifier 요약에서 일치한다. | 통과 |
| 27 | 결과 screenshot이 없거나 evidence에서 찾을 수 없음 | 1280×633 Chrome 157 screenshot을 추가하고 README에서 상대 경로로 삽입한다. PNG SHA는 `6e54c8854af7d0a275225f12c724ae626b4eb1b39c5c3539aff8dae32c079676`다. | 통과 |
| 28 | runtime rejection·회복 경계가 계획에서 빠짐 | 계획은 position 4–6, malformed list/cap, 잘못된 span·placement, stale revision과 atomic rejection 후 valid recovery를 검증 관문에 둔다. | 통과 |
| 29 | cross-platform 계획이 fixture 전용 우회나 pixel readback을 약속 | 계획은 Android/iOS 기본 Root HTML route smoke를 요구하면서 fixture FFI 우회를 금지하고 OS screenshot을 PixelCopy/Metal readback으로 부르지 않는다. | 통과 |
| 30 | diff 링크·형식 또는 계획 검토 결과가 깨짐 | 새 계획·계획 검토·README의 상대 링크를 확인했고 `git diff --cached --check` 및 Chrome matrix verifier가 통과했다. 별도 계획 실패 관점 검토 20개도 포함되어 있다. | 통과 |

**결과:** 계획 PR diff의 서로 다른 30개 실패 관점을 검토했다. 이 제한 계획 후보를 진행하는 데 막히는 diff 결함은 발견되지 않았다. 이는 제품 코드 검증, 플랫폼 smoke, CI 또는 PR merge 결과를 뜻하지 않는다.
