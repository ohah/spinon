# PR #1056 병합 후 상태 문서 PR 변경 검토 30관점

대상은 `spec/STATUS.md`, C11 계획 두 파일, 내부 계약 `0059`, runtime evidence README와 병합 기록이다. 병합 원본은 GitHub PR #1056 및 현재 `origin/main`이다. 각 관점은 서로 다른 상태 오류나 범위 확대 경로를 확인한다.

| # | 실패 관점 | 확인 내용 | 판정 |
|---:|---|---|---|
| 1 | 닫히지 않은 PR을 병합으로 기록 | GitHub PR 상태가 `MERGED`인지 조회 | 통과 |
| 2 | 병합 시각이 잘못 복사됨 | PR API의 `mergedAt`과 모든 갱신 문구 비교 | 통과 |
| 3 | merge SHA 오기 | PR API merge SHA와 상태 문서·계획·계약 대조 | 통과 |
| 4 | 기준 브랜치가 다른 커밋을 가리킴 | `origin/main`과 문서 동기화 기준 HEAD 비교 | 통과 |
| 5 | 상태 체크가 있었는데 미보고로 기록 | PR `statusCheckRollup`을 직접 조회 | 통과 |
| 6 | 후보/열림 문구가 병합 후에도 남음 | 변경된 #1056 상태 문구에서 pending 상태를 확인 | 통과 |
| 7 | 좁은 slice를 C11.1 완료로 승격 | 각 문서가 제한 구현이라고 명시하는지 확인 | 통과 |
| 8 | C11 부모 체크가 실수로 켜짐 | `spec/STATUS.md` C11 checkbox 확인 | 통과 |
| 9 | C11.1 전체를 완료로 암시 | 상태·계획이 C11.1 미완료를 유지하는지 확인 | 통과 |
| 10 | 전체 완료 집계가 임의로 증가 | 문서의 공식 `72/213` 유지 여부 확인 | 통과 |
| 11 | 전체 완료 집계를 C11 퍼센트로 오해 | 문서가 `72/213`을 프로젝트 집계로 한정하는지 확인 | 통과 |
| 12 | sparse column flow를 dense로 확대 | PR 제목과 제한 계약의 sparse 범위 비교 | 통과 |
| 13 | 고정 fixture 입력을 일반 CSS admission으로 확대 | 명시 지원 범위와 전체 CSS Grid 한계 비교 | 통과 |
| 14 | Chrome oracle를 Spinon WPT 통과로 확대 | WPT/전체 적합성 미완료 문구 확인 | 통과 |
| 15 | 체크가 없는 CI를 성공으로 보고 | 빈 check rollup을 CI 성공이 아니라고 기록 | 통과 |
| 16 | focused layout 결과 수 변경 | 병합 PR/evidence의 layout 4/4 대조 | 통과 |
| 17 | projection 또는 runtime 건수 오기 | projection 3/3, runtime 4/4 대조 | 통과 |
| 18 | 기존 Root HTML 회귀 수 오기 | Root HTML Grid regression 37/37 대조 | 통과 |
| 19 | 전체 suite 실패를 숨김 | style-to-layout 305/306 상태가 유지되는지 확인 | 통과 |
| 20 | 실패 원인을 새 변경에 귀속 | 남은 실패가 기존 C07 aspect-ratio serialization 차이인지 확인 | 통과 |
| 21 | AVD를 실기기로 잘못 표기 | Android API 37 ARM64 AVD 표기를 확인 | 통과 |
| 22 | Simulator를 iOS 실기기로 잘못 표기 | iPhone 17 Pro/iOS 26.2 Simulator 표기를 확인 | 통과 |
| 23 | 화면 캡처를 pixel parity로 확대 | runtime README에서 visual smoke와 readback 한계를 확인 | 통과 |
| 24 | 병합 기록 링크가 끊김 | README의 `merge-PR1056.md` 상대 링크와 파일 존재 확인 | 통과 |
| 25 | evidence 링크가 잘못된 디렉터리를 가리킴 | 수정된 계획의 evidence 상대 경로 확인 | 통과 |
| 26 | 내부 계약 버전이 불필요하게 상승 | 계약 변경이 merge 사실·범위만 갱신하는지 확인 | 통과 |
| 27 | C11.3 또는 C11.4 체크가 바뀜 | 변경 파일 밖의 상태가 유지되는지 검사 | 통과 |
| 28 | 제품 코드가 문서 PR에 섞임 | 변경 파일 목록을 검토해 문서만 포함하는지 확인 | 통과 |
| 29 | 사용자 WIP/evidence 로그를 우발적으로 포함 | 선택 stage 경로가 지정 문서 파일로 제한되는지 확인 | 통과 |
| 30 | 문서마다 병합 상태 설명이 서로 충돌 | 상태 대장·계획·계약·evidence 문구를 교차 대조 | 통과 |

정적 확인: `git diff --check` 통과. 변경 파일의 병합·상태·범위 주장은 GitHub PR 정보와 저장소 상태를 대조했다. 이 문서 검토는 코드 테스트나 CI 결과를 뜻하지 않는다.
