# C11.4 signed percentage 상태·공개 로드맵 동기화 PR diff 검토 30개

검토 대상은 `spec/STATUS.md`의 C11.4 최신 병합 범위 요약과 `spec/public/roadmap.html`의 PR #585 카드 및 PR #583 문구 수정이다. 각 관점에서 GitHub 병합 기록, 기존 계획·명세·실행 근거와 수정 diff를 대조했다.

| # | 검토 관점 | 판정 |
| --- | --- | --- |
| 1 | 다른 저장소의 동명 PR을 잘못 참조하는가 | 공개 카드와 상태 요약의 PR 링크가 모두 `ohah/spinon` PR #585를 가리킨다. |
| 2 | PR 제목이 구현 범위를 잘못 식별하는가 | GitHub 제목 `feat(c11.4): signed percentage calc 이웃 지원 추가`가 signed-percentage slice와 일치한다. |
| 3 | 아직 열린 PR을 병합 완료로 표현하는가 | GitHub API의 상태가 `MERGED`이며 `mergedAt` 값과 대조했다. |
| 4 | 병합 시각에 로컬 시간대를 UTC로 잘못 표기하는가 | `2026-10-24T06:25:51Z`를 ISO UTC 값 그대로 기재했다. |
| 5 | 리베이스 머지 뒤 원래 feature commit을 최종 SHA로 기록하는가 | `e42b1573e710c0c36e9c507790902d4a35a53121`을 GitHub `mergeCommit.oid`와 대조했다. |
| 6 | PR #583과 후속 PR #585의 구현을 뒤바꾸는가 | 상태 대장은 #583의 signed length·nonnegative percentage 단계와 #585의 signed percentage 확장을 각각 구분한다. |
| 7 | 임의의 여러 항 `calc()`를 지원한다고 과장하는가 | 두 문서 모두 정확히 두 typed term인 제한 subset으로 한정한다. |
| 8 | 두 항의 타입 조합을 생략하거나 바꾸는가 | 하나의 `LengthPx` 항과 하나의 `Percentage` 항 조합을 실행 README와 대조했다. |
| 9 | 직접값과 부호 연산자 AST 형태를 뒤바꾸는가 | 두 항 각각 직접값 또는 단일 `Negate`만 허용한다고 기록한다. |
| 10 | 지원되는 두 항의 순서를 한 가지로 제한해 적는가 | 문서의 “순서는 양쪽을 허용” 표현이 실행 범위와 일치한다. |
| 11 | node binding이 빠져 서로 다른 요소의 항을 결합하는 것처럼 보이는가 | 동일 node 결합 조건이 상태 대장과 공개 카드에 남아 있다. |
| 12 | property binding이 빠져 다른 grid property의 항을 결합하는가 | 동일 property 조건을 두 문서에서 확인했다. |
| 13 | calc ID binding이 빠져 서로 다른 계산식을 섞는가 | 동일 calc ID 조건이 두 문서에 유지되어 있다. |
| 14 | indefinite 또는 0 content axis도 지원한다고 표현하는가 | definite positive content-axis basis 조건을 두 문서에 기록한다. |
| 15 | non-finite 값을 허용한다고 오해하게 하는가 | finite 값 검사를 지원 경계에 명시했다. |
| 16 | 길이 변환 불가능성에서 값을 조용히 손실하는가 | representability gate를 상태 대장에 보존했고 실행 설명과 대조했다. |
| 17 | 합계 overflow guard가 생략되는가 | aggregate overflow gate를 상태 대장에 명시했다. |
| 18 | 각 항을 조기에 clamp하는 것으로 설명하는가 | 최종 negative track breadth를 0으로 clamp한다고 적어 expression 평가 뒤 property range clamp임을 유지한다. |
| 19 | 신규 장면과 회귀 control을 모두 신규 지원 장면으로 세는가 | 9 신규 scene과 2 regression scene을 구분하고 총 matrix 11개로 기록한다. |
| 20 | Android와 iOS의 실행 결과를 서로 섞거나 scene 수를 부풀리는가 | Android와 iOS 각각 11/11 결과를 runtime matrix README와 대조했다. |
| 21 | PixelCopy 표본 수를 전체 화면 픽셀 비교로 부르는가 | Android는 PixelCopy 22/22 exact로 한정한다. |
| 22 | Metal 표본을 compositor 이후 화면 결과라고 부르는가 | iOS 표본을 제출 전 Metal drawable 22/22 exact로 한정한다. |
| 23 | Android AVD를 실기기 또는 hardware GPU로 표시하는가 | Android API 37 ARM64 software-rendered AVD라는 제한을 카드에 남겼다. |
| 24 | iOS Simulator를 실기기 결과로 확대하는가 | 기종과 iOS Simulator 환경을 명시하고 실기기 미검증 범위를 유지한다. |
| 25 | 두 플랫폼의 surface sample을 동일 API 결과인 것처럼 섞는가 | PixelCopy와 pre-compositor Metal drawable을 플랫폼별로 나눠 기재했다. |
| 26 | static archive 확인을 최종 APK·IPA 제외 증명으로 확대하는가 | 기본/fixture-test static archive 경계만 언급하고 APK/IPA exclusion은 미검증이라고 적었다. |
| 27 | 보고된 required check이 없는데 CI 통과를 주장하는가 | 공개 카드는 required check 미보고를 기록하고 녹색 CI 주장을 하지 않는다. |
| 28 | 이전 PR #583 문장을 현재 작업으로 오인할 수 있게 남기는가 | 공개 로드맵 표현을 “PR #583 병합 구현은”으로 고쳐 상태를 명확히 했다. |
| 29 | 제한 구현을 C11.4 전체 완료로 올려 공식 진행 수를 바꾸는가 | C11.4 미완료 checkbox와 `72/213` 집계를 그대로 유지한다. |
| 30 | 변경으로 내부 계약 숫자 버전이 불필요하게 올라가는가 | 내부 계약 `0.1.0`을 유지하며 이 문서 변경은 구현 계약 확장이 아니다. |

## 대조 자료

- [PR #585 병합 기록](https://github.com/ohah/spinon/pull/585)
- [실행·화면·artifact 근거](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/README.md)
- [구현/runtime 검토 20개](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/implementation-runtime-review-20.md)
- [PR diff 검토 30개](../c11-4-auto-repeat-negative-percentage-neighbors-runtime-2026-10-24/pr-diff-review-30.md)
- [내부 Grid 계약 0059](../../../internal/0059-c11-grid-layout.md)
- [공식 상태 대장](../../../STATUS.md)
