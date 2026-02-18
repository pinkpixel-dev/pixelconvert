#!/usr/bin/env python3

import os
import subprocess
import sys

prefix = os.environ.get('MESON_INSTALL_PREFIX', '/usr/local')
datadir = os.path.join(prefix, 'share')

# Compile GSettings schemas
if not os.environ.get('DESTDIR'):
    print('Compiling GSettings schemas...')
    subprocess.call(['glib-compile-schemas',
                     os.path.join(datadir, 'glib-2.0', 'schemas')])

    print('Updating icon cache...')
    subprocess.call(['gtk-update-icon-cache', '-qtf',
                     os.path.join(datadir, 'icons', 'hicolor')])

    print('Updating desktop database...')
    subprocess.call(['update-desktop-database', '-q',
                     os.path.join(datadir, 'applications')])

print('Post-install complete!')
