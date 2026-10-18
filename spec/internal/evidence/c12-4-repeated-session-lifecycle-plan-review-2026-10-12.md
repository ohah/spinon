# C12.4 반복 session 수명 검증 계획 검토

| 공격 관점 | 발견한 실패 가능성 | 계획에 반영한 경계 |
| --- | --- | --- |
| 실행 경로 노출 | probe가 일반 앱 동작으로 켜질 수 있다. | 검증 전용 intent와 debug fixture로 제한한다. |
| 무제한 반복 | 외부 입력으로 cycle 수가 커져 기기를 압박할 수 있다. | 16 cycle을 코드에 고정하고 intent 입력을 받지 않는다. |
| 동시 리소스 상한 | 한 cycle의 isolate 수가 예기치 않게 커질 수 있다. | cycle당 최대 8, 총 128 session으로 고정한다. |
| 불완전 생성 | N번째 create 실패 후 앞서 생성한 host가 남을 수 있다. | 부분 생성 handle을 역순으로 정리하고 run은 실패 처리한다. |
| free 비동기성 | native free 반환을 worker 종료로 오인할 수 있다. | owner/CSS worker TID의 `/proc` 부재를 별도로 확인한다. |
| OS TID 재사용 | cycle 사이 TID 재사용을 누수나 잘못된 상관으로 볼 수 있다. | 전역 고유성 대신 각 cycle 내부 시간 순서로 비교한다. |
| worker 누락 | V8 owner만 확인하고 CSS worker 종료를 놓칠 수 있다. | 두 thread 역할의 TID 집합을 별도로 수집한다. |
| TID 중복 | 두 host가 같은 TID를 보고해 한 thread를 두 개로 셀 수 있다. | cycle별 8개 distinct TID 조건을 둔다. |
| stale log | 이전 실행 marker가 성공으로 오인될 수 있다. | logcat을 초기화하고 run ID·cycle marker를 묶는다. |
| marker 누락 | 앱이 중간에 멈춰도 일부 성공 로그만으로 통과할 수 있다. | 모든 cycle의 begin/ready/freed/complete와 최종 finished가 필수다. |
| 시간 초과 | 종료 지연이 무한 대기로 바뀔 수 있다. | 수집기는 명시 deadline을 두고 timeout을 실패로 기록한다. |
| process crash | 마지막 저장값만 남아 실패가 숨겨질 수 있다. | process PID 생존과 종료 코드를 확인하고 crash는 실패로 보존한다. |
| 한 번의 성공 편향 | 첫 cycle 뒤에만 정상 종료하면 반복 안정성을 놓친다. | 고정 16회 모든 cycle에서 동일 gate를 검사한다. |
| 매 회 peak 부하 | 낮은 동시 수만 반복해 8 session 해제 경합을 놓칠 수 있다. | 각 cycle은 매번 8개 동시 live 상태까지 도달한다. |
| 장시간 idle 혼입 | 백그라운드 대기 시간이 결과를 좌우할 수 있다. | 고정된 짧은 유지 구간만 두고 경과 시간을 기록한다. |
| `/proc` 권한 | 권한 실패를 TID 부재로 처리할 수 있다. | `run-as` 실패를 별도 수집 실패로 처리하고 부재로 위장하지 않는다. |
| 앱/OS 백그라운드 종료 | lifecycle pause나 절전 종료를 구현 실패와 섞을 수 있다. | foreground 실행을 유지하고 process 중단은 유효 run으로 인정하지 않는다. |
| 메모리 해석 | PSS 감소/증가를 stack 누수 근거로 오독할 수 있다. | process-wide 보조 metric이며 leak oracle이 아님을 명시한다. |
| 미검증 기능 혼입 | session lifecycle을 layout/GPU/WPT 검증으로 과장할 수 있다. | renderer·surface·layout·WPT는 범위에서 제외한다. |
| 플랫폼 일반화 | Android 단일 모델 결과를 iOS/다른 기기로 확대할 수 있다. | 기기·OS·APK hash를 고정하고 단일 기기 결과로 제한한다. |

## 계획에 남긴 미해결 경계

- 전역 allocator OOM, host 생성 부분 실패 주입은 이 계획과 별도다.
- process 메모리 안정성은 누수 증명이 아니다. thread 종료와 반복 재생성을 주 판정으로 둔다.
- iOS 실기기와 실제 제품 session 최대치에 대한 정책은 이 실행으로 정하지 않는다.
