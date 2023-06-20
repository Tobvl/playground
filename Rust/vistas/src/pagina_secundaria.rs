use iced::{Element, Length};
use iced::widget::{Column, Container, Text};

use crate::Messages;

pub struct PaginaSecundaria;

impl PaginaSecundaria {
    pub fn new() -> Self{
        PaginaSecundaria
    }
    
    pub fn view(&self) -> Element<Messages> {
        println!("PAGINA SECUNDARIA");
        let label = Text::new("Pagina SECUNDARIA");
        let columna = Column::new().push(label).spacing(4);
        let pagina = Container::new(columna)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();
        return pagina.into()
    }
}