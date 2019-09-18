use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Java version
///
/// Will display the Java version if any of the following criteria are met:
///     - Current directory contains a file with a `.java`, `.class` or `.jar` extension
///     - Current directory contains a `pom.xml`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_java_project = context
        .try_begin_scan()?
        .set_files(&["pom.xml"])
        .set_extensions(&["java", "class", "jar"])
        .is_match();

    if !is_java_project {
        return None;
    }

    let java_version =  get_java_version()?;
    const JAVA_CHAR: &str = "☕ ";

    let mut module = context.new_module("java");
    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Red.bold());
    module.set_style(module_style);

    let formatted_version = format_java_version(java_version.as_str())?;
    module.new_segment("symbol", JAVA_CHAR);
    module.new_segment("version", &formatted_version);

    Some(module)
}

fn get_java_version() -> Option<String> {
    match Command::new("java").arg("-Xinternalversion").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

// TODO support other jvms than openjdk
fn format_java_version(java_stdout: &str) -> Option<String> {
    let start = java_stdout.find("JRE (")? + "JRE (".len();
    let end = start + (java_stdout[start..].find(|c| match c {
            '0'..='9' | '.' => false,
            _ => true
        })?);
    Some(format!("v{}", &java_stdout[start..end]))
}

#[test]
fn test_format_java_version() {
    let java_8 = "OpenJDK 64-Bit Server VM (25.222-b10) for linux-amd64 JRE (1.8.0_222-b10), built on Jul 11 2019 10:18:43 by \"openjdk\" with gcc 4.4.7 20120313 (Red Hat 4.4.7-23)";
    let java_11 = "OpenJDK 64-Bit Server VM (11.0.4+11-post-Ubuntu-1ubuntu219.04) for linux-amd64 JRE (11.0.4+11-post-Ubuntu-1ubuntu219.04), built on Jul 18 2019 18:21:46 by \"buildd\" with gcc 8.3.0";
    assert_eq!(Some("v11.0.4".to_owned()), format_java_version(java_11));
    assert_eq!(Some("v1.8.0".to_owned()), format_java_version(java_8));
}