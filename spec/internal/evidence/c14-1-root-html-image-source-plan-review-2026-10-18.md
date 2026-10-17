# C14.1 파싱된 루트 HTML 이미지 입력 계획 실패 관점 검토 20개

검토 대상은 기존 C14.1 live-V8-eval 앱 fixture에 `RuntimeSession::new_with_root_html()` 초기 문서 입력을 추가하는 계획과 새 고정 HTML fixture다. 각 관점은 별도 source, ownership, admission 또는 검증 불변 조건을 다룬다.

| # | 실패 관점 | 계획의 대응 및 판정 |
|---:|---|---|
| 1 | 기존 eval 기반 입력을 root HTML로 교체해 기존 evidence가 사라지는가 | 기존 route를 유지하고 두 경로를 한 report에서 별도로 실행한다. 통과 기준 1·6에서 기존 경로와 stale control을 계속 검사한다. 통과. |
| 2 | root HTML의 `<img>`를 다시 JS `createElement`로 만들고 parsed input으로 오인하는가 | parsed route는 `new_with_root_html` source tree에서 image를 찾아야 하며 report에 `parsed-root-html-v8`를 기록한다. 통과. |
| 3 | document parser의 quirks mode 문서가 성공으로 취급되는가 | fixture는 `<!doctype html>`을 고정하고 parser/session 초기화 실패를 성공으로 바꾸지 않는다. 통과. |
| 4 | HTML5 recovery로 예상 밖 image가 삽입되거나 이동해도 첫 항목을 선택하는가 | unique `html/head/body/main#root/img#image`와 연결 상태를 모두 검사한다. 누락·중복은 실패다. 통과. |
| 5 | 같은 id가 문서 다른 위치에 있어도 임의로 대상이 되는가 | 전체 HostDocument tree에서 id와 tag/namespace가 맞는 후보가 정확히 하나여야 한다. 통과. |
| 6 | SVG namespace의 `img` 비슷한 element가 HTML image로 admission되는가 | XHTML namespace와 local name `img`를 함께 요구한다. 통과. |
| 7 | detached 또는 template contents 안의 image를 연결된 resource로 해석하는가 | 선택 후 `HostDocumentSnapshot::is_connected`를 확인하고 template contents는 조회 대상으로 열지 않는다. 통과. |
| 8 | `src`를 trim, URL resolve, case fold 또는 normalize해 다른 요청을 같은 입력으로 본다 | DOM의 unqualified `src` 값을 고정 fixture URL과 byte/string 수준 exact 비교한다. 통과. |
| 9 | 길거나 비어 있는 `src`가 resource 요청 단계에서 잘못 성공하는가 | 기존 `ImageRequest::from_document`의 빈/길이 제한 오류를 그대로 전파하고 exact fixed URL 비교를 추가한다. 통과. |
| 10 | HTML `<style>` 대신 저장소 HTML 문자열에서 CSS를 따로 재검색해 다른 stylesheet를 적용하는가 | parsed snapshot의 연결된 HTML style element text child에서 CSS를 얻는다. 계획은 단일 non-empty style만 허용한다. 통과. |
| 11 | head metadata가 body layout tree에 추가되거나 style element가 보이는 box가 되는가 | `main#root`를 layout mount로 찾고 head/style은 stylesheet text source로만 사용한다. image geometry는 image NodeId로 확인한다. 통과. |
| 12 | inline script가 parse 중 실행되어 HTML source를 동적으로 바꾸는가 | `new_with_root_html`의 inert-script 계약을 유지하고 initial snapshot에서 직접 image를 찾는다. JS script 실행을 성공 조건에 넣지 않는다. 통과. |
| 13 | snapshot 복사가 V8 actor 바깥에서 mutable bridge를 직접 읽는가 | 기존 feature-gated actor snapshot API만 사용하며 timeout/queue/closed 오류를 전파한다. 통과. |
| 14 | V8 actor 준비 완료보다 snapshot 요청이 앞서 빈 document를 읽는가 | `new_with_root_html`이 초기 document import 뒤 session ready를 응답한다는 runtime 계약에 의존하고 unique root gate로 확인한다. 통과. |
| 15 | generation/document/render-tree revision이 다른 request/resource/layout을 섞는가 | 각 source stamp equality를 report gate로 유지하고 parsed route에서도 동일 immutable snapshot만 전달한다. 통과. |
| 16 | 다른 PNG bytes가 같은 크기라는 이유로 자연 크기 oracle을 통과하는가 | 기존 reference의 byte length·SHA-256 검증을 두 경로에서 재사용한다. 통과. |
| 17 | AVD density/iOS scale을 CSS natural size에 중복 곱하는가 | 고정 `120×80 CSS px`, DPR 1 입력을 쓰고 플랫폼 physical scale을 report와 layout에서 분리한다. 통과. |
| 18 | C14.1 layout이 `main#root`를 시작점으로 허용하지 않거나 HTML parser가 만든 보이는 whitespace text node가 intrinsic layout에서 거부되는가 | 초기 계획 검토는 Taffy의 HostRoot 직계 layout root 제한과 visible text 측정 부재를 놓쳤다. Android 실행에서 `HostDocument 레이아웃 루트 15는 HostRoot 직속 요소가 아닙니다`와 뒤이은 `UnsupportedTextNode` 실패가 재현됐다. 수정 후 Stylo와 layout 모두 parsed `html` documentElement를 root로 쓰고 `head, head *`를 명시적으로 숨기며, head 외부 text/구조 whitespace가 없는 고정 source를 사용한다. `html=block`, `head=none`, `main=block`, `img=block` 모두 확인한다. 현재 고정 fixture에 한해 Android AVD에서 통과했으며 일반 HTML text/head layout 지원 주장은 아니다. |
| 19 | 두 경로 중 한 경로가 실패해도 top-level report가 `status=ok`가 되는가 | top-level status는 기존 eval와 root HTML 두 하위 status 및 source/frame gate의 conjunction이다. 통과. |
| 20 | fixture route 결과를 product HTML/resource support로 확대 해석하는가 | 계획과 PR은 test-only opt-in 경로로 한정하고 C14.1/C14/U05 체크를 유지한다. loader·fetch·async·paint·pixel은 명시적 제외다. 통과. |

## 계획 검토 판정

20개의 실패 관점은 중복 없이 source parsing, HostDocument identity, actor ownership, style provenance, C14.1 measurement, simulator/artifact 경계를 각각 다룬다. 계획에 기록된 unique-node, exact-source, revision equality, dual-route aggregate status gates를 구현 전 고정했다.

## 실행 중 발견된 검토 누락 보정

초기 #18의 `main#root` subtree-only layout 제안은 실제 layout API 계약을 충족하지 않았다. 첫 관련 AVD report에서 `HostDocument 레이아웃 루트 15는 HostRoot 직속 요소가 아닙니다`가 나와 즉시 확인했다. documentElement root로 변경한 뒤에도 head의 `display:inline` 및 HTML structure whitespace의 `UnsupportedTextNode` 오류가 이어졌다. 이 반례들은 초기 20개 계획 검토에서 찾아내지 못한 결함이며 runtime 시도 및 수정 로그를 evidence에 보존했다. 보정된 #18은 해결된 제한 입력만 판정하며 C14.1 전체나 일반 HTML layout을 통과 처리하지 않는다.
