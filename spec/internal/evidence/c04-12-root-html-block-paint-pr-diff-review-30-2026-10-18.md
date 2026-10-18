# C04.12 parsed root HTML Block paint — PR diff review

**확인일:** 2026-10-18
**대상:** `origin/main` (`182e59811312311661fed3dba721a566dc0f7c16`) 대비 PR #313 변경과 후속 whitespace 수정
**구분:** 계획 검토 및 구현 검토와 별개인 PR 변경 검토

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | scene 입력에서 원본 HTML5 parser tree 대신 합성 DOM을 쓰는가 | fixture 생성은 `ParsedRootHtml::parse`가 채운 기존 `HostDocument`를 `RuntimeSession`에 전달한다. 새 parser 경로는 없다. 통과. |
| 2 | fixture profile이 일반 RuntimeSession 또는 기본 앱 경로에 자동 적용되는가 | 생성 함수, runtime profile 선택, FFI fixture 심볼은 전용 opt-in Cargo feature에 묶였다. 기본 `RuntimeSession::new()`와 root-HTML 기본 경로는 유지된다. 통과. |
| 3 | root가 HTML namespace의 `html`이 아닌데 HTML scene으로 처리되는가 | tree validation은 document root namespace와 local name을 확인하고 아니면 `invalid_root_html`로 실패한다. 통과. |
| 4 | `<head>`의 metadata, style, title이 layout/paint box로 누출되는가 | 추가 UA stylesheet가 head 계열 요소를 숨기며 visible element validator도 허용 목록 밖 태그를 거부한다. simulator scene은 5개 visible box다. 통과. |
| 5 | foreign namespace의 `main`/`div`가 이름만으로 허용되는가 | namespace와 local name을 함께 검사한다. SVG negative fixture는 실패한다. 통과. |
| 6 | `display:none` 조상 아래의 후손이 box나 paint로 살아나는가 | hidden 상태가 subtree 순회, layout 입력, render 순서에 전달된다. hidden text fixture도 성공 장면에서 제외된다. 통과. |
| 7 | HTML에서 접히지 않는 Unicode 공백 텍스트가 공백으로 오인되어 누락되는가 | 검토 중 Rust `trim()`이 U+2003 등을 제거해 visible text를 성공 처리할 수 있음을 찾았다. 공통 `DomString` 판정을 HTML/CSS ASCII 접이 공백으로 바꿨고 NBSP·U+2003 거부 단위 테스트 1개가 통과했다. 새 runtime 통합 회귀 테스트는 추가했지만 아래 자원 제한으로 실행 완료하지 못했다. |
| 8 | 실제 CSS collapsible ASCII 공백이 불필요한 텍스트 오류를 내는가 | 새 판정은 space/tab/LF/CR/form-feed만 접이 공백으로 취급한다. 해당 문자열의 core 단위 테스트가 통과했고 runtime 테스트에도 성공 기대값을 추가했다. runtime 재실행은 미완료다. |
| 9 | visible glyph가 paint 목록에는 없어서 조용히 사라지는가 | hidden 조상 밖의 공백 아닌 text는 `visible_root_html_text` 실패다. 기존 visible-text negative 경로를 확인했다. 통과. |
| 10 | 지원하지 않는 visible HTML tag가 부분 box로 통과하는가 | visible 목록은 `html/body/main/div`로 제한되고 나머지는 `unsupported_visible_root_html_element`가 된다. 통과. |
| 11 | visible node의 inline/flex/grid/table display가 비슷한 결과만으로 허용되는가 | computed display가 `block`이 아니면 layout 전에 거부한다. 통과. |
| 12 | 별도 UA reset 규칙이 author cascade 뒤에 와서 author 값을 덮는가 | built-in UA 다음, 추가 root-HTML UA, author 순으로 registry에 넣는다. origin과 등록 순서를 확인했다. 통과. |
| 13 | unsupported inline CSS가 allowlist에서 빠져 cascade 입력에서 사라지는가 | root Block paint author property 목록을 별도로 검사하고 미지원 inline property는 실패한다. 통과. |
| 14 | 미지원 stylesheet 기능이나 파서 진단이 조용히 무시되는가 | media 기능 검증 및 profile 진단 검사가 있고, 진단은 cascade 단계에서 실패한다. 통과. |
| 15 | percent 또는 intrinsic dimension이 고정 px처럼 Taffy에 전달되는가 | visible width/height는 `Auto` 또는 `LengthPx`만 허용하고 나머지는 `unsupported_root_html_dimension`이다. 통과. |
| 16 | 기본 body margin이나 margin collapse를 임의로 추정해 frame이 달라지는가 | visible 네 방향 margin은 typed zero만 허용한다. 기준 fixture와 Chrome oracle의 body margin은 0이다. 통과. |
| 17 | layout root의 containing block이 CSS viewport와 surface pixel을 혼동하는가 | root profile은 viewport containing-block sizing을 선택하고 surface 크기는 별도 renderer 단계에서 사용한다. simulator frame은 CSS px 기준이다. 통과. |
| 18 | prefers-color-scheme 변경 뒤 이전 style/environment revision이 게시되는가 | 환경 변경 시 revision이 증가하며 completed key와 현재 환경 revision을 검증한다. 기존 light/dark runtime 비교가 색 변경과 key 증가를 확인한다. 통과. |
| 19 | partial alpha가 opaque sRGB로 잘못 변환되어 blending 없이 그려지는가 | root profile 전용 색 변환이 translucent 값을 거부한다. partial-alpha regression test가 포함되어 있다. 통과. |
| 20 | Display-P3 등 wide-gamut 값을 generic converter가 sRGB로 잘라 허용하는가 | root HTML 전용 색 변환과 regression test가 Display-P3를 거부한다. 구현 검토에서 발견해 수정된 경계다. 통과. |
| 21 | transparent block이 불투명 box처럼 칠해 아래 색을 가리는가 | transparent paint는 paint snapshot에서 투명으로 유지되고 renderer는 그 box에 배경 draw를 만들지 않는다. simulator 표본은 아래 root 색과 일치한다. 통과. |
| 22 | hidden node 또는 text가 style/layout node 집합 대조에서 빠져 잘못 통과하는가 | style, layout, document node 집합을 각각 검증하고 text는 명시된 hidden/ASCII whitespace 예외 외에 거부한다. 새 Unicode 예외 경계를 제외하고 코드 경로를 확인했다. 통과. |
| 23 | document preorder와 paint preorder가 달라 sibling 색상 순서가 뒤바뀌는가 | render snapshot builder가 HostDocument child order를 보존하며 fixture scene order와 Android/iOS 표본이 기준과 맞았다. 통과. |
| 24 | CSS pixel의 정수 좌상단을 샘플링해 경계 픽셀이 이웃 box로 이동하는가 | Rust 좌표는 각 축의 CSS pixel center `(x+0.5,y+0.5)`를 실제 surface 크기에 비례시킨다. 2개 좌표 단위 테스트와 simulator 좌표가 있다. 통과. |
| 25 | surface 축 비율 또는 표면 0 크기에서 잘못된 좌표를 반환하는가 | width/height를 별도로 투영하고 0 크기, 비유한 viewport, viewport 밖·drawable 밖 sample을 거부한다. 통과. |
| 26 | Android AVD에서 지원하지 않는 `COPY_SRC` 경로를 성공으로 오인하는가 | C04.12는 `COPY_SRC`를 요청하지 않고 실제 `SurfaceView`의 PixelCopy를 쓴다. API 37 ARM64 AVD light/dark에서 각각 5/5 RGBA·좌표가 맞았다. 통과. |
| 27 | PixelCopy 응답이 surface generation 변경 후에도 현재 결과로 채택되는가 | 기존 `SurfaceViewPixelCapture` source guard에 generation·handle·size·mode 공급자를 전달한다. 오래된 결과는 현재 상태로 처리하지 않는다. 통과. |
| 28 | runtime report 좌표 파싱 오류나 sample count 불일치가 성공 표시가 되는가 | adapter는 정확한 쌍 개수와 정수 좌표를 검증하고, 매 표본 좌표/RGBA 및 최종 개수가 모두 맞아야 성공 상태를 표시한다. 통과. |
| 29 | native bridge 미활성 빌드에서 잘못된 FFI 심볼 호출이나 조용한 성공이 생기는가 | C header, JNI/Objective-C bridge, Gradle/Xcode/build script의 define과 Cargo feature를 맞췄고 비활성 경로는 명시적 오류를 반환한다. 기본/opt-in archive positive·negative evidence가 있다. 통과. |
| 30 | Simulator 표면 증거, PR 이미지, 공식 완료 상태가 지원 범위를 과장하는가 | Android는 SwiftShader PixelCopy, iOS는 compositor 전 Metal drawable로 각각 한계를 기록했다. PR 이미지 네 장은 Agent Browser에서 로드 크기를 확인했다. C04.12 상위 체크와 공식 완료 수는 그대로 미완료다. 통과. |

