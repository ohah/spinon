# Android 실기기 text matrix 근거 검토 20개

2026-10-21 현재 `origin/main`의 Samsung SM-S731N 실기기 실행과 보존된 산출물을 서로 다른 실패 관점에서 확인했다. 이 검토는 source implementation review가 아니며, C11.4 완료 판정도 아니다.

1. **실기기와 emulator 혼동** — `ro.kernel.qemu=0`, SM-S731N model, API 36을 기기에서 읽었다. 통과.
2. **잘못된 ADB target** — 각 호출에 serial `R5KYB06SMPE`를 명시했다. 통과.
3. **OS 범위 과장** — Android 16/API 36을 metadata와 README에 동일하게 기록했다. 통과.
4. **해상도·density 누락** — 1080×2340/density 450을 기기에서 읽어 기록했다. 통과.
5. **설치 APK와 검증 코드 불일치** — build APK SHA와 설치된 `base.apk` SHA가 같다. 통과.
6. **오래된 source의 실행** — source commit `f086e54519647d51fd44c205b18fb9436099218c`을 기록했다. 통과.
7. **fixture ABI가 기본 제품 빌드로 유입** — 명령에 opt-in `C04_RUNTIME_GPU_FIXTURE_TEST` build flag가 명시되어 있고 범위를 내부 fixture로 제한했다. 통과.
8. **빌드 실패 뒤 기존 APK 재사용** — 새 build log에 `BUILD SUCCESSFUL`이 있다. 통과.
9. **scene allowlist 밖 입력** — 기존 Android activity validator가 허용하는 text 10개 scene만 실행했다. 통과.
10. **동일 run ID의 stale log 혼입** — 실행별 고유 ID와 매 실행 전 logcat clear를 사용했다. 통과.
11. **activity가 scene을 열지 못함** — 10개 launch 결과가 모두 `Status: ok`다. 통과.
12. **V8/runtime evaluation 누락** — 각 run ID의 C11.4 eval은 `status=0`으로 기록됐다. 통과.
13. **layout readiness 실패** — 10개 summary 모두 `status=0 layout=ready boxes=2`다. 통과.
14. **루트 또는 item frame 누락·순서 변경** — 각 scene의 node 1/2 frame을 모두 읽고 preorder ID를 확인했다. 통과.
15. **Chrome expected frame 오염** — 기존 고정 Chrome 155 matrix의 frame 값을 사전 기준으로 사용하고 runtime 결과에서 기준을 만들지 않았다. 통과.
16. **허용 오차 과대 설정** — 비교 허용치는 1/64 CSS px이며 이번 실행의 최대 관측 차이는 0 CSS px다. 통과.
17. **text intrinsic source 누락** — 각 trace의 direct-leaf `text` input과 `fontRegistryRevision=1`을 확인했다. 통과.
18. **trace 조각 손실·중복·잘못된 JSON** — 조각 번호 연속성과 총개수를 확인하고 재조립 JSON을 parse했다. 통과.
19. **한쪽 axis만 계산되거나 surface 제출 누락** — Columns/Rows trace와 `SPINON_C0410_DRAW status=0 presented`를 10/10 확인했다. 통과.
20. **시각·적합성 범위 과장** — 두 Android screen capture는 box geometry를 보여준다. glyph paint, 수치 PixelCopy/compositor 비교, WPT, 다른 기기군 또는 전체 CSS Grid는 확인했다고 쓰지 않았다. 통과.

증거는 [README](./README.md), [결과 JSON](./result.json), [기기·APK 기록](./device.txt), 10개 runtime log와 두 screenshot에 보존했다.
