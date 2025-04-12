# Reaper Icon Creator
This tool creates icons for Reaper's toolbar from your images.

![image](https://github.com/user-attachments/assets/86202cc6-149a-4d19-ba67-1e631a975567)
## Features:
### Top Panel:
- **On Hover Hue/Contrast/Brightness** - Settings for the icon that is displayed when you mouse over the button (Second image in row).
- **Clicked Hue/Contrast/Brightness** - Settings for the icon that is displayed when the button is clicked (Third image in row).
- **Filter Type** - Selecting the type of image sampling.
- **Result Icon Name (result)** - Text edit box that specifies the name of the output file.
- **Render** - A button that renders icons to the central panel.
- **Restore default settings** - The app has an auto-save feature. This button returns the settings to the original version (except path to Reaper).
### Central Panel:
- **100/150/200** - The different icon sizes that can be in Reaper.
- **Export buttons** - Select the size of the icon you are exporting with the “Export” button.
### Bottom Panel:
- **Import** - Select an image for modification.
- **Export** - Exports a single icon of the selected size.
- **Select REAPER folder** - Select the main Reaper folder. You can find it by clicking “Show REAPER resource path in explorer/finder...” in the “Options” window in Reaper.
- **Export directly to REAPER** - Automatically exports all icon sizes to the Reaper folder. For now, a different save window opens for each individual icon.
## Used tools:
- [image](https://github.com/image-rs/image)
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- [egui](https://github.com/emilk/egui)
- [Rusty File Dialog](https://github.com/PolyMeilex/rfd)
