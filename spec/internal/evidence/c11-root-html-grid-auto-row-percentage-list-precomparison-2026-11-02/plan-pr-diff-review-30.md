# C11.1 Root HTML percentage implicit-row 계획 PR 변경 diff 적대 검토 30개

계획 PR의 실제 변경 파일 8개를 병합 전 30개의 서로 다른 실패 관점으로 대조했다. 검토 범위는 계획·precomparison fixture·기대값·관찰값·화면 캡처이며 구현 코드는 포함하지 않는다.

| # | 실패 가설 | 판정 |
|---:|---|---|
| 1 | 계획 전용 변경이 실제 runtime 지원 구현으로 오해된다. | 기각. 계획과 계획 PR diff 검토 단계이며 구현 전이라고 표시했다. |
| 2 | C11.1/C11 또는 공식 완료 수가 선행 변경 없이 완료된다. | 기각. 공식 상태와 `72/213`을 유지한다고 썼다. |
| 3 | `plan/c11-css-grid.md`가 기존 구현 결과를 덮어쓴다. | 기각. #1042 이력 뒤에 다음 미구현 계획을 덧붙였다. |
| 4 | 신규 계획 문서의 relative link가 경로를 잘못 가리킨다. | 기각. 부모 계획·내부 계약·evidence·fixture 경로를 확인했다. |
| 5 | 단일 fixed-px scalar 동작까지 목록 범위로 바뀐다. | 기각. 기존 scalar 표현과 계산을 보존한다. |
| 6 | 지원 목록이 임의의 CSS track grammar로 읽힌다. | 기각. direct finite non-negative px/percentage만 받는다. |
| 7 | 단일 percentage value가 TrackList representation에서 빠진다. | 기각. definite-zero case가 `50%` 단일 목록을 포함한다. |
| 8 | percentage basis가 viewport나 item size로 계산된다. | 기각. definite grid inner block size로 제한하고 숫자식을 고정했다. |
| 9 | percentage basis에서 row-gap을 잘못 차감한다. | 기각. `25% × 120px = 30px`이며 gap을 분리한 mixed case가 있다. |
| 10 | implicit 목록이 explicit row부터 반복되거나 순서가 바뀐다. | 기각. 네 implicit row에서 첫 entry부터 순환하는 cycle case와 used tracks를 고정했다. |
| 11 | `grid-template-rows`도 auto row list로 바뀐다. | 기각. explicit row는 단일 20px/0px로 고정해 구별했다. |
| 12 | 100% 초과 entry가 clamp되거나 거부된다고 읽힌다. | 기각. `120% → 120px`와 overflow geometry를 명시했다. |
| 13 | 0% entry가 auto/min-content로 커진다. | 기각. 0px resolved/used row를 포함한다. |
| 14 | definite zero height가 indefinite height와 합쳐진다. | 기각. 별도 `50%` 단일-value zero-basis scene을 관찰했다. |
| 15 | indefinite/cyclic percentage가 이번 scope에 몰래 포함된다. | 기각. definite basis만 admission하고 cyclic sizing을 범위 밖으로 적었다. |
| 16 | percentage gap도 implicit row list 지원에 딸려 들어온다. | 수정 후 기각. 계획 diff에서 모호한 `fixed-px/percentage gap` 문구를 발견해 fixed-px gap만 유지하고 percentage gap은 거부하도록 고쳤다. |
| 17 | `minmax()`, `calc()`, intrinsic 또는 `fr` implicit entries가 허용된다. | 기각. 목록 admission과 제외 범위에서 direct px/%로 제한했다. |
| 18 | named area와 implicit percentage row의 조합이 허용된다. | 기각. 기존 Root HTML area+implicit restriction을 보존한다. |
| 19 | CSS-invalid negative declaration과 valid-but-unsupported computed value가 혼동된다. | 기각. 두 cascade/rejection 경계를 별도 검사로 계획했다. |
| 20 | 각 percentage나 총 track/gap extent의 f32 overflow가 놓친다. | 기각. resolved value, aggregate extent와 10,000-track cap을 layout 전에 검증하도록 했다. |
| 21 | failure 또는 allocation 실패에서 stale/partial frame이 반환된다. | 기각. no-partial-output과 같은 worker recovery를 acceptance criteria에 뒀다. |
| 22 | precomparison multi-root HTML이 제품 runtime 입력으로 재사용된다. | 기각. browser matrix와 구현 PR의 single-root runtime fixture를 분리했다. |
| 23 | item ID/class가 geometry oracle를 고른다. | 기각. semantic rename 및 source-order reverse control을 구현 수락 기준에 넣었다. |
| 24 | explicit px/%/fr track과 새 implicit list의 조합 regression이 빠진다. | 기각. focused projection/layout matrix에 기존 explicit 조합을 요구했다. |
| 25 | Chrome 관찰만으로 expected geometry를 정의한다. | 기각. 고정 CSS·percentage 곱·gap·placement를 독립 계산하고 raw observation과 대조했다. |
| 26 | Chrome 버전·DPR·fixture hash가 결과에서 분리된다. | 기각. HeadlessChrome 156, DPR 1, source hash와 raw JSON을 연결했다. |
| 27 | 수치 허용오차 또는 모든 item frame이 누락된다. | 기각. 0.02 CSS px tolerance와 4개 case의 전체 root-relative frame을 고정했다. |
| 28 | screenshot이 pixel equality 또는 제품 캡처 증거로 과장된다. | 기각. 시각 관찰로만 분류하고 Android/iOS/readback을 미검증으로 명시했다. |
| 29 | 내부 인터페이스 버전이 변하거나 새 공개 지원이 게시된다. | 기각. 계약 0059는 `0.1.0` 유지 대상이며 C11/C11.1 미완료를 명시했다. |
| 30 | fixture/evidence 외 unrelated file 또는 공백 변경이 포함된다. | 기각. 계획/oracle 산출물 8개와 이 리뷰 기록만 stage하고 `git diff --cached --check`를 확인했다. |

## 결과

수정한 gap 문구를 제외한 신규 scope 변경은 발견하지 않았다. 검토 파일은 결과를 간결히 고정하며, 외부 reviewer나 CI 실행 결과를 뜻하지 않는다.
