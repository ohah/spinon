# PR #1295 병합 후 문서 diff 실패 관점 검토 30개

검토 대상은 `spec/STATUS.md`, 내부 계약 0059, C11.4 계획, `spec/public/roadmap.html`, Tailnet 제공·검증 기록이다. 각 항목은 다른 불변 조건이나 실패 경로를 확인한다.

1. **구현 PR 식별** — #1295를 기존 계획 PR #1294 또는 이전 구현 PR #1292와 혼동하지 않는다. 현재 링크와 문구는 #1295를 가리킨다. **통과**
2. **merge 시각** — `2026-11-08T20:35:50Z`를 UTC로 보존한다. **통과**
3. **merge revision** — 기록 SHA `40b416b4de4942c9251db0f4862626443310d092`가 현재 `HEAD` 및 `origin/main`과 같다. **통과**
4. **C11.4 부모 체크** — bounded slice merge를 상위 항목 완료로 승격하지 않는다. checkbox는 미완료다. **통과**
5. **C11 부모 체크** — Grid 일부 slice를 C11 완료로 승격하지 않는다. checkbox는 미완료다. **통과**
6. **공식 분자·분모** — 상태 대장, 계획, 계약 및 roadmap이 `78/214`를 유지한다. **통과**
7. **계약 버전** — 내부 계약 `0.1.0`을 구현 후보 병합만으로 올리지 않는다. **통과**
8. **API 범위** — 공개 API나 FFI를 추가했다고 문서가 암시하지 않는다. **통과**
9. **Track list 재사용 경계** — 여섯-member repeated intrinsic list를 기존 profile로 제한한다. **통과**
10. **Text grammar 경계** — 임의 text/font/content 지원을 주장하지 않는다. **통과**
11. **Row placement 경계** — row 2/span 2, row 3/span 2, 선택 topology의 row 2/span 3만 기재한다. **통과**
12. **Column placement 경계** — 모든 text item이 column 1에 고정되고 implicit column 지원으로 일반화되지 않는다. **통과**
13. **Topology 수** — 빈 explicit item과 2/3개 text wrapper의 네 고정 topology 범위가 어긋나지 않는다. **통과**
14. **Percentage basis** — `%` cap basis가 definite 220px block size라는 계약을 보존한다. **통과**
15. **Alignment scope** — `start|stretch` 범위를 다른 alignment 값으로 넓히지 않는다. **통과**
16. **Chrome corpus 분모** — 4 topology × 30 순서 × 4 cap × 2 alignment = 960 scene을 810 회귀 corpus와 구별한다. **통과**
17. **Runtime scene 분모** — 960 RuntimeSession scene 결과를 OS simulator 960 parity로 확대 해석하지 않는다. **통과**
18. **Reject denominator** — 35 unsupported shapes가 atomic rejection matrix임을 명시하고 성공 장면 수에 더하지 않는다. **통과**
19. **Recovery evidence** — unsupported input 뒤 같은 session의 valid recovery를 별도 성질로 기록한다. **통과**
20. **Writing-mode cascade** — vertical-writing cascade 거부가 검증됐다는 점과 일반 writing-mode 지원이 아님을 구분한다. **통과**
21. **Layout stage tests** — Layout 16/16 및 trace atomicity 2/2를 서로 다른 검증으로 기록한다. **통과**
22. **Grid regression count** — Grid track sizing 127/127을 모든 layout crate 테스트로 표현하지 않는다. **통과**
23. **Style projection count** — 2/2를 전체 CSS projection coverage로 확대하지 않는다. **통과**
24. **Runtime trace count** — feature trace 4/4 및 기존 810-scene 회귀 1/1을 분리한다. **통과**
25. **Clippy/rustfmt scope** — 변경 crate Clippy와 rustfmt만 통과했다고 적고 workspace CI로 부르지 않는다. **통과**
26. **Broad suite 실패** — `spinon-layout --lib` 506 passed와 C15 HostDocument collection test 1 failed를 함께 보존한다. **통과**
27. **Baseline 제한** — C15 실패를 baseline-confirmed 또는 새 회귀라고 단정하지 않는다. **통과**
28. **GitHub checks** — PR check rollup이 비었다는 사실을 CI 성공과 혼동하지 않는다. **통과**
29. **OS 증거 등급** — Android API 37.2 ARM64 AVD 및 iOS 26.2 Simulator 화면을 visual smoke로 제한하고 실기기·hardware GPU·PixelCopy/Metal readback으로 확대하지 않는다. **통과**
30. **로드맵·게시 상태** — raw roadmap 문구와 선택한 RSPress 페이지를 함께 게시하고 삭제·서버 restart 없이 원본/제공본 hash를 검증한다. GitHub main을 읽는 칸반 데이터는 문서 PR merge 후 별도 확인해야 하므로 이 검토에서는 최종 dynamic-board parity를 주장하지 않는다. **통과 — 후속 확인 명시**
