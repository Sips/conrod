//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use glium::Surface;

mod support;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // 40 * 40 = 1600
    // let _cells:&mut [bool;1600] = &mut [false;1600];
    let mut cells = vec![false;1600];

    for i in 0..cells.len() {
        if i % 27 == 0 {
            cells[i] = true;
        }
    }

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Canvas")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids, &mut cells);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn stepSim(cells: &mut Vec<bool>, rows: i32, cols: i32) {
    let mut cellCount = 
    for i in 1..cols-1 {
        for j in 1..rows-1 {

        }
    }
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids, cells: &mut Vec<bool>) {
    use conrod_core::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().length(100.0).color(color::WHITE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().length(100.0).flow_right(&[
            (ids.left_column, widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
            (ids.middle_column, widget::Canvas::new().color(color::ORANGE)),
            (ids.right_column, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
        ])),
        (ids.footer, widget::Canvas::new().color(color::BLUE).scroll_kids_vertically()),
    ]).set(ids.master, ui);

    // A scrollbar for the `FOOTER` canvas.
    widget::Scrollbar::y_axis(ids.footer).auto_hide(true).set(ids.footer_scrollbar, ui);

    // Now we'll make a couple floating `Canvas`ses.
    let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
    floating.middle_of(ids.left_column).title_bar("Blue").color(color::BLUE).set(ids.floating_a, ui);
    floating.middle_of(ids.right_column).title_bar("Orange").color(color::LIGHT_ORANGE).set(ids.floating_b, ui);

    // Here we make some canvas `Tabs` in the middle column.
    widget::Tabs::new(&[(ids.tab_step, "Step"), (ids.tab_bar, "BAR"), (ids.tab_baz, "BAZ"), (ids.tab_bai, "BAI")])
        .wh_of(ids.middle_column)
        .color(color::BLUE)
        .label_color(color::WHITE)
        .middle_of(ids.middle_column)
        .set(ids.tabs, ui);

    widget::Text::new("Game of Life")
        .color(color::LIGHT_GREEN)
        .font_size(48)
        .middle_of(ids.header)
        .set(ids.title, ui);
    widget::Text::new("by sips")
        .color(color::LIGHT_GREEN)
        .mid_bottom_of(ids.header)
        .set(ids.subtitle, ui);

    widget::Text::new("Top Left")
        .color(color::LIGHT_ORANGE.complement())
        .top_left_of(ids.left_column)
        .set(ids.top_left, ui);

    widget::Text::new("Bottom Right")
        .color(color::DARK_ORANGE.complement())
        .bottom_right_of(ids.right_column)
        .set(ids.bottom_right, ui);


    
    fn text(text: widget::Text) -> widget::Text { text.color(color::WHITE).font_size(36) }
    text(widget::Text::new("Step")).middle_of(ids.tab_step).set(ids.step_label, ui);
    text(widget::Text::new("Bar!")).middle_of(ids.tab_bar).set(ids.bar_label, ui);
    text(widget::Text::new("BAZ!")).middle_of(ids.tab_baz).set(ids.baz_label, ui);
    text(widget::Text::new("BAI!")).middle_of(ids.tab_bai).set(ids.bai_label, ui);

    let footer_wh = ui.wh_of(ids.footer).unwrap();
    let mut elements = widget::Matrix::new(COLS, ROWS)
        .w_h(footer_wh[0], footer_wh[1] * 2.0)
        .mid_top_of(ids.footer)
        .set(ids.button_matrix, ui);
    
    

    while let Some(elem) = elements.next(ui) {
        let (r, c) = (elem.row, elem.col);
         let n = c + r * c;
        // let luminance = n as f32 / (COLS * ROWS) as f32;

        

        let mut button = widget::Button::new().color(color::WHITE);
        if cells[n] {
            button = widget::Button::new().color(color::BLACK);
        
        } 
        for _click in elem.set(button, ui) {
            cells[n] = !cells[n];
            // println!("Hey! {:?}", (r, c));
        }
    }

    let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
    for _click in button.clone().middle_of(ids.floating_a).set(ids.bing, ui) {
        stepSim(cells, r, c);
    }
    for _click in button.middle_of(ids.floating_b).set(ids.bong, ui) {
        println!("Bong!");
    }
}


// Button matrix dimensions.
const ROWS: usize = 40;
const COLS: usize = 40;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        body,
        left_column,
        middle_column,
        right_column,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_step,
        tab_bar,
        tab_baz,
        tab_bai,

        title,
        subtitle,
        top_left,
        bottom_right,
        step_label,
        bar_label,
        baz_label,
        bai_label,
        button_matrix,
        bing,
        bong,
    }
}
