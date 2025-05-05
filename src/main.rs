/*
TO DO:
- Make web version 0%/100%
- Make test button like in REAPER 5%/100%
- Auto detect REAPER folder 0%/100%
- Mass drag&drop import 0%/100%
- Change settings for existing reaper icons 0%/100%
- Change UI scale for existing reaper icons 0%/100%
- Make code cleaner 1%/100%
*/

#![windows_subsystem = "windows"]

use std::{env::temp_dir, path::PathBuf};

use eframe::{egui::{self, ThemePreference, Vec2}, NativeOptions};
use egui_extras::install_image_loaders;
use image::{imageops::FilterType, imageops::resize, open, DynamicImage, GenericImage};
use rfd::FileDialog;

fn main() -> eframe::Result
{
    let mut options = NativeOptions::default();
    options.viewport.inner_size.replace(egui::vec2(803f32,409f32));
    options.viewport.resizable.replace(false);
    eframe::run_native(
        "Reaper Icon Creator",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

impl eframe::App for App
{
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::TopBottomPanel::top("top").show(ctx, |ui|
        {
            ui.horizontal(|ui|
            {
                ui.vertical(|ui|
                {
                    ui.add(egui::Slider::new(&mut self.state.main_hue, 0..=360).text("Icon Hue"));
                    ui.add(egui::Slider::new(&mut self.state.main_contrast, -100f32..=100f32).text("Icon Contrast"));
                    ui.add(egui::Slider::new(&mut self.state.main_brightness, -100..=100).text("Icon Brightness"));
                });
                ui.vertical(|ui|
                {
                    ui.add(egui::Slider::new(&mut self.state.on_hover_hue, 0..=360).text("On Hover Hue"));
                    ui.add(egui::Slider::new(&mut self.state.on_hover_contrast, -100f32..=100f32).text("On Hover Contrast"));
                    ui.add(egui::Slider::new(&mut self.state.on_hover_brightness, -100..=100).text("On Hover Brightness"));
                });
                ui.vertical(|ui|
                {
                    ui.add(egui::Slider::new(&mut self.state.clicked_hue, 0..=360).text("Clicked Hue"));
                    ui.add(egui::Slider::new(&mut self.state.clicked_contrast, -100f32..=100f32).text("Clicked Contrast"));
                    ui.add(egui::Slider::new(&mut self.state.clicked_brightness, -100..=100).text("Clicked Brightness"));
                });
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| 
            {
                if ui.button("Render").clicked()
                {
                    self.render();
                    ctx.forget_all_images();
                }
                if ui.button("Restore default settings").clicked()
                {
                    let mut s = State::default();
                    s.reaper_path = self.state.reaper_path.clone(); // Reaper path don't need to be restored
                    self.state = s;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui|
                {
                    egui::TextEdit::singleline(&mut self.imported_iter.1).interactive(false).hint_text("Result file name...").show(ui);
                    
                    let f = self.state.filter_type;
                    egui::ComboBox::from_label("Filter Type").selected_text(format!("{f:?}")).show_ui(ui, |ui| 
                    {
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Nearest, "Nearest Neighbour");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Triangle, "Linear: Triangle");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::CatmullRom, "Cubic: CatmullRom");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Gaussian, "Gaussian");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Lanczos3, "Lanczos with window 3");
                    }); 
                });
            });
            ui.add_space(5f32);
        });
        egui::CentralPanel::default().show(ctx, |ui|
        {
            let i = &self.imported_iter.1;
            egui::ComboBox::from_label("Icons").selected_text(i).show_ui(ui, |ui| 
            {
                for icon in &self.imported
                {
                    ui.selectable_value(&mut self.imported_iter, icon.clone(), &icon.1);
                }
            });
            ui.add_space(5f32);
            if !self.imported.is_empty()
            {
                ui.horizontal(|ui|
                {
                    ui.vertical(|ui|
                    {
                        ui.label("100 🔍 (Default)");
                        ui.allocate_ui(Vec2{x:90f32,y:30f32}, |ui| {ui.image(String::from("file://") + &self.temp_dir_string + &self.imported_iter.1)});
                        ui.label("150 🔍");
                        ui.allocate_ui(Vec2{x:135f32,y:45f32}, |ui| {ui.image(String::from("file://") + &insert_to_path(self.temp_dir_string.clone() + &self.imported_iter.1, "150"))});
                        ui.label("200 🔍");
                        ui.allocate_ui(Vec2{x:180f32,y:60f32}, |ui| {ui.image(String::from("file://") + &insert_to_path(self.temp_dir_string.clone() + &self.imported_iter.1, "200"))});
                    });
                    ui.add_space(5f32);
                    ui.vertical(|ui|
                    {
                        ui.label("Export");
                        ui.add_space(5f32);
                        ui.checkbox(&mut self.state.export_type[0], "");
                        ui.add_space(35f32);
                        ui.checkbox(&mut self.state.export_type[1], "");
                        ui.add_space(55f32);
                        ui.checkbox(&mut self.state.export_type[2], "");
                    });
                });
            }
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui|
        {
            ui.add_space(5f32);
            ui.horizontal(|ui|
            {
                if ui.button("Import").clicked()
                {
                    if let Some(paths) = self.import_file_dialog.clone().pick_files()
                    {
                        for path in paths
                        {
                            // self.imported_images.push(open(path).unwrap());
                            println!("{:?}", path);
                            self.imported.push((open(&path).unwrap(), path.file_name().unwrap().to_str().unwrap().to_string()));
                        }
                        self.imported_iter = self.imported[0].clone();
                        self.render();
                    }
                }
                if ui.button("Export").clicked()
                {
                    // let path = self.export_file_dialog.clone().pick_folder().expect("Failed to pick export folder");
                    // path.metadata().unwrap().permissions().set_readonly(false);
                    for i in 0..3
                    {
                        if self.state.export_type[i]
                        {
                            match i as i8 {
                                0 => 
                                {
                                    if let Some(path) = self.export_file_dialog.clone().set_file_name(&self.imported_iter.1).save_file()
                                    {
                                        self.image_to_icon(&self.imported_iter, 30).0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                                    }
                                },
                                1 => 
                                {
                                    if let Some(path) = self.export_file_dialog.clone().set_file_name(insert_to_path(self.imported_iter.1.clone(), "150")).save_file()
                                    {
                                        self.image_to_icon(&self.imported_iter, 45).0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                                    }
                                },
                                2 => 
                                {
                                    if let Some(path) = self.export_file_dialog.clone().set_file_name(insert_to_path(self.imported_iter.1.clone(), "200")).save_file()
                                    {
                                        self.image_to_icon(&self.imported_iter, 60).0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                                    }
                                },
                                _=>()
                            }
                        }
                    }
                }
                if ui.button("Select REAPER folder").clicked()
                {
                    if let Some(path) = FileDialog::new().pick_folder()
                    {
                        self.state.reaper_path = path.join(r"Data\toolbar_icons");
                    }
                }
                if ui.button("Export to REAPER").clicked()
                {
                    if self.state.reaper_path != PathBuf::default()
                    {
                        for icon in self.images_to_icon(&self.imported, 30)
                        {
                            if let Some(path) = self.export_file_dialog.clone().set_file_name(icon.1).set_directory(&self.state.reaper_path).save_file()
                            {
                                icon.0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                            } else
                            {
                                break;
                            }
                        }
                        for icon in self.images_to_icon(&self.imported, 45)
                        {
                            if let Some(path) = self.export_file_dialog.clone().set_file_name(icon.1).set_directory(&self.state.reaper_path.join(r"150\")).save_file()
                            {
                                icon.0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                            } else
                            {
                                break;
                            }
                        }
                        for icon in self.images_to_icon(&self.imported, 60)
                        {
                            if let Some(path) = self.export_file_dialog.clone().set_file_name(icon.1).set_directory(&self.state.reaper_path.join(r"200\")).save_file()
                            {
                                icon.0.save_with_format(path, image::ImageFormat::Png).expect("Failed to export");
                            } else
                            {
                                break;
                            }
                        }
                    } else
                    {
                        rfd::MessageDialog::new().set_title("Error!").set_description("Set your REAPER folder first").show();
                    }
                }
            });
            ui.add_space(3f32);
        });
    }
}

impl App 
{
    fn new(cc: &eframe::CreationContext<'_>) -> Self 
    {
        cc.egui_ctx.set_theme(ThemePreference::System);
        install_image_loaders(&cc.egui_ctx);
        let mut s = Self 
        {
            state: State::default(),
            imported: vec![],
            imported_iter: (DynamicImage::new(30, 30, image::ColorType::Rgba8), String::from("")),
            
            import_file_dialog: FileDialog::new().add_filter("Images", &["png", "jpeg", "jpg", "gif", "bmp", "ico", "farbfeld", "hdr", "exr", "pnm", "qoi", "tga", "tiff"]),
            export_file_dialog: FileDialog::new().add_filter("png", &["png"]),
            
            temp_dir_string: temp_dir().to_str().unwrap().to_string(),
        };
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                s.state = state;
            }
        }
        s.start();
        s
    }
    
    #[inline]
    fn start(&self)
    {
        self.render();
    }
    
    fn render(&self)
    {
        for icon in self.images_to_icon(&self.imported, 30)
        {
            icon.0.save_with_format(temp_dir().join(icon.1), image::ImageFormat::Png).expect("Failed to export");
        }
        for icon in self.images_to_icon(&self.imported, 45)
        {
            icon.0.save_with_format(temp_dir().join(insert_to_path(icon.1, 150)), image::ImageFormat::Png).expect("Failed to export");
        }
        for icon in self.images_to_icon(&self.imported, 60)
        {
            icon.0.save_with_format(temp_dir().join(insert_to_path(icon.1, 200)), image::ImageFormat::Png).expect("Failed to export");
        }
    }
    
    fn images_to_icon(&self, images: &Vec<(DynamicImage, String)>, size: u32) -> Vec<(DynamicImage, String)>
    {
        let mut res: Vec<(DynamicImage, String)> = vec![];
        for img in images
        {
            let mut temp = DynamicImage::new(size*3, size, image::ColorType::Rgba8);
            let name = &img.1;
            let img = DynamicImage::ImageRgba8(resize(&img.0, size, size, self.state.filter_type));
            temp.copy_from(&img, 0, 0).expect("Can't transform 1 step"); // normal
            temp.copy_from(&img.huerotate(self.state.on_hover_hue) // on mouse hover
                            .adjust_contrast(self.state.on_hover_contrast)
                            .brighten(self.state.on_hover_brightness), size, 0).expect("Can't transform 2 step");
            temp.copy_from(&img.huerotate(self.state.clicked_hue) // clicked
                            .adjust_contrast(self.state.clicked_contrast)
                            .brighten(self.state.clicked_brightness), size*2, 0).expect("Can't transform 3 step");
            res.push((temp, name.clone()));
        }
        res
    }
    fn image_to_icon(&self, img: &(DynamicImage, String), size: u32) -> (DynamicImage, String)
    {
        let mut res: (DynamicImage, String) = (DynamicImage::new(size*3, size, image::ColorType::Rgba8), img.1.clone());
        let img = DynamicImage::ImageRgba8(resize(&img.0, size, size, self.state.filter_type));
        res.0.copy_from(&img, 0, 0).expect("Can't transform 1 step"); // normal
        res.0.copy_from(&img.huerotate(self.state.on_hover_hue) // on mouse hover
            .adjust_contrast(self.state.on_hover_contrast)
            .brighten(self.state.on_hover_brightness), size, 0).expect("Can't transform 2 step");
        res.0.copy_from(&img.huerotate(self.state.clicked_hue) // clicked
            .adjust_contrast(self.state.clicked_contrast)
            .brighten(self.state.clicked_brightness), size*2, 0).expect("Can't transform 3 step");
        res
    }
}

#[inline]
pub fn get_first_side<T: Clone, U>(arr: &Vec<(T, U)>) -> Vec<T>
{
    let mut temp: Vec<T> = vec![];
    for i in arr
    {
        temp.push(i.0.clone());
    }
    temp
}

#[inline]
pub fn get_sec_side<T, U: Clone>(arr: &Vec<(T, U)>) -> Vec<U>
{
    let mut temp: Vec<U> = vec![];
    for i in arr
    {
        temp.push(i.1.clone());
    }
    temp
}

#[inline]
pub fn insert_to_path<T: ToString>(mut path: String, some: T) -> String
{
    path.insert_str(path.find('.').unwrap(), &some.to_string());
    path
}

struct App
{
    state: State,
    imported: Vec<(DynamicImage, String)>,
    imported_iter: (DynamicImage, String),
    
    import_file_dialog: FileDialog,
    export_file_dialog: FileDialog,
    
    temp_dir_string: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct State 
{
    main_hue: i32,
    main_contrast: f32,
    main_brightness: i32,
    
    on_hover_hue: i32,
    on_hover_contrast: f32,
    on_hover_brightness: i32,
    
    clicked_hue: i32,
    clicked_contrast: f32,
    clicked_brightness: i32,
    
    filter_type: FilterType,
    
    export_type: [bool; 3],
    reaper_path: PathBuf,
}

impl Default for State
{
    fn default() -> Self {
        Self
        {
            main_hue: 0,
            main_contrast: 0f32,
            main_brightness: 0,
            
            on_hover_hue: 0,
            on_hover_contrast: 0f32,
            on_hover_brightness: -30,
            
            clicked_hue: 0,
            clicked_contrast: 15f32,
            clicked_brightness: 0,
            
            filter_type: FilterType::Lanczos3,
            
            export_type: [true, false, false],
            reaper_path: PathBuf::new(),
        }
    }
}
