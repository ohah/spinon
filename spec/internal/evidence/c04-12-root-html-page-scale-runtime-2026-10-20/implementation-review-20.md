# C04.12 기본 root HTML page-scale runtime 구현 실패 관점 검토 20개

- **대상:** `feat/c04-12-root-html-page-scale-runtime-20261020`의 Rust host 연결, Android/iOS C0410 경로, 동일 HTML fixture, 계약·상태 문서와 simulator 결과.
- **범위:** 계획 검토 20개와 구별되는 구현 후 검토다. 코드는 현재 diff와 focused tests로, 화면은 [pixel 분석](./simulator-pixel-analysis.md) 및 프로세스별 실행 로그로 확인했다.

| # | 공격 관점 | 확인 결과와 판정 |
| ---: | --- | --- |
| 1 | metadata를 native 경로에서 다시 파싱해 런타임과 브라우저가 다른 값을 선택하는가 | `RootHtmlPageScaleMapping::from_environment`가 `RootHtmlEnvironmentSnapshot`의 DPR·page scale·revision을 그대로 받는다. 별도 metadata parser가 없어 통과. |
| 2 | 화면 physical pixel 값이 CSS viewport로 잘못 쓰여 DPR이 두 번 반영되는가 | CSS layout/visual viewport는 environment report 그대로고 mapping만 `DPR × page_scale`을 쓴다. Android surface 840×630, iOS 960×720과 보고된 CSS viewport가 분리돼 통과. |
| 3 | `initial-scale`이 layout frame 크기를 직접 바꾸는가 | scale 0.5·2에서 layout/visual viewport의 기존 report와 root CSS frame을 따로 보존한다. transform은 completed scene 이후 적용해 통과. |
| 4 | fixed width 500이 screen width 320으로 축소되거나 page scale에 한 번 더 나뉘는가 | 양 플랫폼 report가 layout 500×375, visual 320×240, page scale 1을 기록하고 pixel bounds가 3×/2.625×로 맞아 통과. |
| 5 | environment revision이 다른 장면에 새 mapping을 붙이는가 | scene key의 `EnvironmentRevision`을 mapping과 비교하고 다르면 `ERR_STALE`을 반환한다. stale revision unit test 통과. |
| 6 | root HTML host mapping이 없는 초기 상태에서 unscaled scene이 게시되는가 | root HTML 기본 host는 mapping이 없으면 `ERR_LAYOUT`; 변환 전에 반환하므로 scene publish에 도달하지 않아 통과. missing mapping test 통과. |
| 7 | 이미 page-scale transform이 있는 scene에 중복 곱셈이 발생하는가 | 기존 transform을 감지해 `ERR_LAYOUT`으로 거부한다. duplicate-transform test 통과. |
| 8 | 새 environment 요청이 이전 DPR/page scale을 재사용하는가 | root HTML default host는 요청 초기에 Canvas와 scale mapping을 비우고, 새 snapshot 및 Canvas resolve 성공 뒤에만 교체한다. 코드 순서와 모든 새 화면의 새 HTML source hash를 확인해 통과. |
| 9 | Canvas resolve 실패 후 기존 scene mapping이 새 revision으로 남는가 | mapping을 먼저 비운 뒤 Canvas resolve를 실행한다. resolve 실패는 `?`로 반환하고 mapping을 다시 쓰지 않아 실패 장면을 게시하지 않는다. 코드 검사 통과. |
| 10 | image/glyph attachment 순서가 transform을 덮거나 유실시키는가 | scale 함수는 scroll·image·glyph attachment 뒤, revision 재검사와 presentation publish 전에 실행된다. 코드 순서 확인 통과. |
| 11 | mapping 오류 뒤 partial scene이 게시되는가 | transform 오류와 complete-key 검사가 `presentation.publish`보다 앞선다. errors use early return; publication path를 통과하지 않아 통과. |
| 12 | `DPR × page_scale` overflow가 무한 물리 좌표를 만든다 | transform constructor 검증에서 거부하며 focused overflow test 통과. |
| 13 | 양수인 작은 입력의 곱이 0으로 underflow해 identity처럼 오인되는가 | 검토에서 기존 사례 누락을 찾아 `MIN_POSITIVE × MIN_POSITIVE` rejection test를 추가했다. 현재 5 focused tests 통과. |
| 14 | JavaScript-only/기존 GPU host에 HTML mapping이 실수로 적용되는가 | root HTML route가 아니면 helper가 mapping을 읽기 전에 원본 scene을 반환한다. 잘못된 최대 입력 mapping도 legacy scene에 적용하지 않는 test로 확인. |
| 15 | `initial-scale=1`에서 default 앱 화면이 이동·확대되는가 | Android/iOS default run은 DPR만 physical scale로 기록하고 기존 기본 route asset hash 및 전체 앱 화면을 유지한다. 통과. |
| 16 | 확대된 scene이 물리 surface 바깥까지 보이는가 | scale 2의 green edge control은 Android/iOS 캡처 모두 surface 밖에 있어 나타나지 않고 magenta box는 예상 구간에서 clip된다. 통과. |
| 17 | transform이 native Canvas clear나 root background를 축소하는가 | 여섯 scale 캡처에서 CSS `(10,10)` 표본은 전부 정확 RGB `(17,34,51)`이고 surface mask가 전체 물리 크기다. 통과. |
| 18 | Android/iOS fixture bytes가 달라 oracle 비교가 무효인가 | test, Android asset, iOS resource가 모두 613 bytes와 SHA-256 `3516b552a76777632679ca1554ccceb14c1ac52548f7ddd264a67689fd744829`로 일치한다. 통과. |
| 19 | 잘못된/충돌하는 route argument가 기본 앱 또는 다른 CSS fixture를 조용히 선택하는가 | 양 플랫폼이 허용 scale case 세 개, root HTML route 요구와 기타 C0410 route 충돌을 검증한다. 기본 launch는 기존 asset을 사용한다. native source inspection 및 simulator 실행 통과. |
| 20 | iOS 첫 drawable 교체 중 stale 오류를 성공으로 감추거나 stale draw를 내보내는가 | per-process log에 surface generation 1 구성 중 첫 요청 `-12` 취소와 sequence 3의 `status=0` 최신 environment/draw가 모두 남는다. 최종 캡처만 성공 보고로 꾸미지 않고 두 결과를 함께 보존해 통과. |

## 결과

실패 관점에서 찾은 보완점은 두 가지였다. physical scale 곱셈의 underflow test가 없어 추가했고, 일반 GPU host 경계 test가 `None` mapping만 확인해 잘못된 mapping도 전달하는 경우를 추가했다. 동작 결함은 발견되지 않았다. C0410 fixture harness의 기존 화면 초기화가 iOS 첫 surface configure와 겹쳐 stale `-12` 한 번을 만들지만 latest-wins 계약에 따라 최신 environment와 draw는 성공한다. 이 취소는 실행 기록과 계약에 명시했다.
