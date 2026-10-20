# PR 변경 실패 관점 검토 30개

검토 대상은 C11.4 누적 실기기 결과, `spec/STATUS.md` 추가 기록, 실행 근거·화면·집계 JSON이다. 각 항목은 별도 실패 경로 또는 주장 경계를 확인했다.

1. **scene 목록 개수** — 현재 capture script가 선언한 150개 ID와 집계 기대 개수가 일치한다. 통과.
2. **scene 중복** — 세 성공 run의 scene ID를 합쳤을 때 중복 없이 150개다. 통과.
3. **첫 run 분할 수** — 성공 로그가 126개이고 `typed-mixed-cycle` timeout을 성공 수에 포함하지 않는다. 통과.
4. **길이 제한 setup 실패** — 첫 실행의 80자 run ID 초과는 layout 결과와 분리해 harness 매개변수 실패로 기록했다. 통과.
5. **첫 mixed-cycle 누락** — 최초 scene의 draw marker 누락을 성공으로 세지 않고 실패 기록으로 보존한다. 통과.
6. **두 번째 mixed-cycle 누락** — 단독 probe의 draw marker timeout도 성공 집계에서 제외했다. 통과.
7. **재시도 중복 집계** — 통과한 `typed-mixed-cycle`은 단독 retest 1회만 합산한다. 통과.
8. **나머지 scene 연속성** — `typed-auto`부터 `image-span3-before-span2`까지 23개가 뒤 구간을 채운다. 통과.
9. **누적 ID 완전성** — script의 scene ID 전체와 성공 집합의 차집합이 비어 있음을 aggregate 생성 시 확인했다. 통과.
10. **기기 실행 summary** — 성공마다 run ID와 scene ID가 포함된 runtime summary가 있다. 통과.
11. **전체 node frame 수** — harness가 기대 node 수와 관찰 node 수를 scene별 비교한다. 통과.
12. **frame 좌표 허용치** — scene별 x/y/width/height를 1/64 CSS px 이내로 검사한 결과만 성공 집계에 포함했다. 통과.
13. **track 크기와 collapse** — 각 축의 track 개수·base size·collapsed 상태를 oracle 기대값에 대조한다. 통과.
14. **trace 잘림** — aggregate 생성 중 각 성공 로그의 trace chunk index가 1부터 전체 개수까지 모두 있는지 재확인했다. 통과.
15. **실제 제출 marker** — 150개 모두 `SPINON_C0410_DRAW ... presented` 로그가 있는지 확인했다. 통과.
16. **runtime assertion 거부** — 성공 집합에서 `SPINON_C114_GRID_ASSERTION_ERROR`와 argument error가 없는지 확인했다. 통과.
17. **PixelCopy scene 수** — 기대 sample 설정이 있는 9개 scene만 표면 sample scene으로 집계한다. 통과.
18. **PixelCopy 총량** — 수집 로그에서 관찰한 42개 표본 모두 `exact=true`다. 통과.
19. **색 비교 근거** — screenshot 육안 판정이 아니라 fixture 기대 RGBA와 PixelCopy 실제 channel을 비교한 로그를 사용한다. 통과.
20. **overflow 위치** — root 내부·외부, root 밖 item, item 뒤 배경 sample을 포함하는 장면 로그를 연결했다. 통과.
21. **border 범위** — 선택 표본 결과를 CSS border stroke 전체 정확성으로 확대하지 않았다. 통과.
22. **image paint 범위** — image multi-span의 `runtime_image_count=0`, `image_draw_count=0`을 밝혀 texel painting을 제외했다. 통과.
23. **설치 APK 동일성** — 세 성공 run의 `device.json`에서 로컬 APK와 설치된 base APK hash가 같다. 통과.
24. **실기기 판별** — serial, `ro.kernel.qemu=0`, model, API, ABI를 기기 metadata에 보존했다. 통과.
25. **에뮬레이터 혼동** — 이 근거는 Android API 36의 Samsung 물리 기기만 대상으로 표기한다. 통과.
26. **GPU 판별** — 기기 SurfaceFlinger의 renderer와 앱 WGPU Vulkan 로그를 함께 보존한다. 통과.
27. **화면 해상도** — 성공 캡처는 1080×2340 PNG이며 overflow·mixed·image 장면을 README에 표시한다. 통과.
28. **진단 이미지 오인** — timeout 뒤 캡처는 진단 폴더에 두고 성공 화면으로 embed하지 않았다. 통과.
29. **무결성 manifest** — SHA-256 manifest의 508개 파일을 읽어 모두 재계산했고 manifest 자신은 순환 해시에서 제외했다. 통과.
30. **문서 주장 경계** — README와 상태 대장에 C11.4 미완료, WPT·일반 CSS 미검증, 원인 미확정 timeout을 명시했다. 통과.

추가 교차 확인: README의 상대 링크 14회 참조(13개 고유 경로)가 모두 존재하고, `spec/STATUS.md`의 완료 checkbox 수는 기준 `origin/main`과 동일하게 88개다. C11.4 checkbox는 미완료 상태다.
