use iced::border;
use iced::keyboard;
use iced::mouse;
use iced::widget::{
    button, canvas, center, checkbox, column, container, horizontal_space,
    pick_list, row, scrollable, text,
};
use iced::{
    color, Center, Element, Fill, Font, Length, Point, Rectangle, Renderer,
    Subscription, Theme,
};

pub fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}

#[derive(Default, Debug)]
struct Layout {
    example: Example,
    explain: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    ClickButton,
    ClickButtonUp1,
    ClickButtonUp2,
    ClickButtonUp3,
    ClickButtonDown1,
    ClickButtonDown2,
    ClickButtonDown3,
    ExplainToggled(bool),
    ThemeSelected(Theme),
}

impl Layout {
    fn title(&self) -> String {
        format!("{} - Layout - Iced", self.example.title)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.example = self.example.next();
            }
            Message::Previous => {
                self.example = self.example.previous();
            }
            Message::ClickButton => {
                self.example.button_click();
            }
            Message::ClickButtonUp1 => {
                self.example.button_click_up_1();
            }
            Message::ClickButtonUp2 => {
                self.example.button_click_up_2();
            }
            Message::ClickButtonUp3 => {
                self.example.button_click_up_3();
            }
            Message::ClickButtonDown1 => {
                self.example.button_click_down_1();
            }
            Message::ClickButtonDown2 => {
                self.example.button_click_down_2();
            }
            Message::ClickButtonDown3 => {
                self.example.button_click_down_3();
            }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_release(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => {
                Some(Message::Previous)
            }
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text(self.example.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain)
                .on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_y(Center);

        let example = center(if self.explain {
            self.example.view().explain(color!(0x0000ff))
        } else {
            self.example.view()
        })
        .style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .border(border::color(palette.background.strong.color).width(4))
        })
        .padding(4);

        let controls = row([
            (!self.example.is_first()).then_some(
                button("← Previous")
                    .padding([5, 10])
                    .on_press(Message::Previous)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self.example.is_last()).then_some(
                button("Next →")
                    .padding([5, 10])
                    .on_press(Message::Next)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        let controls2 = row([
            (!self.example.is_first()).then_some(
                button("← Random Button →")
                    .padding([5, 10])
                    .on_press(Message::ClickButton)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self.example.is_last()).then_some(
                button("← Random Button →")
                    .padding([5, 10])
                    .on_press(Message::ClickButton)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        column![header, example, controls, controls2]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Example {
    title: &'static str,
    view: fn() -> Element<'static, Message>,
}

impl Example {
    const LIST: &'static [Self] = &[
        Self {
            title: "Centered",
            view: centered,
        },
        Self {
            title: "Column",
            view: column_,
        },
        Self {
            title: "Row",
            view: row_,
        },
        Self {
            title: "Space",
            view: space,
        },
        Self {
            title: "Application",
            view: application,
        },
        Self {
            title: "Column2",
            view: column_,
        },
        Self {
            title: "Row2",
            view: row_,
        },
        Self {
            title: "RowAndColumn",
            view: row_and_column_,
        },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) =
            Self::LIST.iter().position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) =
            Self::LIST.iter().position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST.get(index + 1).copied().unwrap_or(self)
    }

    fn view(&self) -> Element<Message> {
        (self.view)()
    }

    fn button_click(&self) {
        println!("... button click ... ");
    }

    fn button_click_up_1(&self) {
        println!("... button click up 1 ... ");
    }

    fn button_click_up_2(&self) {
        println!("... button click up 2 ... ");
    }

    fn button_click_up_3(&self) {
        println!("... button click up 3 ... ");
    }

    fn button_click_down_1(&self) {
        println!("... button click down 1 ... ");
    }

    fn button_click_down_2(&self) {
        println!("... button click down 2 ... ");
    }

    fn button_click_down_3(&self) {
        println!("... button click down 3 ... ");
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn centered<'a>() -> Element<'a, Message> {
    center(text("I am centered!").size(50)).into()
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically.",
        square(50),
        square(50),
        square(50),
        "The amount of space between",
        "elements can be configured!",
    ]
    .spacing(40)
    .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![
        "A row works like a column...",
        square(50),
        square(50),
        square(50),
        "but lays out widgets horizontally!",
    ]
    .spacing(40)
    .into()
}

fn row_and_column_<'a>() -> Element<'a, Message> {
    column![
        "column ... ",
        square(50),
        
        row![
            "row ... ",
            square(50),
            button("^")
                .padding([5, 10])
                .on_press(Message::ClickButtonUp1),
            button("^")
                .padding([5, 10])
                .on_press(Message::ClickButtonUp2),
            button("^")
                .padding([5, 10])
                .on_press(Message::ClickButtonUp3),
        ]
        .spacing(40),

        row![
            "row ... ",
            square(50),
            button("v")
            .padding([5, 10])
            .on_press(Message::ClickButtonDown1),
            button("v")
            .padding([5, 10])
            .on_press(Message::ClickButtonDown2),
            button("v")
            .padding([5, 10])
            .on_press(Message::ClickButtonDown3),
        ]
        .spacing(40),

        row![
            "row ... ",
            square(50),
            square(50),
            square(50),
        ]
        .spacing(40),

        "... ",
    ]
    .spacing(40)
    .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_y(Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default()
            .border(border::color(palette.background.strong.color).width(1))
    });

    let sidebar = container(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_x(Center),
    )
    .style(container::rounded_box)
    .center_y(Fill);

    let content = container(
        scrollable(
            column![
                "Content!",
                row((1..10).map(|i| square(if i % 2 == 0 { 80 } else { 160 })))
                    .spacing(20)
                    .align_y(Center)
                    .wrap(),
                "The end"
            ]
            .spacing(40)
            .align_x(Center)
            .width(Fill),
        )
        .height(Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
