use druid::widget::{Label, ListIter, Scroll, SizedBox};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, FontDescriptor, FontFamily, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

const PADDING_VERTICAL: f64 = 5.0;

pub struct TableDescription<U> {
    pub expand_header: &'static str,
    pub expand_cell: fn(&U) -> String,
    pub header: &'static str,
    pub cell: fn(&U) -> String,
    pub width: f64,
    pub padding: f64,
    pub background: fn(&U) -> Option<Color>,
}

pub struct Table<U, M> {
    width_full: f64,
    expand_text: String,
    expand: WidgetPod<(), SizedBox<()>>,
    headers: Headers,
    content: WidgetPod<M, Scroll<M, Content<U>>>,
}

struct Headers {
    closure: Vec<HeaderDescription>,
    headers: Vec<WidgetPod<(), SizedBox<()>>>,
}

struct Content<U> {
    closure: Vec<CellDescription<U>>,
    expand_text: String,
    cells: Vec<Vec<WidgetPod<U, SizedBox<U>>>>,
}

struct HeaderDescription {
    expand_header: &'static str,
}

struct CellDescription<U> {
    expand_cell: fn(&U) -> String,
    cell: fn(&U) -> String,
    width: f64,
    padding: f64,
    background: fn(&U) -> Option<Color>,
}

impl<U: Data, M: Data + ListIter<U>> Table<U, M> {
    pub fn new(table_description: Vec<TableDescription<U>>) -> Table<U, M> {
        let mut width_full = 0.0;
        for t in &table_description {
            width_full += t.width + t.padding;
        }
        let expand = WidgetPod::new(Label::new("").fix_width(width_full));

        let headers_closure = table_description
            .iter()
            .map(|t| HeaderDescription {
                expand_header: t.expand_header,
            })
            .collect::<Vec<HeaderDescription>>();
        let headers_headers = table_description
            .iter()
            .map(|t| WidgetPod::new(Label::new(t.header).fix_width(t.width + t.padding)))
            .collect::<Vec<WidgetPod<(), SizedBox<()>>>>();

        let content_closure = table_description
            .iter()
            .map(|t| CellDescription {
                expand_cell: t.expand_cell,
                cell: t.cell,
                width: t.width,
                padding: t.padding,
                background: t.background,
            })
            .collect::<Vec<CellDescription<U>>>();
        let content = Content {
            closure: content_closure,
            expand_text: String::new(),
            cells: Vec::new(),
        };

        Table {
            width_full,
            expand_text: String::new(),
            expand,
            headers: Headers {
                closure: headers_closure,
                headers: headers_headers,
            },
            content: WidgetPod::new(Scroll::new(content).vertical()),
        }
    }
}

impl<U: Data, M: Data + ListIter<U>> Widget<M> for Table<U, M> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut M, env: &Env) {
        self.content.widget_mut().event(ctx, event, data, env);

