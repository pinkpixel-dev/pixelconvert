use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;

/// Preferences window for PixelConvert
#[allow(dead_code)]
pub struct PreferencesWindow {
    window: adw::PreferencesWindow,
}

#[allow(dead_code)]
impl PreferencesWindow {
    pub fn new(parent: &impl IsA<gtk4::Window>) -> Self {
        let window = adw::PreferencesWindow::builder()
            .transient_for(parent)
            .modal(true)
            .search_enabled(true)
            .build();

        // General preferences page
        let general_page = adw::PreferencesPage::builder()
            .title("General")
            .icon_name("preferences-system-symbolic")
            .build();

        // Appearance group
        let appearance_group = adw::PreferencesGroup::builder().title("Appearance").build();

        let dark_mode_row = adw::SwitchRow::builder()
            .title("Dark Mode")
            .subtitle("Use dark color scheme")
            .build();

        appearance_group.add(&dark_mode_row);

        // Default settings group
        let defaults_group = adw::PreferencesGroup::builder()
            .title("Default Settings")
            .build();

        let quality_row = adw::ActionRow::builder()
            .title("Default Quality")
            .subtitle("Quality setting for new conversions (0-100)")
            .build();

        let quality_spin = gtk4::SpinButton::builder()
            .adjustment(&gtk4::Adjustment::new(85.0, 0.0, 100.0, 1.0, 10.0, 0.0))
            .valign(gtk4::Align::Center)
            .build();

        quality_row.add_suffix(&quality_spin);
        defaults_group.add(&quality_row);

        general_page.add(&appearance_group);
        general_page.add(&defaults_group);

        // Performance page
        let performance_page = adw::PreferencesPage::builder()
            .title("Performance")
            .icon_name("preferences-other-symbolic")
            .build();

        let performance_group = adw::PreferencesGroup::builder()
            .title("Batch Processing")
            .build();

        let threads_row = adw::ActionRow::builder()
            .title("Concurrent Conversions")
            .subtitle("Number of images to process simultaneously")
            .build();

        let threads_spin = gtk4::SpinButton::builder()
            .adjustment(&gtk4::Adjustment::new(4.0, 1.0, 16.0, 1.0, 2.0, 0.0))
            .valign(gtk4::Align::Center)
            .build();

        threads_row.add_suffix(&threads_spin);
        performance_group.add(&threads_row);
        performance_page.add(&performance_group);

        window.add(&general_page);
        window.add(&performance_page);

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }

    // TODO: Connect to GSettings
    // TODO: Save/load preferences
    // TODO: Add more settings (output naming, metadata options, etc.)
}
