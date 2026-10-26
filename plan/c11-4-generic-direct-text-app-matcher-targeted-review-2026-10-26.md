# C11.4 일반 direct text 앱 matcher 제한 검토

## 검토 범위

이 recheck는 `generic-root-normal`의 Android/iOS C0410 intrinsic/trace matcher와 계획된 행 축 contribution contract만 정적으로 다시 읽었다. 요청에 따라 build, test, formatter, simulator, Agent Browser는 실행하지 않았다. 이 문서는 런타임 pass나 앱 artifact 증거가 아니다.

## 확인한 수정

- Android는 `JSONObject.opt`의 원본 값을 검사한다. 숫자는 실제 `Number`여야 하고 finite여야 하며, integer는 정수성·범위·왕복 정밀도를 검사한다. 문자열과 Boolean도 각각 실제 `String`/`Boolean`만 받는다.
- iOS는 숫자 `NSNumber`에서 `CFBoolean` type ID를 거부한다. 숫자는 finite인지, 정수는 `Int(exactly:)`로 정확히 표현되는지 확인한다. 문자열은 String, Boolean은 CFBoolean 타입으로만 받는다.
- 두 플랫폼 matcher는 layout source/style/environment/intrinsic-size revision의 JSON 타입, 두 direct text input과 source order, item별 text intrinsic 측정값을 확인한다. Track의 gutter/collapsed/infinitelyGrowable 및 contribution의 crossesFlexibleTrack은 JSON Boolean으로만 받는다.
- Columns matcher는 node `2`의 `MinContent=20px`, node `4`의 `MaxContent=40px`를 대조한다. Rows matcher는 node `2`와 `4` 각각의 `Minimum=20px`, `MinContent=20px`, `MaxContent=10px`를 대조한다. 모든 tuple은 span 1이고 flexible track을 지나지 않아야 한다. 동일 tuple의 sizing query 반복은 허용하고, 예상 밖 tuple은 거부한다.
- 행 축 값은 [사전 비교 기록](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md#row-axis-trace-expectations)과 [입력 계획](./c11-4-generic-direct-text-intrinsic-input.md)에 Ahem/CSS 계산 및 runtime trace acceptance로 추가했다. Chrome DOM observation이 contribution kind를 측정했다는 의미는 아니다.

정적 확인에서 요청한 두 matcher 약점의 코드 경계는 닫혔다. 잘못된 JSON scalar의 실제 거부 동작, contribution tuple이 runtime trace에서 예상대로 나오는지, 두 플랫폼의 source compile은 이 recheck로 입증하지 않았다.

## 변경 파일과 SHA-256

| 파일 | SHA-256 |
| --- | --- |
| [`C0410RuntimeGpuDemo.java`](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java) | `7b3c09211d1bea04a9ba48cc0959a68563609c035d13e1832770a58ed27c717e` |
| [`C0410RuntimeGpuDemo.swift`](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift) | `413ba10c8dbe4b7aec29ae5330900d7f51d50da8190ae5066f3a9a867b2c71d2` |
| [`generic direct text plan`](./c11-4-generic-direct-text-intrinsic-input.md) | `75f7f90f01c775f79b16955cb6729ae11870615e2885571078fabcaa77fa18e8` |
| [사전 비교 README](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md) | `f01105b9621a9d0eb67354c7b70994d03c171c94c1728dab5403ef7c46edb3d7` |

`spec/STATUS.md`는 수정하지 않았다. C11.4 공식 완료와 `72/213` 집계는 그대로다.

## 통합 후 필요한 검증

- 이미지 lane 병합 뒤 이 text worktree를 rebase하고, 두 C0410 파일의 겹치는 변경 및 `CoreFoundation` import를 하나의 소스로 통합한다.
- 합쳐진 소스에서 Android/iOS build와 AVD/Simulator 실행을 수행해 이 matcher가 실제 trace를 통과하는지 확인한다.
- 잘못된 integer/number/string/Boolean JSON mutation이 각 플랫폼에서 거부되는 대조도 별도 실행 evidence로 남긴다.
- 이 문서의 source hashes는 현재 text worktree candidate만 가리킨다. rebase나 통합 수정 이후 해시를 갱신해야 한다.
