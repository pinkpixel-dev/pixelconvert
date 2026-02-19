cargo_sources = files(
  '../Cargo.toml',
  '../Cargo.lock',
)

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

cargo_options += [ '--offline' ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'pixelconvert',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'cargo', 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / 'pixelconvert', '@OUTPUT@',
  ]
)
