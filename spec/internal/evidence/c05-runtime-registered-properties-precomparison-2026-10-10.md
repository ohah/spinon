# C05.2 구현 전 Chromium 비교 모델

## 고정 기준

- 기준 브라우저: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 입력: `tests/fixtures/css/c05/runtime-registered-properties.html`
- 인벤토리: `tests/fixtures/css/c05/runtime-registered-properties-inventory.json`
- 실행기: `tools/css-reference/capture-c05-runtime-registered-properties.mjs`
- 기준 산출물: `tests/fixtures/css/references/c05-runtime-registered-properties-v1.json`
- viewport: `301×100` CSS px, scale factor `1`, locale `en-US`, timezone `UTC`, light, coarse pointer, hover 없음

## 사전 판정

| 입력 경로 | 예상 computed value |
| --- | --- |
| `<length>` 등록, 지정 `41px` | `width: 41px` |
| 값 미지정, initial `23px` | `width: 23px` |
| `<length>`에 잘못된 `red` 지정 | 등록된 initial `23px` |
| `inherits:false` 속성을 조상에서 `71px` 지정 | 자식은 initial `23px` |
| `inherits:true` 속성을 조상에서 `19px` 지정 | 자식은 `19px` |
| 사용 위치보다 나중 stylesheet에 등록된 속성 | initial `31px` |
| 유효 등록에 알 수 없는 descriptor 추가 | 등록 유효, initial `13px` |
| inline `!important`가 stylesheet보다 우선 | `width: 47px` |
| 등록 `<color>`의 initial/지정값 | 파란색 `rgb(51, 102, 255)`, override `rgb(255, 102, 0)` |
| 뒤 stylesheet가 같은 이름을 `<number>`로 재등록 | computed custom value `3`; `width`는 auto로 계산되며 고정 scene 폭은 `0px` |

일반 computed value는 문자열 정확 일치로 비교하고, 개별 노드 좌표·크기는 CSS px 기준 각 최대 절대 오차 `0.5`로 비교합니다. 기준 캡처가 위 고정 Chrome 버전·fixture hash·실행기 hash를 기록하지 못하면 구현 비교를 시작하지 않습니다.

## 다중 HostRoot 보조 비교

기준 브라우저와 revision은 위와 동일합니다. `tests/fixtures/css/c05/runtime-registered-properties-multi-root.html`에서 첫 body 직속 root에 연결한 `<style>`은 둘째 body 직속 root의 `#shared-target`을 선택합니다. 고정 Chrome은 두 root, `--cross-root-size: 19px`, `width: 19px`, rectangle `{x:0,y:0,width:19,height:14}`를 관찰했습니다. 실행 명령은 `node tools/css-reference/verify-c05-runtime-registered-properties-multi-root.mjs`입니다. 이 비교는 등록이 fragment root마다 격리되지 않고 문서 연결 cascade로 공유되는지를 판정합니다. Spinon runtime layout은 다중 HostRoot를 아직 `multiple_host_roots`로 거부하므로 이 결과를 다중 root 화면 layout 지원의 근거로 쓰지 않습니다.

## 플랫폼 검증 범위

비교 구현 경로는 HostDocument → Stylo → Taffy → WGPU이며 실제 Android API 37 emulator와 iOS 26.2 Simulator 화면을 확인합니다. 이는 실기기 동작이나 전체 CSS 호환성의 증거가 아닙니다.
