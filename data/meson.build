desktop_file = configure_file(
  input: 'dev.pinkpixel.PixelConvert.desktop.in',
  output: 'dev.pinkpixel.PixelConvert.desktop',
  configuration: {
    'bindir': bindir,
  },
  install: true,
  install_dir: datadir / 'applications'
)

# Validate desktop file
desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

# AppStream metadata
appstream_file = configure_file(
  input: 'dev.pinkpixel.PixelConvert.metainfo.xml.in',
  output: 'dev.pinkpixel.PixelConvert.metainfo.xml',
  configuration: {
    'appid': application_id,
  },
  install: true,
  install_dir: datadir / 'metainfo'
)

# Validate appstream file
appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test('Validate appstream file',
    appstreamcli,
    args: ['validate', '--no-net', '--explain', appstream_file]
  )
endif

# GSettings schema
install_data(
  'dev.pinkpixel.PixelConvert.gschema.xml',
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

# Compile schemas for development
compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file',
    compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()]
  )
endif

subdir('icons')
