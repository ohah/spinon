# C12.4 후속 검증 정합성 검토

**대상:** 후속 시뮬레이터 실행 기록, 상태 대장·내부 인덱스·공개 로드맵의 C12.4 상태, 미완료 경계

1. **병합 상태가 실제 GitHub와 일치하는가?** `gh pr view 141`은 `MERGED`이며 merge commit은 `5ed036b36e1a08c62b0bcef82a3e93bc98d67804`다. 세 문서의 “검토 중”을 “리베이스 병합”으로 수정했다.
2. **C12.4를 전체 완료로 잘못 체크하는가?** 체크박스는 미완료로 유지하고 제한 profile과 전체 CSS 지원을 분리했다.
3. **WPT 실행 결과를 Spinon 적합성으로 부풀리는가?** 자동 4개와 별도 수동 1개는 Chrome 154 oracle 결과로만 기록하며, Spinon WPT runner가 없다는 경계를 유지했다.
4. **iOS 표본이 screenshot/offscreen 결과를 surface 결과로 오인하는가?** 새 comparison의 renderer는 `metal-surface`; 실제 WGPU `CAMetalLayer` surface texture를 판독했다.
5. **iOS 표본 수와 오차를 과장하는가?** 8/8 case가 channel 오차 0으로 일치했으며 전체 scene 픽셀 동일성을 주장하지 않는다.
6. **reverse 방향 조합을 빠뜨렸는가?** `row-reverse` 2개와 `column-reverse` 2개가 새 8-case 표본에 포함됐다.
7. **실패 표본을 성공으로 덮었는가?** 이전 실패 run은 보존하고 별도 성공 run ID `c124-ios-20261011T133240528Z-d5df9bb0`를 기록했다.
8. **수정된 reverse fixture가 앱 경로에서 실행됐는가?** 새 iOS 26-scene matrix 입력에서 `flex-row-reverse-paint`를 확인했고 실행 상태는 `matched`다.
9. **scene 수와 node 수를 맞게 기록했는가?** 새 matrix는 26 scene·77 source node를 넣고 64 supported node를 비교했다.
10. **기대 거부를 성공으로 세는가?** 지원 21개와 예상 거부 5개를 별도 집계했다.
11. **geometry와 pixel 검사를 같은 결과로 합치는가?** matrix는 geometry·hit-test이고, Metal surface 실행은 8개 RGBA 표본이다.
12. **허용오차와 실제 오차를 혼동하는가?** matrix 최대 오차는 0 CSS px, 허용치는 0.5 CSS px다.
13. **Android 결과를 hardware GPU로 잘못 부르는가?** API 37 AVD renderer는 ANGLE/SwiftShader software GL로 식별됐다.
14. **실기기 측정을 했다고 주장하는가?** 이번 재실행은 iOS Simulator와 Android AVD뿐이며 실기기 RSS/GPU는 남아 있다.
15. **동시 session stage/반복을 생략했는가?** 양 플랫폼 모두 0·1·2·4·8 stage에서 process memory를 세 번 수집하고 해제했다.
16. **8 MiB 요청을 실제 RAM commit으로 해석하는가?** 이는 worker stack requested/reservation 값이며 resident·committed bytes가 아니다.
17. **process memory를 worker stack 비용으로 귀속하는가?** PSS/RSS/footprint에는 V8·allocator·OS runtime이 포함되어 worker 전용 값으로 해석하지 않는다.
18. **mincore 표본을 stack high-water로 오인하는가?** 순간 residency와 pthread bounds만 보고하며 stack-only commit/high-water는 미측정이다.
19. **기본 test thread의 256-depth crash를 통과로 숨기는가?** 실패 관측을 유지하고 앱 worker 경계 결과와 분리했다.
20. **worker의 1,024/1,025 결과와 `Vec::try_reserve` fault injection을 일반 보장으로 확장하는가?** 8 MiB worker에서 1,024 성공·1,025 거부·revision 복구 범위만 주장하고, 특정 painter reserve 실패 주입은 전역 allocator/V8 OOM 복구를 보증하지 않는다고 남겼다.

**수정한 정합성 문제:** 후속 실행이 이미 있었는데도 이전 요약에서 일부를 미실행으로 적었다. 또한 상태 대장·내부 인덱스·공개 roadmap의 PR #141 상태가 병합 후에도 “검토 중”으로 남아 있었다. 새 iOS/AVD 실행을 재현하고 병합 상태를 동기화했다. 나머지 실기기·전역 OOM·WPT runner 경계는 해소한 것처럼 보고하지 않는다.
