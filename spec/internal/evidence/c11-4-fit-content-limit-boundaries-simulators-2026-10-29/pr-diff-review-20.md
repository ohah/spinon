# C11.4 `fit-content()` limit 경계 PR diff 검토 20개

2026-10-29 · 구현 후 변경 diff만 대상으로 별도의 실패 경로를 확인했다. 범위는 Rust runtime/FFI, Android/iOS fixture route, capture/assertion, C11.4 계획·내부 계약·상태 기록과 실행 근거다.

| # | diff 위험 | 확인 결과 |
| ---: | --- | --- |
| 1 | 새 CSS 동작이 일반 author admission에 섞이는가 | 새 코드는 기존 opt-in Grid fixture route에 연결했고 일반 profile allowlist는 넓히지 않았다. |
| 2 | Cargo feature gate 일부가 빠져 기본 build가 fixture 구조체를 내보내는가 | observation export와 runtime fixture 모듈은 기존 `c11-4-grid-track-sizing-fixture-test` gate를 따른다. |
| 3 | 플랫폼 build flag와 fixture FFI source gate가 서로 다른가 | Android/iOS 실행은 기존 C04 fixture runtime opt-in을 사용하고 iOS pixel readback flag를 별도로 명시했다. |
| 4 | scene routing이 이전 fit-content 또는 spanning source를 가리는가 | dispatch 등록 순서와 exact ID 확인으로 세 scene만 새 source에 매핑한다. |
| 5 | report 필드가 다른 C11.4 diagnostic을 덮어쓰거나 누락시키는가 | 독립 `fitContentLimitBoundaries` key를 추가하고 기존 필드 및 trace report 경로를 유지했다. |
| 6 | trace snapshot clone·초기화 지점 중 새 필드가 빠지는가 | diagnostics clone 및 `None` 생성 지점을 모두 갱신했다. |
| 7 | scene 목록만 늘리고 selector·catalog도 갱신하지 않는가 | capture selector 세 ID, 전체 545 scene count, focused fixture boundary assertion을 함께 갱신했다. |
| 8 | 기존 테스트가 이전 전체 scene 수에 고정되어 회귀하는가 | capture module 경계 파일의 542 기준을 새 545 기준으로 동기화했고 focused 경계 검증 로그가 통과했다. |
| 9 | Android만 세 scene 일부를 제외하거나 다른 판정식을 쓰는가 | Android allowlist와 기대 limit/row 매핑이 세 scene을 모두 포함한다. |
| 10 | iOS만 세 scene 일부를 제외하거나 다른 판정식을 쓰는가 | Swift scene matcher와 fixture route가 세 scene 및 같은 기대값을 사용한다. |
| 11 | Java/Swift에서 raw growth limit을 fit-content 인수로 혼동하는가 | 양쪽 matcher가 row base와 raw 100px growth limit을 별도 확인한다. |
| 12 | screenshot 표본이 경계 밖 좌표나 child가 없는 영역을 가리키는가 | 40/30px case는 각각 item 바깥 child 내부의 표본을 포함하고 root 바깥 좌표를 별도 확인한다. |
| 13 | 화면 확인이 runtime frame/trace의 동일 revision과 분리되는가 | capture route가 draw/presentation identity를 요구하고 양 플랫폼의 표본을 결과 JSON에 연결한다. |
| 14 | 표본 결과가 Android 실기기·iOS compositor 이후 화면으로 잘못 서술되는가 | 문서는 AVD software renderer, iOS compositor 제출 전 Metal 표본이라는 한계를 명시한다. |
| 15 | 근거 폴더가 실패 실행을 최종 성공으로 섞는가 | 실패 시도는 별도 디렉터리로 남기고 README가 원인·수정·최종 실행을 분리한다. |
| 16 | 빌드·정적 분석 근거가 세션 외부로 빠져 재현이 안 되는가 | Android/iOS build, fmt, Clippy, diff, focused Rust/Node 로그를 evidence 디렉터리에서 연결했다. |
| 17 | artifact scan이 전체 APK/IPA 제외를 과장하는가 | 근거와 문서는 확인한 macOS ARM64 static archive에만 한정하며 `nm` 제한도 공개한다. |
| 18 | C11.4나 공식 완료 수를 부분 fixture 근거만으로 올리는가 | `spec/STATUS.md`와 plan은 미완료 및 공식 `72/213`을 유지한다. |
| 19 | 규범 문서 설명과 구현 기대값이 충돌하는가 | W3C §11.5.1에 맞게 argument는 base-size 분배를 제한하고 finite growth limit은 max-content일 수 있다고 수정했다. |
| 20 | 구현 결과 화면과 로그·source hash·scene oracle 링크가 끊기는가 | README에서 result JSON, simulator 정보, PNG hash, build/validation log 및 precomparison을 상대 경로로 연결했다. |

문서와 캡처 도구를 포함한 전체 변경은 최종 diff에서 다시 검사한다. 이 검토도 일반 CSS 지원, WPT harness 전체 통과 또는 공식 완료 판정을 추가하지 않는다.
