# PR #1156 mixed column-flow 변경분 적대 검토 30회

제품 변경 검토 기준은 `origin/main`의 `c925c8a5253fddd305a4bb74f92e5bff9c761a21`부터 구현·계약 문서 head `44b0b1a80984bc0bbad04abb098d14b19c956964`까지다. 런타임 코드, fixture, 모바일 실행 adapter, 계약·상태 문서를 서로 다른 실패 경로로 확인했다. 이 검토 기록과 0059·상태 대장의 검토 링크 갱신은 evidence-only 문서 변경이며 지원 범위·완료 상태를 바꾸지 않는 것을 1·2·30번 관점에서 다시 대조했다. PR 본문·라벨은 제출 직전 별도로 대조한다.

1. **검토 변경이 C11 완료 수를 올리거나 상위 체크를 바꾸는가?** 통과 — C11 상위 체크와 공식 완료 `72/213`은 유지되고, `spec/STATUS.md`에 미완료 제한 후보로만 기록됐다.
2. **내부 계약의 숫자 버전 또는 공개 API 약속이 바뀌는가?** 통과 — 0059의 `0.1.0`과 기존 공개 API 경계가 유지된다.
3. **Parsed Root HTML 이외의 임의 문서가 제한 profile에 들어오는가?** 통과 — typed computed-style snapshot과 `RuntimeGridNodes`의 후보 판정 아래에서만 선택한다.
4. **여러 Grid owner 또는 중첩 Grid가 한 후보로 섞이는가?** 통과 — column-flow 후보 하나와 Grid container 하나만 허용하고, item validator는 중첩 Grid를 거부한다.
5. **dense 또는 다른 auto-flow가 sparse column-flow로 오인되는가?** 통과 — 별도 enum 값의 `Column`만 허용하며 dense 경로와 fixture allowlist가 분리돼 있다.
6. **row-locked 또는 column-locked 한쪽만으로 혼합 모드에 들어오는가?** 통과 — layout validator가 양쪽 locked item 존재를 모두 요구하고, 양축 배치 단위 테스트가 한쪽만 있는 입력을 거부한다.
7. **fully definite와 fully auto item이 혼합 scene에서 분류 오류를 내는가?** 통과 — 두 분류를 별도로 인정하고 4 placement phase의 고정 frame을 검사한다.
8. **column-flow가 row-flow의 cursor 방향을 재사용하는가?** 통과 — `RootHtmlMixedCrossAxisColumnAutoPlacementV1` 전용 경로가 column 정규화를 호출하며 별도 배치 테스트가 있다.
9. **한 축 span이 cursor 진행 또는 점유 판정을 어긋나게 하는가?** 통과 — 두 축 span, 앞선 점유 영역, 다음 sparse 후보를 함께 고정하는 layout case가 있다.
10. **음수 explicit line이 명시 Grid 끝 기준과 다르게 해석되는가?** 통과 — row·column 음수 line을 각각 포함한 배치 case가 기대 frame과 대조된다.
11. **explicit grid 바깥 placement가 implicit track을 조용히 추가하는가?** 통과 — row/column 경계 control은 모두 오류가 나고 runtime은 완료 snapshot을 내지 않는다.
12. **큰 track 수나 배치 개수에서 상한·정수 변환을 우회하는가?** 통과 — `GRID_MAX_TRACKS` 검증과 checked count, track 내부 위치 검증이 유지된다.
13. **intrinsic, fraction, percentage, named line 또는 repeat track이 px-only explicit grammar를 우회하는가?** 통과 — style admission과 layout validator 양쪽에서 finite non-negative explicit px만 통과한다.
14. **자동 track이 제한 밖에서 생성되거나 크기가 달라지는가?** 통과 — `grid-auto-rows:auto`, mixed column 모드의 `grid-auto-columns:auto`를 강제하고 implicit extent를 거부한다.
15. **container의 gap, edge, 정렬, overflow 또는 writing mode가 단순 배치 가정에 섞이는가?** 통과 — 공통 제한 validator가 fixed container·zero gap/edge·visible overflow·LTR `horizontal-tb` 범위를 검사한다.
16. **item의 min/max, aspect ratio, CSS math 또는 비고정 크기가 span geometry를 바꾸는가?** 통과 — projection과 layout item validator 모두 fixed finite px 크기를 요구한다.
17. **nonempty, nested, positioned 또는 intrinsic item이 빈 direct div profile을 우회하는가?** 통과 — HostDocument item kind/children과 layout item 구조를 각각 검증한다.
18. **`visibility:hidden`이 배치 점유를 잃거나 paint box로 노출되는가?** 통과 — 두 lock class의 hidden qualifier scenes에서 frame은 유지되고 render box와 pixel sample은 제외된다.
19. **`display:none` item이 점유 셀이나 결과 frame 수에 포함되는가?** 통과 — runtime test는 frame 부재를, 양 플랫폼 matrix는 non-participating scene과 pixel 제외를 검사한다.
20. **`order` 또는 positioned item이 DOM order phase를 바꾸고도 성공하는가?** 통과 — nonzero order와 non-static position을 명시적으로 거부한다.
21. **item이 다른 HostDocument parent의 node인데 해당 Grid의 child처럼 처리되는가?** 통과 — computed owner와 직접 자식 handle이 일치하는 항목만 세고, container handle 누락은 진단 오류다.
22. **단일 후보 검사에서 두 번째 column-flow owner를 놓치는가?** 통과 — iterator의 두 번째 후보와 전체 Grid owner 수를 모두 검사한다.
23. **typed computed placement가 없거나 네 endpoint 중 하나가 비수치 문법이어도 admission되는가?** 통과 — projection은 지원 문법으로 분류할 수 없는 placement를 후보에서 제외하고 item validator는 placement 부재를 거부한다.
24. **실패 원인이 모호하거나 오류 뒤 부분 결과가 남는가?** 통과 — row·column 각각 property와 node ID를 단위/runtime 검사로 대조하고 실패 시 `completed`가 없는 것을 확인한다.
25. **Android 앱 density가 틀려도 CSS geometry pixel 좌표가 통과하는가?** 통과 — 실제 display density를 ADB `wm density` 420 dpi와 교차 대조하며 2.625×를 사용한다.
26. **iOS 표면 크기를 키우거나 로그의 scale만 속여도 픽셀 검사가 통과하는가?** 통과 — Metal drawable의 양축을 160×90 CSS viewport와 대조하고 forged 6× case를 거부한다.
27. **reference, fixture 또는 oracle 버전이 바뀐 결과를 같은 결과로 섞는가?** 통과 — Chrome 156 capture, expected JSON, positive/rejection fixture, generator의 SHA-256을 matrix 입력에 고정한다.
28. **색 sample이 item 밖이나 잘못된 pixel center에서 맞아도 surface 검증으로 보고하는가?** 통과 — 31 item별 CSS 중심 좌표와 authored opaque sRGB RGBA를 Android PixelCopy 및 iOS Metal readback에서 비교한다.
29. **오래된 app launch나 scene alias의 report가 현재 실행 결과를 가장하는가?** 통과 — 제한 scene ID allowlist, 새 app process/PID, run ID, device report를 묶어 matrix 결과를 기록한다.
30. **시뮬레이터 근거가 hardware GPU·post-compositor·WPT·CI·전체 C11 완료로 과장되는가?** 통과 — Android는 software AVD, iOS는 compositor 전 Metal readback, WPT 미실행, GitHub check 없음으로 계약·상태·PR 본문에 각각 한계를 기록한다.

## 검토 결과

30개 관점에서 미해결 변경 결함은 찾지 못했다. 앞서 구현 review에서 찾은 column diagnostic 문구, iOS surface scale 신뢰, Android density 교차검증 문제는 수정 후 해당 검사를 다시 확인했다. 실행 세부와 제한은 [Android/iOS runtime 근거](./README.md), 구현 review는 [별도 20개 관점 기록](../c11-root-html-column-flow-mixed-implementation-review-20-2026-11-04.md)에 둔다. GitHub PR의 check rollup과 workflow run은 비어 있고, main required status checks도 설정되어 있지 않아 CI 통과로 기록하지 않는다.
