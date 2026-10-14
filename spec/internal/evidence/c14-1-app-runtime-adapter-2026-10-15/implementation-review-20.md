# C14.1 앱 adapter 구현 실패 관점 검토 20개

구현 코드, Android API 37 ARM64 AVD 실행, iPhone 17 Pro/iOS 26.2 Simulator 실행, feature-off/route-conflict 결과, 정적 archive 경계를 서로 독립적인 실패 경로로 검토했다. 구현 전 계획 검토의 항목이나 판정을 복사하지 않았다.

| # | 실패 관점 | 코드·실행에서 확인한 결과 |
| --- | --- | --- |
| 1 | 기본 Cargo 빌드에서 fixture module 또는 payload가 들어간다. | `src/lib.rs`의 feature gate와 C14.1 opt-in feature를 확인했다. Android/iOS 기본 정적 archive 모두 symbol·PNG payload가 없고 opt-in positive control에는 있다. |
| 2 | 선언부만 feature-gated이고 C implementation symbol은 기본 artifact에 남는다. | C header와 Rust `no_mangle` 진입점의 compile gate를 대조했다. 양 플랫폼 archive boundary log에서 default 부재와 opt-in 존재를 확인했다. |
| 3 | C ABI 선언, Android JNI 호출, iOS ObjC++ 호출이 서로 다른 flag 이름을 쓴다. | Cargo feature, Gradle environment/BuildConfig, JNI macro, Xcode define, Swift condition 및 ObjC++ macro가 `C14_1_APP_FIXTURE_TEST`로 연결된다. 두 플랫폼 모두 opt-in 앱 실행 성공으로 호출 경로를 확인했다. |
| 4 | 잘못된 build flag 값이 조용히 기본값으로 바뀐다. | Android/iOS build script가 `2`를 status 2와 0-or-1 오류로 거부하는 negative control을 확인했다. |
| 5 | adapter가 필요한 core/image/layout/style 의존성을 feature에서 빠뜨린다. | feature dependency 목록을 실행 경로의 `HostDocument`, `ImageRequest`, layout snapshot, Stylo/Taffy 호출과 대조했다. locked feature build와 앱 양쪽 링크가 통과했다. |
| 6 | `img` 요청이 다른 node의 `src` 또는 빈 URL에서 decode된다. | synthetic document에서 image node의 `src`가 설정되고 그 `HostNodeHandle`로 `ImageRequest::from_document`를 호출한다. 최종 report의 NodeId 및 resource lookup이 image node와 일치한다. |
| 7 | 고정 PNG decode 실패가 성공 report로 변환된다. | decode와 snapshot 생성은 각자 오류를 반환하고, 오류면 C ABI가 `-2`와 `status:error`를 쓴다. Simulator 성공 report에는 status와 실제 24×16 decode 크기가 기록됐다. |
| 8 | resource snapshot과 intrinsic layout snapshot의 source/revision이 서로 다르다. | 두 equality를 pass predicate에 포함한다. Android와 iOS report에서 `intrinsic_source_matches`와 `intrinsic_revision_matches`가 모두 true다. |
| 9 | stale snapshot admission을 구현한 것으로 오해한다. | 코드는 이 fixture에서 매번 새 synthetic document와 snapshot을 만든다. 비동기 교체/stale render gate는 실행하지 않으며 README/STATUS에서 미검증으로 구분했다. |
| 10 | layout에 replaced leaf frame이 없는데 default frame으로 통과한다. | image frame map lookup 실패는 오류를 반환한다. 성공 시 report에 실제 frame `(0,0,24,16)`이 있다. |
| 11 | reference JSON에 natural dimension이나 rectangle component가 빠져도 기본값으로 성공한다. | observation, width/height, rect, x/y/width/height를 각각 필수로 읽는다. 누락은 fixture 오류가 되고 성공 판정에 도달하지 못한다. |
| 12 | natural size와 frame 오차에 동일한 허술한 허용치를 쓴다. | PNG natural size는 reference와 정수 dimension exact match이며, CSS frame component만 `0.5` CSS px tolerance를 사용한다. report에 두 expected 값과 tolerance가 남는다. |
| 13 | NaN/무한 frame이 비교에서 우연히 통과한다. | frame oracle 비교에서 actual component의 `is_finite()`를 먼저 요구한다. invalid frame은 일치 결과를 만들지 못한다. |
| 14 | null/zero-capacity 또는 짧은 C output buffer에서 호출자가 성공으로 오해한다. | C ABI null/zero 입력은 `-1`, 짧은 buffer는 `-3`을 반환한다. focused C ABI buffer test가 workspace verification에 기록돼 있다. |
| 15 | Android native 계산이 UI thread를 막거나 화면이 Activity 종료 뒤 갱신된다. | fixture는 `bootstrapExecutor`에서 계산하고 main thread에서 화면을 변경하며 `activityClosing`을 검사한다. 실제 AVD에서 즉시 report 화면과 log를 확인했다. |
| 16 | iOS native 계산이 main queue를 막거나 controller 해제 뒤 callback을 호출한다. | C ABI는 global user-initiated queue에서 실행하고 UI 변경은 main queue에서 weak self를 확인한다. iOS Simulator에서 성공 report, log와 화면 갱신을 확인했다. |
| 17 | C 외 실행군(R10 등)을 함께 전달했을 때 충돌 검사가 놓친다. | 첫 검토에서 C-prefixed route만 검사하는 경계를 발견했다. Android는 다른 `spinon_*` intent extra, iOS는 다른 `--spinon-*` argument를 거부하도록 수정했다. 양쪽 Simulator에서 C fixture와 R10을 각각 조합한 뒤 모두 `SPINON_C141_ARGUMENT_ERROR`를 남겼다. |
| 18 | feature-off 앱에서 fixture route가 enabled처럼 보이거나 성공 report를 만든다. | Android BuildConfig와 iOS compile condition이 disabled 화면/log로 분기한다. 양쪽 feature-off 설치 실행이 `SPINON_C141_DISABLED`를 남겼다. |
| 19 | 화면 요약 parser가 report 오류를 PASS로 표시하거나 실제 수치를 숨긴다. | 두 화면은 native status와 JSON status를 함께 확인하고 실패·parse 오류를 별도로 표시한다. feature-on 화면은 PASS, 24×16, frame, revision, viewport를 캡처했다. |
| 20 | 앱 화면 PASS를 image paint, overflow, 실기기 또는 Metal pixel parity로 확대한다. | 이 adapter는 결과 report만 표시한다. README와 STATUS는 paint/overflow/실기기/Metal readback을 별도 미검증으로 기록하고 C13.1 overflow evidence와 연결했다. |

구현 실행 중 iOS 첫 빌드에서 nullable native string 처리, 그 다음 실행에서 `status=0 report=` wrapper를 포함한 JSON parse 오류가 드러났다. Swift bridge의 nil fallback과 wrapper 제거를 수정한 뒤 iOS feature-on build/run이 통과했고 화면 캡처의 보고 값과 runtime log가 일치했다. 현재 이 검토에서 남은 미해결 코드 P1/P2는 없다. C14.1 전체의 stale admission, live V8 document 통합과 paint는 구현 gate 밖이라 미완료다.
