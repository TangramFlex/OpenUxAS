lib_tasks_rs = static_library(
  'tasks_rs',
  'src/lib.rs',
  rust_crate_type: 'staticlib',
  rust_args: ['--cfg', 'meson'],
  link_with: lib_lmcp_rs,
)
