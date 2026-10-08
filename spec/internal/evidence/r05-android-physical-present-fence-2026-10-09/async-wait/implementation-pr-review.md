# 변경 묶음 적대 검토

구현·문서·실기기 근거를 하나의 변경 묶음으로 보고, 병합 시 빠지기 쉬운 서로 다른 경계 20개를 다시 확인했다.

| # | 검토 관점 | 판정 |
|---:|---|---|
| 1 | 상태 대장이 R05.3을 완료로 잘못 체크하는가 | 완료 표시는 건드리지 않았고 async 결과 뒤에도 R05.3 미완료를 남겼다. |
| 2 | 계획 문서가 실행 전 상태로 남는가 | async 계획 상태를 구현·실기기 진단 완료로 갱신하고 범위를 구분했다. |
| 3 | 동일 수치가 계획·상태 대장·근거 문서에서 어긋나는가 | 세 파일 모두 3 process, 30 scored + 3 drain, 33/33 연결 및 callback 당시 상태 32/1을 사용한다. |
| 4 | 공개 JS 또는 플랫폼 API 계약을 실험 코드로 오해하는가 | debug 내부 진단이며 공개 API 변경은 없다. |
| 5 | debug flag를 일반 앱 동작으로 오해하게 되는가 | 명시적인 intent extra와 SDK gate를 요구한다. |
| 6 | release 코드가 debug 타입을 참조해 빌드가 깨지는가 | debug/release 동일 이름 구현을 분리했고 두 source set의 Java compile이 통과했다. |
| 7 | Android 빌드 증거가 compile만인데 APK 설치를 주장하는가 | debug APK를 실기기에 설치해 실행했고 설치 APK와 build APK 해시가 일치한다. release runtime은 주장하지 않는다. |
| 8 | 빌드 대상 V8 또는 앱 코드가 재현 불가능한가 | 고정 V8 revision, 변경 Java 파일 SHA, APK SHA를 함께 기록했다. |
| 9 | 기기 식별 정보가 불필요하게 공개되는가 | 모델/API만 보존하고 ADB serial은 manifest와 로그에서 제외했다. |
| 10 | simulator 결과를 physical device로 합산하는가 | 세 block은 모두 같은 Android physical SM-S731N 로그로만 집계한다. |
| 11 | 실제 손가락 touch라고 과장하는가 | 입력을 synthetic ADB MotionEvent로 명시하고 실제 touch 검증을 미완료로 유지한다. |
| 12 | drain 입력을 scored 표본에 더하는가 | 30개 scored와 3개 drain을 별도 표기한다. |
| 13 | 각 scored input의 submit·callback·wait 결과가 exact join됐는가 | 각 새 process에서 sequence/revision 1–11을 세 단계 log와 대조했다. |
| 14 | callback 당시 pending을 이후에도 pending이라고 과장하는가 | 32 pending·1 signaled 즉시 상태와 33 async signaled 결과를 별도로 기록한다. |
| 15 | OS fence signal을 실제 화면 광자 시각이라고 주장하는가 | event→present latency·scanout·photon 주장은 하지 않는다. |
| 16 | API 36.1의 FrameTimeline 부재를 숨기는가 | `api_below_37`, `target_vsync_id=-1` 및 VSync join 미확인을 명시한다. |
| 17 | worker 누적 또는 테스트 종료 뒤 자원이 남는가 | 마지막 결과에서 process별 pending/active/queue depth가 0이고 completed 11이다. |
| 18 | 실패 block 또는 민감한 화면 캡처가 성공 자료에 포함되는가 | foreground 실패 block은 제외했고 overlay가 담긴 이미지 하나를 폐기했다. 두 유효 화면 캡처만 연결한다. |
| 19 | 화면 설정이나 전면 앱 상태가 바뀐 채 남는가 | 화면 켜짐, brightness, timeout, stay-awake 값이 유지됐고 Chrome을 foreground로 복귀시켰다. |
| 20 | PR 문서·사이트 배포 규칙과 커밋 언어가 어긋나는가 | PR 본문은 한글, 제목 접두어만 영어로 작성한다. 본문에 Tailscale URL을 넣지 않고 GitHub Pages 배포를 실행하지 않는다. |

현재 증거 범위 안에서 병합을 막는 미해결 결과 불일치는 없다. 포화·오류 주입·API 전체·실제 touch·release runtime은 범위 밖으로 표시되어 있다.
