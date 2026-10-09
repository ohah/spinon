# C04.7 계획 검토 기록

계획 단계의 검토 질문을 서로 다른 실패 경로로 나눴다. 확인 근거는 고정된 `stylo = 0.22.0` 소스 `servo/media_features.rs`, `device/servo.rs`와 저장소의 Chromium CDP fixture 도구다. 이 검토는 구현 통과나 제품 runtime 완성을 뜻하지 않는다.

| # | 적대적 질문 | 반영 또는 경계 |
|---:|---|---|
| 1 | 색상 모드를 viewport와 따로 revision 관리하면 서로 다른 시점 값이 섞이지 않는가? | media 입력을 `CssViewport`에 묶고 기존 `EnvironmentRevision` 하나로 stamp한다. 발급자는 이 slice 밖이다. |
| 2 | viewport 크기만 바뀌어도 revision을 올려야 하는가? | 그렇다. viewport와 media 입력을 같은 environment snapshot으로 취급한다. |
| 3 | 모바일 컴파일 타깃이라는 이유로 coarse를 자동 선택하는가? | 아니다. target OS의 Stylo 기본값에 의존하지 않고 호출자가 명시한다. |
| 4 | 데스크톱 값이 host OS별 기본값에 흔들리는가? | 결정적 desktop fixture를 기본으로 지정한다. |
| 5 | primary pointer와 all pointer가 같은 필드로 합쳐지지 않는가? | CSS `pointer`/`hover`와 `any-pointer`/`any-hover` 입력을 나눠 모델링한다. |
| 6 | primary fine이 all capability에 없을 수 있는가? | 모순으로 거부한다. |
| 7 | primary hover가 all capability에 없을 수 있는가? | 모순으로 거부한다. |
| 8 | 전체 hover만 있고 실제 pointer가 하나도 없는 입력이 가능한가? | 모순으로 거부한다. |
| 9 | 전체 capability가 fine과 coarse를 동시에 표현할 수 있는가? | 허용한다. Stylo는 bitflag 집합으로 표현한다. |
| 10 | primary가 none이어도 다른 장치가 있을 수 있는가? | primary hover는 금지하되 all 집합은 독립적으로 표현한다. 실제 플랫폼 정책 결정은 runtime 계약에 남긴다. |
| 11 | media parser와 cascade에서 query가 평가되는가? | Stylo Device가 scheme와 primary/all capabilities를 받는 생성자를 사용하고 Chromium 값과 비교한다. |
| 12 | 색상 query만 검증하고 hover 의미는 빠지지 않는가? | `hover:none/hover`, `any-hover:none/hover` probe를 포함한다. |
| 13 | pointer none query를 coarse/fine query로 잘못 해석하지 않는가? | no-pointer 입력을 별도 mapping 테스트로 두고 Chromium emulation 실측과 구분한다. |
| 14 | `matchMedia()` 입력과 CSS computed 결과가 불일치해도 기준을 저장하는가? | capture가 양쪽 관찰값을 기록하고 기대값이 아니면 reference를 생성하지 않는다. |
| 15 | CDP의 mobile flag만으로 touch capability가 보장되는가? | touch emulation을 별도로 켜고 실제 media query 결과로 검증한다. |
| 16 | device scale과 CSS viewport 단위가 뒤섞이는가? | viewport는 CSS px, DPR은 별도 양수 입력이며 CSS media 값 측정은 geometry 비교를 하지 않는다. |
| 17 | invalid 입력에서 일부 스타일 결과가 반환될 수 있는가? | cascade 진입 전에 전체 environment를 검증하고 오류로 끝낸다. |
| 18 | 기존 profile의 동작이 조용히 바뀌는가? | 구현 중 기존 `FlexMarginV1`이 모든 at-rule을 거부하는 것을 확인했다. 이를 확장하지 않고 신규 `FlexMediaEnvironmentV1`과 전용 검사 경로를 계획에 추가한다. |
| 19 | OS 환경 수집이나 자동 invalidation까지 구현했다고 과장할 수 있는가? | 계획과 상태에서 수집·revision manager·scheduler·재계산·GPU 반영을 미구현으로 둔다. |
| 20 | reduced-motion 등 다른 media feature까지 지원한다고 오해할 수 있는가? | 이번 표면을 scheme/pointer/hover로 제한하고 나머지를 명시적으로 제외한다. |

## 계획 검토 후 고정한 기준

Chromium `matchMedia()`는 입력 environment를 직접 확인하는 oracle로 사용한다. mobile emulation이 coarse/no-hover를 실제 제공하지 않으면 fixture나 reference를 기대값에 맞춰 고치지 않고 캡처를 실패시킨다. 혼합 포인터 및 장치 없음 case는 이번 Chromium 실행에서 만들 수 없으므로 source-defined Stylo mapping 시험으로 분리한다. 제품 runtime 연결, OS notification과 stale result admission은 별도 후속 작업이다.

구현 시작 때 기존 profile의 at-rule 거부 계약을 확인해 계획을 보강했다. C04.6 profile을 넓히면 기존 미지원 CSS 경계가 바뀌므로 신규 profile을 추가하고, media feature surface도 입력 검증으로 제한한다.
