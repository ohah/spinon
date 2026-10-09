# C04 Runtime CSS→Taffy 계획 PR 적대 검토

**검토 기준:** 이 브랜치의 plan·status·conformance·OTA placeholder·Chromium inventory·fixture·capture tool·reference JSON 변경 전체. 제품 Stylo/Taffy implementation은 포함하지 않는다.

| # | 변경 집합 공격 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 미출시 crate 버전이 오르는지 | `Cargo.toml`, package manifests, lockfiles는 바뀌지 않았다. 내부 숫자 버전 규칙은 `0.1.0` 고정을 유지한다. |
| 2 | 기능 ID가 출시 버전처럼 보이는지 | `C04`, `C04.8`은 로드맵 ID라고 계획이 분리해 말하고 새 SemVer를 넣지 않았다. |
| 3 | OTA 예제가 미확정 schema를 확정 버전처럼 보이는지 | `1.0.0-draft`를 `TBD`로 바꾸고 실제 version 형식 미정임을 문장으로 밝혔다. |
| 4 | OTA placeholder가 이미 유효 manifest처럼 제시되는지 | 문서는 예시가 자리표시자임을 유지하고 구현·정식 schema 완료를 주장하지 않는다. |
| 5 | 상태 대장이 C04.8을 계속 구현 전이라고 하는지 | 완료된 runtime UA 계산으로 고쳤고 남은 Taffy/GPU 경계를 따로 적었다. |
| 6 | CSS checklist 요약이 C04.7/4.8을 누락하는지 | 구현된 항목을 .5부터 .8까지 반영하고 제품 runtime CSS→layout/GPU를 미완료로 유지했다. |
| 7 | 미뤄 둔 외부 URL 로더가 완료 처리되는지 | 기존 미구현 하위 항목은 그대로 남아 있다. |
| 8 | 계획이 구현 전에 oracle을 만들도록 하는지 | Chromium reference를 코드 변경보다 먼저 수집하는 순서와 `spec/0001-conformance.md` 링크가 있다. |
| 9 | 비교 fixture가 작성 목표와 다른 CSS를 관찰하는지 | HTML은 JS에서 재현할 inline style, 단일 root, Flex·Block·`display:none` 다섯 노드를 포함한다. |
| 10 | fixture 환경이 page 기본 여백 때문에 좌표를 오염시키는지 | HTML oracle은 body/html margin을 0으로 고정하고 앱 root frame은 원점부터 비교한다. |
| 11 | inventory가 layout property를 빠뜨리는지 | 기존 7 UA 속성과 layout box/Flex/margin/padding 값 27개를 노드별로 고정했다. |
| 12 | oracle이 다른 Chrome 실행 파일로 바뀌어도 같은 ID를 쓰는지 | 도구가 Chrome `154.0.8037.98`, Chromium revision, 실행 파일 SHA-256을 모두 확인한다. |
| 13 | locale·timezone·미디어 차이를 숨기는지 | CDP에서 locale·UTC·light·reduced-motion·forced-colors를 설정하고 반환 환경도 검사한다. |
| 14 | 빠진 property 관찰을 빈 문자열로 통과시키는지 | capture tool은 모든 inventory property가 존재하고 nonempty string인지 확인한다. |
| 15 | inventory의 중복 노드나 property가 reference를 흐리는지 | capture tool은 노드 ID와 property ID 중복을 거부하고 node 수를 고정 확인한다. |
| 16 | Chromium 시작 실패·CDP timeout 뒤 임시 자원이 남는지 | temp profile을 `finally`에서 지우고 해당 실행이 만든 process group만 종료한다. |
| 17 | 개발자 Chrome이나 기존 profile을 종료하는지 | 매번 자체 temp user-data-dir과 detached child process group을 쓰며 전역 process 탐색·종료를 하지 않는다. |
| 18 | capture가 기존 reference를 조용히 교체하는지 | reference ID에 input/tool/binary hash가 들어가며 같은 path는 `wx`로 거부한다. |
| 19 | capture 도구가 Python·Playwright를 설치하거나 외부 서버에 의존하는지 | 고정 Node 24 환경과 내장 WebSocket/CDP만 쓰며 fixture는 로컬 파일이고 외부 요청을 넣지 않았다. |
| 20 | baseline을 제품 검증으로 오해하는지 | precomparison evidence는 Chromium 결과만 기록하고 Spinon 구현·Taffy·플랫폼 성공을 명시적으로 부정한다. |

검토 중 capture tool의 빈 computed 값 허용 경로를 찾아 거부하도록 고쳤다. 첫 capture가 사용한 이전 도구 hash의 생성물은 최종 reference에 남기지 않고, 수정된 도구 hash로 새 immutable reference를 만들었다. 구현 코드 변경과 Android/iOS 실행은 이 PR 범위에 포함하지 않는다.
