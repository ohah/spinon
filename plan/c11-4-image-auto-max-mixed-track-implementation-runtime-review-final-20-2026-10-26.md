# C11.4 image auto/max mixed-track 구현/runtime 검토 final 20

**검토 대상:** `cac4e33950aa55e8bb8e7ec1f81bf790316ba7ac` · Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator run `c114-image-auto-max-20261026T020007Z-cac4e339-final05`.

**판정:** 구현/runtime blocker 없음. `source`와 intrinsic input의 JSON 검사도 양 플랫폼에서 strict number/string type 및 finite 값 검사로 바뀌었고, 최신 앱 빌드와 runtime matcher가 모두 통과했다. PR head `9b3101dca186ee55016b2023c9610186b4df9ecb`에서 Agent Browser로 final05 스크린샷 두 링크가 정상 로드되는 것도 확인했다. 다만 malformed JSON type을 넣는 native matcher negative test는 실행되지 않았다.

| # | 실패 관점 | 판정 | 확인 결과 |
| --- | --- | --- | --- |
| 1 | Oracle과 기대값 독립성 | 통과 | CSS Grid 2025 CRD와 독립 산술을 runtime 실행 전에 고정했다. [사전 비교](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/README.md) · [기대값 JSON](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/spec-derived-expectations.json) |
| 2 | PNG 리소스 출처와 natural size | 통과 | 고정 PNG의 bytes/hash와 natural `24×16`을 기록하고 adapter에서 decode한 결과의 치수를 확인한다. [adapter](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L133) |
| 3 | Grid geometry 산술 | 통과 | 열 `[24,17]px`, gap `5px`, 행 `16px`, root `100×40px`, item `24×16px` 기대가 서로 일관된다. [계획](./c11-4-image-auto-max-mixed-track.md#L17) |
| 4 | automatic minimum 적용 조건 | 통과 | `Minimum` contribution source의 관련 flags와 replaced image natural size가 기대값에 고정돼 있다. overflow applicability는 Chrome 관측으로만 기록하고 Spinon overflow 동작으로 확대하지 않는다. [계획](./c11-4-image-auto-max-mixed-track.md#L21) |
| 5 | §11.4/§11.5 stage 의미 | 통과 | `Initialize`의 JSON `null`은 non-finite projection으로만 해석하며 내부 `+∞`를 증명한다고 하지 않는다. `IntrinsicItemBatch` 이후 유한 growth limit을 별도로 기대한다. [runtime README](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/README.md#L28) |
| 6 | Raw track slot mapping | 통과 | CSS track과 sentinel/gutter를 포함한 `2N+1` 배열을 열·행 모두 검사한다. [Android trace matcher](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L3388) |
| 7 | Stage 배열 값과 slot flags | 통과 | 두 axis의 6 stages, `baseSize/growthLimit`, `gutter`, `collapsed`, `infinitelyGrowable`, internal index를 검사한다. Finite 값 tolerance는 `0.0001px`다. [Android](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L3424) · [iOS](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L3707) |
| 8 | Contribution 종류·순서·source | 통과 | 두 축의 `Minimum`, `MinContent`, `MaxContent`, span, flexible-cross flag, 값 및 `minimumSource`를 검사한다. [Android](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L3442) · [iOS](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L3726) |
| 9 | Root style 경계 | 통과 | root 치수, margin/padding/border, gap, alignment와 정확한 track list를 검사한다. [검증 코드](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L264) |
| 10 | Item style·placement 경계 | 통과 | width/height, min/max, aspect ratio, box spacing, position과 자동 placement 조건을 제한한다. [검증 코드](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L337) |
| 11 | HTML topology와 root allowlist | 통과 | 허용 root ID, HTML namespace/tag, 직접 자식 image 하나, leaf/source 조건을 거부 경계로 확인한다. [adapter](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L53) |
| 12 | Resource decode·metadata 실패 | 통과 | request, decode, snapshot 생성 실패를 구체적인 runtime error로 반환하고 decoded dimension이 고정 입력과 다르면 거부한다. [adapter](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L133) |
| 13 | Snapshot generation/revision identity | 통과 | request와 resource snapshot이 같은 `HostDocumentSnapshot`에서 만들어지고, 새 intrinsic revision을 넘겨 layout input으로 변환한다. [adapter](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image.rs#L133) |
| 14 | Feature 및 product artifact 경계 | 통과, 범위 제한 | fixture 경계는 opt-in feature이며 default/product와 fixture-test FFI archive를 대조했다. 이 근거는 최종 APK/IPA 전체 fixture exclusion 증거가 아니다. [feature boundary](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/feature-boundary/README.md) |
| 15 | Android trace JSON raw type | 통과 | trace matcher가 JSON `Number`, `Boolean`, `String`을 구분하고 finite 수만 허용한다. 문자열 숫자, 불리언 수치, NaN/Infinity는 해당 helper에서 거부된다. [helpers](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L3498) |
| 16 | iOS trace JSON raw type | 통과 | `CFNumberGetTypeID`와 `CFBooleanGetTypeID`를 분리하고 finite 검사와 exact string/boolean 검사를 한다. [trace matcher](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L3646) |
| 17 | Android intrinsic-input JSON raw type | 통과 | revision, grid/node/sourceOrder는 integer matcher, natural width/height는 finite-number matcher, source kind는 string matcher를 사용한다. [intrinsic matcher](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java#L2676) |
| 18 | iOS intrinsic-input JSON raw type | 통과 | 같은 필드에 CFNumber 기반 integer/finite 검사와 exact string 검사 적용을 확인했다. `aspectRatio`는 고정 contract가 아니라 이 검사 대상이 아니다. [intrinsic matcher](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L1986) · [호출부](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift#L2115) |
| 19 | Final-source build·simulator evidence | 통과, paint 제외 | Android `BUILD SUCCESSFUL`, iOS `BUILD SUCCEEDED`; final05에서 두 플랫폼 모두 `status=0`, root `100×40`, item `24×16`, trace와 draw/present가 통과했다. screenshots와 app/archive hashes는 manifest와 일치한다. [run README](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/README.md#L5) · [manifest](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/runs/c114-image-auto-max-20261026T020007Z-cac4e339-final05/source-hashes.json) |
| 20 | Malformed JSON mutation test 및 PR image link | 부분 확인 | native matcher에 문자열 수·불리언 대체·NaN/Infinity를 주입하는 negative test/runtime 실행은 찾지 못했다. PR head `9b3101dca186ee55016b2023c9610186b4df9ecb`에서 Android `1080×2400`, iOS `1206×2622` final05 이미지가 모두 `complete=true`로 로드됐고, Agent Browser 세션은 종료되어 `active=false`, `pid=null`, 일치 프로세스 0개다. JSON fail-closed negative test 부재만 잔여 검증 제한이다. [link-check JSON](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/pr-image-link-check-2026-10-26.json#L20) |

## Runtime 한계와 병합 범위

Android log는 `runtime_image_count=0`, `image_draw_count=0`; iOS도 같은 값이다. 최종 화면에서 보이는 빨간 사각형은 fixture의 image background color이며 PNG raster paint/readback이나 pixel equality를 증명하지 않는다. Android 결과는 software-rendered API 37 ARM64 AVD, iOS 결과는 Simulator다. WPT, 실기기, Android hardware GPU, iOS compositor 이후 pixels 및 overflow pixels는 확인하지 않았다.

이 제한 fixture는 전체 C11.4 지원 완료가 아니다. 공식 상태 대장 `72/213`을 그대로 둔다.
