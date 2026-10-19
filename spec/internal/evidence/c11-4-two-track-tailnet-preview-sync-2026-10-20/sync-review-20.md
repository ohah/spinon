# C11.4 two-track auto-repeat Tailnet 동기화 실패 관점 검토 20개

**대상:** pass11 선택 root, Rspress 산출물, 고정 Tailnet 경로, backup·복사 절차, HTTP 응답, Agent Browser 화면. 각 항목은 별도 경계나 실패 원인을 확인한다.

| # | 적대 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | 미리보기에 PR #365 미병합 브랜치의 내용을 넣었나? | PR #365 Rebase merge SHA `a2c137213f72e82b3a8e3cc153e92c8b9db9a653`와 Tailnet 반영 내용을 대조했다. pass11 source HEAD는 문서 업데이트 커밋 `1590056dec37feacdeb8b84234a71d709bc3d1a9`다. |
| 2 | 공개 roadmap만 고치고 상세 근거 route를 빼먹었나? | 집중 계획, 사전 비교, 구현 README 및 실행 근거를 선택 root에 포함하고 roadmap 링크가 실제 route에 도착하는 것을 Browser에서 확인했다. |
| 3 | 선택 문서에서 C11.4를 완료로 올렸나? | 공개 roadmap·STATUS·계획에서 C11.4 체크와 완료 집계를 유지하고 남은 범위를 명시했다. |
| 4 | UI counter를 Markdown의 모든 checkbox와 같은 집계라고 잘못 설명하나? | UI는 72/212 task rows를 표시하고, raw Markdown은 86/225 checkbox다. ID 없는 하위 확인 13개가 UI inventory 밖이며 중복 ID도 같은 상태 map key를 공유한다. README에서 두 집계를 분리했다. |
| 5 | 78/0 중간 렌더 값을 완료 집계로 기록했나? | 초기 hydration 중 읽은 값은 버리고 complete document의 UI counter `72/212`를 재확인했다. 중복 ID의 행별 상태 의미가 별도이므로 이 수는 UI 표시값이라고만 썼다. |
| 6 | plan이나 evidence 링크가 base path 밖으로 나가거나 404가 되나? | `/spinon/plan/...`과 `/spinon/spec/internal/evidence/...` route가 Agent Browser에서 열리고 실제 본문을 표시했다. |
| 7 | 페이지의 `/internal/` 복제 route가 누락됐나? | 선택 root 생성 목록은 내부 근거를 `/internal/` 및 `/spec/internal/` 경로에 각각 매핑한다. 두 렌더 결과가 933개 사이트 파일 안에 포함된다. |
| 8 | 빌드가 선택된 증거 대신 전체 저장소를 실수로 문서화하나? | `prepare-root-pass11.json`의 선택 source/destination과 file_count 1,074를 확인했고 빌드는 그 root만 사용했다. |
| 9 | 사용자 홈 절대경로나 로컬 checkout 경로가 결과물에 남았나? | staging sanitization은 한 번들에서 1개 파일을 정리했고 remaining host path는 0개다. 선택 root의 source-local redactions 125개도 기록했다. |
| 10 | 설치된 문서 엔진이나 lockfile을 임의로 올렸나? | 고정된 `@rspress/core` 2.0.22와 선택 config로 빌드했고 package/lockfile 버전 변경은 없다. |
| 11 | 이전 pass를 pass11 선택 root나 build directory로 덮어쓰나? | 출력 이름이 run 값에 따라 `pass11`로 분리되고 helper가 `pass6|7|8|9|11` 외의 입력을 거부한다. |
| 12 | pass11 추가가 과거 pass6–pass9 백업 위치를 바꿨나? | 최종 diff에서 backup operation을 pass11에만 분기했다. 과거 pass 재실행은 기존 `c11-4-image-tailnet-preview-update-2026-10-20` 경로를 유지한다. |
| 13 | sync source가 staging Markdown root일 수 있나? | helper는 source를 `build/spinon/c11-4-image-tailnet-site-pass11`로 제한한다. 실제 preflight도 sanitized site output을 사용한다. |
| 14 | 다른 사용자 디렉터리나 preview 경로를 덮어쓸 수 있나? | target은 `~/Library/Application Support/SpinonPreview`, backup은 operation/pass별 고정 하위 경로와 일치해야 한다. |
| 15 | symlink를 따라 source·target·backup 경계 밖으로 쓸 수 있나? | 선택 build의 symlink와 각 경로 구성 요소·destination 경로 symlink를 거부하는 검사를 유지했다. |
| 16 | 오래된 preflight를 이용해 변경된 source/target을 복사하나? | copy 단계가 source set/hash와 target 기존 상태를 preflight manifest에 대조한 뒤 복사한다. 기록된 933개가 모두 검증됐다. |
| 17 | 실패 중 기존 37개 페이지가 복구 불가능하게 교체되나? | 교체 전에 별도 pass11 backup을 만들고 37/37 byte·SHA를 확인했다. |
| 18 | 열려 있는 Tailnet 파일을 직접 덮거나 삭제하나? | 교체 경로 37개에 열린 handle 0개를 확인했고 삭제는 없었다. atomic same-directory replacement를 사용한다. |
| 19 | 파일 복사 성공만으로 Tailnet 제공 성공을 주장하나? | 933개 URL 모두 HTTP 200, byte 수, SHA-256을 대조해 933/933 일치했다. |
| 20 | Browser 검증 후 프로세스나 공식 카운트를 잘못 남겼나? | 3개 페이지와 Android/iOS 원본 크기 이미지를 확인한 뒤 전용 Browser/Chrome을 닫았다. 완료 집계는 72/212로 유지했다. |

## 발견 및 수정

- diff 재검토에서 pass11 backup 경로 변경이 과거 pass6–pass9 재실행까지 바꿀 수 있음을 찾았다. helper를 pass11 전용 backup 경로로 분기해 기존 재현 경로를 보존했다. pass11 실제 경로는 그대로다.
- 첫 DOM 조회는 hydration 진행 중 `78/0`을 반환했다. 완전 로드 뒤 Agent Browser counter는 `72/212`였다. 별도 대조에서 raw Markdown은 `86/225`; 13개 ID 없는 nested checkbox와 `C04.12`·`C05.2` 중복 ID 때문에 숫자가 서로 다른 집계임을 발견해 두 값이 같다고 주장하지 않았다.
