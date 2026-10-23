# C11.4 percentage neighbor 계획 보정 PR diff 적대 검토 30개

2026-10-24 계획 보정 PR의 실제 변경 파일을 원 계획 검토 20개 및 원 계획 PR diff 검토 30개와 분리해 검토했다. 각 항목은 규범 계산, 코드 후보 상태, 상태 대장·로드맵 집계, 사전 비교 자료의 서로 다른 실패 경로를 확인한다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 자동 반복 횟수와 auto-fit collapse 뒤 track 수를 혼동 | repeat count 4와 배치 후 collapsed track을 구분해 규범 기대를 기록했다. |
| 2 | 240px 축에서 percentage basis가 border box라고 가정 | 입력의 content axis를 규범 계산 기준으로 고정했다. |
| 3 | 24px prefix와 12px suffix를 repeated track에 합산 | 두 neighbor의 사용 크기를 각각 보존해 계산했다. |
| 4 | 첫 점유 track 앞 gutter까지 collapse | prefix와 점유 track 사이 gutter 하나는 남겨 200px을 계산했다. |
| 5 | 비어 있는 repeat 뒤의 gutter를 남김 | 나머지 빈 auto-fit track과 양쪽 gutter를 collapse했다. |
| 6 | gutter 제거 없이 200px을 계산 | `240 - 24 - 12 - 4 = 200px`으로 한 개 남는 gutter를 명시했다. |
| 7 | 동일 장면의 CSSOM 196px을 규범 기대값으로 사용 | Chrome 156의 196px을 호환성 관측으로 분리하고 규범 기대는 200px로 유지했다. |
| 8 | 기존 Rust 실패의 원인을 잘못 설명 | 코드 테스트가 여전히 196px을 기대한 상태에서 Taffy가 200px을 반환해 실패한 것으로 정확히 썼다. |
| 9 | 규범 기대값을 200px로 고친 테스트가 이미 통과했다고 오해 | 기대값 교정·validator 재실행은 아직 없다고 명시했다. |
| 10 | columns 관측을 rows에 복사 | row-axis scene과 관측값을 별도 입력으로 보존했다. |
| 11 | row percentage basis에 inline-size를 적용 | definite block-axis basis와 row 결과를 별도로 계산한다. |
| 12 | 시작 정렬 frame만으로 track의 끝 좌표를 추정 | 2px item의 DOM-only end-alignment 진단을 분리해 규범 236px과 Chrome 232px을 제시했다. |
| 13 | end-alignment 진단을 제품 지원 기능으로 해석 | 진단용 DOM 조작이며 앱 runtime 지원을 증명하지 않는다고 경계를 남겼다. |
| 14 | suffix 시작 좌표 238px로 gutter 개수를 판정 | 200px+한 gutter와 196px+두 gutter가 같은 좌표를 내므로 비판별 probe로 정정했다. |
| 15 | Chromium `main` 코드를 Chrome 156 원인 증명으로 일반화 | 바이너리 source revision이 연결되지 않았다고 적고 원인 단정을 피했다. |
| 16 | CSS Grid 기준 문서의 초안·시점을 혼동 | 2025 CRD의 고정 링크와 적용 section을 기록했다. |
| 17 | indefinite/cyclic axis에 percentage neighbor를 허용 | definite positive content-axis gate를 필수 조건으로 유지했다. |
| 18 | negative·non-finite percentage 입력의 fail-open | 기존 rejection test가 확인한 입력 경계를 보존했다. |
| 19 | `calc()` neighbor 또는 다른 track grammar를 이번 slice에 포함 | 계획에서 제외 범위를 유지했다. |
| 20 | 하나의 list 내 복수 auto-repeat 등 제한 범위를 확장 | 원 계획의 제한 grammar를 바꾸지 않았다. |
| 21 | Chrome observation JSON을 normative expected data로 재사용 | JSON에 independent expected values와 Chrome observations를 분리했다. |
| 22 | validator가 기대 track list만 검증하고 입력을 무시 | scene input, repeat count, track order, used sizes를 계산해 비교한다. |
| 23 | validator scene 수 또는 axis 누락 | 13개 scene 식별자와 row·column 자료를 확인해 13/13으로 검증했다. |
| 24 | fractional value 반올림이 4px 차이를 숨김 | end-alignment 4px 차이를 허용 오차로 흡수하지 않는다. |
| 25 | evidence hash가 새 진단 JSON·스크린샷을 빠뜨림 | precomparison `SHA256SUMS`를 갱신하고 13개 파일을 검증했다. |
| 26 | manifest가 자기 자신 hash를 요구 | `SHA256SUMS`는 manifest 자신을 제외한다. |
| 27 | checkout의 미커밋 코드 후보를 main 병합 코드로 서술 | `origin/main`·HEAD와 working-tree 후보를 분리하고 PR #577 병합 상태를 확인했다. |
| 28 | C11.4 제한 slice를 상위 완료로 승격 | C11.4 checkbox와 공식 72/213 집계를 유지했다. |
| 29 | C13의 미완료 기능을 완료 또는 활성 작업으로 오해 | C13 미완료·비활성 lane, scroll CSSOM 미구현, 일부 simulator/wheel 근거를 분리해 썼다. |
| 30 | Tailnet 화면을 runtime 또는 pixel 검증으로 확대 | Agent Browser 캡처는 문서 표시 근거로만 두고 Android/iOS runtime 미실행을 유지했다. |

결과: 계획 수용 oracle을 CSS Grid 독립 계산으로 바로잡고 Chrome 관측을 별도로 보존했다. 코드 테스트와 Android/iOS runtime은 기준 교정 뒤 아직 실행하지 않았다. 따라서 이 검토는 문서·계획 diff에 한정하며 기능 구현 완료를 뜻하지 않는다.
