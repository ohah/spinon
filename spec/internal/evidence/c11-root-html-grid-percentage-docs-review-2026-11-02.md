# C11.1 percentage-track 병합 후 문서 변경 검토 30개 관점

이 검토는 PR #1019 병합 후 계획·계약·runtime evidence의 후보/병합 상태 문구를 대조한다. 저장소 규칙에 따라 서로 다른 실패 경로를 30개 관점으로 순차 확인했다. 이는 외부 reviewer나 CI 결과가 아니다.

| # | 실패 가설 | 판정 |
|---:|---|---|
| 1 | 공식 상태 대장 `spec/STATUS.md`가 이 제한 slice 때문에 완료 표시로 바뀐다. | 기각. 해당 파일은 변경하지 않고 C11 checkbox를 그대로 둔다. |
| 2 | 공식 완료 수가 72/213에서 증가한다. | 기각. 바뀐 문서들은 모두 72/213 유지로 기록한다. |
| 3 | PR #1019 상태 또는 merge SHA가 실제 원격 PR과 다르다. | 기각. GitHub PR metadata의 merged 상태와 `8e549ed55ac6f7e9611a4f7e1e571fca3a1e5684`를 대조했다. |
| 4 | PR 병합 시각이 UTC/현지 시각 사이에서 혼동된다. | 기각. 병합 시각은 `2026-11-02T01:42:07Z`로 통일했다. |
| 5 | 계획 PR #1018과 구현 PR #1019가 같은 변경으로 혼동된다. | 기각. 계획 승인과 구현 병합을 별도로 기록한다. |
| 6 | 계획 파일이 여전히 구현을 진행 중인 candidate라고 표시한다. | 수정 확인. `plan/c11-root-html-grid-percentage-tracks.md`는 구현 병합 상태를 기록한다. |
| 7 | Root HTML runtime 계획의 요약과 상세 단락이 서로 다른 병합 상태를 말한다. | 수정 확인. 두 위치 모두 PR #1019 병합과 범위 제한을 표시한다. |
| 8 | C11 상위 계획이 제한 slice 병합을 전체 C11 완료로 확대한다. | 기각. `plan/c11-css-grid.md`는 C11, C11.1, C11.3.3, C11.4를 미완료로 둔다. |
| 9 | C11.1 percentage track 절에 병합 전 표현이 남는다. | 수정 확인. 절 제목·본문 모두 병합된 제한 slice로 표현한다. |
| 10 | 내부 계약 0059가 percentage 지원을 아직 예외 후보로만 설명한다. | 수정 확인. 일반 경계와 전용 절이 병합된 limited slice로 맞춰졌다. |
| 11 | runtime README가 병합 전 source snapshot과 병합 후 checkout을 동일시한다. | 기각. README는 pre-merge snapshot임을 밝히고 현재 merge SHA를 별도로 기록한다. |
| 12 | runtime fixture SHA가 플랫폼 사이에서 바뀐 것으로 보인다. | 기각. 두 platform package의 동일 HTML SHA `957686766fd7c11441c4c4d84f755f4eb353b69867a73fd01489f8ffa5523153`을 유지한다. |
| 13 | Android 실행을 실기기 또는 hardware GPU 검증으로 오해한다. | 기각. API 37 ARM64 AVD 및 SwiftShader 소프트웨어 renderer 한계를 보존한다. |
| 14 | iOS 실행을 실기기 또는 compositor 이후 readback으로 오해한다. | 기각. iOS 26.2 Simulator와 표면 readback 미실행 한계를 보존한다. |
| 15 | screenshots를 pixel parity 결과라고 주장한다. | 기각. 화면 배치 확인용 visual evidence로만 설명한다. |
| 16 | 로그의 `layout=ready`/draw/present를 독립 pixel readback으로 오해한다. | 기각. readback과 pixel equality를 수행하지 않았다고 명시한다. |
| 17 | projection/layout/runtime 테스트 수가 원 evidence와 달라진다. | 기각. 4/4, 1/1, 16/16 수치를 원래대로 유지한다. |
| 18 | 문서 수정이 새 테스트 실행이나 새 플랫폼 실행을 주장한다. | 기각. 기존 evidence만 참조하며 새 실행을 주장하지 않는다. |
| 19 | PR 상태 check가 없는데 CI 통과라고 기록한다. | 기각. 빈 PR check rollup과 필수 check 미설정을 명시한다. |
| 20 | 일반 CSS admission 또는 전체 CSS Grid 지원을 선언한다. | 기각. 제한 Root HTML profile 및 일반 admission 부재를 명시한다. |
| 21 | cyclic/intrinsic percentage basis가 이번 구현에 포함됐다고 읽힌다. | 기각. definite finite CSS-pixel 축만 허용하고 intrinsic/cyclic sizing은 제외한다. |
| 22 | percentage gap이 지원 범위에 들어간다. | 기각. fixed-px gap만 유지하고 percentage gap은 거부한다. |
| 23 | percentage와 named area 결합이 지원 범위에 들어간다. | 기각. named-area 결합 제한을 유지한다. |
| 24 | `calc()`, `minmax()`, repeat, implicit placement 또는 named line이 허용된다고 읽힌다. | 기각. 해당 grammar와 placement는 지원 밖으로 남는다. |
| 25 | over-100% track이 clamp되거나 overflow가 숨겨진다고 설명한다. | 기각. percentage clamp 없음과 visible overflow evidence를 유지한다. |
| 26 | f32 유한 범위 및 definite-axis 전제 설명이 삭제된다. | 기각. plan과 contract의 범위 설명을 유지한다. |
| 27 | 내부 계약 숫자 버전이 기능 slice 병합만으로 증가한다. | 기각. 계약 `0.1.0`을 유지한다. |
| 28 | 이번 변경이 추가·수정한 상대 링크의 위치가 잘못됐다. | 통과. 수정된 source link와 새 PR/evidence 링크의 대상이 존재함을 확인했다. 다른 문단의 기존 누락 링크는 이번 diff 바깥이며 변경하지 않았다. |
| 29 | README 수정으로 기존 SHA256 manifest가 오래된 해시를 기록한다. | 수정 확인. README 항목을 갱신하고 manifest의 전체 항목을 `shasum -c`로 확인했다. |
| 30 | diff에 불필요한 상태 대장 변경·공백 오류·별도 worktree 변경이 섞인다. | 기각. 상태 대장 변경 없음, `git diff --check` 통과, 단일 checkout에서 작업한다. |

## 결과

문서 변경은 PR #1019의 제한 Root HTML percentage-track 구현을 병합 상태로 반영한다. 공식 C11 상태와 72/213 집계는 변경하지 않는다. PR #1019의 check rollup은 비어 있었고 필수 checks는 설정되지 않아 CI 통과를 주장하지 않는다.
