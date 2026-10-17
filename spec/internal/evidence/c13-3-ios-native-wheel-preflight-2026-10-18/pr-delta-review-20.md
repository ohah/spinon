# C13.3 iPadOS preflight 근거 PR 변경 관점 검토 20개 · 2026-10-18

이 문서는 이번 증거·인덱스 변경을 대상으로 각기 다른 실패 가능성을 대조한다. adapter 구현이나 C13.3 완료를 검토한 기록이 아니다.

1. **다른 소스 커밋을 잘못 가리킬 위험:** `source-revision.json`의 revision과 새 worktree `HEAD`가 모두 `12e5daa75f8ae15dfd1c720e08b1ca739dcbb637`인지 확인했다. 일치한다.
2. **다른 iPad Simulator를 기록할 위험:** 이름과 UDID를 `simctl list devices` 출력에 대조했다. iPad Pro 13-inch (M5), `9F29B0ED-C7E6-46AE-A933-14E1A87FC4DF`다.
3. **OS 버전 혼동:** runtime version을 대상 simulator에서 읽었다. iOS 26.2다.
4. **fixture가 꺼진 빌드를 성공으로 셀 위험:** 원본 `build.log`의 build flags와 native feature 목록에 C04 runtime GPU 및 C13.3 fixture가 모두 `1`로 나온다.
5. **C13.3 단독 빌드가 제품 빌드처럼 보일 위험:** C13.3 gesture fixture가 opt-in이며 C04 runtime GPU fixture와 함께 켜졌음을 로그와 README에 한정해 적었다.
6. **다른 V8 source를 사용한 위험:** 저장소 pin, 사용한 V8 checkout revision, build setting이 같은 `7b50b62cb18f28617959e8452e2cd18195b38bcf`다.
7. **앱 hash가 다른 빌드에서 복사될 위험:** `SpinonBootstrap` executable을 현재 worktree의 DerivedData에서 직접 해시하고 JSON의 SHA-256과 재검증했다.
8. **화면 hash 불일치:** simulator `initial-launch.png`를 다시 해시해 JSON과 일치하는지 확인했다.
9. **잘못된 런치 경로:** 설치·실행 명령에 gesture fixture와 native preflight 인자가 모두 들어 있다.
10. **다른 시나리오의 ready marker를 오인할 위험:** 로그에서 C13.3 gesture fixture의 `SPINON_C133_READY`를 확인했다.
11. **ready를 wheel 성공으로 승격할 위험:** ready 로그는 초기 revision/offset만 제공한다. README와 plan은 callback, source, sign이 미관찰이라고 명시한다.
12. **callback 부재를 OS의 입력 거부로 오인할 위험:** 이번 run에서 scroll event를 보내지 않았음을 JSON과 보고서에 기록했다.
13. **누락된 callback을 결과 `0건`이라고 주장할 위험:** README는 입력 자체를 보내지 않아 callback 판정을 할 수 없었다고 제한한다.
14. **직접 FFI를 native 경로로 오인할 위험:** 이 run에서 event를 보내지 않았고 FFI enqueue를 호출하지 않았다.
15. **secure overlay의 창 순서를 놓칠 위험:** host window snapshot에서 전체 화면 `loginwindow` layer 2004가 Simulator 창보다 앞에 있다.
16. **전경 앱을 추측할 위험:** System Events 결과를 별도 파일에 보존했고 전경 앱은 System Settings다.
17. **창 올리기 요청 성공을 과장할 위험:** Simulator 활성화/raise 시도 뒤에도 전경 앱이 바뀌지 않아 README가 실패를 그대로 기록한다.
18. **queue/draw/pixel을 검증했다고 오인할 위험:** 이번 run에서는 queue admission, revision 변화, screenshot 비교를 수행하지 않았다고 보고한다.
19. **Simulator 근거를 물리 입력장치 근거로 확대할 위험:** 계획과 evidence 모두 실제 mouse/trackpad 및 iOS hardware 검증을 미완료로 둔다.
20. **로드맵 상태를 조기 완료로 바꿀 위험:** `spec/STATUS.md`의 C13 상태를 건드리지 않았고 plan은 iOS adapter와 C13.3 전체를 미완료로 유지한다.

## 판정

20개 관점을 확인했다. 발견된 근거 과장은 없다. 이 문서와 report는 native wheel 기능의 구현·검증 완료를 뜻하지 않는다.
