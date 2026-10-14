# C14.1 PR 통합 적대 검토

## 범위

PR #154 `feat(c14): HTML 이미지 자연 크기 입력 경계 연결`의 실제 branch diff, 공개 crate 경계, revision 전파, fixture provenance, Android build/startup, 문서 주장과 현재 GitHub 상태를 검토했다. 계획의 20개 항목과 구현 코드의 20개 실패 관점을 다시 세지 않고, PR을 다른 crates·도구·플랫폼·병합 게이트에 연결할 때의 실패 조건을 확인했다.

| # | 독립 PR 관점 | 확인 결과와 판정 |
| ---: | --- | --- |
| 1 | core revision type가 다른 crate에서 이름을 찾을 수 있는가 | `spinon_core::IntrinsicSizeRevision`으로 re-export되고 layout snapshot API가 공개된다. workspace compile과 Android target build가 모두 해결했다. |
| 2 | 기존 style-to-layout 호출자가 새 API 때문에 source 수정이 필요한가 | 기존 `compute_runtime_style_layout` 시그니처를 유지하고 측정 입력 overload를 추가했다. 모든 기존 호출부가 그대로 컴파일된다. |
| 3 | 기존 LayoutEngine trait 구현자가 새 필수 method를 구현해야 하는가 | 추가 계산 entry는 `TaffyLayoutEngine` inherent method이며 trait 요구사항을 늘리지 않았다. 기존 engine 호출부 전체가 통과했다. |
| 4 | old revision constructor를 쓰는 결과에 초기값이 일관되게 들어가는가 | `LayoutInputRevision::new`와 `RuntimeRenderKey::new`는 measurement 축을 `INITIAL`로 초기화한다. 비-C14 경로의 기존 revision 의미를 보존한다. |
| 5 | runtime key equality에서 새 revision 축이 누락되는가 | key의 derived equality는 새 private field도 비교한다. 같은 document/style/environment에서 measurement revision만 바뀐 경우 key가 달라진다. |
| 6 | static S04 key가 layout 결과의 실제 measurement revision을 읽는가 | `build_s04_static_render_snapshot`은 `LayoutOutput.revision`에서 값을 복사한다. 입력 축을 임의 상수로 재구성하지 않는다. |
| 7 | static admission이 current measurement revision과 비교하는가 | `CurrentLayoutInputs`의 전체 revision tuple에서 intrinsic 축을 검사하고 불일치 시 `LayoutIntrinsicSizeRevision`으로 거부한다. |
| 8 | 일반 runtime snapshot builder가 measurement revision을 누락하는가 | `build_runtime_render_snapshot`이 current input revision을 key에 복사한다. 회귀 검사가 값 보존을 확인한다. |
| 9 | stacking snapshot builder만 별도 key 생성으로 revision을 잃는가 | stacking 경로도 동일한 축을 복사하고 전용 회귀 검사를 가진다. 일반 경로 결과에 의존하지 않는다. |
| 10 | document revision rebind 때 measurement key가 기본값으로 재설정되는가 | `RuntimeRenderSnapshot::rebind_document_revision`이 기존 key 축을 다시 붙인다. 공유 box storage와 revision 보존 검사가 통과한다. |
| 11 | 신규 오류가 style adapter 외부로 전파되며 진단이 사라지는가 | `StyleLayoutError`가 `LayoutError`를 보존하고 node ID가 포함된 오류를 반환한다. API 호출부에서 임의 success fallback은 추가되지 않았다. |
| 12 | layout source가 runtime document와 tree source 사이에서 잘못 합쳐지는가 | 공통 `LayoutSourceRevision`으로 layout engine과 HostDocument adapter가 같은 source 타입을 비교한다. 기존 tree 입력 생성자는 그대로 남는다. |
| 13 | Rust public exports에 모듈 경로 의존이 강제되는가 | `spinon-layout` 루트에서 snapshot/data type을, `spinon-style-to-layout` 루트에서 overload를 다시 내보낸다. 내부 module 경로를 소비자에게 요구하지 않는다. |
| 14 | PR이 Cargo dependency·lockfile drift를 끌어오는가 | Rust dependency와 Cargo lockfile 변경은 없다. 새 CSS reference test도 기존 Node built-in test runner만 사용한다. |
| 15 | 고정된 Android toolchain에서 새 layout code가 ARM64에 compile되는가 | Android 17/API 37 arm64 AVD용 release build에서 변경된 `spinon-ffi`, layout crates, WGPU smoke library와 APK 빌드가 성공했다. 고정 V8 revision과 NDK 27.1.12297006을 사용했다. |
| 16 | 설치된 APK가 단순 assemble에 그치지 않고 AVD에서 로드되는가 | `emulator-5554` 설치·activity start 후 PID와 `SPINON_BOOTSTRAP_RESULT`를 확인했다. JNI shared library load와 기본 V8 bootstrap이 성공했다. |
| 17 | 실기기 결과가 emulator 검증으로 잘못 섞이는가 | `adb devices -l`에는 `emulator-5554` 하나만 있었고 model은 `sdk_gphone16k_arm64`다. 실기기 serial은 사용하지 않았다. |
| 18 | 원래 ADB 연결 오류가 현재 PR의 simulator 결과와 혼동되는가 | ADB client는 동작했고 실패 원인은 존재하지 않는 `.avd` 디렉터리를 가리키는 stale `spinon_api37_2.ini` 및 QEMU2의 미지원 `arm` profile이었다. 유효한 `spinon_api37_1_compat16k` arm64 AVD로 복구한 기록을 추가했다. |
| 19 | Chrome 관찰 fixture가 변경된 입력이나 캡처와 조용히 어긋나는가 | 검토에서 자동 digest 검사가 없음을 발견해 C14 test를 추가했다. HTML·inventory·screenshot hash, Chrome 버전, 단일 DPR 한계와 세 관찰 frame을 고정하고 CSS reference suite 415개가 통과했다. |
| 20 | GitHub green status가 실제로 존재하는데 로컬 결과와 혼동되는가 | PR은 `MERGEABLE`이지만 `statusCheckRollup=[]`, `gh pr checks`는 “no checks reported”, review request와 review도 없다. 로컬 통과를 GitHub CI/승인으로 부르지 않는다. |

## 결론

검토에서 찾은 PR 통합 결함은 Chrome fixture provenance를 자동으로 검증하는 테스트가 없다는 점이었다. 위의 새 무결성 테스트로 수정했다. 다른 소스·revision·기존 호출자·Android build 경계에서 미해결 P1/P2 코드는 발견하지 않았다.

PR은 GitHub 검토 및 CI status가 아직 없다. Android AVD 결과는 build/startup smoke이며 C14 runtime 통합 증거가 아니다. 앱에 intrinsic-size snapshot 공급 경로, decoder/U05 owner, C14 simulator fixture가 없으므로 문서와 PR은 C14.1 및 C14를 진행 중으로 유지한다.
