use std::path::PathBuf;
use std::fs::File;
use std::process::Command;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Figure {
    prelude: String,
    content: String,
}

impl Figure {
    pub fn tex(&self) -> String {
        format!(r#"
\documentclass[crop,tikz,multi=false]{{standalone}}[2012/04/13]

{prelude}

%\usetikzlibrary{{...}}% tikz package already loaded by 'tikz' option
\makeatletter
\begin{{document}}

{content}

\end{{document}}
    "#, prelude = self.prelude, content = self.content)
    }

    pub fn compile(&self) -> PathBuf {
        let id = uuid::Uuid::new_v4();
        let prefix = format!("/tmp/fig-{}", id);

        let mut file = File::create(format!("{}.tex", prefix)).expect("Failed to create temp file");
        println!("Tex:  {}", self.tex());
        file.write_all(self.tex().into_bytes().as_mut()).unwrap();

        Command::new("/usr/bin/latex")
            .current_dir("/tmp/")
            .arg(format!("{}.tex", prefix))
            .output()
            .expect("Failed to compile LaTeX");

        Command::new("/usr/bin/dvisvgm")
            .current_dir("/tmp/")
            .arg(format!("{}.dvi", prefix))
            .output()
            .expect("Failed to convert to svg");

        println!("{}.svg", prefix);

        PathBuf::from(format!("{}.svg", prefix))
    }
}

impl Default for Figure {
    fn default() -> Figure {
        Figure {
            prelude: "\\usepackage{chemfig}".into(),
            content: "\\chemfig{*6(-=-=-=)}".into(),
        }
    }
}
