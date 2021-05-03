use gtk;
use webkit2gtk;

use gtk::{ContainerExt, Inhibit, WidgetExt, Window, WindowType};
use webkit2gtk::{WebView, WebViewExt};

fn main() {
	gtk::init().unwrap();

	let window = Window::new(WindowType::Toplevel);
	let webview = WebView::new();
	webview.load_uri("https://webk.telegram.org/");
	window.add(&webview);

	window.show_all();

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	gtk::main();
}
