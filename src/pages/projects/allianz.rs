use leptos::*;

#[component]
pub fn Allianz() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "Converting Excel files into fully functional Git repository"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Allianz Trade, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200 ">2023 - now</p>
                    <br/>
                    // <img src="/images/portailbleu.png" class="m-auto lg:w-1/3"/>
                    <p class="text-center dark:text-slate-400 sm:text-xl pt-4 lg:pt-10">
                        "Simply reconstruct the Git history for the legacy project. 
                        I'm using this opportunity to learn and work with Rust!"
                    </p>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-52">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "1: Converting Excel file into Apache parquet using polars-rs"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Implement a Rust function to read data from an Excel file and write it into an Apache Parquet 
                            format using the Polars library. This process will enhance performance by converting the tabular data 
                            from a row-based format (Excel) into a columnar format (Parquet) for optimized storage and querying."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "2: Link rows and recreate Git history from parquet file using polars-rs and git2"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Using the parquet file generated in the previous step, implement a function that links 
                            rows together and recreates a Git-like history for each row. Each row should be treated as a commit, 
                            with relationships (e.g., parent-child) established between rows based on specified criteria 
                            (such as IDs, timestamps, or other unique identifiers)."
                        </p>
                        <br/>
                        <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
                            <a href="https://github.com/CaoKha/rewrite_git_history" target="_blank">
                                <span>Demo Code</span>
                            </a>
                        </button>

                    </div>
                    <div class="pt-1 lg:pt-0 block m-auto">
                        <img src="/images/legacy_git.png" class="aspect-auto"/>
                    </div>

                </div>

            </section>

        </main>
    }
}
