# C11.1 Root HTML intrinsic Grid tracks 구현/runtime 실패 관점 검토 20개

계획의 20개 검토와 분리해 실제 변경 코드, fixed fixture, focused Rust 결과와 두 simulator 로그를 검토했다. 각 항목은 런타임 경계나 값 전달 경로가 다르다.

1. **profile 밖 전파** — intrinsic breadth를 허용하는 cascade flag가 parsed Root HTML profile에만 켜지는지 확인했다. `element.rs`에서 해당 flag는 `RuntimeRootHtmlTextPaintV1`에만 연결되고 다른 profile projection은 22개 Root HTML profile 검사에서 기존 contract를 유지한다.
2. **typed value 손실** — `auto`/`min-content`/`max-content`가 serialized CSS 문자열 재파싱 없이 typed `GridTrackSize`로 전달되는지 대조했다. 양 축 projection matrix는 22/22 통과했다.
3. **minmax 방향·문법 경계** — minimum과 maximum을 뒤집거나 `<flex>` minimum을 허용하지 않는지 확인했다. CSS-valid minima 5종 × maxima 6종은 typed value를 보존하고, flex minimum 및 non-finite/negative/out-of-scope shapes는 node/property 오류로 거부한다.
4. **integer repeat 확장** — intrinsic `minmax()`를 fixed `repeat(2, …)` 안에서 expanded tracks로 연결하는 fixture scene을 확인했다. 기존 10,000-track 및 line-metadata budget 검사보다 먼저 무제한 reserve를 하지 않고, 반복 line groups도 track 개수에 맞춰 생성한다.
5. **후행 값 preflight 누락** — 첫 intrinsic track 뒤의 큰 fixed/percentage breadth가 aggregate validation을 건너뛰지 않는지 확인했다. `3e38px` overflow mutation은 `grid-template-columns` 진단으로 실패했고 partial publish가 없었다.
6. **definite basis 혼동** — `%`/`fr` maximum이 definite grid axis와 fixed container에 도달하는지 확인했다. fixture의 `minmax(min-content, 1fr)`은 독립 expected에 포함되고 `height:auto` 변이는 layout 전에 `height` 진단으로 거부된다.
7. **unsupported container 스타일 누출** — intrinsic route에서 auto/percentage/zero-size axes, non-zero gap, box edges, writing direction 및 unsupported alignment가 일반 Taffy fallback으로 통과하지 않는지 확인했다. 현재 지원 계약은 두 축이 positive fixed CSS px이고 gap/edge가 0인 경우로 제한한다.
8. **text measurement route 우회** — 앱 전용 Grid 성공 fixture가 아니라 parsed Root HTML default route에서 17 text runs를 만들고 같은 document layout/paint로 연결하는지 확인했다. runtime test는 17 scene의 Grid/item frames와 published render boxes를 검사한다.
9. **min-content line opportunity 오류** — `alpha beta`의 breakable run이 min-content breadth에 반영되는지 독립 expected와 Chrome 관측을 대조했다. Root HTML runtime font backend의 `min-content` item은 30.004883px, Chrome 156은 30.1094px여서 integrated text frame에 `0.5 CSS px` tolerance를 사용한다. font-independent direct layout trace matrix는 20px box contribution과 `auto`/`minmax()` 결과를 `0.02 CSS px` 안에서 확인한다.
10. **minmax floor 역전** — max-content가 `80px` minimum보다 작을 때 track이 floor 아래로 내려가지 않는지 `minmax-floor` scene에서 확인했다. 독립 expected와 runtime frame을 함께 비교했다.
11. **fraction free-space 오배분** — `minmax(min-content, 1fr)`의 content base 이후 여유 공간 배분을 `minmax-intrinsic-flex` scene과 font-independent layout trace test로 각각 확인했다. 160px definite basis에서 flex track은 160px이 되고 후행 fixed/percentage overflow preflight도 별도 mutation에서 실행된다.
12. **auto stretch와 normal 정렬 합침** — `auto` max sizing function의 default stretch 및 명시 `justify-content:start`의 분배 차이를 `auto-stretch` 대조 scene과 분리했다. Taffy adapter는 intrinsic mode의 `normal`을 stretch 동작으로 보존한다.
13. **row axis 회귀/columns 기대값 재사용** — rows의 min-/max-content scene은 fixed 160px columns control과 별도 frame expectation을 사용한다. `min-content-rows`/`max-content-rows`가 같은 17-scene runtime matrix에 있다.
14. **span contribution 누락** — two-column span이 owning grid의 track contribution까지 전달되는지 `span-two-intrinsic` scene으로 확인했다. route는 intrinsic grid container만 처리하며 mixed auto와 one-axis placement의 허용 조합은 별도로 막는다.
15. **hidden item이 sizing에 참여** — hidden second item의 긴 unbreakable text가 contribution/frame/paint를 만들지 않는지 확인했다. runtime test는 `display:none` node에 frame과 render box가 없음을 검사한다.
16. **max-content overflow clamp** — 20px container보다 넓은 `max-content-overflow` item의 frame을 expected에 유지해 root/container bounds에 clamp하지 않는지 검사했다. 캡처는 전체 viewport screenshot이라 pixel readback 증거로 확대하지 않는다.
17. **implicit-track fallback** — `grid-auto-rows:4px` 변이가 intrinsic explicit mode에 섞여 implicit row로 계산되지 않는지 검사했다. RuntimeSession은 `grid-auto-rows`를 가리켜 거부하고 completed scene을 publish하지 않는다.
18. **layout error의 부분 성공** — auto/percentage/zero-size axis, gap, margin, padding, border 변이 9개와 implicit row, aggregate overflow를 각각 적용했다. 열한 failure cases 모두 partial layout/render를 publish하지 않고 node/property 진단으로 실패하며 cascade는 ready 상태를 유지한다.
19. **stale revision 또는 scene 일부 혼합** — 17 grids가 든 하나의 Root HTML document에서 completed layout key와 UA cascade key가 같은지 확인했다. runtime assertion이 key equality를 요구하고 성공 시 54 boxes를 한 snapshot으로 낸다.
20. **simulator evidence를 제품 표면으로 오인** — Android APK와 iOS app bundle의 fixture source hash는 동일하고 앱은 기존 C04.12 Root HTML DefaultRender route를 썼다. Android/iOS 모두 `layout=ready boxes=54`, text 17, draw/present를 기록했다. PixelCopy/Metal drawable readback은 없으며 Android는 ANGLE/SwiftShader다. 제품 `c04-runtime-gpu` check 및 fixture boundary test 33/33은 통과했고, 새 C11 FFI selector는 없다.

## 판정과 남은 suite 실패

Intrinsic profile focused runtime 4/4, style projection 22/22, direct-text aspect-ratio rejection 2/2, scoped Clippy, product-feature `cargo check`, fixture-boundary 33/33 및 rustfmt는 통과했다. 새 intrinsic matrix에서 발견한 실패는 수정 후 다시 실행했다.

넓은 회귀 집합은 green이 아니다. `spinon-style-to-layout --lib`는 394/404이며 C07 aspect-ratio 직렬화 1건, fixed-track auto-size/stretch contract에 걸리는 C11.5 alignment/gap 8건, mixed partial-implicit placement 기대값 1건이 실패했다. `spinon-layout --lib`는 443/444이며 이번 변경이 추가한 두 고정 입력 intrinsic track/placement 검사는 통과했고, C15 HostDocument direct-text source-order test 1건이 실패했다. 이 실패들은 이 review에서 통과 처리하지 않았고 별도 회귀로 남긴다. 전체 simulator 및 test 한계는 [runtime evidence](README.md)에 기록했다.
