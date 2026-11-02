# C11.1 fit-content default-route semantic gate 계획 정정 PR diff 검토 20개

이번 실제 PR diff(상위/전용 계획, projection 및 runtime source 경계 근거와 두 검토표)를 다음 20개 관점으로 확인했다.

| # | PR diff 관점 | 결과 |
| --- | --- | --- |
| 1 | 전용 계획이 현재 구현 전 상태를 표시하는지 | 이번 PR은 plan/evidence만 바꾸며 status와 완료 checkbox를 수정하지 않는다. |
| 2 | projection admission을 실제 validator와 맞췄는지 | explicit dimension match와 finite non-negative px/FitContent(px) 조건을 분리해 썼다. |
| 3 | default-route semantic gate를 따로 다뤘는지 | root_html_named_area.rs의 기존 120×70/세 area 경계를 새 source note에 기록했다. |
| 4 | root dimensions를 과거 값으로 고정하는지 | 새 경로는 definite positive px width/height를 요구하고 30×20 scene을 쓴다. |
| 5 | area names를 고정하는지 | "fixed fit"은 예제로만 남고 arbitrary parsed area names를 명시한다. |
| 6 | item count가 여전히 세 개로 고정되는지 | 새 경로는 각 named area 하나씩이며 scene identity와 무관한 count를 사용한다. |
| 7 | item-child 범위가 과도하게 넓어지는지 | visible in-flow empty HTML div direct child와 whitespace-only text 제약을 유지한다. |
| 8 | duplicate/missing area placement가 빠지는지 | area당 한 item, 중복과 unused/missing area 거부를 구현 기준에 적었다. |
| 9 | CSS parser 변경이 끼어드는지 | Stylo parsed typed rectangular area와 placement만 사용하고 parser 변경을 제외한다. |
| 10 | fit-content argument 제한이 모호한지 | px만 허용하고 percentage/calc argument와 음수·비유한 값을 거부한다. |
| 11 | 다른 track mode가 새 named-area path로 들어오는지 | direct percentage, auto, fr, minmax, auto-repeat, named lines, implicit tracks를 제외한다. |
| 12 | existing named-area runtime 지원이 바뀌는지 | 기존 fixed/fraction/percentage 120×70 header/side/content path를 보존한다. |
| 13 | Ahem runtime과 앱 font를 섞는지 | Ahem intrinsic matrix와 default system-font box scenes를 별도로 설명한다. |
| 14 | Chrome oracle screenshot을 Spinon 동작으로 부르는지 | 문서상 Chrome reference only로 유지하고 Spinon evidence를 따로 요구한다. |
| 15 | Android AVD evidence 한계를 누락하는지 | API 37 ARM64와 PixelCopy sample, software renderer 한계를 계획에 남겼다. |
| 16 | iOS Simulator evidence 한계를 누락하는지 | iPhone 17 Pro/iOS 26.2 Metal drawable sample과 compositor 전 시점을 명시한다. |
| 17 | screen overflow 요구가 바뀌는지 | root 경계 안/밖과 fit child 안 표본을 기존 runtime matrix에 유지한다. |
| 18 | official status/counter/API가 변하는지 | spec/STATUS.md, 72/213, API/0059 version을 건드리지 않는다. |
| 19 | 계획 PR에 로컬 product WIP가 섞였는지 | PR diff 대상은 계획과 evidence 문서뿐이며 source/product code는 포함하지 않는다. |
| 20 | 다음 구현 단계가 정정 병합을 기다리는지 | 새 semantic gate 계획 정정 병합 후 한 체크아웃에서 순차 구현하도록 gate를 갱신했다. |

PR diff는 추가 source boundary를 반영하며 구현 완료나 simulator 결과를 주장하지 않는다.
