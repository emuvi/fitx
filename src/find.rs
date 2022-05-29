use rubx::RubxResult;

use std::path::PathBuf;

pub struct Setup {
  pub extensions: Option<()>,
  pub by_name: Option<String>,
}

pub fn start(from: PathBuf, setup: Setup) -> RubxResult<()> {
  if let Some(()) = setup.extensions {
    println!("Finding extensions:");
    let mut found: Vec<String> = vec![];
    find_extensions(from, &mut found, &setup)?;
  }
  Ok(())
}

fn find_extensions(from: PathBuf, found: &mut Vec<String>, setup: &Setup) -> RubxResult<()> {
  let act = |from: &PathBuf, _: &Setup, found: &mut Vec<String>| {
    if let Some(ext) = from.extension() {
      if let Some(ext) = ext.to_str() {
        let ext = ext.to_string();
        if !found.contains(&ext) {
          println!("{}", ext);
          found.push(ext);
        }
      }
    }
  };
  find(from, found, setup, &act)?;
  Ok(())
}

fn find<F: Fn(&PathBuf, &Setup, &mut Vec<String>)>(
  from: PathBuf,
  found: &mut Vec<String>,
  setup: &Setup,
  act: &F,
) -> RubxResult<()> {
  if from.is_dir() {
    for inside in std::fs::read_dir(from)? {
      let inside = inside?;
      let inside = inside.path();
      find(inside, found, setup, act)?;
    }
  } else {
    act(&from, setup, found);
  }
  Ok(())
}
