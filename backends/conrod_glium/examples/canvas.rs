//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use glium::Surface;

mod support;

struct GameBoard {
    cells: Vec<bool>,
    cols: i32,
    rows: i32,
}

impl GameBoard {
    fn cell(&self, r: i32, c: i32) -> bool {
        return self.cells[idx(r,c)];
    }
    fn cell_click(&mut self, r:i32, c:i32) {
        self.cells[idx(r,c)] = !self.cells[idx(r,c)];
    }
    fn step(&mut self) { //cells: &mut Vec<bool>, rows: i32, cols: i32) {
        let cell_count = &mut vec![0;1600];
        let new_cells = &mut vec![false;1600];
        
        for i in 1..self.rows-1 {
            for j in 1..self.cols-1 {
                // check i, j vs j, i
                // if the cell is alive, increment the count of the surrounding cells
                if self.cells[idx(i, j)] == true {
                    for p in i-1..i+2 {
                        for q in j-1..j+2  {
                            cell_count[idx(p,q)] += 1;
                        }
                    }
                }
            }
        }

        for i in 1..self.cols-1 {
            for j in 1..self.rows-1 {
                let idx = idx(i,j);
                let alv = self.cells[idx];
                let cnt = cell_count[idx];

                if ( cnt == 3 ) || (alv && (cnt == 4)) {
                    new_cells[idx] = true;
                }
            }
        }
        self.cells = new_cells.to_vec();
    }
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop { last_update: std::time::Instant::now(),
                    ui_needs_update: true,
                  }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) ->
                Vec<glium::glutin::Event> {

        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the UI does not need updating, wait
        // for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| { events.push(event);
                                    glium::glutin::ControlFlow::Break });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }


    /// Notifies the event loop that the `Ui` requires another update whether
    /// or not there are any pending events.
    ///
    /// This is primarily used on the occasion that some part of the UI is
    /// still animating and requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

fn idx (r: i32, c: i32) -> usize {
    return (c + r * 40) as usize;
}

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // 40 * 40 = 1600
    // let _cells:&mut [bool;1600] = &mut [false;1600];
    let board = &mut GameBoard {
        cells: vec![false;1600],
        cols: 40,
        rows: 40
    };

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
    // let mut event_loop = support::EventLoop::new();
    let mut event_loop = EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = support::convert_event(
                event.clone(),
                &display
            ) {
                ui.handle_event(event);
            }

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            // if let Some(event) = support::convert_event(event.clone(), &display) {
            //     ui.handle_event(event);
            //     event_loop.needs_update();
            // }

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
        set_widgets(ui.set_widgets(), ids, board);

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

// Draw the Ui.
fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids, board: &mut GameBoard) {
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

    let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
    for _click in button.clone().top_left_of(ids.left_column).set(ids.bing, ui) {
        board.step();
    }
    for _click in button.top_left_of(ids.left_column).set(ids.bong, ui) {
        println!("Bong!");
    }

    // A scrollbar for the `FOOTER` canvas.
    widget::Scrollbar::y_axis(ids.footer).auto_hide(true).set(ids.footer_scrollbar, ui);

    // Now we'll make a couple floating `Canvas`ses.
    let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
    //floating.middle_of(ids.left_column).title_bar("Blue").color(color::BLUE).set(ids.floating_a, ui);
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

    // widget::Text::new("Top Left")
    //     .color(color::LIGHT_ORANGE.complement())
    //     .top_left_of(ids.left_column)
    //     .set(ids.top_left, ui);

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
        // let n = c + r * c;
        // let luminance = n as f32 / (COLS * ROWS) as f32;

        

        let mut button = widget::Button::new().color(color::WHITE);
        if board.cell(r as i32,c as i32) {
            button = widget::Button::new().color(color::BLACK);
        
        } 
        for _click in elem.set(button, ui) {
            board.cell_click(r as i32,c as i32);
            // cells[n] = !cells[n];
            // println!("Hey! {:?}", (r, c));
        }
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
