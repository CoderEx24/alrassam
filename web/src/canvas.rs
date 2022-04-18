use yew::prelude::*;
use program_core::{ Vector2, Canvas, Color, Props };
use crate::properties::PropertiesPanel;

pub enum CanvasMsg {
    DrawLine,
    DrawCircle,
    DrawRect,
    Click(Vector2),
}

pub struct CanvasComponent {
    canvas: Canvas,
    canvas_element: NodeRef,
    current_drawable: Option<CanvasMsg>,
    points: Vec<Vector2>,
    selected_drawable: Option<Props>,
}

impl Component for CanvasComponent {
    type Message = CanvasMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut canvas = CanvasComponent {
            canvas: Canvas::new(500, 500),
            canvas_element: NodeRef::default(),
            current_drawable: None,
            points: vec![],
            selected_drawable: None,
        };
        
        canvas
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let line_onclick = ctx.link().callback(|_| CanvasMsg::DrawLine);
        let canvas_onclick = ctx.link().callback(|evt: MouseEvent| {
            CanvasMsg::Click(Vector2::new(evt.offset_x() as f64, evt.offset_y() as f64))
        });  
        
        html! {
            <>
                <div id={"controls-panel"}>
                    <button class={"control-item"} onclick={line_onclick}>{"line"}</button> <br />
                </div>

                <div id={"canvas"}  onclick={canvas_onclick} ref={self.canvas_element.clone()}>
                </div>
                {
                    if let Some(props) = &self.selected_drawable {
                        html! { <PropertiesPanel props={props.clone()}/> }
                    } else {
                        html! {}
                    }
                }
            </>
        }
    }

    fn rendered(&mut self, _: &Context<Self>, _: bool) {
        let div = gloo_utils::document().get_element_by_id("canvas").unwrap();

        div.set_inner_html(self.canvas.to_svg().as_str());
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CanvasMsg::DrawLine |
            CanvasMsg::DrawRect |
            CanvasMsg::DrawCircle => {
                self.current_drawable = Some(msg);
                println!("selected a drawable");
            },
            CanvasMsg::Click(point) => {
                log::info!("click recevied");
                if self.canvas.select_drawable_at(point) {
                    let props = self.canvas.get_selected_drawable_properties().expect("properties claiming failed!");
                    self.selected_drawable = Some(props.clone());

                    log::info!("drawable selected!");

                    return true;
                }
                
                if let Some(_) = self.current_drawable {
                    log::info!("appended point to points stack");
                    self.points.push(point);
                }
            }
        }
        
        match self.current_drawable {
            Some(CanvasMsg::DrawLine) => {
                if self.points.len() < 2 {
                    return false;
                }

                let (start, end) = (self.points[0], self.points[1]);
                self.canvas.add_line(start, end, None, None, None);
                self.points.clear();
                self.current_drawable = None;
                return true;
            }
            _ => { return false; },
        }

    }

}

