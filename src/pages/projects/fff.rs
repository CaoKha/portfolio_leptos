use leptos::*;

#[component]
pub fn FFF() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        Football Applications
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        France Footbal Federation, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200 ">2021 - 2023</p>
                    <br/>
                    // <img src="/images/portailbleu.png" class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        "This is just a list of all the projects I have done at FFF. Most of them are NDAs, hence I can't show code."
                    </p>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            1: Portail Bleu
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "A Customer Portal Website where you can access all information related to your FFF account."
                        </p>

                    </div>
                    <div class="pt-1 lg:pt-0 block m-auto">
                        <img src="/images/portailbleu.png" class="aspect-auto"/>
                    </div>
                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            2: Licence Club Federale
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Money Calculator Application: This tool assists you in calculating expenses for your club, similar to Settle Up"
                        </p>
                        <br/>
                    </div>
                    <div class="pt-5 block m-auto lg:pt-0">
                        <img src="/images/lcf.png" class="aspect-auto"/>
                    </div>

                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "3: Cotisation en Ligne"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">"Payment online for FFF"</p>
                        <br/>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/cotis.png" class="aspect-auto"/>
                    </div>
                </div>

                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "4: Vie de Clubs"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Club Registration Application. Legacy project. Just some maintenance and bug fixing stuffs."
                        </p>
                        <br/>
                    </div>
                    <div class="pt-5 lg:pt-0">
                        <img src="/images/vdc.png" class="aspect-auto"/>
                    </div>
                </div>
            </section>

        </main>
    }
}
