# C14.1 parsed root HTML 입력 PR 변경 실패 관점 30개

검토 대상은 이 PR이 추가하는 fixture-only Rust 경로, Android/iOS report UI, 고정 HTML source, 계획/status/evidence 문서와 artifact 경계 기록이다. 계획 리뷰 20개 및 구현 리뷰 20개와 다른 PR diff 관점으로 변경 전체를 다시 확인했다.

| # | PR 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 기존 eval 기반 C14.1 경로를 root HTML 경로가 대체해 이전 stale-source 검증을 잃는가 | 기존 eval 경로 및 `SourceChanged` negative control이 남아 있고 새 경로를 별도 JSON key로 추가한다. 통과. |
| 2 | 기존 route 결과 report가 새 field 때문에 JSON decoding에서 실패하는가 | Android/iOS formatter는 새 top-level `live_v8_eval`과 `parsed_root_html_v8` object를 각각 확인한다. 두 platform 앱 report 통과. |
| 3 | 하위 route 하나가 실패해도 top-level `status=ok`가 되는가 | Rust top-level status는 두 개의 route boolean conjunction이고 route parse/layout 오류는 FFI error를 반환한다. AVD/Simulator 양쪽 실제 report 통과. |
| 4 | HTML parse가 실패하거나 document mode가 아닌 입력이 성공으로 분류되는가 | `new_with_root_html()` parse 오류를 전파하며 fixture에 doctype이 있고 exact source SHA gate가 있다. 통과. |
| 5 | HostRoot 바로 아래 documentElement가 아닌 node를 document root로 허용하는가 | root child 하나와 HTML `html` tag/namespace를 확인한다. parsed documentElement 외 root는 거부한다. 통과. |
| 6 | head/body 누락·중복 또는 잘못된 parent가 성공하는가 | `html`의 direct child 중 각 HTML `head`·`body`가 정확히 하나여야 한다. 통과. |
| 7 | SVG/XHTML의 같은 local name을 HTML node로 오인하는가 | 모든 선택은 XHTML namespace와 local name을 함께 비교한다. 통과. |
| 8 | `main#root` 또는 `img#image` 중복 id에서 첫 항목이 임의 선택되는가 | 전체 HostDocument를 순회해 exact tag/id/namespace 조합이 unique해야 한다. 통과. |
| 9 | 다른 body 또는 nested element의 image가 기대 subtree로 오인되는가 | `body > main#root > img#image` direct relationship을 확인한다. 통과. |
| 10 | detached image/resource를 연결된 페이지 resource로 측정하는가 | 선택한 main과 image의 connected 상태를 확인한다. 통과. |
| 11 | source script가 parser/import 중 실행돼 report 입력을 변경하는가 | runtime 계약상 root HTML script를 실행하지 않으며 report가 `script_execution=not-run`을 남긴다. 통과. |
| 12 | V8 worker/actor 밖에서 HostDocument mutable state를 읽는가 | actor-owned fixture snapshot API를 사용하고 actor queue error/timeout은 실패로 전달한다. runtime test 및 Simulator route 통과. |
| 13 | 다른 HTML source가 같은 report ID를 쓰며 재현 입력을 숨기는가 | compile-time source hash `56fc0d9c...d77b386`를 보고하고 동일 값이 아니면 route pass를 거부한다. 통과. |
| 14 | HTML `<style>` 외 별도 CSS 상수가 계산을 주도하는가 | stylesheet text는 parsed HostDocument의 direct `head/style` text child에서 생성된다. 통과. |
| 15 | 복수/누락/빈 style 또는 element child를 조용히 넘기는가 | 단일 style element, non-empty text, text-only child를 fail-closed로 요구한다. 통과. |
| 16 | cascade가 documentElement selector/inheritance 대신 `main` subtree만 계산하는가 | Stylo shared document root와 cascade input은 parsed `html` documentElement다. 통과. |
| 17 | head/out-of-root style이 필터링돼 layout profile mismatch가 숨겨지는가 | computed snapshot을 잘라내지 않고 layout에 전달하며 html/head/main/img display를 각각 pass gate에 둔다. 통과. |
| 18 | `main`을 Taffy root로 전달해 HostRoot 계약을 위반하는가 | 초기 runtime에서 재현 후 layout 시작점을 actual documentElement로 수정했다. 실패 log와 최종 passing screen/report 모두 PR evidence에 있다. 통과. |
| 19 | 지원하지 않는 head/default display를 browser UA 결과로 잘못 추정하는가 | fixture의 block/none display를 명시하고 이를 일반 UA stylesheet 지원으로 해석하지 않는다. 계획, README, status가 동일 경계를 말한다. 통과. |
| 20 | document formatting whitespace가 지원하지 않는 text measurement를 트리거하는가 | layout failure가 실제 발생했고 고정 source를 no-visible-text input으로 바꿔 source hash로 고정했다. 일반 whitespace/text layout을 주장하지 않는다. 통과 범위 한정. |
| 21 | image의 잘못된 `src`를 alias/normalization해서 고정 bytes에 연결하는가 | parsed attribute에서 생성한 request URL이 고정 URL과 exact-match해야 한다. 통과. |
| 22 | 외부 네트워크를 사용하거나 fetch 실패를 숨기는가 | decode에는 고정 PNG byte slice를 직접 넣는다. fake URL은 identity뿐이고 loader/fetch를 범위 밖으로 명시한다. 통과. |
| 23 | 같은 크기의 대체 이미지 bytes가 PNG oracle과 같다고 처리되는가 | 기존 PNG reference가 97-byte length와 SHA를 검증한 뒤 두 경로에서 같은 bytes를 쓴다. Rust negative metadata test 통과. |
| 24 | request, completion, resource snapshot이 서로 다른 source revision을 섞는가 | request/source/intrinsic source equality와 intrinsic revision equality를 비교한다. 최종 JSON은 true다. 통과. |
| 25 | density/scale 또는 `naturalWidth` 반올림이 CSS px를 바꾸는가 | 예상 natural size는 `24×16` exact; viewport CSS px에서 platform scale은 곱하지 않는다. 양 simulator report 일치. |
| 26 | NaN 또는 큰 오차를 허용치 안으로 감추는가 | frame 각 성분은 finite여야 하고 각 차이는 `≤0.5 CSS px`; 최종 frame `(0,0,24,16)`. 통과. |
| 27 | Android 화면이 두 route 중 하나를 숨기거나 subreport mismatch를 pass라고 보이는가 | JSON formatter가 둘을 필수로 찾고 각각 natural/frame/source revision/status를 표시한다. AVD screenshot에서 두 route visible/pass. |
| 28 | iOS 화면이 Android와 다른 key/경로를 요구하거나 subreport 상태를 누락하는가 | Swift formatter가 같은 두 JSON key를 요구하고 route별 status와 revision 결과를 표시한다. iPhone screenshot에서 모두 visible/pass. |
| 29 | JNI/Objective-C fixed buffer가 부족한데 truncated JSON을 성공 처리하는가 | Rust FFI writer capacity failure는 error code로 반환하며 Android/iOS route status marker가 native report result를 구분한다. 실제 두 앱 report complete status는 `ok`; 현재 4096-byte buffer 안이다. |
| 30 | fixture code가 default artifact에 들어가거나 C14.1을 완료 처리하는가 | default/opt-in archive와 linked app image scans 및 feature-off screenshots를 보존했다. C14.1/C14 checkbox는 미완료이고 status/evidence는 non-product slice로 설명한다. 통과. |

## 발견·수정된 사항

실행 과정에서 PR이 처음 제안한 `main#root` layout root는 API 계약에 어긋났다. documentElement를 root로 바꾼 뒤에도 제한 UA display 및 whitespace text admission이 실패했다. 구현은 runtime error를 무시하지 않았고 root/display/input을 수정한 다음 Android/iOS Simulator에서 최종 source hash가 포함된 report를 재실행했다. 자세한 최초 실패와 수정 경로는 [실행 README](README.md)에 있다.

## 판정과 남은 범위

고정 input·두 simulator·feature boundary·focused Rust 검사에서 이 PR의 scope에 해당하는 관문은 통과했다. PR은 C14.1, C14 또는 U05를 완료로 표시하지 않는다. HTML loader/fetch, general text/whitespace, UA styles, async replacement, image paint/pixels, overflow와 실기기 동작은 미검증이다.
