# C11.1 implicit-column 상태 대장·공개 로드맵 변경 검토 30개

검토 대상은 `spec/STATUS.md`의 PR #1059 병합 기록 한 건과 `spec/public/roadmap.html`의 이전 계획 문구 수정 및 PR #1056·#1059 카드 추가다. 제품 구현 검토와 분리해 상태, 이력, 범위, 링크, 공개 문구를 대조했다.

| # | 독립 실패 관점 | 판정과 대조 |
|---:|---|---|
| 1 | 병합되지 않은 PR을 완료 기록으로 올리는가 | 통과. PR #1059는 GitHub에서 Rebase and merge된 상태다. |
| 2 | PR 번호가 다른 변경을 가리키는가 | 통과. #1059는 fixed-px implicit columns 구현 PR이고 #1056은 explicit-cell sparse placement다. |
| 3 | merge timestamp를 잘못 기록하는가 | 통과. 기록의 UTC 시각 `2026-11-02T13:03:56Z`를 PR 병합 metadata와 대조했다. |
| 4 | PR head SHA를 merge SHA로 오인하는가 | 통과. `16c62916b4a72de6a44a9cbd3bec021d0454541e`가 main의 병합 결과다. |
| 5 | #1056과 #1059의 순서가 뒤바뀌는가 | 통과. #1056 sparse placement 다음 #1059 implicit columns 순이다. |
| 6 | 이전 계획 카드가 아직 구현 전이라고 잘못 말하는가 | 통과. #1054 계획 뒤에 병합된 제한 구현 #1056·#1059를 반영했다. |
| 7 | #1056의 explicit-cell 경계를 implicit sizing까지 확대하는가 | 통과. 해당 카드는 overflow와 implicit sizing 거부를 유지한다. |
| 8 | #1059 범위를 일반 CSS Grid 지원으로 확대하는가 | 통과. parsed Root HTML 기본 route의 제한 slice로 적었다. |
| 9 | `grid-auto-columns`의 허용 값을 확대하는가 | 통과. finite non-negative fixed-px scalar/list로 한정했다. |
| 10 | list 순환 순서를 잘못 설명하는가 | 통과. 필요한 implicit columns에 authored 순서로 순환한다고 썼다. |
| 11 | 10,000-track cap 또는 오류 복구를 누락해 무제한 확장으로 읽히는가 | 통과. 상태 대장 기록에 cap과 atomic reservation failure가 있다. |
| 12 | hidden-item 행위를 보편 CSS 규칙으로 오해하게 하는가 | 통과. 구현의 hidden-item 처리만을 기록하고 일반 적합성을 주장하지 않는다. |
| 13 | 12-item fixture를 전체 auto-placement 적합성으로 확대하는가 | 통과. 2×2 explicit grid와 12 items라는 비교 범위를 명시한다. |
| 14 | Chrome oracle 수치를 Spinon 전체 적합성으로 읽는가 | 통과. root/semantic frame의 0 CSS px 비교를 해당 fixture 결과로 한정한다. |
| 15 | Simulator를 실기기로 오인하는가 | 통과. Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator라고 구체화했다. |
| 16 | 화면 캡처를 표면 readback이나 픽셀 동등성으로 과장하는가 | 통과. PixelCopy/Metal readback이 아님을 명시한다. |
| 17 | focused Rust 검증을 전체 suite 성공으로 오인하는가 | 통과. focused tests·fmt·scoped Clippy와 전체 suite를 분리했다. |
| 18 | 전체 style-to-layout 실패를 감추는가 | 통과. 308/309와 C07 computed-serialization 차이를 기록했다. |
| 19 | 변경 밖 C07 실패 원인을 C11 변경의 회귀로 단정하는가 | 통과. 변경 밖 mismatch로 한정하고 값 두 개를 병기했다. |
| 20 | 비어 있는 GitHub check rollup을 CI 성공으로 표현하는가 | 통과. check rollup이 비어 있다고 명시한다. |
| 21 | C11.1을 완료 체크로 표시하는가 | 통과. 상태 대장의 상위 체크박스는 변경하지 않았다. |
| 22 | C11 상위 항목을 완료로 표시하는가 | 통과. C11 체크박스는 미완료로 유지한다. |
| 23 | 공식 완료 집계가 늘어난 것처럼 보이는가 | 통과. `72/213`을 두 문서 모두 유지한다. |
| 24 | 다른 C11 하위 상태를 함께 바꾸는가 | 통과. 이번 diff는 C11.1 기록과 해당 공개 카드에만 한정된다. |
| 25 | 문서 숫자 버전을 올리는가 | 통과. 내부 계약 버전은 변경하지 않았다. |
| 26 | 상태 대장의 전용 계획 링크가 이전 slice를 가리키는가 | 통과. implicit-column 계획 파일 경로를 사용한다. |
| 27 | 내부 계약 링크가 공개 API인 것처럼 표시되는가 | 통과. `내부 계약 0059`라고 분명히 적었다. |
| 28 | 공개 roadmap의 plan/evidence URL이 저장소 경로 규칙과 어긋나는가 | 통과. 기존 `/spinon/plan/...` 및 `/spinon/internal/...` 라우트 패턴을 재사용한다. |
| 29 | HTML의 기존 의미 구조나 진행 보드 상태 계산을 바꾸는가 | 통과. 기존 `p.kanban-note.hero-progress` 카드 패턴만 사용하고 checkbox·counter·보드 계산 코드는 수정하지 않았다. |
| 30 | Tailnet 게시 결과를 소스 diff만으로 확인했다고 주장하는가 | 통과. 이 검토는 소스 변경만 다루며 실제 preview 동기화·브라우저 확인은 별도 단계로 남긴다. |

## 판정

30개의 서로 다른 관점에서 PR metadata, C11 미완료 상태, 공식 집계, 구현 한계, 검증 실패와 문서 링크를 대조했다. Tailnet preview의 실제 게시 상태는 이 검토 범위에 포함하지 않는다.

전체 `mise exec -- bun run docs:build`는 저장소 전체 dead-link 검사에서 실패했다. 오류는 다수의 기존 상태·보관 근거 문서에 걸쳐 있었고, 새 로드맵 HTML과 이 검토 파일은 오류 원본으로 보고되지 않았다. `STATUS.md` 자체는 기존 dead link들로 계속 보고됐지만 이번 C11.1 항목의 새 링크는 오류 목록에 없음을 확인했다. 이 문서 빌드 결과는 성공으로 취급하지 않는다.
