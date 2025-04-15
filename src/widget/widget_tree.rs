use crate::{rendering::render_object::Constraints, RenderObject, Widget};

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
                .map(|&child| WidgetNode::new(unsafe { Box::from_raw(child) }))
                .collect()
        };
        let tree = WidgetTree {
            root: WidgetRootNode {
                widget: root,
                children,
                render_object: None, // TODO initialize render object
            },
        };

        return tree;
    }

    pub fn draw(&mut self, hwnd_parent: windows::Win32::Foundation::HWND) {
        match self.root.render_object.as_mut() {
            Some(render_object) => {
                render_object.parent = Some(hwnd_parent);
                render_object.draw();
            }
            None => {
                panic!("Render object is None");
            }
        }
    }
}

struct WidgetRootNode {
    widget: Box<dyn Widget>,
    children: Vec<WidgetNode>,
    render_object: Option<RenderObject>,
}
impl WidgetRootNode {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let width = widget.width();
        let height = widget.height();
        let color = widget.color();
        let children =  {
            widget
                .as_ref()
                .children()
                .iter()
                .map(|&child| WidgetNode::new(unsafe { Box::from_raw(child) }))
                .collect()
        };
        WidgetRootNode {
            widget,
            children,
            render_object: Some(RenderObject {
                id: 0,
                constraints: Constraints{ 
                    min_width: 0,
                    min_height: 0,
                    max_width: Some(1000),
                    max_height: Some(1000),
                    width: Some(width),
                    height: Some(height),
                },
                x: 0,
                y: 0,
                color,
                handle: None,
                parent: None,
                children: vec![],
                is_visible: true,
                is_enabled: true,
                input_handler: None,
            }),
        }
    }
}

struct WidgetNode {
    widget: Box<dyn Widget>,
    children: Vec<WidgetNode>,
    render_object: Option<RenderObject>,
}

impl WidgetNode {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let width = widget.width();
        let height = widget.height();
        let color = widget.color();
        let children =  {
            widget
                .as_ref()
                .children()
                .iter()
                .map(|&child| WidgetNode::new(unsafe { Box::from_raw(child) }))
                .collect()
        };
        WidgetNode {
            widget,
            children,
            render_object: Some(RenderObject {
                id: 0,
                constraints: Constraints{ 
                    min_width: 0,
                    min_height: 0,
                    max_width: Some(1000),
                    max_height: Some(1000),
                    width: Some(width),
                    height: Some(height),
                },
                x: 0,
                y: 0,
                color,
                handle: None,
                parent: None,
                children: vec![],
                is_visible: true,
                is_enabled: true,
                input_handler: None,
            }),
        }
    }

}
