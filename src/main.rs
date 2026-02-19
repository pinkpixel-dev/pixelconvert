mod batch;
mod converter;
mod preferences;
mod preview;
mod window;

use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

const APP_ID: &str = "dev.pinkpixel.PixelConvert";

fn main() -> glib::ExitCode {
    // Initialize GTK
    gtk4::init().expect("Failed to initialize GTK");

    // TODO: Load resources when we have a gresource file
    // gio::resources_register_include!("pixelconvert.gresource")
    //     .expect("Failed to register resources");

    // Create the application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect activate signal
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &adw::Application) {
    // Set up keyboard shortcuts
    app.set_accels_for_action("win.open", &["<primary>o"]);
    app.set_accels_for_action("win.convert", &["<primary>Return"]);
    app.set_accels_for_action("win.clear", &["<primary><shift>Delete"]);
    app.set_accels_for_action("app.preferences", &["<primary>comma"]);
    app.set_accels_for_action("app.shortcuts", &["<primary>question"]);
    app.set_accels_for_action("app.quit", &["<primary>q"]);

    // Set up app actions
    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(
        #[weak]
        app,
        move |_, _| {
            app.quit();
        }
    ));
    app.add_action(&quit_action);

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(glib::clone!(
        #[weak]
        app,
        move |_, _| {
            show_about_dialog(&app);
        }
    ));
    app.add_action(&about_action);

    let shortcuts_action = gio::SimpleAction::new("shortcuts", None);
    shortcuts_action.connect_activate(glib::clone!(
        #[weak]
        app,
        move |_, _| {
            if let Some(window) = app.active_window() {
                show_shortcuts_window(&window);
            }
        }
    ));
    app.add_action(&shortcuts_action);

    // Create and present the main window
    let window = window::PixelConvertWindow::new(app);

    // Set up window actions
    setup_window_actions(&window);

    window.present();
}

fn setup_window_actions(window: &window::PixelConvertWindow) {
    // Open action
    let open_action = gio::SimpleAction::new("open", None);
    open_action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            window.open_files();
        }
    ));
    window.add_action(&open_action);

    // Convert action
    let convert_action = gio::SimpleAction::new("convert", None);
    convert_action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            window.convert();
        }
    ));
    window.add_action(&convert_action);

    // Clear action
    let clear_action = gio::SimpleAction::new("clear", None);
    clear_action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            window.clear();
        }
    ));
    window.add_action(&clear_action);
}

fn show_about_dialog(app: &adw::Application) {
    let about = adw::AboutDialog::builder()
        .application_name("PixelConvert")
        .application_icon(APP_ID)
        .developer_name("PinkPixel")
        .version("1.0.0")
        .comments("A modern image conversion tool for Linux")
        .website("https://github.com/pinkpixel-dev/pixelconvert")
        .issue_url("https://github.com/pinkpixel-dev/pixelconvert/issues")
        .license_type(gtk4::License::MitX11)
        .developers(vec!["PinkPixel Team".to_string()])
        .build();

    if let Some(window) = app.active_window() {
        about.present(Some(&window));
    }
}

fn show_shortcuts_window(parent: &gtk4::Window) {
    let builder = gtk4::Builder::from_string(
        r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <interface>
          <object class="GtkShortcutsWindow" id="shortcuts">
            <property name="modal">1</property>
            <child>
              <object class="GtkShortcutsSection">
                <property name="section-name">shortcuts</property>
                <child>
                  <object class="GtkShortcutsGroup">
                    <property name="title" translatable="yes">File Operations</property>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Open Files</property>
                        <property name="accelerator">&lt;Primary&gt;o</property>
                      </object>
                    </child>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Convert Images</property>
                        <property name="accelerator">&lt;Primary&gt;Return</property>
                      </object>
                    </child>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Clear Files</property>
                        <property name="accelerator">&lt;Primary&gt;&lt;Shift&gt;Delete</property>
                      </object>
                    </child>
                  </object>
                </child>
                <child>
                  <object class="GtkShortcutsGroup">
                    <property name="title" translatable="yes">Application</property>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Preferences</property>
                        <property name="accelerator">&lt;Primary&gt;comma</property>
                      </object>
                    </child>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Keyboard Shortcuts</property>
                        <property name="accelerator">&lt;Primary&gt;question</property>
                      </object>
                    </child>
                    <child>
                      <object class="GtkShortcutsShortcut">
                        <property name="title" translatable="yes">Quit</property>
                        <property name="accelerator">&lt;Primary&gt;q</property>
                      </object>
                    </child>
                  </object>
                </child>
              </object>
            </child>
          </object>
        </interface>
        "#,
    );

    let shortcuts_window: gtk4::ShortcutsWindow = builder.object("shortcuts").unwrap();
    shortcuts_window.set_transient_for(Some(parent));
    shortcuts_window.present();
}
