# C11.4 nested Grid two-axis 계획 PR diff 실패 관점 검토 30개

계획·oracle 추가와 완료된 roadmap 기록 수정의 최종 diff를 30개의 서로 다른 실패 관점으로 읽었다. 검토 중 발견한 링크 오류는 수정하고 해당 링크 관점을 다시 확인했다.

1. **Evidence → plan 링크 깊이** — README의 계획 문서 상대 경로가 한 단계 짧아 저장소 루트 밖을 가리켰다. `../../../../plan/...`로 고치고 대상 파일 존재를 확인했다.
2. **Evidence → HTML fixture 링크** — README 상대경로가 `tests/fixtures/...`로 해석되는지 확인했다. 파일 존재를 확인했다.
3. **Ahem font URL** — HTML의 상대 font URL이 실제 `tests/fixtures/resources/c15/Ahem.ttf`를 가리키는지 확인했다. Agent Browser에서 `ahemReady=true`였다.
4. **Ahem asset drift** — 계획 JSON과 SHA manifest가 고정 repository font를 참조하는지 확인했다. `shasum -c`에서 manifest 항목이 모두 일치했다.
5. **Font readiness race** — 측정값이 font fallback 전에 기록될 위험을 확인했다. capture script가 `document.fonts.ready`를 기다리고 `document.fonts.check`를 저장한다.
6. **min-content text metric** — longest word 대신 전체 text width를 잘못 기대할 위험을 확인했다. 독립 계산은 70px longest unbreakable word를 사용한다.
7. **max-content text metric** — collapsible space advance 누락을 확인했다. 독립 계산은 full unwrapped advance 100px을 명시한다.
8. **min-content block contribution** — wrapping이 row 기대값과 연결되지 않을 위험을 확인했다. 70px에서 두 10px line, 20px block을 고정했다.
9. **max-content block contribution** — 100px scene이 두 줄이 될 위험을 확인했다. 한 10px line을 고정했다.
10. **축 계산 순서** — 행이 열보다 먼저 계산된다고 가정할 위험을 확인했다. 근거와 JSON에서 columns-before-rows를 분리해 적었다.
11. **Grid container intrinsic size 규범** — CSS Grid container 크기가 frame 우연값으로 추정될 위험을 확인했다. 2025 CRD §5.2 링크를 넣었다.
12. **track sizing overview 규범** — 열/행 순서가 없는 citation을 쓸 위험을 확인했다. 2025 CRD §11.1 링크를 추가했다.
13. **Chrome과 규범 계산 혼합** — Chrome을 독립 기대값 근거로 취급할 위험을 확인했다. 독립 JSON과 browser observation을 별도 파일·서술로 유지했다.
14. **두 축 동시성** — 새 계획이 기존 width-only 범위를 반복할 위험을 확인했다. width/height와 columns/rows를 동일 constraint로 고정했다.
15. **한 축만 intrinsic인 scene** — two-axis feature를 증명하지 않는 조합이 allowlist에 섞일 위험을 확인했다. 네 개 scene만 명시했다.
16. **depth-one control 손실** — 깊이 차이가 size 변화의 원인인지 분리하지 못할 위험을 확인했다. min/max 각각에 depth-one control을 유지한다.
17. **depth-two topology** — middle Grid가 빠져도 scene이 통과할 위험을 확인했다. root → middle → inner → text 구성을 계획에 명시했다.
18. **item count 확장** — 복수 child 처리까지 지원한다고 오인할 위험을 확인했다. 단일 direct child topology와 exact order 검사로 제한했다.
19. **track grammar 확장** — 복수·implicit track을 실수로 포함할 위험을 확인했다. 양 축 모두 단일 track만 허용한다고 명시했다.
20. **layout style 경계** — 계획이 일반 CSS projection admission으로 읽힐 위험을 확인했다. opt-in fixture profile의 네 고정 scene만 대상으로 했다.
21. **오류 후 partial layout** — projection 거부가 부분 track/frame을 게시할 위험을 확인했다. `LayoutOutput`과 trace fail-closed 조건을 적었다.
22. **node/revision 혼합** — 부모와 자식의 다른 style revision이 한 결과로 결합될 위험을 확인했다. 정확한 owner/topology와 같은 입력 revision을 검사한다.
23. **track/frame assertion 누락** — root frame만으로 nested 오류를 놓칠 위험을 확인했다. 모든 Grid의 두 track, 크기와 text frame을 판정 대상으로 삼았다.
24. **page offset 오판** — HTML section의 absolute y 좌표를 CSS 계산 결과로 오인할 위험을 확인했다. root-relative origin만 판정한다.
25. **subpixel tolerance** — browser serialization 오차가 테스트를 흔들 위험을 확인했다. 1/64 CSS px tolerance를 JSON과 계획에서 일치시켰다.
26. **screenshot 의존 주장** — 스크린샷을 track·pixel conformance 증거로 과장할 위험을 확인했다. 수치 JSON을 별도로 두고 screenshot은 시각적 fixture 보조자료로 제한했다.
27. **미검증 runtime 주장** — Chrome 관측을 Spinon 결과라고 오해할 위험을 확인했다. Evidence README와 plan 모두 Spinon 구현 증거가 아니라고 구분한다.
28. **platform scope 과장** — 사용자 지시와 달리 physical device gate를 암시할 위험을 확인했다. Android AVD/iOS Simulator를 명시하고 physical device, GPU·compositor pixels를 제외했다.
29. **roadmap 병합 상태 drift** — #408/#419/#422/#425/#428/#448의 미완료 문구가 남을 위험을 확인했다. live merge SHA·시각에 맞춰 관련 계획 문서와 상위 roadmap 문구를 갱신했다.
30. **완료 수·계약 버전 변경** — plan PR이 제품 완료나 interface version bump를 암시할 위험을 확인했다. C11.4 `72/213`과 interface `0.1.0`을 유지하고 status ledger를 완료로 바꾸지 않았다.

**결과:** 링크 경로 수정 후 로컬 대상 확인, HTML/Ahem browser readiness, Chrome 관측, SHA manifest 검증을 다시 확인했다. 남은 차단 결함을 찾지 못했다. 이 PR diff 검토는 계획 검토 20개와 이후 구현/runtime 검토 20개를 대체하지 않는다.
