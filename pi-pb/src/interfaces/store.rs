use crate::schemas::{app::App, store::Store};
use pi_lib::utils::download::download;

use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
impl Store {
    pub fn new(apps: Vec<App>) -> Store {
        Store { apps: apps }
    }
    pub fn export(&self) -> Result<()> {
        let output_file = &format!("./dist/{}.json", "store");
        let mut f = File::create(output_file)?;
        f.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;

        f.sync_all()?;
        Ok(())
    }
    pub fn pull(&self) {
        self.apps.iter().for_each(|app| {
            let address = &format!(
                "http://mirror.xtom.com.hk/archlinux/{}/os/{}/{}",
                &app.SUBREPO, &app.ARCH, &app.FILENAME
            );

            let file_path = &format!("dist/{}/{}", &app.SUBREPO, &app.FILENAME);
            match download(file_path, &app.NAME, address) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        })
    }
}
