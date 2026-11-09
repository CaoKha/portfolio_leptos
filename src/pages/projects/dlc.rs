use leptos::*;

const STACK: [&str; 12] = [
    "Rust",
    "rust-bitcoin",
    "secp256k1",
    "Tokio",
    "Serde",
    "PSBT",
    "Taproot",
    "Schnorr",
    "adaptor signatures",
    "CETs",
    "Bitcoin Core",
    "cargo workspace",
];

#[component]
pub fn DlcMarkets() -> impl IntoView {
    view! {
        <main class="min-h-screen py-10">
            <section>
                <div class="mx-5 lg:m-auto lg:w-2/3">
                    <h1 class="text-center text-4xl font-bold tracking-tight dark:text-slate-200 sm:text-5xl">
                        "DLC Markets — protocol and engine in Rust"
                    </h1>

                    <h2 class="mt-3 text-center text-lg font-medium tracking-tight dark:text-slate-200 sm:text-xl">
                        Ito, Paris, France
                    </h2>

                    <p class="text-center font-medium dark:text-slate-200">"Mar 2025 – Jan 2026"</p>

                    <p class="pt-4 text-center dark:text-slate-400 sm:text-xl lg:pt-10">
                        "Production Rust implementation of Discreet Log Contracts: the oracle engine, the contract lifecycle, and on-chain settlement with adaptor signatures and oracle attestations."
                    </p>

                    <ul class="mx-auto mt-6 flex max-w-3xl flex-wrap justify-center" aria-label="Technologies used">
                        {STACK
                            .map(|tech| {
                                view! {
                                    <li class="mr-1.5 mt-2">
                                        <div class="flex items-center rounded-full bg-teal-400/10 px-3 py-1 text-xs font-medium leading-5 text-teal-700 dark:text-teal-300">
                                            {tech}
                                        </div>
                                    </li>
                                }
                            })}
                    </ul>
                </div>
            </section>
            <section>
                <div class="mx-5 grid pt-10 lg:mx-10 lg:grid-cols-2 lg:gap-4 lg:pt-32">
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "1: Oracle engine"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Refactored the oracle engine: event announcement, attestation and signing paths. Turned a PhD researcher's protocol research into production Rust code."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "2: Contract lifecycle"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Implemented the DLC contract lifecycle: offer, accept and sign message exchange, funding transaction, and Contract Execution Transactions (CETs). Settled contracts on-chain with adaptor signatures and oracle attestations."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "3: Wallet and transaction building"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Built on the rust-bitcoin ecosystem — secp256k1, PSBT, Taproot and Schnorr signatures — and wrote the wallet, key management and transaction-building logic."
                        </p>
                    </div>
                    <div>
                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "4: Testing against Bitcoin Core"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Wrote end-to-end integration tests against Bitcoin Core in regtest and signet, covering the full lifecycle from announcement to settlement."
                        </p>
                        <br/>

                        <h1 class="mt-3 text-lg font-medium tracking-tight dark:text-slate-200 sm:text-2xl">
                            "5: Cargo workspace"
                        </h1>
                        <p class="pt-5 dark:text-slate-400 sm:text-xl">
                            "Split the codebase into a cargo workspace: one crate per domain (oracle, contract lifecycle, wallet), with the domain crates free of I/O. Optimised and refactored existing crates and kept the workspace building on Linux and macOS."
                        </p>
                        <br/>
                        <button class="rounded-full bg-slate-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
                            <a href="https://dlcmarkets.com/" target="_blank" rel="noreferrer">
                                <span>"DLC Markets"</span>
                            </a>
                        </button>
                    </div>
                </div>
            </section>
        </main>
    }
}
