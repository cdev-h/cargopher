pub mod tui {

    use cursive::{
        menu::MenuTree,
        theme::{BaseColor, Color, ColorStyle, PaletteColor},
        view::{Boxable, Scrollable},
        views::{Dialog, Layer, LinearLayout, TextView},
        Cursive,
    };

    use crate::{gopher::gopher, tcp::tcp};

    fn display_url_bar(siv: &mut Cursive) {
        siv.pop_layer(); // remove the current layer being displayed (either the welcome message or the gopher website)
        siv.add_layer(Dialog::info("insert url"));
    }

    fn print_url_to_screen(siv: &mut Cursive, host: String) {
        match tcp::go_to("gopher://gopher.floodgap.com", 70, "/") {
            Ok(get_gopher) => {
                let parser = gopher::Parser::new(&get_gopher);

                let mut layout = LinearLayout::vertical();

                for item in parser.page {
                    let text_view = TextView::new(item.title).style(ColorStyle::new(
                        Color::Light(BaseColor::White),
                        Color::TerminalDefault,
                    ));

                    layout.add_child(text_view);
                }

                let layer = Layer::with_color(
                    layout,
                    ColorStyle::new(Color::TerminalDefault, Color::TerminalDefault),
                )
                .full_screen()
                .scrollable();

                siv.add_layer(layer);
            }
            Err(err) => {
                siv.pop_layer();
                siv.add_layer(Dialog::info(err.get_message()));
            }
        }
    }

    fn init_menu(siv: &mut Cursive) {
        siv.menubar()
            .add_subtree(
                "Browser",
                MenuTree::new()
                    .leaf("Open URL", display_url_bar)
                    .leaf("Quit", |s| s.quit()),
            )
            .add_delimiter();

        siv.set_autohide_menu(false);
        siv.add_global_callback('f', |s| s.select_menubar());
    }

    fn init_browser(siv: &mut Cursive) {
        let text_view = TextView::new(
            "Hello, welcome to Cartographer.\nPress <F> to focus the menubar or <Q> to quit.",
        )
        .style(ColorStyle::new(
            Color::Light(BaseColor::White),
            Color::TerminalDefault,
        ));

        let layout = LinearLayout::horizontal().child(text_view);
        let layer = Layer::with_color(
            layout,
            ColorStyle::new(Color::TerminalDefault, Color::TerminalDefault),
        )
        .scrollable();

        siv.add_layer(layer);

        siv.add_global_callback('q', |s| s.quit());
    }

    pub fn init() {
        let mut siv = cursive::default();
        let mut theme = siv.current_theme().clone();

        theme.palette[PaletteColor::Background] = Color::TerminalDefault;

        siv.set_theme(theme);

        init_menu(&mut siv);
        init_browser(&mut siv);

        siv.run();
    }
}
