# 정리 기록 범위 수정 PR diff 적대적 검토 30개

검토 대상은 `resource-cleanup.json`의 Agent Browser 프로세스 범위, iOS Simulator 상태, 정리·보존 기록이다. 각 항목은 별도 오기·과잉 주장·자원 소유권 오류를 확인한다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 작업 세션 두 개를 기록에서 빠뜨림 | `spinon-docs-1369-20261112`와 `spinon-pr1370-20261112`를 각각 유지했다. |
| 2 | 세션 종료 성공과 프로세스 종료 확인을 같은 값으로 취급 | 세션별 종료 명령 성공과 namespace 프로세스 0개를 분리해 기록했다. |
| 3 | 전체 머신의 Agent Browser 프로세스 수를 작업 세션 수로 오해 | 전역 `agentBrowserBinaryProcessAfterClose` 주장을 제거했다. |
| 4 | 다른 작업 namespace가 관찰된 사실을 숨김 | `noori-public-r7-gui-r1` namespace가 관찰된 점을 범위 한정해 기록했다. |
| 5 | 다른 namespace 프로세스를 현재 작업 소유로 귀속 | 해당 프로세스는 현재 작업 밖이라고 명시했다. |
| 6 | 다른 작업의 브라우저를 닫아도 되는 대상으로 분류 | 외부 namespace 프로세스는 보존했다고 기록했다. |
| 7 | 공유된 Agent Browser 실행 파일의 존재를 세션 생존으로 간주 | 실행 파일 전체 수 대신 작업 namespace 프로세스만 판정한다. |
| 8 | 세션 ID와 namespace를 서로 바꾸어 기록 | 각 항목의 namespace와 session 값을 따로 보존했다. |
| 9 | namespace 종료가 다른 namespace를 함께 닫았다고 암시 | 외부 namespace는 그대로 보존했으며 혼합 정리 주장을 하지 않는다. |
| 10 | Terminal Browser 프로세스를 Agent Browser로 잘못 셈 | 사전 존재 PID 72772를 별도 보존 항목으로 유지했다. |
| 11 | 사용자가 요청하지 않은 Terminal Browser를 종료 | 해당 PID의 action은 보존으로 남아 있다. |
| 12 | Tailnet preview server를 임시 작업 프로세스로 오분류 | PID 90711 / port 8768 서버를 계속 제공 상태로 기록했다. |
| 13 | preview server 보존과 브라우저 세션 종료를 혼동 | 서버와 두 Agent Browser 세션을 서로 다른 자원으로 기술했다. |
| 14 | ADB inventory 뒤 서버가 남았다고 잘못 기록 | ADB 서버는 inventory 이후 종료됐다는 기존 상태를 유지했다. |
| 15 | Android emulator가 부팅된 채라고 기록 | emulator process 0과 booted device 0을 유지했다. |
| 16 | iOS simulator 기기를 booted로 오기 | `bootedDevices`는 0이다. |
| 17 | S03/Pilot 기기가 존재하거나 사용 중이라고 오기 | 일치하는 available device가 없다는 기존 관찰을 유지했다. |
| 18 | Simulator GUI 앱 프로세스와 CoreSimulator 서비스를 동일시 | 앱 프로세스 0과 시스템 서비스를 별도 필드로 구분했다. |
| 19 | CoreSimulator 시스템 서비스를 task 앱으로 종료 | 공유 시스템 서비스는 보존했다고 명시했다. |
| 20 | 공유 simulator profile을 삭제했다고 오기 | 프로필은 보존했다. |
| 21 | 선택 빌드 경로 밖의 산출물까지 제거했다고 주장 | 삭제 기록은 선택 빌드 경로 168개 파일에만 한정했다. |
| 22 | 선택 빌드 삭제가 검증보다 먼저였다고 오해 | hash와 브라우저 검증 뒤 제거했다는 순서를 유지했다. |
| 23 | 증거용 screenshot을 임시물로 보고 지웠다고 암시 | 두 Tailnet screenshot을 보존 목록에 둔다. |
| 24 | Tailnet 교체 백업을 정리 대상으로 혼동 | 이전 동기화에서 만든 백업 보존·검증 기록에 손대지 않았다. |
| 25 | UTC timestamp 형식을 로컬 시간으로 바꾸거나 필드를 누락 | ISO UTC 형식으로 검증 시각을 기록했다. |
| 26 | 시점 관찰을 영구적인 머신 상태로 일반화 | 확인된 namespace와 프로세스 범위만 기록하고 전역 0 주장은 하지 않는다. |
| 27 | JSON의 쉼표·문자열 오류로 읽을 수 없는 상태 | 수정 뒤 JSON parser로 구조 유효성을 확인한다. |
| 28 | cleanup scope 문장과 세부 필드가 서로 모순 | 작업 자원 정리와 외부·공유 자원 보존 문구가 필드와 일치한다. |
| 29 | 제품 상태나 C11 완료 수치를 cleanup PR에서 변경 | 이번 변경은 정리 근거 파일만 수정하며 공식 상태는 건드리지 않는다. |
| 30 | 작업 세션 종료를 전체 브라우저 프로세스 종료라고 과장 | 보고 범위를 task namespaces로 제한하고 무관한 브라우저를 보존한다. |

검토는 정리 기록의 소유 범위와 주장의 강도를 점검한다. 이 문서는 현재 머신의 이후 상태를 보장하거나 전체 브라우저 종료를 주장하지 않는다.
