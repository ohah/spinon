# C14.1 parsed root HTML 입력 구현·실행 실패 관점 20개

검토 대상은 opt-in C14.1 route에 추가한 `RuntimeSession::new_with_root_html()` 경로, HostDocument snapshot 소비, parsed `<style>` cascade, image request/decode와 report/UI 연결이다. 각 항목은 별도 입력 경계나 실패 경로를 확인한다. 아래 판정은 고정 fixture 및 기록된 Android/iOS Simulator 실행에만 해당한다.

| # | 실패 관점 | 코드·근거 대조 및 판정 |
|---:|---|---|
| 1 | fixture HTML이 검토 후 바뀌어도 성공 report가 재현성을 주장하는가 | compile-time embedded source의 SHA-256을 고정 상수와 비교하고 `root_html_source_matches`를 aggregate pass gate에 넣었다. fixture SHA `56fc0d9c5a444550866eba7ee65fbe3960b27631830294093107c8c20d77b386`; 양 플랫폼 최종 report가 true다. 통과. |
| 2 | 초기 HTML import가 실패하거나 quirks document를 사용해도 성공하는가 | `new_with_root_html()`의 parse/import 오류를 전파하고 결과에서 documentElement가 정확히 HTML namespace `html`인지 요구한다. 고정 source에는 doctype이 있으며 runtime root test와 양 플랫폼 실행이 통과했다. 통과. |
| 3 | parser가 documentElement를 누락·중복하거나 임의의 root를 반환하는가 | `root_children()` 길이가 정확히 1인지 검사하고 그 node의 tag와 XHTML namespace를 확인한다. parser 오류/다른 root를 성공 처리하지 않는다. 통과. |
| 4 | 문서의 nested head/body를 direct child로 잘못 받아들이는가 | `head`와 `body`를 documentElement의 direct child 중 각각 정확히 하나로 선택한다. 잘못된 관계면 unique-direct-child gate에서 실패한다. 통과. |
| 5 | 같은 id나 다른 tag·namespace의 node가 fixture image로 선택되는가 | 전체 snapshot을 탐색해 HTML namespace + local name + exact id가 유일한 `main#root`/`img#image`만 받는다. 첫 일치 항목을 임의 선택하지 않는다. 통과. |
| 6 | image가 main 밖이나 body 밖에 있거나 disconnected인데 선택되는가 | main이 body direct child, img가 main direct child인지 별도 검사하고 두 node의 `is_connected`도 요구한다. 통과. |
| 7 | V8 actor의 mutable bridge를 바깥 thread가 직접 읽는가 | fixture 전용 `snapshot_document_for_c14_1_fixture()` actor queue API만 사용한다. queue full/stopped, closing, timeout, disconnected 응답 오류는 실패로 전파된다. runtime root HTML test 및 양 앱 route 통과. |
| 8 | root HTML의 inline script가 import 중 실행되어 선택 source를 바꾸는가 | `new_with_root_html()` 문서 계약은 script를 실행하지 않는다. report는 `script_execution=not-run`을 명시하고 parsed source 자체에서 image를 찾는다. 통과. |
| 9 | stylesheet가 source 문자열 재검색이나 별도 상수에서 와서 DOM과 달라지는가 | HostDocument의 direct `<head><style>` element 및 그 Text child에서 CSS를 만든다. source SHA는 원본 전체를 pin한다. 통과. |
| 10 | 누락·복수 `<style>` 또는 nested element가 조용히 무시되는가 | direct style element 정확히 1개, 비어 있지 않은 text, text-only child를 요구한다. 누락·중복·빈 값·element child는 오류다. 통과. |
| 11 | Stylo cascade가 실제 documentElement selector/inheritance context 대신 main subtree만 보는가 | `StyloDocumentView::new_html_shared()`가 동일 snapshot의 실제 `html` root를 받으며 parsed stylesheet를 그대로 cascade에 넘긴다. 통과. |
| 12 | cascade 결과에서 head/default-UA display가 기록 없이 layout 입력에서 삭제되는가 | computed snapshot을 필터링하지 않고 전부 전달한다. `html`, `head`, `main`, `img` display를 각각 report/pass gate로 검증한다. 통과. |
| 13 | 제한 HostDocument layout에 허용되지 않는 main node를 root로 전달하는가 | 초기 실행에서 Taffy가 `main`이 HostRoot 직계 요소가 아니라며 실패했다. layout root를 actual `html`로 바꾸고 root direct-child 계약을 만족시켰다. 실패 log는 `android-avd/runtime.log`에 보존했다. 수정 후 통과. |
| 14 | 제한 UA cascade의 `head`/`main` display 기본값을 browser UA와 같다고 가정하는가 | fixture가 `html/body/main/img`의 block과 `head, head *`의 none을 직접 선언한다. `html=block`, `head=none`, `main=block`, `img=block`을 검사하고 일반 UA default 지원 주장은 하지 않는다. 통과. |
| 15 | fixture의 보이는 whitespace/text node가 layout에 들어가 unsupported intrinsic measurement로 잘못 성공하는가 | 초기 multiline source에서 `UnsupportedTextNode`가 재현됐다. 현재 고정 source는 head 외부의 text/markup whitespace가 없고 hash로 exact input을 pin한다. visible text layout은 지원한다고 표시하지 않는다. 수정 로그와 성공 로그가 모두 보존됐다. 제한 입력 통과. |
| 16 | `src`가 비었거나 다른 URL인데 고정 PNG bytes가 엉뚱한 image에 묶이는가 | `ImageRequest::from_document()`을 parsed image handle로 호출하고 request URL을 fixture URL과 exact 비교한다. URL normalization/fetch를 하지 않는다. 통과. |
| 17 | 다른 PNG bytes가 같은 dimension으로 oracle을 통과하는가 | 바깥 `verify_png_reference()`가 embedded byte length와 SHA-256을 고정 reference와 비교한 뒤, 같은 bytes만 두 route에 준다. PNG hash는 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`. 통과. |
| 18 | request·resource·intrinsic snapshot이 다른 문서 revision 또는 generation에서 혼합되는가 | 하나의 immutable HostDocument snapshot으로 request와 resource를 만들고 request/resource source 및 intrinsic source/revision equality를 pass gate로 둔다. 양 플랫폼 final JSON에서 모두 true다. 통과. |
| 19 | NaN/infinite/잘못된 좌표가 허용 오차 비교를 통과하는가 | 네 frame 성분 각각 finite를 요구하고 Chrome 155 oracle 값과 최대 `0.5 CSS px` 차이만 허용한다. natural size는 `24×16` exact 비교다. 양 플랫폼 결과가 `(0,0,24,16)`이다. 통과. |
| 20 | 두 경로 중 하나가 실패해도 앱 화면이 pass를 보이거나 feature-off artifact에서 fixture code가 켜지는가 | top-level status는 live-V8 eval과 parsed-root 결과의 conjunction이다. Android/iOS UI는 두 subreport를 모두 파싱해 표시한다. Android API 37 AVD/iPhone 17 Pro Simulator opt-in report는 모두 `status=ok`; feature-off screenshot과 archive scan은 disabled 표시, C14.1 symbol/payload 부재를 확인했다. C14.1 제품 범위는 미완료다. 통과. |

## 검토에서 발견해 수정한 결함

실행 전 구현 검토가 놓쳤던 세 layout admission 문제를 Android AVD가 순서대로 드러냈다: non-root `main` mount, documentElement의 unsupported `head` display, visible structure whitespace의 text measurement. layout을 parsed documentElement로 통일하고 fixture display를 고정했으며 text가 없는 fixed source로 범위를 좁혔다. 이후 HTML source hash를 pass gate로 추가했다. 각 실패와 수정 후 최종 Simulator 결과는 상위 evidence README에 연결한다.

## 한계

이 표는 단일 고정 root HTML fixture의 HTML parser→초기 V8 document→HostDocument snapshot→Stylo/Taffy C14.1 geometry 경로만 판정한다. general HTML whitespace/text layout, browser UA stylesheet, CSSOM, resource loader/network fetch, async decode/replacement, image paint/pixel, overflow, 물리 기기 또는 C14.1/C14 완료를 판정하지 않는다.
