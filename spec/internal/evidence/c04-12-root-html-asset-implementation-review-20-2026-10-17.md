# C04.12 bundle HTML asset fixture 구현 실패 관점 검토

- **검토일:** 2026-10-17
- **범위:** opt-in Android/iOS bundle asset의 실제 V8 초기 document 입력 및 assertion
- **기존 parser 검토와 분리:** HTML5 parser/HostDocument import의 기존 검토표와 PR diff 검토를 재사용하지 않았다. 아래 항목은 fixture가 전달한 HTML tree와 V8에서 관찰한 값의 실패 가능성을 다시 확인한다.
- **판정:** 고정 bundle asset의 tree와 content assertion은 두 Simulator에서 통과했다. 이 결과는 viewport/color-scheme 의미 적용이나 production HTML loading을 뜻하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | fixture 코드가 asset을 읽지 않고 고정 문자열을 V8에 전달 | Android는 `readAsset`, iOS는 `Bundle.main` URL과 UTF-8 file read를 거친 source를 FFI에 전달한다. 양쪽 source hash는 동일했다. |
| 2 | 한글 text가 platform decode/encode에서 손상 | V8 assertion이 `<main>` text `앱 HTML asset`을 exact 비교했고 Android/iOS 실제 실행이 통과했다. |
| 3 | `document.documentElement`가 파싱 asset root가 아니라 빈 default document를 반환 | fixture의 root `html`, `lang=ko` assertion이 실제 V8에서 통과했다. |
| 4 | head/body getter가 root와 다른 문서나 중첩 descendant를 참조 | assertion이 head/body 존재와 둘 다 현재 root의 직접 자식임을 확인했다. |
| 5 | viewport meta가 head 문서 순서 밖으로 이동하거나 사라짐 | V8 head의 direct element children에서 `name=viewport`를 찾아 `width=device-width, initial-scale=1`을 비교했다. |
| 6 | viewport content가 source와 다르게 정규화되거나 누락 | 고정 content attribute exact 비교가 두 플랫폼에서 통과했다. 이 값이 layout viewport에 적용된다는 검증은 아니다. |
| 7 | color-scheme meta가 viewport meta와 혼동되거나 head에 남지 않음 | 별도 `name=color-scheme` element를 찾아 `light dark` content를 검사했다. |
| 8 | meta 지원 목록을 native OS preference로 취급 | 이 fixture는 선언된 content만 비교한다. `prefers-color-scheme`나 OS 설정은 호출하거나 판정하지 않는다. |
| 9 | root `<style>`의 text가 HostDocument 경계에서 누락/손상 | `:root { color-scheme: light dark; }` textContent를 실제 V8에서 exact 비교했다. |
| 10 | CSS declaration이 있다는 이유로 계산 style 또는 UA Canvas 색을 assertion 없이 PASS | 테스트가 확인하는 것은 `<style>` node/text 보존뿐이다. 계산 style, Canvas 색과 paint는 확인하지 않는다. |
| 11 | body의 `<main>`이 빠지거나 id가 잘못된 node에서 읽힘 | body의 direct element children 중 `main`을 찾고 `id=root-asset`을 비교한다. |
| 12 | main custom attribute가 다른 element에 붙음 | 동일 main node에서 `data-fixture=c04-12`를 확인했다. |
| 13 | body text 순서가 달라짐 | `<main>`의 aggregate textContent를 한글 fixture string과 exact 비교했다. |
| 14 | HTML script가 V8 세션 생성 중 실행되거나 source 평가됨 | script element는 남아 있고 `globalThis.__c04_12_asset_script_ran`는 정의되지 않은 조건이 통과했다. |
| 15 | root HTML 경로 뒤 JavaScript-only 빈 문서 세션으로 바뀜 | asset fixture API가 `RuntimeSession::new_with_root_html(source)`를 직접 호출한 뒤 assertion을 실행한다. |
| 16 | Android read/FFI가 UI thread에서 실행되어 화면이 얼거나 callback 순서가 뒤집힘 | asset 읽기와 native 호출은 `bootstrapExecutor`에서 수행하고 결과 문구 갱신만 main thread에서 한다. |
| 17 | iOS bundle read/V8 실행이 main thread에 놓임 | controller가 global user-initiated queue에서 resource read와 fixture 실행을 수행한다. |
| 18 | missing asset 또는 file decode 오류가 성공 상태로 기록 | iOS file lookup/UTF-8 read 오류는 `status=-1` report로 가고 success marker assertion을 충족하지 못한다. |
| 19 | FFI가 잘못된 source bytes를 session startup 전에 제한하지 않음 | non-UTF8, 1 MiB 초과, 양수 길이 null source의 Rust negative tests 3개가 통과했다. 길이 초과는 raw pointer read보다 앞선다. |
| 20 | native report 오류인데 앱이 PASS 화면·로그를 표시 | Android/iOS 모두 status와 asset-specific PASS marker를 함께 요구하며 실제 두 Simulator에서 asset marker가 남았다. |

## 실행 근거

- `cargo test --locked -p spinon-runtime root_html`: 8/8.
- `cargo test --locked -p spinon-ffi root_html --features c04-12-root-html-fixture-test`: 5/5.
- runtime/FFI Clippy와 rustfmt 검사 통과.
- Android API 37 ARM64 AVD `emulator-5554`: `SPINON_C0412_ROOT_HTML_ASSET status=0 status=0 c04_12_root_html_asset=PASS`.
- iPhone 17 Pro / iOS 26.2 Simulator: `SPINON_C0412_ROOT_HTML_ASSET status=pass`.
- 캡처와 source checksum은 [실행 근거](./c04-root-html-document-environment-2026-10-17/README.md)에 묶었다.

이 검토의 PASS는 고정 asset이 실제 V8 document tree에 반영된 테스트 경로에만 해당한다. 기본 앱 `index.html` loader, CSS layout, OS color preference, `prefers-color-scheme`, UA Canvas와 HTML paint는 미구현이다.
