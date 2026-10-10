# C06.6a viewport 단위 계획 실패 경로 검토

계획 문서 [C06.6a 네이티브 viewport 길이 단위](../../../plan/c06-6-viewport-units.md)를 구현 전 별도로 검토했다. 각 항목은 서로 다른 경계·실패 입력·동시성 위험을 공격한다. 조치 사항은 계획에 반영했다. 이 자료는 구현 검토를 대신하지 않는다.

| # | 공격 관점 | 발견 가능 결함 | 계획의 방어·판정 |
| --- | --- | --- | --- |
| 1 | `vw` 기본 기준 | 기본 `vw`를 dynamic viewport로 오해 | default `v*`는 large viewport라는 CSS Values 4 기준을 fixture와 표에 고정한다. |
| 2 | small/large/dynamic 구별 | 브라우저 주소창이 없는 네이티브 앱에서 존재하지 않는 toolbar 상태를 추정 | 현 native surface에서 세 크기가 같다는 명시적 호스트 계약을 둔다. |
| 3 | `vi` 축 매핑 | inline 축을 항상 width로 고정 | 현재 성공 경로를 horizontal-tb로 제한하고 vertical writing mode를 거부한다. |
| 4 | `vb` 축 매핑 | block 축을 항상 height로 가정한 채 vertical mode를 허용 | 단위 표에 현재 writing-mode 경계를 명시하고 vertical mode negative case를 둔다. |
| 5 | `vmin` / `vmax` | portrait만 검사해 min/max 축 혼동 미발견 | portrait와 landscape, 양 축을 독립 probe로 고정한다. |
| 6 | DPR 변환 | CSS px 길이에 DPR을 곱해 layout이 두 배가 됨 | DPR 1/2에서 CSS values와 frame이 동일한 negative control을 둔다. |
| 7 | drawable 대 CSS surface | GPU pixel 크기를 CSS viewport로 사용 | native surface content bounds CSS px를 단일 source로 지정한다. |
| 8 | resize 동일값 재전달 | 값이 안 바뀌어도 revision이 계속 증가 | same-input no-op와 revision 유지 조건을 둔다. |
| 9 | resize 실제 변경 | dimension은 변경됐지만 cache/cascade가 이전 결과를 재사용 | width/height 변화 시 environment revision과 cascade/layout 새 계산을 검사한다. |
| 10 | 빠른 연속 resize | 오래 걸린 이전 viewport 계산이 최신 frame을 덮음 | immutable snapshot과 latest revision stale-discard를 검증한다. |
| 11 | DPR-only update | DPR 변경이 CSS viewport unit을 바꿈 | DPR-only 변경은 raster input만 바꾸고 CSS unit 결과는 유지하는 비교를 둔다. |
| 12 | invalid dimension | NaN, infinity, 음수, zero가 Taffy에 전달 | 기존 `CssViewport` validation/error policy를 보존하고 negative cases로 검사한다. |
| 13 | overflow | 유한 입력 두 개를 곱한 device pixel 크기만 overflow | 현재 width×DPR validation을 유지하고 invalid environment rejection을 시험한다. |
| 14 | math 함수 혼합 | `calc(10vw + 2px)`가 숫자/길이 단위 혼합에서 잘못 계산 | C06.5 허용 function만 fixture에 넣고 computed value와 frame 모두 비교한다. |
| 15 | CSS variable | `var()` 대체 과정에서 viewport 단위 또는 fallback 소실 | custom property, fallback, inherited value를 별도 fixture로 고정한다. |
| 16 | unsupported property | 구현 안 된 property에 viewport 값이 전달돼 성공을 가장 | 현재 지원 property allowlist만 성공으로 인정하고 외부 property는 명시 거부한다. |
| 17 | container-unit fallback | Stylo가 `cqw`를 작은 viewport에 fallback해 성공처럼 계산 | 모든 container unit을 C21 전 명시적으로 거부하고 성공 경로에서 제외한다. |
| 18 | root/detached/hidden node | fragment root나 detached node에서 viewport 기준이 달라짐 | connected layout tree만 수치 성공으로 비교하고 hidden/detached 결과는 기존 경계를 따른다. |
| 19 | cascade/cache key | stylesheet 변경·환경 변경·문서 변경이 겹쳐 잘못된 결과 재사용 | 기존 revision tuple과 viewport snapshot을 유지하고 concurrent stale rejection을 검증한다. |
| 20 | 기준 모델 과장 | headless fixture로 브라우저 chrome 수축·키보드 동작까지 검증했다고 주장 | headless Chromium의 단일 viewport 동등값만 비교하며 toolbar/keyboard 특수 동작은 명시 제외한다. |

## 검토 후 계획 상태

- 서로 다른 실패 경로가 20개 확인됐다. 검토 내용을 계획에 반영했다.
- 구현 전 선행 산출물은 Chromium fixture/reference, Stylo typed value 관찰, container-unit fail-closed 경계다.
- 이 계획 검토는 구현 완료 근거나 구현 후 적대 검토 횟수에 포함하지 않는다.
- 내부 숫자 계약과 Spinon crate 버전은 `0.1.0`이다. `0042`는 문서 순번이며 제품·계약 버전이 아니다.
