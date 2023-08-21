use crossterm::{execute, style::{Color, SetForegroundColor, SetBackgroundColor}, ExecutableCommand};
use crate::type_text;


pub fn color_me(text: &str, color: Color){
    let result = execute!(
        std::io::stdout(),
        SetForegroundColor(color),
    );

    if result.is_err(){
        eprint!("Colour Error");
        return;
    }
    type_text(text);
    let _ = execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Reset),
    );
}

