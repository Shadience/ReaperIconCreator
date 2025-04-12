/*
TO DO:
- Make web version 0%/100%
- Make test button like in REAPER 5%/100%
- Auto detect REAPER folder 0%/100%
- Make code cleaner -1%/100%
*/
#![windows_subsystem = "windows"]
use std::{env::temp_dir, path::PathBuf};

use rfd::{FileDialog, MessageDialog};

use image::{imageops::{resize, FilterType::{self}}, DynamicImage, GenericImage, RgbaImage, open};

use eframe::{egui::{self, vec2, ThemePreference, Vec2}, NativeOptions};
use egui_extras::install_image_loaders;

fn main() -> eframe::Result
{
    let mut options = NativeOptions::default();
    options.viewport.inner_size.replace(vec2(728f32,339f32));
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
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) // Saving empty image to temp
    {
        let path = temp_dir().to_str().unwrap().to_string() + "result.png";
        let path1 = temp_dir().to_str().unwrap().to_string() + "result150.png";
        let path2 = temp_dir().to_str().unwrap().to_string() + "result200.png";
        self.image_to_icon(&self.image, 30).save_with_format(&path, image::ImageFormat::Png).unwrap();
        self.image_to_icon(&self.image, 45).save_with_format(&path1, image::ImageFormat::Png).unwrap();
        self.image_to_icon(&self.image, 60).save_with_format(&path2, image::ImageFormat::Png).unwrap();
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        let path = temp_dir().to_str().unwrap().to_string() + "result.png";
        let path1 = temp_dir().to_str().unwrap().to_string() + "result150.png";
        let path2 = temp_dir().to_str().unwrap().to_string() + "result200.png";
        egui::TopBottomPanel::top("Adjustments").show(ctx, |ui| 
        {
            ui.horizontal(|ui|
            {
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
                ui.vertical(|ui|
                {
                    let f = self.state.filter_type;
                    egui::ComboBox::from_label("Filter Type").selected_text(format!("{f:?}")).show_ui(ui, |ui| 
                    {
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Nearest, "Nearest Neighbour");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Triangle, "Linear: Triangle");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::CatmullRom, "Cubic: CatmullRom");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Gaussian, "Gaussian");
                        ui.selectable_value(&mut self.state.filter_type, FilterType::Lanczos3, "Lanczos with window 3");
                    });
                    //ui.add_space(20f32);
                    egui::TextEdit::singleline(&mut self.result_name).hint_text("Result file name...").show(ui);
                });
            });
            
            ui.horizontal(|ui| 
            {
                if ui.button("Render").clicked()
                {
                    // println!("{}", temp_dir().join("result.png").to_str().unwrap());
                    // println!("{}", "file://".to_string() + &temp_dir().join("result.png").to_str().unwrap().replace(r"\", "/"));
                    self.image_to_icon(&self.image, 30).save_with_format(&path, image::ImageFormat::Png).unwrap();
                    self.image_to_icon(&self.image, 45).save_with_format(&path1, image::ImageFormat::Png).unwrap();
                    self.image_to_icon(&self.image, 60).save_with_format(&path2, image::ImageFormat::Png).unwrap();
                    // ctx.forget_image(&path);
                    ctx.forget_all_images();
                    // ctx.request_repaint();
                    // println!("{}", path);
                }
                if ui.button("Restore default settings").clicked()
                {
                    let mut s = State::default();
                    s.reaper_path = self.state.reaper_path.clone(); // Reaper path don't need to be restored
                    self.state = s;
                }
            });
            ui.add_space(5f32);
        });
        egui::CentralPanel::default().show(ctx, |ui|
        {
            ui.horizontal(|ui| {
                ui.vertical(|ui|
                {
                    ui.label("100 🔍 (Default)");
                    ui.allocate_ui(Vec2{x:90f32,y:30f32}, |ui| {ui.image(format!("file://{path}"))});
                    ui.label("150 🔍");
                    ui.allocate_ui(Vec2{x:135f32,y:45f32}, |ui| {ui.image(format!("file://{path1}"))});
                    ui.label("200 🔍");
                    ui.allocate_ui(Vec2{x:180f32,y:60f32}, |ui| {ui.image(format!("file://{path2}"))});
                });
                // ui.add_space(5f32);
                // ui.vertical(|ui|
                // {
                //     ui.label("Test Button");
                //     ui.add_space(5f32);
                //     ui.allocate_ui(Vec2{x:30f32,y:30f32}, |ui| {ui.image(format!("file://{path}"))});
                //     ui.add_space(35f32);
                //     ui.;
                //     ui.add_space(55f32);
                //     ui.
                // });
                ui.add_space(5f32);
                ui.vertical(|ui|
                {
                    ui.label("Export");
                    ui.add_space(5f32);
                    ui.radio_value(&mut self.state.export_type, ExportZoom::FIRST, "");
                    ui.add_space(35f32);
                    ui.radio_value(&mut self.state.export_type, ExportZoom::SECOND, "");
                    ui.add_space(55f32);
                    ui.radio_value(&mut self.state.export_type, ExportZoom::THIRD, "");
                });
            });
        });
        egui::TopBottomPanel::bottom("File stuff").show(ctx, |ui|
        {
            ui.add_space(5f32);
            ui.horizontal(|ui| 
            {
                if ui.button("Import").clicked()
                {
                    let path = self.import_file_dialog.clone().pick_file();
                    if path != None
                    {
                        self.image = open(path.unwrap()).unwrap();
                    }
                }
                if ui.button("Export").clicked()
                {
                    let path = self.export_file_dialog.clone().set_file_name(&self.result_name).save_file();
                    if path != None
                    {
                        match &self.state.export_type
                        {
                            ExportZoom::FIRST =>
                            {
                                self.image_to_icon(&self.image, 30).save_with_format(path.unwrap(), image::ImageFormat::Png).unwrap();
                            }
                            ExportZoom::SECOND =>
                            {
                                self.image_to_icon(&self.image, 45).save_with_format(path.unwrap(), image::ImageFormat::Png).unwrap();
                            }
                            ExportZoom::THIRD =>
                            {
                                self.image_to_icon(&self.image, 60).save_with_format(path.unwrap(), image::ImageFormat::Png).unwrap();
                            }
                        }
                    }
                }
                if ui.button("Select REAPER folder").clicked()
                {
                    let path = FileDialog::new().pick_folder();
                    if path != None
                    {
                        self.state.reaper_path = path.unwrap().join(r"Data\toolbar_icons");
                    }
                    // println!("{}", self.reaper_path.to_str().unwrap());
                    // println!("{}", self.reaper_path.join(r"150\").to_str().unwrap());
                    // println!("{}", self.reaper_path.join(r"200\").to_str().unwrap());
                }
                if ui.button("Export directly to REAPER").clicked()
                {
                    if self.state.reaper_path != PathBuf::default()
                    {
                        let path1 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.state.reaper_path).save_file();
                        if path1 != None
                        {
                            self.image_to_icon(&self.image, 30).save_with_format(path1.unwrap(), image::ImageFormat::Png).unwrap();
                        }
                        let path2 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.state.reaper_path.join(r"150\")).save_file();
                        if path2 != None
                        {
                            self.image_to_icon(&self.image, 45).save_with_format(path2.unwrap(), image::ImageFormat::Png).unwrap();
                        }
                        let path3 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.state.reaper_path.join(r"200\")).save_file();
                        if path3 != None
                        {
                            self.image_to_icon(&self.image, 30).save_with_format(path3.unwrap(), image::ImageFormat::Png).unwrap();
                        }
                    } else
                    {
                        MessageDialog::new().set_title("Error!").set_description("Set your REAPER folder first").show();
                    }
                }
            });
            ui.add_space(5f32);
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
            image: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            // image_on_hover: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            // image_clicked: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            result_name: "result".to_string(),
            import_file_dialog: FileDialog::new().add_filter("Images", &["png", "jpeg", "jpg", "gif", "bmp", "ico", "farbfeld", "hdr", "exr", "pnm", "qoi", "tga", "tiff"]),
            export_file_dialog: FileDialog::new().add_filter("png", &["png"]),
        };
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                s.state = state;
            }
        }
        s
    }
    
    // Sizes:
    // Default: 30
    // 150: 45
    // 200: 60
    fn image_to_icon(&self, img: &DynamicImage, size: u32) -> RgbaImage
    {
        let img = DynamicImage::ImageRgba8(resize(img, size, size, self.state.filter_type)); // open change to input function
        let mut res = RgbaImage::new(size*3, size);

        res.copy_from(&img, 0, 0).unwrap(); // normal
        res.copy_from(&img.huerotate(self.state.on_hover_hue).adjust_contrast(self.state.on_hover_contrast).brighten(self.state.on_hover_brightness), size, 0).unwrap(); // on mouse hover
        res.copy_from(&img.huerotate(self.state.clicked_hue).adjust_contrast(self.state.clicked_contrast).brighten(self.state.clicked_brightness), size*2, 0).unwrap(); // clicked

        res
    }
}

struct App
{
    pub state: State,
    image: DynamicImage,
    // image_on_hover: DynamicImage,
    // image_clicked: DynamicImage,
    result_name: String,
    import_file_dialog: FileDialog,
    export_file_dialog: FileDialog,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct State 
{
    export_type: ExportZoom,
    reaper_path: PathBuf,
    on_hover_hue: i32,
    on_hover_contrast: f32,
    on_hover_brightness: i32,
    clicked_hue: i32,
    clicked_contrast: f32,
    clicked_brightness: i32,
    filter_type: FilterType,
}

impl Default for State
{
    fn default() -> Self {
        Self
        {
            export_type: ExportZoom::FIRST,
            reaper_path: PathBuf::default(),
            on_hover_hue: 0,
            on_hover_contrast: 0f32,
            on_hover_brightness: -30,
            clicked_hue: 0,
            clicked_contrast: 15f32,
            clicked_brightness: 0,
            filter_type: FilterType::Lanczos3,
        }
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum ExportZoom 
{
    FIRST,
    SECOND,
    THIRD
}