        if let Event::MouseMove(mouse_event) = event {
            let expand_text_previous = self.expand_text.clone();

            if ctx.is_hot() {
                let mut expand_header = false;

                for (j, header) in self.headers.headers.iter_mut().enumerate() {
                    if j == 0 {
                        let header_first_rect = header.layout_rect();
                        if mouse_event.pos.y < header_first_rect.y0
                            || header_first_rect.y1 < mouse_event.pos.y
                        {
                            break;
                        }
                    }
                    header.event(ctx, event, &mut (), env);
                    if header.is_hot() {
                        self.expand_text = self.headers.closure[j].expand_header.to_string();
                        expand_header = true;
                        break;
                    }
                }

                if !expand_header {
                    self.expand_text = self.content.widget().child().expand_text.clone();
                }
            } else {
                self.expand_text = String::new();
            }

            if self.expand_text != expand_text_previous {
                self.expand =
                    WidgetPod::new(Label::new(self.expand_text.clone()).fix_width(self.width_full));
                ctx.children_changed();
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &M, env: &Env) {
        self.expand.lifecycle(ctx, event, &(), env);

        for header in self.headers.headers.iter_mut() {
            header.lifecycle(ctx, event, &(), env);
        }

        self.content.widget_mut().lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &M, data: &M, env: &Env) {
        if data.data_len() != old_data.data_len() {
            self.content.widget_mut().update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &M, env: &Env) -> Size {
        let mut size = Size::ZERO;
        let mut origin = Point::new(0.0, 0.0);

        let expand_layout = self.expand.layout(ctx, bc, &(), env);
        self.expand.set_origin(ctx, &(), env, origin);

        size.width += expand_layout.width;
        size.height += expand_layout.height + PADDING_VERTICAL;

        origin.x = 0.0;
        origin.y = size.height;
        for (j, h) in self.headers.headers.iter_mut().enumerate() {
            let header_layout = h.layout(ctx, bc, &(), env);

            h.set_origin(ctx, &(), env, origin);
            origin.x += header_layout.width;

            if j == 0 {
                size.height += header_layout.height + PADDING_VERTICAL;
            }
        }

        let content_layout = self.content.widget_mut().child_mut().layout(ctx, bc, data, env);
        origin.x = 0.0;
        origin.y = size.height;
        self.content.set_origin(ctx, data, env, origin);

        size.height += content_layout.height;

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &M, env: &Env) {
        self.expand.paint(ctx, &(), env);

        for header in self.headers.headers.iter_mut() {
            header.paint(ctx, &(), env);
        }

        self.content.widget_mut().paint(ctx, data, env);
    }
}

impl<U: Data, M: Data + ListIter<U>> Widget<M> for Content<U> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut M, env: &Env) {
        if let Event::MouseMove(mouse_event) = event {
            let expand_text_previous = self.expand_text.clone();

            let mut expand_cell = false;

            if ctx.is_hot() {
                let mut rows = self.cells.iter_mut();
                data.for_each_mut(|data_row, _| {
                    if let Some(row) = rows.next() {
                        for (j, cell) in row.iter_mut().enumerate() {
                            if j == 0 {
                                if expand_cell {
                                    break;
                                }

                                let cell_first_rect = cell.layout_rect();
                                if mouse_event.pos.y < cell_first_rect.y0
                                    || cell_first_rect.y1 < mouse_event.pos.y
                                {
                                    break;
                                }
                            }
                            cell.event(ctx, event, data_row, env);
                            if cell.is_hot() {
                                self.expand_text = (self.closure[j].expand_cell)(data_row);
                                expand_cell = true;
                                break;
                            }
                        }
                    }
                });
            }

            if !expand_cell {
                self.expand_text = String::new();
            }

            if expand_text_previous != self.expand_text {
                ctx.children_changed();
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &M, env: &Env) {
        let mut rows = self.cells.iter_mut();
        data.for_each(|data_row, _| {
            if let Some(row) = rows.next() {
                for cell in row.iter_mut() {
                    cell.lifecycle(ctx, event, data_row, env);
                }
            }
        });
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &M, data: &M, _env: &Env) {
        if data.data_len() != old_data.data_len() {
            self.cells = Vec::with_capacity(data.data_len());

            data.for_each(|data_row, _| {
                let row = self
                    .closure
                    .iter()
                    .map(|t| {
                        WidgetPod::new(match (t.background)(data_row) {
                            Some(color) => Label::new((t.cell)(data_row))
                                .with_font(FontDescriptor::new(FontFamily::MONOSPACE))
                                .background(color)
                                .fix_width(t.width),
                            None => Label::new((t.cell)(data_row))
                                .with_font(FontDescriptor::new(FontFamily::MONOSPACE))
                                .fix_width(t.width),
                        })
                    })
                    .collect::<Vec<WidgetPod<U, SizedBox<U>>>>();

                self.cells.push(row);
            });

            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &M, env: &Env) -> Size {
        let mut size = Size::ZERO;
        let mut origin = Point::new(0.0, 0.0);

        for t in &self.closure {
            size.width += t.width + t.padding;
        }

        let mut rows = self.cells.iter_mut();
        data.for_each(|data_row, _| {
            if let Some(row) = rows.next() {
                origin.x = 0.0;
                origin.y = size.height;
                for (j, cell) in row.iter_mut().enumerate() {
                    let cell_layout = cell.layout(ctx, bc, data_row, env);
                    cell.set_origin(ctx, data_row, env, origin);
                    origin.x += cell_layout.width + self.closure[j].padding;
                    if j == 0 {
                        size.height += cell_layout.height;
                    }
                }
            }
        });

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &M, env: &Env) {
        let mut rows = self.cells.iter_mut();
        data.for_each(|data_row, _| {
            if let Some(row) = rows.next() {
                for cell in row.iter_mut() {
                    cell.paint(ctx, data_row, env);
                }
            }
        });
    }
}
