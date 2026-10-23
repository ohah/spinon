# C11.4 auto-repeat + template areas 구현/runtime 실패 관점 검토 20개

검토 기준은 base `3df7fe704fe253e5edcc4946ed215e33dc2e935e`의 구현 diff와 `source-files-manifest.json`에 고정한 파일이다. 계획 검토 표를 재사용하지 않고 cascade, projection, layout, vendored Taffy, app adapter, artifact 및 실측 surface의 별도 실패 경로를 확인했다. 아래 20개는 서로 다른 불변식이며 같은 결론을 세어 늘리지 않았다.

| # | 검토 관점 | 확인한 실패 경로와 판정 |
| ---: | --- | --- |
| 1 | profile admission | `RuntimeBlockFlexGridTrackSizingV1` 밖에서 auto-repeat/areas가 새로 허용되지 않는지 확인했다. cascade와 style-to-layout이 지원 profile 외 입력을 거부하고 일반 CSS admission은 추가되지 않았다. 통과. |
| 2 | repeat 축 개수 | repeat이 없거나 양 축에 동시에 있을 때 area path가 실행되는지 확인했다. cascade와 projection 양쪽에서 두 축 조합은 오류로 끝난다. 통과. |
| 3 | non-repeat 축 dimension | area의 fixed-axis dimension이 explicit track count와 다를 때 Taffy까지 도달하는지 확인했다. projection과 layout validator가 모두 불일치를 거부한다. 통과. |
| 4 | repeat 축 authored dimension | area count를 expanded track 수나 repeat component 수로 덮어쓰는지 확인했다. area의 authored rows/columns와 rectangle을 유지해 Taffy로 전달한다. shorter/larger scene 통과. |
| 5 | coordinate 범위와 정수 overflow | 0/10,001 dimension, `usize`→`u16`, Taffy line bounds의 `+1` overflow 경로를 확인했다. 범위를 먼저 제한하고 conversion은 checked 처리한다. 통과. |
| 6 | rectangle 유효성 | 빈 row/column span, matrix 밖 end, reversed bounds가 resolver로 들어가는지 확인했다. layout validator가 fail closed하며 각 case의 단위 테스트가 있다. 통과. |
| 7 | area 이름과 중복 | 빈/NUL/중복 이름이 alias metadata를 모호하게 하는지 확인했다. `HashSet` 중복 검사와 입력 검사가 있으며 duplicate test가 통과했다. parser가 만든 valid Unicode 이름은 문자열 그대로 보존한다. 통과. |
| 8 | metadata 크기 산술 | area count×record size 및 name bytes 합산이 overflow하거나 1 MiB를 넘을 수 있는지 확인했다. `checked_mul`/`checked_add`와 두 경계의 cap이 있고 초과 test가 통과했다. 통과. |
| 9 | allocation 실패 | style projection vector/name copy, layout duplicate set, Taffy-style slot reserve 실패가 panic/부분 결과로 변환되는지 확인했다. Spinon 소유 reserve는 node/property가 있는 오류로 전파되고 projection failpoint 두 지점이 검증된다. 통과. |
| 10 | Taffy area translation과 clone 경계 | half-open 0-based 경계가 Taffy 1-based line과 다르게 되는지, no-area style이 바뀌는지 확인했다. bounds마다 checked `+1`, row/column count 전달, empty iterator/zero count 기본값을 확인했다. Taffy resolver 내부 `String` clone allocation은 전역 OOM 보장에 포함하지 않는다고 계약에 적었다. 통과. |
| 11 | repeat expansion / resolver 순서 | Spinon이 repeat count를 재계산하거나 area alias를 expansion 전에 숫자로 고정하는지 확인했다. 반복 축 placement는 symbolic name으로 남고 vendored Taffy 0.15.0 `NamedLineResolver`가 expanded grid와 area metadata를 함께 resolve한다. 통과. |
| 12 | authored/generated alias collision | area-generated `hero-start/end`와 authored alias 충돌이 먼저 발견한 이름을 잃는지 확인했다. `columns-line-collision` frame 및 양 플랫폼 runtime scene이 pass했다. 통과. |
| 13 | bare `grid-area` placement | shorthand가 row/column longhand defaulting 뒤 고정 축 또는 repeat 축에서 잘못 숫자화되는지 확인했다. fixed-axis bare area test와 13-scene runtime matrix가 통과했다. |
| 14 | area가 expanded template보다 짧음 | 반복 count 뒤 suffix와 numeric negative tail 위치가 authored area 마지막 경계에 붙는지 확인했다. columns/rows shorter scene에서 전체 child frame과 track이 양 플랫폼 exact다. 통과. |
| 15 | area가 expanded template보다 큼 | 추가 explicit track이 누락되거나 잘못된 `grid-auto-*` size를 쓰는지 확인했다. area-larger control은 고정 20px extra track, 16px root overflow와 child frame을 검사하고 표본이 root 바깥 suffix color를 확인한다. 통과. |
| 16 | auto-fit occupancy/collapse | area 정의만으로 반복 track이 occupied 처리되는지 확인했다. item 없는 area scene은 반복 4개를 모두 0으로 collapse하고 suffix는 fixed neighbor 뒤에 온다. occupied area/numeric control도 함께 통과했다. Chrome 156 gutter 4px 차이는 규범 기대와 분리했다. |
| 17 | no-area / axis regression | 기존 numeric auto-fit과 row axis가 바뀌는지 확인했다. columns/rows numeric controls 및 area/non-area track-sizing regression package 전체가 통과했다. |
| 18 | cascade revision 경계 | template area 변화가 stale layout state와 다른 document revision을 섞는지 확인했다. runtime logs의 document/render-tree revision을 scene 단위로 보존하며 기존 host-document stale-revision regression도 함께 통과했다. |
| 19 | fixture module과 app routing | 새 JSON/module이 기본 FFI path에 들어가거나 한 플랫폼에서 scene allowlist/frame count가 빠지는지 확인했다. fixture-boundary 검사가 새 module gate와 `include_str!` path를 확인하도록 보강했고 6/6 통과; Android/iOS 각 13/13의 box/frame/track/draw assertions도 통과했다. **검토 중 fixture-boundary 회귀 목록이 신규 모듈을 명시하지 않던 점을 발견해 목록과 assertion을 추가했다.** |
| 20 | surface 좌표 및 결과 해석 | sample point가 item overlay/gap/root 바깥을 잘못 짚거나 PixelCopy와 Metal 결과를 동일 compositor 증거로 확대하는지 확인했다. platform logs 각각 66/66 exact, 0 mismatch, 13/13 matrix summary; Android AVD PixelCopy와 iOS pre-compositor drawable 범위만 기록한다. CSS border stroke, `visibility:hidden`, 실기기, hardware GPU 및 compositor-after pixels는 미검증으로 남긴다. |

전체 Rust focused test, FFI fixture test, workspace Clippy, formatter, fixture-boundary test와 양 simulator 최종 run이 pass했다. runtime/source/artifact details는 [실행 근거](./README.md), [교차 플랫폼 비교](./cross-platform-comparison.json), [source hash](./source-files-manifest.json)에 연결했다.
