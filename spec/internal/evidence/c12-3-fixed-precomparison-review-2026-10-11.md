# C12.3 Chrome 사전 비교 실패 관점 검토

이 검토는 fixture, Chromium capture, reference JSON과 테스트를 실제로 대조했다. 고정 좌표를 정답으로 다시 기록한 수준인지, 환경·owner·환경 변경을 잘못 섞었는지를 중점 확인했다.

| # | 공격 관점 | 확인 및 반영 |
| --- | --- | --- |
| 1 | 이름만 Chrome 154이고 다른 patch build를 oracle로 쓰지 않는가? | 실행 파일 version, DevTools product/revision, 실행 파일 SHA-256을 모두 pin한다. 불일치하면 캡처를 중단한다. |
| 2 | host 언어·시간대·media 조건이 좌표에 섞이지 않는가? | `en-US`, UTC, color/forced-colors/pointer/hover를 CDP로 고정하고 navigator/media 결과를 캡처마다 검사한다. |
| 3 | 요청한 viewport와 percentage 기준 client area가 scrollbar 때문에 다르지 않은가? | HTML은 root/body overflow를 숨긴다. `innerWidth/Height`와 document client width/height가 요청 viewport와 같은지 검증한다. scroll 동작은 범위 밖이다. |
| 4 | viewport보다 큰 root를 containing block으로 오인하지 않는가? | static root의 min-height를 viewport보다 크게 두고 fixed owner/frame을 root frame과 분리해 캡처한다. |
| 5 | 작은 단일 viewport만으로 비율 계산을 우연히 맞추지 않는가? | 동일 inventory를 `360×800`과 `390×844`에서 관찰한다. |
| 6 | DPR 1·2만으로 분수·고밀도 차이를 놓치지 않는가? | 두 viewport 각각 DPR 1·2·2.625·3을 캡처하며 각 node의 computed style과 frame을 DPR별 deep-equal 비교한다. |
| 7 | resize 뒤 이전 frame을 새 환경 결과로 오인하지 않는가? | 360→390→동일 390 재적용→360 왕복 기록을 분리하고 no-op 및 왕복을 node별 비교한다. |
| 8 | DOM source parent와 containing-block owner를 한 값으로 합치지 않는가? | 각 node의 parent ID와 owner를 별도 저장·검증한다. |
| 9 | fixed가 일반 static·relative·absolute ancestor에 잘못 붙지 않는가? | 세 조상 아래 fixed box를 각각 둔다. Chrome owner는 viewport이며 frame도 따로 보존한다. |
| 10 | owner 문자열만 지정하고 실제 좌표는 viewport frame일 수 있지 않은가? | fixed/absolute box마다 owner 원점, computed used inset·margin 및 coordinate error를 기록한다. 최대 오차는 `0.005 CSS px`다. |
| 11 | percentage basis의 세로·가로 축 또는 shorthand 순서를 뒤바꾸지 않는가? | 네 면 percentage shorthand를 두 viewport에서 캡처해 각 axis frame을 확인한다. |
| 12 | `calc()` 분수 결과를 정수 반올림으로 숨기지 않는가? | 390×844에서 `x=22.5`, `y=21.875`를 원시 frame에 고정한다. |
| 13 | 0·음수 inset과 nonzero margin이 빠져 edge 계산이 검증되지 않는가? | zero inset, negative inset, margin이 있는 fixed node를 각각 기록한다. |
| 14 | auto stretch와 box-model 크기를 content size로 오독하지 않는가? | border-box/padding/border stretch, min/max, aspect ratio를 outer `getBoundingClientRect()`로 고정한다. border paint는 제외한다. |
| 15 | fixed/absolute descendant가 owner를 같은 방식으로 상속한다고 가정하지 않는가? | fixed parent의 absolute child는 그 parent owner, fixed descendant는 viewport owner로 분리한다. |
| 16 | hidden subtree를 viewport synthetic child로 승격하는 경우를 놓치지 않는가? | `display:none` parent와 fixed descendant의 box 없음·owner none을 모든 observation에서 확인한다. |
| 17 | CSS fixed-CB 효과를 stylesheet·inline·개별 transform에서 누락하지 않는가? | 둘 다 사용해 transform·단독 preserve-3d·rotate/scale/translate·filter·contain·content-visibility·will-change를 캡처하고 opacity를 음성 대조군으로 둔다. |
| 18 | inventory/reference를 재캡처해 oracle을 조용히 바꾸지 않는가? | 기본 캡처는 기존 파일이 있으면 중단한다. 교체에는 `--replace-reference`가 필요하고 fixture/tool/helper 해시를 기록한다. |
| 19 | 대응 WPT 경로를 실행 결과로 과장하지 않는가? | WPT 상태는 `not-run`이며 test와 evidence가 그 상태를 확인한다. |
| 20 | Chrome frame을 Spinon·모바일·GPU 구현 완료로 오인하지 않는가? | Android/iOS runtime, revision 게시 순서, failure atomicity, paint/scroll은 미측정으로 남기고 상태표도 미완료다. |

검토 중 발견해 반영한 경계 수정은 scrollbar 방지와 client-size assertion, 단독 `preserve-3d`, 개별 transform/`will-change:contain`, owner-origin 좌표 probe다. 이 reference는 구현 전 oracle만 확정한다.
