use clap::{App, Arg};
use std::fs::File;
use std::process;

use imprint_of_light::{
    config::Config,
    render::{render as r, Entity, Scene},
};

fn main() {
    args_check();
}

fn args_check() {
    let matches = App::new("imprint_of_light")
        .version("0.1.0")
        .author("Luke Euler <luke16times@gmail.com>")
        .about("draw the light with shapes in 2D")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("the config file for rendering images")
                .default_value("config.json"),
        )
        .get_matches();

    let config_file_name = matches.value_of("config").unwrap();

    let file = match File::open(config_file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("{}: {}", config_file_name, e.to_string());
            process::exit(1)
        }
    };

    let configs: Vec<Config> = serde_json::from_reader(file).unwrap();

    for item in configs {
        if !item.enable {
            continue;
        }
        if item.scenes.len() == 0 {
            continue;
        }
        println!("try to render image: {}", item.out);

        let mut entities: Vec<Entity> = Vec::new();
        for entity_json in item.scenes {
            entities.push(entity_json.get_entity());
        }
        let scene = Scene { entities };
        let img = r(
            &scene,
            (item.width, item.height),
            item.stratification,
            item.max_depth,
        );
        img.save(item.out.clone()).unwrap();
    }
}
