use iced::{Element, Length};
use iced::widget::{Column, Container, Text};

use crate::Messages;

pub struct PaginaPrincipal;

impl PaginaPrincipal {
    pub fn new() -> Self{
        PaginaPrincipal
    }
    
    pub fn view(&self) -> Element<Messages> {
        println!("PAGINA PRINCIPAL");
        let label = Text::new("Pagina PRINCIPAL");
        let columna = Column::new().push(label).spacing(4);
        let pagina = Container::new(columna)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();
        return pagina.into()
    }
}