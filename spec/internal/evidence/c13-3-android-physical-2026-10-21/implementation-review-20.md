# C13.3 Android 실기기 입력 fixture 실행 실패 관점 검토 20개

이번 검토 대상은 opt-in Android gesture fixture 검증기와 2026-10-21 실기기 실행이다. 제품 코드는 바꾸지 않았다. 판정은 [실행 근거](README.md), [검증기](run-physical-device.mjs), `summary.json`, 캡처와 원본 logcat에 한정한다.

1. **AVD가 실기기 실행으로 잘못 계산되는가?** — 실행은 `R5KYB06SMPE` serial에 `adb -s`를 붙인다. 기기 목록에 emulator 두 대가 함께 있었지만 fixture 입력·캡처는 지정 serial로만 보냈다.
2. **ADB가 연결만 되고 승인은 안 된 기기를 통과시키는가?** — `adb devices -l`의 고정 serial 행에서 상태가 `device`인지 확인한다. 최종 기록은 authorized USB device다.
3. **serial이 다른 모델로 재할당되거나 Android 버전이 달라져도 같은 결과로 처리하는가?** — 모델 `SM-S731N`, Android `16`, API `36`을 assertion으로 고정했다. 최종 기기 정보가 기대값과 일치한다.
4. **해상도나 density가 달라 화면 좌표와 CSS 좌표 변환이 어긋나는가?** — `1080×2340`, density `450`, surface X/Y 균일 scale `2.8125`를 확인한다. 현재 물리 좌표와 CSS 좌표 변환이 일치했다.
5. **실기기에는 이전 APK가 설치됐는데 로컬 APK 결과로 오인하는가?** — 설치된 package APK와 build output APK의 SHA-256을 비교한다. 양쪽 모두 `50c92c48…9922c7`이다.
6. **앱이 떠도 opt-in gesture fixture가 준비되기 전에 입력을 보내는가?** — 새 process ID의 PID별 logcat에서 `READY`, `EVAL_READY`, ready surface bounds를 모두 기다린다. 두 scenario 모두 이 gate를 통과했다.
7. **SurfaceView의 음수 screen origin을 잘라내 입력이 엉뚱한 곳에 가는가?** — signed integer parser가 `x=-22`를 유지하고 screen 좌표로 변환한다. 초기 시도에서 이 parser 문제를 발견해 고친 뒤 최종 실행했다.
8. **0 크기나 비균일 surface scale을 정상 좌표로 받아들이는가?** — 양의 scale과 X/Y 차이 `≤0.01`을 assertion으로 검사한다. 최종 surface는 `1125×675`에서 균일 scale이었다.
9. **scrollport 일부가 display 밖인데 pixel 영역 산정이 비교를 과장하는가?** — 비교 전 root scrollport 전체가 화면에 보이는 surface 교집합 안에 있는지 확인한다. 이번 visible scrollport는 `(23,986)..(1058,1571)`이었다.
10. **합성 swipe 시작·끝점이 물리 display 바깥에 있는가?** — 양성·대조 좌표 각각 display 경계를 검사한다. `(400,1363)→(400,1110)` 및 5px 대조점은 모두 화면 안이었다.
11. **ADB 합성 입력을 사람 손가락 raw touch로 과장하는가?** — 입력 명령, summary, README는 `adb shell input swipe`와 synthetic MotionEvent라고 명시한다. callback의 source/tool 값은 관찰했지만 실제 사람 접촉이나 optical scanout은 주장하지 않는다.
12. **큰 swipe가 native touch slop을 넘지 않아도 positive로 세는가?** — 양성 callback에 `pan-began` 및 `ViewConfiguration.scaledTouchSlop=23px`이 기록됐다. pointer tool은 finger로 보고됐다.
13. **DOWN·UP 또는 pan callback이 없어도 중간 offset만 보고 성공 처리하는가?** — 양성 gate가 down, pan-began, up callback을 각각 요구한다. 세 단계 모두 로그에 있다.
14. **native callback이 worker 완료를 동기 대기하는가?** — pan-began callback의 `waits_for_worker=false`를 assertion으로 확인한다. callback은 비대기 enqueue 경로였다.
15. **bounded queue 입력이 worker까지 도달하지 않는데 일부 touch 로그만으로 성공하는가?** — 성공 status의 processed batch가 하나 이상 있어야 하며 최종 run은 29개였다. 완료되지 않은 process error는 별도 failure marker로 거부한다.
16. **worker 처리 뒤 stale revision이 draw에 남는데 화면 경로를 성공 처리하는가?** — 마지막 processed revision과 마지막 draw revision이 같아야 한다. 최종 revision `28`이 draw에 게시됐다.
17. **positive fixture가 경계가 아닌 다른 시작 상태에서 remainder 결과를 만드는가?** — 시작 root/outer는 0, inner는 `(86,106)`인지 검사한다. 로그의 initial snapshot이 지정 boundary와 일치했다.
18. **scroll delta가 inner나 root로 잘못 가거나 수평축으로 새는가?** — 최종 root와 inner가 시작값 그대로이고 outer X도 그대로이며 outer Y만 증가해야 한다. 최종 결과는 outer Y `88.42639 CSS px`였다.
19. **5px below-slop 대조가 일부 move에서 pan을 시작하거나 revision을 바꾸는가?** — 모든 below-slop move가 `accepted=false`, pan-began 부재, 모든 owner offset과 revision 불변을 검사했다. 최종 결과는 시작 revision 0 그대로였다.
20. **스크린 비교가 scrollport 밖 누출, 잘못된 PNG 크기, 앱 crash를 놓치는가?** — 시작/종료 PNG 크기를 display와 비교하고 visible scrollport 밖 변경 0을 요구한다. 대조 PNG hash가 같고, 최종 log에는 fatal exception·fixture failure·nonzero process status marker가 없으며 앱이 foreground에 남았다.

검토 과정에서 검증기는 signed SurfaceView origin 처리와 ready/draw offset 표기 차이를 지원하지 못해 두 번 준비 단계에서 실패했다. 두 parser를 고친 뒤 boundary 시작값, inner 유지·outer 전달, below-slop 무변화와 오류 marker 검사를 추가했다. 최종 실기기 실행은 새 assertion을 모두 통과했다. surface가 좌우 `22/23px` 잘린 상태는 fixture 폭 한계로 남기며, 단일 기기·합성 입력 결과를 일반 제품 지원이나 raw-touch 검증으로 넓히지 않는다.
