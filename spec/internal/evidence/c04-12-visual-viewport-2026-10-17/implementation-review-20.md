# C04.12 viewport 구현 실패 관점 검토 20개

검토 대상은 `environment.rs`의 viewport 계산, Rust cascade 입력, C ABI 반환 구조, Android/iOS bridge, opt-in app fixture와 이 변경의 증거다. 계획 검토 기록을 재사용하지 않고 구현을 대상으로 다시 확인했다.

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | viewport metadata가 없는 기존 모바일 기본 페이지의 폭·overview scale 회귀 | 기존 기본 metadata 테스트가 layout `980px`, page scale `0.39795917`과 새 visual 크기를 검사한다. 통과. |
| 2 | `width=device-width`에서 layout과 visual 폭이 달라지는 회귀 | 390 CSS px device-width 결과가 layout·visual 양쪽에 전달된다. 런타임 테스트와 3개 앱 행렬에서 확인. 통과. |
| 3 | 화면보다 좁은 고정 width를 선택해 보이지 않는 빈 영역이 생기는 회귀 | 기존 narrow fixed-width 테스트에서 resolved width가 390px로 확장된다. 통과. |
| 4 | 화면보다 넓은 고정 width에서 이전처럼 visual height를 layout height로 오인 | `width=500, initial-scale=1`은 layout `500×1082.0513`, visual `390×844`다. 단위·cascade·양 플랫폼 앱 fixture가 확인한다. 통과. |
| 5 | `initial-scale=0.5`가 DPR에 합쳐지거나 CSS layout 폭이 잘못되는 경우 | layout/visual `780×1688`, DPR `3`, page scale `0.5`를 각각 보존한다. 통과. |
| 6 | `initial-scale=2`에서 visual 크기를 layout 크기처럼 반환하는 경우 | layout `390×844`, visual `195×422`, page scale `2`를 검사한다. 통과. |
| 7 | DPR과 page scale을 곱해 하나의 값으로 덮어쓰는 경우 | snapshot과 C struct는 두 값을 독립 필드로 유지하고 3개 C ABI 행렬에서 기대값과 비교한다. 통과. |
| 8 | layout 높이 계산이 CSS viewport가 아니라 visual height를 사용 | resolved layout 폭과 현재 surface 물리 종횡비로 높이를 계산한다. Chrome 고정-width oracle과 정확 허용오차를 적용한 fixture가 일치한다. 통과. |
| 9 | `(min-height: 1000px)`가 visual height `844px`를 읽어 거짓이 되는 경우 | 고정 폭 case에서 cascade가 `123px` 값을 선택한다. Rust session 테스트와 Android/iOS 앱 fixture에서 확인. 통과. |
| 10 | `(max-height: 900px)`가 visual height를 읽어 잘못 참이 되는 경우 | 같은 cascade에서 `456px` 값이 나타나지 않는다. Rust session 테스트와 두 앱 fixture에서 확인. 통과. |
| 11 | width 기반 media query가 새 visual 폭에 따라 바뀌어 layout 기준과 혼합 | CSS cascade의 `CssViewport`에는 layout width/height가 들어가며 기존 width·environment projection 테스트가 통과한다. |
| 12 | 잘못된 surface width/height, DPR 또는 viewport 산술값이 cascade에 부분 게시 | 입력 검증과 계산값의 유한·양수 검사를 거쳐 snapshot 생성 전 오류를 반환한다. 기존 invalid-surface 테스트 통과. |
| 13 | page scale 경계 밖 값이나 극단적인 metadata가 새 계산에서 허용 | 기존 `0.01..=10` page-scale 제한과 100,000px layout width 제한을 유지한다. 추가 visual 값도 유한·양수 검사 대상이다. |
| 14 | height 계산 결과가 무한대/0이 되어 Stylo에 유입 | 계산 후 `is_finite`와 양수 조건을 확인한다. 코드 경로 확인; 극단 비율 표면의 별도 device 실행은 하지 않았다. |
| 15 | 중복 viewport meta 선택이 바뀌어 기존 문서와 결과가 달라짐 | 마지막 viewport 요소 우선 기존 Chrome fixture 및 테스트를 유지했다. 고정 폭 높이는 새 layout 공식에 따라 갱신됐다. 통과. |
| 16 | 지원하지 않는 viewport descriptor가 CSS cascade 오류나 잘못된 값으로 변환 | parser allowlist와 무시 규칙은 수정하지 않았다. 기존 파서 경계 테스트를 유지했다. 통과. |
| 17 | 환경 revision/cascade에 새 layout 높이가 아닌 이전 visual 높이가 남음 | `CssViewport` 생성은 resolved layout 값을 사용하고 고정-width media 결과가 새 높이를 관찰한다. 통과. |
| 18 | Rust `repr(C)`와 헤더의 field 순서 또는 타입이 달라 native에서 필드가 뒤섞임 | Rust와 C header에 layout width/height, visual width/height, DPR, page scale, scheme, revision 순서로 맞췄다. Android C++·iOS ObjC++가 빌드되고 양 앱 fixture가 ABI 값을 직접 비교한다. 통과. |
| 19 | environment setter와 Canvas 응답 중 한 경로에만 visual 값이 복사됨 | 앱 fixture가 C setter의 environment와 Canvas nested environment 양쪽을 세 행렬 각각 비교하며 revision key도 확인한다. Android/iOS에서 통과. |
| 20 | 검증이 제품 기본 경로에 섞이거나 PASS가 HTML 렌더 완료로 과장 | default 정적 라이브러리에서 C04.12 fixture 심볼이 빠지고 opt-in iOS Simulator 라이브러리에만 두 심볼이 있음을 `nm`으로 비교했다. 문서와 화면 증거는 HTML body paint·page-scale transform·native input 매핑을 지원한다고 주장하지 않는다. C04.12 상위 상태는 미완료로 둔다. |

## 검증 결과와 남은 경계

- 20개 관점에서 수정하지 않은 결함은 남지 않았다. 18–19번에서 확인 범위가 부족해 C ABI 결과 비교를 opt-in 앱 fixture에 추가하고 두 Simulator에서 다시 실행했다.
- `cargo test --locked -p spinon-runtime root_html`: 30/30 통과. `cargo test --locked -p spinon-ffi root_html --features c04-12-root-html-fixture-test`: 5/5 통과. `cargo fmt --all -- --check` 통과.
- 전체 workspace Clippy와 fixture-feature Clippy, 기본/opt-in FFI artifact gate가 통과했다.
- Android API 37.2 ARM64 AVD `emulator-5554`와 iPhone 17 Pro/iOS 26.2 Simulator의 fixture report/log가 모두 PASS다.
- 별도 landscape event, 실기기, root HTML body layout·paint, renderer page-scale transform, pinch/scroll, native pointer coordinate는 이 검토에서 검증하지 않았다.
