# C11.4 auto-repeat + template areas 구현 PR diff 적대 검토 30개

검토 대상은 PR [#539](https://github.com/ohah/spinon/pull/539)의 최신 diff `3df7fe704fe253e5edcc4946ed215e33dc2e935e..84d3fc254a646fd4e973d1ed6cd907bfb4a0cb0c`이다. 코드·fixture·상태 문서·명세·실행 증거·PR 화면을 독립 실패 경계로 나눠 확인했다. 원시 `.log` 파일의 trailing whitespace/끝 빈 줄은 캡처 내용을 보존한 결과이며, 로그를 제외한 PR diff의 `git diff --check`는 통과했다.

| # | 실패 관점 | 확인 및 결과 |
| ---: | --- | --- |
| 1 | 변경 범위에 무관한 로컬 작업이 PR에 섞임 | PR base-to-head 변경 파일 147개를 확인했다. 기존 미추적 로컬 자료는 commit/PR에 포함되지 않았다. 통과. |
| 2 | 부분 구현을 공식 C11.4 완료로 승격 | `spec/STATUS.md`의 C11.4 checkbox는 미완료로 유지되고 완료 집계도 `77/213`이다. 통과. |
| 3 | 내부 계약 숫자 버전 또는 공개 CSS API를 무단 변경 | 계약 0059의 숫자 버전 `0.1.0`과 공개 API 목록은 유지된다. 통과. |
| 4 | 지원 profile 밖에서 새 Grid 입력을 허용 | cascade와 projection 모두 `RuntimeBlockFlexGridTrackSizingV1` 밖의 repeat+areas 조합을 거부한다. 통과. |
| 5 | 한 Grid에 auto-repeat 축이 둘 이상 전달 | XOR 축 검증, cascade 거부 control, projection 회귀가 양 축 조합을 막는다. 통과. |
| 6 | repeat 없는 C11.3 area projection의 회귀 | style projection은 auto-repeat이 있을 때만 layout-owned area metadata를 전달한다. 기존 no-repeat area fixture/test는 그대로 있고 focused package가 통과했다. 통과. |
| 7 | non-repeat 축 area dimension과 explicit track 수가 다름 | style projection과 layout validator가 고정 축의 mismatch를 각각 거부한다. 통과. |
| 8 | repeat 축 authored area dimension이 확장 전 track 수로 잘못 축소 | source rows/columns 및 rectangle을 그대로 보존하며 area-shorter/area-larger scene이 repeat 확장과 분리된 결과를 확인한다. 통과. |
| 9 | Taffy line bounds 변환에서 `u16` 경계가 overflow | dimensions를 1..=10,000으로 제한하고 0-based half-open 끝점을 checked `+1`로 변환한다. 경계 초과는 결과 게시 전에 거부한다. 통과. |
| 10 | 비어 있거나 역전된 rectangle 또는 template 밖 bounds | start < end와 각 authored dimension 이내 end를 검증하고 단위 negative controls가 있다. 통과. |
| 11 | 빈/NUL/중복 area name이 alias map을 모호하게 만듦 | 세 입력을 거부하고 중복은 layout validation `HashSet`에서도 확인한다. 통과. |
| 12 | metadata 크기 계산의 곱셈/덧셈 overflow 또는 과대 입력 | checked byte arithmetic과 1 MiB 상한을 projection/layout 양쪽에 두고 over-budget test를 실행했다. 통과. |
| 13 | metadata/name allocation 실패가 panic 또는 부분 결과로 노출 | Spinon-owned vector와 String 복사에 `try_reserve`를 사용하고 failpoint 오류를 node/property로 반환한다. 통과. |
| 14 | compact adapter가 한 node에 여러 area template을 보유하거나 count를 잃음 | style adapter는 최대 한 template을 예약하고 row/column count 및 area iterator를 동일 값에서 제공한다. no-area 기본값은 `None`/0이다. 통과. |
| 15 | Taffy style 변환이 기존 Grid template/auto track을 덮어씀 | area 전달은 기존 rows, columns, line groups, auto rows/columns 필드와 분리된 `CompactTaffyStyle` 입력으로 이어진다. 통과. |
| 16 | Spinon이 auto-repeat 수를 재계산해 Taffy와 불일치 | fixture와 adapter는 symbolic repeat, template areas 및 named placements만 전달한다. repeat count는 Taffy 0.15.0 경로에 맡긴다. 통과. |
| 17 | repeat 축 named placement가 확장 전에 숫자화 | auto-repeat axis projection은 authored named line 및 area alias를 Taffy resolver가 받도록 보존한다. bare `grid-area` 양 축 layout test와 runtime scene이 있다. 통과. |
| 18 | authored `hero-start/end`와 generated area alias 충돌에서 한쪽 이름을 잃음 | dedicated collision scene이 shorthand 및 명시적 area-line placement의 별도 node frame/track을 검사한다. 통과. |
| 19 | 음수 occurrence 또는 named span이 repeat expansion 경계에서 변함 | 기존 auto-repeat placement symbolic path를 재사용하고 numeric/no-area controls를 함께 실행했다. 새 scene은 모든 occurrence/span 조합을 다루지 않으므로 지원 범위 밖 조합은 기존 거부 계약에 남긴다. 통과, 범위 한정. |
| 20 | area matrix가 expanded template보다 짧아 suffix/tail 위치가 바뀜 | columns/rows shorter scene의 전체 node frames와 used tracks가 두 simulator에서 각각 일치한다. 통과. |
| 21 | area matrix가 더 커도 추가 explicit track의 크기·overflow가 잘못됨 | fixed `grid-auto-* : 20px` 추가 track, 16px root overflow 및 root 밖 suffix 표본을 검사한다. 통과. |
| 22 | area 선언만으로 auto-fit 반복 track이 occupied 처리됨 | item 없는 area scene에서 반복 4개가 모두 collapse하고 suffix가 fixed neighbor를 따른다. 양 축 결과 통과. |
| 23 | auto-fit collapsed track 양쪽 gutter가 CSS 규범과 어긋남 | 13-scene expectation은 normative gutter 값을 사용하고 Chrome 156에서 관측한 +4px 차이는 별도 진단으로 남긴다. 통과. |
| 24 | row/column 축 대칭 구현에서 frame/track 혼동 | 13개 고정 장면에 양 축 auto-fill/auto-fit 및 controls가 있고 Android/iOS 각 13/13이 통과했다. 통과. |
| 25 | shorthand parsing 뒤 bare area placement에서 fixed 축을 잘못 매핑 | 기존 grid placement shorthand 흐름을 유지하고 양 축 test와 runtime node frames로 확인했다. 통과. |
| 26 | repeat 축 line-name group의 길이를 physical line-name count로 오인 | repeat 축은 symbolic component boundary와 Taffy named resolver를 사용한다. non-repeat 축 dimension만 authored explicit tracks에 대해 검사한다. 통과. |
| 27 | 잘못된 metadata 오류 뒤 이전/부분 LayoutOutput이 공개 | validator와 conversion이 layout publication 앞에서 오류를 반환한다. 오류 계약과 focused negative tests를 대조했다. 통과. |
| 28 | 새 FFI fixture 또는 JSON이 기본 제품 archive에 포함 | fixture module cfg와 `include_str!` 경계를 Node test가 검사하고, 기본 archive marker/symbol 없음과 opt-in positive control을 기록했다. 통과. 최종 APK/IPA exclusion은 주장하지 않는다. |
| 29 | Android/iOS allowlist·sample 좌표·장면 수가 달라 가짜 시각 통과 | 공통 JSON 13 ID와 양 플랫폼 allowlist를 캡처 도구가 비교한다. PixelCopy/Metal 표본 각각 66/66 exact이며 iOS 결과는 compositor 전이다. 통과. |
| 30 | PR screenshot URL은 유효해도 GitHub에서 이미지가 표시되지 않음 | Agent Browser PR #539 DOM에서 Android PNG `1080×2400`, iOS PNG `1206×2622`의 `complete=true`, `naturalWidth>0`을 확인했다. GitHub는 Checks 0을 표시하고 `gh pr checks --required`에는 보고된 required check이 없으며 merge state는 `CLEAN`이다. 이를 CI 통과로 표현하지 않는다. 통과. |

검토 중 새 코드 결함은 추가로 발견하지 않았다. 앞선 구현/runtime 검토에서 찾아 수정한 fixture module boundary assertion은 해당 PR diff에 포함되어 6/6으로 재확인됐다. 병합 시점의 체크 상태는 이 파일과 별도로 다시 조회한다.
