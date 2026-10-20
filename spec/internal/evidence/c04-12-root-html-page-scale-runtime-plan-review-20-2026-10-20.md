# C04.12 기본 root HTML runtime page-scale 계획 검토 20개

구현 전 계획 [`c04-12-root-html-page-scale-runtime.md`](../../../plan/c04-12-root-html-page-scale-runtime.md)을 서로 다른 실패 경로와 계약 경계 20개로 검토했다. 기준은 현재 `main`의 `RootHtmlEnvironmentSnapshot`, `RuntimeRenderSnapshot::with_page_scale_transform()`, 기존 page-scale Chrome/WGPU evidence다. 이 기록은 코드 구현 검증을 대신하지 않는다.

| # | 독립 실패 관점 | 계획 검토와 조치 |
| ---: | --- | --- |
| 1 | `page_scale`이 surface 환경보다 앞선 임의 값에서 오는가 | 계획은 root HTML parser와 `set_root_html_environment()` snapshot만 입력으로 지정한다. 다른 source를 허용하지 않는다. 통과. |
| 2 | DPR과 page scale을 하나의 값으로 미리 덮어써 CSS viewport에 넣는가 | CSS layout viewport와 DPR/page scale을 별도 값으로 유지하고 mapping에서만 곱하도록 했다. 통과. |
| 3 | 새 환경 revision이 이전 scale mapping을 재사용하는가 | 새 환경 입력 때 기존 mapping을 무효화하고 render key와 environment revision 일치 후에만 적용하도록 명시했다. 통과. |
| 4 | 최신 layout key와 scale 값이 서로 다른 document/environment에서 오는가 | metadata/source 변경은 별도 scope 밖이며, environment revision이 render key와 일치하지 않으면 게시 거부한다. 통과. |
| 5 | JavaScript-only 또는 root HTML 외 GPU host까지 확대 적용되는가 | mapping host/profile 경계를 root HTML GPU route로 제한하고 legacy 경로 regression을 요구한다. 통과. |
| 6 | page scale이 native Canvas clear나 system UI까지 확대하는가 | Canvas clear는 전체 물리 surface에서 기존 동작을 유지하고 system UI는 제외했다. 통과. |
| 7 | initial scale이 media query viewport를 visual viewport로 바꾸는가 | media/layout은 CSS layout viewport를 계속 사용한다고 고정했다. 통과. |
| 8 | `RuntimeRenderSnapshot`의 box frame이나 CSS hit-test 좌표가 물리값으로 변환되는가 | scene의 WGPU 투영만 mapping하고 layout frame·hit-test 좌표는 CSS px로 유지한다. 통과. |
| 9 | 적용 시점이 paint scene 생성 전이라 text/layout 좌표가 변하는가 | completed CSS scene 이후 presentation 직전에 mapping하도록 지정했다. 통과. |
| 10 | images/glyphs/paint command 추가 후 transform이 사라지는가 | 모든 image attachment가 끝난 완성 scene에 한 번 적용하고 scene 재발행 경계를 검사하도록 했다. 통과. |
| 11 | renderer가 DPR을 이미 적용한 뒤 DPR을 두 번 곱하는가 | 기존 WGPU `RuntimePageScaleTransform` 경로를 재사용하고 DPR×scale geometry fixture와 대조한다. 통과. |
| 12 | transform 부재의 legacy surface-fit scene이 달라지는가 | mapping 없는 기존 renderer snapshot을 regression control로 고정했다. 통과. |
| 13 | 잘못된 0·음수·NaN·무한 배율이나 곱셈 overflow가 silent fallback 되는가 | mapping 생성 오류로 닫고 새 장면을 게시하지 않도록 요구했다. 통과. |
| 14 | 환경 갱신 timeout/실패 뒤 stale scene이 최신 성공으로 남는가 | environment revision mismatch와 기존 publication invalidation 후 실패 원자성을 검증하도록 했다. 통과. |
| 15 | Android 분수 physical scale의 픽셀 경계를 정수 oracle로 오판하는가 | iPhone 12 Chrome oracle과 AVD DPR `2.625`의 pixel-center/최대 1 px 경계 기준을 분리했다. 통과. |
| 16 | iOS Simulator의 iPhone12 oracle surface와 실제 iPhone17Pro surface를 같은 입력으로 취급하는가 | Chrome geometry는 기존 iPhone12 oracle, app 실행은 iPhone 17 Pro의 실제 DPR 및 고정 `320×240` CSS surface로 구분한다. 통과. |
| 17 | OS light/dark 변경이나 surface resize 뒤 scene이 예전 scale로 보이는가 | 양 플랫폼 case에 actual scene revision과 Canvas clear 표본을 함께 연결하도록 했다. 통과. |
| 18 | 테스트가 offscreen WGPU readback만으로 actual root HTML route를 주장하는가 | 세 metadata fixture를 실제 Android/iOS app GPU surface에서 실행하도록 계획했다. 통과. |
| 19 | native test가 일반 제품 FFI host와 다른 fixture-only C ABI를 쓰는가 | debug route도 일반 root HTML GPU host 생성·환경 호출을 사용한다고 고정했다. 통과. |
| 20 | page-scale renderer 성공을 hit-test/pinch/scroll/browser 호환성으로 과장하는가 | input mapping, pinch/pan/scroll, text rerasterization과 전체 C04.12 완료를 제외 범위에 명시했다. 통과. |

## 판정

20개 관점에서 입력 source, revision freshness, renderer·Canvas 경계, legacy 동작, 기기별 oracle 범위와 실제 앱 검증 경로를 확인했다. 계획 단계 blocker는 발견하지 않았다. 코드 변경은 이 review 완료 뒤 시작한다.

## 구현 시작 후 계획 전제 보정

코드 경로를 다시 대조하다 기존 표가 Chrome iPhone 12의 `390×844` viewport와 C0410 test host의 `320×240` CSS surface를 혼동한 것을 발견했다. 구현 전 검토는 기기별 scale 분리를 다뤘지만 앱 test host의 고정 viewport dimension을 확인하지 못했다. 위 계획을 실제 host 입력에 맞춰 갱신하고, 아래 추가 관점으로 보정사항을 적대적으로 재검토했다.

| # | 독립 실패 관점 | 보정 검토와 조치 |
| ---: | --- | --- |
| 21 | full-device Chrome viewport를 고정 C0410 surface viewport인 것처럼 기록하는가 | 앱 runtime expected viewport를 `320×240`으로 고치고 기존 iPhone 12 oracle은 브라우저 동작/선형 mapping 근거로 한정했다. 통과. |
| 22 | AVD logical density와 physical resolution을 혼동해 DPR 및 expected bounds를 잘못 계산하는가 | 활성 API 37 AVD의 `1080×2400`, density `420`을 재조회해 DPR `2.625`를 사용하며 iOS scale은 실행 log로 확인한다. 통과. |
| 23 | 확대된 fixture box는 확인하지만 surface 밖 control과 전체 Canvas clear는 놓치는가 | scene에 두 번째 좌표 control을 두고 simulator screenshot에서 edge clipping과 `(10,10)` clear sample을 확인하도록 했다. 통과. |
| 24 | 현재 실행 중인 iPhone 17 Pro의 기존 검증 앱을 종료해 증거를 덮어쓰는가 | 활성 iPhone 17 Pro는 보존하고 같은 iOS 26.2 runtime의 별도 iPhone 17 Pro Max에서 검증한다. 통과. |
