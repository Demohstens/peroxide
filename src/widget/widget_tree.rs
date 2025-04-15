use crate::Widget;

pub struct WidgetTree {
    root: WidgetRootNode,
}

impl WidgetTree {
    pub fn new(root: Box<dyn Widget>) -> Self {
        let children = unsafe {
            root
                .as_ref()
                .children()
                .iter()
                .map(|child| WidgetNode::new(*child))
                .collect()
        };
        let tree = WidgetTree {
            root: WidgetRootNode {
                widget: root,
                children,
            },
        };

        return tree;
    }

    pub fn draw(&self, hwnd_parent: windows::Win32::Foundation::HWND) {
        self.root.widget.draw(hwnd_parent);
    }
}

struct WidgetRootNode {
    widget: Box<dyn Widget>,
    children: Vec<WidgetNode>,
}
impl WidgetRootNode {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let children = unsafe {
            widget
                .as_ref()
                .children()
                .iter()
                .map(|child| WidgetNode::new(*child))
                .collect()
        };
        WidgetRootNode {
            widget,
            children,
        }
    }
}

#[derive(Debug)]
struct WidgetNode {
    widget: *mut dyn Widget,
    children: Vec<WidgetNode>,
}

impl WidgetNode {
    pub fn new(widget: *mut dyn Widget) -> Self {
        WidgetNode {
            widget,
            children: unsafe {
                widget
                    .as_ref()
                    .unwrap()
                    .children()
                    .iter()
                    .map(|child| WidgetNode::new(*child))
                    .collect()
            },
        }
    }

}
