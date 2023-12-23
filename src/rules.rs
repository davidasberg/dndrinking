use yew::prelude::*;

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center">
            <h1 class="text-4xl font-extrabold tracking-tight text-center sm:text-5xl lg:text-6xl">{"Rules"}</h1>

            <p class="text-lg font-bold tracking-tight text-center w-1/2">{"Spelet spelas med en D20. Alla börjar med att slå tärningen en gång. Personen som slår högst får välja klass först. Vid slag av 20 får man även välja DM. Om två personer slår samma får de tävla i sten sax påse om vem som får välja först."}</p>

            <ol class="list-decimal list-inside space-y-4">
                <li class="text-lg">{"Critical Failure! Ta en shot!"}</li>
                <li class="text-lg">{"Svep resten!"}</li>
                <li class="text-lg">{"Slå och drick"}</li>
                <li class="text-lg">{"Four is whore"}</li>
                <li class="text-lg">{"Alla dricker"}</li>
                <li class="text-lg">{"Six is dicks"}</li>
                <li class="text-lg">{"Slå igen"}</li>
                <li class="text-lg">{"Räkna till 3. På 3 håller alla upp ett nummer 1-5. Drick så många klunkar som antal personer som väljer samma nummer. Är du själv, dricker du inget!"}</li>
                <li class="text-lg">{"Ta två klunkar"}</li>
                <li class="text-lg">{"Ta ett ölkort! Ge det till någon annan."}</li>
                <li class="text-lg">{"Alla byter plats"}</li>
                <li class="text-lg">{"Slå igen. Vid jämt slag dricker personen till höger 2 klunkar, vid udda dricker personen till vänster 2 klunkar."}</li>
                <li class="text-lg">{"Vänta på nästa spelare. Både du och nästa spelare får det tärningslaget."}</li>
                <li class="text-lg">{"Slå igen"}</li>
                <li class="text-lg">{"Question Master"}</li>
                <li class="text-lg">{"Alla andra dricker en klunk."}</li>
                <li class="text-lg">{"Slå och ge"}</li>
                <li class="text-lg">{"Välj en kompis! Kompisen måste dricka med dig"}</li>
                <li class="text-lg">{"Ge en shot!"}</li>
                <li class="text-lg">{"Ta ett ölkort!"}</li>
            </ol>

        </div>
    }
}
