# C04.12 parsed root HTML Block paint — implementation review 20

**범위:** 실제 diff, opt-in fixture runtime, Android/iOS adapter, Rust test와 Simulator evidence를 대상으로 한 구현 후 검토다. 계획 검토 및 PR diff 검토와는 별도다.

| # | 실패 관점 | 확인·결과 |
|---:|---|---|
| 1 | synthetic DOM이나 별도 parser가 HTML scene 입력을 대신하는가 | fixture는 `RuntimeSession::new_runtime_gpu_root_html_block_paint_fixture`에서 기존 parsed HostDocument를 사용한다. Android/iOS report의 15개 document node와 scene 5 box가 확인됐다. 통과. |
| 2 | `<head>` metadata·style·title이 visible paint node로 누출되는가 | profile UA stylesheet가 metadata 요소를 숨기고 tree validator는 지원되지 않는 visible element를 거부한다. simulator scene order에 hidden node가 없다. 통과. |
| 3 | `display:none` 자손의 배경이나 후손이 장면에 들어오는가 | ancestor hidden 상태를 내려 보내고 renderer 순서 생성에서도 숨긴 subtree를 건너뛴다. 테스트와 5-box scene order에서 hidden fixture node가 빠진다. 통과. |
| 4 | hidden ancestor 아래의 text가 불필요하게 전체 장면을 실패시키는가 | `display:none` 경계가 있을 때만 hidden text를 제외한다. visible text와 hidden text가 함께 있는 runtime test가 통과한다. |
| 5 | visible non-whitespace text가 glyph 없는 빈 상자로 성공 처리되는가 | tree validation은 hidden ancestor 밖의 non-whitespace text를 거부한다. visible text 거부 test가 통과한다. |
| 6 | SVG 등 foreign namespace 요소가 HTML tag 이름만으로 통과하는가 | namespace와 local name을 둘 다 확인한다. runtime test의 SVG 실패가 통과한다. |
| 7 | 지원 목록 밖의 visible HTML element가 부분 paint되는가 | visible element는 `html/body/main/div`로 닫고 나머지는 실패한다. hidden element만 제외된다. 통과. |
| 8 | inline/flex/grid/table display가 Taffy에서 우연히 비슷한 box를 만들어 허용되는가 | visible element computed display를 `block`으로 제한한다. 그 외 display는 안정된 layout failure를 만든다. 코드 경계 확인. |
| 9 | body margin이나 collapse 동작을 브라우저 기본값으로 추정해 geometry가 달라지는가 | visible node margin은 typed zero만 허용한다. fixture와 Chrome oracle 모두 body margin 0이다. 비영 margin collapse는 미지원으로 fail-closed한다. |
| 10 | percent/content/min-max 등 지원하지 않는 dimension이 부분적으로 투영되는가 | visible width/height는 Auto 또는 LengthPx만 허용하며 layout projection은 profile allowlist를 따른다. 기준 fixture px frame이 두 platform log와 일치한다. |
| 11 | 허용되지 않은 inline property 또는 stylesheet 기능이 조용히 빠지는가 | root profile inline author allowlist, stylesheet feature 검사, 첫 cascade diagnostic 실패를 확인했다. malformed stylesheet는 layout 이전에 오류 처리한다. 코드 경계 확인. |
| 12 | partial alpha나 wide-gamut 배경이 sRGB byte로 조용히 바뀌는가 | 검토 중 root profile이 generic legacy sRGB converter를 타 Display-P3를 수용할 수 있음을 발견했다. profile을 root HTML color-space validator로 연결하고 translucent 결과를 거부하도록 수정했다. 새 test에서 partial alpha·Display-P3 모두 `UnsupportedComputedBackgroundColor`; 통과. |
| 13 | transparent box가 draw 되거나 sibling paint order가 바뀌어 아래 root 색이 달라지는가 | transparent paint는 draw 생략, scene preorder는 `[0:1,1:9,2:10,3:11,4:14]`다. root 배경 표본 세 지점이 모두 기대값과 일치한다. |
| 14 | light/dark preference가 CSS media query가 아니라 잘못된 native fallback에서 읽히는가 | 같은 CSS viewport와 OS scheme input으로 environment revision을 전달했다. light/dark에서 child 색만 green/brown로 변하고 나머지 네 표본은 고정된다. |
| 15 | layout key의 document/style/environment revision이 stale snapshot과 섞이는가 | request key와 HostDocument snapshot revision을 검증하고 render snapshot의 `CurrentLayoutInputs`를 같은 source revision에서 만든다. stale 값은 layout failure 경로다. 코드 확인. |
| 16 | root 누락·중복 또는 root 아닌 fragment가 성공 장면으로 반환되는가 | 0/복수 host root는 각각 empty scene/실패로 분기하며 단일 root는 HTML namespace `html`이어야 한다. 기준 fixture는 실제 parsed document root다. 코드 확인. |
| 17 | CSS pixel 중심 좌표가 scale·표면 크기에서 overflow나 축 혼동을 일으키는가 | Rust는 각 축을 CSS viewport와 실제 surface 크기 비로 계산하고 0·비유한·viewport 밖·drawable 밖 좌표를 거부한다. 좌표 단위 테스트 2개 통과, AVD와 iOS의 서로 다른 scale에서 일치했다. |
| 18 | Android `COPY_SRC`를 제공하지 않는 AVD에서 표면 생성 실패를 색 성공으로 오해하는가 | AVD의 surface capability 실패 뒤 별도 Android path를 사용한다. WGPU surface를 `COPY_SRC`로 만들지 않고 실제 `SurfaceView` PixelCopy를 수행하며 Rust 좌표와 RGBA를 각각 확인했다. light/dark 5/5. |
| 19 | PixelCopy callback이 surface generation·dark mode 변경 뒤 오래된 결과를 통과시키거나 adapter 부재·reflective argument mismatch로 worker가 죽는가 | capture source guard는 generation, renderer handle, dimension과 mode를 재검사한다. count mismatch는 index 전에 거부하며 reflection·argument 예외는 visible failure status로 닫는다. 일반 callback path는 simulator에서 확인했고 현재 adapter Java는 별도 compile 성공; generation race·adapter 부재는 코드 경로 검토만 했다. |
| 20 | iOS pre-compositor readback, fixture ABI, 기본 artifact warning이 제품 지원처럼 오해되는가 | iOS log는 Metal `Bgra8Unorm` drawable sample로 제한한다. 최초 기본 check에서 난 dead-code warning은 fixture 계산 entry를 feature gate한 뒤 최종 default build에서 사라졌다. 기본 FFI archive에는 fixture symbols·payload가 없고 opt-in artifact에만 있다. AVD SwiftShader와 iOS Simulator의 검증을 실기기·hardware GPU·최종 compositor proof로 확대하지 않는다. |

## 판정

색 경계 수정 뒤 `spinon-style` 180/180, opt-in `spinon-runtime` 집중 테스트 2/2, `cargo fmt --all -- --check`, `git diff --check`가 통과했다. 이 수정 뒤 Android/iOS 앱을 다시 빌드하거나 캡처하지 않았다. 기록된 고정 fixture는 opaque 8-bit sRGB라 성공 경로 입력은 바뀌지 않지만, 그 한계를 PR 본문과 실행 근거에 그대로 표시한다.
