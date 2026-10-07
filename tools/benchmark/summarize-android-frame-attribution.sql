-- trace_processor query -f tools/benchmark/summarize-android-frame-attribution.sql <frame-attribution.pftrace>

SELECT
  name,
  COUNT(*) AS samples,
  ROUND(MAX(dur) / 1e6, 3) AS max_ms,
  ROUND(SUM(dur) / 1e6, 3) AS total_ms
FROM slice
WHERE name GLOB 'SpinonR05:*'
GROUP BY name
ORDER BY name;

SELECT
  ct.name,
  COUNT(*) AS samples,
  MIN(c.value) AS first_value,
  MAX(c.value) AS last_value
FROM counter c
JOIN counter_track ct ON c.track_id = ct.id
WHERE ct.name GLOB 'SpinonR05*'
GROUP BY ct.name
ORDER BY ct.name;

SELECT
  p.name AS process,
  a.layer_name,
  a.jank_type,
  a.present_type,
  a.on_time_finish,
  COUNT(*) AS frame_timeline_rows,
  COUNT(DISTINCT a.surface_frame_token) AS unique_surface_frames,
  COUNT(DISTINCT a.display_frame_token) AS unique_display_frames
FROM actual_frame_timeline_slice a
LEFT JOIN process p USING (upid)
GROUP BY p.name, a.layer_name, a.jank_type, a.present_type, a.on_time_finish
ORDER BY p.name, a.layer_name, frame_timeline_rows DESC;

SELECT
  name,
  idx,
  severity,
  source,
  value
FROM stats
WHERE name = 'ftrace_setup_errors'
   OR name GLOB 'ftrace_cpu_*dropped*'
   OR name IN (
     'ftrace_cpu_has_data_loss',
     'traced_buf_chunks_overwritten',
     'traced_buf_chunks_discarded',
     'traced_buf_trace_writer_packet_loss',
     'traced_buf_sequence_packet_loss',
     'traced_buf_incremental_sequences_dropped',
     'traced_buf_data_loss_read_gap'
   )
ORDER BY name, idx;
