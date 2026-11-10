# C11.4 row-span positions 4–6 구현 PR diff 적대 검토 30개

기준 checkout은 `60679bb5a8769b384d103078e7688a345c26e7b5`다. 구현 diff와 함께 제출할 실행 근거를 서로 다른 실패 관점으로 확인한다. 구현 동작 검토는 별도 `implementation-review-20.md`에 기록한다.

1. **제품 표면 확장** — public Rust API와 C/FFI signature를 추가하지 않는다.
2. **CSS parser 확장** — parser, property allowlist 또는 일반 Grid parsing을 바꾸지 않는다.
3. **공개 지원 선언** — bounded RuntimeSession profile을 WPT 또는 전체 CSS Grid support로 부르지 않는다.
4. **상태 집계** — C11.4 checkbox나 공식 `78/214` 값을 이 implementation PR에서 완료로 올리지 않는다.
5. **내부 버전** — numeric contract `0.1.0`을 구현 범위 추가만으로 올리지 않는다.
6. **의존성** — Cargo dependency/lockfile와 Android/iOS dependency를 변경하지 않는다.
7. **기본 asset 오염** — simulator 캡처에 넣었던 candidate HTML은 앱 기본 asset에서 복원해 원본 hash를 확인한다.
8. **fixture 격리** — 새 행렬 검증은 `c11-4-grid-track-sizing-fixture-test` opt-in 테스트 대상에 남긴다.
9. **fixture 중복** — 새 six-row source는 기존 고정 fixture의 topology·cap·tail 행렬을 이용하고 제품 runtime 안에 별도 scene payload를 심지 않는다.
10. **source generator 범위** — six-row generator는 검사된 topology id와 A–E 정적 item만 만든다. arbitrary markup을 지원하는 일반 generator가 아니다.
11. **control 재작성** — baseline/candidate에서 explicit control의 min-height가 모두 0으로 유지되는지 paired scene 생성 코드를 확인한다.
12. **행렬 paired key** — each pair의 authored key, fit position, topology가 같고 minimum mode만 달라지는지 검사한다.
13. **행렬 분모** — 4,320 pair / 8,640 observation의 길이 assertion을 두어 부분 실행을 전체 성공으로 세지 않는다.
14. **내용 바닥값** — automatic-minimum 대상 수와 item frame 변화를 oracle 기대 수량과 대조한다.
15. **Source order** — 역순 DOM에 대해 semantic row와 item frame을 forward order와 대조한다.
16. **Alignment** — `start`와 `stretch`의 pair에서 non-auto row 불변과 auto row 비축소를 확인한다.
17. **Trace owner** — trace가 target grid node와 같은지, 한 개뿐인지 확인한다.
18. **Trace count** — explicit/positive implicit track 수를 별도로 검사한다.
19. **Content contribution 종류** — 단순 item size가 아니라 typed `Minimum`·`AutomaticMinimum`의 source flags를 검사한다.
20. **Mixed profile 처리** — mixed `auto`/`0` 및 control 변경이 partial completed layout을 남기지 않는지 확인한다.
21. **Same-session recovery** — 실패 뒤 새 session을 만들어 성공으로 오인하지 않고 기존 RuntimeSession에서 valid revision을 복구한다.
22. **반올림 모드 범위** — `ResidualRounding` 새 variant 선택이 7-row/single-fit/span-minimum 조건식에 국한된다.
23. **Track cap 허용** — content floor가 cap보다 큰 상태의 반올림 허용량이 한 개 1/64 단위 경계를 넘지 않는지 검사한다.
24. **Non-fit track rounding** — fit cap과 무관한 row의 기존 floor, fraction, growth potential 검증을 유지한다.
25. **Tie break 변경 범위** — later-track 우선 비교는 `UpLaterTrack`에만 적용되고 nearest와 기존 two-fit 순서를 보존한다.
26. **Stretch 후처리** — 새 variant는 기존 auto stretch 단계 뒤에 결과 오차 검사를 통과해야 publish된다.
27. **모듈 추출 가시성** — 500줄을 넘던 추적/양자화 책임을 두 내부 모듈로 나누고 바깥 API visibility를 넓히지 않는다.
28. **파일 응집도** — `track_trace.rs`, `rounding.rs`, six-row test 파일 각각 500줄 이하이며 helper를 제품과 fixture 경계에 혼합하지 않는다.
29. **플랫폼 증거 해석** — iOS/Android 스크린샷과 log는 default route visual smoke로만 설명하고 PixelCopy/Metal readback·실기기·compositor parity 주장을 넣지 않는다.
30. **리소스 정리·본문 링크** — simulator/AVD 종료, 원본 asset/APK 복구와 task temp backup 삭제를 확인했다. Agent Browser에서 commit-pinned iOS/Android light/dark PNG 네 개가 모두 complete 및 nonzero dimensions로 로드되는 것을 확인했고, 고정 Chromium 157.0.8103.40 source도 대조했다. PR 본문 link에는 PNG가 들어 있던 commit을 사용한다.

현재까지 발견된 차단 결함은 없다. Six-row paired matrix, positions 1–3 regression, focused rejection/recovery/cap tests, two-fit cap control, layout rounding unit tests, rustfmt, changed-crate Clippy와 Agent Browser image/source 확인을 끝냈다. GitHub CI rollup이 비어 있으면 CI green으로 표현하지 않는다.
