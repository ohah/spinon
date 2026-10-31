# C11.4 `text-indent` 병합 상태 동기화 PR diff 검토 30개

## 검토 대상과 경계

검토 대상은 이 PR의 `plan/c11-4-anonymous-grid-text-computed-text-indent.md`, `plan/c11-4-grid-track-sizing.md`, `spec/STATUS.md`, 그리고 문서 변경 검토 기록이다. 병합 PR #930의 실제 상태는 GitHub에서 다시 조회했다. 제품 코드, Rust 테스트, Android/iOS 앱 실행은 이 문서 diff 검토에 포함되지 않는다.

| # | 독립 실패 관점 | diff 대조 결과 |
|---:|---|---|
| 1 | PR #930이 실제 병합되지 않았는데 merged로 적을 수 있다 | GitHub PR 상태가 `MERGED`다. 통과. |
| 2 | 병합 커밋 SHA가 GitHub merge commit과 다를 수 있다 | 세 문서의 SHA `b1b20e30cc8d36e90d87e902668eb95339698c25`가 일치한다. 통과. |
| 3 | 병합 시각이 PR의 병합 시각과 다를 수 있다 | 계획·상태의 `2026-10-31T16:37:17Z`가 GitHub 값과 일치한다. 통과. |
| 4 | PR 링크가 다른 구현 PR을 가리킬 수 있다 | 링크를 #930으로 열어 제목과 목적을 대조했다. 통과. |
| 5 | 수정한 계획 상태가 PR 병합 후에도 검토 중이라고 남을 수 있다 | 상단 상태는 병합 결과로 바뀌었고, 제한 범위를 보존한다. 통과. |
| 6 | 계획 본문 구현 절차가 사후에 삭제되어 재현 정보가 사라질 수 있다 | 구현 전 고정 범위·expected values·검증 순서는 그대로다. 통과. |
| 7 | 사후 상태 문구가 plan 단계와 runtime 단계의 근거를 혼동할 수 있다 | 기존 plan-review 링크와 runtime evidence를 각각 별도 링크로 둔다. 통과. |
| 8 | 부모 계획에 늦은 항목이 과거 항목 뒤에 삽입되어 순서가 뒤집힐 수 있다 | 새 `2026-11-01` 이력을 기존 `2026-10-31` 이력 앞에 배치했다. 통과. |
| 9 | 부모 계획에 text-indent의 구현 범위가 빠질 수 있다 | 0/5/10px·Ahem 10px·LTR·normal white-space·18 scenes를 기록했다. 통과. |
| 10 | 부모 계획의 기존 word-spacing 기록이 실수로 교체될 수 있다 | 기존 #924 섹션과 그 뒤 이력은 보존됐다. 통과. |
| 11 | STATUS가 최신 관찰일과 merge 일자를 같은 날처럼 보이게 할 수 있다 | 11월 1일 기록일과 10월 31일 merge 시각을 구분했다. 통과. |
| 12 | STATUS의 상태 용어가 미병합 구현 후보로 남을 수 있다 | `구현 후보`를 “구현 PR #930 병합”으로 고쳤다. 통과. |
| 13 | C11.4 하위 slice가 C11.4 공식 완료로 확대될 수 있다 | C11.4 checkbox를 미완료로 남겼다. 통과. |
| 14 | C11.4 slice가 C11 parent 완료로 확대될 수 있다 | C11 checkbox를 미완료로 남겼다. 통과. |
| 15 | 구현 PR 병합으로 전체 로드맵 완료 수가 잘못 증가할 수 있다 | 공식 집계 `72/213`을 유지했다. 통과. |
| 16 | 18개 fixed scene이 임의 문서와 임의 텍스트를 뜻한다고 과장될 수 있다 | scene 수는 고정 fixture에 한정되어 있다. 통과. |
| 17 | 0/5/10px 결과가 모든 computed `text-indent` 값에 적용된다고 읽힐 수 있다 | 이 세 값만 지원하며 미지원 입력 목록을 유지한다. 통과. |
| 18 | Ahem 10px/400 조건이 빠져 글꼴 독립 지원으로 보일 수 있다 | 두 문서의 범위와 STATUS에 고정 font 조건을 남겼다. 통과. |
| 19 | min-/max-content 산출과 line metric 검증이 사라질 수 있다 | STATUS 기록에 intrinsic 크기·line advance·track 검증을 유지했다. 통과. |
| 20 | Chrome observation을 독립 규범 기대값으로 오해할 수 있다 | Chrome 관측과 독립 Ahem 산식을 별개 근거로 표현했다. 통과. |
| 21 | Android Simulator/AVD 증거가 Android 실기기 증거로 오기될 수 있다 | Android API 37 ARM64 AVD로 명시했다. 통과. |
| 22 | iOS Simulator 증거가 iOS 실기기 증거로 오기될 수 있다 | iPhone 17 Pro/iOS 26.2 Simulator로 명시했다. 통과. |
| 23 | 실행 요약이 hardware GPU 검증을 주장할 수 있다 | hardware GPU는 범위 밖으로 남겼다. 통과. |
| 24 | 화면 캡처가 glyph raster 정확성을 입증한다고 읽힐 수 있다 | 검은 renderer 영역과 glyph paint 한계를 유지했다. 통과. |
| 25 | runtime scene 결과가 전체 WPT 적합성으로 확대될 수 있다 | WPT는 검증 범위 밖이라고 명시했다. 통과. |
| 26 | opt-in fixture가 일반 CSS admission으로 확대될 수 있다 | 두 문서 모두 일반 CSS admission을 명시적으로 제외한다. 통과. |
| 27 | 문서 수정이 공개 API 또는 내부 계약을 새로 바꾼 것처럼 보일 수 있다 | 내부 계약 0059 링크만 유지했으며 내용·버전은 변경하지 않았다. 통과. |
| 28 | relative evidence 링크가 누락 경로를 가리킬 수 있다 | precomparison/runtime/review 파일이 현재 main에 추적되고 존재한다. 통과. |
| 29 | 별도 20개 변경 검토가 PR diff 검토와 합산되거나 재사용된 것처럼 보일 수 있다 | 두 보고서를 분리하고 이 문서에서 서로 다른 diff 관점을 새로 점검했다. 통과. |
| 30 | review 문서 자체가 코드 재실행이나 simulator 검증을 주장할 수 있다 | 20개 review 결과를 문서 대조로 한정하고 재빌드·simulator 미실행을 적었다. 통과. |

## 판정

30개 diff 관점에서 stale 상태 표현 외의 계약·증거·공식 완료 집계 회귀를 찾지 못했다. 문서 PR이므로 테스트나 앱 실행은 새로 주장하지 않는다. 저장소 규칙에 따라 문서 사이트 자동 배포도 실행하지 않았다.
