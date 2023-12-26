use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Classes)]
pub fn classes() -> Html {
    html! {
        <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
        <h1 class="text-4xl my-10 font-extrabold tracking-tight text-center sm:text-5xl lg:text-6xl">{"Klasser"}</h1>
        <section class="grid lg:grid-cols-2 gap-8 lg:w-4/5">
            <ClassCard class="Barbarian" description="Du får dela ut 1 klunk till valfri person varje gång du dricker." image="img/barbarian.jpg" />
            <ClassCard class="Bard" description="I början av din runda får du välja en mate. Din mate dricker alltid när du dricker, samma mängd." image="img/bard.jpg" />
            <ClassCard class="Cleric" description="Bless or Bane! Använd din reaction för att addera 2 eller subtrahera 2 från ett tärningsslag, en gång per runda. 1 och 20 går inte.            " image="img/cleric.jpg" />
            <ClassCard class="Druid" description="Du är en Furry. Grattis." image="img/druid.jpg" />
            <ClassCard class="Fighter" description="Du kan istället för att dricka, utmana en annan person i sten sax påse. Förlorar du dricker du dubbelt, annars dricker den andra personen. Gäller ej på Critical Failure." image="img/fighter.jpg" />
            <ClassCard class="Monk" description="Du får välja alkoholfritt." image="img/monk.jpg" />
            <ClassCard class="Paladin" description="Använd din reaction för att halvera antal klunkar, ge den andra hälften till någon annan." image="img/paladin.jpg" />
            <ClassCard class="Ranger" description="Placera Hunters Mark på någon i början av din runda. Personen rullar med disadvantage på sin nästa runda." image="img/ranger.jpg" />
            <ClassCard class="Rogue" description="Evasion! Rulla för att halvera antal klunkar. Över 10 halverar, 1 dubblar." image="img/rogue.jpg" />
            <ClassCard class="Sorcerer" description="Använd din reaction för att redirecta vad som helst till en annan person. En gång per runda. Gäller inte 1 och 20." image="img/sorcerer.jpg" />
            <ClassCard class="Warlock" description="Du får rulla om en gång per runda, i utbyte av 2 klunkar. Gäller ej Critical Failure." image="img/warlock.jpg" />
            <ClassCard class="Wizard" description="Använd sköld som reaction! Förhindra en attack en gång per runda. Få tillbaka din reaction efter din egna tur." image="img/wizard.jpg" />
        </section>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct ClassCardProps {
    pub class: AttrValue,
    pub description: AttrValue,
    pub image: AttrValue,
}

#[function_component(ClassCard)]
fn class_card(class_card: &ClassCardProps) -> Html {
    let ClassCardProps {
        class,
        description,
        image,
    } = class_card;

    html! {
        <div class="card flex shadow-xl bg-neutral transition duration-0 hover:duration-150 hover:scale-105">
            <figure><img src={image} alt={class}/></figure>
            <div class="card-body">
                    <h2 class="card-title">{class}</h2>
                    <p>{description}</p>
                // <div class="card-actions justify-end">
                //     <button class="btn btn-primary">{"Read more"}</button>
                // </div>
            </div>
        </div>
    }
}
