# Android 증거 적대 검토 20개

검토 대상은 Android API 37.2에서 실행한 Chrome 관측 sentinel 세 장면과 그 실행 증거다. Rust 구현 검토 20개와 PR 전체 diff 검토 30개를 대신하지 않는다.

1. **Chrome record 식별:** `chrome-observation.json`의 index가 각각 0, 2471, 1222이고 scene JSON과 모든 필드가 일치하는지 검사했다. 세 건 모두 일치한다.
2. **early/adjacent/separated 범위:** fit 위치가 각각 `[1,2]`, `[3,4]`, `[1,6]`으로 기록되어 세 pair 위치군이 모두 포함된다.
3. **cap 위치 귀속:** 각 관측의 trackList에서 두 cap 문자열이 authored fit 위치와 대응한다. `0px/0%`, `20%/10px`, `20%/120%`가 위치 순서대로 보존된다.
4. **alignment 분기:** early case는 `start`, 나머지는 `stretch`를 사용해 두 alignment가 들어 있다.
5. **topology 분기:** `span2Six-forward`와 `span2Span3Six`의 forward/reverse가 모두 들어 있다.
6. **reverse DOM 순서:** observation 2471은 DOM item 순서 `D,C,B,A`와 각자의 authored row `5,4,3,2`를 기록하고, Android 입력 HTML도 이를 그대로 담는다.
7. **명시 행 유지:** 세 HTML 모두 row 1에 10px empty item을 두며 각 Chrome record의 첫 item도 같은 explicit row다.
8. **6-member track list:** scene trackList 문자열이 Android 입력 HTML의 `grid-auto-rows`와 정확히 일치한다.
9. **payload 경계:** HTML의 A–E payload는 사전 비교 fixture 문구만 사용하며 임의 텍스트나 fixture route marker를 추가하지 않는다.
10. **제품 route:** 모든 실행 로그에 `SPINON_ROOT_HTML_DEFAULT_APP_ROUTE source=index.html`가 있다. opt-in fixture FFI나 별도 fixture route를 거친 결과로 오인하지 않는다.
11. **layout 상태:** 각 로그에 `layout=ready`가 있고 실패 status는 없다.
12. **box 개수:** 5개 text item 장면은 13 boxes, 4개 장면은 12 boxes다. 관찰 item 수와 DOM/payload 구조에 맞는다.
13. **root frame:** 세 Android 로그의 CSS root frame은 `411.42856×220`으로 같고 서로 다른 scene에서 높이가 바뀌지 않았다.
14. **draw 종료 상태:** 각 로그에 `SPINON_C0410_DRAW status=0 presented`가 남아 layout-ready 로그만 있고 화면 제출이 빠진 경우가 아니다.
15. **text 제출 수:** early case의 `runtime_text_count=5`, 나머지의 `4`가 item 수와 일치한다.
16. **스크린샷 파일:** 세 PNG signature가 맞고 크기는 각각 1080×2400이다. sha256은 실행 로그 옆의 PNG를 대상으로 기록했다.
17. **화면 관찰:** 세 스크린샷에서 앱 화면 상단 좌측에 각 입력의 Grid/text 결과가 보이고 launcher나 빈 surface가 아니다.
18. **렌더러 한계:** 실행은 Vulkan Goldfish GFXStream/SwiftShader다. 이를 Android hardware GPU나 기기 호환성 결과로 넓혀 쓰지 않았다.
19. **asset 출처/복원:** 각 HTML SHA와 APK SHA를 `apk-source-hashes.txt`에 기록했다. 테스트 뒤 tracked bootstrap 파일은 원래 SHA `e7ecee32…`/`b87b5388…`로 돌아왔고 현재 HEAD의 파일 hash와 같다.
20. **AVD 격리와 정리:** 기존 AVD는 read-only/no-snapshot-load/no-snapshot-save로 열고 userdata/cache를 임시 경로로 분리했다. 실행 후 emulator를 종료하고 임시 userdata/cache와 debug APK 출력 디렉터리를 제거했다. 기존 snapshot/profile과 공유 SDK를 지우지 않았다.

### 남은 한계

이 검토는 세 대표 Android sentinel만 다룬다. 3,840-scene 전체 native 실행, Android row-sizing trace readback, PixelCopy, hardware GPU, 실기기, iOS/Android pixel parity는 입증하지 않는다. 이전 AVD boot 실패 기록은 별도 파일에 보존한다.
