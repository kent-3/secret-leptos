use crate::keplr::*;
use crate::GlobalState;
use leptos::*;

#[component]
pub fn KeplrTests() -> impl IntoView {
    let ctx = use_context::<GlobalState>().expect("provide global state context");
    let setter = ctx.keplr_enabled;

    // TODO - create "resources" that try to get the viewing keys (later balances) without making
    // keplr pop-up
    // let balances_resource = create_resource(move || signal_user_approves, get_all_the_balances);

    // actions
    let enable_keplr_action = create_action(|_input: &()| async move { enable_keplr().await });

    let get_signer_action =
        create_action(
            |_input: &()| async move { get_offline_signer().map(|x| format!("{:?}", x)) },
        );
    let get_account_action = create_action(|_input: &()| async move { get_account().await });

    let get_enigma_utils_action =
        create_action(|_input: &()| async move { keplr_get_enigma_utils().await });

    let get_viewing_key_action = create_action(|input: &String| {
        let token_address = input.to_owned();
        async move { get_viewing_key(token_address).await }
    });

    // dispatchers
    let enable_keplr = move |_| enable_keplr_action.dispatch(());
    let get_signer = move |_| get_signer_action.dispatch(());
    let get_account = move |_| get_account_action.dispatch(());
    let get_enigma_utils = move |_| get_enigma_utils_action.dispatch(());
    let get_sscrt_viewing_key = move |_| {
        let tokens = vec![
            "secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek",
            "secret1k6u0cy4feepm6pehnz804zmwakuwdapm69tuc4",
        ];
        for token in tokens {
            get_viewing_key_action.dispatch(token.to_string())
        }
    };

    let pending_enable = enable_keplr_action.pending();
    let pending_signer = get_signer_action.pending();

    let enabled = enable_keplr_action.value();
    let signer = get_signer_action.value();
    let address = get_account_action.value();
    let sscrt_key = get_viewing_key_action.value();

    let derived_signal = move || {
        if let Some(Ok(status)) = enabled() {
            setter.set(status)
        }
    };

    // use this to access the input that was used for the action
    // let submitted = action.input(); // RwSignal<Option<String>>
    // let pending = action.pending(); // ReadSignal<bool>
    // let address = action.value(); // RwSignal<Uuid>>

    view! {
        <h2>"Keplr Tests"</h2>
        <div class="space-x-4">
            <button
                on:click=enable_keplr
                class="transition-all duration-200"
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
            <span class="font-mono" >"window.keplr.enable(CHAIN_ID)"</span>
        </div>
        // This seems like a hack to get this reactive function to happen...
        {derived_signal}

        <div class="space-x-4">
            <button
                on:click=get_signer
                class=("success", pending_enable)
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
            <span class="font-mono" >"window.keplr.getOfflineSignerOnlyAmino(CHAIN_ID)"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_account >
                "Get Account"
            </button>
            <span class="font-mono" >"keplrOfflineSigner.getAccounts()"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_enigma_utils >
                "Get Enigma Utils"
            </button>
            <span class="font-mono" >"window.keplr.getEnigmaUtils(CHAIN_ID)"</span>
        </div>

        <div class="space-x-4">
            <button on:click=get_sscrt_viewing_key >
                "Get sSCRT Viewing Key"
            </button>
            <span class="font-mono" >"window.keplr.getSecret20ViewingKey(CHAIN_ID, TOKEN_ADDRESS)"</span>
            <p>
        {move || {
            if let Some(result) = sscrt_key() {
                match result {
                    Ok(string) => string,
                    Err(_) => "ERROR".to_string(),
                }
            } else { "key tbd".to_string() }
        }    }
        </p>
        </div>

        // KEEP THIS as an example of how to show a dialog while something is true
        // <Show
        //     when=pending_enable
        //     fallback=|| view! { <meta/> }
        // >
        // <dialog open>
        //     <p>
        //         "Waiting for Approval..."
        //         // <code>{move || format!("{:#?}", pending())}</code>
        //     </p>
        // </dialog>
        // </Show>
    }
}
