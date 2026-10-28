# 공개 로드맵 PR 변경 검토 30개

대상은 `spec/public/roadmap.html`의 C11.4 two-track span overflow runtime 단일 문단 교체다. 기준 구현 PR #759는 merge SHA `9937a510439e46c5a6374a6ab373edc556afe2b0`로 고정했다. 각 항목은 서로 다른 실패 경로나 상태 계약을 대조한다.

1. **구현 PR 연결** — 연결된 PR이 #759인지 확인했다. 다른 C11.4 runtime PR의 결과를 섞지 않는다.
2. **병합 메타데이터** — 시간 `2026-10-28T13:20:23Z`와 merge SHA가 GitHub PR 상태와 일치한다.
3. **구현 단위 경계** — #756은 앞선 pure Rust 단계로 남기고, #759가 앱 runtime 연결 단계임을 구별한다.
4. **opt-in 경계** — 문단은 `test-only` 및 opt-in fixture라고 명시하고 일반 profile 지원으로 확장하지 않는다.
5. **기본 profile** — `RuntimeBlockFlexGridTrackSizingV1`이 authored overflow를 일반 허용한다는 표현을 쓰지 않는다.
6. **일반 CSS 범위** — arbitrary HTML/CSS author admission은 지원 범위 밖이라고 적었다.
7. **scene 수** — 양 플랫폼에서 고정 5 scene을 실행했다고 기록하며 다른 matrix의 scene 수와 혼동하지 않는다.
8. **Android 완료 수** — Android API 37 ARM64 AVD의 결과는 5/5다.
9. **iOS 완료 수** — iPhone 17 Pro / iOS 26.2 Simulator의 결과는 5/5다.
10. **하드웨어 경계** — AVD와 Simulator 결과를 물리 기기 결과로 표현하지 않는다.
11. **Android 표면 경계** — PixelCopy 표본을 전체 surface parity나 모든 좌표 비교로 과장하지 않는다.
12. **iOS 표면 경계** — Metal drawable은 compositor 제출 전이라고 밝히며 OS 최종 표시 결과와 혼동하지 않는다.
13. **visible 기대값** — 두 track `[50,50]px`, item width 100px으로 기록했다.
14. **clip 기대값** — `clip`도 이 fixture 계산에서 content-based minimum을 유지하지만, 이 값으로 clipping 동작을 주장하지 않는다.
15. **hidden 기대값** — `hidden`은 두 track `[40,40]px`, item width 80px 그룹에 속한다.
16. **auto 기대값** — `auto`를 zero automatic minimum 그룹에 포함한다.
17. **scroll 기대값** — `scroll`을 zero automatic minimum 그룹에 포함한다.
18. **축 변환 표현** — 다섯 authored overflow 값의 computed x/y 차이는 상세 evidence에 맡겨 로드맵 문장에 불완전한 축 표를 재구성하지 않는다.
19. **MaxContent invariant** — 모든 scene에서 100px MaxContent contribution이 유지된다고 기록했다.
20. **frame/trace 경계** — 요약에 frame과 Grid track trace 연결을 포함하고 layout만 실행한 것처럼 축소하지 않는다.
21. **draw/present 경계** — app 경로를 draw/present까지 실행했다고 기록한다.
22. **clip 표본 오해** — 바깥 표본은 overflow 위치만 보여 주며 clipping semantics는 입증하지 않는다고 경고한다.
23. **scrolling 경계** — scroll offset이나 실제 scrolling 동작을 확인했다고 주장하지 않는다.
24. **scrollbar 경계** — scrollbar pixels는 미검증으로 남긴다.
25. **WPT 경계** — 고정 fixture 결과를 WPT pass 또는 전체 C11.4 적합성으로 올리지 않는다.
26. **CI 경계** — PR check rollup이 비었다는 사실을 적고 CI 통과로 바꾸지 않는다.
27. **required-check 경계** — main에 required status checks가 없다는 현재 저장소 설정과 PR 결과를 구분한다.
28. **공식 개수** — 공식 완료 수 `72/213`은 그대로라고 기록했다.
29. **상위 완료 상태** — C11.4 전체는 미완료로 유지한다고 문단에 남긴다.
30. **링크·페이지 경계** — 계획, 계획 검토, 구현 evidence, 두 화면 링크는 merge SHA에 고정했다. 공개 roadmap 한 문단을 갱신하는 작업이며 전체 docs publish나 다른 페이지 교체를 뜻하지 않는다.

모든 항목에서 로드맵 요약과 #759 실행 근거의 범위가 일치한다. 이번 공개 문서 변경은 runtime 코드나 공식 완료 checkbox를 바꾸지 않는다.
