use std::rc::Rc;

use gdk;
use gdk::{Display, WindowExt};
use gtk::{GLAreaExt, WidgetExt};
use servo::BrowserId;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::WindowMethods;
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::gl;
use servo::ipc_channel::ipc;
use servo::msg::constellation_msg::{Key, KeyModifiers};
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::LoadData;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::cursor::Cursor;
use servo::style_traits::DevicePixel;

use view::View;

pub struct GtkWindow {
    gl: Rc<gl::Gl>,
    view: View,
    waker: Box<EventLoopWaker>,
}

impl GtkWindow {
    pub fn new(gl: Rc<gl::Gl>, view: View, waker: Box<EventLoopWaker>) -> Self {
        GtkWindow {
            gl,
            view,
            waker,
        }
    }
}

impl WindowMethods for GtkWindow {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        self.view.make_current();
        true
    }

    fn present(&self) {
        self.view.queue_render();
    }

    fn supports_clipboard(&self) -> bool {
        false
    }

    fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
        self.waker.clone()
    }

    fn gl(&self) -> Rc<gl::Gl> {
        self.gl.clone()
    }

    fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
        ScaleFactor::new(self.view.get_scale_factor() as f32)
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        let window = self.view.get_window().unwrap();
        let width = window.get_width();
        let height = window.get_height();
        let scale_factor = self.view.get_scale_factor() as u32;
        TypedSize2D::new(scale_factor * width as u32, scale_factor * height as u32)
    }

    fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
        TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
    }

    fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
        let window = self.view.get_window().unwrap();
        let width = window.get_width();
        let height = window.get_height();
        TypedSize2D::new(width as f32, height as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        let window = self.view.get_window().unwrap();
        let width = window.get_width();
        let height = window.get_height();
        let (x, y) = window.get_position();
        (Size2D::new(width as u32, height as u32), Point2D::new(x as i32, y as i32))
    }

    fn set_page_title(&self, _id: BrowserId, title: Option<String>) {
        let window = self.view.get_window().unwrap();
        window.set_title(match title {
            Some(ref title) => title,
            None => "",
        });
    }

    fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) {
        chan.send(true).ok();
    }

    fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {
    }

    fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {
    }

    fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {
    }

    fn status(&self, _id: BrowserId, _status: Option<String>) {
    }

    fn load_start(&self, _id: BrowserId) {
    }

    fn load_end(&self, _id: BrowserId) {
    }

    fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {
    }

    fn head_parsed(&self, _id: BrowserId) {
    }

    fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {
    }

    fn set_cursor(&self, cursor: Cursor) {
        let cursor_name = match cursor {
            Cursor::Pointer => "pointer",
            _ => "default",
        };
        let display = Display::get_default().unwrap();
        let cursor = gdk::Cursor::new_from_name(&display, cursor_name);
        let window = self.view.get_window().unwrap();
        window.set_cursor(&cursor);
    }

    fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {
    }

    fn handle_key(&self, _id: Option<BrowserId>, _ch: Option<char>, _key: Key, _mods: KeyModifiers) {
    }
}