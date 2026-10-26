# PR #655 계획 diff 적대 검토 30개

**검토 대상:** [PR #655](https://github.com/ohah/spinon/pull/655)
**Base:** `main` / `77fb2144d72b5bab2f12e7e61c33be54aa0f31dd`
**Head:** `1cc5e5462b35c5115d1a183ada6487483f557fe0`
**기준:** `docs/project-rules.md`의 PR diff 적대 검증 규칙, CSS Grid Layout Module Level 1 2025 CRD §11.8, 해당 head의 전체 PR diff와 본문·라벨·GitHub 상태

각 항목은 서로 다른 실패 경로 또는 계약 경계를 확인했다. 최종 head에서 **PASS 30 / PARTIAL 0 / FAIL 0, 차단 0개**다. 초안 검토에서 발견한 계획 상태 문구와 `justify-content` 입력 표기는 각각 `12f31b1b`, `af61f257`, `1cc5e546`에서 수정됐으며 아래 상태 및 입력 관점에서 최종 head를 다시 확인했다.

## 30개 관점

1. **기준 브랜치 최신성 — PASS.** GitHub PR base는 `main`의 `77fb2144d72b5bab2f12e7e61c33be54aa0f31dd`이고, 로컬 `origin/main` 및 GitHub `main` ref와 일치한다.
2. **검토 대상 고정 — PASS.** PR의 현재 head, 체크아웃 `HEAD`, 이 보고서의 head SHA가 모두 `1cc5e5462b35c5115d1a183ada6487483f557fe0`이다.
3. **변경 경계 — PASS.** base 대비 6개 파일, 201줄 추가다. 변경 파일은 계획·검토 문서와 사전 비교 README/JSON뿐이며 Rust 제품 코드·테스트 코드·FFI·플랫폼 파일은 없다.
4. **PR 본문 범위 — PASS.** 본문은 `align-content: normal` 후속 단위 테스트 계획과 독립 기대값만 추가하며 구현 코드는 포함하지 않는다고 정확히 말한다.
5. **제목·커밋 제목 — PASS.** PR 제목과 4개 커밋 제목은 타입/범위 접두어 뒤에 한국어 설명을 두며 코드/속성 식별자만 원문으로 유지한다.
6. **라벨 분류 — PASS.** `planning` 작업 유형 라벨과 `area: layout`, `area: testing` 영역 라벨이 함께 지정돼 있다.
7. **CI 경계 — PASS.** `statusCheckRollup`은 비어 있다. PR 본문은 CI 통과를 주장하지 않으며, `mergeStateStatus: CLEAN`을 테스트 통과 증거로 취급하지 않는다.
8. **규범 문서 고정 — PASS.** 계획과 evidence는 CSS Grid 2025 CRD §11.8의 공식 W3C anchor를 지정한다: [§11.8 Stretch auto Tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch).
9. **normal 적용 조건 — PASS.** §11.8의 `normal` 또는 `stretch` 조건과 남은 양수 definite free space라는 조건을 계획에서 정확히 제한한다.
10. **대상 트랙 정의 — PASS.** 공간은 `auto` max track sizing function을 가진 행에만 분배한다고 적고, 고정 최대 함수 행은 제외한다.
11. **정렬 축 선택 — PASS.** `align-content: normal`을 행 축 content distribution으로 사용하고, 행 기대값을 `justify-content` 정렬로 설명하지 않는다.
12. **기존 projection 연결 — PASS.** 현재 `taffy_style.rs`의 `LayoutAlignContent::Normal → AlignContent::STRETCH` 매핑 및 Taffy §11.8 stage gate와 계획의 `applied:true` 기대가 일치한다.
13. **definite root 크기 — PASS.** root block size가 고정 `100px`으로 명시돼 indefinite free space 경로를 실수로 섞지 않는다.
14. **viewport·root sizing — PASS.** viewport도 `20×100px`이고 `RootSizingPolicy::Match`가 기록돼 있어 계획 입력과 현재 helper의 크기 조건이 맞는다.
15. **content-box 산정 — PASS.** root border/padding 0 조건이 문서와 입력 JSON에 있어 definite height를 content-box 기준으로 바로 쓸 수 있다.
16. **행 topology — PASS.** 명시 행은 `[auto, auto, 5px]` 세 개이고 각 direct child가 대응하는 행 하나에 놓이는 구성이 명확하다.
17. **자식 순서 — PASS.** 입력 JSON의 DOM 순서와 row index가 대응해 frame 기대값의 source order가 모호하지 않다.
18. **아이템 크기 조건 — PASS.** 자식 높이 `[10,20,5]px`와 너비 `20px`가 고정돼 auto 행 contribution 및 고정 행 control을 분리한다.
19. **§11.8 전 기초값 — PASS.** `[10,20,5]px` 기초값은 사전 비교에서 규범·고정 입력으로 독립 산출하고 Taffy 결과를 새 oracle로 삼지 않는다.
20. **gutter 개수 — PASS.** 세 행 사이 내부 row-gap 두 개만 각 `10px`이며 선행·후행 trace sentinel은 `0px`로 분리한다.
21. **free-space 산술 — PASS.** `100 - (10 + 20 + 5) - (10 + 10) = 45px`로 gaps를 한 번씩 포함한다.
22. **eligible 집합 — PASS.** auto 최대 함수 행 index `[0,1]`만 대상이고 fixed track index `2`는 제외한다.
23. **균등 분배량 — PASS.** `45/2 = 22.5px` 증가량을 두 eligible 행 각각에 적용한다.
24. **최종 크기 불변식 — PASS.** 최종 `[32.5,42.5,5]px`와 20px의 gutter 합은 100px이며 fixed 세 번째 행은 유지된다.
25. **stage trace 활성화 — PASS.** `GridPlacementMode::TrackSizing`을 입력에 명시해 stage snapshot이 생성되는 테스트 경로를 계획한다.
26. **stage 순서·중복 — PASS.** 각 stage를 정확히 한 번 찾고 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서를 확인하도록 계획한다.
27. **pre/post snapshot 구분 — PASS.** 앞의 세 stage는 `[10,20,5]px`, 마지막 stage는 `[32.5,42.5,5]px`로 각각 검사해 결과를 섞지 않는다.
28. **raw gutter trace 해석 — PASS.** CSS track 집계에서 gutter를 제외하면서 두 내부 gutter와 양 끝 collapsed sentinel을 각각 검증하도록 기존 helper 사용 경계를 적었다.
29. **frame·대조군·수치 비교 — PASS.** root/자식 y 위치는 최종 row starts에서 유도되고, 기존 `stretch`와 `start`는 독립 기대값의 positive/negative control이다. 허용 오차 `1e-4 CSS px` 및 finite 값 조건을 명시한다.
30. **사전 비교·상태 동기화 — PASS.** README, input JSON, 계획은 기존 helper의 `LayoutJustifyContent::FlexStart`와 일치하도록 `justify-content:flex-start`를 기록한다. 계획 자체 검토 `PASS 20/20`, PR diff 검토 대기, 구현 전 상태, C11.4 미완료 및 `72/213` 유지가 세 문서와 PR 본문에서 일치한다.

## 최종 판정

**PASS 30 / PARTIAL 0 / FAIL 0. 차단 항목은 없다.** 최초 검토에서 확인된 두 문서 불일치는 최종 head에서 수정 및 재확인했다. PR 본문이 제시한 focused test, fmt, Clippy, diff-check은 향후 구현 검증 계획으로만 기록돼 있으며, 이번 리뷰에서는 테스트·formatter·Clippy·시뮬레이터·브라우저를 실행하지 않았다. `git diff --check`는 오류 없이 끝났다. GitHub status rollup은 여전히 비어 있으므로 CI pass 증거는 없다.
