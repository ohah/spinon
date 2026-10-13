# C11.3.2 named grid lines 구현 실패 경계 검토 · 2026-10-14

## 검토 범위

기준 commit `ed089f8` 위의 `feat/c11-3-2-named-grid-lines` 후보를 검토했다. Stylo typed computed value, explicit N+1 line-name metadata, placement projection, Taffy 정규화, 오류 반환, 앱 전용 fixture FFI와 증거 경계를 포함한다. 계획 자체의 실패 관점은 [별도 C11.3 계획 검토](./c11-3-template-repeat-named-lines-areas-plan-review-2026-10-13.md)에서 다뤘다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검사와 결과 |
| --- | --- | --- |
| 1 | named-line 처리가 기존 Grid profile까지 새어 들어감 | cascade가 `RuntimeBlockFlexGridTemplateV1`에서만 named value를 허용한다. 기존 numeric placement profile은 named 입력을 계속 `Unsupported`로 둔다. 전체 workspace와 legacy profile 회귀가 통과했다. |
| 2 | parser 문자열을 다시 읽거나 `name N`과 `N name` 순서를 다르게 해석 | Stylo `GridLine` typed value에서 이름·정수·span을 읽는다. 두 identifier/integer 순서의 cascade/layout 결과가 같은 직접 검사를 통과했다. |
| 3 | positive occurrence가 0-based 또는 track index로 투영됨 | `name 1`은 첫 이름 line의 CSS line number로 변환한다. 여러 이름 group을 가진 fixture의 `name 2` 및 Chrome geometry 검사가 통과했다. |
| 4 | negative occurrence가 explicit grid 시작에서 역산됨 | 음수 occurrence는 마지막 explicit line부터 역방향으로 세고 부족하면 실패한다. `-2`, `-3` 및 성공/부족 fixture를 단위·cascade 검사했다. |
| 5 | 같은 physical line의 동일 이름 토큰을 여러 occurrence로 셈 | line별 이름 목록에서 `.any()`로 한 번만 세며 원본 중복은 metadata에 보존한다. 같은 line에 이름 토큰 2개가 있어도 occurrence/span 하나로 취급하는 단위 검사가 통과했다. |
| 6 | repeat seam의 두 metadata group을 다른 line으로 셈 | C11.3.1 cascade의 seam 병합 reference에 더해, `repeat(2, [seam] 40px)`의 두 번째 physical seam occurrence를 배치하는 Stylo→layout integration 검사가 통과했다. |
| 7 | 이름 비교가 대소문자를 무시하거나 locale에 따라 바뀜 | CSS identifier의 보존된 문자열을 bytewise equality로 조회한다. `Edge`/`edge` 성공과 `EDGE` 거부 단위 검사가 통과했다. |
| 8 | occurrence 0 또는 `i32::MIN`에서 산술 overflow·임의 line 반환 | 0은 명시 오류로, 음수 ordinal은 `unsigned_abs()`를 거쳐 처리한다. `0`과 `i32::MIN` 둘 다 `UnsupportedNamedGridLine`로 닫는 직접 회귀 검사가 통과했다. |
| 9 | 없는 이름이나 occurrence 부족이 부분 layout을 생성 | 시작/끝 metadata 조회 오류는 layout projection 전에 반환한다. 누락·양/음 방향 부족 테스트가 오류 반환을 확인했고, integration 결과 객체가 생성되지 않는다. |
| 10 | 지원 밖 implicit named line을 explicit line으로 가장 | 현재 후보는 implicit line 생성과 이름 가정을 지원하지 않고 구체 오류로 닫는다. 이것은 제한 profile 경계다. CSS Grid 전체 규범 동작 또는 WPT 적합성으로 주장하지 않는다. |
| 11 | forward `span <name>`이 grid 시작이나 같은 시작 line부터 셈 | definite start 다음 line부터 end 방향으로만 찾는다. 2번째 이름 line으로 가는 unit·Chrome geometry 검사가 통과했다. |
| 12 | backward `span <name>`이 end 반대 방향으로 검색되지 않음 | definite end line 앞에서 start 방향으로 찾는다. count 2의 결과와 `-1` numeric end에서 count 1의 결과를 직접 확인했다. |
| 13 | span 안에서 같은 line 중복 이름이 count를 증가 | line metadata group마다 한 번 카운트한다. 같은 line의 토큰 중복과 이후 physical line을 구분하는 단위 검사가 통과했다. |
| 14 | span count가 0이거나 `u32::MAX`일 때 0 span·overflow·긴 무한 탐색 발생 | count 0은 decode 단계에서 거부한다. 매우 큰 count는 explicit N+1 경계까지만 순회하고 implicit line을 만들지 않은 채 오류로 끝난다. 두 경계 검사가 통과했다. |
| 15 | 반대편 definite line이 없는 lone named span이 이름을 찾아 implicit grid를 만듦 | CSS conflict/profile 규칙대로 이름을 무시하고 `span 1`로 축소한다. 누락 이름 lone-span layout 검사가 통과했다. |
| 16 | 두 span 충돌에서 잘못된 쪽을 남기거나 버린 span 이름을 조회 | end property에서 온 span을 제거하고 남은 named span을 1로 바꾼다. 두 named span의 이름을 조회하지 않는 direct negative test가 통과했다. |
| 17 | named span과 음수 numeric definite edge 혼합에서 방향 또는 normalization 오류 | `normalize_explicit_line()`이 음수 numeric index를 explicit grid에 대해 정규화한 뒤 방향 검색을 시작한다. `span edge / -1` 상당의 backward unit 검사가 통과했다. |
| 18 | row 이름을 column metadata에서 찾거나 부모 Grid 이름을 중첩 child에 사용 | 각 축이 자기 container의 row/column line names를 전달한다. nested Grid 안에서 부모와 다른 row/column 이름을 쓰는 layout 검사가 통과했다. |
| 19 | N개 track인데 line group N개만 받거나 손상된 metadata를 신뢰 | projection은 checked `track_count + 1 == names.len()`를 먼저 요구한다. N+1 정상 형상과 잘못된 길이 fail-closed 단위 검사가 통과했다. |
| 20 | named resolution 뒤 역전·동일 line conflict를 Taffy에 잘못 넘기거나 fixture가 기본 산출물에 포함 | numeric line의 역전 swap·동일 line span-1은 CSS geometry integration에서 통과했다. 기본 `spinon-ffi` static library에서는 C11.3.2 심벌/payload가 없고 opt-in positive control에는 있다. 이는 app release bundle 검증은 아니다. |

