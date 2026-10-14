# C14.1 앱 adapter PR 변경분 적대 검토 30개

PR #168의 최종 코드·빌드 경계·두 Simulator 실행·증거 문서를 서로 다른 실패 관점으로 다시 확인했다. 계획 검토 20개나 구현 검토 20개를 복사하지 않았고, 발견한 route 충돌 누락은 수정 후 다시 빌드·실행했다.

| # | 검토 관점 | 최종 변경분에서 확인한 결과 |
| --- | --- | --- |
| 1 | 개발용 fixture C ABI가 제품 public API처럼 노출된다. | `spinon_ffi.h` 선언은 C14.1 fixture-test define으로 숨긴다. 상태와 PR은 public API 추가가 없다고 구분한다. |
| 2 | header 선언과 Rust export 이름 또는 반환 코드가 어긋난다. | `spinon_c14_1_app_intrinsic_size_fixture` 이름과 성공/인자/오류/버퍼/불일치 코드가 C header·Rust 정의에서 일치한다. |
| 3 | 기본 Cargo feature가 image/layout 의존성을 끌어온다. | 새 dependency는 optional이며 feature-on/off archive 대조에서 기본 C14.1 symbol/payload는 없다. |
| 4 | `include_bytes!` PNG/HTML payload가 module 경계만으로 기본 archive에서 빠졌다고 주장한다. | `#[cfg(feature=...)]` module gate와 실제 archive 검사 둘 다 확인했다. README는 정적 archive 결과를 앱 bundle 크기 주장으로 확대하지 않는다. |
| 5 | Cargo feature는 켜졌지만 Android 앱의 Gradle flag가 누락된다. | 환경값→Gradle prepare task→native build→BuildConfig 필드 경로를 확인하고 opt-in AVD app에서 호출했다. |
| 6 | Cargo feature는 켜졌지만 iOS Swift와 ObjC++ define 상태가 갈린다. | Xcode build setting의 default 0, Swift active condition, ObjC++ preprocessor flag와 native feature 전달을 확인하고 opt-in Simulator 앱에서 호출했다. |
| 7 | C14.1 opt-in이 C04 GPU fixture를 부수적으로 활성화한다. | Cargo feature dependency에 C04가 없고 두 앱 경로는 C ABI report만 실행한다. GPU paint 경로가 아님을 UI/evidence 범위에 적었다. |
| 8 | 환경값 `2` 또는 임의 문자열이 플랫폼마다 다르게 해석된다. | Android/iOS build script negative log가 각각 flag `2`를 exit 2로 거부한다. |
| 9 | JNI 함수가 feature-off 정적 라이브러리에 없는 symbol을 무조건 호출한다. | JNI call은 compile define 내부에 있고 off branch는 disabled JSON을 반환한다. feature-off Android 앱이 빌드·설치·실행됐다. |
| 10 | iOS ObjC++ feature-off method가 빠진 C symbol을 무조건 참조한다. | C call은 macro gate 안에 있고 off branch는 disabled report를 반환한다. feature-off iOS 앱이 빌드·설치·실행됐다. |
| 11 | FFI 포인터·capacity 오류가 성공 또는 panic으로 처리된다. | null/zero-capacity, fixture error, short-buffer, oracle mismatch의 분기를 확인했다. short-buffer C ABI focused test가 통과했다. |
| 12 | C ABI `unsafe` 포인터 쓰기 범위가 호출자에게 숨겨진다. | Rust export의 `# Safety`가 output capacity만큼 writable memory를 요구하고 플랫폼 caller는 4096-byte buffer를 제공한다. |
| 13 | decode 요청이 embedded PNG 대신 외부 network resource를 기다린다. | `ImageRequest`는 synthetic HostDocument의 fixed `img src`에서 만들고, decode bytes는 embedded fixture다. `.invalid` host에 network loader 호출은 없다. |
| 14 | HTML parsing이 실행됐다고 오해하거나 synthetic tree의 출처를 숨긴다. | 코드와 README는 `<style>` block 추출과 synthetic `#root > #image` HostDocument 생성을 분리하고 HTML parser를 실행하지 않는다고 적는다. |
| 15 | 다른 node의 resource나 snapshot entry로 이미지 크기를 읽는다. | completion과 resource lookup 모두 예약한 image handle/NodeId를 사용한다. app report는 `NodeId(2)`와 같은 image report다. |
| 16 | source document가 바뀐 뒤 stale intrinsic snapshot을 현재 layout에 합친다. | fixture는 매 호출에서 새 document와 resource snapshot을 생성한다. 실시간 stale admission은 실행하지 않았고 완료 범위에서 제외했다. |
| 17 | revision 번호는 있으나 resource와 layout snapshot의 source identity가 다르다. | source 및 intrinsic revision 비교가 Rust pass predicate에 포함된다. Android/iOS 결과 모두 두 값이 true다. |
| 18 | frame lookup 실패가 0-size default로 PASS 처리된다. | `frames.get(image.id())` 누락은 fixture 오류다. 두 Simulator는 실제 `(0,0,24,16)`을 보고한다. |
| 19 | PNG natural dimensions와 CSS layout frame에 잘못된 tolerance를 적용한다. | PNG size는 reference와 exact 비교하고 frame의 4축만 `0.5 CSS px` 이내로 비교한다. report에는 expected/actual/tolerance가 모두 들어간다. |
| 20 | viewport/DPR 차이로 Android와 iOS가 다른 단위를 비교한다. | 두 앱이 공통 `120×80 CSS px`, DPR 1 입력을 report에 남긴다. 환경별 물리 screenshot 크기는 CSS oracle 비교에 사용하지 않는다. |
| 21 | FFI mismatch가 platform UI에서 성공 메시지가 된다. | Android는 JSON status를, iOS는 native code 0과 JSON status를 함께 확인한다. 오류는 별도 실패 화면과 error log로 표시한다. |
| 22 | JSON parsing 중 native status prefix가 남아 화면 parser를 깨뜨린다. | iOS Swift가 `status=0 report=` wrapper를 먼저 제거한다. 초기 빌드에서 관찰한 parse failure를 수정했고 현재 iOS feature-on 실행이 통과한다. |
| 23 | native work가 Android main thread를 막거나 Activity 종료 뒤 UI를 갱신한다. | JNI fixture는 `bootstrapExecutor`에서 돌고 결과는 main thread에 전달된다. `activityClosing` guard와 `onDestroy` executor shutdown을 확인했다. |
| 24 | C-prefixed route 이외의 Android mode가 먼저 선택되거나 조용히 무시된다. | 최초 검토에서 C-only 필터를 발견해 모든 활성 `spinon_*` extra를 검사하도록 수정했다. `spinon_c131`와 `spinon_r10` 동시 실행이 AVD에서 각각 거부됐다. |
| 25 | C-prefixed route 이외의 iOS launch mode가 early return과 공존한다. | 모든 다른 `--spinon-*` argument를 거부하도록 수정했다. `--spinon-c131-overflow-clip`와 `--spinon-r10` 조합을 각각 iOS Simulator에서 거부했다. |
| 26 | feature-off route를 실행해도 stale on-build 앱이 설치되어 결과가 오염된다. | 양 플랫폼에서 각 feature-off build 직후 APK/app을 재설치하고 disabled log와 disabled screen을 새로 캡처했다. |
| 27 | Android 검증이 USB 실기기나 예기치 않은 device에 설치된다. | ADB 목록에는 API 37 ARM64 AVD `emulator-5554`만 연결돼 있었다. 설치/실행 명령에 해당 serial을 지정했다. |
| 28 | iOS 검증이 다른 booted simulator 또는 물리 iPhone을 사용한다. | install/launch/screenshot에 iPhone 17 Pro Simulator UDID를 지정했다. iOS 26.2 환경과 device type이 환경 파일에 기록돼 있다. |
| 29 | C14.1 report 화면을 overflow/border paint parity로 해석한다. | 현재 adapter는 report-only다. PR은 별도 C13.1 Android PixelCopy와 iOS geometry/hit/surface 결과를 연결하며 iOS Metal pixel readback 미실행을 명시한다. |
| 30 | 상태 대장이 simulator adapter를 C14.1/C14 완료로 승격한다. | `spec/STATUS.md` 체크박스는 미완료이고 live V8 document, async resource replacement/stale admission, image paint와 실기기 검증을 남은 범위로 기록한다. |

첫 iOS feature-on 확인에서는 nullable Objective-C string 처리와 wrapper가 포함된 JSON parsing 문제를 재현해 수정했다. PR 변경 검토에서 C-only route 필터 누락을 추가로 발견해 Android/iOS 모두 보강하고 cross-family negative control을 통과했다. 격리 worktree의 V8 source 경로 누락은 `SPINON_V8_DIR`를 고정 revision checkout으로 지정해 해결했다. 미해결 코드 P1/P2는 없다. 검증 결과의 한계는 위 표와 [실행 근거](README.md)에 남겼다.
