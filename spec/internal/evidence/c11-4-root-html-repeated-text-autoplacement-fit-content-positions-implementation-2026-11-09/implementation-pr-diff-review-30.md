# 반복 text auto-placement fit-content 위치 구현 PR 변경분 검토 30개

계획 PR 검토와 분리해 실제 구현 코드·fixture·증거 변경분만 대상으로 검토한다. 코드 경로, 입력 조합, 실패 원자성, 기존 C11.4 계약과 문서가 일치하는지 각 항목을 독립적으로 확인한다.

1. **변경 범위** — 기본 Root HTML `RootHtmlFitContentAutoRowsV1`의 제한 profile이며 새 public CSS API나 FFI가 없다.
2. **목록 길이** — Style projection과 Layout admission이 여섯-member list만 대상으로 삼는다.
3. **키워드 개수** — intrinsic member의 exact multiplicity 검증과 invalid-count control이 함께 있다.
4. **위치 보존** — 새 fit position 2–6을 포함하고 기존 position 1도 대조군으로 남겼다.
5. **Cap 범위** — 기존 finite non-negative px/percentage admission만 연결하고 `calc()`를 넓히지 않는다.
6. **Span predicate 분리** — row-span profile은 기존 위치 1–3 gate를 유지한다.
7. **Auto-placement topology** — 고정 9개 direct children, 8개 implicit item과 1개 explicit item의 구별이 유지된다.
8. **Payload grammar** — direct text와 고정 computed text style의 allowlist가 새로 광범위해지지 않는다.
9. **Cycle index** — explicit row가 반복 pattern을 한 칸 밀지 않는다는 비교를 추가한다.
10. **Cross-product 크기** — 6 positions × 30 tails × 4 caps × 2 alignments = 1,440을 fixture test에서 완전성 검사한다.
11. **Scene uniqueness** — matrix key의 중복과 누락을 같은 예상 scene 수 외에도 별도 set으로 확인한다.
12. **Chrome metadata** — browser revision, OS, DPR, viewport와 serialized input metadata가 고정되어 있다.
13. **독립 기대값** — Chrome 관측을 그대로 expected output으로 재사용하지 않고 fit/cycle/frame 관계를 별도 검증한다.
14. **Computed track list** — authored 순서와 typed projection 순서를 정확히 비교한다.
15. **Item geometry** — item 및 payload의 top/height/frame을 row track과 비교한다.
16. **Text metrics** — line count와 glyph count를 matrix scene별로 검사한다.
17. **Overflow 경계** — border/root 바깥 visible overflow 장면을 검사하고 개수와 최대 초과를 기록한다.
18. **Stretch 순서** — content alignment가 sizing 이후 적용되며 auto 외 track은 고정되는지 확인한다.
19. **Trace 선택** — 가장 최신 row sizing pass를 선택하고 ambiguous/missing final stages에서 fail-closed한다.
20. **Trace serialization** — used row string과 final track base가 일치하지 않으면 보정하지 않는다.
21. **0% 경계** — 0 크기 fit-content track에 음수 또는 stale size를 주지 않는다.
22. **Over-constrained Grid** — free space가 음수여도 auto track shrink를 임의 추가하지 않는다.
23. **Numeric test compatibility** — lower-level legacy/compact payload profiles를 전체 child 기준으로 일관되게 적용한다.
24. **Mixed numeric profiles** — 한 grid 안에서 두 profile을 섞는 경우를 명시적으로 거부한다.
25. **Multiple Grid owners** — 새 normalizer가 둘 이상의 대상 Grid를 부분 처리하지 않는다.
26. **Revision atomicity** — stale revision이나 frame mismatch에서는 descendants 변경이 시작되지 않는다.
27. **Span regression isolation** — strict trace validator를 신규 auto-placement 전용으로 분리하고 이전 span 경로의 trace extraction을 유지한다.
28. **Span row-count rejection** — 기존 span의 position 4–6 거부 회귀를 별도 테스트 행으로 포함한다.
29. **Platform sentinel 정직성** — Android/iOS 8개씩은 기본 route의 draw/present·화면 smoke이며 compositor pixel parity로 표현하지 않는다.
30. **상태·완료 판정** — C11.4/C11 checkbox와 `78/214`는 유지하고, merge·CI·일반 CSS 지원을 선행 주장하지 않는다.

검토 결과: 알려진 span trace 경계 문제는 수정 후 1,920-scene 기존 RuntimeSession 비교로 재확인했다. 전체 검토의 실행 근거와 남은 범위는 동봉된 `README.md` 및 `rust-verification/` 로그에 연결한다.
