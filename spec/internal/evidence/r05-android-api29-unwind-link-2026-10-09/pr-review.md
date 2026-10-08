# PR 통합 변경 검토 · Android API 29 unwind 링크

대상: PR #82, base `2b0b0218d8a0e0ce639a96244774fce9d456b424`, 구현 commit `9ff756ebaef6debe0c7d5a4a99bcf336575e7eef`.

계획 검토와 구현 검토를 반복하지 않고, 변경 묶음·상태 대장·PR 본문·첨부 증거의 경계에서 서로 다른 실패 경로를 확인했다.

| # | 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | PR base가 오래되어 최신 main의 변경을 누락 | 원격 main과 PR base가 모두 `2b0b0218d8a0e0ce639a96244774fce9d456b424`로 일치한다. |
| 2 | 작업 브랜치에 관련 없는 사용자 변경이 섞임 | PR diff 158개 경로를 확인했다. 구현 스크립트, R05 계획·상태, 실행 로그·캡처·검토 자료로 범위가 한정돼 있다. |
| 3 | 제목·본문·라벨이 저장소 규칙과 불일치 | 제목 접두어만 영어이며 나머지는 한글이다. `bug`, `area: android`, `area: docs`가 지정됐다. |
| 4 | API29 수정으로 R05.3 전체를 완료 처리 | 상태 대장은 R05.3을 미완료로 유지하고 API30–33과 물리 입력 경계를 남겼다. |
| 5 | 공개 JS 동작 계약을 변경했지만 명세 누락 | PR diff는 네이티브 링크 입력과 내부 계획·증거만 바꾸며 공개 JS API는 변경하지 않는다. |
| 6 | NDK/V8 버전 또는 ABI가 암묵적으로 바뀜 | NDK 27.1.12297006 및 고정 V8 revision은 그대로이며 Android AArch64 산출물에 한정된다. |
| 7 | 복수 host prebuilt 중 의도하지 않은 디렉터리를 선택 | 이번 macOS NDK에는 `darwin-x86_64` host prebuilt 하나만 존재했다. 변경 archive도 그 고정 NDK root 아래에서 선택됐다. 다른 host 패키지는 이번 환경에서 검증하지 않았다. |
| 8 | Clang resource archive 후보 없음·중복 시 임의 경로 사용 | 후보 없음과 중복 후보 부정 대조가 link 전에 실패하며 기존 출력 hash를 보존했다. |
| 9 | 잘못된 archive를 필요한 구현으로 오인 | 고정 NDK `llvm-nm`이 `_Unwind_Resume` 정의를 확인한다. 정의가 없는 모의 archive는 거부됐다. |
| 10 | `llvm-nm` 실행 실패가 pipeline에서 숨겨짐 | 스크립트는 `set -euo pipefail`이며 실행 파일·symbol 검사 실패가 선행 오류로 전달된다. |
| 11 | 정적 archive 순서가 참조 해소를 놓침 | 최종 link 순서는 기존 V8 C++ archive와 `libc++abi.a` 다음에 `libunwind.a`를 둔다. 실제 최종 link가 성공했다. |
| 12 | libunwind 코드가 DSO 공개 심볼로 노출 | `--exclude-libs,libunwind.a`가 적용됐고 APK DSO에서 exported `_Unwind_*`가 0개다. |
| 13 | dynamic import 또는 `DT_NEEDED`가 남아 구형 loader에서 재실패 | APK DSO의 unresolved `_Unwind_*`와 libunwind `DT_NEEDED`가 모두 0이며 API29 cold start를 확인했다. |
| 14 | 링크 수정이 16KB page 정렬을 손상 | `PT_LOAD` 정렬 `0x4000`, GNU_RELRO 끝 `0x2dd4000`, APK `zipalign -P 16` 결과가 기록돼 있다. |
| 15 | Gradle이 이전 DSO를 재사용해 수정 성공으로 오판 | 빌드 APK, API29·34 AVD 설치본, API36 기기 설치본의 APK hash를 대조했다. |
| 16 | API29 native load만 통과하고 JS bootstrap 또는 fallback이 실패 | API29에서 bootstrap, API35 guard, 분리 process의 GLES present-fence/FrameTimeline draw와 색상 변경을 확인했다. |
| 17 | API29 성공을 API30–34 전체 지원으로 확대 | API29와 API34는 별도 경계 증거로 기록하며 API30–33은 미측정이다. |
| 18 | API36 실기기 회귀를 API29 증거 또는 실터치로 오해 | 실기기 결과를 API36 회귀로 분리했다. 입력 13회는 ADB synthetic이며 raw direct touch positive sample은 없다. |
| 19 | fence callback을 VSync·화면 표시 시간 또는 성능으로 해석 | `target_vsync_id=-1`을 보고하고 latency, VSync, scanout 및 성능 수치를 주장하지 않는다. |
| 20 | 첨부·원본·체크 상태가 서로 다른 실행을 가리키거나 Release 고지를 완료로 오해 | PR 첨부는 API36 캡처이며 본문 APK hash가 증거와 일치한다. NDK NOTICE/license의 Release 패키징은 미검증으로 남겼다. 원본 logcat의 끝 공백은 무결성 보존을 위해 그대로 두었고, 코드·계획·명세 파일은 `git diff --check`를 통과했다. |

## 결과

통합 경계에서 코드 차단 결함은 찾지 않았다. 현재 PR은 mergeable이며 GitHub에 보고되는 CI check는 없다. 원본 실행 파일들의 checksum manifest는 검증됐고 GitHub Pages 또는 Tailnet 공개 배포는 하지 않았다.
