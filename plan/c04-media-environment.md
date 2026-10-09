# C04.7 · CSS media 환경 입력

## 목표

CSS cascade가 컴파일 타깃의 기본값에 기대지 않도록 viewport와 함께 색상 모드, primary pointer, 전체 pointer capability를 한 요청의 명시적 입력으로 받는다. 이번 구현은 Stylo 0.22 cascade 입력 전달과 고정 fixture 비교까지 다룬다.

## 범위

- `prefers-color-scheme: light | dark`
- `pointer: none | coarse | fine` 및 `hover: none | hover`
- `any-pointer: none | coarse | fine` 및 `any-hover: none | hover`
- 기존 C04.6 profile의 경계를 유지하는 신규 `FlexMediaEnvironmentV1` cascade profile과 `compute_flex_media_environment_cascade`
- `@media`에서 위 feature만 허용하고, Stylo가 알더라도 viewport 단위·reduced-motion·기타 feature는 이 profile에서 거부
- malformed `@media` parse diagnostic도 fail-closed 처리
- media environment를 `CssViewport`에 포함하고 기존 `EnvironmentRevision`과 함께 출력 snapshot에 보존
- 모순된 primary/all pointer 입력은 cascade 전에 오류로 거부
- desktop/mobile × light/dark Chromium 비교 fixture와 Stylo computed `display` 비교
- 모바일 OS 설정을 읽는 코드, 환경 변경 관찰자, 자동 revision 증가, 재계산 scheduler, GPU 반영은 범위 밖

## 입력 계약

요청 소유자는 viewport 또는 media environment가 바뀔 때 같은 `EnvironmentRevision`을 증가시킨다. 이 slice는 revision을 발급하거나 동시 입력을 조정하지 않는다. `CssViewport`의 기본 fixture 값은 결정적인 desktop/light/fine/hover 조합이다. 앱 runtime은 명시적으로 수집한 모바일 환경을 전달해야 한다.

Primary pointer capability는 `pointer`와 `hover`에, 전체 capability 집합은 `any-pointer`와 `any-hover`에 각각 전달한다. Primary coarse/fine는 전체 집합에도 존재해야 한다. primary hover는 primary pointer가 있고 전체 집합에도 hover가 가능해야 한다. 전체 hover는 coarse 또는 fine 장치가 있을 때만 허용한다. 전체 집합은 coarse와 fine을 동시에 포함할 수 있다.

## 비교 기준

- 기준 구현: 같은 버전의 Chromium DevTools Protocol media emulation 및 실제 `matchMedia()`·`getComputedStyle()` 관찰
- 고정 viewport: `390×844` CSS px, device scale factor `3`
- 네 측정 case: desktop/mobile 각각 light/dark
- mobile은 CDP touch emulation으로 primary coarse/no-hover를 요청하고, 결과 `matchMedia()`가 fixture 예상과 일치하지 않으면 캡처를 실패시킨다.
- Rust 결과는 고정 Chromium reference와 각 probe의 computed `display` 문자열을 정확히 비교한다.
- 혼합 pointer와 pointer 없음은 이 Chromium emulation에서 만든 실측 결과로 주장하지 않는다. Stylo 0.22의 고정 media feature 구현에 맞춘 별도 mapping/invariant 테스트만 둔다.

## 통과 기준

1. 네 Chromium case에서 요청한 scheme/pointer 상태가 `matchMedia()`의 관찰값과 일치한다.
2. Rust Stylo cascade의 probe computed 값이 고정 reference와 모두 일치한다.
3. 잘못된 primary/all 조합은 부분 computed snapshot 없이 `InvalidMediaEnvironment`를 반환한다.
4. snapshot은 입력한 media environment와 environment revision을 그대로 보존한다.
5. 기존 CSS profile의 fixture와 사용 가능한 범위는 변경하지 않는다.

## 아직 구현하지 않는 항목

OS color scheme/touch/hover capability 수집, viewport와 환경 revision의 단일 소유자, stale cascade 취소, async style worker, media query에 따른 runtime 재계산, 앱 수준 dark-mode 갱신, `prefers-reduced-motion`, contrast/forced-colors, orientation/resolution, 브라우저 전체 media feature 및 GPU 표시 일치.
