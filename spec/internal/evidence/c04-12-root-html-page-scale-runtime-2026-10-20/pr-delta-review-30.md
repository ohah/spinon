# C04.12 기본 root HTML page-scale runtime PR diff 적대 검토 30개

- **대상 diff:** branch `feat/c04-12-root-html-page-scale-runtime-20261020`의 Rust runtime, Android/iOS C0410 route와 resource, 계약·상태·계획 문서, simulator evidence, 두 개의 20관점 검토 기록.
- **판정:** 아래 항목을 각각 독립 경계로 대조했다. 각 결과는 현재 worktree diff, focused tests, simulator logs/screenshots 또는 bundle build로 확인했다.

| # | 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 새 mapping 경로가 fixture-only feature를 요구해 제품 API 경로에서 빠지는가 | `c04-runtime-gpu`의 기존 product route에 적용하고 `dep:spinon-core`만 기능 의존성으로 선언한다. product feature build 통과. |
| 2 | `EnvironmentRevision`이나 snapshot 타입이 feature-off product build에서 resolve되지 않는가 | Cargo feature에 `spinon-core` 직접 의존성을 추가했다. `cargo build -p spinon-ffi --features c04-runtime-gpu --lib` 통과. |
| 3 | 기본 HTML GPU host 외 일반 runtime GPU host까지 변환되는가 | `root_html_default_render`가 false면 helper가 mapping과 상관없이 원본 scene을 반환한다. invalid mapping을 넘긴 legacy test 통과. |
| 4 | metadata 값을 환경 snapshot 대신 native code에서 재계산하는가 | mapping factory는 `RootHtmlEnvironmentSnapshot` 필드만 읽는다. helper와 두 simulator의 reports 대조 통과. |
| 5 | scene completion key와 environment mapping revision이 어긋난 채 scene을 게시하는가 | revision mismatch는 `ERR_STALE`이며 test가 해당 status/detail을 검증한다. |
| 6 | scene가 root HTML profile인데 mapping이 준비되지 않아 scale 1로 조용히 렌더되는가 | missing mapping은 구체 `ERR_LAYOUT`로 변환 전에 종료한다. focused test 통과. |
| 7 | 별도 fixture나 이전 layer가 붙인 transform 위에 scale이 중복되는가 | 기존 `page_scale_transform()`이 있으면 거부한다. duplicate guard test 통과. |
| 8 | mapping은 양수 입력이어도 물리 scale 곱셈이 infinity가 되는가 | `RuntimePageScaleTransform::new` 재사용으로 overflow를 거부하고 boundary test 통과. |
| 9 | 물리 scale 곱셈이 0으로 underflow해 잘못된 identity scene을 만드는가 | `MIN_POSITIVE` 입력 쌍을 거부하는 추가 test 통과. |
| 10 | 새 environment 요청 도중 이전 map이 남아 최신 실패 scene에 재사용되는가 | default host가 environment 작업 전에 Canvas와 map을 비운다. 검증/resolve 오류에서는 map을 다시 넣지 않는다. 코드 검사 통과. |
| 11 | Canvas 색 선택 실패 뒤 page-scale map만 새 revision에 저장되는가 | mapping 검증은 하되 Canvas resolve 성공 뒤 Canvas와 mapping을 commit한다. resolve `?` 이전에는 map write가 없다. |
| 12 | layout worker가 끝나기 전에 새 map이 이전 scene에 붙는가 | scene revision 대조가 map 적용 전 수행되고, 실패는 publication 전에 반환한다. stale test 통과. |
| 13 | image·glyph·scroll attachment가 scale transform을 제거하는가 | transform 적용 지점은 해당 attachment 블록 이후다. source 순서 대조 통과. |
| 14 | transform 오류 또는 complete-key 불일치인데 `PresentationScenes::publish`가 호출되는가 | transform과 key 검사가 publish보다 먼저 실패한다. publish 순서 코드 검사 통과. |
| 15 | environment update가 실패했는데 과거 장면이 새 revision 성공처럼 보이는가 | update 시작 때 presentation sequence와 mapping을 무효화한다. 최신 sequence만 publish되는 existing presentation test 및 simulator reports 통과. |
| 16 | HTML scene 변환이 CSS layout frame을 물리 pixel로 덮어쓰는가 | transform은 `RuntimeRenderSnapshot`의 mapping만 설정한다. CSS viewport/frame reports는 기존 값으로 남고 screenshots의 surface size와 분리된다. |
| 17 | CSS hit-test 좌표를 새 물리 scale로 조용히 재해석한다고 문서에 쓰는가 | code에서 hit-test 입력은 바꾸지 않았고 계약은 input remapping을 미지원으로 둔다. 문서 경계 확인 통과. |
| 18 | Canvas clear도 scale transform을 따라 작아져 물리 surface 밖을 비우지 못하는가 | renderer clear는 existing clear path를 사용한다. 6 screenshot의 full surface mask와 `(10,10)` exact RGB sample 통과. |
| 19 | DPR 2.625와 1.3125 fractional edge에서 색 box가 한 pixel 밀리는가 | Android 캡처의 half/fixed/scale-two probe bounds가 pixel-center coverage 예상과 일치한다. |
| 20 | scale 2에서 CSS 밖 green probe가 물리 표면 안에 잘못 보이는가 | Android/iOS 캡처에서 green exact-color mask pixel 수가 0이고 magenta probe bounds는 예상 위치다. |
| 21 | `initial-scale=1` default case가 새 fixture로 바뀌어 기존 default rendering을 검증하지 못하는가 | default route는 기존 `root-html-default-rendering.html`을 선택하고 scale arg가 없을 때 기존 source hash가 기록된다. 앱 default screenshots/build 통과. |
| 22 | width=500 case가 실제 viewport meta를 적용하지 않고 case label만 바꾸는가 | 두 플랫폼은 viewport string 정확 한 개 대체를 요구하고, report는 layout 500×375/visual 320×240이다. 화면 bounds도 DPR 3/2.625에 맞는다. |
| 23 | Android launcher가 새 string extra를 받아도 C0410 demo route로 진입하지 않는가 | `MainActivity`의 C0410 route selector가 non-null page-scale case extra를 포함한다. API 37 AVD source hash와 environment logs가 실행을 확인한다. |
| 24 | Android가 잘못된 scale case나 다른 C0410 route와의 충돌을 허용하는가 | `half/double/fixed-width` allowlist와 conflicting extras 검사에 새 key만 예외로 둔다. source inspection과 scale simulator run 통과. |
| 25 | iOS AppDelegate가 page-scale argument를 C0410 runtime GPU controller로 보내지 않는가 | `runRuntimeGpuBase` predicate가 해당 argument prefix를 포함한다. iOS 26.2 app 실행 logs가 해당 route를 확인한다. |
| 26 | iOS가 여러 page-scale arguments 중 첫 번째만 조용히 선택하는가 | argument count가 1 초과면 host init에서 거부한다. case value도 세 지원 값만 허용한다. |
| 27 | iOS resource reference는 있으나 app bundle Resources phase에서 빠지는가 | PBXFileReference, PBXBuildFile, group, resources build phase를 모두 추가했고 `build:ios-sim` 성공. |
| 28 | 두 OS source hash가 platform encoding/API 차이 때문에 달라지는가 | `Data(source.utf8)`와 `getBytes(UTF_8)`로 SHA-256을 산출한다. base asset 3개는 동일 SHA, 4 runtime inputs는 양 플랫폼 로그 hash가 일치한다. |
| 29 | spec update가 C04.12 상위 항목이나 완료 수를 앞당겨 표시하는가 | `spec/STATUS.md`는 새 하위 항목을 검토 대기 상태로 두고 parent checkbox와 완료 수를 유지한다. |
| 30 | 증거 파일이 UI screenshot만 담고 input·scale·revision·build 한계를 잃는가 | evidence 문서는 source hashes, viewport, physical scale, pixel bounds, stale iOS request, build/test 결과와 simulator-only 한계를 함께 기록한다. plan/implementation review/PR review 경로도 상대 링크로 연결된다. |

## 판정

이 diff 검토에서 새 동작 결함은 발견되지 않았다. plan review 20개 및 구현 review 20개와 별도인 PR diff 관점 30개를 모두 기록했다. 외부 PR의 required checks와 merge 조건은 실제 PR을 연 뒤 다시 조회한다.
