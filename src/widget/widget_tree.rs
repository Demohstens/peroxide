use std::{os::raw::c_void, rc::Rc};

use crate::{rendering::render_object::Constraints, RenderObject, Widget};


/// The Widget tree is the main structure that holds the widgets and their relationships.
/// Widgets merely control how it is created. The tree stores state, layout, and rendering information.
/// Therefor it owns the widgets and their Render Objects and manages them independently of oneanother.
pub struct WidgetTree {
    root: WidgetRootNode,
}

impl WidgetTree {
    pub fn new(root: Box<dyn Widget>) -> Self {
        
        let widget = Rc::new(root);
        let tree = WidgetTree {
            root: WidgetRootNode::new(widget)
        };

        return tree;
    }

    pub fn draw(&mut self, hwnd_parent: *mut c_void) {
        match self.root.render_object.as_mut() {
            Some(render_object) => {
                render_object.parent = Some(hwnd_parent);
                render_object.draw();
                for child in self.root.children.iter_mut() {
                    match child.render_object() {
                        Some(ro) => {
                            ro.parent = Some(render_object.handle.unwrap());
                            ro.draw();
                        },
                        None => unreachable!()
                    }
                    
                }
            }
            None => {
                panic!("Render object is None");
            }
        }
    }
}

struct WidgetRootNode {
    widget: Rc<Box<dyn Widget>>,
    children: Vec<WidgetNode>,
    render_object: Option<RenderObject>,
}
impl WidgetRootNode {
    

    pub fn new(widget: Rc<Box<dyn Widget>>) -> Self {
        let width = widget.width();
        let height = widget.height();
        let color = widget.color();
        let x = widget.x();
        let y = widget.y();
        let id = widget.id();
        let children: Vec<WidgetNode> =  {
            widget
                .as_ref()
                .children()
                .iter()
                .map(|child| WidgetNode::new( Rc::clone(child)))
                .collect()
        };
        println!("Root node has {:?} children", widget.children().len());
        let render_object  = RenderObject {
            id: id as u32,
            constraints: Constraints{
                min_width: width,
                min_height: height,
                max_width: Some(1000),
                max_height: Some(1000),
                width: Some(width),
                height: Some(height),
            }, 
            x: x,
            y: y,
            color: color,
            handle: None,
            parent: None,
            // children: vec![],
            is_visible: true,
            is_enabled: true,
            input_handler: None,
        };
        WidgetRootNode {
            widget,
            children,
            render_object: Some(render_object),
        }
    }
}

struct WidgetNode {
    widget: Rc<dyn Widget>,
    children: Vec<WidgetNode>,
    render_object: Option<RenderObject>,
}

impl WidgetNode {

    pub fn render_object(&mut self) -> Option<&mut RenderObject> {
        match self.render_object {
            Some(ref mut render_object) => {
                Some(render_object)
            }
            None => {
                if (self.children.len() == 0) {
                    None 
                } else {
                    todo!("Construct render child");
                }
            }
        }
    }
    pub fn new(widget: Rc<dyn Widget>) -> Self {
        let width = widget.width();
        let height = widget.height();
        let color = widget.color();
        let x = widget.x();
        let y = widget.y();
        let id = widget.id();
        let children =  {
            widget
                .as_ref()
                .children()
                .iter()
                .map(|child| WidgetNode::new(Rc::clone(child)))
                .collect()
        };
        WidgetNode {
            widget,
            children,
            render_object: Some(RenderObject {
                id: id as u32,
                constraints: Constraints{ 
                    min_width: width,
                    min_height: height,
                    max_width: Some(1000),
                    max_height: Some(1000),
                    width: Some(width),
                    height: Some(height),
                },
                x: x,
                y: y,
                color,
                handle: None,
                parent: None,
                is_visible: true,
                is_enabled: true,
                input_handler: None,
            }),
        }
    }

}
