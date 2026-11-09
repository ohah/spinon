# C11.4 두 `fit-content()` 계획 PR · 변경 diff 실패 관점 30개

검토 대상은 이 PR의 전체 staged diff다: 새 집중 계획과 계획 검토, Chrome fixture·collector·3,840 observation·verification·screenshot, 상태 대장, 계획 인덱스, 공개 roadmap heading. 제품 layout 코드는 변경하지 않는다. 계획 실패 관점 20개 검토와 다른 변경 누락·범위·표현 위험을 확인한다.

| # | diff 공격 관점 | 확인할 실패 | 검토 결과 | 판정 |
|---:|---|---|---|---|
| 1 | 계획 단계 표시 | 제품 구현이나 완료 상태를 앞당겨 기록 | 집중 계획·STATUS·roadmap 모두 구현 전이라고 표시 | 통과 |
| 2 | 공식 집계 오염 | 제한 후보가 공식 완료 수를 변경 | `78/214`, C11.4/C11 미완료, 계약 `0.1.0` 유지 | 통과 |
| 3 | 상태 대장 상위 항목 체크 | C11.4 checkbox가 실수로 완료됨 | checkbox 변경 없이 next-candidate 메모만 추가 | 통과 |
| 4 | 공개 roadmap 오래된 후보 문구 | 후보 미선정 상태가 남음 | 제목을 계획 링크와 3,840 scene, 구현 전으로 갱신하고 Agent Browser 데스크톱·390px 모바일에서 문서 가로 넘침 없음 확인 | 통과 |
| 5 | plan index 단절 | 집중 계획이 순차 작업 기록에서 빠짐 | track-sizing 계획에 같은 제목과 evidence link 추가 | 통과 |
| 6 | 변경 파일 오염 | 관련 없는 worktree 산출물이 PR에 포함 | staged 경로는 계획·상태·roadmap 및 이 사전 비교의 fixture/evidence로 한정 | 통과 |
| 7 | 제품 코드 혼입 | 사전 비교 fixture가 런타임 코드·FFI로 연결 | production source·FFI·ABI 변경 없음 | 통과 |
| 8 | 한 fit-content를 두 개로 잘못 설명 | 새 profile이 기존 한 개 제한과 구분되지 않음 | 두 fit member·기존 1–6 위치 선행 scope를 분명히 표시 | 통과 |
| 9 | track multiplicity drift | `min/max/auto` member 수가 다르게 전파됨 | `fit×2, min×2, max×1, auto×1`을 문서와 fixture에서 고정 | 통과 |
| 10 | position pair 산식 오류 | 위치 pair 15개가 과소/과대 계산됨 | 6개 위치에서 서로 다른 두 위치 조합 15개를 생성 | 통과 |
| 11 | 고유 track-list count 오류 | authored list 수가 scene 수와 혼동 | 480 unique lists와 3,840 scenes를 별도 기록 | 통과 |
| 12 | cap pair 숫자 누락 | 네 cap 표본 조합 또는 assignment 하나가 빠짐 | 네 cap pair와 양방향 assignment 8개를 기록 | 통과 |
| 13 | percentage basis 불명 | percentage 결과를 임의 px로 사용 | definite 220px basis와 20%=44px, 120%=264px를 기록 | 통과 |
| 14 | 모든 permutation 주장 | 네 tail order를 전체 허용으로 오해 | 네 order만 허용하고 전체 순열을 제외한다고 명시 | 통과 |
| 15 | implicit row index 오프셋 | explicit row와 auto-row index가 섞임 | explicit row 1개와 여섯 implicit row를 분리 | 통과 |
| 16 | implicit column 누락 | auto column이 생겨 sizing 결과가 혼합 | fixture item을 모두 `grid-column:1 / 2`로 고정 | 통과 |
| 17 | topology 집합 drift | 2 topology와 reverse control이 하나씩 빠짐 | 두 topology × forward/reverse를 fixture와 verification에 기록 | 통과 |
| 18 | content/style 변경 | cap 결과가 text/font 변형으로 오염 | 기존 제한 payload grammar와 monospace 10/10 유지 | 통과 |
| 19 | zero-size track 오표기 | 모든 implicit track이 양수라고 주장 | observation에서 0px 허용 사실을 계획과 README에 명시 | 통과 |
| 20 | frame sum 대리 기준 | item y 차를 track size 기준으로 사용 | 점유한 used row 합과 item frame을 직접 비교 | 통과 |
| 21 | source-order 검사 축소 | item frame만 비교하고 text payload 누락 | item·payload·line rect와 row geometry를 비교 | 통과 |
| 22 | stretch 분배 과장 | auto track이 여러 개인 듯 균등 분배 주장 | 목록에 auto가 하나라 해당 track만 늘고 multi-auto는 범위 밖 | 통과 |
| 23 | cap 민감도 과대 해석 | 특정 관측을 CSS 보편 법칙으로 기록 | 300/480 grouping을 oracle 표본으로 한정하고 min floor 예외 명시 | 통과 |
| 24 | collector cleanup 실패 | browser 세션이 캡처 후 남음 | `finally` close, lifecycle 기록, 별도 close/list 확인을 포함 | 통과 |
| 25 | Terminal Browser 혼용 | 사용자 지정 브라우저 원칙 위반 | 고정 Agent Browser CLI 사용, Terminal Browser 실행 없음 | 통과 |
| 26 | fixture line-count drift | payload wrapping이 scene마다 바뀜 | A/B/C/D/E line count를 scene 단위로 검사 | 통과 |
| 27 | unsupported scope 누출 | arbitrary span/Unicode/calc가 허용으로 보임 | 계획에 별도 negative 경계와 일반 CSS 비주장을 명시 | 통과 |
| 28 | atomic failure 누락 | rejection 뒤 partial frame 또는 poisoned session 잔류 | revision/frame 원자성 및 같은 session valid recovery를 구현 gate에 둠 | 통과 |
| 29 | simulator 데이터 파괴 | 검증 종료 때 재사용 profile까지 제거 | 시작한 인스턴스만 종료하고 reusable profile을 보존하도록 계획 | 통과 |
| 30 | screenshot 증거 등급 혼동 | Chrome 화면을 Spinon/native 결과로 표시 | README와 plan에서 Chrome oracle·runtime·surface readback을 분리 | 통과 |

**결과:** 30개의 독립 PR diff 실패 관점을 확인했다. 확장한 Chrome matrix를 다시 실행해 3,840/3,840 scene 검증을 통과했다. 남은 것은 계획 PR merge 뒤 제품 구현 및 별도 Android/iOS RuntimeSession 검증이다.
