use leptos::*;

#[component]
pub fn Etl() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "Rewriting the ETL process from PySpark to Rust with Polars"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Allianz Trade, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200">2024 - present</p>
                    <br/>
                    <img src="/images/polarsvsspark.png" class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        "This project involves rewriting an existing ETL pipeline from PySpark to Rust using Polars.
                        The goal is to improve performance and reduce memory usage by transitioning from Spark's multiprocessing 
                        to Rust's multithreading. This shift allows us to move from a multi-server architecture to a single-server model, 
                        significantly reducing operational costs. If the single-server approach doesn't meet our needs, 
                        we can easily revert to the original model using DataFusion. Additionally, Rust can be seamlessly 
                        integrated with Python via Maturin, ensuring flexibility across languages."
                    </p>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "1: Extract data from diverse sources using Rust"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "The first step involves reading data from various formats (e.g., CSV, JSON, Parquet, Postgres database, Kafka topic) using Polars and Rust.
                            The goal is to build a robust, efficient extraction pipeline while taking advantage of Rust's strong typing and performance."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "2: Transform data efficiently with Polars DataFrames"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Use the Polars library to perform data transformations, including filtering, joining, and aggregation of large datasets.
                            This step leverages Rust’s parallelism and memory safety to optimize performance and ensure data integrity."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "3: Load data into the destination with Rust"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "The final step is loading the transformed data back into storage or databases, ensuring minimal latency and maximum throughput.
                            Rust’s fine-grained control over memory and threads helps in achieving high efficiency in this phase."
                        </p>
                        <br/>
                        <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
                            <a href="https://github.com/CaoKha/etl-rs" target="_blank">
                                <span>Demo Code</span>
                            </a>
                        </button>

                    </div>
                    <div class="pt-5 lg:pt-0 block m-auto">
                        <img src="/images/etl.png" class="aspect-auto"/>
                        <br/>
                        <img src="/images/rust_etl.jpeg" class="aspect-auto"/>
                    </div>
                </div>

            </section>

        </main>
    }
}
