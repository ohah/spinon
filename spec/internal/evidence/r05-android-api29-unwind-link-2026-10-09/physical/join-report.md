# 13회 synthetic 입력 연결 확인

입력은 `adb shell input tap`으로 발생시킨 실기기 synthetic event다. 직접 손가락 입력 표본으로 세지 않는다.

- 입력: 13개, sequence 1–13
- submit: 13개, sequence/revision 일치, generation 1, `draw_accepted=true`
- SurfaceControl transaction callback: 13개, request ID와 input sequence 1:1
- 복제 fence wait: 13개 `signaled`, usable=true, 마지막 queue pending/active/depth 0
- `target_vsync_id=-1`; VSync, physical scanout, touch latency 계산 안 함.

- input_sequence_is_1_to_13: PASS
- one_draw_accepted_submit_per_input: PASS
- one_callback_per_input: PASS
- one_usable_signaled_fence_wait_per_callback: PASS
- no_runtime_crash_linkage_verifier_or_anr: PASS

submit 로그의 중간 field 순서가 달라 처음 사용한 간단한 정규식이 0건을 반환했다. 원본 로그 field 순서에 맞춘 정규식으로 다시 확인했고 각 sequence가 13회 모두 exact join됐다. 이는 parser 표현식 오류였으며 원본 submit 누락은 아니었다.
