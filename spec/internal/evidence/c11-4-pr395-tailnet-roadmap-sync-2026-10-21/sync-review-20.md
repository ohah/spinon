# C11.4 PR #395 Tailnet 동기화 독립 실패 관점 검토 20개

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | C11.4 부분 검증을 전체 완료로 표시 | 공개 로드맵 메모에 전체 미완료를 적고 공식 체크를 유지했다. |
| 2 | 공개 로드맵 설명만 바꾸고 공식 대장 상태도 바뀐 것으로 오해 | `spec/STATUS.md`는 수정하지 않았다. 로드맵 체크와 완료 수가 유지됐다. |
| 3 | merge commit을 잘못 연결 | PR #395의 merge commit `8ec32c6`을 기준 SHA로 기록했다. |
| 4 | 사용자에게 PR 화면 링크가 없는 메모 제공 | 로드맵 메모에 PR #395, 집중 계획, 실행 근거 링크를 넣었다. |
| 5 | Chrome 비교값을 규범 요구값처럼 서술 | CSS Grid §11.5를 기준으로 만든 기대값과 Chrome 155 serialization을 기존 근거에서 구분했다. |
| 6 | 실기기와 Simulator 근거 혼합 | Samsung SM-S731N 실기기와 iPhone 17 Pro Simulator를 별도로 명시했다. |
| 7 | Simulator를 iOS 실기기라고 표현 | iOS 26.2 Simulator로 표기했고 물리 iPhone 근거로 승격하지 않았다. |
| 8 | 특정 장면 일부 통과를 전체 통과로 확장 | Android와 iOS에서 각각 4/4 장면으로 범위를 고정했다. |
| 9 | draw/present 이전의 geometry만 실행 성공으로 표시 | 기존 실행 근거의 draw/present assertion을 포함했다. |
| 10 | f32와 Chrome의 1/64 CSS px 양자화 차이를 실패나 완전 동일로 오해 | 최대 track 차이 0.010415 CSS px를 수치로 적고 전체 동등성 주장은 하지 않았다. |
| 11 | 0.010415가 1/64 CSS px 허용치보다 큰 것으로 잘못 비교 | 허용치 0.015625 CSS px보다 작다는 원본 비교 근거를 확인했다. |
| 12 | 새 메모가 자동 생성 checklist를 바꿈 | 메모는 단락이며 체크박스 데이터와 상태 변환 코드를 건드리지 않았다. |
| 13 | 213개·72개 카운터가 PR에서 증가 | Agent Browser에서 213개 표시·72개 완료와 C11.4 미완료를 확인했다. |
| 14 | 기존 pass14/PR #370 백업을 덮어씀 | pass15/16 모두 `c11-4-pr395-tailnet-roadmap-sync-2026-10-21` 아래의 새 경로를 사용했다. |
| 15 | 잘못된 선택 root를 동기화 | source 경로 guard가 sanitized `c11-4-image-tailnet-site-pass15/16`만 허용한다. |
| 16 | 전체 저장소나 비관련 untracked 근거를 공개 | manifest와 prepare helper가 고정 입력 목록 및 최종 Android/iOS 실행 경로만 선택한다. |
| 17 | 입력 manifest가 자기 자신이나 최종 해시를 반복 포함 | manifest 출력 경로와 최종 증거 해시 경로를 입력 순회에서 제외했다. |
| 18 | 실행 중인 Tailnet 서버를 중지하거나 열린 파일을 덮어씀 | PID 1191을 유지했다. 열린 대상은 helper의 임시 파일 작성·원자 rename 경계를 거쳤다. |
| 19 | 복사 오류 뒤 복구 기준을 잃음 | 교체 파일 76개와 84개의 백업을 각각 76/76, 84/84 해시로 확인했다. |
| 20 | 오래된 응답 또는 누락 파일을 새 배포로 간주 | HTTP 200, 응답 바이트 수, SHA-256을 pass15/16 각 1,640개 모두 비교했고 삭제는 없었다. |

**남은 한계:** Tailnet 미리보기는 개발 중 검토 경로다. GitHub Pages 배포, 전체 Rspress 문서 사이트 build, 앱 또는 WPT 실행의 증거로 사용하지 않는다.
