use crate::keplr::*;

#[component]
pub fn KeplrTests() -> impl IntoView {
    let enable_keplr_action = create_action(|_: &()| async move { enable_keplr().await });
    let get_signer_action =
        create_action(|_: &()| async move { get_offline_signer().map(|x| format!("{:?}", x)) });
    let get_account_action = create_action(|_: &()| async move { get_account().await });
    let get_enigma_utils_action =
        create_action(|_: &()| async move { keplr_get_enigma_utils().await });
    let get_viewing_key_action = create_action(|input: &String| {
        let token_address = input.to_owned();
        async move { get_viewing_key(token_address).await }
    });

    let enable_keplr = move |_| enable_keplr_action.dispatch(());
    let get_signer = move |_| get_signer_action.dispatch(());
    let get_account = move |_| get_account_action.dispatch(());
    let get_enigma_utils = move |_| get_enigma_utils_action.dispatch(());
    let get_viewing_key = move |_| {
        // sSCRT
        get_viewing_key_action.dispatch("secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek".to_string())
    };

    let pending_enable = enable_keplr_action.pending();
    let pending_signer = get_signer_action.pending();

    let enabled = enable_keplr_action.value();
    let signer = get_signer_action.value();
    let address = get_account_action.value();
    let enigma_utils = get_enigma_utils_action.value();
    let viewing_key = get_viewing_key_action.value();

    view! {
        <h2>"Keplr Tests"</h2>

        <div class="flex space-x-4">
            <button
                on:click=enable_keplr
            >
                {move || {
                    if let Some(result) = enabled() {
                        match result {
                            Ok(true) => "ENABLED",
                            Ok(false) => "DISABLED",
                            Err(_) => "ERROR",
                        }
                    } else {
                        "Enable Keplr"
                    }
                } }
            </button>
            <span class="font-mono">"window.keplr.enable(CHAIN_ID)"</span>
        </div>

        <div class="space-x-4">
            <button
                on:click=get_signer
            >
                {move || {
                    if let Some(result) = signer() {
                        match result {
                            Ok(_) => "SUCCESS",
                            Err(_) => "ERROR",
                        }
                    } else { "Get Offline Signer" }
                } }
            </button>
            <span class="font-mono">"window.keplr.getOfflineSignerOnlyAmino(CHAIN_ID)"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_account >
                {move || {
                    if let Some(result) = address() {
                        match result {
                            Ok(_) => "SUCCESS",
                            Err(_) => "ERROR",
                        }
                    } else { "Get Account" }
                } }
            </button>
            <span class="font-mono">"keplrOfflineSigner.getAccounts()"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_enigma_utils >
                {move || {
                    if let Some(result) = enigma_utils() {
                        match result {
                            Ok(_) => "SUCCESS",
                            Err(_) => "ERROR",
                        }
                    } else { "Get Enigma Utils" }
                } }
            </button>
            <span class="font-mono">"window.keplr.getEnigmaUtils(CHAIN_ID)"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_viewing_key >
                {move || {
                    if let Some(result) = viewing_key() {
                        match result {
                            Ok(_) => "SUCCESS",
                            Err(_) => "ERROR",
                        }
                    } else { "Get sSCRT Viewing Key" }
                } }
            </button>
            <span class="font-mono">"window.keplr.getSecret20ViewingKey(CHAIN_ID, TOKEN_ADDRESS)"</span>
        </div>

        // Example of how to show a dialog while an action is pending
        <Show
            when=pending_enable
            fallback=|| ()
        >
            <dialog open>
                <p> "Waiting for Approval..." </p>
            </dialog>
        </Show>
    }
}
