/*
TO DO:
- Make web version 0%/100%
- Make test button like in REAPER 5%/100%
- Auto detect REAPER folder 0%/100%
*/

use std::{env::temp_dir, path::PathBuf};

use eframe::{egui::{self, vec2, ThemePreference, Vec2}, NativeOptions};
use egui_extras::install_image_loaders;
use image::{imageops::{resize, FilterType::{self}}, DynamicImage, GenericImage, RgbaImage, open};

use rfd::{FileDialog, MessageDialog};

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

#[derive(PartialEq)]
enum ExportZoom 
{
    FIRST,
    SECOND,
    THIRD
}

impl eframe::App for App
{
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        let path = temp_dir().to_str().unwrap().to_string() + "result.png";
        let path1 = temp_dir().to_str().unwrap().to_string() + "result150.png";
        let path2 = temp_dir().to_str().unwrap().to_string() + "result200.png";
        self.image_to_icon(&self.image, 30).save_with_format(&path, image::ImageFormat::Png).unwrap();
        self.image_to_icon(&self.image, 45).save_with_format(&path1, image::ImageFormat::Png).unwrap();
        self.image_to_icon(&self.image, 60).save_with_format(&path2, image::ImageFormat::Png).unwrap();
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::TopBottomPanel::top("Adjustments").show(ctx, |ui| 
        {
            ui.horizontal(|ui|
            {
                ui.vertical(|ui|
                {
                    ui.add(egui::Slider::new(&mut self.on_hover_brightness, -100..=100).text("On Hover Brightness"));
                    ui.add(egui::Slider::new(&mut self.on_hover_contrast, -100f32..=100f32).text("On Hover Contrast"));
                    ui.add(egui::Slider::new(&mut self.on_hover_hue, 0..=360).text("On Hover Hue"));
                });
                ui.vertical(|ui|
                {
                    ui.add(egui::Slider::new(&mut self.clicked_brightness, -100..=100).text("Clicked Brightness"));
                    ui.add(egui::Slider::new(&mut self.clicked_contrast, -100f32..=100f32).text("Clicked Contrast"));
                    ui.add(egui::Slider::new(&mut self.clicked_hue, 0..=360).text("Clicked Hue"));
                });
                ui.vertical(|ui|
                {
                    let f = self.filter_type;
                    egui::ComboBox::from_label("Filter Type").selected_text(format!("{f:?}")).show_ui(ui, |ui| 
                    {
                        ui.selectable_value(&mut self.filter_type, FilterType::Nearest, "Nearest Neighbour");
                        ui.selectable_value(&mut self.filter_type, FilterType::Triangle, "Linear: Triangle");
                        ui.selectable_value(&mut self.filter_type, FilterType::CatmullRom, "Cubic: CatmullRom");
                        ui.selectable_value(&mut self.filter_type, FilterType::Gaussian, "Gaussian");
                        ui.selectable_value(&mut self.filter_type, FilterType::Lanczos3, "Lanczos with window 3");
                    });
                    //ui.add_space(20f32);
                    egui::TextEdit::singleline(&mut self.result_name).hint_text("Result file name...").show(ui);
                });
            });
            
            if ui.button("Render").clicked()
            {
                // println!("{}", temp_dir().join("result.png").to_str().unwrap());
                // println!("{}", "file://".to_string() + &temp_dir().join("result.png").to_str().unwrap().replace(r"\", "/"));
                let path = temp_dir().to_str().unwrap().to_string() + "result.png";
                let path1 = temp_dir().to_str().unwrap().to_string() + "result150.png";
                let path2 = temp_dir().to_str().unwrap().to_string() + "result200.png";
                self.image_to_icon(&self.image, 30).save_with_format(&path, image::ImageFormat::Png).unwrap();
                self.image_to_icon(&self.image, 45).save_with_format(&path1, image::ImageFormat::Png).unwrap();
                self.image_to_icon(&self.image, 60).save_with_format(&path2, image::ImageFormat::Png).unwrap();
                // ctx.forget_image(&path);
                ctx.forget_all_images();
                // ctx.request_repaint();
                // println!("{}", path);
            }
            ui.allocate_space(Vec2{x:0f32,y:5f32});
        });
        egui::CentralPanel::default().show(ctx, |ui|
        {
            let path = temp_dir().to_str().unwrap().to_string() + "result.png";
            let path1 = temp_dir().to_str().unwrap().to_string() + "result150.png";
            let path2 = temp_dir().to_str().unwrap().to_string() + "result200.png";
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
                    ui.radio_value(&mut self.export_type, ExportZoom::FIRST, "");
                    ui.add_space(35f32);
                    ui.radio_value(&mut self.export_type, ExportZoom::SECOND, "");
                    ui.add_space(55f32);
                    ui.radio_value(&mut self.export_type, ExportZoom::THIRD, "");
                });
            });
        });
        egui::TopBottomPanel::bottom("File stuff").show(ctx, |ui|
        {
            ui.allocate_space(Vec2{x:0f32,y:5f32});
            ui.horizontal(|ui| 
            {
                if ui.button("Import").clicked()
                {
                    self.image = open(self.import_file_dialog.clone().pick_file().unwrap()).unwrap();
                }
                if ui.button("Export").clicked()
                {
                    let path = self.export_file_dialog.clone().set_file_name(&self.result_name).save_file();
                    if path != None
                    {
                        match &self.export_type
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
                        self.reaper_path = path.unwrap().join(r"Data\toolbar_icons");
                    }
                    // println!("{}", self.reaper_path.to_str().unwrap());
                    // println!("{}", self.reaper_path.join(r"150\").to_str().unwrap());
                    // println!("{}", self.reaper_path.join(r"200\").to_str().unwrap());
                }
                if ui.button("Export directly to REAPER").clicked()
                {
                    if self.reaper_path != PathBuf::default()
                    {
                        let path1 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.reaper_path).save_file();
                        if path1 != None
                        {
                            self.image_to_icon(&self.image, 30).save_with_format(path1.unwrap(), image::ImageFormat::Png).unwrap();
                        }
                        let path2 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.reaper_path.join(r"150\")).save_file();
                        if path2 != None
                        {
                            self.image_to_icon(&self.image, 45).save_with_format(path2.unwrap(), image::ImageFormat::Png).unwrap();
                        }
                        let path3 = self.export_file_dialog.clone().set_file_name(&self.result_name).set_directory(&self.reaper_path.join(r"200\")).save_file();
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
            ui.allocate_space(Vec2{x:0f32,y:5f32});
        });
    }
}

impl App 
{
    fn new(cc: &eframe::CreationContext<'_>) -> Self 
    {
        cc.egui_ctx.set_theme(ThemePreference::System);
        install_image_loaders(&cc.egui_ctx);
        Self 
        {
            reaper_path: PathBuf::default(),
            image: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            // image_on_hover: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            // image_clicked: DynamicImage::new(30, 30, image::ColorType::Rgba8),
            on_hover_hue: 0,
            on_hover_contrast: 0f32,
            on_hover_brightness: -30,
            clicked_hue: 0,
            clicked_contrast: 15f32,
            clicked_brightness: 0,
            filter_type: FilterType::Lanczos3,
            import_file_dialog: FileDialog::new().add_filter("Images", &["png", "jpeg", "jpg", "gif", "bmp", "ico", "farbfeld", "hdr", "exr", "pnm", "qoi", "tga", "tiff"]),
            export_file_dialog: FileDialog::new().add_filter("png", &["png"]),
            result_name: "result".to_string(),
            export_type: ExportZoom::FIRST,
        }
    }
    
    // Sizes:
    // Default: 30
    // 150: 45
    // 200: 60
    fn image_to_icon(&self, img: &DynamicImage, size: u32) -> RgbaImage
    {
        let img = DynamicImage::ImageRgba8(resize(img, size, size, self.filter_type)); // open change to input function
        let mut res = RgbaImage::new(size*3, size);

        res.copy_from(&img, 0, 0).unwrap(); // normal
        res.copy_from(&img.huerotate(self.on_hover_hue).adjust_contrast(self.on_hover_contrast).brighten(self.on_hover_brightness), size, 0).unwrap(); // on mouse hover
        res.copy_from(&img.huerotate(self.clicked_hue).adjust_contrast(self.clicked_contrast).brighten(self.clicked_brightness), size*2, 0).unwrap(); // clicked

        res
    }
}

struct App
{
    export_type: ExportZoom,
    reaper_path: PathBuf,
    image: DynamicImage,
    // image_on_hover: DynamicImage,
    // image_clicked: DynamicImage,
    result_name: String,
    on_hover_hue: i32,
    on_hover_contrast: f32,
    on_hover_brightness: i32,
    clicked_hue: i32,
    clicked_contrast: f32,
    clicked_brightness: i32,
    filter_type: FilterType,
    import_file_dialog: FileDialog,
    export_file_dialog: FileDialog,
}