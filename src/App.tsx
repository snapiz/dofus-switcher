import { createEffect, createResource, createSignal, For, onCleanup, Suspense } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import "./App.css";

interface Character {
  name: string;
  breed?: string;
  enabled: boolean;
}

interface Group {
  name: string;
  characters: Character[]
}

const breeds = ["Ecaflip", "Eniripsa", "Iop", "Cra", "Feca", "Sacrieur", "Sadida", "Osamodas", "Enutrof", "Sram", "Xelor", "Pandawa", "Roublard", "Zobal", "Steamer", "Eliotrope", "Huppermage", "Ouginak", "Forgelance"]

const isSidebarWindow = getCurrentWindow().label === "sidebar"

function App() {
  return !isSidebarWindow ? (<MainApp></MainApp>) : (<SidebarApp></SidebarApp>)
}

function MainApp() {
  const [groups, { mutate }] = createResource<Group[]>(async () => (await invoke("get_groups")));
  const [available_characters, { refetch }] = createResource<Character[]>(async () => (await invoke("get_available_characters")));
  const [selectedGroup, setSelectedGroup] = createSignal(0)

  return (
    <>
      <div>
        <form onsubmit={async (e: SubmitEvent) => {
          e.preventDefault();
          const form = e.target as HTMLFormElement;
          const data = new FormData(form);
          const groups: Group[] = await invoke("create_group", Object.fromEntries(data));
          mutate(groups);
          setSelectedGroup(0)
          form.reset();
        }}>
          <input type="text" name="name" required minlength="3" />
        </form>
      </div>
      <Suspense>
        <For each={groups()}>{(group, i) =>
          <div>
            <label for={group.name}>
              <h2>{group.name}
                <button style="margin-left: 5px" onclick={async () => {
                  const groups: Group[] = await invoke("delete_group", { id: i() });
                  mutate(groups);
                  setSelectedGroup(0)
                }}>
                  Delete
                </button>
              </h2>
            </label>
            <input type="radio" id={group.name} name="group" value={group.name} checked={i() === selectedGroup()} />
            <div class="content">
              <div class="group-settings">
                <div>
                  <h4 ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                    const name = e.dataTransfer?.getData("application/group-available")
                    if (!name) {
                      return
                    }

                    const groups: Group[] = await invoke("add_character_to_group", { id: i(), name })
                    mutate(groups)
                  }}>Selected</h4>
                  <div>
                    <For each={group.characters}>
                      {(character, characterId) =>
                        <div title={character.name} classList={{ avatar: true, disabled: !character.enabled }} draggable ondragstart={(e: DragEvent) => {
                          e.dataTransfer?.setData("application/group-selected", characterId().toString())
                          e.dataTransfer?.setData("application/group-selected-pos", character.name)


                        }} ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                          const name = e.dataTransfer?.getData("application/group-available") || e.dataTransfer?.getData("application/group-selected-pos")

                          if (!name || name === character.name) {
                            return
                          }

                          let { x, width } = (e.target as HTMLElement).getBoundingClientRect()
                          const middleX = x + width / 2
                          const groups: Group[] = await invoke("add_character_to_group_at", { id: i(), name, targetName: character.name, right: e.clientX > middleX })
                          mutate(groups)

                        }} onclick={async () => {
                          const groups: Group[] = await invoke("set_character_enabled", { id: i(), characterId: characterId(), value: !character.enabled })
                          mutate(groups)
                        }}>
                          <img src={`/breeds/${character.breed || 'None'}.png`} />
                        </div>
                      }
                    </For>
                  </div>
                </div>
                <div>
                  <h4 ondragover={(e: DragEvent) => e.preventDefault()} ondrop={async (e: DragEvent) => {
                    const characterId = e.dataTransfer?.getData("application/group-selected")
                    if (!characterId) {
                      return
                    }

                    const groups: Group[] = await invoke("remove_character_from_group", { id: i(), characterId: parseInt(characterId, 10) })
                    mutate(groups)
                  }}>
                    Available
                    <button style="margin-left: 5px" onclick={() => {
                      setSelectedGroup(i())
                      refetch()
                    }}>Refresh</button>
                  </h4>
                  <div>
                    <For each={available_characters()?.filter((ac) => !group.characters.some((c) => c.name === ac.name))}>
                      {(character) => {
                        const [open, setOpen] = createSignal(false);

                        return <>
                          <div title={character.name} class="avatar" onclick={() => setOpen(true)} draggable ondragstart={(e: DragEvent) => {
                            e.dataTransfer?.setData("application/group-available", character.name)

                          }}>
                            <img src={`/breeds/${character.breed || 'None'}.png`} />
                          </div>
                          <dialog open={open()}>
                            <h4>{character.name}</h4>
                            <For each={breeds}>
                              {(breed) =>
                                <div class="avatar" onclick={async () => {
                                  const groups: Group[] = await invoke("set_character_breed", { breed, name: character.name })
                                  mutate(groups)
                                  refetch()
                                  setOpen(false)
                                }}>
                                  <img src={`/breeds/${breed}.png`} />
                                </div>
                              }
                            </For>
                          </dialog>
                        </>
                      }}
                    </For>
                  </div>
                </div>
              </div>
            </div>
          </div>
        }
        </For>
      </Suspense>
    </>
  );
}

function SidebarApp() {
  const [characters, { mutate }] = createResource<[Number, Character[]]>(async () => (await invoke("get_active_characters")));

  const updater = setInterval(async () => {
    const resp: [Number, Character[]] | undefined = await invoke("get_active_characters");
    for (const [i, c] of (resp?.[1].entries() || [])) {
      const e = characters()?.[1][i]
      if (c.name !== e?.name || c.breed !== e.breed) {
        mutate(resp)
        break
      }
    }
  }, 1000)

  createEffect(() => {
    const count = characters()?.[1].length || 1

    getCurrentWindow().setSize(new LogicalSize(80, 80 * count))
  })

  onCleanup(() => clearInterval(updater))

  return (
    <>
      <Suspense>
        <For each={characters()?.[1]}>
          {(character, characterId) =>
            <div title={character.name} classList={{ avatar: true, disabled: !character.enabled }} onclick={async () => {
              await invoke("set_character_enabled", { id: characters()?.[0], characterId: characterId(), value: !character.enabled })
              mutate(v => {

                if (!v) {
                  return v
                }

                let td = [...v[1]]
                td[characterId()] = { ...v[1][characterId()], enabled: !character.enabled }

                return [v[0], td]
              })
            }}>
              <img src={`/breeds/${character.breed || 'None'}.png`} />
            </div>
          }
        </For>
      </Suspense>
    </>
  );
}

export default App;
