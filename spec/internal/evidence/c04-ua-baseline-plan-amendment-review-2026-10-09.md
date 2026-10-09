# C04.5 계획 수정 · media environment 경계 검토

**대상:** [C04.5 지원 HTML UA 계산 스냅샷 계획](../../../plan/c04-ua-baseline-snapshot.md)
**수정 계기:** 구현 코드 검토에서 `CssViewport`에 없는 target-dependent pointer capability를 확인했다.
**검토 구분:** 기존 20개 계획 검토와 구별되는 수정 범위 점검이다. 코드 구현 통과 근거는 실행 근거 문서에 둔다.

| # | 수정 계획의 실패 관점 | 판정 및 반영 |
|---:|---|---|
| 1 | viewport 폭·높이·scale을 완전한 media environment로 오해 | plan/contract에 입력 범위를 구분했다. |
| 2 | `environment_revision` 숫자만으로 OS 환경 값이 복원된다고 오해 | revision은 snapshot echo이며 OS 조회/복원은 아님을 명시했다. |
| 3 | C01 UA 기본 규칙에 media query가 있다고 잘못 판단 | 고정한 UA CSS에서 환경 의존 media query가 없음을 확인했다. |
| 4 | C01 비교 author CSS가 환경 분기를 바꿀 수 있음 | oracle 기준 author sheet 수가 0임을 확인하고 한계를 명시했다. |
| 5 | Android/iOS와 desktop이 primary pointer 조건에서 동일하다고 가정 | pinned Stylo `PointerCapabilities::default()`가 target별 coarse/fine으로 갈림을 계약에 반영했다. |
| 6 | primary pointer와 any-pointer를 같은 입력으로 간주 | 둘 다 별도 환경 항목으로 후속 제품 계약 목록에 넣었다. |
| 7 | hover 가능 여부가 pointer 정확도만으로 결정된다고 가정 | hover를 독립 후속 환경 입력으로 기록했다. |
| 8 | dark/light scheme이 viewport 크기 입력에 포함된다고 오해 | 현재 내부 기본값 `light`를 별도 표기했다. |
| 9 | reduced-motion 선호가 자동으로 OS에서 전달된다고 오해 | motion 선호는 현재 입력되지 않고 후속 환경 계약 대상임을 명시했다. |
| 10 | contrast·forced-colors 설정을 이미 계산한다고 오해 | contrast 계열 사용자 선호 입력도 후속 항목으로 분리했다. |
| 11 | platform compile check가 실제 media query 결과 동등성을 증명 | iOS·Android 확인은 target compile뿐이며 실행 결과가 아님을 plan/evidence에 남겼다. |
| 12 | font metrics가 실기기 글꼴과 동일하다고 오해 | fixed font metrics는 제한된 내부 기본값이며 실제 font 공급·shaping과 별도임을 기록했다. |
| 13 | CSS `resolution`의 device scale을 viewport scale 하나로 완전히 증명 | 현재 입력은 viewport/device scale 값뿐이며 나머지 environment 조합은 보장하지 않는다고 적었다. |
| 14 | media `screen`/`print` 전환 가능 | 현재 device는 screen으로 고정되고 print 전환 입력이 없음을 명시했다. |
| 15 | locale·time zone이 CSS 계산 환경 전부라고 판단 | C01의 locale/time zone 관찰 조건과 API 입력 계약을 구별하고, 완전한 지역 환경 입력은 보장하지 않았다. |
| 16 | author `@media`가 UA 기본값 비교와 같은 기준을 자동 상속 | UA baseline oracle과 author media parity를 별도 경계로 나눴다. |
| 17 | target-dependent 결과를 Chromium 값 허용오차로 감춤 | CSS 문자열 exact comparison 대상은 고정 UA baseline 19개 값으로 한정했다. |
| 18 | 스타일 재계산이 environment revision 증가를 스스로 감지 | 호출자는 새 입력과 revision으로 재호출해야 하며 무효화/스케줄링은 미구현임을 명시했다. |
| 19 | 이 내부 slice에 완전한 environment API까지 끌어들여 구현 범위 팽창 | C04.5는 환경 객체 설계를 결정하지 않고 product runtime의 별도 결정으로 미뤘다. |
| 20 | 제한된 C04.5 결과를 범용 cross-platform stylesheet API로 사용 | 공개/API 전체 CSS 호환 선언을 금지하고 media 환경 경계를 계약·상태·preview에 동기화하기로 했다. |

## 반영 후 결정

C04.5의 합격 범위는 환경 의존성이 없는 고정 Chromium UA baseline과 기본 author/inline override로 유지한다. 입력에 포함되지 않는 media feature parity는 완료로 표시하지 않는다. 제품 runtime에서 이를 지원할 때는 environment snapshot 항목·source of truth·revision 전파·재계산 조건을 별도 계획으로 먼저 정한다.
