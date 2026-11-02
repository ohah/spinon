# PR diff 적대 검증 30개 — C11.1 percentage gap 상태·roadmap 동기화

대상은 구현 PR #1072 병합 이후의 상태 대장, C11 계획, percentage-gap 계획, 내부 계약 0059, 실행 근거 README, 공개 roadmap HTML 변경이다. 각 행은 별도 실패 경로 또는 상태·계약 경계를 확인했다.

| # | 독립 실패 경로 / 불변 조건 | 확인 결과 |
|---:|---|---|
| 1 | 구현 PR이 열린 상태인데 이미 병합됐다고 적힘 | #1072의 GitHub 상태와 merge SHA를 대조했고 `MERGED`와 `6508a1a91d04c7e83c00b10f230519d52e8ed86b`가 일치한다. |
| 2 | 구현 시각에 UTC와 로컬 시각을 혼용 | GitHub `mergedAt`의 UTC `2026-11-02T17:41:15Z`를 그대로 기록했다. |
| 3 | merge SHA가 후보 커밋 또는 구현 커밋과 혼동 | merge SHA와 simulator source commit `f92352d97eddfa5e6142d9db78d5a1900879587b`를 별도 표기했다. |
| 4 | 제한 구현이 C11 전체 또는 C11.1 전체 완료로 승격 | 모든 변경 문서가 C11.1/C11 미완료라고 기록하고 상위 checkbox를 유지한다. |
| 5 | 제한 slice만으로 공식 완료 수 증가 | 상태 대장, 계획, contract, public roadmap의 `72/213`이 일치한다. |
| 6 | 체크박스가 의도치 않게 완료로 변경 | diff에서 `spec/STATUS.md`는 설명 이력 한 줄만 추가하고 checkbox 토큰은 바꾸지 않았다. |
| 7 | 공식 상태의 단일 원본이 preview용 계획과 뒤바뀜 | 상태 판정은 `spec/STATUS.md`에 유지하고 계획 문서는 실행 범위를 보충한다. |
| 8 | percentage gap이 전체 CSS parser/admission 지원처럼 설명 | 지원 문구를 기본 parsed Root HTML Grid 제한 경로로 한정했다. |
| 9 | percentage basis가 불명확해 row/column 축이 뒤섞임 | 각 gap의 definite content-box 대응 축 기준임을 계획과 public card에 유지했다. |
| 10 | `gap` shorthand와 `grid-gap` alias가 서로 다른 계산 경로로 오해 | 두 표기가 computed longhand의 같은 지원 경로를 쓴다고 기록했다. |
| 11 | 100% 초과 값이 clamp된다고 문서화 | 0% 및 100% 초과 overflow를 보존하고 clamp하지 않는다고 기록했다. |
| 12 | indefinite basis를 지원하는 것으로 오해 | cyclic/indefinite basis는 범위 밖임을 계획과 실행 근거에 유지했다. |
| 13 | `calc()` 및 typed math gap 지원으로 오해 | math-backed 값을 fail-closed/범위 밖으로 명시했다. |
| 14 | 음수·비유한·잘못된 computed 값이 허용된다고 오해 | 유한 비음수 제한 및 invalid input fail-closed 경계를 유지했다. |
| 15 | f32 범위 초과 값이 유효한 큰 간격으로 허용된다고 오해 | finite-f32 overflow 거부 경계를 실행 근거와 계획에서 확인했다. |
| 16 | 변경이 Flex/Grid 외 profile로 전파됐다고 오해 | Flex 및 다른 runtime profile은 변경하지 않았음을 근거에 남겼다. |
| 17 | screenshot을 surface pixel readback으로 과장 | Android/iOS 캡처를 visual smoke로 한정하고 PixelCopy/Metal readback이 아니라고 썼다. |
| 18 | simulator 결과가 실기기 증거로 불림 | Android API 37 AVD와 iPhone 17 Pro Max/iOS 26.2 Simulator라고 정확히 구분했다. |
| 19 | Android renderer가 hardware GPU로 오해 | 실행 근거의 ANGLE/SwiftShader 제한을 유지했다. |
| 20 | iOS draw/present가 compositor 이후 pixel equality로 과장 | surface readback/equality 주장을 하지 않는다고 기록했다. |
| 21 | focused tests 결과가 broad suite 전체 성공으로 합쳐짐 | focused 통과와 broad suite 실패를 별도 수치로 적고 전체 green이라고 부르지 않는다. |
| 22 | broad style-to-layout 분모·실패 수 오기 | `319/321` 결과를 실행 README와 대조했다. |
| 23 | broad layout 분모·실패 수 오기 | `376/377` 결과를 실행 README와 대조했다. |
| 24 | 변경과 무관한 실패를 이번 변경 원인으로 단정 | broad suite의 기존 C07/C15 실패임을 기록하고 범위 밖으로 분리했다. |
| 25 | GitHub check가 없는데 CI 성공으로 보고 | PR check rollup 빈 배열을 확인했고 CI 통과라고 적지 않았다. |
| 26 | required checks 설정 부재를 통과 상태로 표현 | main required status checks 미설정을 별도로 적었다. |
| 27 | 내부 계약 버전을 구현 확장만으로 올림 | 계약 0059를 `0.1.0`으로 유지했다. |
| 28 | 계약 문서의 merge 상태·남은 실패 설명이 상태 대장과 충돌 | 계약, 실행 README, 계획의 merge SHA·미완료 상태·broad test 한계를 대조했다. |
| 29 | public roadmap 링크의 source route 또는 screenshot filename이 틀림 | 링크 stem을 저장소의 계획·계약·evidence 및 두 tracked screenshot 파일과 대조했다. |
| 30 | 새 roadmap card가 기존 카드 순서·공식 상태 링크를 깨뜨림 | PR #1072 카드를 PR #1070 뒤, 이전 C11.1 항목 앞에 넣고 공식 `STATUS.html` anchor와 이 변경 검토 문서 링크를 유지했다. |

## 결과

30개 관점에서 상태, 범위, 검증 근거와 문서 링크를 대조했다. `git diff --check`를 실행했다. 미해결 C11/C11.1 범위, broad package test 실패와 CI check 부재를 명시해 완료 또는 green으로 과장하지 않았다.
