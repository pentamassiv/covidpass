use super::*;

pub struct DetailPageContent {}
pub struct DetailPage {
    toast_overlay: adw::ToastOverlay,
    detail_page_content: gtk::Box,
    content: Option<DetailPageContent>,
    view_stack: adw::ViewStack,
}

impl DetailPage {
    pub fn new(view_stack: &adw::ViewStack, toast_overlay: adw::ToastOverlay) -> Self {
        let detail_page_content = gtk::Box::new(Orientation::Vertical, 0);
        let label = gtk::Label::new(Some("Name"));
        let label_name = gtk::Label::new(Some("Max"));
        detail_page_content.append(&label);
        detail_page_content.append(&label_name);

        let detail_page = Self {
            toast_overlay: toast_overlay.clone(),
            detail_page_content,
            content: None,
            view_stack: view_stack.clone(),
        };
        detail_page
    }

    fn connect(&self) {}
    pub fn content(&self) -> &gtk::Box {
        &self.detail_page_content
    }
}
