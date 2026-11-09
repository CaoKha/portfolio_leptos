use leptos::*;

use crate::composables::site_path;

#[component]
pub fn Etl() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "Data infrastructure and tooling in Rust"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Allianz Trade, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200">"Sep 2023 – Mar 2025"</p>
                    <br/>
                    <img src=site_path("/images/polarsvsspark.png") class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        "For the data science team I converted Excel tables to Parquet with Polars,
                        profiled and optimised the existing data infrastructure with Grafana Pyroscope,
                        and built Rust CLI tools that removed manual steps from their workflow.
                        The Python side of the platform ran on PySpark, Kafka and FastAPI."
                    </p>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "1: Excel tables to Apache Parquet with Polars"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Converted Excel tables to Apache Parquet files with Polars for the data infrastructure.
                            Moving the tabular data from a row-based format to a columnar format made storage and
                            querying substantially cheaper."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "2: Profiling with Grafana Pyroscope"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Profiled and optimised the existing data infrastructure with Grafana Pyroscope,
                            targeting the hotspots the data scientists hit most often and keeping the code
                            platform-independent."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "3: CLI tools in Rust"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Built CLI tools in Rust that removed manual steps from the data scientists' workflow,
                            with the services structured as a cargo workspace with one crate per domain."
                        </p>
                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src=site_path("/images/etl.png") class="aspect-auto"/>
                        <br/>
                        <img src=site_path("/images/rust_etl.jpeg") class="aspect-auto"/>
                    </div>
                </div>
            </section>
        </main>
    }
}
