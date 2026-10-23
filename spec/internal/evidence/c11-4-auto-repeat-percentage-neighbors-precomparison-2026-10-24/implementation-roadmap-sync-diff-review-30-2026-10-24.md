# C11.4 percentage neighbor 로드맵 동기화 PR diff 검토 30개

2026-10-24 공식 상태·공개 로드맵·로컬 검증 근거의 변경만 따로 대조했다. 구현 PR이나 C11.4 완료 판정으로 확대하지 않는지, 코드 후보·독립 기대값·테스트 결과·미검증 경계를 각각 확인했다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 완료 수가 status checkbox 수와 어긋남 | Agent Browser 렌더링은 `213개 표시 · 공식 완료 72/213`이다. |
| 2 | 신규 작업 노트를 완료 checkbox로 처리 | `STATUS.md`의 C11.4 항목은 계속 미완료다. |
| 3 | C11.4 전체를 slice 완료처럼 표현 | 노트에 C11.4 미완료를 명시한다. |
| 4 | 미커밋 코드가 `main`에 들어간 듯 표현 | 상태와 로드맵 모두 후보를 미커밋 checkout 변경으로 한정한다. |
| 5 | 계획 PR과 구현 PR을 혼동 | 계획 PR #577 병합과 코드 후보 상태를 별도 문장으로 둔다. |
| 6 | Chrome 196px을 규범값으로 사용 | CSS Grid 2025 CRD gutter collapse 계산 200px을 matrix 기준으로 둔다. |
| 7 | Chrome 관측을 삭제하거나 규범으로 덮음 | Chrome 156 CSSOM 196px을 별도 관측으로 보존한다. |
| 8 | Taffy 결과만으로 규범값을 추정 | 독립 CRD 계산·Taffy 결과·Chrome 관측의 역할이 분리돼 있다. |
| 9 | end-alignment 좌표를 track 크기로 오해 | 규범 236px과 Chrome 232px을 별도 diagnostic으로 기록한다. |
| 10 | suffix x/y=238px을 gutter 판별로 확대 | 두 계산이 같은 좌표를 내므로 비판별 probe라고 유지한다. |
| 11 | row scene을 column과 동일하게 가정 | 고정 matrix는 columns와 rows 축 사례를 모두 포함한다. |
| 12 | 13개 scene 수를 다른 matrix와 혼동 | 명령에 고정 13-scene matrix라고 명시하고 증거 테스트도 13 case다. |
| 13 | 빈 auto-fit의 정상 조기 종료를 실패로 표시 | `FixedTrackEarlyReturn` trace를 비교하는 이유를 설명한다. |
| 14 | flex stage가 없는 case에 가짜 확장 결과 사용 | 빈 장면은 early-return track snapshot을 읽고 나머지는 실제 flex stage를 읽는다. |
| 15 | negative percentage 허용 | layout rejection test가 두 축의 negative percentage를 거부한다. |
| 16 | NaN·infinity 입력 허용 | 두 축의 non-finite 입력 rejection을 검증 근거에 포함한다. |
| 17 | 유한 percentage 곱셈 overflow 누락 | `f32::MAX` 이웃에 대한 basis 곱셈 overflow 거부를 별도로 적었다. |
| 18 | indefinite basis에 percentage 통과 | 각 축의 indefinite test가 fail-closed임을 기록한다. |
| 19 | zero-size 축과 `0%` 이웃 혼동 | gate는 zero percentage 이웃을 허용할 수 있지만 basis 축은 definite positive여야 한다. |
| 20 | `calc()` 이웃까지 지원으로 확대 | cascade rejection 회귀는 유효한 `calc()` neighbor를 계속 거부한다. |
| 21 | `1fr` invalid grammar를 runtime error 계약으로 둠 | parser 뒤 computed grid가 `columns=[]`, `columns_auto_repeat=None`임을 확인했고 rejection 목록에서 제외했다. |
| 22 | CSS parser fallback을 기능 지원으로 오해 | 잘못된 전체 declaration이 기본값으로 가는 결과만 기록하고 지원으로 주장하지 않는다. |
| 23 | `spinon-layout` 결과를 layout 전체 검증으로 일반화 | 두 focused test만 통과한 것으로 한정한다. |
| 24 | `spinon-style` 결과를 전체 style suite로 일반화 | `auto_repeat` 필터 16개만 통과했다고 정확히 적는다. |
| 25 | 형식 검사를 전체 Clippy로 표현 | `cargo fmt --all -- --check`만 통과했다고 적는다. |
| 26 | 전체 workspace 테스트가 통과한 듯 표현 | 전체 workspace 미실행 경계를 명시한다. |
| 27 | Android/iOS 런타임 완료를 암시 | 두 플랫폼 앱 runtime 미실행을 status와 roadmap에 남겼다. |
| 28 | FFI/artifact·화면·pixel 검증을 암시 | local evidence에 이번 검증에서 실행하지 않은 항목을 열거한다. |
| 29 | Tailnet 확인을 production 배포로 해석 | selected-root·개발용 Tailnet preview로만 설명하며 Pages 배포는 주장하지 않는다. |
| 30 | 링크·렌더링·C13 활성 상태 누락 | Agent Browser에서 evidence route, 72/213 counter, C13 미완료·비활성 note를 확인했다. |

이 변경은 roadmap/official status 동기화다. 기능 구현 검토 20개, runtime 확인, 전체 workspace 검사 또는 C11.4 완료 판정을 대체하지 않는다.
