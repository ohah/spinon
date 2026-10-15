# C13.3 computed overscroll simulator matrix 구현 검토 20개

대상은 현재 작업 브랜치의 Rust host/state/FFI, Android/iOS simulator adapter, simulator 실행 기록이다. 계획 문서의 사전 검토와 별도로 실제 구현·실행 경로를 검토했다.

1. **시나리오 ID의 범위 밖 값** — Rust `TryFrom<i32>`는 0·1·2만 매핑하고 나머지는 오류를 반환한다. 기본 시나리오로 조용히 대체하지 않는다.
2. **Android extra의 잘못된 타입** — `Bundle` 값이 `Integer`가 아니면 `-1`이 되어 host 생성 전에 실패한다. `getIntExtra`의 기본값으로 잘못된 값을 숨기지 않는다.
3. **iOS 중복/비정수/범위 밖 인수** — 단일 정수만 허용하고 나머지는 `-1`로 거부한다. scenario 인수 단독 전달도 fixture route 조건 검사에서 차단한다.
4. **여러 fixture route 충돌** — Android와 iOS 모두 C13 route 중복을 검사한다. policy ID가 다른 route 선택을 덮어쓰지 않는다.
5. **기본 빌드 ABI 노출** — Cargo feature, C header define, JNI/Objective-C bridge의 조건부 컴파일을 대조했다. feature-off archive에 C13.3 constructor symbol이 없고 opt-in positive control에는 있다.
6. **계산 style 우회** — scenario는 fixture script prelude의 ID만 전달한다. host는 cascade 뒤 owner snapshot의 computed policy를 검증하고, fixture가 router policy를 직접 설정하지 않는다.
7. **ancestor policy 오염** — inner owner만 지정한 policy와 비교하고 root·outer는 `auto/auto`를 검증한다. target policy가 ancestor까지 복사되는 경로를 차단한다.
8. **없는 scroll owner 또는 잘못된 owner identity** — 준비된 root/outer/inner NodeId와 각 scroll range를 조회해 확인한다. 누락은 fixture 준비 실패로 끝난다.
9. **boundary seed의 잘못된 좌표** — seed delta `(86,106)`을 actual hit route에 전달하고 남은 delta와 최종 inner offset을 검증한다. 범위 초과·미소비 seed는 거부된다.
10. **zero scenario의 숨은 seed** — `auto-zero`는 offset `(0,0)`과 첫 draw revision 0을 확인한다. boundary 상태를 이전 session에서 재사용하지 않는다.
11. **touch-action 혼입** — scene point에서 계산된 touch action이 `auto`인지 확인해 overscroll-only 차이를 유지한다. 이 행렬로 non-default touch-action 동작을 주장하지 않는다.
12. **준비 snapshot과 presented frame 분리** — 두 앱은 fixture report만으로 입력을 열지 않는다. 첫 successful draw의 revision과 offsets가 report와 일치해야 `SPINON_C133_READY`를 낸다.
13. **iOS presentation 재설정** — 공통 환경 갱신이 sequence를 바꾸며 fixture snapshot을 지우던 경로를 찾아 현재 snapshot을 새 sequence에 재게시하도록 수정했다. 최신 iOS 재빌드 첫 draw가 seed와 일치했다.
14. **surface resize 뒤 stale state** — resize가 성공한 경우 새 sequence에 gesture snapshot을 재게시하고 실패한 resize 결과는 그대로 전달한다. 재빌드 뒤 실제 surface에서 seed revision을 확인했다.
15. **stale publish 순서** — scene sequence와 presentation publish 결과를 검사한다. 오래된 sequence publish는 ready가 되지 않으며 stale 경로는 오류로 돌아온다.
16. **Android 좌표계와 slop** — surface width 기반 CSS scale과 native `scaledTouchSlop`을 로그에서 분리했다. 실제 AVD run은 slop 이후의 `MotionEvent`만 gesture 이동으로 분류했다.
17. **iOS 좌표계와 system slop** — `UIPanGestureRecognizer` translation을 surface scale로 CSS 좌표에 변환하고 system slop 값은 임의 수치로 보고하지 않는다. 실제 simulator swipe 로그에서 delta와 offset을 대조했다.
18. **축별 containment** — X-only, Y-only, diagonal의 inner/outer/root offset을 두 플랫폼에서 독립적으로 비교했다. X containment 중 Y chaining은 유지되고 X ancestor 이동은 0이다.
19. **revision 및 no-op 입력** — containment horizontal은 `changes=0`, scroll revision 1 유지, screenshot 변경 0개였다. 실제 입력 callback을 offset commit으로 잘못 집계하지 않는다.
20. **bounded input queue와 코드 경계** — 64 regular/8 terminal 용량과 기존 queue contract를 유지했고 scenario/state 책임을 하위 모듈로 분리했다. host 파일은 500줄 이내이며 feature-off 경계도 재검사했다.

## 판정

구현에서 발견한 주요 회귀는 iOS presentation invalidation 뒤 준비된 C13.3 snapshot이 새 frame sequence에 재게시되지 않던 점이다. presentation update/resize에서 snapshot 복구를 추가하고 Android/iOS 최신 앱을 다시 빌드해 첫 draw 및 containment 입력 smoke로 확인했다. 현재 검토에서 남은 구현 P1/P2는 없다. 전체 simulator/runtime 범위는 C13.3 미완료 조건을 유지한다.
