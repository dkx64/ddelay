use nih_plug::prelude::*;

use ddelay::AudioPlugin;

fn main() {
    nih_export_standalone::<AudioPlugin>();
}
