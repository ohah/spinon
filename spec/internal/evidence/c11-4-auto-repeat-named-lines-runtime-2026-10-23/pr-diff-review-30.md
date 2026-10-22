# PR 변경 실패 관점 검토 30개

구현 diff와 최종 문서·evidence diff를 공개 계약, 입력 경계, 플랫폼 검증과 재현 가능성 관점에서 각각 확인했다. 아래에서 `통과`는 해당 제한 slice의 코드·테스트·고정 simulator evidence가 서로 모순되지 않았다는 뜻이며, 전체 CSS 적합성을 뜻하지 않는다.

| # | 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | 신규 named-line 동작이 일반 CSS admission으로 우발 승격 | fixture feature에 묶였다. 제한 profile 외 공개 admission을 주장하지 않는다. |
| 2 | cascade가 outer line names를 버림 | N+1 outer groups를 computed snapshot에 보존하고 fixed cascade test가 검사한다. |
| 3 | repeat fragment names가 별도 구조를 잃고 outer index에 섞임 | repeated N+1 names는 auto-repeat metadata로 따로 projection한다. |
| 4 | escaped/case-sensitive identifier가 이름 복사에서 변형 | Stylo computed custom identifier 문자열을 소유 복사하며 named-line tests의 identity guard가 유지된다. |
| 5 | component shorthand의 cascade 순서가 line metadata를 덮어씀 | 기존 `grid-template`/`grid` shorthand와 author stylesheet tests 및 수정 cascade allowlist를 검토했다. |
| 6 | auto-repeat 전 숫자 coercion이 named occurrence를 고정 pattern line으로 바꿈 | Auto-repeat axis는 symbolic `NamedLine`/`NamedSpan`을 Taffy까지 전달한다. |
| 7 | 생략 occurrence가 표준 first-match로 보존되지 않음 | `bare` 고정 scene 결과 `(0,0) 40×40`이 두 플랫폼에서 일치한다. |
| 8 | 음수 occurrence의 end-origin 계산 | `negative` 결과 x=100과 세 반복 track 결과가 독립 기대값에 맞는다. |
| 9 | named span이 occurrence 경계를 넘을 때 길이가 틀림 | `span`은 두 40px track과 10px gutter를 포함해 폭 90px다. |
| 10 | `auto-fit` mode를 fill처럼 배치하거나 line ordinal을 collapse 후 재계산 | collapsed `[0,40,0]`와 item frame을 track trace 및 pixel boundary와 함께 확인했다. |
| 11 | repeat 앞뒤 explicit neighbor를 named index에서 빠뜨림 | `neighbors`는 20/10px neighbors와 반복 40/30px pattern에서 x=120을 기록한다. |
| 12 | 같은 line의 merge 이름이 occurrence 두 개로 세어짐 | repeat boundary merge scene의 line placements와 existing unique-physical-line tests를 대조했다. |
| 13 | 두 이름 그룹을 합칠 때 order나 다른 이름이 유실 | Top-level/repeated vectors의 순서를 별도로 보존하고 `merge` scene을 실행했다. |
| 14 | row 방향에서 columns 가정이 재사용 | `row` scene의 y=50 및 rows `[40,40,40]`이 AVD/iOS 모두 일치한다. |
| 15 | columns와 rows 동시 auto-repeat에서 보조축 placement를 authored count로 풀이 | 두 축 named metadata와 validation 경로를 확인했고 broader dual-axis tests가 계속 통과한다. |
| 16 | non-repeat axis numeric/negative placement 동작 회귀 | 해당 축은 기존 numeric resolver를 유지하고 layout suite 234개가 통과한다. |
| 17 | 없는 이름에서 implicit line을 무제한 생성 | 존재하지 않는 이름은 guard에서 거부하며 fallback은 계획의 제외 범위다. |
| 18 | template-areas와 named auto-repeat 결합이 area mapping을 손상 | profile guard가 조합을 거부하고 negative cascade control을 유지한다. |
| 19 | integer repeat와 auto-repeat 혼합 및 여러 repeat가 부분 지원처럼 통과 | cascade subset validation이 거부한다. 일반 integer-repeat tests 192 style tests에 포함해 통과한다. |
| 20 | non-px neighbor, invalid minimum, arbitrary grammar가 허용 | 기존 fixed px/fixed-minimum guards를 유지하고 negative style cases를 검토했다. |
| 21 | malformed group length가 indexing panic 발생 | list 및 repeat group length를 checked shape로 검증한 뒤 copy한다. |
| 22 | line name bytes/count 또는 track expansion이 overflow | checked arithmetic, 10,000 track cap, metadata byte budget이 유지된다. |
| 23 | track reserve 실패가 일반 Unsupported CSS로 뭉개짐 | named auto-repeat reserve 실패는 node/property를 담은 `GridAllocationFailure`로 반환한다. |
| 24 | line group/name/text 예약 실패 중 partial style snapshot 또는 stale failpoint | 네 failpoint와 다음 성공 cascade recovery를 새로 확인했다. |
| 25 | named string copy가 infallible Clone/duplicate allocation 사용 | placement는 property별 `try_reserve` owned clone을 사용한다. |
| 26 | Taffy compact style에서 line name arrays가 drop됨 | compact style가 Taffy rows/columns name iterator를 구현하며 Clippy 및 layout tests가 통과한다. |
| 27 | Taffy trait 변경 때문에 기존 empty-name Grid가 깨짐 | 기존 non-name/default semantics와 full `spinon-layout` suite 234/234를 확인했다. |
| 28 | simulator fixture scene manifest와 Android/iOS allowlists 불일치 | JSON에서 8 IDs를 열거하고 각 harness가 8/8을 처리한다. final run source diff hash가 같다. |
| 29 | pixel validator가 다른 scene sample을 허용하거나 색 기대값 누락 | sample target/id/coords를 검증하며 PixelCopy와 Metal 각각 29/29 exact다. |
| 30 | 문서가 simulator를 실기기/WPT/full C11.4 증거로 확대하거나 오래된 77/213 갱신 | `STATUS`/conformance에서 checkbox와 77/213을 유지하고 AVD, pre-compositor, static-archive 경계를 명시한다. |

## 수정 후 재검토

Review 중 새 auto-repeat track-reservation failpoint 누락을 발견해 수정했다. Taffy iterator API와 두 축 auto-repeat `Option` 경계의 Clippy findings도 반영했다. 후속 full tests, workspace Clippy, Android/iOS latest-source 8-scene captures를 다시 통과했다. 자세한 실패 관점·실행 SHA와 범위는 [구현/runtime 검토](./implementation-runtime-review-20.md)와 [실행 근거](./README.md)에 기록한다.
