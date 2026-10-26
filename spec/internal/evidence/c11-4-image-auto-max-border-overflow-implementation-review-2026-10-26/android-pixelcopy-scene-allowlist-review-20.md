# C11.4 Android PixelCopy scene allowlist review

## 검토 대상

- 검토 시점 기준 HEAD: `eb7d2d416b899ca6c98ee1bbafec8dce953f5fdf`
- 검토 시점 tracked worktree diff SHA-256 (commit/rebase 전): `24c823047c4a03c1e30fb2335eaa0d242ad5765c1475f8e67743bbbee3f5fc16`
- Android adapter source blob: `80f46d06c16d340dbd8e15d722059f09c52ccc9a`
- 검증 APK SHA-256: `f2ceeaa7cfc56bb732b4eca6d6cc6aa9dd4f8d73a2ee7fcb86c915d0ec5c3b8a`
- 통과 실행: `c114-android-avd-20261026T063238848Z-970016a2`

## 실패 관점별 확인

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | JS capture manifest에 control scene ID가 등록되지 않음 | `image-auto-max-mixed-tracks`가 등록되어 있고 Android control scene이 통과했다. |
| 2 | JS capture manifest에 border scene ID가 등록되지 않음 | `image-auto-max-mixed-tracks-border-overflow`가 등록되어 있고 수정 후 통과했다. |
| 3 | Android pixel scene allowlist가 신규 scene을 거부함 | 두 ID만 명시적으로 추가했다. 두 장면 모두 `scene-index-mismatch` 없이 진행했다. |
| 4 | 잘못된 scene이 넓은 prefix 규칙으로 허용됨 | 새 조건은 정확한 두 문자열만 비교하며 prefix 허용을 추가하지 않았다. |
| 5 | pixel capture run ID가 grid 실행과 분리됨 | 기존 `c114GridTrackSizingRunId`와 pixel run ID의 동등성 검사를 그대로 유지했다. |
| 6 | control scene과 border scene의 payload가 뒤바뀜 | control은 geometry control만 사용하고, border scene의 4개 sample은 해당 scene의 runtime log에서 일치했다. |
| 7 | outside-root 색이 Chrome body 색으로 잘못 회귀함 | Android outside-root 표본은 `[0,0,0,255]`이며 PixelCopy에서 정확히 일치했다. |
| 8 | item 표본이 잘못된 노드를 가리킴 | item marker는 `c114-image-auto-max-border-overflow-item`을 가리키며 `[229,72,77,255]`로 일치했다. |
| 9 | 좌우 border 표본 좌표가 root 내부를 벗어남 | 두 표본은 root의 좌우 가장자리 CSS 좌표에서 `[34,197,94,255]`로 일치했다. |
| 10 | root exterior와 border 샘플의 표면 좌표 변환 오류 | 실행 로그의 PixelCopy 좌표와 각 기대 CSS 좌표가 동일한 색 표본을 반환했다. |
| 11 | layout geometry가 pixel gate 통과를 대신함 | runner는 geometry, trace, draw, PixelCopy gate를 각각 확인하고 전체 실행에서 모두 통과했다. |
| 12 | grid track 결과가 이미지의 intrinsic sizing을 우회함 | border scene에서 columns `[24,17]`, row `[16]`, item `(1,1,24,16)`을 확인했다. |
| 13 | border overflow가 root frame으로 잘려 사라짐 | root `12×40`보다 넓은 item `24×16`이 캡처 이미지에 border 밖으로 표시된다. |
| 14 | scene 재실행 때 이전 Activity 로그를 재사용함 | 각 launch는 별도 run ID를 사용하며, 통과 summary와 pixel samples가 해당 실행 ID에 연결됐다. |
| 15 | PixelCopy 실패 뒤 성공으로 기록됨 | 수정 전 실패 실행은 별도 폴더에 남아 있고 성공 결과는 새 실행 폴더에 기록됐다. |
| 16 | PixelCopy 허용 목록 수정이 제품 renderer를 바꿈 | 수정은 Android fixture adapter의 scene admission 조건에만 적용했고 renderer clear 경로는 바꾸지 않았다. |
| 17 | 설치된 APK가 검증된 APK와 다름 | `installed-apk-verification.json`에서 로컬·설치 APK hash가 모두 `f2ceeaa7...c3b8a`로 일치했다. |
| 18 | AVD 밖의 물리 Android 기기를 검증 대상으로 착각함 | 실행 metadata는 `target=android-avd`, `serial=emulator-5556`, API 37, ARM64를 기록한다. |
| 19 | simulator screenshot이 다른 scene 또는 세션에서 나옴 | PNG는 통과 실행 폴더와 scene ID에 저장됐고 SHA-256 `02203d1e...a3f5`로 고정했다. |
| 20 | AVD 결과를 제품 완료 또는 실기기 증거로 확대 해석함 | 이 결과는 API 37 AVD 범위로 한정하며 C11.4 공식 상태와 `72/213` 완료 수는 바꾸지 않았다. |

