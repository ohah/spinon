# C12.4 동시 session stack 비용 시뮬레이터 관측

- 실행 ID: c124-ios-20261011T111339143Z-8d16f3bf
- 플랫폼: ios
- 단계: 0, 1, 2, 4, 8 sessions; 각 단계에서 process memory를 두 차례 수집했다.
- session당 CSS worker의 stack 요청: 8 MiB. 8개 시점의 합계 예약은 64 MiB다.
- 프로세스 memory는 V8과 플랫폼 런타임 등도 포함하며 CSS worker에 직접 귀속되지 않는다.
- 실제 기기의 RSS나 thread별 stack commit은 측정하지 않았다.
- 원본: `build.log`, `device.txt`, process 로그, stage별 footprint/vmmap 또는 dumpsys/proc 파일.
