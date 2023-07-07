//use std::path::Path;

use egui::{self, Context};
use egui::{Color32, RichText};
//use egui_extras::image::RetainedImage;
use egui_macroquad;
//use image::open;
use macroquad::prelude::*;
//use macroquad::ui::StyleBuilder;

use crate::agent::Agent;
use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::sim::*;
use crate::{progress_bar::*, Signals};


pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
    temp_sim_name: String,
}

impl UISystem {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
            temp_sim_name: String::new(),
        }
    }

    pub fn ui_process(
        &mut self,
        sim_state: &SimState,
        agent: Option<&Agent>,
        signals: &mut Signals,
        camera2d: &Camera2D,
    ) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(
                egui_ctx,
                sim_state.fps,
                sim_state.dt,
                sim_state.sim_time,
                sim_state.agents_num,
                sim_state.physics_num,
                sim_state.asteroids_num,
            );
            self.build_debug_window(egui_ctx, camera2d);
            match agent {
                Some(agent) => self.build_inspect_window(egui_ctx, agent),
                None => {}
            }
            self.build_create_window(egui_ctx, signals);
            self.build_new_sim_window(egui_ctx, signals);
        });
    }

    fn build_top_menu(&mut self, egui_ctx: &Context, sim_name: &str) {
        egui::TopBottomPanel::top("top_panel").default_height(100.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            egui::menu::bar(ui, |ui| {
                ui.heading(RichText::new(sim_name).strong().color(Color32::GREEN));
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                egui::menu::menu_button(ui, RichText::new("SIMULATION").strong(), |ui| {
                    if ui
                        .button(
                            RichText::new("New Simulation")
                                .strong()
                                .color(Color32::BLUE),
                        )
                        .clicked()
                    {
                        self.state.new_sim = true;
                    }
                    if ui
                        .button(
                            RichText::new("Load Simulation")
                                .weak()
                                .color(Color32::from_gray(100)),
                        )
                        .clicked()
                    {}
                    if ui
                        .button(
                            RichText::new("Save Simulation")
                                .weak()
                                .color(Color32::from_gray(100)),
                        )
                        .clicked()
                    {}
                    if ui
                        .button(RichText::new("Quit").color(Color32::RED).strong())
                        .clicked()
                    {
                        self.state.quit = true;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                egui::menu::menu_button(ui, RichText::new("TOOLS").strong(), |ui| {
                    if ui
                        .button(
                            RichText::new("Monitor")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.performance = !self.state.performance;
                    }
                    if ui
                        .button(
                            RichText::new("Inspector")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.inspect = !self.state.inspect;
                    }
                    if ui
                        .button(
                            RichText::new("Debug Info")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.mouse = !self.state.mouse;
                    }
                    if ui
                        .button(
                            RichText::new("Create")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.create = !self.state.create;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                egui::menu::menu_button(ui, RichText::new("ABOUT").strong(), |ui| {
                    if ui
                        .button(
                            RichText::new("Credits")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.credits = !self.state.credits;
                    }
                    if ui
                        .button(
                            RichText::new("Documentation")
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .clicked()
                    {
                        self.state.docs = !self.state.docs;
                    }
                });
            });
        });
    }

    fn build_monit_window(
        &self,
        egui_ctx: &Context,
        fps: i32,
        delta: f32,
        time: f64,
        agents_num: i32,
        physics_num: i32,
        asteroids_num: usize,
    ) {
        if self.state.performance {
            egui::Window::new("MONITOR")
                .default_pos((5.0, 5.0))
                .default_width(125.0)
                .show(egui_ctx, |ui| {
                    ui.label(format!("DELTA: {}ms", (delta * 1000.0).round()));
                    ui.separator();
                    ui.label(format!("FPS: {}", fps));
                    ui.separator();
                    ui.label(format!("TIME: {}", time.round()));
                    ui.separator();
                    ui.label(format!("AGENTS: {}", agents_num));
                    ui.separator();
                    ui.label(format!("ASTEROIDS: {}", asteroids_num));
                    ui.separator();
                    ui.label(format!("PHYSICS OBJECTS: {}", physics_num));
                });
        }
    }

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            let tg_pos = agent.enemy_position;
            let tg_ang = agent.enemy_dir;
            let pos = agent.pos;
            egui::Window::new("INSPECT")
                .default_pos((175.0, 5.0))
                .default_width(200.0)
                .show(egui_ctx, |ui| {
                    ui.label(format!("ROTATION: {}", ((rot * 10.0).round()) / 10.0));
                    ui.label(format!("SIZE: {}", size));
                    ui.label(format!("POSITION: [X: {} | Y:{}]", pos.x.round(), pos.y.round()));
                    ui.separator();
                    ui.label(RichText::new("ENEMY").strong());
                    match (tg_pos, tg_ang) {
                        (Some(target), Some(ang)) => {
                            ui.label(format!("enemy pos: [x: {} | y:{}]", target.x.round(), target.y.round()));
                            ui.label(format!("angle to enemy: [{}]", ang.round()));
                        },
                        (None, None) => {
                            ui.label(format!("enemy pos: ---"));
                            ui.label(format!("angle to enemy: ---"));
                        },
                        (_, _) => {
                            ui.label(format!("enemy pos: ---"));
                            ui.label(format!("angle to enemy: ---"));
                        }
                    }
                    ui.separator();
                    ui.label(format!(
                        "ENERGY: {}/{}",
                        agent.eng.round(),
                        agent.max_eng.round()
                    ));
                    let eng_prog = agent.eng / agent.max_eng;
                    ui.add(
                        ProgressBar::new(eng_prog)
                            .desired_width(100.0)
                            .fill(Color32::BLUE)
                            .show_percentage(),
                    );
                });
        }
    }

    fn build_debug_window(&self, egui_ctx: &Context, camera2d: &Camera2D) {
        if self.state.mouse {
            let (mouse_x, mouse_y) = mouse_position();
            egui::Window::new("DEBUG INFO")
                .default_pos((375.0, 5.0))
                .default_width(175.0)
                .show(egui_ctx, |ui| {
                    ui.label(RichText::new("MOUSE").strong());
                    ui.label(format!("coords [x: {} | y: {}]", mouse_x.round(), mouse_y.round()));
                    ui.separator();
                    ui.label(RichText::new("CAMERA").strong());
                    ui.label(format!("target [x:{} | Y:{}]", camera2d.target.x.round(), camera2d.target.y.round()));
                    ui.label(format!("offset [x:{} | Y:{}]", camera2d.offset.x.round(), camera2d.offset.y.round()));
                    ui.label(format!("zoom [x:{} | Y:{}]", (camera2d.zoom.x*10000.).round()/10., (camera2d.zoom.y*10000.).round()/10.));
                    ui.label(format!("rotation: {}", camera2d.rotation.round()));
                });
        }
    }

    fn build_quit_window(&mut self, egui_ctx: &Context) {
        if self.state.quit {
            egui::Window::new("QUIT")
                .default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0))
                .default_width(125.0)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|head| {
                        head.heading("Are you sure?");
                    });
                    ui.horizontal(|mid| {
                        mid.columns(2, |columns| {
                            if columns[0]
                                .button(RichText::new("No").color(Color32::WHITE))
                                .clicked()
                            {
                                self.state.quit = false;
                            }
                            if columns[1]
                                .button(RichText::new("Yes").color(Color32::RED))
                                .clicked()
                            {
                                std::process::exit(0);
                            }
                        });
                    });
                });
        }
    }

    fn build_new_sim_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.new_sim {
            //let mut sim_name: String = String::new();
            egui::Window::new("NEW SIMULATION")
                .default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0))
                .default_width(125.0)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|head| {
                        head.heading("Start new simulation?");
                    });
                    ui.horizontal(|txt| {
                        let response =
                            txt.add(egui::widgets::TextEdit::singleline(&mut self.temp_sim_name));
                        if response.gained_focus() {
                            self.temp_sim_name = String::new();
                        }
                        if response.changed() {
                            //self.temp_sim_name = String::from(&sim_name);
                            //println!("{:?}", sim_name);
                            //println!("{:?}", self.temp_sim_name);
                        }
                        if response.lost_focus() && txt.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.state.new_sim = false;
                            signals.new_sim = true;
                            signals.new_sim_name = String::from(&self.temp_sim_name);
                            self.temp_sim_name = String::new();
                        }
                        //let response = txt.text_edit_singleline(&mut sim_name);
                    });
                    ui.horizontal(|mid| {
                        mid.columns(2, |columns| {
                            if columns[0]
                                .button(RichText::new("No").color(Color32::WHITE))
                                .clicked()
                            {
                                self.state.new_sim = false;
                                self.temp_sim_name = String::new();
                            }
                            if columns[1]
                                .button(RichText::new("Yes").color(Color32::RED))
                                .clicked()
                            {
                                self.state.new_sim = false;
                                signals.new_sim = true;
                                signals.new_sim_name = String::from(&self.temp_sim_name);
                                self.temp_sim_name = String::new();
                            }
                        });
                    });
                });
        }
    }

    fn build_create_window(&self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.create {
            egui::Window::new("CREATE")
                .default_pos((600.0, 5.0))
                .default_width(125.0)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|head| {
                        head.heading("Create new creature");
                    });
                    ui.horizontal(|mid| {
                        mid.style_mut().visuals.extreme_bg_color = Color32::BLUE;
                        if mid
                            .button(RichText::new("CREATE").strong().color(Color32::WHITE))
                            .clicked()
                        {
                            //self.state.create = false;
                            signals.spawn_agent = true;
                        }
                    });
                });
        }
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
    }
}

//?         [[[UISTATE]]]
pub struct UIState {
    pub new_sim_name: String,
    pub performance: bool,
    pub inspect: bool,
    pub mouse: bool,
    pub create: bool,
    pub quit: bool,
    pub agents_num: i32,
    pub new_sim: bool,
    pub credits: bool,
    pub docs: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            new_sim_name: String::new(),
            performance: false,
            inspect: false,
            mouse: false,
            create: false,
            quit: false,
            agents_num: 0,
            new_sim: false,
            credits: false,
            docs: false,
        }
    }
}
