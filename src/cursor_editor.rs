use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::text::{self, Paragraph as _, Renderer as _};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::mouse;
use iced::{Color, Element, Event, Font, Length, Rectangle, Size, Theme, Vector};

type Renderer = iced::Renderer;

pub struct CursorEditor<'a, Message> {
    inner: Element<'a, Message>,
    vim_line: usize,
    vim_col: usize,
    show_cursor: bool,
}

impl<'a, Message> CursorEditor<'a, Message> {
    pub fn new(
        inner: Element<'a, Message>,
        vim_line: usize,
        vim_col: usize,
        show_cursor: bool,
    ) -> Self {
        Self { inner, vim_line, vim_col, show_cursor }
    }
}

impl<Message> Widget<Message, Theme, Renderer> for CursorEditor<'_, Message>
{
    fn size(&self) -> Size<Length> {
        self.inner.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.inner.as_widget_mut().layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.inner.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );

        if !self.show_cursor {
            return;
        }

        let bounds = layout.bounds();
        let font_size = renderer.default_size();
        let line_height = font_size.0 * 1.3;

        let char_para = <Renderer as text::Renderer>::Paragraph::with_text(
            text::Text {
                content: "M",
                bounds: Size::new(f32::INFINITY, f32::INFINITY),
                size: font_size,
                line_height: text::LineHeight::default(),
                font: Font::MONOSPACE,
                align_x: iced::alignment::Horizontal::Left.into(),
                align_y: iced::alignment::Vertical::Top,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::None,
            },
        );
        let char_width = char_para.min_bounds().width;

        let padding = 5.0;
        let x = bounds.x + padding + self.vim_col as f32 * char_width;
        let y = bounds.y + padding + self.vim_line as f32 * line_height;

        let cursor_rect = Rectangle {
            x,
            y,
            width: char_width,
            height: line_height,
        };

        if cursor_rect.intersection(&bounds).is_none() {
            return;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds: cursor_rect,
                ..Default::default()
            },
            Color::from_rgba(0.7, 0.7, 0.7, 0.5),
        );
    }

    fn tag(&self) -> widget::tree::Tag {
        self.inner.as_widget().tag()
    }

    fn state(&self) -> widget::tree::State {
        self.inner.as_widget().state()
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(self.inner.as_widget())]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        self.inner.as_widget().diff(&mut tree.children[0]);
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.inner.as_widget_mut().operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.inner.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.inner.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.inner.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<CursorEditor<'a, Message>> for Element<'a, Message>
where
    Message: Clone,
{
    fn from(w: CursorEditor<'a, Message>) -> Element<'a, Message> {
        Element::new(w)
    }
}
