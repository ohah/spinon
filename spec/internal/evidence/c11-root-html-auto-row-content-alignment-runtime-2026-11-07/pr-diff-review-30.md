# C11.4 auto-row content alignment implementation PR diff review

실제 구현 변경분을 서로 다른 30개 실패 경로로 검토했다. 계획 PR diff review와 구현/runtime review의 항목을 그대로 재사용하지 않았다.

1. **Mode 폭 확장:** 새 layout enum variant나 전역 Grid 정책 변경 없이 `RootHtmlAutoRowStretchV1` 분기만 확장한다. diff가 이를 지킨다.
2. **Legacy intrinsic sizing 오염:** `RootHtmlIntrinsicTrackSizingV1` 허용값은 바뀌지 않았다. regression matrix에서 확인한다.
3. **Computed value 목록 오염:** parser 입력 대신 computed keyword의 고정 allowlist를 사용하며 10개 밖 값을 helper test에서 거부한다.
4. **Null computed value:** 값이 없는 경우도 허용목록 밖으로 처리한다.
5. **Baseline/safety 값의 우회:** baseline 계열과 safe/unsafe modifier는 test controls에서 거부한다.
6. **Alias의 별도 의미:** Grid의 `flex-start`/`flex-end`가 start/end 기대 geometry와 일치한다.
7. **Shorthand second-axis leak:** 두 `place-content` 순서가 모두 검증돼 `justify-content` 신규 값을 실수로 허용하지 않는다.
8. **Old two-item shape 변경:** single-text auto-row regression은 새 positional value를 거부한다.
9. **Visible child-count 약화:** 새 keyword에서 세 visible items를 강제하고 기존 exact direct-text item validator를 유지한다.
10. **Source order 변화:** runtime test는 fixed item 다음 `A`, `B` text row 순서를 보존한다.
11. **Item geometry 회귀:** 세 item 모두 frame의 네 scalar를 독립 기대값과 비교한다.
12. **Track size assertion 편향:** row trace에서 gutter/collapsed track을 제외하고 실제 track bases를 비교한다.
13. **Stretch trace 오독:** `StretchAutoTracks` stage 개수와 `applied`를 각각 확인한다.
14. **Wrong-axis placement gate:** root one-axis placement validator도 같은 값 집합인지 diff 대조 및 Grid matrix로 확인한다.
15. **Projection-only path:** candidate helper와 projection rejection scope의 allowlist가 일치한다.
16. **Layout-only path:** layout validator의 허용값은 style projection gate와 일치한다.
17. **Computed style revision mismatch:** RuntimeSession completed key와 cascade key, text source style revision을 비교한다.
18. **Failed request partial publish:** 거부 control은 completed layout이 비었는지 검사한다.
19. **Session recovery:** 거부 뒤 valid request가 같은 session에서 새 layout을 게시한다.
20. **Existing baseline regression:** `normal|stretch` old expected geometry stays `[10,15,15]px`.
21. **Independent values matrix:** start/end/center와 세 distribution value가 plan의 Chrome observation 및 독립 산술과 일치한다.
22. **Subpixel rounding claim:** `space-around` tolerance is capped at `1/32 CSS px`; other case expectations are exact integer or half-pixel coordinates.
23. **Platform input drift:** Android/iOS logs carry byte-identical `start` fixture SHA and byte-identical `space-between` fixture SHA.
24. **Simulator smoke overclaim:** README limits PNG interpretation to visual smoke and names absent PixelCopy/Metal readback.
25. **Simulator device ambiguity:** Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator are named; physical-device claims are excluded.
26. **Candidate code attribution:** `source-revision.txt` distinguishes base HEAD from the uncommitted Rust diff used by simulator builds.
27. **Default artifact leakage:** new test marker scan is absent in release runtime and FFI artifacts.
28. **Artifact scan positive control:** the exact marker is present in the unit-test executable.
29. **Build-state damage:** raw restore logs and installed APK/executable hashes show previous app installs and bootstrap assets restored.
30. **Status/count inflation:** `STATUS.md` and the public roadmap keep C11/C11.4 unchecked and official count `78/214`; plan, contract and roadmap all call this a bounded candidate.

광범위한 test suite 실패는 candidate와 base HEAD에서 비교해 동일한 이름으로 재현되며 별도 raw log를 첨부했다. Focused matrix, scoped Clippy, rustfmt, artifact positive/negative controls, simulator source hashes와 restore hashes를 다시 대조했다. 이 diff에서 추가로 수정할 결함은 발견하지 못했다. 리뷰 범위와 검증의 한계는 [runtime README](README.md)에 기록했다.
