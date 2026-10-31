# C11.3.3 양 끝 implicit named-line 문서 PR diff 검토 30개

**검토 대상:** PR #933 병합 후 `spec/STATUS.md`, 내부 계약 0059, 전용 계획의 병합 상태·범위 설명과 상호 링크.

**기준 사실:** 구현 PR #933은 2026-10-31T17:35:07Z Rebase and merge됐고 merge SHA는 `ada12109a6b0394ed0a43565aa93721748955d49`다. 계획 PR #932도 선행 병합됐다. PR #933의 check rollup은 비어 있었다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | 다른 구현 PR 번호를 문서에 연결한다 | PASS — 상태 대장·계약·계획이 모두 #933을 연결한다. |
| 2 | PR head commit을 merge SHA로 잘못 기록한다 | PASS — 문서에는 GitHub merge SHA `ada12109…`만 merge 결과로 표기한다. |
| 3 | merge 시각을 PR 생성 시각으로 대체한다 | PASS — `mergedAt`인 `2026-10-31T17:35:07Z`를 적었다. |
| 4 | 저장소 규칙과 다른 merge 방법을 기록한다 | PASS — 세 문서 모두 Rebase and merge로 적었다. |
| 5 | 계획 PR #932의 선행 관계가 빠진다 | PASS — 전용 계획의 상태에서 #932와 #933의 순서를 밝힌다. |
| 6 | 계획이 아직 구현 전이라고 남는다 | PASS — 전용 계획의 상태를 구현 PR 병합으로 바꿨다. |
| 7 | 상위 C11 checkbox가 제한 slice 병합 때문에 체크된다 | PASS — 상태 대장의 C11 checkbox는 변경하지 않았다. |
| 8 | 상위 C11.3.3 candidate가 완료 처리된다 | PASS — candidate와 상위 미완료를 유지한다. |
| 9 | 공식 aggregate가 `72/213`에서 증가한다 | PASS — 세 문서가 `72/213`을 유지한다. |
| 10 | 내부 계약 숫자 버전이 바뀐다 | PASS — 계약 `0.1.0`을 유지한다. |
| 11 | 구현 profile이 template area가 있는 장면까지 확장된다 | PASS — no-area 한정이 명시돼 있다. |
| 12 | participating direct item 수 제한이 흐려진다 | PASS — layout-participating direct item 하나로 한정한다. |
| 13 | auto-sized item이 지원되는 것처럼 읽힌다 | PASS — width/height fixed 조건과 auto-size 거부를 적었다. |
| 14 | 두 implicit 축이 모두 지원되는 것처럼 읽힌다 | PASS — 정확히 한 축만 implicit이라고 제한한다. |
| 15 | 두 endpoint가 모두 implicit이어야 한다는 admission이 빠진다 | PASS — 양 끝 positive trailing implicit line을 요구한다. |
| 16 | 반대 축의 definite placement 조건이 빠진다 | PASS — opposite axis 조건을 적었다. |
| 17 | `grid-auto-*` sizing이 무제한 지원으로 표현된다 | PASS — 승인된 `auto` 또는 fixed-px만 적었다. |
| 18 | equal endpoint에서 CSS Grid의 end 제거 처리가 누락된다 | PASS — 같은 선의 end 제거를 적었다. |
| 19 | reversed endpoint 처리가 누락된다 | PASS — 역순 endpoint 교환을 적었다. |
| 20 | 여러 implicit track의 span 분배가 빠진다 | PASS — multi-track 분배와 Chrome 기준을 적었다. |
| 21 | 과도한 implicit track 생성 제한이 빠진다 | PASS — 공통 10,000-track ceiling을 적었다. |
| 22 | signed/leading, span, numeric 또는 혼합 endpoint가 허용된다고 오해한다 | PASS — 지원 밖 shape를 열거하고 fail-closed로 남긴다. |
| 23 | Chrome 156 고정 oracle이 전체 CSS 적합성 결과로 오해된다 | PASS — 전체 WPT와 일반 Grid 지원을 명시적으로 제외한다. |
| 24 | Chrome 9-scene oracle과 모바일 앱 행렬의 크기가 뒤섞인다 | PASS — 고정 oracle과 Columns/Rows 앱 scene 근거를 분리한다. |
| 25 | Android AVD 결과가 Android 실기기 결과로 바뀐다 | PASS — API 37 ARM64 AVD라고 정확히 적는다. |
| 26 | Android software renderer 한계가 빠진다 | PASS — ANGLE/Vulkan/SwiftShader라고 기록한다. |
| 27 | root 바깥 item overflow가 검증에서 빠졌다고 오해한다 | PASS — 두 platform item이 root 밖에 걸친다고 밝힌다. |
| 28 | iOS Metal readback이 compositor 이후 표시 pixel로 오해된다 | PASS — compositor 제출 전임을 적었다. |
| 29 | 전체 Rust 실행의 C07 실패를 숨기거나 새 결함으로 오인한다 | PASS — 변경과 무관한 C07 문자열 assertion 실패 한 건을 공개한다. |
| 30 | 빈 PR check rollup이 CI 통과로 둔갑한다 | PASS — check 부재를 기록하고 CI 성공이라고 표현하지 않는다. |

변경 문서와 구현 근거 README는 동일한 limited slice와 남은 범위를 가리킨다. Tailnet preview는 GitHub merge와 별도이며 배포와 Agent Browser 확인 뒤 served-content 근거를 추가한다. 이 문서 검토는 구현/runtime 20개 검토와 PR #933 실제 diff 30개 검토를 대체하지 않는다.