## 새로 발견해 고친 항목

- 새 named-line 함수의 별도 metadata 인수 2개가 Clippy `too_many_arguments`를 만들었다. `LineMetadata` 입력으로 묶고 range loop를 iterator 기반으로 바꿨다.
- feature FFI의 overflow negative test가 정수에서 raw pointer를 만들어 `manual_dangling_ptr`를 냈다. `NonNull::<u32>::dangling().as_ptr()`로 바꿔 feature Clippy를 통과시켰다.
- line metadata 변경의 다음 style revision 반영을 직접 확인하는 회귀가 없었다. root/item style을 바꾸고 다음 `StyleRevision`에서 새 이름으로 배치하는 integration 테스트를 추가했다.
- 새 회귀에서 `i32::MIN`, `u32::MAX`, negative numeric definite line과 named span 조합을 추가 확인했다.
- repeat-seam integration의 첫 fixture는 공통 item helper가 기대하는 row line name 및 explicit line 범위와 맞지 않아 fail했다. helper 계약과 N+1 line 수에 맞춰 fixture를 바로잡았고 최종 검사에서 통과했다. 제품 경로 결함은 아니었다.

## 증거와 판정

- 직접 projection: [named-line unit log](./c11-3-2-grid-named-lines-runtime-2026-10-14/named-line-test.log)
- Stylo→layout: [named-line integration log](./c11-3-2-grid-named-lines-runtime-2026-10-14/named-line-integration-test.log)
- 전체 Rust workspace, CSS reference, 기본/fixture FFI, Clippy, rustfmt, diff: [통합 실행 요약](./c11-3-2-grid-named-lines-runtime-2026-10-14/README.md)
- 고정 기준: [Chrome/WPT 사전 비교](./c11-3-2-named-lines-precomparison-2026-10-14/README.md)
- 플랫폼 및 opt-in 산출물: [Android/iOS 실행·fixture artifact 경계](./c11-3-2-grid-named-lines-runtime-2026-10-14/README.md)

검토 결과는 제한된 C11.3.2 구현 후보를 지지한다. C11.3.2 PR은 아직 미병합이고, implicit named line 생성, 전체 15-scene 모바일 행렬, iOS 실기기 및 compositor 이후 pixel, Spinon WPT 적합성, template areas/C11.3.3와 CSS Grid 전체 지원은 미완료다. 공개 제품 CSS 지원으로 표시하지 않는다.
