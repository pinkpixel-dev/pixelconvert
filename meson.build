project(
  'pixelconvert',
  'rust',
  version: '1.0.0',
  meson_version: '>= 0.59',
  license: 'Apache-2.0',
)

application_id = 'org.pinkpixel.PixelConvert'
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
localedir = prefix / get_option('localedir')

# Dependencies
dependency('gtk4', version: '>= 4.12')
dependency('libadwaita-1', version: '>= 1.5')

# Subdirectories
subdir('data')
subdir('src')

# Post-install scripts
gnome = import('gnome')
glib_compile_schemas = find_program('glib-compile-schemas', required: false)
if glib_compile_schemas.found()
  meson.add_install_script('build-aux/meson/postinstall.py')
endif