## 판정과 남은 검증

검토에서 발견한 Unicode whitespace 누락 경계를 수정했다. `cargo fmt --all -- --check`, `git diff --check`, core의 `collapsible_html_whitespace_uses_the_css_ascii_set` 1/1은 수정 뒤 통과했다. 수정 뒤 runtime 통합 회귀 테스트도 추가했으나, 해당 `cargo test`가 Stylo 의존성을 새로 컴파일하기 시작한 뒤 사용 가능 공간이 ZTSBuild 543 MiB에서 216 MiB, data volume 945 MiB에서 535 MiB로 감소해 더 큰 빌드가 공간을 고갈시키기 전에 중단했다. runtime test는 수정 후 완료되지 않았다. 실행 중 cargo/rustc 프로세스가 없고 대상 경로를 연 프로세스도 없음을 확인했다.

GitHub branch protection 응답에는 required status check가 없고 PR checks도 0개다. 따라서 원격 CI green 판정은 없다. Android/iOS Simulator 캡처와 앱 빌드는 앞서 문서화한 Rust 색상 경계 수정 전 결과이며, 이후 수정은 그 고정 opaque sRGB fixture 입력을 바꾸지 않는다. 이 검토는 PR diff의 범위 확인이며 일반 HTML, 실기기, hardware GPU 또는 iOS compositor 이후 동작을 증명하지 않는다.
