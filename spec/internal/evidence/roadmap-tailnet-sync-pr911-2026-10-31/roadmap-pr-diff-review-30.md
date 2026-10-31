# PR #911 병합 상태 공개 로드맵 변경 검토 30개

**검토 대상:** `spec/public/roadmap.html`에 추가할 C11.3.3 PR #911 병합 상태 카드와 연결 링크.

**기준 사실:** PR #911은 2026-10-31T08:06:44Z에 Rebase and merge됐고 merge SHA는 `eeba00fa9faf05ffcc8160f8adbb220fe90204db`다. 제한 runtime slice의 official completion count는 `72/213` 그대로다.

각 관점은 서로 다른 과장·오표기·링크 결함을 찾는 검토다. 아래 상태는 변경 초안과 병합된 PR #911의 계획·runtime 근거를 대조한 결과다.

| # | 실패 관점 | 대조 결과 |
| ---: | --- | --- |
| 1 | 제한 구현을 C11 전체 완료로 표시 | 통과: C11은 미완료로 명시한다. |
| 2 | C11.3.3 candidate checkbox를 완료로 간주 | 통과: 카드가 제한 후속 slice로 한정한다. |
| 3 | C11.4 상태를 PR #911로 완료 처리 | 통과: C11.4 미완료를 유지한다. |
| 4 | 공식 완료 수를 증가 | 통과: `72/213`을 그대로 쓴다. |
| 5 | 날짜만 표시하고 병합 사실을 생략 | 통과: PR #911 병합 상태와 UTC 시각을 연결한다. |
| 6 | 원래 head SHA를 merge SHA로 오기 | 통과: `eeba00fa9faf05ffcc8160f8adbb220fe90204db`와 일치한다. |
| 7 | area 없는 template에도 확장된 것처럼 서술 | 통과: area-backed 경로로 범위를 제한한다. |
| 8 | 새 track sizing 알고리즘을 추가한 것처럼 표현 | 통과: 기존 C11.4 typed subset 재사용으로 설명한다. |
| 9 | area가 만든 부족 track과 authored track을 혼동 | 통과: 제한 profile 설명과 계획 문서가 일치한다. |
| 10 | trailing named-line phase를 전체 배치 지원으로 과장 | 통과: positive trailing named-line slice라고 한정한다. |
| 11 | named-line phase Rust 단위 검증을 앱 행렬로 오기 | 통과: Rust layout test임을 별도로 밝힌다. |
| 12 | `fit-content()` cap을 검증했다고 암시 | 통과: cap은 검증되지 않은 경계로 적는다. |
| 13 | 전체 WPT 적합성을 주장 | 통과: 전체 WPT는 범위 밖이다. |
| 14 | 일반 authored CSS admission을 주장 | 통과: 일반 CSS admission은 범위 밖이다. |
| 15 | Android 결과를 실기기 결과로 오기 | 통과: Android API 37 ARM64 AVD를 쓴다. |
| 16 | Android renderer 한계를 숨김 | 통과: SwiftShader software renderer를 명시한다. |
| 17 | iOS 시뮬레이터를 실기기 결과로 오기 | 통과: iPhone 17 Pro/iOS 26.2 Simulator로 쓴다. |
| 18 | iOS drawable을 compositor 이후 화면으로 오기 | 통과: compositor 제출 전 한계를 적는다. |
| 19 | simulator matrix scene 수를 잘못 표기 | 통과: 39 scenes·126 nodes를 runtime README와 대조한다. |
| 20 | matrix success/fallback/rejection을 오기 | 통과: 34 success·5 cascade fallback·0 expected rejection과 일치한다. |
| 21 | geometry 오차를 누락하거나 확대 | 통과: 최대 0 CSS px는 고정 matrix 범위로만 표기한다. |
| 22 | Android PixelCopy 적용 장면 수를 전체 scene 수로 오기 | 통과: typed area surface sample 7/7로 범위를 한정한다. |
| 23 | iOS Metal 표본 수 또는 시점 오기 | 통과: typed area 7/7, compositor 제출 전임을 밝힌다. |
| 24 | PR checks를 CI 성공으로 오기 | 통과: check rollup이 없고 required checks가 설정되지 않았다고 적는다. |
| 25 | 계획 링크가 존재하지 않는 route를 가리킴 | 통과: merged `plan/c11-3-3-template-area-auto-track-sizing.md`에 대응한다. |
| 26 | 내부 계약 route를 공개 API 명세로 오인 | 통과: 내부 계약 0059 링크로만 소개한다. |
| 27 | 실행 근거 링크가 다른 slice를 가리킴 | 통과: PR #911 전용 runtime README 경로를 사용한다. |
| 28 | Android/iOS 이미지 링크가 다른 run의 screenshot을 가리킴 | 통과: PR #911 본문의 run ID와 screenshot 파일명에 대응한다. |
| 29 | status route를 빠뜨려 공식 수 확인이 어렵게 됨 | 통과: 공식 상태 anchor를 카드에 둔다. |
| 30 | 새 긴 카드가 모바일에서 가로 overflow를 만들거나 HTML 구조를 깨뜨림 | 통과: 기존 반응형 문단·링크 패턴을 유지하고 본문에 inline block 구조를 추가하지 않는다. 렌더 결과는 Tailnet Agent Browser에서 확인한다. |

**판정:** 30개 독립 관점에서 상태·범위·숫자·이미지 출처를 대조했다. 실제 Tailnet 렌더 및 screenshot 로드는 반영 후 별도로 확인한다.
