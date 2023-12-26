use yew::prelude::*;

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl font-extrabold text-center py-4 sm:text-5xl lg:text-6xl">{"Regler"}</h1>

            <p class="text-lg font-bold text-center tracking-tight text-balance">{"Spelet spelas med en D20. Alla börjar med att slå tärningen en gång. Personen som slår högst får välja klass först. Vid slag av 20 får man även välja DM. Om två personer slår samma får de tävla i sten sax påse om vem som får välja först."}</p>

            <div class="divider"></div>

            <ol class="list-decimal list-inside font-bold space-y-4">
                <li>{"Critical Failure! Ta en shot!"}</li>
                <li>{"Svep resten!"}</li>
                <li>{"Slå och drick"}</li>
                <li>{"Four is whore"}</li>
                <li>{"Alla dricker"}</li>
                <li>{"Six is dicks"}</li>
                <li>{"Slå igen"}</li>
                <li>{"Räkna till 3. På 3 håller alla upp ett nummer 1-5. Drick så många klunkar som antal personer som väljer samma nummer. Är du själv, dricker du inget!"}</li>
                <li>{"Ta två klunkar"}</li>
                <li>{"Ta ett ölkort! Ge det till någon annan."}</li>
                <li>{"Alla byter plats"}</li>
                <li>{"Slå igen. Vid jämt slag dricker personen till höger 2 klunkar, vid udda dricker personen till vänster 2 klunkar."}</li>
                <li>{"Vänta på nästa spelare. Både du och nästa spelare får det tärningslaget."}</li>
                <li>{"Slå igen"}</li>
                <li>{"Question Master"}</li>
                <li>{"Alla andra dricker en klunk."}</li>
                <li>{"Slå och ge"}</li>
                <li>{"Välj en kompis! Kompisen måste dricka med dig"}</li>
                <li>{"Ge en shot!"}</li>
                <li>{"Ta ett ölkort!"}</li>
            </ol>

        </div>
    }
}
