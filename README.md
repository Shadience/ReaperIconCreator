# Reaper Icon Creator
This tool creates icons for Reaper's toolbar from your images.

![showcase](https://github.com/user-attachments/assets/03f8cab5-434c-41fd-b806-9dac66a77e44)
## Features:
### Top Panel:
- **Icon Hue/Contrast/Brightness** - Settings for the icon that is displayed by default (First image in row).
- **On Hover Hue/Contrast/Brightness** - Settings for the icon that is displayed when you mouse over the button (Second image in row).
- **Clicked Hue/Contrast/Brightness** - Settings for the icon that is displayed when the button is clicked (Third image in row).
- **Render** - A button that renders icons to the central panel.
- **Restore default settings** - The app has an auto-save feature. This button returns the settings to the original version (except path to Reaper).
- **Filter Type** - Selecting the type of image sampling.
- **Result Icon Name (result)** - Text edit box that specifies the name of the output file. For now, it is not available for modification
### Central Panel:
- **Icons** - Selecting an icon for viewing.
- **100/150/200** - The different icon sizes that can be in Reaper.
- **Export checkboxes** - Select the size of the icons you are exporting with the “Export” button.
### Bottom Panel:
- **Import** - Selecting an images for modification.
- **Export** - Exports icons of selected sizes.
- **Select REAPER folder** - Select the main Reaper folder. You can find it by clicking “Show REAPER resource path in explorer/finder...” in the “Options” window in Reaper. You only have to specify it once, it will be saved for next sessions.
- **Export directly to REAPER** - Automatically exports all icon sizes to the Reaper folder. For now, a different save window opens for each individual icon.
## Used tools:
- [image](https://github.com/image-rs/image)
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- [egui](https://github.com/emilk/egui)
- [Rusty File Dialog](https://github.com/PolyMeilex/rfd)
