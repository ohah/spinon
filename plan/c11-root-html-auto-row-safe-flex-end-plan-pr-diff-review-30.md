# C11.4 `safe|unsafe flex-end` 계획 PR 변경 검토 30개

계획, 사전 비교 자료, 상태 대장, 내부 계약의 병합 메타데이터, 공개 로드맵 변경을 서로 다른 실패 경로로 검토했다. 이는 계획 PR 변경 검토이며 구현/runtime의 별도 실패 관점 검토를 대체하지 않는다.

| # | 실패 경로·경계 | 확인 기준 | 결과 |
|---:|---|---|---|
| 1 | 변경 전후 상태 대장의 C11.4 공식 checkbox가 달라질 수 있음 | 해당 줄 외 기존 미완료 체크·전체 집계 유지 확인 | 통과 |
| 2 | 공식 완료 수가 부분 구현으로 증가할 수 있음 | 공개 roadmap·STATUS 모두 `78/214` 유지 | 통과 |
| 3 | C11 상위 항목이 실수로 완료될 수 있음 | C11 미완료 표기 보존 | 통과 |
| 4 | C11.4 자체가 완료로 오인될 수 있음 | 제한 slice와 상위 완료 상태를 문장으로 구분 | 통과 |
| 5 | 계획만 끝난 것을 구현 완료로 표시할 수 있음 | 계획·사전 비교 완료, 구현 미시작을 명시 | 통과 |
| 6 | 진행 카드가 공식 상위 ID를 중복 등록할 수 있음 | 기존 공식 상태 집합을 건드리지 않는 `detailInProgress` 사용 | 통과 |
| 7 | 카드 제목이 진행 단계보다 앞설 수 있음 | `flex-end 계획` 카드와 구현 미시작 문구가 일치 | 통과 |
| 8 | PR #1256의 병합 기록이 후보로 남을 수 있음 | UTC 병합 시각과 최종 merge SHA를 공개·내부 문서에 반영 | 통과 |
| 9 | 원래 작업 SHA가 rebase merge SHA로 혼동될 수 있음 | merge commit `0c3fa3f94e9a4af1db2f3e22be012df2282e6a67` 기록 | 통과 |
| 10 | 비어 있는 check rollup을 CI 성공으로 표현할 수 있음 | 기존 기록에서 CI green 주장을 추가하지 않음 | 통과 |
| 11 | status 문장에 PR 생성 중 같은 임시 상태가 남을 수 있음 | 이 변경 자체가 계획·명세 동기화임을 기술 | 통과 |
| 12 | public roadmap의 최신 시각순 항목이 틀릴 수 있음 | #1256 뒤에 flex-end 계획 항목을 둠 | 통과 |
| 13 | 다음 계획 링크가 오타 경로로 연결될 수 있음 | 저장소 파일명과 `/spinon/plan/...html` 라우트 일치 확인 | 통과 |
| 14 | 계획 검토 링크가 잘못된 확장자를 가리킬 수 있음 | 계획 문서 `.md` 및 공개 roadmap `.html` 목적 구분 | 통과 |
| 15 | 사전 비교 링크가 브라우저에서 열리지 않을 수 있음 | RSPress 내부 evidence HTML 경로 사용 | 통과 |
| 16 | 내부 계약에 병합되지 않은 구현을 현재 지원처럼 표시할 수 있음 | 기존 flex-start는 merged, flex-end는 미검증으로 분리 | 통과 |
| 17 | 내부 계약의 숫자 버전을 문서 변경만으로 올릴 수 있음 | `0.1.0` 유지 | 통과 |
| 18 | 이번 계획에 미지원 topology 범위가 빠질 수 있음 | 다른 writing mode·direction·Grid topology를 남은 범위에 유지 | 통과 |
| 19 | 제한 범위가 generic CSS 지원으로 확대 해석될 수 있음 | 일반 CSS admission·parser 변경 없음을 명시 | 통과 |
| 20 | 공용 API·FFI 계약 변경을 암시할 수 있음 | API/FFI 변경 없음 명시 | 통과 |
| 21 | Chrome 관측과 구현 결과가 섞일 수 있음 | precomparison README에서 browser oracle 전용임을 제한 | 통과 |
| 22 | 독립 기대값이 Chrome 출력에서 역산될 수 있음 | 산식·기대 좌표를 별도 파일로 고정하고 비교기를 분리 | 통과 |
| 23 | scene 수와 좌표 수가 서로 모순될 수 있음 | 6 scenes·72 coordinates와 비교 JSON 확인 | 통과 |
| 24 | 허용오차와 최대오차가 상충할 수 있음 | 최대 오차 0.0, tolerance 1/64 CSS px 확인 | 통과 |
| 25 | positive 공간에서 safe fallback을 잘못 기대할 수 있음 | 세 alignment 모두 `[10,20,30]` 기대 | 통과 |
| 26 | overflow의 safe와 unsafe를 같은 결과로 합칠 수 있음 | safe `[0,56,112]`, unsafe `[-82,-26,30]` 분리 | 통과 |
| 27 | 음수 좌표를 screenshot pixels로 입증했다고 과장할 수 있음 | 계획에서 음수 frame은 layout/runtime 좌표 전용으로 한정 | 통과 |
| 28 | 계획의 선행조건을 이미 완료된 검증으로 표현할 수 있음 | 구현·simulator·WPT 게이트는 future gate로 기술 | 통과 |
| 29 | Tailnet 미리보기와 GitHub 문서 병합을 같은 배포로 오인할 수 있음 | Tailnet sync와 Agent Browser 확인을 후속 단계로 남김 | 통과 |
| 30 | PR의 파일 집합에 무관한 사용자 자료를 포함할 수 있음 | 계획·비교 evidence·상태·계약·로드맵만 선별 stage | 통과 |

이 문서 검토는 병합 전 계획 diff에 한정된다. 문서 사이트·Tailnet live preview의 반영은 별도 제공본 hash 및 Agent Browser viewport 확인으로 검증한다.
