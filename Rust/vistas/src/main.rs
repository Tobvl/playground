use iced::{Element, Sandbox, Settings, Length};
use iced::widget::{Column, Button, Container, Row};

mod pagina_principal;
use pagina_principal::PaginaPrincipal;
mod pagina_secundaria;
use pagina_secundaria::PaginaSecundaria;

fn main() -> Result <(), iced::Error>{
    Vistas::run(Settings::default())
}

struct Vistas {
    pagina_actual : Vista,
    pagina_principal: PaginaPrincipal,
    pagina_secundaria: PaginaSecundaria
}


#[derive(Debug, Clone, Copy)]
pub enum Vista {
    Principal,
    Secundaria
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    CambioPagina(Vista)
}

impl Sandbox for Vistas{

    type Message = Messages;

    fn new() -> Self{
        Vistas {
            pagina_actual: Vista::Principal,
            pagina_principal: PaginaPrincipal::new(),
            pagina_secundaria: PaginaSecundaria::new()
        }
    }
    
    fn title (&self) -> String{
        String::from("Multiples ventanas")
    }

    fn update (&mut self, mensaje: Self::Message){
        match mensaje{
            Messages::CambioPagina(pagina) => {self.pagina_actual = pagina;}
        }
    }

    fn view (&self) -> Element<Self::Message> {
        let menu = Container::new(
            Row::new()
                .padding(15)
                .width(Length::Fill)
                .height(Length::from(60))
                .push(Button::new("Principal").on_press(Messages::CambioPagina(Vista::Principal)))
                .push(Button::new("Secundaria").on_press(Messages::CambioPagina(Vista::Secundaria)))
                .spacing(10));

        let contenido = match self.pagina_actual {
            Vista::Principal => self.pagina_principal.view(),
            Vista::Secundaria => self.pagina_secundaria.view()
        };

        Column::new()
            .push(menu)
            .push(contenido)
            .into()
    }
}




