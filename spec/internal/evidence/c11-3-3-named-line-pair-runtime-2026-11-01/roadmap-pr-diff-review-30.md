# C11.3.3 양 끝 implicit named-line 공개 로드맵 diff 검토 30개

**검토 대상:** `spec/public/roadmap.html`의 PR #933 카드와 카드가 연결하는 계획·내부 계약·실행 근거·시뮬레이터 화면.

**기준 사실:** 구현 PR #933은 2026-10-31T17:35:07Z Rebase and merge됐고 merge SHA는 `ada12109a6b0394ed0a43565aa93721748955d49`다. 공식 완료 집계는 `72/213`이다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | 새 카드가 이전 no-area named-start 항목과 분리되지 않는다 | PASS — PR #927 card 뒤에 새 paragraph를 둔다. |
| 2 | 날짜가 Chrome 사전 비교 날짜로 잘못 표현된다 | PASS — 새 구현 기록 날짜와 구현 병합 시각을 구별한다. |
| 3 | 구현 PR 링크가 잘못된 번호를 가리킨다 | PASS — GitHub PR #933을 연결한다. |
| 4 | merge timestamp가 PR 작성 시각을 가리킨다 | PASS — 확인된 `mergedAt` `2026-10-31T17:35:07Z`를 적는다. |
| 5 | merge SHA가 PR head SHA와 혼동된다 | PASS — 실제 merge SHA `ada12109…`를 정확히 쓴다. |
| 6 | merge 방식이 프로젝트 정책과 다르게 표현된다 | PASS — Rebase and merge를 명시한다. |
| 7 | area-backed profile로 오해된다 | PASS — no-area opt-in Grid 한정이다. |
| 8 | layout-participating child 수가 빠진다 | PASS — direct item 하나로 제한한다. |
| 9 | intrinsic/auto-sized item까지 포함한다고 오해된다 | PASS — fixed-size 조건과 auto-size 거부를 적는다. |
| 10 | 양 endpoint 중 하나만 implicit이면 성공한다고 오해된다 | PASS — 양 끝 endpoint 모두 positive trailing implicit named line이어야 한다. |
| 11 | 반대 축의 placement 조건이 빠진다 | PASS — 반대 축 definite placement를 명시한다. |
| 12 | `grid-auto-*` 크기 문법을 무제한으로 확대한다 | PASS — `auto` 또는 fixed-px만 적는다. |
| 13 | equal endpoint의 end 제거 처리가 빠진다 | PASS — equal endpoint 제거를 기록한다. |
| 14 | 역순 endpoint의 교환 처리가 빠진다 | PASS — reversed endpoint 교환을 기록한다. |
| 15 | 다중 implicit track 분배 지원이 빠진다 | PASS — multi-track distribution을 설명한다. |
| 16 | Android AVD를 실기기로 부른다 | PASS — API 37 ARM64 AVD로 구체화한다. |
| 17 | Android PixelCopy를 hardware GPU 결과로 오해한다 | PASS — SwiftShader 사용을 밝힌다. |
| 18 | iOS Simulator를 실기기로 오해한다 | PASS — iPhone 17 Pro/iOS 26.2 Simulator로 표기한다. |
| 19 | Metal drawable readback을 compositor 이후로 오해한다 | PASS — compositor 제출 전이라고 설명한다. |
| 20 | root 바깥 item이 sample 대상에서 빠진 것으로 오해한다 | PASS — 실행 근거 링크가 두 축 overflow 표본을 보여주며 본문도 root 밖 표면으로 한정한다. |
| 21 | 앱 scene 행렬을 9 scene 전체 플랫폼 검증으로 과장한다 | PASS — 9-scene은 Chrome 기준이고 플랫폼 앱 sample은 Columns/Rows 두 scene이다. |
| 22 | per-platform pixel 표본 수가 부정확하다 | PASS — Android와 iOS 각각 2/2 exact로 적는다. |
| 23 | `grid-template-areas` 지원으로 오해된다 | PASS — template areas를 남은 범위에 명시한다. |
| 24 | 양 축 implicit, signed/leading line이 새 지원으로 보인다 | PASS — 모두 미완료 경계로 열거한다. |
| 25 | 추가 participating child 허용으로 읽힌다 | PASS — 추가 child를 거부 범위에 적는다. |
| 26 | generic CSS author admission을 구현했다고 오해된다 | PASS — 일반 CSS author admission 미완료를 적는다. |
| 27 | Spinon WPT 적합성으로 오해된다 | PASS — WPT와 전체 C11.3.3/C11을 미완료로 적는다. |
| 28 | 공식 완료 count가 증가한 것으로 읽힌다 | PASS — `72/213` 유지와 미완료를 같은 카드에 둔다. |
| 29 | PR check 부재를 CI 통과로 표현한다 | PASS — check rollup이 비었다고 기록한다. |
| 30 | GitHub merge를 Tailnet 동기화·화면 확인 완료로 오해한다 | PASS — 카드에는 게시 주장을 하지 않고 served hash와 Agent Browser를 별도 검증한다. |

이 검토는 구현/runtime 20개와 PR #933 실제 diff 30개 검토를 대체하지 않는다. 카드 링크의 HTTP 상태·실제 렌더링·모바일 넘침은 PR 병합 후 Tailnet preview를 갱신하고 Agent Browser로 확인한다.
