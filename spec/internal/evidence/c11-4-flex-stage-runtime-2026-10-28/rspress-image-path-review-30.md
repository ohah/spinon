# Rspress screenshot 경로 수정 검토 30개

검토 대상은 Android/iOS screenshot 두 Markdown image path와 Rspress 2.0.22 선택 빌드 결과다. 서로 다른 asset 해석·경로·증거 경계를 확인한다.

| # | 실패 가설 | 판정과 근거 |
| ---: | --- | --- |
| 1 | `runs/...`가 module 경로로 해석되어 빌드가 실패한다. | **수정 후 PASS** — 링크에 `./`를 붙였고 Rspress 2.0.22 build가 두 이미지 모두 번들링했다. |
| 2 | Android 경로가 이전 superseded run을 가리킬 수 있다. | **PASS** — source URL은 matcher-order 수정 뒤 최종 AVD run `225654572Z-2745705b`를 가리킨다. |
| 3 | iOS 경로가 이전 run을 가리킬 수 있다. | **PASS** — source URL은 final iOS run `224932984Z-8e6b2435`를 가리킨다. |
| 4 | screenshot 파일 이름의 대소문자가 잘못될 수 있다. | **PASS** — 두 URL의 case-sensitive 경로가 tracked PNG 이름과 일치한다. |
| 5 | relative path가 README 상위 route 또는 repository root 기준일 수 있다. | **PASS** — `./runs`는 두 README alias의 evidence 디렉터리에서 각각 해석됐다. |
| 6 | `spec/internal` alias만 성공하고 public Tailnet alias는 실패할 수 있다. | **PASS** — `internal/.../README.html`과 `spec/internal/.../README.html` 두 route에서 Android/iOS asset source를 확인했다. |
| 7 | 첫 alias 빌드만 성공하고 두 번째 alias가 stale README를 사용할 수 있다. | **PASS** — selected source에 두 README 경로를 최신 Markdown으로 채우고 두 페이지의 asset map을 검사했다. |
| 8 | Rspress가 이미지 파일을 생성하지 않고 URL만 출력할 수 있다. | **PASS** — 생성 페이지의 fingerprinted `/spinon/static/image/*.png` 두 파일이 실제 build output에 있다. |
| 9 | fingerprint 경로와 source image 이름이 달라 잘못된 파일을 링크할 수 있다. | **PASS** — 생성된 Android·iOS asset 각각을 원본 fixture image와 매핑했다. |
| 10 | asset loader가 PNG를 변환하거나 손상시킬 수 있다. | **PASS** — 빌드 asset SHA-256이 원본 PNG SHA-256과 일치한다. |
| 11 | route 설정이 `/spinon/` base를 누락할 수 있다. | **PASS** — HTML은 `/spinon/static/image/...` 경로를 생성했고 해당 파일은 site root에서 찾았다. |
| 12 | Android 이미지 alt text가 없어 문맥을 잃을 수 있다. | **PASS** — AVD·scene을 식별하는 기존 한글 alt text를 유지했다. |
| 13 | iOS 이미지 alt text가 없어 플랫폼을 구분하지 못할 수 있다. | **PASS** — iPhone 17 Pro / iOS 26.2 Simulator와 scene을 식별하는 alt text를 유지했다. |
| 14 | 수정을 위해 local absolute path가 공개 문서에 들어갈 수 있다. | **PASS** — 두 source URL은 repository 내부 상대 경로만 사용한다. |
| 15 | 경로에 `..` traversal이 들어가 다른 evidence를 끌어올 수 있다. | **PASS** — 두 경로는 README 바로 아래 `runs/`로만 이동한다. |
| 16 | screenshot 파일이 빠져도 markdown만 바뀌어 검증을 통과할 수 있다. | **PASS** — tracked run directory의 두 PNG 존재와 source SHA를 확인했다. |
| 17 | Android image 링크가 iOS screenshot을 가리키거나 반대가 될 수 있다. | **PASS** — platform alt·filename·run directory·manifest를 일치시켰다. |
| 18 | image Markdown 대신 일반 hyperlink가 되어 본문에 표시되지 않을 수 있다. | **PASS** — `![alt](./runs/...)` image 구문을 유지했다. |
| 19 | 이미지 한 장만 새 경로를 쓰고 다른 한 장은 이전 경로를 남길 수 있다. | **PASS** — 두 `![...]` URL 모두 `./runs/` prefix를 갖는다. |
| 20 | Rspress version이 project pin과 달라 local 결과가 재현되지 않을 수 있다. | **PASS** — `packages/docs`의 pinned Rspress 2.0.22 CLI로 빌드했다. |
| 21 | `spec` 경로 copy와 `internal` 경로 copy가 서로 다른 screenshot을 포함할 수 있다. | **PASS** — selected root inventory는 HEAD와 hash를 비교하고 두 alias output 모두 동일 asset bytes를 생성했다. |
| 22 | image fix가 runtime source나 화면 자체를 바꿀 수 있다. | **PASS** — 변경은 README의 두 Markdown URL뿐이며 PNG bytes는 수정하지 않았다. |
| 23 | screenshot이 앱의 배치와 다른 장면을 설명할 수 있다. | **PASS** — 두 파일은 이미 simulator manifest의 weighted-columns 관측과 hash로 연결돼 있다. |
| 24 | 렌더링된 asset을 pixel equality 또는 GPU readback 증명으로 과장할 수 있다. | **PASS** — 검증 문서는 asset resolution과 바이트 동일성만 주장한다. |
| 25 | 현재 PR이 전체 Rspress 문서 링크를 검증했다고 오해할 수 있다. | **PASS** — 범위를 두 screenshot asset으로 제한하고 전체 문서 link validation은 주장하지 않는다. |
| 26 | CI checks가 없는데 빌드 검증을 CI 통과로 표시할 수 있다. | **PASS** — Rspress는 로컬 pinned CLI 결과로만 기록하고 CI와 구별한다. |
| 27 | 이미지 import fix가 Rspress publicDir 정책에 의존해 다른 route를 오염시킬 수 있다. | **PASS** — pinned config에서 상대 import로 static asset을 생성했고 publicDir 설정은 바꾸지 않았다. |
| 28 | screenshot을 URL encode하지 않아 dash·숫자가 포함된 run 경로를 못 찾을 수 있다. | **PASS** — 지정 run directory의 source file과 빌드 결과 hash를 직접 비교했다. |
| 29 | 수정된 README에 검증 자료로 가는 link가 없어 상태를 추적하기 어려울 수 있다. | **PASS** — README에 image resolution summary와 JSON report로 가는 상대 링크를 추가했다. |
| 30 | 이 변경을 전체 C11.4 완료나 공식 집계 증가로 오해할 수 있다. | **PASS** — 이 문서 수정은 presentation asset 경로만 다루며 C11.4와 `72/213` 상태를 바꾸지 않는다. |

## 범위

이 표는 screenshot asset route를 검토한다. Tailnet server 게시와 그 외 documentation route는 별도 확인 대상이다.
