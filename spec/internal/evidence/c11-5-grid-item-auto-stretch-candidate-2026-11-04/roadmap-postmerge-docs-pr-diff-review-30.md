# PR #1132 병합 후 문서 동기화 변경 검토 30개

검토 대상은 PR #1132 병합 뒤 공식 상태 대장, 내부 계약 0059, C11.5 전용 계획, 실행 근거 README의 상태 문구와 링크다. 이 검토는 문서 변경의 정확성과 범위를 확인하며 새 제품 동작이나 테스트 결과를 만들지 않는다. 확인 기준은 현재 `origin/main`의 PR 상태와 병합 SHA `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`이다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | GitHub PR의 상태가 실제로 `MERGED`인지 | 통과: PR #1132가 `MERGED`다. |
| 2 | PR의 merge SHA가 본문에 적은 전체 SHA와 같은지 | 통과: `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`로 일치한다. |
| 3 | 병합 시각이 UTC 표기로 보존됐는지 | 통과: `2026-11-03T20:03:49Z`가 GitHub 값과 일치한다. |
| 4 | C11 상위 체크를 제한 PR 병합만으로 완료 처리했는지 | 통과: C11 체크는 미완료로 남는다. |
| 5 | C11.5 전체를 제한 auto-size/stretch 구현만으로 완료 처리했는지 | 통과: C11.5 전체 완료를 명시적으로 부정한다. |
| 6 | 공식 프로젝트 완료 수를 올렸는지 | 통과: `72/213`을 유지한다. |
| 7 | 상태 대장의 실제 진행 항목이 PR #1132를 후보로 남겨 두었는지 | 통과: 후보 표기를 병합된 제한 slice로 바꿨다. |
| 8 | 상태 대장의 고정-size self-alignment PR #1128 정보가 손상됐는지 | 통과: 앞선 병합 기록과 근거는 유지한다. |
| 9 | 상태 항목이 C11.1/C11 완료로 잘못 승격됐는지 | 통과: 상위 미완료 경계를 유지한다. |
| 10 | 내부 계약 0059의 섹션 제목이 미병합 상태로 남았는지 | 통과: PR #1132 병합 상태를 반영한다. |
| 11 | 내부 계약의 숫자 버전이 임의로 변경됐는지 | 통과: `0.1.0`을 유지한다. |
| 12 | 내부 계약이 public CSS 지원을 선언했는지 | 통과: 공개 CSS 지원이 아님을 유지한다. |
| 13 | 전용 계획의 상태가 로컬 후보로 남아 있는지 | 통과: 병합 시각과 SHA를 기록했다. |
| 14 | 전용 계획이 아직 미완료인 C11.5/C11을 완료로 표시했는지 | 통과: 두 항목 모두 미완료라고 적는다. |
| 15 | 실행 근거 README가 검증 branch의 원래 source commit을 잃었는지 | 통과: 기준 및 호환성 수정 source SHA를 유지한다. |
| 16 | 실행 근거 README가 병합 후 PR 상태를 후보로 설명하는지 | 통과: 병합된 제한 구현 PR 실행 근거로 정정했다. |
| 17 | Android 표면 결과가 실기기나 hardware GPU로 과장됐는지 | 통과: API 37 AVD와 ANGLE/Vulkan SwiftShader 한계를 유지한다. |
| 18 | Android PixelCopy 표본 수가 바뀌었는지 | 통과: 11/11 exact를 그대로 유지한다. |
| 19 | iOS Metal drawable이 compositor 이후 결과로 과장됐는지 | 통과: compositor 제출 전 readback이라고 유지한다. |
| 20 | 고정-size 회귀 표본과 auto-stretch 표본이 혼동됐는지 | 통과: 11/11과 별도 7/7 회귀 결과를 분리한다. |
| 21 | default artifact와 opt-in fixture artifact 경계가 누락됐는지 | 통과: 기본 산출물 제외 및 opt-in positive control을 유지한다. |
| 22 | Rust accepted matrix/negative controls를 합산해 과장했는지 | 통과: 10개 accepted scene과 16개 atomic negative controls를 별도 표기한다. |
| 23 | 관련 projection 및 Grid text stretch 검사 결과가 누락됐는지 | 통과: 3/3 및 2/2를 유지한다. |
| 24 | PR check rollup 공백을 CI 성공으로 표현했는지 | 통과: rollup이 비어 있어 CI 성공으로 세지 않는다고 적는다. |
| 25 | PR URL이 다른 번호를 가리키는지 | 통과: 모든 새 참조가 #1132를 가리킨다. |
| 26 | 상태 대장에서 계획·실행 근거의 상대 링크가 잘못됐는지 | 통과: `plan/` 기준 상대 경로와 `spec/` 기준 evidence 경로를 확인했다. |
| 27 | 내부 계약에서 evidence 경로가 문서 위치 기준으로 잘못됐는지 | 통과: evidence 경로가 `spec/internal/` 위치에 맞는다. |
| 28 | Tailnet preview 근거 링크가 제거됐는지 | 통과: 기존 Tailnet sync 링크를 유지한다. |
| 29 | 새 post-merge review 링크가 실제 경로와 맞는지 | 통과: 같은 evidence 폴더의 검토 파일을 연결한다. |
| 30 | 문서에서 범위 밖 일반 admission/WPT/전체 Grid를 지원한다고 읽힐 수 있는지 | 통과: 이를 미지원·미완료로 유지한다. |

**결론:** 병합 사실과 공식 문서를 동기화하되 제품 완료 체크와 `72/213`은 변경하지 않는다. PR check rollup은 비어 있어 CI 통과를 주장하지 않는다.
