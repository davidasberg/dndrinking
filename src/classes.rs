use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Classes)]
fn classes() -> Html {
    html! {
        <section class="grid lg:grid-cols-1 items-center gap-8 w-4/5">
            <ClassCard class="Fighter" description="Fight your way through anything" image="img/fighter.jpg" />
            <ClassCard class="Ranger" description="Shoot stuff" image="img/ranger.jpg" />
            <ClassCard class="Wizard" description="Cast spells" image="img/wizard.jpg" />
            <ClassCard class="Rogue" description="Sneak around" image="img/rogue.jpg" />
            <ClassCard class="Cleric" description="Heal stuff" image="img/cleric.jpg" />
            <ClassCard class="Barbarian" description="Rage" image="img/barbarian.jpg" />
            <ClassCard class="Druid" description="Turn into animals" image="img/druid.jpg" />
            <ClassCard class="Monk" description="Punch stuff" image="img/monk.jpg" />
            <ClassCard class="Paladin" description="Smite stuff" image="img/paladin.jpg" />
            <ClassCard class="Sorcerer" description="Cast spells" image="img/sorcerer.jpg" />
            <ClassCard class="Warlock" description="Cast spells" image="img/warlock.jpg" />
            <ClassCard class="Bard" description="Cast spells" image="img/bard.jpg" />
        </section>
    }
}

#[function_component(BeerCard)]
fn beer_card() -> Html {
    html! {
        <section class="py-10 bg-white sm:py-16 lg:py-24">
            <h1 class="text-4xl font-extrabold tracking-tight text-center text-gray-900 sm:text-5xl lg:text-6xl">{"Ölkort"}</h1>
        </section>
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
        <div class="card flex shadow-xl bg-neutral">
            <figure><img src={image} alt={class}/></figure>
            <div class="card-body">
                    <h2 class="card-title">{class}</h2>
                    <p>{description}</p>
                <div class="card-actions justify-end">
                    <button class="btn btn-primary">{"Read more"}</button>
                </div>
            </div>
        </div>
    }
}
