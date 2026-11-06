# C11.4 auto-row content alignment implementation PR diff review

실제 구현 변경분을 서로 다른 30개 실패 경로로 검토했다. 계획 PR diff review와 구현/runtime review의 항목을 그대로 재사용하지 않았다.

1. **Mode 범위 확대:** 새 layout enum이나 전역 Grid 정책을 추가하지 않고 `RootHtmlAutoRowStretchV1` 경로만 확장했다.
2. **기존 intrinsic profile 오염:** `RootHtmlIntrinsicTrackSizingV1`의 허용값은 바뀌지 않았다. 회귀 matrix가 유지됨을 확인한다.
3. **Computed 값 목록 오염:** parser 입력 대신 computed keyword 고정 allowlist를 사용하고 helper test가 10개 밖 값을 거부한다.
4. **computed 값 누락:** 값이 전달되지 않은 경우도 allowlist 밖으로 처리한다.
5. **baseline/safety 우회:** baseline 계열과 `safe`/`unsafe` modifier는 거부 control에서 차단한다.
6. **alias 의미 분리:** Grid의 `flex-start`/`flex-end` geometry가 `start`/`end`와 각각 일치한다.
7. **shorthand 두 번째 축 누출:** 두 `place-content` 값 순서를 시험해 `justify-content` 새 값이 우회 허용되지 않음을 확인한다.
8. **기존 2-item shape 변경:** single-text auto-row 회귀가 새 positional 값 `start`를 계속 거부한다.
9. **visible 자식 수 완화:** 새 keyword는 visible item 세 개를 요구하고 기존 direct-text item 검증을 그대로 사용한다.
10. **source order 변경:** runtime test가 fixed item 뒤의 `A`, `B` text row 순서를 보존한다.
11. **item geometry 회귀:** 세 item 모두 x/y/width/height 네 값을 독립 기대값에 대조한다.
12. **track 크기 판정 편향:** row trace에서 gutter와 collapsed track만 빼고 실제 track base를 비교한다.
13. **stretch trace 오판:** `StretchAutoTracks` 단계 수와 `applied` 값을 각각 확인한다.
14. **다른 축 placement gate 불일치:** Root one-axis placement validator의 allowlist를 대조하고 Grid matrix를 통과시켰다.
15. **projection 경로만 변경:** candidate helper와 projection rejection scope의 허용값이 일치한다.
16. **layout 경로만 변경:** layout validator와 style projection의 허용값이 일치한다.
17. **computed revision 혼합:** RuntimeSession completed key, cascade key, text source style revision을 서로 비교한다.
18. **거부 요청 일부 결과 게시:** negative controls가 거부 때 completed layout이 없음을 확인한다.
19. **거부 뒤 session 회복:** 같은 RuntimeSession에서 다음 유효 요청이 새 layout을 게시한다.
20. **기본 값 회귀:** `normal|stretch`의 기존 `[10,15,15]px` 기대 rows를 유지한다.
21. **독립 alignment matrix 불일치:** start/end/center와 세 분배값을 Chrome 관측·독립 계산과 대조한다.
22. **소수점 허용오차 과대 주장:** `space-around`만 `1/32 CSS px` 이내로 비교하고 정수·반 픽셀 좌표는 정확 기대값을 쓴다.
23. **플랫폼 입력 불일치:** Android/iOS 로그가 각 `start`, `space-between` scene에서 서로 같은 fixture SHA를 기록한다.
24. **screenshot 과대 해석:** README는 PNG를 visual smoke로 한정하고 PixelCopy/Metal readback이 없다고 밝힌다.
25. **Simulator 환경 모호성:** API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 명시하며 실기기 주장은 제외한다.
26. **candidate 코드 출처 혼동:** `source-revision.txt`가 simulator 실행 당시 base HEAD와 미커밋 Rust diff를 구분한다.
27. **기본 artifact에 test marker 유출:** release runtime/FFI artifact scan에서 새 test marker가 없다.
28. **artifact scan positive control 부재:** 동일 marker가 unit-test executable 안에는 있음을 확인한다.
29. **기존 앱·asset 손상:** 복원 로그와 설치 APK/executable SHA가 이전 값과 같아 원상 복구를 확인한다.
30. **상태·완료 수 부풀림:** `STATUS.md`와 공개 roadmap에서 C11/C11.4 미완료, 공식 `78/214`를 유지하고 plan·contract·roadmap 모두 limited candidate라 표기한다.

광범위한 test suite 실패는 candidate와 base HEAD에서 비교해 동일한 이름으로 재현되며 별도 raw log를 첨부했다. Focused matrix, scoped Clippy, rustfmt, artifact positive/negative controls, simulator source hashes와 restore hashes를 다시 대조했다. 이 diff에서 추가로 수정할 결함은 발견하지 못했다. 리뷰 범위와 검증의 한계는 [runtime README](README.md)에 기록했다.
