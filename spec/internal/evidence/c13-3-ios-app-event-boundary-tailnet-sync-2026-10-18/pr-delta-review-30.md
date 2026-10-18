# C13.3 Tailnet 동기화 PR 변경 실패 관점 검토 30개

검토 범위는 C13.3 내부 인덱스 링크, Tailnet 동기화 기록, 선택 문서 렌더와 preview 복사·served hash 근거다. UIKit probe 구현 자체를 다시 승인하거나 C13.3 완료로 판정하지 않는다.

| # | 독립 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 원본 구현 PR이 아직 미병합인데 동기화 문서가 병합을 전제함 | PR #307의 `MERGED`와 merge SHA를 GitHub에서 다시 확인한 뒤 기록했다. |
| 2 | 문서가 잘못된 commit을 merge SHA로 기록함 | merge SHA `faf48de4ba2ffc1d5d12bb2607419ecc220bc40b`를 PR 메타데이터와 대조했다. |
| 3 | 실험 근거가 공식 완료 수를 몰래 올림 | 상태 대장과 로드맵 표시 모두 `74/207`; C13.3 체크는 미완료다. |
| 4 | Tailnet 로드맵 counter가 저장된 오래된 숫자를 보여 줌 | Agent Browser에서 페이지의 동적 counter와 GitHub `main` 상태 요청을 함께 확인한다. |
| 5 | 부분 Simulator 관찰을 제품 입력 지원 완료로 표현함 | probe와 동기화 범위를 분리하고 제품 adapter는 바꾸지 않았다고 명시했다. |
| 6 | 30 observation window를 30개의 인과적 이벤트 join으로 과장함 | action ID 직접 join 부재와 window count의 한계를 유지했다. |
| 7 | 90개 `.scroll` event를 90개 UIKit callback이라고 오독함 | 앱 event count와 recognizer callback을 21/9 window로 분리했다. |
| 8 | callback 없는 window에서 `.discrete` 오분류라고 단정함 | 기존 type-mask 대조의 결과와 별도로 두며, 세부 UIKit 원인은 미정으로 적었다. |
| 9 | direct-touch control을 indirect trackpad 증거로 혼동함 | `.touches`·pan 대조를 별도 입력 source로 구분했다. |
| 10 | 다른 기기나 iOS 버전 결과를 현재 Simulator 결과에 섞음 | iPad Pro 13-inch (M5)/iOS 26.2 Simulator로 범위를 고정했다. |
| 11 | 최종 source focused run과 full suite의 source revision을 혼동함 | focused 2/2와 이전 revision full suite 17/17을 분리했다. |
| 12 | disk-full full-suite 재시도를 통과로 세거나 숨김 | 최종 retry `unknown/0 tests`를 성공 근거에서 제외했다. |
| 13 | C13.3 index가 새 sync README로 연결되지 않음 | 내부 인덱스 C13.3 행에 새 상대 경로를 추가하고 렌더했다. |
| 14 | Rspress 버전이 고정 정책을 벗어남 | 선택 build와 lockfile이 `2.0.22`를 사용한다. |
| 15 | 일부 페이지만 빌드하고 전체 문서 build라고 씀 | 선택 root의 파일/출력 수와 전체 build가 아님을 명시했다. |
| 16 | subset 외부 dead link 검사가 꺼진 사실을 누락함 | 선택 config에서 dead-link/image 검사 비활성화 범위를 build 보고서에 기록했다. |
| 17 | 본문에 링크된 raw Markdown이 preview에서 빠짐 | 선택된 plan·status·internal 문서의 Markdown 원본도 preview overlay에 포함한다. |
| 18 | 실행 원본 JSON·log·screenshot 첨부가 누락됨 | event-boundary 원본 evidence directory의 regular file 전부를 대상 목록에 넣는다. |
| 19 | `/spinon/` base가 붙은 HTML route 경로가 잘못됨 | roadmap·plan·status·실행·sync README route를 Agent Browser에서 직접 연다. |
| 20 | 원본 Simulator 화면이 잘린 파일이나 0-byte asset으로 변함 | 두 화면의 served bytes/SHA-256과 browser `naturalWidth × naturalHeight`를 확인한다. |
| 21 | Tailnet 화면 캡처가 화면에 counter를 표시하지 않음 | counter가 보이는 위치로 스크롤한 Agent Browser 캡처를 보관한다. |
| 22 | C13.3 roadmap 설명이 다른 Tailnet 버전으로 남음 | event-boundary 문구와 공식 미완료 표식을 같은 화면에서 확인한다. |
| 23 | GitHub 상태 원본 요청 실패를 정적 화면 결과로 대체함 | browser network evidence에서 `main` status API 응답을 별도로 확인한다. |
| 24 | screenshot 파일만 있고 브라우저 요청 status 증거가 없음 | 관련 route·이미지·동적 status 요청만 남긴 sanitized network log를 저장한다. |
| 25 | 공개 근거에 Tailnet DNS, 사용자 경로, PID가 노출됨 | browser log와 JSON은 hostname/query/PID/absolute path를 제거해 저장한다. |
| 26 | 덮어쓴 preview 파일이 복구 불가능함 | replace 대상마다 사전 backup을 만들고 backup SHA-256을 원본과 비교한다. |
| 27 | preview 경로 바깥이나 symlink 대상에 파일을 기록함 | target root 아래 상대 경로만 허용하고 중간 구성 요소의 symlink를 거부한다. |
| 28 | 사전 검사 뒤 기존 파일이 바뀐 상태에서 무조건 덮어씀 | copy 직전에 target의 존재·SHA를 preflight snapshot과 재비교한다. |
| 29 | 누락된 산출물을 정리한다며 기존 preview 파일을 삭제함 | sync는 source overlay만 적용하고 삭제 수는 0으로 기록한다. |
| 30 | 검증 후 Agent Browser 또는 사용자의 tmux 작업이 남거나 종료됨 | 해당 Agent Browser만 닫고 고유 Chrome profile process가 사라졌는지 확인한다. tmux는 건드리지 않는다. |

## 판정

검토는 선택한 문서·preview 동기화의 범위와 파일 일치만 다룬다. route·HTTP·hash·browser 종료 항목은 최종 산출물의 JSON 근거가 생성된 뒤 값과 서로 대조한다. 전부 확인되면 Tailnet 미리보기 반영은 완료로 판정하되, C13.3 구현 상태는 계속 미완료다.
