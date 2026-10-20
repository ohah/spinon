# C11.4 직접 이미지 multi-span 계획 실패 관점 검토 20개

**검토 대상:** `plan/c11-4-image-multispan-contributions.md`, Chrome 155 관찰 JSON·screenshot, CSS fixture

**검토 시점:** 구현 전

| # | 실패 관점 | 검토와 조치 |
| ---: | --- | --- |
| 1 | 브라우저 버전이 바뀌어 oracle이 재현되지 않음 | UA에 Chrome `155.0.8059.85`, DPR, screenshot, 원본 observation hash를 고정했다. |
| 2 | 이미지 decode 전에 0×0 값을 golden으로 저장함 | `Promise.all(img.decode())` 뒤에만 frame과 natural size를 읽고, 네 이미지 모두 24×16인지 확인한다. |
| 3 | height 지정에서 width 24/48/72 추정이 틀림 | 같은 PNG의 3:2 natural ratio를 저장한다. Chrome 관찰에서 각 used image frame이 기대 폭을 확인했다. |
| 4 | min-content와 transferred/minimum size를 같은 값이라고 잘못 설명함 | 계획에 Grid §6.6의 `transferred size suggestion`, §11.5의 item contribution과 §11.5.1을 분리해 적는다. |
| 5 | item별 추가 공간을 순서대로 누적해 결과가 달라짐 | spec의 track별 `planned increase` 최대 규칙으로 span-2와 span-3 기대 배열을 각각 유도한다. |
| 6 | DOM 역순 scene이 placement 값까지 바뀜 | ID와 column/row placement는 그대로 두고 자식 DOM 순서만 역전한다. |
| 7 | scene의 expected frame이 DOM order와 node id 대응을 혼동함 | runner는 actual node order를 읽고 expected node identities를 scene source 순서로 생성한다. |
| 8 | auto width image가 grid cell까지 stretch해 intrinsic frame 검사가 무의미해짐 | item의 `justify-self:auto`와 `align-self:auto`를 사용하고 Chrome에서 replaced image intrinsic frame을 확인한다. 부모 `align-items:start`는 cross axis를 지정한다. 앱의 computed alignment 제한으로 `start` 명시값은 허용되지 않아 auto로 맞췄다. |
| 9 | 40px root에서 content-box와 track 합계 경계가 불명확함 | root는 border-box 40×60, 1px padding으로 고정하고 Chrome absolute frame과 root-local item offset을 함께 보존한다. |
| 10 | root inset 때문에 item x/y를 한 CSS pixel 잘못 비교함 | plan은 item local origin에 padding 1px을 포함하고 runner expected frame에 이 offset을 반영하도록 요구한다. |
| 11 | 20px fixed rows와 큰 image height가 column oracle을 오염시킴 | row 1/2/3과 track sizing 축을 고정하고, 높이 overflow는 별도 frame/surface 경계로 판정한다. |
| 12 | 배치 order가 source order대로 자동 placement되어 span test가 달라짐 | 모든 scene의 grid row/column을 명시하고 browser observation에 DOM order와 item positions를 보존한다. |
| 13 | span 2/3의 contribution stage가 같은 그룹으로 합쳐짐 | trace에서 `IntrinsicItemBatch` span 순서를 `[2]` 또는 `[2,3]`으로 정확히 비교한다. |
| 14 | 마지막 0px explicit track 누락으로 기준 배열이 짧아짐 | 네 명시 column을 모두 비교하고 네 번째 zero track를 기대값에 포함한다. |
| 15 | root definite width 때문에 남는 free space가 track에 늘어남 | `min-content` max function과 `justify-content:start`를 고정하며 computed used tracks를 Chrome과 비교한다. |
| 16 | image fixture host와 Chromium에서 source URL/bytes가 다름 | browser는 저장소 PNG를 file URL로 decode하며 앱은 같은 `c14-1-png-basic.png` bytes를 고정 invalid URL에 응답한다. |
| 17 | 같은 natural image map key가 여러 node에 덮어씀 | runtime 입력 map은 각 HostNodeId별 24×16 entry와 공통 revision을 갖도록 검사한다. |
| 18 | PixelCopy에서 PNG texel 검증을 주장함 | 앱 C11.4 image fixture의 red CSS item box와 browser의 PNG texel 화면을 분리하고, texel parity는 범위 밖으로 남긴다. |
| 19 | 표면 좌표가 device pixel과 CSS px를 혼동함 | Android capture adapter가 fixture의 CSS coordinates를 SurfaceView PixelCopy 좌표로 변환하고, root interior/exterior·item overflow anchor를 해시와 함께 기록한다. |
| 20 | 임의 `img`, hidden child, 중복 ID가 지원으로 새어 나옴 | root/image count·ID·src·style·placement 전체 allowlist와 잘못된 source/count의 기대 거부 검사를 구현한다. |

**결과:** 20개 관점을 검토했다. 핵심 계산과 browser observation은 서로 분리했고, 계획에 §6.6 링크를 추가해 definite opposite-axis height와 content/minimum contribution 용어를 구분한다. 구현 후 이 표를 재사용하지 않는다.
