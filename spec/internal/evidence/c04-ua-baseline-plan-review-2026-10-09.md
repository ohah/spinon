# C04.5 계획 적대 검토

**대상:** [지원 HTML UA 계산 스냅샷 계획](../../../plan/c04-ua-baseline-snapshot.md)
**상태:** 구현 전에 검토 완료
**검토 구분:** 계획 검토이며 구현 코드 검토나 기능 통과 근거가 아니다.

| # | 실패 관점 | 검토 결과와 반영 |
|---:|---|---|
| 1 | computed-style 함수 하나를 제품 CSS 완성으로 홍보 | 계획에 V8·레이아웃·GPU·앱 연결 제외와 C04 미완료 유지 명시 |
| 2 | Chromium revision이 C01 기준과 달라 비교가 재현되지 않음 | revision·버전·reference 파일을 함께 고정 |
| 3 | 선택자 존재만 보고 UA 규칙 적용으로 간주 | 19개 computed value를 직접 비교하도록 합격 조건 설정 |
| 4 | 9개 요소 중 일부만 실행해 성공 처리 | inventory와 fixture 노드 ID 전체를 검사하도록 명시 |
| 5 | 19개 feature 중 빠진 property를 성공으로 오인 | 누락·중복·추가 항목을 모두 실패 경계로 설정 |
| 6 | UA stylesheet를 author origin에 넣어 origin 우선순위 검증으로 오해 | 내장 규칙은 UA origin으로 등록하고 별도 author override 확인 |
| 7 | author stylesheet의 origin이 잘못되어도 받아들임 | author origin 이외 입력은 구체 오류로 거부하도록 계획 |
| 8 | 지원 요소 이름은 같지만 namespace가 달라 UA 규칙이 잘못 적용 | HTML namespace 일치 및 namespace 불일치 부정 사례 포함 |
| 9 | 다중 HostRoot 요소를 한 문서 root로 묵시적으로 합침 | 호출자가 기존 계약에 따라 하나의 HostRoot 직속 root를 명시 |
| 10 | 다른 generation의 root handle을 사용 | 잘못된 root 사례와 view 생성 오류를 검증 범위에 포함 |
| 11 | 실제 화면 viewport 대신 fixture 상수를 제품 입력으로 사용 | viewport를 매 호출 명시 입력으로 받고 고정값은 oracle 테스트 전용 |
| 12 | NaN·무한대·0 viewport에서 Stylo 내부로 진입 | 잘못된 viewport를 실패 경계로 명시 |
| 13 | device scale과 CSS viewport를 혼동 | width·height·scale 값을 별개로 고정하고 snapshot 보존 확인 |
| 14 | `1em` margin이 폰트 metrics 차이로 달라짐 | 고정 Chromium reference의 computed value만 비교하고 폰트 shaping은 범위 밖으로 분리 |
| 15 | `inline`·`inline-block`·`list-item` 계산을 레이아웃 구현으로 주장 | 계획에서 계산값과 배치·marker 구현을 분리 |
| 16 | 외부 URL을 테스트 중 네트워크에서 가져와 결과가 달라짐 | 고정 입력과 메모리 author sheet만 사용하고 네트워크 loader 제외 |
| 17 | parser 진단을 버려 malformed CSS를 조용히 성공으로 처리 | 진단 보존을 출력 계약과 테스트 경계에 포함 |
| 18 | 문서·style·environment revision이 계산 중 빠짐 | 네 축의 snapshot echo를 검사하도록 완료 기준에 포함 |
| 19 | 새 profile 추가가 C04.1/2/3/4의 허용 범위를 넓힘 | 새 profile/함수 분리 및 기존 profile 회귀 검증을 순서에 명시 |
| 20 | 새 API에 예제·오류·상태 문서가 없어 내부 사용자가 계약을 추측 | 버전 있는 내부 계약·상태 대장·근거 갱신을 완료 조건에 포함 |

## 검토 후 확정한 경계

이 계획은 C04.5의 계산 스타일 API만 구현한다. UI root 정책, 스타일 실행기 소유권, Taffy의 inline/list-item 배치, 런타임·GPU 연결을 임의로 결정하지 않으며 이 구현으로 C04 전체나 CSS 제품 지원을 완료 처리하지 않는다.
