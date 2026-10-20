# C11.4 PR #395 Tailnet 로드맵 변경 적대 검토 30개

| # | 실패 경로·불변 조건 | 판정 |
| ---: | --- | --- |
| 1 | C11.4가 완성된 것처럼 체크될 수 있음 | C11.4 미완료 문장을 추가했고 status 체크를 그대로 뒀다. |
| 2 | overview 완료 수가 새로 증가할 수 있음 | Agent Browser에서 213개 표시·72개 완료였고 체크 상태는 변경하지 않았다. |
| 3 | PR 링크가 닫힌 다른 번호를 가리킬 수 있음 | 링크는 병합된 PR #395를 가리킨다. |
| 4 | 사용자 로컬 path가 Tailnet HTML에 포함될 수 있음 | sanitizer 후 남은 호스트 경로가 0개다. |
| 5 | 개인정보가 든 Android serial이 공개될 수 있음 | 기기 serial은 선택된 JSON에서 제거되어 있음을 확인했다. |
| 6 | Android 실기기와 AVD 결과가 섞일 수 있음 | 새 evidence README와 상태 메모가 Samsung 실기기로 지정한다. |
| 7 | iOS Simulator가 실기기 테스트로 오인될 수 있음 | iPhone 17 Pro / iOS 26.2 Simulator로 한정해 적었다. |
| 8 | Chrome oracle와 Spinon runtime이 같은 evidence인 것처럼 보임 | 로드맵 문구는 기준 비교와 플랫폼 실행을 구분한다. |
| 9 | CSS track 차이가 허용치를 넘는데도 통과로 표현 | 0.010415 CSS px는 0.015625 CSS px 임계보다 작다. |
| 10 | span 단계의 입력 순서가 기록에서 빠짐 | 상세 실행 README가 `[2]`와 `[2,3]` trace 단계를 구분한다. |
| 11 | UI draw/present가 확인되지 않았는데 scene 통과로 기록 | 실행 원본의 draw/present assertion을 기준으로 썼다. |
| 12 | C11.4의 남은 sizing 영역을 닫았다고 해석 | 전체 C11.4 완료가 아니라고 메모·계획 링크에서 제한했다. |
| 13 | 공개 CSS 지원 계약이 새로 생긴 것처럼 표현 | `spec/STATUS.md`와 공개 API 계약을 추가하지 않았다. |
| 14 | 내부 인터페이스 버전이 불필요하게 올라감 | 내부 계약 버전 및 문서를 수정하지 않았다. |
| 15 | Rspress가 2.0.22보다 올라감 | 최종 빌드는 설치된 2.0.22 실행 파일로 만들었고 package/lock diff가 없다. |
| 16 | 전체 docs build 결과를 주장 | 289개 입력의 선택된 문서 root만 빌드했다고 범위를 명시했다. |
| 17 | 수동 Pages workflow를 의도치 않게 실행 | workflow와 GitHub Pages 배포를 실행하지 않았다. |
| 18 | PR #370 백업이 덮어써짐 | 별도의 PR #395 operation과 pass15/pass16 경로를 쓴다. |
| 19 | pass15 산출물이 pass16 산출물과 혼동 | staging root, site output, input manifest가 실행 ID를 포함한다. |
| 20 | 잘못된 source root에서 Markdown source를 직접 복사 | sync helper가 sanitized Rspress output 경로를 검증한다. |
| 21 | target path 오타가 임의 디렉터리를 바꿀 수 있음 | target과 backup 경로를 승인된 `SpinonPreview` 위치와 비교한다. |
| 22 | 심볼릭 링크를 따라 예상 밖 파일을 교체 | source, target, backup 경로와 선택 파일에 symlink 검사가 있다. |
| 23 | 현재 열려 있는 서버 파일이 부분 쓰기로 보일 수 있음 | 동일 디렉터리 임시 파일 뒤 atomic rename으로 교체한다. |
| 24 | 서버를 종료해 다른 미리보기 사용자를 끊음 | 기존 PID 1191은 두 실행 모두 유지했다. |
| 25 | 기존 파일을 백업하지 않고 교체 | 바뀐 파일 수만큼 백업하고 모든 백업 해시가 일치했다. |
| 26 | 이전 해시 자산 삭제가 다른 링크를 끊음 | sync 결과가 `no_deletes: true`이며 삭제하지 않았다. |
| 27 | 복사 성공 메시지만 보고 서버 제공 결과를 추정 | HTTP 200·byte·SHA를 직접 확인해 pass16 1,640/1,640 일치했다. |
| 28 | C11.4 새 근거의 링크 또는 이미지가 누락 | Agent Browser에서 evidence route와 Android·iOS 이미지 각각을 확인했다. |
| 29 | Tailnet 화면 확인용 브라우저가 사용자 프로세스로 남음 | Agent Browser 두 세션을 닫고 daemon/Chrome이 남지 않음을 확인했다. |
| 30 | 기존 사용자 untracked artifact를 staging할 수 있음 | 명시된 경로만 `git add`했고 다른 evidence를 포함하지 않았다. |

**최종 판정:** 공개 로드맵은 C11.4 부분 진행을 표시하며 완료 상태를 바꾸지 않는다. 남은 경계와 플랫폼 근거를 함께 노출한다.
