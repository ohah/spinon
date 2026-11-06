# C11.4 row-gap PR 변경 실패 관점 검토 30개

**검토 대상:** PR [#1240](https://github.com/ohah/spinon/pull/1240), base `main` at `2a949afd2a048e5a3d3199987053f0e1d8f927cc`, 검토 시 head `d74567c7c` (`docs(c11.4): row-gap 구현 후보 상태 기록`). 기능 코드·RuntimeSession tests·상태/계약/계획 문서·simulator evidence 변경을 서로 다른 실패 경계로 확인했다. 이 파일은 검토 결과 산출물이다.

## 검토 항목

1. **범위 누수 — placement mode:** 새 row-gap admission은 `RootHtmlAutoRowStretchV1`에만 분기한다. 다른 intrinsic/mixed placement profile은 기존 generic validator를 계속 사용한다. **통과.**
2. **CSS 값 종류 — px만 허용:** Style projection은 typed `ComputedCssSpacingValue::LengthPx`만 검사하며 percentage·normal·auto·unsupported는 거부하는 단위 테스트가 있다. **통과.**
3. **NaN/무한대 입력:** `is_finite()`가 양의 경계 비교보다 먼저 필요조건으로 적용되고 NaN·양/음 infinity negative controls가 존재한다. **통과.**
4. **음수 입력:** projection allowlist와 layout contract 모두 0 미만을 거부한다. **통과.**
5. **상한 경계:** 7.5px은 inclusive로 받고 7.5001px은 projection/layout/runtime에서 거부한다. **통과.**
6. **하한 경계:** 0px baseline을 허용하고 기존 zero-gap alignment 동작을 40-scene 행렬에 남긴다. **통과.**
7. **Column gap 우회:** computed column-gap은 계속 exact zero px만 허용하며 positive non-zero column gap controls가 오류를 확인한다. **통과.**
8. **다른 property의 오진단:** style gate는 `row-gap`과 `column-gap`을 분리해 검사하고 layout 오류도 거부 property 이름을 유지한다. **통과.**
9. **범위 밖 legacy 경로:** legacy `RootHtmlIntrinsicTrackSizingV1`의 auto-row가 허용되지 않는 기존 회귀 조건은 변경되지 않았다. **통과.**
10. **기존 helper 의미 변경:** 기존 `root_html_intrinsic_gap_is_supported` 구현은 보존하고 auto-row profile에만 별도 helper를 둔다. **통과.**
11. **이중 gutter 적용:** 새 코드는 validation만 넓히며 gutter를 수동 frame 보정으로 더하지 않는다. Taffy 계산 결과만 사용한다. **통과.**
12. **Gutter 개수 산식:** 기대 free space는 40−30−2g로 계산해 두 track 사이 gutter만 차감하며 세 번 차감하지 않는다. **통과.**
13. **Explicit row stretch 혼입:** 예상 row sizes에서 explicit row는 항상 10px이고 남는 양수 공간은 implicit auto rows 둘에만 절반씩 배분한다. **통과.**
14. **Normal/stretch 조건:** positive free space에서만 두 implicit rows가 늘고 zero/negative space에서는 base가 유지되는 행렬 기대값을 확인한다. **통과.**
15. **Positive space positional alignment:** start/end/flex-start/flex-end/center frame offset을 네 gap 값으로 대조한다. **통과.**
16. **Positive space distributed alignment:** space-between/around/evenly에서 distribution space가 gutters에 더해지는 각 item 위치를 비교한다. **통과.**
17. **Zero free-space 경계:** row-gap 5px에서 전체 base rows와 두 gutter가 정확히 40px을 차지해 alignment 간 시작점이 일치하는 expected scene이 있다. **통과.**
18. **Negative free-space positional overflow:** 7.5px `end`와 `center`에서 첫 item y가 각각 -5px, -2.5px이며 이를 clamp하지 않는 Rust geometry 대조가 있다. **통과.**
19. **Negative distributed fallback:** space-between/around/evenly는 negative space에서 `[0,17.5,35]px` start fallback을 적용한다. **통과.**
20. **정렬값 누락:** `normal`, `stretch`, start/flex-start, end/flex-end, center, 세 distributed 값 전체 10종을 4 gaps × 10 scenes로 돈다. **통과.**
21. **Track trace의 독립 확인:** frame 외에 `StretchAutoTracks` trace에서 세 non-gutter row base를 각각 예상값과 비교한다. **통과.**
22. **Frame 축 검증:** 세 item의 x·width·height를 고정값으로 대조하고 y는 1/32 CSS px 이내로 비교한다. **통과.**
23. **Runtime oracle provenance:** RuntimeSession expected values는 저장소에서 추적되는 Chrome/독립 계산 precomparison JSON을 읽는다. 로컬 전용 파일 의존이 없다. **통과.**
24. **직접 text geometry:** 두 text item의 painted boxes와 glyph origins를 기대 row 시작점에 비교한다. **통과.**
25. **Revision 정합성:** cascade/layout completed key를 일치시키고 각 text source의 style revision도 completed revision에 묶는다. **통과.**
26. **Computed percentage rejection:** runtime positive-percent row gap과 typed layout percentage를 각각 거부 대조한다. **통과.**
27. **Computed math rejection:** `calc(5px)` declaration도 성공 장면으로 흘러가지 않고 거부되어 admission이 typed fixed px에 한정된다. **통과.**
28. **실패 원자성과 복구:** 거부 RuntimeSession 결과는 `completed=None`이고 별도 recovery test가 같은 세션의 다음 유효 alignment 계산을 확인한다. **통과.**
29. **제품 범위·완료 수 과장:** PR body, plan, STATUS, contract 모두 제한 후보·미완료를 명시하고 C11.4/C11 및 공식 `78/214`를 유지한다. **통과.**
30. **플랫폼·증거 한계:** AVD/iOS는 default Root HTML route와 복원 기록을 남기며 simulator screenshots를 visual smoke로만 분류한다. PixelCopy/Metal readback, 실기기, hardware GPU, WPT를 주장하지 않는다. PR 본문 Android/iOS images는 Agent Browser에서 각각 `1080×2400`, `1206×2622`로 로드됐고 browser process는 닫혔다. **통과.**

## 결과

위 30개 경계에서 blocking 결함은 찾지 못했다. 이 검토는 PR review 결과이며 CI 통과나 merge를 대신하지 않는다. PR 당시 초안이라 GitHub checks는 아직 시작되지 않았다. 최신 checks와 PR 대화 검토 후 병합 가능 여부를 별도 확인한다.
