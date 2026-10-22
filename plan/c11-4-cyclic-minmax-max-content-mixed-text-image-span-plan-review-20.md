# C11.4 cyclic max-content mixed text/image 계획 실패 관점 검토 20개

구현을 시작하지 않은 상태에서 서로 다른 계획·기대값 실패 경로 20개를 검토했다. Chrome 156 관측과 CSS Grid 2025 CRD 독립 산술은 분리했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | max-content column basis를 percentage가 resolve된 최종 track과 혼동하는가 | 110px intrinsic content width와 최종 `[107/3,125/3,86/3]` track을 다른 단계로 기록했다. |
| 2 | intrinsic sizing이 끝나기 전에 cyclic `10%`를 적용하는가 | §7.2.1에 따라 intrinsic 단계에서는 auto처럼 취급하고 결과값에 대해 나중에 resolve한다. |
| 3 | cyclic percentage와 함께 절대 `2px` term까지 없애는가 | 초기 track base와 최종 minimum resolve 양쪽에 `2px`를 보존한다. |
| 4 | 같은 span의 A/B contribution을 DOM 순서대로 누적해 per-track 최대값을 훼손하는가 | A/B를 한 span-2 group으로 처리하고 source order를 바꿔도 track이 같은지 검사한다. |
| 5 | 뒤 span-3 C를 span-2 contribution보다 먼저 처리하는가 | span 길이가 짧은 group부터 처리하도록 명시했다. |
| 6 | image B의 40px minimum을 preferred size만으로 보는가 | CSS `width`와 `min-width`를 둘 다 40px로 고정하고 natural width 24px과 분리했다. |
| 7 | image natural ratio와 natural pixel dimension을 혼동하는가 | 24×16 source와 그로부터 계산한 `80/3px` used block size를 별도로 고정했다. |
| 8 | Columns 이후 image block contribution이 사라지는가 | 같은 revision의 image ratio를 Columns 계산 뒤 Rows에 전달해야 한다. |
| 9 | `80/3px` row contribution을 gutter/base를 빼기 전에 나누는가 | 2px gutter와 두 개의 2px base를 뺀 뒤 남은 증가량을 분배한다. |
| 10 | cyclic row percentage를 border box나 잘못된 축 기준으로 resolve하는가 | intrinsic inner height `80/3px`를 사용해 `14/3px` minimum을 얻는다. |
| 11 | A의 max-content width를 min-content width로 처리하는가 | A의 min/max contribution 20/60px과 max-content 한 줄 60px을 확인한다. |
| 12 | C의 max-content line width를 rendered item frame과 혼동하는가 | 110px contribution과 110px max-content frame을 따로 검증한다. |
| 13 | 다른 Ahem 파일로 line contribution이 바뀌는가 | font weight, size, line-height, load 상태와 SHA-256을 고정했다. |
| 14 | image decode 전에 0 natural size를 유효 입력으로 처리하는가 | decode 완료와 24×16 natural dimensions를 gate로 둔다. |
| 15 | reverse order에서 identity/placement까지 바뀌어 원인을 분리할 수 없는가 | A/B/C ID와 placement는 유지하고 DOM 순서만 바꾼다. |
| 16 | 40px control이 border overflow를 실제로 만들지 않는가 | 이미지 x=16px, width=40px, border box width=44px로 12px 넘도록 고정했다. |
| 17 | border 한 지점 표본을 전체 CSS stroke 적합성으로 확대하는가 | 선택한 지점만 검증하고 전체 border-stroke 판정은 제외한다고 적었다. |
| 18 | simulator 표본 대신 브라우저 screenshot이나 test overlay를 쓰는가 | 실제 앱 surface의 PixelCopy와 compositor 전 Metal readback을 요구한다. |
| 19 | 기본 artifact가 fixture ID/payload를 포함하는가 | 기본 negative control과 opt-in positive control을 모두 확인하되 APK/IPA 전체 제외라고 확대하지 않는다. |
| 20 | 고정 후보를 전체 C11.4 완료나 일반 CSS 지원으로 승격하는가 | C11.4는 unchecked, `77/213` 그대로이며 일반 CSS·WPT·전체 Grid를 제외했다. |

## 결론

장면 allowlist, 독립 산술, browser 관측, overflow 조건과 모바일 근거의 범위를 계획에서 검토할 수 있다. 계획 수준의 차단 결함은 찾지 못했다. 구현 뒤에는 별도의 새 runtime 실패 검토를 작성한다.
