use druid::im::Vector;
use druid::widget::{Button, Flex, ListIter};
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};

mod table;
use crate::table::{Table, TableDescription};

#[derive(Clone, Data, Lens)]
struct Trees {
    trees: Vector<Tree>,
    counter: usize,
}

#[derive(Clone, Data, Lens)]
struct Tree {
    name_english: &'static str,
    name_latin: &'static str,
    typical_height_m: Option<u8>,
    identifiable_features: &'static str,
}

const TREES: [Tree; 5] = [
    Tree {
        name_english: "Cedar",
        name_latin: "Cedrus",
        typical_height_m: None,
        identifiable_features: "Scale leaves in spiral clusters",
    },
    Tree {
        name_english: "Juniper",
        name_latin: "Juniperus",
        typical_height_m: Some(30),
        identifiable_features: "Scale leaves; dark blue berry-like cones",
    },
    Tree {
        name_english: "Oak",
        name_latin: "Quercus",
        typical_height_m: None,
        identifiable_features: "Lobed leaves; Acorns",
    },
    Tree {
        name_english: "Sycamore",
        name_latin: "Platanus",
        typical_height_m: None,
        identifiable_features: "Thin, peeling reddish-brown bark",
    },
    Tree {
        name_english: "Willow",
        name_latin: "Salix",
        typical_height_m: None,
        identifiable_features: "Long drooping branches",
    },
];

impl ListIter<Tree> for Trees {
    fn for_each(&self, mut cb: impl FnMut(&Tree, usize)) {
        for (i, item) in self.trees.iter().enumerate() {
            cb(item, i);
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Tree, usize)) {
        for (i, item) in self.trees.clone().iter().enumerate() {
            let mut new_item = item.to_owned();
            cb(&mut new_item, i);

            if !new_item.same(item) {
                self.trees[i] = new_item;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.trees.len()
    }
}

fn main() {
    let window = WindowDesc::new(build_root_widget())
        .title("Types of Trees")
        .window_size((700.0, 400.0));

    AppLauncher::with_window(window)
        .launch(Trees { trees: Vector::new(), counter: 0 })
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<Trees> {
    let table_description = vec![
        TableDescription {
            expand_header: "Name (English)",
            expand_cell: |t: &Tree| t.name_english.to_string(),
            header: "Name",
            cell: |t: &Tree| t.name_english.to_string(),
            width: 70.0,
            padding: 0.0,
            background: |_| None,
        },
        TableDescription {
            expand_header: "Name (Latin)",
            expand_cell: |t: &Tree| t.name_latin.to_string(),
            header: "Latin",
            cell: |t: &Tree| t.name_latin.to_string(),
            width: 70.0,
            padding: 0.0,
            background: |_| None,
        },
        TableDescription {
            expand_header: "Typical Height (m)",
            expand_cell: |t: &Tree| {
                if let Some(h) = t.typical_height_m {
                    format!("{h} m")
                } else {
                    String::from("No Data")
                }
            },
            header: "Height",
            cell: |t: &Tree| {
                if let Some(h) = t.typical_height_m {
                    h.to_string()
                } else {
                    String::new()
                }
            },
            width: 50.0,
            padding: 0.0,
            background: |_| None,
        },
        TableDescription {
            expand_header: "Identifiable Features",
            expand_cell: |t: &Tree| t.identifiable_features.to_string(),
            header: "Features",
            cell: |t: &Tree| t.identifiable_features.to_string(),
            width: 500.0,
            padding: 0.0,
            background: |_| None,
        },
    ];

    let button = Button::new("Add").on_click(|_ctx, data: &mut Trees, _env| {
        if data.counter < TREES.len() {
            data.trees.push_back(TREES[data.counter].clone());
            data.counter += 1;
        }
    });

    Flex::column()
        .with_child(button)
        .with_spacer(30.0)
        .with_child(Table::new(table_description))
}