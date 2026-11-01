# C11.4 text-indent flags 상태 동기화 적대 검토 30개

**대상:** `spec/STATUS.md`의 PR #959 이력 문구, 연결된 계획·비교·실행 기록 및 PR 병합 메타데이터.

**기준:** `origin/main` `359047a833132113bf4192b0340c1f8165602356`; PR #959 merge SHA `590f5946e97e0252113abf34b23ae31b13819acf`.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | 계획 후보를 구현 병합으로 잘못 표기 | PASS — PR #959 merged state와 구현 본문을 확인했다. |
| 2 | 인접 PR #962의 line-break 결과를 #959에 귀속 | PASS — 이 문단은 computed `text-indent` flags만 기록한다. |
| 3 | merge 날짜와 PR 작성 날짜 혼동 | PASS — `mergedAt` UTC `2026-11-01T02:06:32Z`를 사용한다. |
| 4 | Rebase and merge가 아닌 병합 방식 표기 | PASS — PR #959 merge record와 일치한다. |
| 5 | 잘못된 merge SHA 링크 | PASS — `590f5946e97e0252113abf34b23ae31b13819acf`를 PR metadata와 대조했다. |
| 6 | `hanging`과 `each-line` 키워드 누락 또는 뒤바뀜 | PASS — 두 computed flag를 정확히 병기한다. |
| 7 | CSS computed profile을 일반 CSS 지원으로 확대 | PASS — 제한 opt-in fixture라고 명시한다. |
| 8 | 20개 Chrome 사전 비교 scene을 20개 앱 scene 실행으로 혼동 | PASS — runtime 수치는 별도 양 플랫폼 결과로 기술한다. |
| 9 | 사전 비교 기대값과 runtime 결과 혼합 | PASS — 계획/Chrome evidence와 runtime README를 각각 연결한다. |
| 10 | AVD 결과를 실기기 결과로 표기 | PASS — Android API 37 ARM64 AVD라고 쓴다. |
| 11 | AVD software renderer 한계를 생략 | PASS — 연결된 plan/runtime evidence에 SwiftShader 제한이 남는다. |
| 12 | iOS Simulator 결과를 iOS 실기기로 확대 | PASS — iPhone 17 Pro/iOS 26.2 Simulator라고 표기한다. |
| 13 | Metal drawable 표본을 compositor 이후 픽셀로 확대 | PASS — compositor 이후 검증은 미지원으로 남긴다. |
| 14 | flags 20/20을 전체 C11.4 scene 수로 오독 | PASS — `20 scene` flags matrix로 한정한다. |
| 15 | positive 18/18의 모집단 불명확 | PASS — `positive` 결과로 분리하고 기존 plan/runtime이 scene matrix를 정의한다. |
| 16 | negative 30/30에서 positive control 구성을 잃음 | PASS — positive control 포함이라고 명시한다. |
| 17 | 양 플랫폼 수치를 합산해 허위 총량 생성 | PASS — Android/iOS에서 각각 같은 matrix를 통과했다고 쓴다. |
| 18 | 136 scene 합계를 platform당 합계로 오해 | PASS — 양 플랫폼 136 scenes라고 표현한다. |
| 19 | root frame 확인을 정확한 text paint로 확대 | PASS — glyph pixel 동등성을 미검증으로 둔다. |
| 20 | draw/present 확인을 OS compositor 표시 완료로 확대 | PASS — compositor 이후 화면은 미검증이다. |
| 21 | zero retry 기록 누락 | PASS — `superseded retry 0`을 포함한다. |
| 22 | 설치 hash를 이 문서 PR의 새 artifact로 오해 | PASS — 이 PR은 이전 PR evidence를 참조할 뿐 새 hash를 주장하지 않는다. |
| 23 | 누락된 GitHub checks를 CI 통과로 처리 | PASS — check rollup이 비어 있다고 명시한다. |
| 24 | 전체 WPT 또는 일반 author CSS admission을 암시 | PASS — 둘 다 미검증으로 명시한다. |
| 25 | 임의 font/language/writing mode 확장을 주장 | PASS — 범위 밖으로 남긴다. |
| 26 | 공식 `72/213` 수를 하위 slice 때문에 증가 | PASS — 공식 수를 그대로 둔다. |
| 27 | C11/C11.4 상위 checkbox를 완료 처리 | PASS — 상위 체크를 유지한다. |
| 28 | linked plan을 로컬 문서 상대경로로 잘못 연결 | PASS — STATUS 기준 상대경로를 확인한다. |
| 29 | PR body와 내부 실행 evidence 사이 범위 모순 | PASS — 핵심 counts, platforms, artifact limits가 서로 맞는다. |
| 30 | 문서 상태 수정이 제품 code change처럼 서술 | PASS — 이 PR을 기록 동기화로 한정한다. |

## 판정

30개 관점에서 새로운 기능 주장, 집계 변경 또는 근거 경계 위반은 발견하지 못했다. 기존 상태의 잘못된 “계획 후보·제품 구현 없음”만 실제 병합 구현에 맞게 수정했다. 이 검토는 코드/런타임을 새로 실행한 검증이 아니다.
