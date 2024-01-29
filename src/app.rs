use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DragEvent, FormData, HtmlDialogElement, HtmlFormElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct CreateGroupArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct GroupIdArgs {
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct DeleteGroupCharacterArgs {
    group: usize,
    character: usize,
}

#[derive(Serialize, Deserialize)]
struct SwapGroupCharacterArgs {
    group: usize,
    a: usize,
    b: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Breed {
    Ecaflip,
    Eniripsa,
    Iop,
    Cra,
    Feca,
    Sacrieur,
    Sadida,
    Osamodas,
    Enutrof,
    Sram,
    Xélor,
    Pandawa,
    Roublard,
    Zobal,
    Steamer,
    Eliotrope,
    Huppermage,
    Ouginak,
    Forgelance,
}

impl Breed {
    pub fn to_css_id(&self) -> i8 {
        match self {
            Breed::Ecaflip => 6,
            Breed::Eniripsa => 7,
            Breed::Iop => 8,
            Breed::Cra => 9,
            Breed::Feca => 1,
            Breed::Sacrieur => 11,
            Breed::Sadida => 10,
            Breed::Osamodas => 2,
            Breed::Enutrof => 3,
            Breed::Sram => 4,
            Breed::Xélor => 5,
            Breed::Pandawa => 12,
            Breed::Roublard => 13,
            Breed::Zobal => 14,
            Breed::Steamer => 15,
            Breed::Eliotrope => 16,
            Breed::Huppermage => 17,
            Breed::Ouginak => 18,
            Breed::Forgelance => 20,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub breed: Breed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub characters: Vec<Character>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    groups: Vec<Group>,
    current_group: usize,
}

#[component]
pub fn App() -> impl IntoView {
    let settings = create_resource(
        || (),
        |_| async move {
            let value = invoke("get_settings", to_value(&None::<bool>).unwrap()).await;

            from_value::<Settings>(value).unwrap()
        },
    );

    let on_create_group = move |ev: SubmitEvent| {
        ev.prevent_default();

        let Some(create_group_dialog) = document()
            .get_element_by_id("create_group_dialog")
            .and_then(|el| el.dyn_into::<HtmlDialogElement>().ok())
        else {
            return;
        };

        let Some(form) = ev
            .target()
            .and_then(|target| target.dyn_into::<HtmlFormElement>().ok())
        else {
            return;
        };

        let Ok(data) = FormData::new_with_form(&form) else {
            return;
        };

        let Some(name) = data.get("name").as_string() else {
            return;
        };

        spawn_local(async move {
            let args = to_value(&CreateGroupArgs { name: &name }).expect("args to be JsValue");
            let resp = invoke("create_group", args).await;
            let Ok(resp) = from_value::<Settings>(resp) else {
                return;
            };

            settings.set(resp);

            form.reset();
            create_group_dialog.close();
        });
    };

    view! {
        <div class="navbar bg-base-100">
            <div class="navbar-start"></div>
            <div class="navbar-end">
                <button class="btn btn-ghost btn-circle" title="Create group" onclick="create_group_dialog.showModal()">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                </button>
            </div>
        </div>
        <main class="container px-4">
            {move || settings.get().map(|s| s.groups.iter().enumerate().map(|(i, group)| view! {
                <article class="prose max-w-none">
                    <div class="flex items-center">
                        <h2 class="m-0">{group.name.to_owned()}</h2>
                        <Show when=move || s.current_group == i>
                            <span class="badge badge-info badge-outline ml-2">current</span>
                        </Show>
                        <button class="btn btn-sm btn-ghost btn-circle ml-2" title="Set has current" on:click=move |_| {
                            spawn_local(async move {
                                let args = to_value(&GroupIdArgs { id: i }).expect("args to be JsValue");
                                let resp = invoke("set_current_group", args).await;
                                let Ok(resp) = from_value::<Settings>(resp) else {
                                    return;
                                };

                                settings.set(resp);
                            });
                        }>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                            </svg>
                        </button>
                        <button class="btn btn-sm btn-ghost btn-circle" title="Refresh group characters" on:click=move |_| {
                            spawn_local(async move {
                                let args = to_value(&GroupIdArgs { id: i }).expect("args to be JsValue");
                                let resp = invoke("refresh_group", args).await;
                                let Ok(resp) = from_value::<Settings>(resp) else {
                                    return;
                                };

                                settings.set(resp);
                            });
                        }>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
                            </svg>
                        </button>
                        <button class="btn btn-sm btn-ghost btn-circle" title="Delete group" on:click=move |_| {
                            spawn_local(async move {
                                let args = to_value(&GroupIdArgs { id: i }).expect("args to be JsValue");
                                let resp = invoke("delete_group", args).await;
                                let Ok(resp) = from_value::<Settings>(resp) else {
                                    return;
                                };

                                settings.set(resp);
                            });
                        }>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-wrap">
                        {group.characters.iter().enumerate().map(|(c_id, character)| view! {
                            <div draggable="true" on:dragstart=move |ev: DragEvent| {
                                let Some(data) = ev.data_transfer() else {
                                    return;
                                };

                                let _ = data.set_data("application/switcher-character", c_id.to_string().as_str());
                                data.set_effect_allowed("move");
                            }
                            on:dragover=move |ev| {
                                ev.prevent_default();

                                let Some(data) = ev.data_transfer() else {
                                    return;
                                };

                                data.set_drop_effect("move");
                            }
                            on:drop=move |ev| {
                                ev.prevent_default();

                                let Some(data) = ev.data_transfer() else {
                                    return;
                                };

                                let Ok(value) = data.get_data("application/switcher-character") else {
                                    return;
                                };

                                let Ok(from_id) = value.parse::<usize>() else {
                                    return;
                                };

                                spawn_local(async move {
                                    let args = to_value(&SwapGroupCharacterArgs { group: i, a: c_id, b: from_id }).expect("args to be JsValue");
                                    let resp = invoke("swap_group_character", args).await;
                                    let Ok(resp) = from_value::<Settings>(resp) else {
                                        return;
                                    };

                                    settings.set(resp);
                                });
                            }>
                                <div class="flex items-center">
                                    <button class="btn btn-sm btn-ghost btn-circle" title="Delete group" on:click=move |_| {
                                        spawn_local(async move {
                                            let args = to_value(&DeleteGroupCharacterArgs { group: i, character: c_id }).expect("args to be JsValue");
                                            let resp = invoke("delete_group_character", args).await;
                                            let Ok(resp) = from_value::<Settings>(resp) else {
                                                return;
                                            };

                                            settings.set(resp);
                                        });
                                    }>
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                                        </svg>
                                    </button>
                                </div>
                                <div class="avatar tooltip tooltip-bottom" data-tip=character.name.to_owned()>
                                    <div class=format!("ak-breed-icon ak-breed-icon-big breed{}_0 rounded-xl", character.breed.to_css_id())></div>
                                </div>
                            </div>
                        }).collect_view()}
                    </div>
                </article>
            }).collect_view())}

            <Portal>
                <dialog id="create_group_dialog" class="modal modal-bottom sm:modal-middle">
                    <div class="modal-box">
                        <h3 class="font-bold text-lg">Create group</h3>
                        <p class="py-4">
                            <form id="create_group_form" on:submit=on_create_group>
                                <input class="input input-bordered w-full max-w-xs" type="text" name="name" minlength="3" required />
                            </form>
                        </p>
                        <div class="modal-action">
                            <form method="dialog">
                                <button class="btn">Close</button>
                            </form>
                            <input type="submit" class="btn btn-neutral" form="create_group_form" value="Add"/>
                        </div>
                    </div>
                </dialog>
            </Portal>
        </main>
    }
}
