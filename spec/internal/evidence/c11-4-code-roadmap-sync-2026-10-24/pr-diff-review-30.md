# C11.4 코드·로드맵 동기화 PR diff 적대 검토 30개

**검토 대상:** 상위 C11.4 plan의 PR #580 이후 neighbor 지원 설명, 새 calc-neighbor 계획과 Chrome 사전 비교, `spec/STATUS.md`, 공개 roadmap 및 코드·로드맵 재대조 근거. 제품 Rust 구현은 포함하지 않는다. Tailnet live preview는 PR 병합 뒤 다시 동기화한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 계획 PR이 calc-neighbor runtime 구현을 완료로 기록한다. | 새 plan과 공개 note가 runtime 미구현이라고 명시한다. 통과. |
| 2 | C11.4 상위 checkbox가 제한 slice 때문에 완료로 바뀐다. | `spec/STATUS.md`의 `[ ] C11.4`를 유지한다. 통과. |
| 3 | 공식 완료 수가 세부 plan 때문에 증가한다. | status, plan, public roadmap 모두 `72/213`을 유지한다. 통과. |
| 4 | PR #580 병합 뒤 fixed-only neighbor로 문서를 축소한다. | 상위 plan은 finite nonnegative px와 percentage 지원, definite positive basis 및 overflow gate를 기록한다. 통과. |
| 5 | plan이 실제 Stylo admission과 다른 지원 shape를 설명한다. | `is_auto_repeat_neighbor_track` source와 plan을 대조했고 현재 `Calc`는 거부된다. 통과. |
| 6 | plan이 layout gate의 percentage 조건을 빠뜨린다. | `auto_repeat_neighbor_tracks_are_supported`와 positive definite content-axis 요구를 대조했다. 통과. |
| 7 | repeat fragment 내부 calc 지원을 바깥 neighbor 지원과 혼동한다. | 새 plan에서 두 typed projection 경계를 별도 설명한다. 통과. |
| 8 | 새 scope가 임의 CSS math evaluator로 커진다. | exact two-term typed `ComputedCssMath::Sum` whitelist로 제한한다. 통과. |
| 9 | sum 항 순서가 한 방향만 허용된다고 오해한다. | 계획과 fixture가 length/percentage 두 순서를 모두 고정한다. 통과. |
| 10 | signed length 항을 지원 목록에서 제외하거나 무제한으로 허용한다. | finite signed length만 입력으로 두고 final used value range clamp를 별도 규정한다. 통과. |
| 11 | negative percentage term이 accidentally 지원된다. | CSS-valid control에 남기고 현재 AST whitelist 밖의 fail-closed 입력으로 분리한다. 통과. |
| 12 | 세 항 이상 또는 nested math가 두 항 sum으로 잘못 축약된다. | 3-term control과 일반 math reject 경계를 명시한다. 통과. |
| 13 | indefinite/cyclic basis가 percentage 0 또는 임의 크기로 처리된다. | max-content control을 두고 unresolved basis rejection을 계획한다. 통과. |
| 14 | calc 최종 음수값을 CSS-invalid 선언으로 판정한다. | CSS Values §10.12에 따라 지원 shape의 최종 값을 0px으로 clamp한다. 통과. |
| 15 | basis가 border-box나 padding 포함 크기로 계산된다. | 독립 scene은 136px border-box에서 16px padding과 4px border를 빼 116px content basis를 고정한다. 통과. |
| 16 | finite 값의 합·f32 변환이 overflow돼 neighbor를 왜곡한다. | f64 계산, 표현 불가 및 aggregate overflow 시 fail-closed를 계획한다. 통과. |
| 17 | 실패 입력 뒤 partial layout 결과가 반환될 수 있다. | plan은 오류에서 partial `LayoutOutput`을 만들지 않도록 acceptance를 명시한다. 통과. |
| 18 | minimum auto-repeat count 규칙이 바뀐다. | overflow scene은 최소 1회 규칙을 고정하고 plan은 count용 neighbor만 다룬다. 통과. |
| 19 | auto-fit collapse 또는 실제 track sizing 순서가 새 neighbor 계산에 섞인다. | fit-first/fit-empty controls와 분리 단계 acceptance를 둔다. 통과. |
| 20 | 독립 normative expectation을 Chrome 출력에서 역산한다. | `spec-derived-expectations.json`과 계산 설명을 Chrome observation과 별도 보존한다. 통과. |
| 21 | CSS-valid negative controls를 파싱 오류로 오인한다. | negative percentage, 3-term sum, indefinite basis는 유효 선언과 profile 거부를 구분한다. 통과. |
| 22 | browser observation을 규범 기대값으로 승격한다. | 문서가 CSSOM 관측과 normative expected values를 분리한다. 통과. |
| 23 | browser 관측을 재현할 환경 정보가 없다. | Chrome 156.0.8078.48, Agent Browser 0.31.1, macOS arm64, viewport 1280×633, DPR 1을 기록한다. 통과. |
| 24 | fixture와 JSON의 scene 이름 또는 count가 어긋난다. | fixture·observation·expectation은 같은 14 ID를 사용하고 validator가 14 scene을 검사한다. 통과. |
| 25 | float 차이로 올바른 Chrome 결과를 임의 실패 처리한다. | 비교 허용치를 1/64 CSS px로 기록하고 summary에서 14 scene validator 통과 범위를 제한한다. 통과. |
| 26 | overflow item이 grid border 안에 있어도 바깥 paint를 주장한다. | 고정 DOM에서 root right 248px, item x=252px로 4px 바깥임을 근거에 적고 runtime 표본은 후속 acceptance로 둔다. 통과. |
| 27 | screenshot만으로 Android/iOS paint가 이미 확인됐다고 과장한다. | README와 status는 simulator/runtime 미실행으로 구분하며 simulator는 향후 계획으로만 적는다. 통과. |
| 28 | fixture 전용 코드가 기본 제품 artifact에 들어간다고 현행 사실처럼 쓴다. | 기본/opt-in artifact 검사는 후속 구현 acceptance로 표기하고 이번 PR의 source 변경에서 제외한다. 통과. |
| 29 | C13 미완료를 C13 활성 진행 상태로 노출한다. | 상위 checkbox는 미완료 유지, C13 노트는 “백로그 · 미완료·비활성”, lane source는 C11.4만 active로 명시한다. 통과. |
| 30 | 로드맵이 C13.1–3의 제한 slice를 전체 scroll 구현이나 완료로 읽게 한다. | public note가 DOM scroll event/CSSOM, shadow overflow, scrollbar/gutter, scroll snap, logical axis의 남은 경계를 열거한다. 통과. |

## 판정

30개 관점에서 계획 범위·상태 집계·코드 설명·oracle 경계를 확인했다. 차단 결함은 발견하지 않았다. Tailnet preview 갱신과 그 화면 확인은 이 저장소 PR 병합 뒤 별도 단계로 수행한다.
