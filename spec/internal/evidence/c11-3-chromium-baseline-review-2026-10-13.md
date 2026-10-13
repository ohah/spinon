# C11.3 Chromium 기준 버전 교체 검토 · 2026-10-13

## 검토 대상

[C11.3 구현 계획](../../../plan/c11-3-template-repeat-named-lines-areas.md)의 기준 브라우저를 Chrome `154.0.8037.98`에서 공식 Chrome for Testing `154.0.8037.92`로 바꾼 결정과 새 기준의 재현 경계를 검토했다. 기존 계획 검토 20개 관점과 별도이며 CSS 구현이나 WPT 결과를 검증하지 않는다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검토 결과와 처리 |
| --- | --- | --- |
| 1 | 원래 고정한 `.98`이 설치되어 있지 않은데 시스템 Chrome으로 조용히 대체 | 설치된 시스템 Chrome 155를 쓰지 않고 공식 CfT에서 직접 `.92`를 받아 별도 실행했다. |
| 2 | CfT stable 목록에 없다는 이유만으로 `.98`이 존재하지 않는다고 과장 | `.98`은 계획 작성 시 확보했던 Chrome 빌드로 기록하고, 이번 검증에서만 공식 CfT 배포 아티팩트를 찾지 못했다고 한정했다. |
| 3 | 두 버전이 모두 154라 같은 소스라고 가정 | 실제 `.92`의 `Browser.getVersion` revision `@334b65d254ccc35df4fca82706d1753227b01039`을 확인했다. 기존 `.98`의 `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`와 다르므로 별도 기준으로 취급한다. |
| 4 | 이전 C11 fixture reference를 새 `.92` 기준 결과처럼 재사용 | 기존 C11.1–C11.2.3 산출물은 덮어쓰지 않고, C11.3 새 실행은 `.92` 실행 파일 hash를 포함한 별도 reference ID로 만든다. |
| 5 | 브라우저 자동 탐색이 사용자가 설치한 다른 Chrome을 실행 | C11.3 실행 도구는 절대 경로를 받으며 `Browser.getVersion`의 product·revision을 기대값과 대조한다. |
| 6 | macOS arm64 실행 파일을 받았다고 기록하지만 실제로 x64 binary 실행 | CfT manifest의 `mac-arm64` artifact를 고정하고 실행 파일 경로·SHA-256·호스트 architecture를 결과에 저장한다. |
| 7 | CfT manifest의 정수 revision을 CDP Git revision과 같은 값으로 기록 | CfT `r1689415`와 CDP `@334b65…`를 서로 다른 필드로 남긴다. |
| 8 | 다운로드 URL만 남기고 받은 파일이 바뀌어도 재현 불가 | zip과 추출된 Chrome 실행 파일의 SHA-256을 실행 근거에 기록한다. |
| 9 | WPT source checkout이 plan의 pinned revision과 다름 | 실행 전 `build/wpt-c124-source`가 `c999c58338ee1d223df5ad62e48be1202ced0d37`인지 확인한다. |
| 10 | 고정 WPT 경로의 파일 내용이 계획 작성 이후 바뀜 | 다섯 repeat 후보 원문 각각을 계획의 SHA-256과 대조하고 불일치하면 실행을 중단한다. |
| 11 | WPT manifest 자동 갱신이 선택 테스트의 입력 집합을 변경 | pinned checkout manifest를 사용하고 manifest 다운로드·갱신을 끈 실행 명령과 로그를 보관한다. |
| 12 | ChromeDriver 버전 불일치가 브라우저 실행·테스트 결과를 흔듦 | WPT runner에는 같은 CfT 버전의 ChromeDriver를 절대 경로로 지정하고 양쪽 버전을 로그에 남긴다. |
| 13 | WPT Chrome PASS를 Spinon 지원으로 표현 | 실행 결과는 Chrome oracle로만 분류하고 Spinon WPT 적합성이나 구현 통과로 승격하지 않는다. |
| 14 | testharness의 subtest 실패를 파일 단위 PASS에 감춤 | WPT report에서 test와 모든 subtest의 상태·예상 상태·결과를 각각 보존한다. |
| 15 | 선택 WPT가 원격 지원 파일에 의존해 네트워크 변화로 결과 변동 | pinned WPT checkout의 harness·support 파일을 로컬 테스트 서버에서 제공하고 요청 누락·오류를 로그에 남긴다. |
| 16 | WPT runner와 CSS fixture capture가 서로 다른 브라우저 binary를 사용 | 두 도구 모두 동일한 절대 Chrome 경로와 full product/revision assertion을 사용한다. |
| 17 | headless 실행을 사용자 화면 동작이나 성능 자료로 해석 | 결과는 CSSOM·layout oracle로만 사용하고 GUI 렌더링·성능 근거라고 부르지 않는다. |
| 18 | locale·viewport·DPR 차이가 CSS Grid resolved value에 섞임 | 기존 계획의 viewport 320×240 CSS px, DPR 1·2, `en-US`/UTC, light, horizontal-tb/LTR 환경을 고정하고 reference에 저장한다. |
| 19 | Grid line-name resolved serialization을 사전 확인 없이 원문 선언과 동일시 | 고정 CfT `.92`에서 별도 probe를 실행했다. `getComputedStyle().gridTemplateColumns`는 expanded track과 합쳐진 line-name groups를 직렬화하며 inline declaration은 repeat 원문을 보존한다. 두 관측값을 별도 필드로 저장한다. |
| 20 | 브라우저 버전 교체로 제품 지원 범위나 이전 결정까지 암묵 변경 | C11.3 plan의 oracle만 새로 고정한다. CSS subset·profile·WPT revision·release/contract version은 바꾸지 않고 Tailnet 계획 페이지도 같은 버전으로 동기화한다. |

## 결정과 남은 검증

공식 CfT stable 배포 자료에는 Chrome `154.0.8037.92`가 `r1689415`로 등록되어 있고 mac-arm64 binary를 받을 수 있다. 받은 binary는 `Chrome/154.0.8037.92`, CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`을 보고했다. 따라서 C11.3 reference는 이 artifact를 새 고정 oracle로 사용하고 이전 `.98` 자료와 비교 기준을 혼합하지 않는다.

이 검토는 기준 버전 선택만 다룬다. pinned WPT 후보 실행, C11.3.1 fixture capture, Rust 변경, Android/iOS runtime, 표면 픽셀, 구현 실패 관점 검토는 아직 수행하지 않았으며 구현 완료 근거가 아니다.