## 판정

Android PixelCopy scene allowlist의 누락은 수정 후 재현 입력에서 해소됐다. Control과 border-overflow 장면이 모두 통과했고, PixelCopy 표본은 4/4 정확히 일치했다. 실기기 동작과 C11.4 전체 범위 완료는 이 검토에서 주장하지 않는다.

## 독립 peer 검토

별도 read-only 검토는 아래 20개 실패 경계를 확인했고 코드 blocker를 찾지 못했다. 검토 중 발견한 문서 상태 오류는 현재 runtime README와 계획 문서에서 실행 시점에 맞게 정리했다.

| # | 독립 확인 관점 | 결과 |
|---:|---|---|
| 1 | 기준 `origin/main` source SHA가 고정되어 있는가 | `29449e4f794e53255909b58ed8fb2fd4995dd5d2`로 기록했다. |
| 2 | Rust fixture, FFI, Android/iOS adapter, runner scene ID가 같은가 | 동일한 `image-auto-max-mixed-tracks-border-overflow` ID를 확인했다. |
| 3 | root/item ID가 계획과 구현 사이에서 일치하는가 | root와 item ID가 각 경계에서 일치한다. |
| 4 | direct PNG의 크기와 paint claim 경계가 맞는가 | 24×16 PNG이며 texel paint를 주장하지 않는다. |
| 5 | root border-box와 content width 계산이 맞는가 | 12×40 border-box, 1px border, 10px content width를 확인했다. |
| 6 | track·gap·row 기대값이 계획과 맞는가 | columns `[24,17]`, gap `5`, row `[16]`이다. |
| 7 | image가 root의 오른쪽 경계를 넘는가 | image right `x=25`, root right `x=12`로 13px overflow한다. |
| 8 | 정수 sample index와 pixel center 변환이 맞는가 | 네 index에 `+0.5` center 변환을 적용한다. |
| 9 | Chrome body 배경과 앱 surface clear가 구분되는가 | Chrome `[16,24,32,255]`, app `[0,0,0,255]`로 구분한다. |
| 10 | Android allowlist가 scene 및 run ID를 검사하는가 | 정확한 scene ID와 기존 run ID 조건을 유지한다. |
| 11 | control scene의 명시적 allowlist가 과도하게 넓은가 | 두 scene만 열며 일반 prefix 허용은 없다. |
| 12 | payload 파서가 잘못된 sample을 fail-closed하는가 | 형식, 중복, 경계 밖 좌표를 거부하는 기존 검사를 유지한다. |
| 13 | sample validator가 ID/node/index를 검증하는가 | scene의 sample ID, 대상 node, index 조합을 확인한다. |
| 14 | Android color expectation이 실제 marker와 맞는가 | item 빨강, exterior 검정, border 초록이 각각 exact다. |
| 15 | API 37 AVD pixel scale 적용이 맞는가 | scale `2.625`에서 기록 좌표와 표본이 일치한다. |
| 16 | PixelCopy가 해당 surface generation을 읽는가 | overflow scene의 PixelCopy 결과가 4/4 exact다. |
| 17 | 설치 APK와 캡처 source가 연결되는가 | 설치 APK hash가 빌드 APK hash와 일치한다. |
| 18 | AVD geometry/track/trace/draw gate가 모두 통과하는가 | control과 overflow 두 장면의 모든 gate가 통과했다. |
| 19 | iOS 결과가 Android와 독립적으로 보존되는가 | iOS Metal 표본 4/4와 화면 캡처가 별도 evidence에 있다. |
| 20 | AVD 결과가 실기기·전체 완료로 확대 해석되는가 | API 37 AVD로만 기록하고 official `72/213`은 유지한다. |

peer 검토에서 지적한 이전 `Android capture pending` 문구는 이후 실행 evidence에 맞게 고쳤다. 초기 계획·구현 review의 미검증 판정은 각 검토 시점의 상태로 한정했다.
