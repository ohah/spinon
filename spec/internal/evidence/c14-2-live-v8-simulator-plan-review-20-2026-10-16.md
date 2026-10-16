# C14.2 live V8 simulator 계획 실패 관점 20개 — 2026-10-16

검토 대상은 [C14.2 live V8 simulator 검증 계획](../../../plan/c14-2-live-v8-simulator-verification.md)과 기존 Chrome 155 fixture/oracle 참조다. 구현 시작 전 검토이며 구현 실패 검토와 PR 변경 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 계획의 차단 기준 |
|---:|---|---|
| 1 | inventory의 전체 27행을 PNG 지원 분모로 오인해 SVG와 실패 대조까지 양성 통과로 센다 | Block 21 PNG + Flex 8개만 29개 양성 집합으로 고정하고 나머지 행의 이유를 분리했다. |
| 2 | case ID와 DOM node 순서가 어긋나 한 이미지 결과를 다른 oracle 행과 비교한다 | 각 case ID·순서·img request identity를 oracle과 일대일로 검증한다. |
| 3 | fixture가 PNG 파일은 사용하지만 V8 document는 synthetic HostDocument로 만들어 live V8 경로를 우회한다 | 모든 root/container/img는 RuntimeSession::eval으로 만든 연결 문서여야 한다. |
| 4 | img src가 달라도 고정 PNG bytes를 decode해 현재 리소스인 것처럼 처리한다 | source/document stamp와 함께 각 ImageRequest의 exact src를 고정 URL과 비교한다. |
| 5 | 디코드 PNG의 natural size 또는 hash가 바뀌어도 24×16 기대값만 강제한다 | PNG bytes SHA-256, natural 24×16, request/resource/layout source를 함께 확인한다. |
| 6 | CSS px 결과에 Android density나 iOS scale을 곱해 비교값이 왜곡된다 | DPR 1 Chrome oracle의 CSS px를 비교 단위로 고정하고 simulator scale을 적용하지 않는다. |
| 7 | case 결과 크기만 비교해 부모 containing block의 잘못된 basis를 감춘다 | Block image full frame을 비교하고 Flex parent와 image frame을 모두 비교한다. |
| 8 | max-width 20px의 subpixel Chrome 결과를 정수 반올림해 회귀를 통과시킨다 | 원시 oracle `19.984375×13.328125`를 report에 보존하고 반올림 전 값을 비교한다. 0.05 CSS px tolerance 안의 차이는 의도적으로 허용하며 20px 결과도 통과할 수 있다고 계획에 명시했다. |
| 9 | CSS 2.2 min/max 10개 조합이 하나 이상의 분기를 누락한다 | 21개 원본 PNG oracle 전체를 분모로 삼고 각 행 ID를 report에 보존한다. |
| 10 | authored numeric aspect-ratio와 natural ratio를 혼동한다 | authored ratio, border-box ratio, aspect-ratio:auto를 별도 oracle case로 유지한다. |
| 11 | border-box padding/border가 자연 content ratio에 두 번 반영된다 | box-sizing edge case의 full frame을 Chrome oracle과 대조한다. |
| 12 | Flex measure pass와 final pass가 서로 다른 cross-axis sizing을 사용한다 | 8개 Flex oracle에서 parent와 image frame을 독립 비교한다. |
| 13 | definite cross-axis stretch를 natural ratio transfer로 오인하거나 max-height를 무시한다 | stretch, max-height, no-stretch 3개 경계를 Flex 전체 행렬에 포함한다. |
| 14 | label text가 C15 미완료 텍스트 측정을 fixture geometry에 섞는다 | V8 DOM은 text-free로 만들고 결과는 app report에만 표시한다. |
| 15 | StylesheetSource가 실제 고정 CSS와 달라 oracle이 다른 조건을 측정한다 | 기존 fixture 선언을 pinned stylesheet로 전달하고 viewport·origin 조건을 기록한다. |
| 16 | 29개 completion이 다른 document revision의 image를 섞거나 style/environment change를 source identity와 혼동한다 | request별 generation/document/render stamp와 resource/layout source를 검사하고 StyleRevision, EnvironmentRevision, IntrinsicSizeRevision을 별도로 기록한다. |
| 17 | 동일 src의 29개 image 요청이 NodeId 충돌로 한 image metadata를 덮어쓴다 | completion node identity와 개수/중복을 검사하고 source snapshot에 모두 존재해야 한다. |
| 18 | 앱 기본 산출물에 fixture symbol이나 고정 JS·CSS·PNG가 들어간다 | Android/iOS default negative control과 opt-in positive control을 각각 확인한다. |
| 19 | fixture route의 잘못된 build flag나 동시 route 요청이 다른 fixture를 호출한다 | strict 0/1 build flag, 전용 launch argument, route collision error를 계획에 포함했다. |
| 20 | report가 화면 또는 FFI capacity에서 잘려도 status=ok와 29/29를 남긴다 | 전체 case count·status를 fail-closed하고 원본 report/log와 화면을 함께 보존한다. |

**판정:** 구현 전 oracle 범위·단위·case identity·artifact 격리 기준이 분리됐다. 발견된 계획 P1/P2 결함은 없다. 구현 중 실제 cascade/profile이 고정 조건을 표현할 수 있는지 다시 확인한다.
