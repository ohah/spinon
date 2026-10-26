# C19.1 incremental color reuse 구현 실패 관점 20개

검토 기준은 source patch SHA-256 `3bef4bc6237df85228456722183e23b69c373e85bf310266feda93fcd876e1ce`, 계획 blob SHA `97d31e561670a3a3de128bdf84c9a5c0c86ee071`이다. 계획 검토와 분리해 실제 코드·회귀 동작을 확인했다.

| # | 구현 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | incremental dispatch가 C19 profile에 다른 속성 목록을 연결 | full/incremental 양쪽이 `c19_computed_color_properties()`를 공유한다. |
| 2 | property list가 color carrier와 무관한 layout/font Longhand를 포함 | `background-color`와 `color` 두 항목만 만든다. |
| 3 | 두 Longhand 문자열과 `LonghandId`가 서로 다른 속성을 가리킴 | `BackgroundColor`/`Color` pairing이 typed style output과 대응한다. |
| 4 | C19 `properties` map에 이전 layout/font key가 남음 | C19 full·incremental fixture에서 map 길이 2와 두 key를 검사한다. |
| 5 | 이전 paint profile이 새 전용 목록을 받아 동작이 달라짐 | `RuntimeBlockPaintV1` 경로와 함수는 변경하지 않았다. |
| 6 | C19.2 box opacity profile을 C19.1 incremental 지원 때문에 잘못 라우팅 | `C19BoxPaintOpacityV1`의 기존 분기와 계산은 바뀌지 않았다. |
| 7 | color-only 출력 변경이 Stylo author declaration preflight 범위를 축소 | C19 author allowlist는 변경하지 않았고 기존 Chromium fixture의 display/size 선언도 통과한다. |
| 8 | dirty inline color 변경이 `computed_color`를 갱신하지 않고 old carrier 재사용 | descendant inherited color가 바뀌고 full cascade와 동일함을 확인한다. |
| 9 | `currentColor` background가 새 inherited `color`에 연결되지 않음 | target의 background carrier도 바뀌며 computed color carrier와 같다. |
| 10 | Display-P3 source가 sRGB로 미리 변환된 값만 남기거나 source tuple이 손상 | full/incremental `ComputedElementStyle` 전체 동등성으로 source tuple을 비교한다. |
| 11 | alpha, missing-component bit 또는 gamut-mapped output이 reuse 중 변형 | `ComputedCssColor` 전체 `PartialEq` 결과를 동일 문서의 full cascade와 비교한다. |
| 12 | parent context 없이 dirty child만 계산해 inheritance가 stale | root context style를 재계산하고 output은 이전 값 그대로 재사용한다. |
| 13 | context root 재계산을 통계상 재사용에서 제외 | fixture에서 recomputed/context/reused node 수를 각각 검사한다. |
| 14 | dirty subtree 밖의 sibling이 재계산되거나 stale한데 통계가 놓침 | sibling output은 이전 snapshot과 정확히 같고 full cascade와도 맞는다. |
| 15 | reused/recomputed element의 source order가 달라짐 | ordered `elements` 전체를 full snapshot과 비교한다. |
| 16 | reuse 결과에 diagnostics가 빠지거나 중복 | diagnostics 전체를 full cascade와 비교한다. |
| 17 | C19.1 computed carrier가 legacy paint input까지 채워 paint처럼 노출 | 각 element의 `background_color`와 `background_paint`가 모두 `None`인지 검사한다. |
| 18 | author stylesheet가 포함된 호출이 보통 reuse처럼 받아들여짐 | non-empty author stylesheet 입력에서 `None` fallback을 확인한다. |
| 19 | profile/viewport 변경으로 다른 계산 결과를 재사용 | profile mismatch 및 viewport mismatch에서 `None`을 확인한다. |
| 20 | 먼 style revision 또는 새 문서 revision 전 결과를 수용 | style revision 2단계 건너뜀과 동일 문서 revision 재호출이 각각 `None`을 반환한다. |

**판정:** 고정된 C19.1 carrier 범위는 통과했다. 이 검토는 C19.2 paint, GPU, C19 parent 완료 또는 공개 CSS/DOM 동작을 판정하지 않는다.
